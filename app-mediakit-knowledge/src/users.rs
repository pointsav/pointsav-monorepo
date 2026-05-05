//! Phase 5: user model, SQLite schema, and password verification.
//!
//! Two roles: `admin` (direct edits + review queue) and `editor`
//! (edits go to pending queue). Accounts are created by admin only —
//! no public sign-up. The first admin is seeded from env on startup
//! when the users table is empty.

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use rusqlite::{Connection, OptionalExtension, params};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: String, // "admin" | "editor"
    pub created_at: i64,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

pub type DbPool = Arc<Mutex<Connection>>;

/// Create all tables. Idempotent — safe to call on every startup.
pub fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS users (
            id          TEXT PRIMARY KEY,
            username    TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role        TEXT NOT NULL DEFAULT 'editor',
            created_at  INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS sessions (
            token       TEXT PRIMARY KEY,
            user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            expires_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
        CREATE TABLE IF NOT EXISTS pending_edits (
            id          TEXT PRIMARY KEY,
            slug        TEXT NOT NULL,
            author_id   TEXT NOT NULL REFERENCES users(id),
            body        TEXT NOT NULL,
            edit_summary TEXT NOT NULL DEFAULT '',
            status      TEXT NOT NULL DEFAULT 'pending',
            reviewer_note TEXT,
            submitted_at INTEGER NOT NULL,
            reviewed_at  INTEGER,
            reviewed_by  TEXT REFERENCES users(id)
        );
        CREATE INDEX IF NOT EXISTS idx_pending_slug   ON pending_edits(slug, status);
        CREATE INDEX IF NOT EXISTS idx_pending_author ON pending_edits(author_id, submitted_at);
        CREATE INDEX IF NOT EXISTS idx_pending_status ON pending_edits(status, submitted_at);
    ")
}

/// Seed the admin user if the table is empty. The password hash must be
/// a pre-computed argon2id string (operator generates with `argon2 hash`
/// CLI or the helper route below). Does nothing if any user already exists.
pub fn seed_admin_if_empty(
    conn: &Connection,
    username: &str,
    password_hash: &str,
) -> rusqlite::Result<()> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM users",
        [],
        |row| row.get(0),
    )?;
    if count == 0 {
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO users (id, username, password_hash, role, created_at) VALUES (?1, ?2, ?3, 'admin', ?4)",
            params![Uuid::new_v4().to_string(), username, password_hash, now],
        )?;
        tracing::info!(username, "seeded admin user from env");
    }
    Ok(())
}

/// Verify a plaintext password against the stored argon2id hash.
/// Returns true if valid.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else { return false; };
    Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
}

/// Hash a plaintext password with argon2id. Used when creating accounts via UI.
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("argon2 error: {e}"))?
        .to_string();
    Ok(hash)
}

/// Look up a user by username. Returns None if not found.
pub fn get_by_username(conn: &Connection, username: &str) -> rusqlite::Result<Option<User>> {
    conn.query_row(
        "SELECT id, username, password_hash, role, created_at FROM users WHERE username = ?1",
        params![username],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            created_at: row.get(4)?,
        }),
    ).optional()
}

/// Look up a user by session token. Returns None if token not found or expired.
pub fn get_by_session(conn: &Connection, token: &str) -> rusqlite::Result<Option<User>> {
    let now = chrono::Utc::now().timestamp();
    conn.query_row(
        "SELECT u.id, u.username, u.password_hash, u.role, u.created_at
         FROM sessions s JOIN users u ON u.id = s.user_id
         WHERE s.token = ?1 AND s.expires_at > ?2",
        params![token, now],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            created_at: row.get(4)?,
        }),
    ).optional()
}

/// Create a session token and insert it. Returns the token string.
pub fn create_session(conn: &Connection, user_id: &str) -> rusqlite::Result<String> {
    let token = Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now().timestamp() + 7 * 24 * 3600; // 7 days
    conn.execute(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES (?1, ?2, ?3)",
        params![token, user_id, expires_at],
    )?;
    Ok(token)
}

/// Delete a session (logout).
pub fn delete_session(conn: &Connection, token: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
    Ok(())
}

/// List all users (for admin UI).
pub fn list_users(conn: &Connection) -> rusqlite::Result<Vec<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, role, created_at FROM users ORDER BY created_at"
    )?;
    let users = stmt.query_map([], |row| Ok(User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
        role: row.get(3)?,
        created_at: row.get(4)?,
    }))?.collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(users)
}

/// Create a new user account (admin only via UI).
pub fn create_user(
    conn: &Connection,
    username: &str,
    password: &str,
    role: &str,
) -> anyhow::Result<()> {
    let hash = hash_password(password)?;
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO users (id, username, password_hash, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![Uuid::new_v4().to_string(), username, hash, role, now],
    )?;
    Ok(())
}
