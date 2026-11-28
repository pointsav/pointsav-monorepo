// SPDX-License-Identifier: Apache-2.0

//! Engine registry: the fixed set of engines a Doorman will run jobs on,
//! loaded from `SLM_ENGINE_REGISTRY_FILE`.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineKind {
    /// Local llama.cpp-style chat endpoint (Tier A, OLMo).
    LlmLocal,
    /// OpenAI-style vision chat endpoint (llama.cpp `llama-server` with an
    /// mmproj, or compatible) used as an OCR reader.
    OcrOpenaiVision,
    /// GLiNER-class span extractor (`POST /v1/extract` returning
    /// `{entities:[{entity_name, classification}]}`), e.g. the guest Tier 0.
    SpanGliner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSpec {
    pub engine_id: String,
    pub kind: EngineKind,
    /// Base URL of the engine (no trailing path).
    pub endpoint: String,
    /// Model identifier sent to the engine and recorded per job.
    pub model: String,
    /// Declared SHA-256 of the weights file; recorded, not verified here.
    pub weights_sha256: String,
    pub licence: String,
    /// Concurrent jobs for this engine.
    pub permits: u32,
    /// Declared CPU affinity. Enforced by the engine's systemd unit
    /// (`CPUAffinity=`), reported here; the Doorman cannot pin another process.
    #[serde(default)]
    pub cpu_set: Option<String>,
    #[serde(default)]
    pub nice: Option<i32>,
    pub max_job_secs: u64,
    pub max_input_bytes: u64,
    /// Entity-mining chunk size in bytes (default 8192). Inputs longer than
    /// this are split with overlap; each chunk is one engine call.
    #[serde(default)]
    pub chunk_bytes: Option<u64>,
    #[serde(default)]
    pub chunk_overlap: Option<u64>,
    /// Span extractors only: the engine's label domain (default `projects`).
    #[serde(default)]
    pub domain_id: Option<String>,
    /// Span extractors only: the label domains a job may ask for through
    /// `params.domain_id`. Absent means only the engine's own `domain_id`
    /// (so existing registries keep their meaning). The engine's own domain
    /// must be in the list.
    #[serde(default)]
    pub domains: Option<Vec<String>>,
    /// Must be absent or null: there is no fallback between engines.
    #[serde(default)]
    pub fallback: Option<String>,
    /// Declared descriptive metadata. Reported by `GET /v1/engines`; nothing
    /// here is enforced or verified by the Doorman.
    #[serde(default)]
    pub declared: Option<EngineDeclared>,
}

/// What an operator declares about an engine. Every field is optional text;
/// the Doorman records and reports it and acts on none of it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineDeclared {
    /// What the engine does, e.g. `entity-spans` or `ocr`.
    #[serde(default)]
    pub capability: Option<String>,
    /// Where calls to it travel, e.g. `local` or `gateway`.
    #[serde(default)]
    pub route: Option<String>,
    /// The machine or service that runs it.
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    /// What leaves the machine when it is called, in words.
    #[serde(default)]
    pub egress: Option<String>,
    /// The engine software version someone last checked.
    #[serde(default)]
    pub verified_version: Option<String>,
    /// Who cleared the engine's licence, and the date (`YYYY-MM-DD`).
    #[serde(default)]
    pub licence_cleared_by: Option<String>,
    #[serde(default)]
    pub licence_cleared_on: Option<String>,
    /// Whether the engine moves with the archive it serves.
    #[serde(default)]
    pub travels_with_archive: Option<bool>,
}

/// A domain label: 1 to 64 characters of `a-z`, `0-9`, `-` or `_`.
pub fn valid_domain_token(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
}

impl EngineSpec {
    /// The engine's own label domain (default `projects`).
    pub fn default_domain(&self) -> &str {
        self.domain_id.as_deref().unwrap_or("projects")
    }

    /// Domains a span-extraction job may name: the declared list, else only
    /// the engine's own domain.
    pub fn served_domains(&self) -> Vec<String> {
        match &self.domains {
            Some(d) => d.clone(),
            None => vec![self.default_domain().to_string()],
        }
    }
}

const DECLARED_TEXT_MAX: usize = 200;

