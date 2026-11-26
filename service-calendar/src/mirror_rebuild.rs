// SPDX-License-Identifier: Apache-2.0

//! Rebuilds the mirror index from the mirror's files. Reads the
//! mirror; never writes into it. The index file is replaced
//! atomically (temporary file, fsync, rename).

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use worm_ledger::blob::sha256_hex;

use crate::mirror_index::{
    parse_csv, parse_mirror_item, Annotation, MirrorIndex, MirrorItem, Problem, ProblemKind,
};

/// One mailbox of the mirror: the name used in index rows and the
/// directory that holds its `calendar/` tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailboxRoot {
    pub mailbox: String,
    pub root: PathBuf,
}

/// Per-item facts that neither the file name nor the `.ics` bytes
/// carry. The mirror's calendar sidecar (organizer, attendees,
/// response status) and the source of `change_seq` and the folder
/// flag are not fixed yet; until they are, [`NoSidecar`] supplies
/// nothing and those columns stay empty.
pub trait MirrorSidecar {
    fn change_seq(&self, _mailbox: &MailboxRoot, _file: &str) -> Option<u64> {
        None
    }

    fn folder_flag(&self, _mailbox: &MailboxRoot, _file: &str) -> Option<String> {
        None
    }
}

/// The default: no sidecar is read.
pub struct NoSidecar;

impl MirrorSidecar for NoSidecar {}

/// Reads `change_seq` from the mirror's `calendar.csv`, one file per
/// mailbox, loaded once. Columns are found by header name (`file`,
/// `change_seq`, and `sha256` when present), never by position. A row
/// is used only when it is whole, its `change_seq` is a number, its
/// `sha256` (if the column exists) equals the file name's hash, and
/// every row for the same file agrees. Anything else gives `None` for
/// that file, and a line in [`reasons`](Self::reasons). What
/// `change_seq` counts (per mailbox, per folder, whether it resets on
/// a resync) is not confirmed: the index only uses it to break ties
/// within one mailbox, after SEQUENCE, LAST-MODIFIED and DTSTAMP.
#[derive(Debug, Default)]
pub struct CalendarCsvSidecar {
    /// mailbox -> file -> change_seq; `None` marks a conflicted file.
    seqs: HashMap<String, HashMap<String, Option<u64>>>,
    reasons: Vec<String>,
}

impl CalendarCsvSidecar {
    /// `csvs` pairs each mailbox name with its `calendar.csv` path. A
    /// file that cannot be read is reported, not fatal.
    pub fn load(csvs: &[(String, PathBuf)]) -> Self {
        let mut me = Self::default();
        for (mailbox, path) in csvs {
            match fs::read(path) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(text) => me.add_text(mailbox, &text),
                    Err(_) => me
                        .reasons
                        .push(format!("{mailbox}: calendar.csv is not UTF-8")),
                },
                Err(e) => me
                    .reasons
                    .push(format!("{mailbox}: calendar.csv not read: {e}")),
            }
        }
        me
    }

    /// Adds one mailbox's `calendar.csv` text.
    pub fn add_text(&mut self, mailbox: &str, text: &str) {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let rows = match parse_csv(text) {
            Ok(r) => r,
            Err(e) => {
                self.reasons.push(format!("{mailbox}: calendar.csv: {e}"));
                return;
            }
        };
        let Some((header, rows)) = rows.split_first() else {
            self.reasons
                .push(format!("{mailbox}: calendar.csv has no header"));
            return;
        };
        let col = |name: &str| header.iter().position(|h| h == name);
        let (Some(file_col), Some(seq_col)) = (col("file"), col("change_seq")) else {
            self.reasons.push(format!(
                "{mailbox}: calendar.csv has no `file` or no `change_seq` column"
            ));
            return;
        };
        let sha_col = col("sha256");
        let (mut short, mut unreadable, mut conflicts, mut mismatches) = (0, 0, 0, 0);
        let map = self.seqs.entry(mailbox.to_string()).or_default();
        let mut bad_files = BTreeSet::new();
        for row in rows {
            if row.len() != header.len() {
                short += 1;
                continue;
            }
            let file = row[file_col].as_str();
            let stem = file
                .rsplit('/')
                .next()
                .and_then(|n| n.strip_suffix(".ics"))
                .unwrap_or("");
            if let Some(c) = sha_col {
                if row[c] != stem {
                    mismatches += 1;
                    bad_files.insert(file.to_string());
                    continue;
                }
            }
            let cell = row[seq_col].trim();
            if cell.is_empty() {
                continue;
            }
            let Ok(seq) = cell.parse::<u64>() else {
                unreadable += 1;
                continue;
            };
            match map.get(file) {
                Some(Some(prev)) if *prev != seq => {
                    conflicts += 1;
                    map.insert(file.to_string(), None);
                }
                Some(_) => {}
                None => {
                    map.insert(file.to_string(), Some(seq));
                }
            }
        }
        for f in bad_files {
            map.insert(f, None);
        }
        for (n, what) in [
            (short, "rows with the wrong number of fields skipped"),
            (unreadable, "rows with a non-numeric change_seq skipped"),
            (
                conflicts,
                "rows disagreeing with an earlier row for the same file",
            ),
            (mismatches, "rows whose sha256 does not match the file name"),
        ] {
            if n > 0 {
                self.reasons.push(format!("{mailbox}: {n} {what}"));
            }
        }
    }

    /// Why anything was skipped or refused, one line each.
    pub fn reasons(&self) -> &[String] {
        &self.reasons
    }
}

