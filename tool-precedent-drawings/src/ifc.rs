//! A minimal, real ISO-10303-21 ("STEP physical file") entity-type counter
//! for FreeCAD's exported IFC files (operator decision, 2026-10-02: this
//! crate parses FreeCAD's **exported IFC** interchange format, not its
//! native `.FCStd`).
//!
//! Deliberately not a full IFC schema parser -- IFC4's EXPRESS schema has
//! hundreds of entity types and a real geometry-resolution layer (placement
//! trees, representation contexts) that this crate does not need yet. What
//! it does need, and does for real: tokenize the `DATA;` section's instance
//! lines (`#<id>=<ENTITY-TYPE>(<params>);`) and count occurrences per
//! entity type -- enough for the catalog module's shard record to say
//! "this drawing has N walls, M doors" without guessing from a filename.
//!
//! Zero third-party dependencies, matching every `tool-precedent-*`
//! sibling's stated bar.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfcParseError {
    pub message: String,
}

impl fmt::Display for IfcParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Entity-type counts from one IFC file's `DATA;` section, keyed by the
/// entity keyword exactly as it appears in the file (e.g. `IFCWALL`,
/// `IFCDOOR`) -- deterministic order via `BTreeMap`.
pub fn parse_entity_counts(source: &str) -> Result<BTreeMap<String, usize>, IfcParseError> {
    let stripped = strip_step_comments(source);

    let data_start = stripped.find("DATA;").ok_or_else(|| IfcParseError {
        message: "no DATA; section found -- not a well-formed STEP physical file".to_string(),
    })?;
    let after_data = &stripped[data_start + "DATA;".len()..];
    let data_end = after_data.find("ENDSEC;").unwrap_or(after_data.len());
    let data_section = &after_data[..data_end];

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for statement in data_section.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        if let Some(entity_type) = extract_entity_type(statement) {
            *counts.entry(entity_type).or_insert(0) += 1;
        }
    }

    if counts.is_empty() {
        return Err(IfcParseError {
            message: "DATA; section present but no #id=ENTITY(...) instances recognized"
                .to_string(),
        });
    }
    Ok(counts)
}

/// `#123=IFCWALL(...)` -> `Some("IFCWALL")`. Returns `None` for a statement
/// that doesn't match the `#<digits>=<UPPERCASE-IDENT>(` shape (simple
/// values, malformed fragments at section boundaries, etc.).
fn extract_entity_type(statement: &str) -> Option<String> {
    let rest = statement.strip_prefix('#')?;
    let eq_pos = rest.find('=')?;
    let (id_part, after_eq) = (&rest[..eq_pos], &rest[eq_pos + 1..]);
    if id_part.is_empty() || !id_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let after_eq = after_eq.trim_start();
    let paren_pos = after_eq.find('(')?;
    let entity_type = after_eq[..paren_pos].trim();
    if entity_type.is_empty()
        || !entity_type
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return None;
    }
    Some(entity_type.to_string())
}

/// Strips `/* ... */` block comments, which real-world STEP files
/// (including FreeCAD's exporter) use freely.
fn strip_step_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            while let Some(c2) = chars.next() {
                if c2 == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../tests/fixtures/sample.ifc");

    #[test]
    fn counts_each_entity_type_in_the_data_section() {
        let counts = parse_entity_counts(SAMPLE).unwrap();
        assert_eq!(counts.get("IFCWALL"), Some(&2));
        assert_eq!(counts.get("IFCDOOR"), Some(&1));
        assert_eq!(counts.get("IFCPROJECT"), Some(&1));
    }

    #[test]
    fn strips_block_comments_before_counting() {
        let src =
            "DATA;\n/* a comment with #999=IFCNOISE(); inside it */\n#1=IFCWALL('g',$,$);\nENDSEC;";
        let counts = parse_entity_counts(src).unwrap();
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get("IFCWALL"), Some(&1));
        assert!(!counts.contains_key("IFCNOISE"));
    }

    #[test]
    fn rejects_a_file_with_no_data_section() {
        let err = parse_entity_counts("HEADER;\nENDSEC;").unwrap_err();
        assert!(err.message.contains("DATA;"));
    }

    #[test]
    fn rejects_a_data_section_with_no_recognizable_instances() {
        let err = parse_entity_counts("DATA;\njust some noise\nENDSEC;").unwrap_err();
        assert!(err.message.contains("no #id=ENTITY"));
    }

    #[test]
    fn ignores_non_instance_statements_between_real_ones() {
        let src = "DATA;\n#1=IFCWALL('g',$,$);\n$;\n#2=IFCDOOR('g2',$,$);\nENDSEC;";
        let counts = parse_entity_counts(src).unwrap();
        assert_eq!(counts.len(), 2);
    }
}
