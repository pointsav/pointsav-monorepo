// SPDX-License-Identifier: Apache-2.0
//
// Content-addressed blob store — the "content" half of the two-tier
// split ratified in BRIEF-cross-compliance-worm-design.md (project-
// input) Decisions locked #7. Document bytes live here, keyed by
// retention-class + sha256 + creation time; the ledger keeps only a
// metadata entry referencing this path by hash. Write-then-rename +
// fsync + chmod 0o444, same D4 discipline `posix_tile.rs` uses for the
// ledger's own log file — genuinely write-once here, unlike the log
// file (which posix_tile.rs rewrites in full on every append).

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum BlobError {
    Io(std::io::Error),
    InvalidBase64(base64::DecodeError),
}

impl std::fmt::Display for BlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlobError::Io(e) => write!(f, "blob store I/O error: {e}"),
            BlobError::InvalidBase64(e) => write!(f, "invalid base64 content: {e}"),
        }
    }
}

impl std::error::Error for BlobError {}

impl From<std::io::Error> for BlobError {
    fn from(e: std::io::Error) -> Self {
        BlobError::Io(e)
    }
}

/// sha256 of the raw (decoded) bytes, hex-encoded.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Extension extracted from a filename, defaulting to `bin` when the
/// filename has none or isn't provided — the blob key must still be
/// constructible even for an extensionless upload.
pub fn extension_from_filename(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .filter(|e| !e.is_empty())
        .unwrap_or("bin")
        .to_string()
}

/// `<blob_root>/<retention_class>/<sha256>--<created-compact-ISO8601>.<ext>`
pub fn blob_path(
    blob_root: &Path,
    retention_class: &str,
    sha256_hex: &str,
    created: DateTime<Utc>,
    ext: &str,
) -> PathBuf {
    let created_compact = created.format("%Y-%m-%dT%H-%M-%SZ");
    blob_root
        .join(retention_class)
        .join(format!("{sha256_hex}--{created_compact}.{ext}"))
}

/// Write `bytes` to `path` using write-then-rename + fsync + chmod
/// 0o444. If `path` already exists, this is a no-op success — the
/// path is content-addressed (sha256 in the name), so an existing
/// file at that exact path is, by construction, the same bytes;
/// re-appending the same document is a legitimate operation (e.g. a
/// retry), not an error.
pub fn write_blob_once(path: &Path, bytes: &[u8]) -> Result<(), BlobError> {
    if path.exists() {
        return Ok(());
    }
    let parent = path.parent().ok_or_else(|| {
        BlobError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "blob path has no parent directory",
        ))
    })?;
    std::fs::create_dir_all(parent)?;

    let tmp_path = with_tmp_suffix(path);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o644)
        .open(&tmp_path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);

    // Another writer may have raced us and already written the final
    // path between our exists() check and now — content-addressed, so
    // the bytes are the same either way; keep our temp-file cleanup
    // path simple and let a concurrent rename's loser just remove its
    // now-orphaned tmp file.
    if path.exists() {
        let _ = std::fs::remove_file(&tmp_path);
        return Ok(());
    }

    std::fs::rename(&tmp_path, path)?;

    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

fn with_tmp_suffix(p: &Path) -> PathBuf {
    let mut name = p.file_name().expect("blob path has filename").to_owned();
    name.push(".tmp");
    p.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir() -> PathBuf {
        static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "svc-fs-blob-test-{}-{}",
            std::process::id(),
            n
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn sha256_hex_matches_known_vector() {
        // sha256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn extension_defaults_to_bin_when_absent() {
        assert_eq!(extension_from_filename("no_extension"), "bin");
        assert_eq!(extension_from_filename("report.pdf"), "pdf");
        assert_eq!(extension_from_filename("archive.tar.gz"), "gz");
    }

    #[test]
    fn blob_path_shape_is_correct() {
        let root = PathBuf::from("/var/lib/local-fs/tenant/blobs");
        let created = DateTime::parse_from_rfc3339("2026-09-04T12:30:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let path = blob_path(&root, "finra-6y", "abc123", created, "pdf");
        assert_eq!(
            path,
            PathBuf::from(
                "/var/lib/local-fs/tenant/blobs/finra-6y/abc123--2026-09-04T12-30-00Z.pdf"
            )
        );
    }

    #[test]
    fn write_blob_once_creates_file_write_once_0o444() {
        let dir = tmpdir();
        let path = dir.join("cls").join("hash--created.pdf");
        write_blob_once(&path, b"hello world").unwrap();

        assert!(path.exists());
        let contents = std::fs::read(&path).unwrap();
        assert_eq!(contents, b"hello world");

        let perms = std::fs::metadata(&path).unwrap().permissions();
        assert_eq!(perms.mode() & 0o777, 0o444);
    }

    #[test]
    fn write_blob_once_is_idempotent_on_existing_path() {
        let dir = tmpdir();
        let path = dir.join("cls").join("hash--created.pdf");
        write_blob_once(&path, b"hello world").unwrap();
        // Second call to the same content-addressed path must not
        // error even though the file is now 0o444.
        write_blob_once(&path, b"hello world").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"hello world");
    }
}
