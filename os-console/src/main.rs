#[cfg(feature = "ssh-server")]
mod ssh_server;

mod mba_client;

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
    use app_console_input::InputCartridge;
    use app_console_keys::{AppConsoleKeys, ConsoleConfig, PairingInfo};

    let cfg = ConsoleConfig::load();
    let p = &cfg.profile;

    // Attempt MBA peer-to-peer link before starting the TUI (5s timeout)
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
    // Drop the runtime — MBA result is captured; session is done for v1
    drop(rt);

    let mut chassis = AppConsoleKeys::new(&p.username, &p.tenant);
    chassis.set_pairing_info(PairingInfo {
        fingerprint: mba.fingerprint,
        host: p.totebox_host.clone(),
        port: p.totebox_ssh_port,
    });
    if mba.active {
        chassis.set_mba_active();
    }

    chassis.register(Box::new(ContentCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.proof_endpoint,
    )));
    chassis.register(Box::new(InputCartridge::new_for(
        &p.username,
        &p.tenant,
        &p.ingest_endpoint,
    )));
    chassis.run_local()
}
