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
    use app_console_keys::AppConsoleKeys;
    let mut chassis = AppConsoleKeys::new("operator", "local");
    chassis.register(Box::new(ContentCartridge::new()));
    chassis.register(Box::new(InputCartridge::new()));
    chassis.run_local()
}
