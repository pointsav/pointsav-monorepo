// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Ring 3b adapter registry — parsed and validated from `registry.yaml`.

use std::collections::HashSet;

use serde::Deserialize;

/// A single entry in the adapter catalogue.
///
/// `version` is validated semver at parse time. `oci_ref` is the
/// GCS/OCI path used when loading the adapter into the inference engine.
#[derive(Debug, Clone, PartialEq)]
pub struct AdapterEntry {
    /// Stable identifier for this adapter family (e.g. `"dka-coa"`).
    pub adapter_id: String,
    /// Immutable semver version. `coa/v3.2` in an OCI path corresponds to
    /// version `3.2.0` here — the `v`-prefix and path segment are separate
    /// from this field.
    pub version: semver::Version,
    /// Base model the adapter was trained on (e.g. `"gemma-4-27b"`).
    pub base_model: String,
    /// OCI Artifact reference or GCS path for the adapter weights.
    pub oci_ref: String,
    /// SHA-256 hash of the training data used to produce this adapter.
    pub training_data_hash: String,
    /// Sigstore keyless signature over the OCI artifact digest.
    pub signature: String,
}

/// The full adapter registry, validated at construction time.
#[derive(Debug, Clone, PartialEq)]
pub struct Registry {
    /// All adapter entries in declaration order.
    pub adapters: Vec<AdapterEntry>,
}

/// Errors produced when parsing or validating a registry.
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// The YAML could not be parsed.
    #[error("invalid YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
    /// A `version` field is not valid semver.
    #[error("invalid semver '{version}' for adapter '{adapter_id}': {source}")]
    InvalidVersion {
        /// Adapter whose version failed to parse.
        adapter_id: String,
        /// The raw string that failed.
        version: String,
        /// The underlying parse error.
        source: semver::Error,
    },
    /// Two entries share the same `(adapter_id, version)`.
    #[error("duplicate entry (adapter_id={adapter_id}, version={version})")]
    DuplicateEntry {
        /// The duplicated adapter identifier.
        adapter_id: String,
        /// The duplicated version string.
        version: String,
    },
}

// ── private serde shapes ─────────────────────────────────────────────────────

#[derive(Deserialize)]
struct RawEntry {
    adapter_id: String,
    version: String,
    base_model: String,
    oci_ref: String,
    training_data_hash: String,
    signature: String,
}

#[derive(Deserialize)]
struct RawRegistry {
    adapters: Vec<RawEntry>,
}

// ── public API ───────────────────────────────────────────────────────────────

impl Registry {
    /// Parse and validate a `registry.yaml` document.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the YAML is malformed, any `version` field is not
    /// valid semver, or any `(adapter_id, version)` pair appears more than once.
    pub fn from_yaml(yaml: &str) -> Result<Self, RegistryError> {
        let raw: RawRegistry = serde_yaml::from_str(yaml)?;

        let mut seen: HashSet<(String, String)> = HashSet::new();
        let mut adapters = Vec::with_capacity(raw.adapters.len());

        for entry in raw.adapters {
            let version = semver::Version::parse(&entry.version).map_err(|source| {
                RegistryError::InvalidVersion {
                    adapter_id: entry.adapter_id.clone(),
                    version: entry.version.clone(),
                    source,
                }
            })?;

            let key = (entry.adapter_id.clone(), entry.version.clone());
            if !seen.insert(key) {
                return Err(RegistryError::DuplicateEntry {
                    adapter_id: entry.adapter_id,
                    version: entry.version,
                });
            }

            adapters.push(AdapterEntry {
                adapter_id: entry.adapter_id,
                version,
                base_model: entry.base_model,
                oci_ref: entry.oci_ref,
                training_data_hash: entry.training_data_hash,
                signature: entry.signature,
            });
        }

        Ok(Registry { adapters })
    }
}
