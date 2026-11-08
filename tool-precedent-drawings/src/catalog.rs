//! The drawing-catalog scan: forked from `tool-documents-core`'s
//! `catalog.rs` (business-admin/project-documents/tool-documents-core/
//! src/catalog.rs) -- same shape (scan `inputs/`/`outputs/` under a
//! runtime-provided data root, group by version-stripped stem, guess a
//! category, emit a CSV row per group), adapted for drawing-type files.
//! Unlike the document census, a drawing's own content is structurally
//! inspectable -- for every `.ifc` file found, this module also calls
//! `crate::ifc::parse_entity_counts` and folds the result into the row, a
//! real capability the document-only version has no equivalent of.
//!
//! "Code in engineering, data in place": `data_root` is always supplied by
//! the caller at runtime (an env var's value, a CLI arg) -- never a
//! compiled-in constant pointing into `business-admin/`.

use std::collections::BTreeMap;
use std::path::Path;
use tool_precedent_core::csv;

const KNOWN_CATEGORIES: &[&str] = &[
    "KeyPlan",
    "Tile",
    "FloorPlate",
    "SitePlan",
    "Elevation",
    "Section",
    "Detail",
];

const VERSION_SUFFIXES: &[&str] = &["_JW", "_MW", "_V", "_rev"];

const DRAWING_EXTENSIONS: &[&str] = &["ifc", "fcstd"];

#[derive(Debug, Default, Clone)]
pub struct DrawingRow {
    pub drawing: String,
    pub category: String,
    pub sample_count: usize,
    pub sample_file: String,
    /// Entity-type counts from the sample file, if it was a parseable
    /// `.ifc` file -- empty for `.fcstd`-only groups (native FreeCAD files
    /// are not parsed by this crate; see `src/ifc.rs`'s header comment).
    pub entity_counts: BTreeMap<String, usize>,
}

/// Scans `outputs/` and `inputs/` under `data_root` and groups files by
/// their version-stripped stem, same two-subdirectory shape
/// `tool-documents-core::catalog::scan` uses.
pub fn scan(data_root: &Path) -> Vec<DrawingRow> {
    let mut groups: BTreeMap<String, (String, usize, String)> = BTreeMap::new();

    for sub in ["outputs", "inputs"] {
        let dir = data_root.join(sub);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
                continue;
            };
            let ext_lower = ext.to_ascii_lowercase();
            if !DRAWING_EXTENSIONS.contains(&ext_lower.as_str()) {
                continue;
            }
            let stripped = strip_version_suffix(stem);
            let category = guess_category(stripped);
            let entry = groups
                .entry(stripped.to_string())
                .or_insert_with(|| (category.clone(), 0, path.to_string_lossy().into_owned()));
            entry.1 += 1;
        }
    }

    groups
        .into_iter()
        .map(|(drawing, (category, sample_count, sample_file))| {
            let entity_counts = if sample_file.to_ascii_lowercase().ends_with(".ifc") {
                std::fs::read_to_string(&sample_file)
                    .ok()
                    .and_then(|src| crate::ifc::parse_entity_counts(&src).ok())
                    .unwrap_or_default()
            } else {
                BTreeMap::new()
            };
            DrawingRow {
                drawing,
                category,
                sample_count,
                sample_file,
                entity_counts,
            }
        })
        .collect()
}

fn strip_version_suffix(stem: &str) -> &str {
    let mut s = stem;
    loop {
        let mut cut = None;
        for suffix in VERSION_SUFFIXES {
            if let Some(pos) = s.rfind(suffix) {
                let after = &s[pos + suffix.len()..];
                if after.is_empty() || after.chars().all(|c| c.is_ascii_digit()) {
                    cut = Some(cut.map_or(pos, |c: usize| c.max(pos)));
                }
            }
        }
        match cut {
            Some(pos) => s = &s[..pos],
            None => break,
        }
    }
    s
}

fn guess_category(stem: &str) -> String {
    for cat in KNOWN_CATEGORIES {
        if stem.contains(cat) {
            return cat.to_string();
        }
    }
    "Uncategorized".to_string()
}

