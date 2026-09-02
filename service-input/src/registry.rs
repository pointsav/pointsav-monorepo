// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// Loader for code-namespaces.csv — the cross-archive short-code namespace
// registry (see data/code-namespaces.csv, BRIEF-service-input-file-naming-
// taxonomy.md for design history). Structural invariant this loader must
// preserve: the registry is namespace-level metadata only — it never
// carries per-code data — and it is advisory only. A missing, unreadable,
// or partly-invalid registry file must never take service-input down; it
// degrades legibility, nothing more. This mirrors the "local advisory
// validation" principle the design converged on across three rounds of
// review: no consumer may take a hard runtime dependency on this file.

use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct CodeNamespace {
    pub namespace: String,
    pub axis: String,
    pub code_shape: String,
    pub scope: String,
    pub status: String,
    pub owning_archive: String,
    pub data_archive: String,
    pub canonical_file: String,
    pub key_column: String,
    pub superseded_by: String,
    pub notes: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CodeRegistry {
    pub namespaces: Vec<CodeNamespace>,
    pub warnings: Vec<String>,
}

const EXPECTED_COLUMNS: usize = 11;

/// Load code-namespaces.csv, skipping leading `#` comment lines. Never
/// panics: a missing or malformed file yields an empty registry plus a
/// warning, not an error the caller must handle specially.
pub fn load_code_namespaces(path: &str) -> CodeRegistry {
    let mut warnings = Vec::new();

    let raw = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            warnings.push(format!("code-namespaces.csv not readable at {path}: {e}"));
            return CodeRegistry {
                namespaces: Vec::new(),
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
        // Flexible: let a ragged row (wrong column count) through to our own
        // explicit check below, which reports a clearer message than the
        // csv crate's own strict-mode error would.
        .flexible(true)
        .from_reader(filtered.as_bytes());

    let mut namespaces = Vec::new();
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
        namespaces.push(CodeNamespace {
            namespace: rec.get(0).unwrap_or("").to_string(),
            axis: rec.get(1).unwrap_or("").to_string(),
            code_shape: rec.get(2).unwrap_or("").to_string(),
            scope: rec.get(3).unwrap_or("").to_string(),
            status: rec.get(4).unwrap_or("").to_string(),
            owning_archive: rec.get(5).unwrap_or("").to_string(),
            data_archive: rec.get(6).unwrap_or("").to_string(),
            canonical_file: rec.get(7).unwrap_or("").to_string(),
            key_column: rec.get(8).unwrap_or("").to_string(),
            superseded_by: rec.get(9).unwrap_or("").to_string(),
            notes: rec.get(10).unwrap_or("").to_string(),
        });
    }

    // Advisory structural check only — active namespaces should point at a
    // real file, but a stale/missing canonical_file is logged, never fatal.
    // "pending" rows (e.g. company-legacy) are expected to have no real
    // file yet and are skipped deliberately.
    for ns in &namespaces {
        if matches!(ns.status.as_str(), "active" | "active-thin")
            && !ns.canonical_file.starts_with("pending")
            && !ns.canonical_file.is_empty()
            && !Path::new(&ns.canonical_file).exists()
        {
            warnings.push(format!(
                "namespace '{}': status={} but canonical_file not found: {}",
                ns.namespace, ns.status, ns.canonical_file
            ));
        }
    }

    CodeRegistry {
        namespaces,
        warnings,
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ResolveResult {
    pub namespace: String,
    pub code: String,
    pub found: bool,
    /// The matched row, as header -> value. Deliberately schema-agnostic
    /// (no hardcoded columns) so this works for any namespace's real file
    /// without this crate needing to know its shape in advance.
    pub row: Option<std::collections::BTreeMap<String, String>>,
    pub warnings: Vec<String>,
}

/// Resolve `code` within `namespace` by reading that namespace's real
/// `canonical_file` live, at request time — this is the "resolver, not
/// replica" pattern the design converged on across all three review
/// rounds: never store or duplicate a namespace's real data in this
/// registry, only ever look it up fresh. Advisory only, same as every
/// other function in this module — any failure becomes a warning, never
/// a panic or an error the caller must specially handle.
pub fn resolve_code(registry: &CodeRegistry, namespace: &str, code: &str) -> ResolveResult {
    let mut warnings = Vec::new();

    let Some(ns) = registry
        .namespaces
        .iter()
        .find(|n| n.namespace == namespace)
    else {
        warnings.push(format!("unknown namespace: '{namespace}'"));
        return ResolveResult {
            namespace: namespace.to_string(),
            code: code.to_string(),
            found: false,
            row: None,
            warnings,
        };
    };

    if ns.canonical_file.starts_with("pending") || ns.canonical_file.is_empty() {
        warnings.push(format!(
            "namespace '{namespace}' has no materialized canonical_file yet (status={})",
            ns.status
        ));
        return ResolveResult {
            namespace: namespace.to_string(),
            code: code.to_string(),
            found: false,
            row: None,
            warnings,
        };
    }

    if ns.key_column.contains('+') {
        warnings.push(format!(
            "namespace '{namespace}' has a composite key ('{}'); single-code resolve is not supported",
            ns.key_column
        ));
        return ResolveResult {
            namespace: namespace.to_string(),
            code: code.to_string(),
            found: false,
            row: None,
            warnings,
        };
    }

    let raw = match std::fs::read_to_string(&ns.canonical_file) {
        Ok(s) => s,
        Err(e) => {
            warnings.push(format!(
                "canonical_file for namespace '{namespace}' not readable at {}: {e}",
                ns.canonical_file
            ));
            return ResolveResult {
                namespace: namespace.to_string(),
                code: code.to_string(),
                found: false,
                row: None,
                warnings,
            };
        }
    };

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(raw.as_bytes());

    let headers: Vec<String> = match reader.headers() {
        Ok(h) => h.iter().map(|s| s.to_string()).collect(),
        Err(e) => {
            warnings.push(format!(
                "canonical_file for namespace '{namespace}': header parse error: {e}"
            ));
            return ResolveResult {
                namespace: namespace.to_string(),
                code: code.to_string(),
                found: false,
                row: None,
                warnings,
            };
        }
    };
    let Some(key_idx) = headers.iter().position(|h| h == &ns.key_column) else {
        warnings.push(format!(
            "canonical_file for namespace '{namespace}' has no column named '{}' (real columns: {:?})",
            ns.key_column, headers
        ));
        return ResolveResult {
            namespace: namespace.to_string(),
            code: code.to_string(),
            found: false,
            row: None,
            warnings,
        };
    };

    for result in reader.records() {
        let rec = match result {
            Ok(r) => r,
            Err(_) => continue,
        };
        if rec.get(key_idx) == Some(code) {
            let row: std::collections::BTreeMap<String, String> = headers
                .iter()
                .enumerate()
                .map(|(i, h)| (h.clone(), rec.get(i).unwrap_or("").to_string()))
                .collect();
            return ResolveResult {
                namespace: namespace.to_string(),
                code: code.to_string(),
                found: true,
                row: Some(row),
                warnings,
            };
        }
    }

    ResolveResult {
        namespace: namespace.to_string(),
        code: code.to_string(),
        found: false,
        row: None,
        warnings,
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

    #[test]
    fn loads_well_formed_registry_with_comments() {
        let csv = "# a header comment\n# another\nnamespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\nentity,legal-entity,shape,global,active,project-accounting,project-bookkeeping,/tmp/does-not-exist.csv,entity_code,,some notes\n";
        let f = write_temp_csv(csv);
        let reg = load_code_namespaces(f.path().to_str().unwrap());
        assert_eq!(reg.namespaces.len(), 1);
        assert_eq!(reg.namespaces[0].namespace, "entity");
        assert_eq!(reg.namespaces[0].axis, "legal-entity");
        // canonical_file doesn't exist -> exactly one advisory warning, not fatal
        assert_eq!(reg.warnings.len(), 1);
        assert!(reg.warnings[0].contains("canonical_file not found"));
    }

    #[test]
    fn pending_status_skips_file_existence_check() {
        let csv = "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\ncompany-legacy,legal-entity,shape,global,frozen-historical,project-input,,pending -- not yet materialized,code,entity,notes here\n";
        let f = write_temp_csv(csv);
        let reg = load_code_namespaces(f.path().to_str().unwrap());
        assert_eq!(reg.namespaces.len(), 1);
        assert!(reg.warnings.is_empty());
    }

    #[test]
    fn missing_file_is_advisory_not_fatal() {
        let reg = load_code_namespaces("/nonexistent/path/code-namespaces.csv");
        assert!(reg.namespaces.is_empty());
        assert_eq!(reg.warnings.len(), 1);
        assert!(reg.warnings[0].contains("not readable"));
    }

    #[test]
    fn malformed_row_column_count_is_skipped_with_warning() {
        let csv = "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\ntoo-few,axis-only\n";
        let f = write_temp_csv(csv);
        let reg = load_code_namespaces(f.path().to_str().unwrap());
        assert!(reg.namespaces.is_empty());
        assert_eq!(reg.warnings.len(), 1);
        assert!(reg.warnings[0].contains("expected 11 columns"));
    }

    fn write_temp_data_csv(contents: &str) -> tempfile::NamedTempFile {
        write_temp_csv(contents)
    }

    #[test]
    fn resolve_code_finds_a_real_match_without_storing_it() {
        let data = write_temp_data_csv(
            "entity_code,legal_name,role\nPRO-CA-01-AST,Woodfine Professional Centres LP,Direct-Hold\n",
        );
        let reg_csv = format!(
            "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\nentity,legal-entity,shape,global,active,project-accounting,project-bookkeeping,{},entity_code,,notes\n",
            data.path().to_str().unwrap()
        );
        let reg_file = write_temp_csv(&reg_csv);
        let registry = load_code_namespaces(reg_file.path().to_str().unwrap());
        assert!(registry.warnings.is_empty());

        let result = resolve_code(&registry, "entity", "PRO-CA-01-AST");
        assert!(result.found);
        let row = result.row.expect("row present");
        assert_eq!(
            row.get("legal_name").unwrap(),
            "Woodfine Professional Centres LP"
        );
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn resolve_code_not_found_is_advisory_not_an_error() {
        let data = write_temp_data_csv("entity_code,legal_name\nPRO-CA-01-AST,Real Entity\n");
        let reg_csv = format!(
            "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\nentity,legal-entity,shape,global,active,project-accounting,project-bookkeeping,{},entity_code,,notes\n",
            data.path().to_str().unwrap()
        );
        let reg_file = write_temp_csv(&reg_csv);
        let registry = load_code_namespaces(reg_file.path().to_str().unwrap());

        let result = resolve_code(&registry, "entity", "NOT-A-REAL-CODE");
        assert!(!result.found);
        assert!(result.row.is_none());
        assert!(
            result.warnings.is_empty(),
            "not-found is not a warning-worthy condition"
        );
    }

    #[test]
    fn resolve_code_unknown_namespace_is_advisory() {
        let registry = CodeRegistry::default();
        let result = resolve_code(&registry, "not-a-namespace", "X");
        assert!(!result.found);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("unknown namespace"));
    }

    #[test]
    fn resolve_code_pending_namespace_is_advisory() {
        let csv = "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\ncompany-legacy,legal-entity,shape,global,frozen-historical,project-input,,pending -- not materialized,code,entity,notes\n";
        let f = write_temp_csv(csv);
        let registry = load_code_namespaces(f.path().to_str().unwrap());
        let result = resolve_code(&registry, "company-legacy", "1000");
        assert!(!result.found);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("no materialized canonical_file"));
    }

    #[test]
    fn resolve_code_composite_key_is_advisory() {
        let data = write_temp_data_csv("entity_code,account_code\nPRO-CA-01-AST,1000\n");
        let reg_csv = format!(
            "namespace,axis,code_shape,scope,status,owning_archive,data_archive,canonical_file,key_column,superseded_by,notes\ngl-account,ledger-account,shape,scoped-by:entity,active,project-accounting,project-bookkeeping,{},entity_code+account_code,,notes\n",
            data.path().to_str().unwrap()
        );
        let reg_file = write_temp_csv(&reg_csv);
        let registry = load_code_namespaces(reg_file.path().to_str().unwrap());
        let result = resolve_code(&registry, "gl-account", "1000");
        assert!(!result.found);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("composite key"));
    }
}
