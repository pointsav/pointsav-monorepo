// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Sanitize-outbound redaction filter.
//!
//! Rust port of `~/Foundry/bin/capture-edit.py` REDACTIONS, applied to
//! every byte that lands in an apprenticeship corpus tuple per the
//! convention §9 sanitize-outbound rule (the L1 capture pipeline does
//! the same thing for engineering-corpus records). Keep the two filter
//! sets in lock-step — when one regex set tightens, the other should
//! too. The Python form is the textual-spec reference.

use std::sync::OnceLock;

use regex::{Captures, Regex};

struct Patterns {
    pem_private_key: Regex,
    aws_access_key: Regex,
    sk_api_key: Regex,
    github_pat: Regex,
    github_oauth: Regex,
    slack_token: Regex,
    bearer_pair: Regex,
}

fn patterns() -> &'static Patterns {
    static P: OnceLock<Patterns> = OnceLock::new();
    P.get_or_init(|| Patterns {
        pem_private_key: Regex::new(
            r"(?s)-----BEGIN (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----.*?-----END (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----",
        )
        .expect("static regex"),
        aws_access_key: Regex::new(r"\bAKIA[0-9A-Z]{16}\b").expect("static regex"),
        sk_api_key: Regex::new(r"\bsk-(?:proj-)?[A-Za-z0-9_\-]{32,}\b").expect("static regex"),
        github_pat: Regex::new(r"\bghp_[A-Za-z0-9]{36,}\b").expect("static regex"),
        github_oauth: Regex::new(r"\bgho_[A-Za-z0-9]{36,}\b").expect("static regex"),
        slack_token: Regex::new(r"\bxox[abprs]-[A-Za-z0-9-]{10,}\b").expect("static regex"),
        // Match `key: "VALUE"` / `key=VALUE` patterns where key ∈
        // {bearer, api_key, api-key, secret, token, password} and the
        // value is at least 32 chars of high-entropy ASCII.
        bearer_pair: Regex::new(
            r#"(?i)\b(?:bearer|api[_-]?key|secret|token|password)\s*[:=]\s*["']?([A-Za-z0-9/+_\-]{32,})["']?"#,
        )
        .expect("static regex"),
    })
}

/// Apply sanitize-outbound to `text`. Mirrors `bin/capture-edit.py`
/// `sanitize()` and is the only redaction surface in the
/// apprenticeship pipeline (briefs in, attempts in, verdicts in,
/// corpus tuples out).
pub fn sanitize(text: &str) -> String {
    let p = patterns();
    let mut out = p
        .pem_private_key
        .replace_all(text, "[REDACTED PRIVATE KEY]")
        .into_owned();
    out = p
        .aws_access_key
        .replace_all(&out, "[REDACTED AWS KEY]")
        .into_owned();
    out = p
        .sk_api_key
        .replace_all(&out, "[REDACTED API KEY]")
        .into_owned();
    out = p
        .github_pat
        .replace_all(&out, "[REDACTED GITHUB TOKEN]")
        .into_owned();
    out = p
        .github_oauth
        .replace_all(&out, "[REDACTED GITHUB OAUTH]")
        .into_owned();
    out = p
        .slack_token
        .replace_all(&out, "[REDACTED SLACK TOKEN]")
        .into_owned();
    // bearer_pair preserves the key/operator/quotes and only swaps the
    // value capture group, matching the Python form.
    out = p
        .bearer_pair
        .replace_all(&out, |caps: &Captures<'_>| {
            let whole = caps.get(0).map(|m| m.as_str()).unwrap_or("");
            let secret = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            if secret.is_empty() {
                whole.to_string()
            } else {
                whole.replacen(secret, "[REDACTED]", 1)
            }
        })
        .into_owned();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_aws_access_key() {
        let s = "rotate AKIAIOSFODNN7EXAMPLE before sharing";
        assert_eq!(sanitize(s), "rotate [REDACTED AWS KEY] before sharing");
    }

    #[test]
    fn redacts_anthropic_style_sk_key() {
        let s = "key=sk-ant-api03-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let out = sanitize(s);
        assert!(out.contains("[REDACTED API KEY]") || out.contains("[REDACTED]"));
        assert!(!out.contains("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }

    #[test]
    fn redacts_github_token() {
        let s = "echo ghp_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let out = sanitize(s);
        assert!(out.contains("[REDACTED GITHUB TOKEN]"));
        assert!(!out.contains("ghp_aaaaa"));
    }

    #[test]
    fn redacts_pem_block() {
        let s = "-----BEGIN PRIVATE KEY-----\nAAAA\nBBBB\n-----END PRIVATE KEY-----";
        let out = sanitize(s);
        assert_eq!(out, "[REDACTED PRIVATE KEY]");
    }

    #[test]
    fn redacts_bearer_value_only() {
        // The regex matches `<key>: <value>` / `<key>=<value>` shapes
        // where key ∈ {bearer, api_key, secret, token, password}. Use
        // a `token=...` shape to exercise the value-only swap.
        let s = "config.token = \"abcdefghij1234567890ABCDEFGHIJ_-aa\"";
        let out = sanitize(s);
        // Key + operator + quotes preserved; only the value swapped.
        assert!(out.contains("token = \"[REDACTED]\""), "got: {out}");
        assert!(!out.contains("abcdefghij"));
    }

    #[test]
    fn passes_through_short_or_unrelated_strings() {
        assert_eq!(sanitize("hello world"), "hello world");
        assert_eq!(sanitize("token=short"), "token=short"); // < 32 chars
    }
}
