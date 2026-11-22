// SPDX-License-Identifier: Apache-2.0

//! Serves mirrored events to CalDAV clients from the index, without
//! scanning the mirror.
//!
//! Each event (one UID under one organizer) is one resource named
//! after its event key. For each component the latest version is
//! taken; a component whose latest version is a cancellation is not
//! served, and an event whose master's latest version is a
//! cancellation is not served at all. When every chosen version sits
//! in one file, that file's bytes are served unchanged; otherwise a
//! calendar is assembled from the chosen VEVENT blocks (verbatim) and
//! the VTIMEZONE blocks of the files they came from. Bytes are read
//! from the mirror on first request and checked against the indexed
//! SHA-256. Only versions from the configured mailboxes are used.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::SystemTime;

use bytes::Bytes;
use worm_ledger::blob::sha256_hex;

use crate::mirror_index::{ComponentRecord, CsvError, MirrorIndex};
use crate::mirror_rebuild::MailboxRoot;

#[derive(Debug)]
pub enum MirrorViewError {
    Io(std::io::Error),
    Csv(CsvError),
    /// The file's bytes no longer match the indexed SHA-256.
    HashMismatch(PathBuf),
    /// The indexed VEVENT could not be found in the file.
    MissingComponent(PathBuf),
    NotUtf8(PathBuf),
}

impl std::fmt::Display for MirrorViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MirrorViewError::Io(e) => write!(f, "{e}"),
            MirrorViewError::Csv(e) => write!(f, "{e}"),
            MirrorViewError::HashMismatch(p) => {
                write!(f, "{} does not match its indexed SHA-256", p.display())
            }
            MirrorViewError::MissingComponent(p) => {
                write!(f, "indexed VEVENT not found in {}", p.display())
            }
            MirrorViewError::NotUtf8(p) => write!(f, "{} is not UTF-8", p.display()),
        }
    }
}

impl std::error::Error for MirrorViewError {}

#[derive(Debug, Clone)]
struct Part {
    uid: String,
    mailbox: String,
    path: PathBuf,
    sha256: String,
    ordinal: usize,
}

#[derive(Debug)]
struct Entry {
    uid: String,
    parts: Vec<Part>,
    modified: SystemTime,
    content: OnceLock<Bytes>,
}

/// Read-only view of mirrored events for the CalDAV file system.
#[derive(Debug, Default)]
pub struct MirrorView {
    entries: BTreeMap<String, Entry>,
}

fn to_system_time(r: &ComponentRecord) -> SystemTime {
    r.last_modified
        .or(r.dtstamp)
        .map(SystemTime::from)
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

impl MirrorView {
    pub fn load(index_path: &Path, mailboxes: &[MailboxRoot]) -> Result<Self, MirrorViewError> {
        let text = std::fs::read_to_string(index_path).map_err(MirrorViewError::Io)?;
        let index = MirrorIndex::from_csv(&text).map_err(MirrorViewError::Csv)?;
        Ok(Self::from_index(&index, mailboxes))
    }

    pub fn from_index(index: &MirrorIndex, mailboxes: &[MailboxRoot]) -> Self {
        let roots: HashMap<&str, &Path> = mailboxes
            .iter()
            .map(|m| (m.mailbox.as_str(), m.root.as_path()))
            .collect();
        let mut entries = BTreeMap::new();
        for event in index.events() {
            let mut chosen: Vec<&ComponentRecord> = Vec::new();
            let mut master_cancelled = false;
            for component in &event.components {
                let latest = component
                    .versions
                    .iter()
                    .rev()
                    .find(|v| roots.contains_key(v.mailbox.as_str()));
                let Some(latest) = latest else { continue };
                if latest.is_cancelled() {
                    master_cancelled |= component.recurrence_id.is_empty();
                    continue;
                }
                chosen.push(latest);
            }
            if master_cancelled || chosen.is_empty() {
                continue;
            }
            let parts = chosen
                .iter()
                .filter_map(|r| {
                    roots.get(r.mailbox.as_str()).map(|root| Part {
                        uid: r.uid.clone(),
                        mailbox: r.mailbox.clone(),
                        path: root.join(&r.file),
                        sha256: r.sha256.clone(),
                        ordinal: r.ordinal,
                    })
                })
                .collect();
            let modified = chosen
                .iter()
                .map(|r| to_system_time(r))
                .max()
                .unwrap_or(SystemTime::UNIX_EPOCH);
            entries.insert(
                event.resource_name(),
                Entry {
                    uid: event.uid.to_string(),
                    parts,
                    modified,
                    content: OnceLock::new(),
                },
            );
        }
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Resource names (without `.ics`) and the UID each one carries.
    pub fn resources(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(k, e)| (k.as_str(), e.uid.as_str()))
    }

    pub fn uid(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|e| e.uid.as_str())
    }

    /// The mailboxes that hold a live version of `name`, sorted, once each.
    pub fn mailboxes(&self, name: &str) -> Vec<&str> {
        let mut out: Vec<&str> = self
            .entries
            .get(name)
            .map(|e| e.parts.iter().map(|p| p.mailbox.as_str()).collect())
            .unwrap_or_default();
        out.sort_unstable();
        out.dedup();
        out
    }

    pub fn modified(&self, name: &str) -> Option<SystemTime> {
        self.entries.get(name).map(|e| e.modified)
    }

    /// The bytes to serve for `name`, or `None` if it is not a
    /// mirrored resource.
    pub fn content(&self, name: &str) -> Result<Option<Bytes>, MirrorViewError> {
        let Some(entry) = self.entries.get(name) else {
            return Ok(None);
        };
        if let Some(b) = entry.content.get() {
            return Ok(Some(b.clone()));
        }
        let bytes = assemble(&entry.parts)?;
        Ok(Some(entry.content.get_or_init(|| bytes).clone()))
    }
}

