// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// Loader + validation for document-naming-taxonomy.csv — the controlled
// vocabulary behind service-input's file-naming/classification taxonomy
// (see data/document-naming-taxonomy.csv, BRIEF-service-input-file-naming-
// taxonomy.md for design history). Validation only — this module does not
// generate filenames; it answers "is this value a real vocabulary member,
// and on which axis." Same advisory posture as registry.rs: a missing or
// malformed file degrades validation to "nothing recognized," it never
// panics and never blocks ingest.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct TaxonomyEntry {
    pub axis: String,
    pub code: String,
    pub label: String,
    pub description_example: String,
    pub observed_in_practice: String,
    pub notes: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Taxonomy {
    pub entries: Vec<TaxonomyEntry>,
    pub warnings: Vec<String>,
}

const EXPECTED_COLUMNS: usize = 6;

/// The three axes that share one template field position ("DESK/WIKI/
/// MINUTEBOOK/PEOPLE" in the source taxonomy) — a value from any one of
/// them is valid there, but a consumer may care which one matched.
const FIRST_FIELD_AXES: [&str; 3] = ["desk", "people", "minutebook"];

pub fn load_document_naming_taxonomy(path: &str) -> Taxonomy {
    let mut warnings = Vec::new();

    let raw = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            warnings.push(format!("document-naming-taxonomy.csv not readable at {path}: {e}"));
            return Taxonomy {
                entries: Vec::new(),
                warnings,
            };
        }
    };

    let filtered: String = raw
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(filtered.as_bytes());

    let mut entries = Vec::new();
    for (i, result) in reader.records().enumerate() {
        let rec = match result {
            Ok(r) => r,
            Err(e) => {
                warnings.push(format!("row {}: parse error: {e}", i + 1));
                continue;
            }
        };
        if rec.len() != EXPECTED_COLUMNS {
            warnings.push(format!(
                "row {} ('{}'): expected {} columns, got {}",
                i + 1,
                rec.get(0).unwrap_or("?"),
                EXPECTED_COLUMNS,
                rec.len()
            ));
            continue;
        }
        entries.push(TaxonomyEntry {
            axis: rec.get(0).unwrap_or("").to_string(),
            code: rec.get(1).unwrap_or("").to_string(),
            label: rec.get(2).unwrap_or("").to_string(),
            description_example: rec.get(3).unwrap_or("").to_string(),
            observed_in_practice: rec.get(4).unwrap_or("").to_string(),
            notes: rec.get(5).unwrap_or("").to_string(),
        });
    }

    Taxonomy { entries, warnings }
}

impl Taxonomy {
    /// Case-sensitive membership check within one axis. The taxonomy's own
    /// values are deliberately mixed-case (e.g. "IT SUPPORT", "Agreement")
    /// and match the source convention verbatim — no normalization here,
    /// since a customer appending their own values in place should not have
    /// to guess a hidden case-folding rule.
    pub fn contains(&self, axis: &str, code: &str) -> bool {
        self.entries.iter().any(|e| e.axis == axis && e.code == code)
    }

