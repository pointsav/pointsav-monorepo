use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".local/share/proof/ingest.db")
}

fn open_db() -> Result<Connection> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(&path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS ingest_log (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at  TEXT    NOT NULL,
            username    TEXT    NOT NULL,
            tenant      TEXT    NOT NULL,
            path        TEXT    NOT NULL,
            ledger_id   TEXT,
            status      TEXT    NOT NULL
        );",
    )?;
    Ok(conn)
}

pub struct IngestRecord {
    pub created_at: String,
    pub username: String,
    pub tenant: String,
    pub path: String,
    pub ledger_id: Option<String>,
    pub status: String,
}

pub fn append(rec: &IngestRecord) -> Result<()> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO ingest_log (created_at, username, tenant, path, ledger_id, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            rec.created_at,
            rec.username,
            rec.tenant,
            rec.path,
            rec.ledger_id,
            rec.status,
        ],
    )?;
    Ok(())
}
