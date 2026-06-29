// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Fleet reader — parses pairings.yaml + archive manifests to build the
//! list of paired Totebox Archives.
//!
//! pairings.yaml is read at startup and cached in-process. The fleet does
//! not update at runtime; restart the server to pick up pairing changes.

use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;
use tracing::warn;

use orchestration_command_core::{ArchiveEntry, LegStatus, TetradStatus};

/// Minimal schema for one entry in pairings.yaml.
#[derive(Debug, Deserialize)]
struct PairingEntry {
    cluster_name: String,
    module_id: String,
    #[serde(default = "default_slm")]
    slm_endpoint: String,
    #[serde(default)]
    #[serde(rename = "type")]
    entry_type: String,
}

fn default_slm() -> String {
    "http://localhost:8011".to_string()
}

/// Minimal schema for an archive manifest's tetrad block.
#[derive(Debug, Deserialize, Default)]
struct ManifestTetrad {
    vendor: Option<serde_json::Value>,
    customer: Option<serde_json::Value>,
    deployment: Option<serde_json::Value>,
    wiki: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    #[serde(default)]
    tetrad: ManifestTetrad,
}

/// pairings.yaml top-level (minimal — only fields we use).
#[derive(Debug, Deserialize)]
struct PairingsYaml {
    archives: Vec<PairingEntry>,
}

fn leg_status_from_value(v: Option<&serde_json::Value>) -> LegStatus {
    match v {
        None => LegStatus::Unknown,
        Some(val) => {
            let status_str = val
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|o| o.get("status"))
                .and_then(|s| s.as_str())
                .unwrap_or("");
            if status_str.starts_with("leg-pending") {
                LegStatus::LegPending
            } else if !status_str.is_empty() {
                LegStatus::Active
            } else {
                LegStatus::Unknown
            }
        }
    }
}

fn inbox_pending_count(clones_root: &Path, cluster_name: &str) -> u32 {
    let inbox = clones_root
        .join(cluster_name)
        .join(".agent")
        .join("inbox.md");
    let content = match std::fs::read_to_string(&inbox) {
        Ok(c) => c,
        Err(_) => return 0,
    };
    content
        .lines()
        .filter(|l| l.trim_start().starts_with("status: pending"))
        .count() as u32
}

/// Load all active paired archives from `pairings_path`.
///
/// `clones_root` is the directory containing `clones/<archive>/`.
pub fn load_fleet(pairings_path: &Path, clones_root: &Path) -> anyhow::Result<Vec<ArchiveEntry>> {
    let raw = std::fs::read_to_string(pairings_path)
        .with_context(|| format!("reading {}", pairings_path.display()))?;
    let parsed: PairingsYaml = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing {}", pairings_path.display()))?;

    let mut entries = Vec::new();
    for pa in parsed.archives {
        if pa.entry_type == "archived" {
            continue;
        }
        let manifest_path: PathBuf = clones_root
            .join(&pa.cluster_name)
            .join(".agent")
            .join("manifest.md");

        let tetrad = if manifest_path.exists() {
            match parse_manifest_tetrad(&manifest_path) {
                Ok(t) => t,
                Err(e) => {
                    warn!(cluster = %pa.cluster_name, error = %e, "failed to parse manifest tetrad");
                    TetradStatus {
                        vendor: LegStatus::Unknown,
                        customer: LegStatus::Unknown,
                        deployment: LegStatus::Unknown,
                        wiki: LegStatus::Unknown,
                    }
                }
            }
        } else {
            TetradStatus {
                vendor: LegStatus::Unknown,
                customer: LegStatus::Unknown,
                deployment: LegStatus::Unknown,
                wiki: LegStatus::Unknown,
            }
        };

        let inbox_pending = inbox_pending_count(clones_root, &pa.cluster_name);

        entries.push(ArchiveEntry {
            module_id: pa.module_id,
            cluster_name: pa.cluster_name,
            slm_endpoint: pa.slm_endpoint,
            tetrad,
            inbox_pending,
        });
    }
    Ok(entries)
}

fn parse_manifest_tetrad(manifest_path: &Path) -> anyhow::Result<TetradStatus> {
    let raw = std::fs::read_to_string(manifest_path)?;
    // Strip YAML frontmatter fences and inline comments from manifest.md
    let yaml_body: String = raw
        .lines()
        .skip_while(|l| !l.trim_start().starts_with("tetrad:"))
        .collect::<Vec<_>>()
        .join("\n");
    if yaml_body.is_empty() {
        return Ok(TetradStatus {
            vendor: LegStatus::Unknown,
            customer: LegStatus::Unknown,
            deployment: LegStatus::Unknown,
            wiki: LegStatus::Unknown,
        });
    }
    // Parse just the tetrad block
    let manifest: Manifest = serde_yaml::from_str(&yaml_body).unwrap_or(Manifest {
        tetrad: ManifestTetrad::default(),
    });
    Ok(TetradStatus {
        vendor: leg_status_from_value(manifest.tetrad.vendor.as_ref()),
        customer: leg_status_from_value(manifest.tetrad.customer.as_ref()),
        deployment: leg_status_from_value(manifest.tetrad.deployment.as_ref()),
        wiki: leg_status_from_value(manifest.tetrad.wiki.as_ref()),
    })
}
