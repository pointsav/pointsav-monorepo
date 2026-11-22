// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Append-only log of [`Annotation`] rows: client deletes and items that
//! left the server. It lives outside the mirror and outside the index, so
//! neither a mirror scan nor an index rebuild can drop it. One JSON object
//! per line. Records are only appended; a line that cannot be read is
//! counted and skipped, never rewritten. The rebuild takes the result of
//! [`AnnotationLog::read`] as its `annotations` argument.

use crate::mirror_index::{Annotation, AnnotationKind};
use crate::mirror_rebuild::MailboxRoot;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum LogError {
    Io(io::Error),
    /// The log path is inside a mailbox root; a mirror holds mail only.
    InsideMirror(PathBuf),
}

impl From<io::Error> for LogError {
    fn from(e: io::Error) -> Self {
        LogError::Io(e)
    }
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogError::Io(e) => write!(f, "annotation log: {e}"),
            LogError::InsideMirror(p) => {
                write!(
                    f,
                    "annotation log path is inside mailbox root {}",
                    p.display()
                )
            }
        }
    }
}

impl std::error::Error for LogError {}

/// What [`AnnotationLog::read`] found.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct LogRead {
    pub annotations: Vec<Annotation>,
    /// Lines that could not be read (torn last line, bad UTF-8, unknown kind).
    pub skipped: usize,
}

#[derive(Serialize, Deserialize)]
struct Record {
    event_key: String,
    recurrence_id: Option<String>,
    mailbox: String,
    kind: String,
    at: Option<String>,
    file: Option<String>,
    sha256: Option<String>,
    note: String,
}

impl Record {
    fn from_annotation(a: &Annotation) -> Self {
        Self {
            event_key: a.event_key.clone(),
            recurrence_id: a.recurrence_id.clone(),
            mailbox: a.mailbox.clone(),
            kind: a.kind.as_str().to_string(),
            at: a.at.map(|t| t.to_rfc3339()),
            file: a.file.clone(),
            sha256: a.sha256.clone(),
            note: a.note.clone(),
        }
    }

    fn into_annotation(self) -> Option<Annotation> {
        let kind = AnnotationKind::parse(&self.kind)?;
        let at = match self.at {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s).ok()?.with_timezone(&Utc)),
            None => None,
        };
        Some(Annotation {
            event_key: self.event_key,
            recurrence_id: self.recurrence_id,
            mailbox: self.mailbox,
            kind,
            at,
            file: self.file,
            sha256: self.sha256,
            note: self.note,
        })
    }
}

pub struct AnnotationLog {
    path: PathBuf,
}

impl AnnotationLog {
    /// Refuses a path inside any mailbox root. The parent directory must
    /// already exist; it is never created, so a bad path cannot create a
    /// directory in a mirror.
    pub fn open(path: impl Into<PathBuf>, mirrors: &[MailboxRoot]) -> Result<Self, LogError> {
        let path = path.into();
        let parent = match path.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
            _ => PathBuf::from("."),
        };
        let parent = fs::canonicalize(parent)?;
        for m in mirrors {
            if let Ok(root) = fs::canonicalize(&m.root) {
                if parent.starts_with(&root) {
                    return Err(LogError::InsideMirror(root));
                }
            }
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// One line, one write, then fsync.
    pub fn append(&self, annotation: &Annotation) -> Result<(), LogError> {
        let mut line = serde_json::to_string(&Record::from_annotation(annotation))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        line.push('\n');
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        f.write_all(line.as_bytes())?;
        f.sync_all()?;
        Ok(())
    }

    /// A missing file reads as empty. Identical records collapse, so a
    /// retried append is harmless. Order is the order of first appearance.
    pub fn read(&self) -> Result<LogRead, LogError> {
        let bytes = match fs::read(&self.path) {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(LogRead::default()),
            Err(e) => return Err(e.into()),
        };
        let mut out = LogRead::default();
        let mut seen = BTreeSet::new();
        for raw in bytes.split(|b| *b == b'\n') {
            if raw.iter().all(|b| b.is_ascii_whitespace()) {
                continue;
            }
            let parsed = std::str::from_utf8(raw)
                .ok()
                .and_then(|s| serde_json::from_str::<Record>(s).ok())
                .and_then(Record::into_annotation);
            match parsed {
                Some(a) => {
                    if seen.insert(a.clone()) {
                        out.annotations.push(a);
                    }
                }
                None => out.skipped += 1,
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror_index::event_key;

    fn dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "annotation-log-{tag}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn full() -> Annotation {
        Annotation {
            event_key: event_key("uid-1"),
            recurrence_id: Some("20260101T100000Z".into()),
            mailbox: "a@example.test".into(),
            kind: AnnotationKind::Deleted,
            at: Some(
                DateTime::parse_from_rfc3339("2026-11-01T12:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            file: Some("calendar/ab/x.ics".into()),
            sha256: Some("ab".repeat(32)),
            note: "removed by client".into(),
        }
    }

    #[test]
    fn append_then_read_round_trips() {
        let d = dir("rt");
        let log = AnnotationLog::open(d.join("log.jsonl"), &[]).unwrap();
        let minimal = Annotation::for_uid(AnnotationKind::LeftServer, "uid-2", "b@example.test");
        log.append(&full()).unwrap();
        log.append(&minimal).unwrap();
        let r = log.read().unwrap();
        assert_eq!(r.skipped, 0);
        assert_eq!(r.annotations, vec![full(), minimal]);
    }

    #[test]
    fn duplicate_lines_collapse() {
        let d = dir("dup");
        let log = AnnotationLog::open(d.join("log.jsonl"), &[]).unwrap();
        log.append(&full()).unwrap();
        log.append(&full()).unwrap();
        assert_eq!(log.read().unwrap().annotations.len(), 1);
    }

    #[test]
    fn torn_last_line_is_skipped_and_counted() {
        let d = dir("torn");
        let log = AnnotationLog::open(d.join("log.jsonl"), &[]).unwrap();
        log.append(&full()).unwrap();
        let mut f = OpenOptions::new().append(true).open(log.path()).unwrap();
        f.write_all(b"{\"event_key\":\"abc\",\"mail").unwrap();
        let r = log.read().unwrap();
        assert_eq!(r.annotations, vec![full()]);
        assert_eq!(r.skipped, 1);
    }

    #[test]
    fn missing_file_reads_as_empty() {
        let d = dir("missing");
        let log = AnnotationLog::open(d.join("none.jsonl"), &[]).unwrap();
        assert_eq!(log.read().unwrap(), LogRead::default());
    }

    #[test]
    fn path_inside_a_mirror_root_is_refused() {
        let d = dir("inside");
        let root = d.join("mail");
        fs::create_dir_all(root.join("calendar")).unwrap();
        let mirrors = vec![MailboxRoot {
            mailbox: "a@example.test".into(),
            root: root.clone(),
        }];
        let r = AnnotationLog::open(root.join("calendar").join("log.jsonl"), &mirrors);
        assert!(matches!(r, Err(LogError::InsideMirror(_))));
        assert!(!root.join("calendar").join("log.jsonl").exists());
    }
}
