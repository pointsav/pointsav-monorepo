// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Personnel reader — derives contributor permission tiers from pairings.yaml.
//!
//! pairings.yaml declares archive pairings; permission tier is inferred
//! from the set of archives a unix user is listed against.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use serde::Deserialize;

use orchestration_command_core::{PermissionTier, PersonnelEntry};

/// Minimal contributor schema inside pairings.yaml.
#[derive(Debug, Deserialize)]
struct ContributorEntry {
    unix_user: String,
    #[serde(default)]
    tier: String,
    #[serde(default)]
    paired_archives: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PairingsYamlContributors {
    #[serde(default)]
    contributors: Vec<ContributorEntry>,
}

fn parse_tier(s: &str) -> PermissionTier {
    match s {
        "P1" => PermissionTier::P1,
        "P2" => PermissionTier::P2,
        "P4" => PermissionTier::P4,
        _ => PermissionTier::P3,
    }
}

/// Load all contributors from `pairings_path`.
pub fn load_personnel(pairings_path: &Path) -> anyhow::Result<HashMap<String, PersonnelEntry>> {
    let raw = std::fs::read_to_string(pairings_path)
        .with_context(|| format!("reading {}", pairings_path.display()))?;
    let parsed: PairingsYamlContributors = serde_yaml::from_str(&raw).unwrap_or(
        PairingsYamlContributors {
            contributors: vec![],
        },
    );

    let mut map = HashMap::new();
    for c in parsed.contributors {
        map.insert(
            c.unix_user.clone(),
            PersonnelEntry {
                unix_user: c.unix_user,
                tier: parse_tier(&c.tier),
                pairing_set: c.paired_archives,
            },
        );
    }
    Ok(map)
}
