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
    /// Must be absent or null: there is no fallback between engines.
    #[serde(default)]
    pub fallback: Option<String>,
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
            fallback: None,
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
    fn json_with_unknown_field_or_fallback_is_refused() {
        let j = r#"{"engines":[{"engine_id":"a","kind":"llm-local","endpoint":"http://x","model":"m","weights_sha256":"w","licence":"l","permits":1,"max_job_secs":1,"max_input_bytes":1,"bogus":1}]}"#;
        let f: Result<RegistryFile, _> = serde_json::from_str(j);
        assert!(f.is_err());
    }
}