    /// Which of the desk/people/minutebook axes (if any) a value belongs
    /// to. These three share one template field position in the filename
    /// — a value can only be a real member of one of them (the base
    /// vocabulary has no cross-axis duplicates), but this returns the
    /// first match rather than asserting uniqueness, since a customer's
    /// in-place edits could introduce one.
    pub fn classify_first_field(&self, value: &str) -> Option<String> {
        FIRST_FIELD_AXES
            .iter()
            .find(|axis| self.contains(axis, value))
            .map(|axis| axis.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct FilenameFieldsRequest {
    pub desk_or_people: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub status_suffix: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FilenameFieldsValidation {
    pub desk_or_people_valid: bool,
    pub desk_or_people_axis: Option<String>,
    pub type_valid: bool,
    pub status_suffix_valid: Option<bool>,
    pub warnings: Vec<String>,
}

/// Validate the non-Company fields of a proposed filename against the
/// taxonomy. The Company axis is deliberately out of scope here — it's
/// resolved through code-namespaces.csv (registry.rs), not this file.
pub fn validate_filename_fields(
    taxonomy: &Taxonomy,
    req: &FilenameFieldsRequest,
) -> FilenameFieldsValidation {
    let mut warnings = Vec::new();

    let desk_or_people_axis = taxonomy.classify_first_field(&req.desk_or_people);
    let desk_or_people_valid = desk_or_people_axis.is_some();
    if !desk_or_people_valid {
        warnings.push(format!(
            "'{}' is not a recognized desk/people/minutebook value",
            req.desk_or_people
        ));
    }

    let type_valid = taxonomy.contains("type", &req.type_);
    if !type_valid {
        warnings.push(format!("'{}' is not a recognized type value", req.type_));
    }

    let status_suffix_valid = req.status_suffix.as_ref().map(|s| {
        let valid = taxonomy.contains("status-suffix", s);
        if !valid {
            warnings.push(format!("'{s}' is not a recognized status-suffix value"));
        }
        valid
    });

    FilenameFieldsValidation {
        desk_or_people_valid,
        desk_or_people_axis,
        type_valid,
        status_suffix_valid,
        warnings,
    }
}

#[derive(Debug, Deserialize)]
pub struct GenerateFilenameRequest {
    pub desk_or_people: String,
    /// The Company axis is a caller-supplied value here, not resolved
    /// against any namespace — that resolution (real entity_code vs.
    /// Jennifer's frozen-historical numeric table vs. a plain display
    /// name) is a still-open design decision (see
    /// BRIEF-service-input-file-naming-taxonomy.md decisions open).
    /// This function only assembles what it's given.
    pub company: String,
    /// Expected shape: YYYY_MM_DD (validated, not enforced).
    pub date: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub initials: String,
    pub status_suffix: Option<String>,
    /// Real file extension, no leading dot (e.g. "pdf", "xlsx").
    pub extension: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateFilenameResult {
    pub filename: String,
    pub fields_validation: FilenameFieldsValidation,
    pub date_shape_valid: bool,
    pub delimiter_collision_warnings: Vec<String>,
}

fn is_valid_date_shape(date: &str) -> bool {
    let parts: Vec<&str> = date.split('_').collect();
    parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

/// Assemble a filename from the document-naming template:
/// `<desk-or-people>_<company>_<date>_<type>_<description>_<initials>[_<status-suffix>].<ext>`
///
/// Best-effort, never blocking: assembles the filename regardless of
/// whether individual fields validate against the taxonomy — the caller
/// decides what to do with an invalid result (same advisory posture as
/// the rest of this module). Field values are used verbatim, including
/// spaces, matching the real source convention (e.g. "Chart of
/// Accounts", "File names") — no case-folding or space-stripping.
pub fn generate_filename(taxonomy: &Taxonomy, req: &GenerateFilenameRequest) -> GenerateFilenameResult {
    let fields_validation = validate_filename_fields(
        taxonomy,
        &FilenameFieldsRequest {
            desk_or_people: req.desk_or_people.clone(),
            type_: req.type_.clone(),
            status_suffix: req.status_suffix.clone(),
        },
    );

    let date_shape_valid = is_valid_date_shape(&req.date);

    // The underscore is the field delimiter — a field value containing one
    // will silently corrupt which segment is which when a human (or a
    // future parser) splits the filename back apart. Advisory only, same
    // as every other check in this module.
    let mut delimiter_collision_warnings = Vec::new();
    let named_fields = [
        ("desk_or_people", &req.desk_or_people),
        ("company", &req.company),
        ("type", &req.type_),
        ("description", &req.description),
        ("initials", &req.initials),
    ];
    for (name, value) in named_fields {
        if value.contains('_') {
            delimiter_collision_warnings.push(format!(
                "field '{name}' contains an underscore, which is the filename delimiter: {value:?}"
            ));
        }
    }

    let mut filename = format!(
        "{}_{}_{}_{}_{}_{}",
        req.desk_or_people, req.company, req.date, req.type_, req.description, req.initials
    );
    if let Some(suffix) = &req.status_suffix {
        filename.push('_');
        filename.push_str(suffix);
    }
    filename.push('.');
    filename.push_str(&req.extension);

    GenerateFilenameResult {
        filename,
        fields_validation,
        date_shape_valid,
        delimiter_collision_warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_csv(contents: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().expect("tempfile");
        f.write_all(contents.as_bytes()).expect("write");
        f
    }

    fn sample_taxonomy() -> Taxonomy {
        let csv = "axis,code,label,description_example,observed_in_practice,notes\n\
                    desk,COMPLIANCE,COMPLIANCE,,,\n\
                    people,SHAREHOLDER,SHAREHOLDER,,,\n\
                    minutebook,MINUTEBOOK,MINUTEBOOK,,,\n\
                    type,Agreement,Agreement,,,\n\
                    status-suffix,FIN,Final draft,,yes,\n";
        let f = write_temp_csv(csv);
        load_document_naming_taxonomy(f.path().to_str().unwrap())
    }

    #[test]
    fn loads_real_shipped_taxonomy_without_warnings() {
        let t = load_document_naming_taxonomy(
            "/srv/foundry/clones/project-input/service-input/tests/fixtures/document-naming-taxonomy.csv",
        );
        assert_eq!(t.entries.len(), 77);
        assert!(t.warnings.is_empty(), "unexpected warnings: {:?}", t.warnings);
    }

    #[test]
    fn classifies_first_field_across_desk_people_minutebook() {
        let t = sample_taxonomy();
        assert_eq!(t.classify_first_field("COMPLIANCE"), Some("desk".into()));
        assert_eq!(t.classify_first_field("SHAREHOLDER"), Some("people".into()));
        assert_eq!(t.classify_first_field("MINUTEBOOK"), Some("minutebook".into()));
        assert_eq!(t.classify_first_field("NOT-A-REAL-VALUE"), None);
    }

    #[test]
    fn validates_full_filename_fields_all_valid() {
        let t = sample_taxonomy();
        let req = FilenameFieldsRequest {
            desk_or_people: "COMPLIANCE".into(),
            type_: "Agreement".into(),
            status_suffix: Some("FIN".into()),
        };
        let v = validate_filename_fields(&t, &req);
        assert!(v.desk_or_people_valid);
        assert_eq!(v.desk_or_people_axis, Some("desk".into()));
        assert!(v.type_valid);
        assert_eq!(v.status_suffix_valid, Some(true));
        assert!(v.warnings.is_empty());
    }

    #[test]
    fn validates_full_filename_fields_reports_each_invalid_field() {
        let t = sample_taxonomy();
        let req = FilenameFieldsRequest {
            desk_or_people: "NOT-REAL".into(),
            type_: "NOT-REAL-TYPE".into(),
            status_suffix: Some("XXX".into()),
        };
        let v = validate_filename_fields(&t, &req);
        assert!(!v.desk_or_people_valid);
        assert_eq!(v.desk_or_people_axis, None);
        assert!(!v.type_valid);
        assert_eq!(v.status_suffix_valid, Some(false));
        assert_eq!(v.warnings.len(), 3);
    }

    #[test]
    fn status_suffix_is_optional() {
        let t = sample_taxonomy();
        let req = FilenameFieldsRequest {
            desk_or_people: "COMPLIANCE".into(),
            type_: "Agreement".into(),
            status_suffix: None,
        };
        let v = validate_filename_fields(&t, &req);
        assert_eq!(v.status_suffix_valid, None);
        assert!(v.warnings.is_empty());
    }

    #[test]
    fn missing_file_is_advisory_not_fatal() {
        let t = load_document_naming_taxonomy("/nonexistent/path/document-naming-taxonomy.csv");
        assert!(t.entries.is_empty());
        assert_eq!(t.warnings.len(), 1);
    }

    #[test]
    fn generates_real_example_filename_verbatim() {
        // Matches the shape of the real example this taxonomy was sourced
        // from: "IT SUPPORT_PointSav_2026_01_06_Chart of Accounts_File
        // names_JW2.FIN" (spaces preserved, not stripped).
        let t = sample_taxonomy();
        let req = GenerateFilenameRequest {
            desk_or_people: "COMPLIANCE".into(),
            company: "PointSav".into(),
            date: "2026_01_06".into(),
            type_: "Agreement".into(),
            description: "File names".into(),
            initials: "JW2".into(),
            status_suffix: Some("FIN".into()),
            extension: "pdf".into(),
        };
        let result = generate_filename(&t, &req);
        assert_eq!(
            result.filename,
            "COMPLIANCE_PointSav_2026_01_06_Agreement_File names_JW2_FIN.pdf"
        );
        assert!(result.fields_validation.desk_or_people_valid);
        assert!(result.fields_validation.type_valid);
        assert_eq!(result.fields_validation.status_suffix_valid, Some(true));
        assert!(result.date_shape_valid);
        assert!(result.delimiter_collision_warnings.is_empty());
    }

    #[test]
    fn generates_filename_without_status_suffix() {
        let t = sample_taxonomy();
        let req = GenerateFilenameRequest {
            desk_or_people: "SHAREHOLDER".into(),
            company: "MCorp".into(),
            date: "2026_01_06".into(),
            type_: "Agreement".into(),
            description: "Notes".into(),
            initials: "JW1".into(),
            status_suffix: None,
            extension: "docx".into(),
        };
        let result = generate_filename(&t, &req);
        assert_eq!(result.filename, "SHAREHOLDER_MCorp_2026_01_06_Agreement_Notes_JW1.docx");
    }

    #[test]
    fn flags_invalid_date_shape_without_blocking_generation() {
        let t = sample_taxonomy();
        let req = GenerateFilenameRequest {
            desk_or_people: "COMPLIANCE".into(),
            company: "PointSav".into(),
            date: "01-06-2026".into(),
            type_: "Agreement".into(),
            description: "Notes".into(),
            initials: "JW1".into(),
            status_suffix: None,
            extension: "pdf".into(),
        };
        let result = generate_filename(&t, &req);
        assert!(!result.date_shape_valid);
        // still assembled — advisory only, never blocking
        assert!(result.filename.contains("01-06-2026"));
    }

    #[test]
    fn warns_on_underscore_inside_a_field_value() {
        let t = sample_taxonomy();
        let req = GenerateFilenameRequest {
            desk_or_people: "COMPLIANCE".into(),
            company: "Point_Sav".into(),
            date: "2026_01_06".into(),
            type_: "Agreement".into(),
            description: "Notes".into(),
            initials: "JW1".into(),
            status_suffix: None,
            extension: "pdf".into(),
        };
        let result = generate_filename(&t, &req);
        assert_eq!(result.delimiter_collision_warnings.len(), 1);
        assert!(result.delimiter_collision_warnings[0].contains("company"));
    }
}
