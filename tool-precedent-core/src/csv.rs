// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Minimal CSV: quoted fields, doubled quotes, newlines inside quotes, "\n" line ends.
//! Input boundary (Foundry Rust Profile rule 10): `read_text` caps every file at `MAX_FILE_BYTES`;
//! `parse_records` refuses an unbalanced quote; `check_widths` reports ragged rows.

/// The largest file any reader of this crate accepts (64 MiB). Larger files are refused with an
/// `std::io::ErrorKind::FileTooLarge` error before they are read into memory.
pub const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;

/// Reads a UTF-8 text file of at most `MAX_FILE_BYTES` (same errors as `read_to_string` otherwise).
pub fn read_text(path: &std::path::Path) -> std::io::Result<String> {
    read_text_capped(path, MAX_FILE_BYTES)
}

/// `read_text` with an explicit cap. Refuses on the size the file system reports, and again on the
/// bytes actually read, so a file that grows while it is read is still bounded.
pub fn read_text_capped(path: &std::path::Path, cap: u64) -> std::io::Result<String> {
    use std::io::{Error, ErrorKind, Read};
    let f = std::fs::File::open(path)?;
    let too_large = || {
        Error::new(
            ErrorKind::FileTooLarge,
            format!("the file is larger than the input limit of {cap} bytes"),
        )
    };
    if f.metadata()?.len() > cap {
        return Err(too_large());
    }
    let mut buf = Vec::new();
    f.take(cap.saturating_add(1)).read_to_end(&mut buf)?;
    if buf.len() as u64 > cap {
        return Err(too_large());
    }
    String::from_utf8(buf)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8"))
}

/// One record: the 1-based line it starts on, and its fields.
pub type Record = (usize, Vec<String>);

/// The records, and the line of a quoted field still open at the end of the input (if any).
fn scan(s: &str) -> (Vec<Record>, Option<usize>) {
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut f = String::new();
    let mut q = false;
    let mut it = s.chars().peekable();
    let mut any = false;
    let (mut line, mut row_line, mut q_line) = (1usize, 1usize, 0usize);
    while let Some(c) = it.next() {
        any = true;
        if c == '\n' {
            line += 1;
        }
        if q {
            if c == '"' {
                if it.peek() == Some(&'"') {
                    it.next();
                    f.push('"');
                } else {
                    q = false;
                }
            } else {
                f.push(c);
            }
        } else if c == '"' {
            q = true;
            q_line = line;
        } else if c == ',' {
            row.push(std::mem::take(&mut f));
        } else if c == '\n' {
            row.push(std::mem::take(&mut f));
            rows.push((row_line, std::mem::take(&mut row)));
            row_line = line;
            any = false;
        } else if c == '\r' {
        } else {
            f.push(c);
        }
    }
    if any || !f.is_empty() || !row.is_empty() {
        row.push(f);
        rows.push((row_line, row));
    }
    (rows, q.then_some(q_line))
}

/// Lenient parse: an unbalanced quote runs to the end of the input. Kept for callers that only
/// look things up; the register readers use `parse_records`.
pub fn parse(s: &str) -> Vec<Vec<String>> {
    scan(s).0.into_iter().map(|r| r.1).collect()
}

/// Strict parse with the line each record starts on; an unbalanced quote is an error naming the
/// line the quoted field opened on.
pub fn parse_records(s: &str) -> Result<Vec<Record>, String> {
    match scan(s) {
        (_, Some(l)) => Err(format!(
            "line {l}: a quoted field opens here and is never closed (unbalanced double quote)"
        )),
        (rows, None) => Ok(rows),
    }
}

/// Refuses ragged rows: every record whose field count fails `ok` is counted, and the first one
/// is named by line. `expected` is the wording of the allowed count, for the message.
pub fn check_widths(
    rows: &[Record],
    expected: &str,
    ok: impl Fn(usize) -> bool,
) -> Result<(), String> {
    let mut bad = rows.iter().filter(|r| !ok(r.1.len()));
    let Some((line, first)) = bad.next() else {
        return Ok(());
    };
    let n = 1 + bad.count();
    Err(format!(
        "{n} {} the wrong number of fields (expected {expected}); the first is line {line} with {}",
        if n == 1 { "row has" } else { "rows have" },
        first.len()
    ))
}

/// Open a register file for appending one row. Checks the header, creates the folder, writes the
/// header when the file is new or empty, and ends an unterminated last line with a newline first,
/// so a new row never fuses onto the previous one.
pub fn open_for_append(path: &std::path::Path, header: &str) -> Result<std::fs::File, String> {
    use std::io::{Read, Seek, SeekFrom, Write};
    check_header(path, header)?;
    if let Some(text) = existing(path)? {
        parse_records(&text)
            .map_err(|e| format!("{}: {e}; fix the file before adding rows", path.display()))?;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let len = f.metadata().map_err(|e| e.to_string())?.len();
    if len == 0 {
        writeln!(f, "{header}").map_err(|e| e.to_string())?;
    } else {
        let mut last = [0u8; 1];
        f.seek(SeekFrom::End(-1)).map_err(|e| e.to_string())?;
        f.read_exact(&mut last).map_err(|e| e.to_string())?;
        if last[0] != b'\n' {
            writeln!(f).map_err(|e| e.to_string())?;
        }
    }
    Ok(f)
}
/// Replace (or create) `path` in one step: write a temporary file beside it, flush it to disk, then
/// rename it over the target, so a crash or a full disk never leaves a half-written file. The
/// permissions of an existing target are kept.
pub fn write_file_atomic(path: &std::path::Path, content: &str) -> std::io::Result<()> {
    use std::io::Write;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let tmp = path.with_file_name(format!(".{name}.tmp.{}", std::process::id()));
    let res = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(content.as_bytes())?;
        f.sync_all()?;
        if let Ok(m) = std::fs::metadata(path) {
            std::fs::set_permissions(&tmp, m.permissions())?;
        }
        std::fs::rename(&tmp, path)
    })();
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res
}
/// An existing shard file must carry the shared header before a row is appended to it
/// (an older file, for example one still keyed `doc_id`, needs its one-time re-key first).
pub fn check_header(path: &std::path::Path, expected: &str) -> Result<(), String> {
    let Some(text) = existing(path)? else {
        return Ok(());
    };
    let first = text.lines().next().unwrap_or("").trim_end_matches('\r');
    if first.is_empty() || first == expected {
        Ok(())
    } else {
        Err(format!(
            "{}: the header is not the shared one ({expected}); re-key the file before adding rows",
            path.display()
        ))
    }
}
/// The text of an existing file, `None` when there is no file yet; any other failure (too large,
/// not UTF-8, unreadable) is an error, so nothing is appended to a file that cannot be read.
fn existing(path: &std::path::Path) -> Result<Option<String>, String> {
    match read_text(path) {
        Ok(t) => Ok(Some(t)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}
pub fn field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
pub fn write(rows: &[Vec<String>]) -> String {
    let mut o = String::new();
    for r in rows {
        o.push_str(&r.iter().map(|x| field(x)).collect::<Vec<_>>().join(","));
        o.push('\n');
    }
    o
}