impl MirrorSidecar for CalendarCsvSidecar {
    fn change_seq(&self, mailbox: &MailboxRoot, file: &str) -> Option<u64> {
        self.seqs
            .get(&mailbox.mailbox)?
            .get(file)
            .copied()
            .flatten()
    }
}

#[derive(Debug, Default)]
pub struct RebuildReport {
    pub files_indexed: usize,
    pub components: usize,
    pub annotations: usize,
    /// Files or VEVENTs that were skipped, and why.
    pub problems: Vec<Problem>,
}

#[derive(Debug)]
pub enum RebuildError {
    Io(std::io::Error),
    /// The index path lies inside a mirror root; refusing to write.
    IndexInsideMirror(PathBuf),
    DuplicateMailbox(String),
}

impl std::fmt::Display for RebuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RebuildError::Io(e) => write!(f, "index write failed: {e}"),
            RebuildError::IndexInsideMirror(p) => {
                write!(f, "index path is inside the mirror at {}", p.display())
            }
            RebuildError::DuplicateMailbox(m) => write!(f, "mailbox {m:?} is listed twice"),
        }
    }
}

impl std::error::Error for RebuildError {}

impl From<std::io::Error> for RebuildError {
    fn from(e: std::io::Error) -> Self {
        RebuildError::Io(e)
    }
}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Collects every file under `dir`, recursively, as paths relative to
/// `base`, sorted. Symlinks are not followed.
fn walk(base: &Path, dir: &Path, out: &mut Vec<String>, problems: &mut Vec<(String, String)>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => {
            let rel = dir
                .strip_prefix(base)
                .unwrap_or(dir)
                .to_string_lossy()
                .into_owned();
            problems.push((rel, e.to_string()));
            return;
        }
    };
    let mut paths: Vec<(PathBuf, fs::FileType)> = Vec::new();
    for entry in entries {
        match entry.and_then(|e| Ok((e.path(), e.file_type()?))) {
            Ok(p) => paths.push(p),
            Err(e) => problems.push((dir.to_string_lossy().into_owned(), e.to_string())),
        }
    }
    paths.sort_by(|a, b| a.0.cmp(&b.0));
    for (path, file_type) in paths {
        if file_type.is_dir() {
            walk(base, &path, out, problems);
        } else if file_type.is_file() {
            if let Ok(rel) = path.strip_prefix(base) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

/// Scans one mailbox's `calendar/` tree. A file whose name is not the
/// SHA-256 of its bytes, or whose path breaks the layout, is reported
/// and skipped; files that do not end in `.ics` are ignored.
pub fn scan_mailbox(
    mailbox: &MailboxRoot,
    sidecar: &dyn MirrorSidecar,
) -> (Vec<MirrorIndexInput>, Vec<Problem>) {
    let mut files = Vec::new();
    let mut walk_problems = Vec::new();
    walk(
        &mailbox.root,
        &mailbox.root.join("calendar"),
        &mut files,
        &mut walk_problems,
    );

    let mut problems: Vec<Problem> = walk_problems
        .into_iter()
        .map(|(file, e)| Problem {
            mailbox: mailbox.mailbox.clone(),
            file,
            kind: ProblemKind::Io(e),
        })
        .collect();
    let mut inputs = Vec::new();
    for file in files {
        if !file.ends_with(".ics") {
            continue;
        }
        let problem = |kind| Problem {
            mailbox: mailbox.mailbox.clone(),
            file: file.clone(),
            kind,
        };
        let parts: Vec<&str> = file.split('/').collect();
        let (prefix, stem) = match parts.as_slice() {
            ["calendar", prefix, name] => match name.strip_suffix(".ics") {
                Some(stem) if is_lower_hex(prefix, 2) && is_lower_hex(stem, 64) => (*prefix, stem),
                _ => {
                    problems.push(problem(ProblemKind::BadPath));
                    continue;
                }
            },
            _ => {
                problems.push(problem(ProblemKind::BadPath));
                continue;
            }
        };
        if !stem.starts_with(prefix) {
            problems.push(problem(ProblemKind::WrongPrefixDir));
            continue;
        }
        let bytes = match fs::read(mailbox.root.join(&file)) {
            Ok(b) => b,
            Err(e) => {
                problems.push(problem(ProblemKind::Io(e.to_string())));
                continue;
            }
        };
        let actual = sha256_hex(&bytes);
        if actual != stem {
            problems.push(problem(ProblemKind::NameMismatch {
                actual_sha256: actual,
            }));
            continue;
        }
        let item = MirrorItem {
            mailbox: mailbox.mailbox.clone(),
            change_seq: sidecar.change_seq(mailbox, &file),
            folder_flag: sidecar.folder_flag(mailbox, &file),
            sha256: actual,
            file,
        };
        inputs.push(MirrorIndexInput { item, bytes });
    }
    (inputs, problems)
}

/// One verified mirror file, ready to parse.
pub struct MirrorIndexInput {
    pub item: MirrorItem,
    pub bytes: Vec<u8>,
}

/// Scans every mailbox and builds the index in memory, without
/// writing anything.
pub fn build_from_mirror(
    mailboxes: &[MailboxRoot],
    annotations: &[Annotation],
    sidecar: &dyn MirrorSidecar,
) -> Result<(MirrorIndex, RebuildReport), RebuildError> {
    for (i, m) in mailboxes.iter().enumerate() {
        if mailboxes[..i].iter().any(|o| o.mailbox == m.mailbox) {
            return Err(RebuildError::DuplicateMailbox(m.mailbox.clone()));
        }
    }
    let mut report = RebuildReport::default();
    let mut components = Vec::new();
    for mailbox in mailboxes {
        let (inputs, problems) = scan_mailbox(mailbox, sidecar);
        report.problems.extend(problems);
        for input in inputs {
            let (records, problems) = parse_mirror_item(&input.bytes, &input.item);
            report.problems.extend(problems);
            if !records.is_empty() {
                report.files_indexed += 1;
            }
            components.extend(records);
        }
    }
    let index = MirrorIndex::build(components, annotations.to_vec());
    report.components = index.components.len();
    report.annotations = index.annotations.len();
    Ok((index, report))
}

/// Rebuilds the index at `index_path` from the mirror. Annotation rows
/// are not derivable from the mirror, so the caller passes them in.
pub fn rebuild_from_mirror(
    mailboxes: &[MailboxRoot],
    annotations: &[Annotation],
    sidecar: &dyn MirrorSidecar,
    index_path: &Path,
) -> Result<RebuildReport, RebuildError> {
    refuse_index_inside_mirror(mailboxes, index_path)?;
    let (index, report) = build_from_mirror(mailboxes, annotations, sidecar)?;
    write_atomically(index_path, index.to_csv().as_bytes())?;
    Ok(report)
}

fn refuse_index_inside_mirror(
    mailboxes: &[MailboxRoot],
    index_path: &Path,
) -> Result<(), RebuildError> {
    let parent = match index_path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
        _ => PathBuf::from("."),
    };
    let parent = fs::canonicalize(parent)?;
    for m in mailboxes {
        if let Ok(root) = fs::canonicalize(&m.root) {
            if parent.starts_with(&root) {
                return Err(RebuildError::IndexInsideMirror(root));
            }
        }
    }
    Ok(())
}

fn write_atomically(path: &Path, bytes: &[u8]) -> Result<(), RebuildError> {
    let name = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "index path has no file name",
        )
    })?;
    let tmp = path.with_file_name(format!(
        ".{}.tmp-{}",
        name.to_string_lossy(),
        std::process::id()
    ));
    let result = (|| {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        fs::rename(&tmp, path)
    })();
    if let Err(e) = result {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    // Persist the rename itself; not every platform allows opening a
    // directory for sync, so a failure here is not an error.
    if let Some(dir) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        if let Ok(d) = fs::File::open(dir) {
            let _ = d.sync_all();
        }
    }
    Ok(())
}