pub fn write_csv(rows: &[DrawingRow], out_path: &Path) -> std::io::Result<()> {
    let mut out = String::from("drawing,category,sample_count,sample_file,entity_counts\n");
    for row in rows {
        let entity_summary = row
            .entity_counts
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("|");
        out.push_str(&format!(
            "{},{},{},{},{}\n",
            csv::field(&row.drawing),
            csv::field(&row.category),
            row.sample_count,
            csv::field(&row.sample_file),
            csv::field(&entity_summary),
        ));
    }
    std::fs::write(out_path, out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("tool-precedent-drawings-test-{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("inputs")).unwrap();
        fs::create_dir_all(dir.join("outputs")).unwrap();
        dir
    }

    #[test]
    fn groups_files_by_version_stripped_stem_and_counts_samples() {
        let dir = temp_dir("groups");
        fs::write(
            dir.join("inputs/SiteA-KeyPlan_V1.ifc"),
            "DATA;\n#1=IFCWALL('g',$,$);\nENDSEC;",
        )
        .unwrap();
        fs::write(
            dir.join("inputs/SiteA-KeyPlan_V2.ifc"),
            "DATA;\n#1=IFCWALL('g',$,$);\nENDSEC;",
        )
        .unwrap();
        fs::write(
            dir.join("outputs/SiteB-FloorPlate.fcstd"),
            "not a real fcstd, fixture only",
        )
        .unwrap();

        let rows = scan(&dir);
        assert_eq!(rows.len(), 2);

        let key_plan = rows.iter().find(|r| r.drawing == "SiteA-KeyPlan").unwrap();
        assert_eq!(key_plan.sample_count, 2);
        assert_eq!(key_plan.category, "KeyPlan");

        let floor_plate = rows
            .iter()
            .find(|r| r.drawing == "SiteB-FloorPlate")
            .unwrap();
        assert_eq!(floor_plate.sample_count, 1);
        assert_eq!(floor_plate.category, "FloorPlate");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn attaches_entity_counts_only_for_parseable_ifc_samples() {
        let dir = temp_dir("entities");
        fs::write(
            dir.join("inputs/Detail-A.ifc"),
            "DATA;\n#1=IFCWALL('g',$,$);\n#2=IFCDOOR('g2',$,$);\nENDSEC;",
        )
        .unwrap();
        fs::write(
            dir.join("inputs/Detail-B.fcstd"),
            "not parsed by this crate",
        )
        .unwrap();

        let rows = scan(&dir);
        let ifc_row = rows.iter().find(|r| r.drawing == "Detail-A").unwrap();
        assert_eq!(ifc_row.entity_counts.get("IFCWALL"), Some(&1));
        assert_eq!(ifc_row.entity_counts.get("IFCDOOR"), Some(&1));

        let fcstd_row = rows.iter().find(|r| r.drawing == "Detail-B").unwrap();
        assert!(fcstd_row.entity_counts.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_data_root_yields_an_empty_catalog_not_an_error() {
        let rows = scan(Path::new("/nonexistent/path/for/this/test"));
        assert!(rows.is_empty());
    }

    #[test]
    fn write_csv_round_trips_a_row_with_a_comma_in_its_sample_file() {
        let dir = temp_dir("csv");
        let row = DrawingRow {
            drawing: "Sample".to_string(),
            category: "Detail".to_string(),
            sample_count: 1,
            sample_file: "has, a comma.ifc".to_string(),
            entity_counts: BTreeMap::new(),
        };
        let out_path = dir.join("catalog.csv");
        write_csv(&[row], &out_path).unwrap();
        let content = fs::read_to_string(&out_path).unwrap();
        assert!(content.contains("\"has, a comma.ifc\""));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_csv_quotes_a_newline_in_a_field() {
        let dir = temp_dir("csv-newline");
        let row = DrawingRow {
            drawing: "plan".into(),
            category: "Uncategorized".into(),
            sample_count: 1,
            sample_file: "a\nb.ifc".into(),
            entity_counts: BTreeMap::new(),
        };
        let out_path = dir.join("catalog.csv");
        write_csv(&[row], &out_path).unwrap();
        let text = fs::read_to_string(&out_path).unwrap();
        let rows = csv::parse(&text);
        assert_eq!(
            rows.len(),
            2,
            "header plus one row, not split by the newline"
        );
        assert_eq!(rows[1][3], "a\nb.ifc");
        let _ = fs::remove_dir_all(&dir);
    }
}