fn read_verified(part: &Part) -> Result<Vec<u8>, MirrorViewError> {
    let bytes = std::fs::read(&part.path).map_err(MirrorViewError::Io)?;
    if sha256_hex(&bytes) != part.sha256 {
        return Err(MirrorViewError::HashMismatch(part.path.clone()));
    }
    Ok(bytes)
}

fn assemble(parts: &[Part]) -> Result<Bytes, MirrorViewError> {
    let mut texts: BTreeMap<&Path, String> = BTreeMap::new();
    for part in parts {
        if !texts.contains_key(part.path.as_path()) {
            let bytes = read_verified(part)?;
            let text = String::from_utf8(bytes)
                .map_err(|_| MirrorViewError::NotUtf8(part.path.clone()))?;
            texts.insert(&part.path, text);
        }
    }

    // One file holding exactly the chosen VEVENTs is served unchanged.
    if let [only] = texts.values().collect::<Vec<_>>().as_slice() {
        if blocks(only, "VEVENT").len() == parts.len() {
            return Ok(Bytes::from(only.as_bytes().to_vec()));
        }
    }

    let mut timezones: Vec<String> = Vec::new();
    let mut seen_tzids: Vec<String> = Vec::new();
    for text in texts.values() {
        for block in blocks(text, "VTIMEZONE") {
            let tzid = block
                .lines()
                .find_map(|l| l.trim_end().strip_prefix("TZID:").map(str::to_string))
                .unwrap_or_default();
            if !seen_tzids.contains(&tzid) {
                seen_tzids.push(tzid);
                timezones.push(block);
            }
        }
    }

    let mut out = String::from(
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//service-calendar//mirror//EN\r\n",
    );
    for tz in &timezones {
        out.push_str(tz);
    }
    for part in parts {
        let text = texts
            .get(part.path.as_path())
            .map(String::as_str)
            .unwrap_or("");
        let block = blocks(text, "VEVENT")
            .into_iter()
            .nth(part.ordinal)
            .filter(|b| block_uid(b).as_deref() == Some(part.uid.as_str()))
            .ok_or_else(|| MirrorViewError::MissingComponent(part.path.clone()))?;
        out.push_str(&block);
    }
    out.push_str("END:VCALENDAR\r\n");
    Ok(Bytes::from(out.into_bytes()))
}

/// The UID of a VEVENT block, read after unfolding (long UIDs are
/// folded across lines).
fn block_uid(block: &str) -> Option<String> {
    let unfolded = block.replace("\r\n ", "").replace("\r\n\t", "");
    unfolded.lines().find_map(|l| {
        let (name, value) = l.split_once(':')?;
        let name = name.split(';').next()?;
        name.eq_ignore_ascii_case("UID")
            .then(|| value.trim().to_string())
    })
}

/// The verbatim `BEGIN:<kind>` .. `END:<kind>` blocks of `text`, in
/// order, each ending in CRLF.
fn blocks(text: &str, kind: &str) -> Vec<String> {
    let begin = format!("BEGIN:{kind}");
    let end = format!("END:{kind}");
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in text.split_inclusive('\n') {
        let bare = line.trim_end_matches(['\r', '\n']);
        if current.is_none() && bare.eq_ignore_ascii_case(&begin) {
            current = Some(String::new());
        }
        if let Some(block) = current.as_mut() {
            block.push_str(bare);
            block.push_str("\r\n");
            if bare.eq_ignore_ascii_case(&end) {
                if let Some(done) = current.take() {
                    out.push(done);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_are_extracted_in_order() {
        let text = "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:a\nBEGIN:VALARM\nEND:VALARM\nEND:VEVENT\nBEGIN:VEVENT\nUID:b\nEND:VEVENT\nEND:VCALENDAR\n";
        let b = blocks(text, "VEVENT");
        assert_eq!(b.len(), 2);
        assert!(b[0].contains("UID:a") && b[0].contains("BEGIN:VALARM"));
        assert!(b[1].starts_with("BEGIN:VEVENT\r\nUID:b\r\n"));
    }
}
