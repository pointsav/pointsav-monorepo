use serde_json::Value;

/// Returns true if `name` looks like a code or environment identifier rather
/// than a proper entity name. Used as a deterministic backstop in
/// raw_entities_to_graph() and clean_dpo_side() to reject noise regardless
/// of model compliance.
pub fn is_noise_entity_name(name: &str) -> bool {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return true;
    }
    const PLACEHOLDERS: [&str; 6] = ["not specified", "n/a", "unknown", "tbd", "none", "null"];
    let lower = trimmed.to_lowercase();
    if PLACEHOLDERS.contains(&lower.as_str()) {
        return true;
    }
    // Backtick-wrapped identifiers: `ghi_kwh_m2_yr`
    if trimmed.starts_with('`') && trimmed.ends_with('`') {
        return true;
    }
    // Environment variable references
    if trimmed.starts_with('$') {
        return true;
    }
    // Glob patterns
    if trimmed.contains('*') {
        return true;
    }
    // File paths (absolute, relative, or URL-like)
    if trimmed.contains('/') {
        return true;
    }
    // File-extension-suffixed names: create-yoyo-snapshot.sh, build.py
    const PATH_SUFFIXES: [&str; 9] = [
        ".sh", ".py", ".rs", ".md", ".json", ".jsonl", ".conf", ".toml", ".yaml",
    ];
    if PATH_SUFFIXES.iter().any(|s| lower.ends_with(s)) {
        return true;
    }
    // Math/code expressions with parentheses: ops(slm), log(employment_35km), func()
    if trimmed.contains('(') && trimmed.contains(')') {
        return true;
    }
    // Snake_case identifiers without spaces: env vars, code symbols, metric names
    if !trimmed.contains(' ') && trimmed.contains('_') {
        return true;
    }
    // Sentence fragment starters
    const FRAGMENT_STARTERS: [&str; 4] = ["the ", "a ", "an ", "this "];
    if FRAGMENT_STARTERS.iter().any(|p| lower.starts_with(p)) {
        return true;
    }
    // Comma-joined or conjunction phrases: not a single entity
    if trimmed.contains(", ") || lower.contains(" and ") {
        return true;
    }
    false
}

/// Returns true if `name` matches a conventional-commit prefix pattern:
/// `type(scope)` — e.g. ops(slm), feat(cache), fix(auth), chore(db).
/// These appear in git commit log text and must not be treated as entities.
pub fn is_commit_prefix(name: &str) -> bool {
    let t = name.trim();
    let Some(open) = t.find('(') else {
        return false;
    };
    if !t.ends_with(')') {
        return false;
    }
    let head = &t[..open];
    !head.is_empty()
        && head
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        && open + 1 < t.len() - 1
}

/// Filter noise entities from one side of a DPO pair before saving.
/// Removes entries whose entity_name is a commit prefix or noise pattern.
pub fn clean_dpo_side(side: &[Value]) -> Vec<Value> {
    side.iter()
        .filter(|e| {
            let Some(name) = e.get("entity_name").and_then(|v| v.as_str()) else {
                return false;
            };
            !is_commit_prefix(name) && !is_noise_entity_name(name)
        })
        .cloned()
        .collect()
}

/// Well-known country names that OLMo commonly misclassifies as Company.
/// Caller must pass the entity name already lowercased.
pub fn is_known_place(name_lower: &str) -> bool {
    const COUNTRIES: [&str; 28] = [
        "portugal",
        "spain",
        "france",
        "germany",
        "italy",
        "netherlands",
        "belgium",
        "austria",
        "switzerland",
        "poland",
        "sweden",
        "norway",
        "denmark",
        "finland",
        "united kingdom",
        "united states",
        "canada",
        "australia",
        "mexico",
        "brazil",
        "argentina",
        "india",
        "china",
        "japan",
        "south korea",
        "singapore",
        "hong kong",
        "new zealand",
    ];
    COUNTRIES.contains(&name_lower)
}

