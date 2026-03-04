//! Local Verification and Compliance Auditing for Generated Artifacts.

use crate::engines::OperationalRules;
use regex::Regex;

/// The result of a compliance audit.
pub struct AuditResult {
    pub is_compliant: bool,
    pub violations: Vec<String>,
}

/// Executes a local regex scan against the generated text to ensure strict compliance
/// with BCSC and internal standards before writing to a Totebox Archive.
pub fn verify_artifact(content: &str, rules: &OperationalRules) -> AuditResult {
    let mut violations = Vec::new();

    // 1. Blacklist Enforcement (Banned Buzzwords)
    if let Some(ref banned) = rules.banned_buzzwords {
        for word in banned {
            // Match exact words, case-insensitive, ignoring punctuation
            let pattern = format!(r"(?i)\b{}\b", regex::escape(word));
            if let Ok(re) = Regex::new(&pattern) {
                if re.is_match(content) {
                    violations.push(format!("Banned buzzword detected: {}", word));
                }
            }
        }
    }

    // 2. Anti-Puffery Doctrine (Defense in Depth)
    if rules.anti_puffery_verbs.unwrap_or(false) {
        // Sample list of absolute or highly subjective verbs forbidden under BCSC guidelines
        let puffery_patterns = vec![
            r"(?i)\bguarantees\b",
            r"(?i)\bfoolproof\b",
            r"(?i)\bunprecedented\b",
        ];
        
        for pattern in puffery_patterns {
             if let Ok(re) = Regex::new(pattern) {
                if re.is_match(content) {
                    violations.push(format!("Subjective/Puffery term detected: {}", pattern));
                }
            }
        }
    }

    AuditResult {
        is_compliant: violations.is_empty(),
        violations,
    }
}
