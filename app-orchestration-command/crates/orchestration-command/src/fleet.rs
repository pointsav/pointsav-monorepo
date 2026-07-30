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
    // Business-admin/local-only archives (e.g. project-jennifer, project-documents) have
    // no DataGraph tenant relevance and legitimately omit this field in pairings.yaml —
    // required-but-missing on even one entry previously failed the WHOLE Vec<PairingEntry>
    // parse, silently zeroing archives_loaded for the entire fleet (found 2026-07-27).
    #[serde(default)]
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
///
/// The real key is `pairings:` (see `/srv/foundry/pairings.yaml`), not `archives:` —
/// mismatched since the original v0.0.1 scaffold. The parse error
/// (`missing field 'archives'`) was clear enough on its own; the bug survived to
/// production (first deploy 2026-06-29, found+fixed 2026-07-16) because
/// `load_fleet`'s caller in `main.rs` treats any error as non-fatal — logs a WARN and
/// falls back to an empty archive list rather than failing startup — so the very
/// visible error message never had a reason to surface anywhere an operator would see it.
#[derive(Debug, Deserialize)]
struct PairingsYaml {
    pairings: Vec<PairingEntry>,
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
    for pa in parsed.pairings {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for the `archives:` vs `pairings:` top-level key mismatch
    /// (found + fixed 2026-07-16) — mirrors the real pairings.yaml shape, not a
    /// synthetic one, so a future rename of either side breaks this test instead of
    /// silently breaking fleet loading in production again.
    #[test]
    fn parses_real_pairings_yaml_shape() {
        let yaml = r#"
pairings:
  - cluster_name: project-bim
    module_id: bim
    slm_endpoint: http://localhost:9080
    paired_on: 2026-04-23
    type: active
    branch: cluster/project-bim
    staging_required: false
    self_service: build-deploy-stage6lite
  - cluster_name: project-data
    module_id: data
    slm_endpoint: http://localhost:9080
    type: archived

deployments:
  - name: gateway-orchestration-command-1
"#;
        let parsed: PairingsYaml = serde_yaml::from_str(yaml).expect("real-shape pairings.yaml must parse");
        assert_eq!(parsed.pairings.len(), 2);
        assert_eq!(parsed.pairings[0].cluster_name, "project-bim");
        assert_eq!(parsed.pairings[0].module_id, "bim");
        assert_eq!(parsed.pairings[1].entry_type, "archived");
    }

    #[test]
    fn missing_slm_endpoint_defaults() {
        let yaml = r#"
pairings:
  - cluster_name: project-x
    module_id: x
"#;
        let parsed: PairingsYaml = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(parsed.pairings[0].slm_endpoint, "http://localhost:8011");
        assert_eq!(parsed.pairings[0].entry_type, "");
    }

    /// Regression test for the missing-module_id bug (found 2026-07-27): 5 real
    /// pairings.yaml entries (business-admin/local-only archives with no DataGraph
    /// tenant relevance — project-jennifer, project-documents, project-mathew,
    /// project-source, project-woodfine) omit module_id entirely. Before this fix,
    /// module_id was required, so even one such entry failed the WHOLE
    /// Vec<PairingEntry> parse, silently zeroing archives_loaded for the entire
    /// fleet in production — not just the entries actually missing the field.
    #[test]
    fn missing_module_id_defaults_and_does_not_fail_whole_list() {
        let yaml = r#"
pairings:
  - cluster_name: project-bim
    module_id: bim
  - cluster_name: project-jennifer
    branch: cluster/jennifer-sandbox
    self_service: none
    local_only: true
    content_class: business-admin
"#;
        let parsed: PairingsYaml = serde_yaml::from_str(yaml).expect("list must parse even with one entry missing module_id");
        assert_eq!(parsed.pairings.len(), 2);
        assert_eq!(parsed.pairings[0].module_id, "bim");
        assert_eq!(parsed.pairings[1].module_id, "");
    }
}