/// Apply type-coherence rules to correct or reject a model-emitted classification.
///
/// Returns `Some(corrected_classification)` to accept (with optional correction),
/// or `None` to reject the entity entirely.
pub fn coerce_classification(entity_name: &str, classification: &str) -> Option<String> {
    let name_lower = entity_name.to_lowercase();
    // Country misclassified as Company → reclassify as Location.
    if classification == "Company" && is_known_place(&name_lower) {
        return Some("Location".to_string());
    }
    // File-path-like name as Project → reject (it's a path, not a project).
    if classification == "Project" && entity_name.contains('/') {
        return None;
    }
    // ALL_CAPS with underscores as Account → reject (it's a constant or env var).
    if classification == "Account"
        && entity_name.len() >= 2
        && entity_name
            .chars()
            .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
        && entity_name.chars().any(|c| c.is_ascii_uppercase())
    {
        return None;
    }
    Some(classification.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_rejects_env_var() {
        assert!(is_noise_entity_name("$SLM_DATA_DIR"));
    }

    #[test]
    fn noise_rejects_snake_case() {
        assert!(is_noise_entity_name("ghi_kwh_m2_yr"));
        assert!(is_noise_entity_name("FOUNDRY_ARCHIVE_NAME"));
    }

    #[test]
    fn noise_rejects_shell_script() {
        assert!(is_noise_entity_name("create-yoyo-snapshot.sh"));
    }

    #[test]
    fn noise_rejects_call_expression() {
        assert!(is_noise_entity_name("log(employment_35km)"));
        assert!(is_noise_entity_name("ops(slm)"));
    }

    #[test]
    fn noise_rejects_placeholder() {
        assert!(is_noise_entity_name("not specified"));
        assert!(is_noise_entity_name("N/A"));
        assert!(is_noise_entity_name("unknown"));
    }

    #[test]
    fn noise_rejects_backtick() {
        assert!(is_noise_entity_name("`ghi_kwh_m2_yr`"));
    }

    #[test]
    fn noise_allows_proper_names() {
        assert!(!is_noise_entity_name("Peter Woodfine"));
        assert!(!is_noise_entity_name("PointSav Digital Systems"));
        assert!(!is_noise_entity_name("Woodfine Management Corp."));
        assert!(!is_noise_entity_name("Vancouver"));
        assert!(!is_noise_entity_name("service-content"));
    }

    #[test]
    fn commit_prefix_positive() {
        assert!(is_commit_prefix("ops(slm)"));
        assert!(is_commit_prefix("feat(cache)"));
        assert!(is_commit_prefix("fix(auth)"));
        assert!(is_commit_prefix("chore(db)"));
    }

    #[test]
    fn commit_prefix_negative() {
        assert!(!is_commit_prefix("service-content"));
        assert!(!is_commit_prefix("Peter Woodfine"));
        assert!(!is_commit_prefix("ops()"));  // empty scope
    }

    #[test]
    fn coerce_country_as_company_to_location() {
        let result = coerce_classification("Portugal", "Company");
        assert_eq!(result, Some("Location".to_string()));
    }

    #[test]
    fn coerce_path_as_project_rejected() {
        let result = coerce_classification("src/main.rs", "Project");
        assert_eq!(result, None);
    }

    #[test]
    fn coerce_caps_as_account_rejected() {
        let result = coerce_classification("SLM_TIER_A_FIRST", "Account");
        assert_eq!(result, None);
    }

    #[test]
    fn coerce_valid_person_passthrough() {
        let result = coerce_classification("Jennifer Woodfine", "Person");
        assert_eq!(result, Some("Person".to_string()));
    }

    #[test]
    fn clean_dpo_side_removes_commit_prefix() {
        let side = vec![
            serde_json::json!({"entity_name": "ops(slm)", "classification": "Project"}),
            serde_json::json!({"entity_name": "Jennifer Woodfine", "classification": "Person"}),
        ];
        let cleaned = clean_dpo_side(&side);
        assert_eq!(cleaned.len(), 1);
        assert_eq!(
            cleaned[0]["entity_name"].as_str().unwrap(),
            "Jennifer Woodfine"
        );
    }
}
