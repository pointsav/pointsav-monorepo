// SPDX-License-Identifier: Apache-2.0

//! Server-side credential table for `/v1/jobs`. Module id, engine allow-list,
//! priority class and input directory all come from the matched entry —
//! never from the request.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use slm_core::ModuleId;

use super::hex;

/// How a credential's jobs are admitted to the engines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PriorityClass {
    /// Idle-only (P2): never preempts or delays interactive traffic.
    Idle,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credential {
    pub credential_id: String,
    /// Lowercase hex SHA-256 of the bearer token.
    pub token_sha256: String,
    pub module_id: ModuleId,
    pub priority_class: PriorityClass,
    pub engines: Vec<String>,
    /// Directory for existing-file inputs (`input.local_name`), if any.
    #[serde(default)]
    pub input_dir: Option<PathBuf>,
}

impl Credential {
    pub fn allows_engine(&self, engine_id: &str) -> bool {
        self.engines.iter().any(|e| e == engine_id)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CredentialFile {
    credentials: Vec<Credential>,
}

#[derive(Clone, Debug)]
pub struct CredentialSet {
    creds: Vec<Credential>,
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

impl CredentialSet {
    pub fn from_credentials(creds: Vec<Credential>) -> Result<Self, String> {
        if creds.is_empty() {
            return Err("credential table is empty (jobs fail closed)".into());
        }
        for (i, c) in creds.iter().enumerate() {
            let hash_ok = c.token_sha256.len() == 64
                && c.token_sha256
                    .bytes()
                    .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
            if !hash_ok {
                return Err(format!(
                    "credential {}: token_sha256 must be 64 lowercase hex chars",
                    c.credential_id
                ));
            }
            if c.engines.is_empty() {
                return Err(format!(
                    "credential {}: empty engine allow-list",
                    c.credential_id
                ));
            }
            if creds[..i]
                .iter()
                .any(|o| o.token_sha256 == c.token_sha256 || o.credential_id == c.credential_id)
            {
                return Err(format!(
                    "credential {}: duplicate id or token hash",
                    c.credential_id
                ));
            }
        }
        Ok(Self { creds })
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read credentials {}: {e}", path.display()))?;
        let f: CredentialFile =
            serde_json::from_str(&raw).map_err(|e| format!("credentials parse error: {e}"))?;
        Self::from_credentials(f.credentials)
    }

    /// Match a bearer token. Every entry is compared (no early exit) so the
    /// time taken does not depend on which entry, if any, matched.
    pub fn authenticate(&self, token: &str) -> Option<&Credential> {
        let hash = hex(&Sha256::digest(token.as_bytes()));
        let mut found = None;
        for c in &self.creds {
            if constant_time_eq(hash.as_bytes(), c.token_sha256.as_bytes()) {
                found = Some(c);
            }
        }
        found
    }

    pub fn validate_engines(&self, known: impl Fn(&str) -> bool) -> Result<(), String> {
        for c in &self.creds {
            for e in &c.engines {
                if !known(e) {
                    return Err(format!(
                        "credential {}: unknown engine {e}",
                        c.credential_id
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;
    use std::str::FromStr;

    pub(crate) fn cred(id: &str, token: &str, module: &str, engines: &[&str]) -> Credential {
        Credential {
            credential_id: id.into(),
            token_sha256: hex(&Sha256::digest(token.as_bytes())),
            module_id: ModuleId::from_str(module).unwrap(),
            priority_class: PriorityClass::Idle,
            engines: engines.iter().map(|s| s.to_string()).collect(),
            input_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tests_support::cred;
    use super::*;

    #[test]
    fn authenticates_only_exact_token() {
        let set = CredentialSet::from_credentials(vec![
            cred("a", "tok-a", "mod-a", &["e1"]),
            cred("b", "tok-b", "mod-b", &["e1", "e2"]),
        ])
        .unwrap();
        assert_eq!(set.authenticate("tok-b").unwrap().credential_id, "b");
        assert!(set.authenticate("tok-c").is_none());
        assert!(set.authenticate("").is_none());
        assert!(set.authenticate("tok-a ").is_none());
    }

    #[test]
    fn empty_or_malformed_tables_fail_closed() {
        assert!(CredentialSet::from_credentials(vec![]).is_err());
        let mut c = cred("a", "t", "m", &["e"]);
        c.token_sha256 = "abc".into();
        assert!(CredentialSet::from_credentials(vec![c]).is_err());
        let c = cred("a", "t", "m", &[]);
        assert!(CredentialSet::from_credentials(vec![c]).is_err());
        let dup = vec![cred("a", "t", "m", &["e"]), cred("b", "t", "m", &["e"])];
        assert!(CredentialSet::from_credentials(dup).is_err());
    }
}
