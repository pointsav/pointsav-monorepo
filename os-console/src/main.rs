#[cfg(feature = "ssh-server")]
mod ssh_server;

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
    use app_console_keys::{AppConsoleKeys, ConsoleConfig};
    let cfg = ConsoleConfig::load();
    let p = &cfg.profile;
    let mut chassis = AppConsoleKeys::new(&p.username, &p.tenant);
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