impl EngineDeclared {
    /// Text fields must be non-blank, short and free of control characters;
    /// the date must look like `YYYY-MM-DD`.
    fn validate(&self) -> Result<(), String> {
        let texts = [
            ("capability", &self.capability),
            ("route", &self.route),
            ("host", &self.host),
            ("owner", &self.owner),
            ("egress", &self.egress),
            ("verified_version", &self.verified_version),
            ("licence_cleared_by", &self.licence_cleared_by),
            ("licence_cleared_on", &self.licence_cleared_on),
        ];
        for (name, value) in texts {
            let Some(v) = value else { continue };
            if v.trim().is_empty() {
                return Err(format!("declared.{name} must not be blank"));
            }
            if v.chars().count() > DECLARED_TEXT_MAX {
                return Err(format!(
                    "declared.{name} is longer than {DECLARED_TEXT_MAX} characters"
                ));
            }
            if v.chars().any(char::is_control) {
                return Err(format!("declared.{name} contains a control character"));
            }
        }
        if let Some(d) = &self.licence_cleared_on {
            let b = d.as_bytes();
            let shaped = b.len() == 10
                && b.iter().enumerate().all(|(i, c)| match i {
                    4 | 7 => *c == b'-',
                    _ => c.is_ascii_digit(),
                });
            if !shaped {
                return Err("declared.licence_cleared_on must be YYYY-MM-DD".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryFile {
    engines: Vec<EngineSpec>,
}

#[derive(Clone, Debug)]
pub struct Registry {
    engines: Vec<EngineSpec>,
    index: HashMap<String, usize>,
}

impl Registry {
    pub fn from_specs(engines: Vec<EngineSpec>) -> Result<Self, String> {
        if engines.is_empty() {
            return Err("engine registry is empty".into());
        }
        let mut index = HashMap::new();
        for (i, e) in engines.iter().enumerate() {
            let id_ok = !e.engine_id.is_empty()
                && e.engine_id.len() <= 64
                && e.engine_id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.');
            if !id_ok {
                return Err(format!("invalid engine_id {:?}", e.engine_id));
            }
            if e.permits == 0 {
                return Err(format!("engine {}: permits must be >= 1", e.engine_id));
            }
            if e.max_job_secs == 0 || e.max_input_bytes == 0 {
                return Err(format!(
                    "engine {}: max_job_secs/max_input_bytes must be > 0",
                    e.engine_id
                ));
            }
            if e.fallback.is_some() {
                return Err(format!(
                    "engine {}: fallback is not permitted (an unavailable engine yields `deferred`)",
                    e.engine_id
                ));
            }
            if let Some(list) = &e.domains {
                if e.kind != EngineKind::SpanGliner {
                    return Err(format!(
                        "engine {}: domains applies only to span-gliner engines",
                        e.engine_id
                    ));
                }
                if list.is_empty() || !list.iter().all(|d| valid_domain_token(d)) {
                    return Err(format!(
                        "engine {}: domains must be a non-empty list of labels (a-z, 0-9, '-', '_')",
                        e.engine_id
                    ));
                }
                let mut seen = std::collections::HashSet::new();
                if !list.iter().all(|d| seen.insert(d.as_str())) {
                    return Err(format!("engine {}: domains has a duplicate", e.engine_id));
                }
                if !list.iter().any(|d| d == e.default_domain()) {
                    return Err(format!(
                        "engine {}: its own domain {:?} must be in domains",
                        e.engine_id,
                        e.default_domain()
                    ));
                }
            }
            if let Some(d) = &e.declared {
                d.validate()
                    .map_err(|m| format!("engine {}: {m}", e.engine_id))?;
            }
            if index.insert(e.engine_id.clone(), i).is_some() {
                return Err(format!("duplicate engine_id {}", e.engine_id));
            }
        }
        Ok(Self { engines, index })
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read engine registry {}: {e}", path.display()))?;
        let file: RegistryFile =
            serde_json::from_str(&raw).map_err(|e| format!("engine registry parse error: {e}"))?;
        Self::from_specs(file.engines)
    }

    pub fn get(&self, engine_id: &str) -> Option<&EngineSpec> {
        self.index.get(engine_id).map(|&i| &self.engines[i])
    }

    pub fn all(&self) -> &[EngineSpec] {
        &self.engines
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use super::*;

    pub(crate) fn spec(id: &str) -> EngineSpec {
        EngineSpec {
            engine_id: id.into(),
            kind: EngineKind::LlmLocal,
            endpoint: "http://127.0.0.1:8080".into(),
            model: "m".into(),
            weights_sha256: "w".into(),
            licence: "apache-2.0".into(),
            permits: 1,
            cpu_set: None,
            nice: None,
            max_job_secs: 900,
            max_input_bytes: 1024,
            chunk_bytes: None,
            chunk_overlap: None,
            domain_id: None,
            domains: None,
            fallback: None,
            declared: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tests_support::spec;
    use super::*;

    #[test]
    fn rejects_empty_duplicate_and_fallback() {
        assert!(Registry::from_specs(vec![]).is_err());
        assert!(Registry::from_specs(vec![spec("a"), spec("a")]).is_err());
        let mut f = spec("a");
        f.fallback = Some("b".into());
        assert!(Registry::from_specs(vec![f]).is_err());
        let mut bad = spec("../a");
        bad.permits = 1;
        assert!(Registry::from_specs(vec![bad]).is_err());
        assert!(Registry::from_specs(vec![spec("a"), spec("b")]).is_ok());
    }

    #[test]
    fn declared_metadata_is_optional_and_loads_from_json() {
        let without = r#"{"engines":[{"engine_id":"a","kind":"llm-local","endpoint":"http://x","model":"m","weights_sha256":"w","licence":"l","permits":1,"max_job_secs":1,"max_input_bytes":1}]}"#;
        let f: RegistryFile = serde_json::from_str(without).unwrap();
        assert!(f.engines[0].declared.is_none());
        let with = r#"{"engines":[{"engine_id":"a","kind":"span-gliner","endpoint":"http://x","model":"m","weights_sha256":"w","licence":"l","permits":1,"max_job_secs":1,"max_input_bytes":1,"declared":{"capability":"entity-spans","route":"gateway","host":"h1","owner":"team","egress":"none","verified_version":"1.0","licence_cleared_by":"someone","licence_cleared_on":"2026-11-19","travels_with_archive":false}}]}"#;
        let f: RegistryFile = serde_json::from_str(with).unwrap();
        let d = f.engines[0].declared.as_ref().unwrap();
        assert_eq!(d.capability.as_deref(), Some("entity-spans"));
        assert_eq!(d.travels_with_archive, Some(false));
        assert!(Registry::from_specs(f.engines).is_ok());
    }

    #[test]
    fn declared_metadata_rejects_blank_long_control_and_bad_date() {
        let with = |d: EngineDeclared| {
            let mut e = spec("a");
            e.declared = Some(d);
            Registry::from_specs(vec![e])
        };
        let ok = EngineDeclared {
            capability: Some("ocr".into()),
            licence_cleared_on: Some("2026-11-19".into()),
            ..Default::default()
        };
        assert!(with(ok).is_ok());
        let blank = EngineDeclared {
            owner: Some("   ".into()),
            ..Default::default()
        };
        assert!(with(blank).unwrap_err().contains("declared.owner"));
        let long = EngineDeclared {
            host: Some("x".repeat(201)),
            ..Default::default()
        };
        assert!(with(long).unwrap_err().contains("declared.host"));
        let control = EngineDeclared {
            egress: Some("a\nb".into()),
            ..Default::default()
        };
        assert!(with(control).unwrap_err().contains("declared.egress"));
        for bad in [
            "2026-1-19",
            "19-11-2026",
            "2026/11/19",
            "20261119",
            "2026-11-1x",
        ] {
            let d = EngineDeclared {
                licence_cleared_on: Some(bad.into()),
                ..Default::default()
            };
            assert!(with(d).unwrap_err().contains("YYYY-MM-DD"), "{bad}");
        }
    }

    #[test]
    fn declared_with_an_unknown_field_is_refused() {
        let j = r#"{"engines":[{"engine_id":"a","kind":"llm-local","endpoint":"http://x","model":"m","weights_sha256":"w","licence":"l","permits":1,"max_job_secs":1,"max_input_bytes":1,"declared":{"capability":"x","bogus":1}}]}"#;
        assert!(serde_json::from_str::<RegistryFile>(j).is_err());
    }

    #[test]
    fn json_with_unknown_field_or_fallback_is_refused() {
        let j = r#"{"engines":[{"engine_id":"a","kind":"llm-local","endpoint":"http://x","model":"m","weights_sha256":"w","licence":"l","permits":1,"max_job_secs":1,"max_input_bytes":1,"bogus":1}]}"#;
        let f: Result<RegistryFile, _> = serde_json::from_str(j);
        assert!(f.is_err());
    }
    fn span(domain_id: Option<&str>, domains: Option<&[&str]>) -> EngineSpec {
        let mut e = spec("g");
        e.kind = EngineKind::SpanGliner;
        e.domain_id = domain_id.map(str::to_string);
        e.domains = domains.map(|d| d.iter().map(|s| s.to_string()).collect());
        e
    }

    #[test]
    fn served_domains_default_to_the_engines_own_domain() {
        assert_eq!(span(None, None).served_domains(), vec!["projects"]);
        assert_eq!(
            span(Some("corporate"), None).served_domains(),
            vec!["corporate"]
        );
        let many = span(
            Some("projects"),
            Some(&["projects", "corporate", "documentation"]),
        );
        assert_eq!(many.served_domains().len(), 3);
    }

    #[test]
    fn domains_list_is_validated() {
        let ok = span(Some("projects"), Some(&["projects", "corporate"]));
        assert!(Registry::from_specs(vec![ok]).is_ok());
        // empty list, malformed token, duplicate, default not in the list
        for bad in [
            span(None, Some(&[])),
            span(None, Some(&["projects", "Bad Domain"])),
            span(None, Some(&["projects", "projects"])),
            span(Some("projects"), Some(&["corporate"])),
        ] {
            assert!(Registry::from_specs(vec![bad]).is_err());
        }
        // only span extractors may list domains
        let mut llm = spec("a");
        llm.domains = Some(vec!["projects".into()]);
        assert!(Registry::from_specs(vec![llm]).is_err());
    }
}
