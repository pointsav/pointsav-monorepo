//! The version lineage (`precedent/versions.csv`): every `<base>_JW<N>.html` in a folder.
use std::collections::BTreeMap;
use std::path::Path;

/// Scans `<root>/<dir>` for `<base>_JW<N>.html` and returns `(base, JW<N>, file, pdf_present)`, ordered by base then N.
pub fn scan_versions(root: &Path, dir: &str) -> Vec<(String, String, String, bool)> {
    let dir = root.join(dir);
    let mut found: BTreeMap<(String, u32), (String, bool)> = BTreeMap::new();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("html") {
            continue;
        }
        let Some(stem) = p.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(i) = stem.rfind("_JW") else { continue };
        let Ok(n) = stem[i + 3..].parse::<u32>() else {
            continue;
        };
        let pdf = dir.join(format!("{stem}.pdf")).exists();
        found.insert((stem[..i].to_string(), n), (format!("{stem}.html"), pdf));
    }
    found
        .into_iter()
        .map(|((b, n), (f, p))| (b, format!("JW{n}"), f, p))
        .collect()
}

pub fn write_versions(rows: &[(String, String, String, bool)], out: &Path) -> std::io::Result<()> {
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut s = String::from("instrument,version,file,pdf_present\n");
    for (b, v, f, p) in rows {
        s.push_str(&format!(
            "{},{},{},{}\n",
            crate::csv::field(b),
            v,
            crate::csv::field(f),
            p
        ));
    }
    std::fs::write(out, s)
}
