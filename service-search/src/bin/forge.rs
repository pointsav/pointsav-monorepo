//! forge — build the search index and exit (releasing RAM), per service-search's mandate.
//!
//! Usage: forge <config.toml>

fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: forge <config.toml>");
        std::process::exit(2);
    });
    let config = service_search::Config::from_toml_path(&config_path)?;

    eprintln!(
        "forge: indexing {} root(s) → {}",
        config.roots.len(),
        config.index_path
    );
    let stats = service_search::forge(&config)?;
    eprintln!(
        "forge: done — {} files indexed, {} content-skipped, {} roots. Exiting to release RAM.",
        stats.files, stats.content_skipped, stats.roots
    );
    Ok(())
}
