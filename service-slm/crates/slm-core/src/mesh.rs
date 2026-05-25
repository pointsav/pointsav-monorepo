// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Mesh discovery and orchestration metadata.

use serde::{Deserialize, Serialize};

/// Represents an energy source for sustainability metrics.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EnergySource {
    Grid,
    Solar,
    Wind,
    Geothermal,
}

/// Environmental impact metadata for routing decisions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentMetadata {
    pub carbon_intensity: u32,
    pub energy_source: EnergySource,
}

/// Unique identifier for a mesh node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

/// Describes a node within the orchestration mesh.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeDescriptor {
    pub id: NodeId,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub environment: EnvironmentMetadata,
}
