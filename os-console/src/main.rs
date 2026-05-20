use anyhow::Result;
use app_console_content::cartridge::ContentCartridge;
use app_console_keys::AppConsoleKeys;

fn main() -> Result<()> {
    let mut chassis = AppConsoleKeys::new("operator", "local");
    chassis.register(Box::new(ContentCartridge::new()));
    chassis.run_local()
}
