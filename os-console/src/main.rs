#[cfg(feature = "ssh-server")]
mod ssh_server;

mod mba_client;
mod metrics;

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

    if mba.active {
        chassis.set_mba_active();
    } else {
        // MBA inactive — start zero-jargon pairing flow
        chassis.set_pairing_unpaired(mba.fingerprint.clone());

        let pub_key_line = load_pubkey_line(&p.ssh_key_path);
        match pairing::post_pair_request(
            &p.pair_endpoint,
            &p.username,
            &p.tenant,
            &pub_key_line,
            &mba.fingerprint,
        ) {
            Ok((request_id, code)) => {
                chassis.set_pairing_awaiting(code, request_id.clone(), mba.fingerprint.clone());
                let rx = pairing::spawn_status_poll(p.pair_endpoint.clone(), request_id);
                chassis.set_pair_rx(rx);
            }
            Err(e) => {
                chassis.set_pairing_error(format!("{e}"));
            }
        }
    }

    chassis.register(Box::new(PeopleCartridge::new(
        &p.people_endpoint,
        p.plain_mode,
    )));
    chassis.register(Box::new(EmailCartridge::new_for(
        &p.email_endpoint,
        p.plain_mode,
    )));
    let sess =
        app_console_keys::SessionState::load(&app_console_keys::SessionState::default_path());
    chassis.register(Box::new(ContentCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.proof_endpoint,
        &p.slm_endpoint,
        &p.drafts_outbound_path,
        &p.content_endpoint,
        sess.content_query,
        sess.content_selected,
        sess.content_scroll,
    )));
    chassis.register(Box::new(InputCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.ingest_endpoint,
    )));
    chassis.register(Box::new(SlmCartridge::new(&p.slm_endpoint, p.plain_mode)));
    chassis.register(Box::new(SystemCartridge::new(&p.pair_endpoint)));
    metrics::spawn_metrics_server(p.metrics_port);
    // Register SIGTERM + SIGINT handler — requests clean shutdown on next chassis tick.
    let _ = ctrlc::set_handler(|| {
        app_console_keys::request_shutdown();
    });

    chassis.run_local()
}

/// Read the OpenSSH public key line from the .pub file alongside the private key.
fn load_pubkey_line(private_key_path: &str) -> String {
    let pub_path = format!("{}.pub", private_key_path);
    std::fs::read_to_string(&pub_path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}
