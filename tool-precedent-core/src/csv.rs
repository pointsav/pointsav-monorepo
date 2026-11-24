// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Minimal CSV: quoted fields, doubled quotes, newlines inside quotes, "\n" line ends.
pub fn parse(s: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut f = String::new();
    let mut q = false;
    let mut it = s.chars().peekable();
    let mut any = false;
    while let Some(c) = it.next() {
        any = true;
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
        } else if c == ',' {
            row.push(std::mem::take(&mut f));
        } else if c == '\n' {
            row.push(std::mem::take(&mut f));
            rows.push(std::mem::take(&mut row));
            any = false;
        } else if c == '\r' {
        } else {
            f.push(c);
        }
    }
    if any || !f.is_empty() || !row.is_empty() {
        row.push(f);
        rows.push(row);
    }
    rows
}
/// Open a register file for appending one row. Checks the header, creates the folder, writes the
/// header when the file is new or empty, and ends an unterminated last line with a newline first,
/// so a new row never fuses onto the previous one.
pub fn open_for_append(path: &std::path::Path, header: &str) -> Result<std::fs::File, String> {
    use std::io::{Read, Seek, SeekFrom, Write};
    check_header(path, header)?;
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
/// An existing shard file must carry the shared header before a row is appended to it
/// (an older file, for example one still keyed `doc_id`, needs its one-time re-key first).
pub fn check_header(path: &std::path::Path, expected: &str) -> Result<(), String> {
    let Ok(text) = std::fs::read_to_string(path) else {
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
