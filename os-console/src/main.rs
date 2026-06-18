#[cfg(feature = "ssh-server")]
mod ssh_server;

mod mba_client;
mod metrics;
mod tunnel;

fn main() -> anyhow::Result<()> {
    inner_main()
}

#[cfg(feature = "ssh-server")]
fn inner_main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(ssh_server::run())
}

#[cfg(not(feature = "ssh-server"))]
fn inner_main() -> anyhow::Result<()> {
    use app_console_content::cartridge::ContentCartridge;
    use app_console_email::EmailCartridge;
    use app_console_input::InputCartridge;
    use app_console_keys::{pairing, AppConsoleKeys, ConsoleConfig};
    use app_console_people::PeopleCartridge;
    use app_console_slm::SlmCartridge;
    use app_console_system::SystemCartridge;

    let cfg = ConsoleConfig::load();
    let p = &cfg.profile;

    // Port list used by both tunnel paths.
    let tunnel_forwards: &[(u16, u16)] = &[
        (9080, 9080), // Doorman
        (9081, 9081), // service-content
        (9092, 9092), // service-proofreader (F4)
        (9100, 9100), // service-input (F12)
        (9093, 9093), // service-email (F3)
        (9205, 9205), // pairing-server (F11)
        (2222, 2222), // MBA SSH
    ];

    // Start embedded SSH tunnel if gce_host is configured
    let mut _ssh_child: Option<std::process::Child> = None;
    if !p.gce_host.is_empty() {
        // Skip tunnel if ports are already bound (user has an external ssh -N running).
        if std::net::TcpStream::connect("127.0.0.1:9205").is_err() {
            tunnel::spawn_tunnel(tunnel::TunnelConfig {
                gce_host: p.gce_host.clone(),
                gce_port: p.gce_ssh_port,
                username: p.gce_user.clone(),
                key_path: p.ssh_key_path.clone(),
                forwards: tunnel_forwards.to_vec(),
            });
            // Wait up to 5s for russh to bind ports.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            loop {
                if std::net::TcpStream::connect("127.0.0.1:9205").is_ok() {
                    break;
                }
                if std::time::Instant::now() >= deadline {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }

        // Russh didn't bind ports — fall back to system ssh.
        if std::net::TcpStream::connect("127.0.0.1:9205").is_err() {
            let mut cmd = std::process::Command::new("ssh");
            cmd.arg("-N")
               .arg("-o").arg("StrictHostKeyChecking=accept-new")
               .arg("-o").arg("ServerAliveInterval=30")
               .arg("-o").arg("ServerAliveCountMax=3")
               .arg("-p").arg(p.gce_ssh_port.to_string())
               .arg("-i").arg(&p.ssh_key_path);
            for &(local_port, remote_port) in tunnel_forwards {
                cmd.arg("-L").arg(format!("{local_port}:localhost:{remote_port}"));
            }
            cmd.arg(format!("{}@{}", p.gce_user, p.gce_host));
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
            _ssh_child = cmd.spawn().ok();

            // Wait up to 15s for system ssh to bind ports.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
            loop {
                if std::net::TcpStream::connect("127.0.0.1:9205").is_ok() {
                    break;
                }
                if std::time::Instant::now() >= deadline {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    }

    // Attempt MBA peer-to-peer link (5s timeout)
    let rt = tokio::runtime::Runtime::new()?;
    let mba = rt.block_on(async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            mba_client::connect_mba(
                &p.totebox_host,
                p.totebox_ssh_port,
                &p.username,
                &p.ssh_key_path,
            ),
        )
        .await
        .unwrap_or_else(|_| mba_client::MbaResult {
            active: false,
            fingerprint: "(connection timed out)".into(),
        })
    });
    drop(rt);

    let mut chassis = AppConsoleKeys::new(&p.username, &p.tenant);
    chassis.set_pair_base_url(p.pair_endpoint.clone());

    if mba.active {
        chassis.set_mba_active();
    } else {
        // MBA inactive — start zero-jargon pairing flow
        chassis.set_pairing_unpaired(mba.fingerprint.clone());

        let pub_key_line = load_pubkey_line(&p.ssh_key_path);
        let pair_rx = match pairing::post_pair_request(
            &p.pair_endpoint,
            &p.username,
            &p.tenant,
            &pub_key_line,
            &mba.fingerprint,
        ) {
            Ok((request_id, code)) => {
                chassis.set_pairing_awaiting(code, request_id.clone(), mba.fingerprint.clone());
                pairing::spawn_status_poll(p.pair_endpoint.clone(), request_id)
            }
            Err(_) => {
                // Tunnel not ready yet — retry in background; TUI shows "Connecting…"
                pairing::spawn_pair_init(
                    p.pair_endpoint.clone(),
                    p.username.clone(),
                    p.tenant.clone(),
                    pub_key_line,
                    mba.fingerprint.clone(),
                )
            }
        };
        chassis.set_pair_rx(pair_rx);

        // Reconnect watchdog — retries MBA link with exponential backoff.
        let (reconnect_tx, reconnect_rx) = std::sync::mpsc::channel::<bool>();
        let host = p.totebox_host.clone();
        let port = p.totebox_ssh_port;
        let username = p.username.clone();
        let key_path = p.ssh_key_path.clone();
        std::thread::spawn(move || {
            let mut delay = std::time::Duration::from_secs(2);
            let max_delay = std::time::Duration::from_secs(60);
            loop {
                std::thread::sleep(delay);
                let result = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map(|rt| {
                        rt.block_on(async {
                            tokio::time::timeout(
                                std::time::Duration::from_secs(5),
                                mba_client::connect_mba(&host, port, &username, &key_path),
                            )
                            .await
                            .unwrap_or_else(|_| mba_client::MbaResult {
                                active: false,
                                fingerprint: "(timeout)".into(),
                            })
                        })
                    });
                let active = result.map(|r| r.active).unwrap_or(false);
                if active {
                    let _ = reconnect_tx.send(true);
                    break;
                }
                delay = (delay * 2).min(max_delay);
            }
        });
        chassis.set_mba_reconnect_rx(reconnect_rx);
    }

    chassis.register(Box::new(PeopleCartridge::new_for(&p.people_endpoint)));
    chassis.register(Box::new(EmailCartridge::new_for(
        &p.email_endpoint,
        p.plain_mode,
    )));
    chassis.register(Box::new(ContentCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.proof_endpoint,
        &p.slm_endpoint,
        &p.drafts_outbound_path,
        &p.content_endpoint,
    )));
    chassis.register(Box::new(InputCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.ingest_endpoint,
    )));
    chassis.register(Box::new(SlmCartridge::new(&p.slm_endpoint, p.plain_mode)));
    chassis.register(Box::new(SystemCartridge::new(&p.pair_endpoint)));
    metrics::spawn_metrics_server(p.metrics_port);
    let _ = ctrlc::set_handler(|| {
        app_console_keys::request_shutdown();
    });
    let result = chassis.run_local();
    if let Some(mut child) = _ssh_child {
        let _ = child.kill();
    }
    result
}

/// Read the OpenSSH public key line from the .pub file alongside the private key.
fn load_pubkey_line(private_key_path: &str) -> String {
    let pub_path = format!("{}.pub", private_key_path);
    std::fs::read_to_string(&pub_path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}
