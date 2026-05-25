// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Mesh discovery and registry for orchestrating compute.

use async_trait::async_trait;
use slm_core::{ComputeRequest, NodeDescriptor};

/// Discovery provider for dynamic node discovery.
#[async_trait]
pub trait DiscoveryProvider: Send + Sync {
    async fn poll_nodes(&self) -> crate::error::Result<Vec<NodeDescriptor>>;
}

/// Registry of mesh nodes.
#[async_trait]
pub trait MeshRegistry: Send + Sync {
    /// Discover available nodes in the mesh.
    async fn discover_nodes(&self) -> Vec<NodeDescriptor>;

    /// Select an optimal node based on request intent.
    async fn select_optimal(&self, req: &ComputeRequest) -> Option<NodeDescriptor>;
}

// ── StaticConfigProvider ─────────────────────────────────────────────────────

/// Discovery provider backed by a fixed list of nodes supplied at construction.
/// Suitable for known, pre-configured nodes (e.g. named Yo-Yo nodes "trainer"
/// and "graph") where dynamic discovery is unnecessary. `poll_nodes()` always
/// returns the same list; `DynamicRegistry` stores it after the first tick.
pub struct StaticConfigProvider {
    nodes: Vec<NodeDescriptor>,
}

impl StaticConfigProvider {
    pub fn new(nodes: Vec<NodeDescriptor>) -> Self {
        Self { nodes }
    }
}

#[async_trait]
impl DiscoveryProvider for StaticConfigProvider {
    async fn poll_nodes(&self) -> crate::error::Result<Vec<NodeDescriptor>> {
        Ok(self.nodes.clone())
    }
}

// ── DynamicRegistry ──────────────────────────────────────────────────────────

use std::sync::{Arc, RwLock};
use tokio::time::{self, Duration};

/// Dynamic registry that polls a provider for node updates.
pub struct DynamicRegistry {
    nodes: Arc<RwLock<Vec<NodeDescriptor>>>,
}

impl DynamicRegistry {
    pub fn new(provider: Arc<dyn DiscoveryProvider>, interval: Duration) -> Self {
        let nodes = Arc::new(RwLock::new(Vec::new()));
        let nodes_clone = nodes.clone();

        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            loop {
                ticker.tick().await;
                if let Ok(new_nodes) = provider.poll_nodes().await {
                    let mut guard = nodes_clone.write().unwrap();
                    *guard = new_nodes;
                }
            }
        });

        Self { nodes }
    }
}

#[async_trait]
impl MeshRegistry for DynamicRegistry {
    async fn discover_nodes(&self) -> Vec<NodeDescriptor> {
        self.nodes.read().unwrap().clone()
    }

    async fn select_optimal(&self, _req: &ComputeRequest) -> Option<NodeDescriptor> {
        self.nodes.read().unwrap().first().cloned()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{EnergySource, EnvironmentMetadata, NodeId};

    fn make_node(id: &str, endpoint: &str) -> NodeDescriptor {
        NodeDescriptor {
            id: NodeId(id.to_string()),
            endpoint: endpoint.to_string(),
            capabilities: vec!["inference".to_string()],
            environment: EnvironmentMetadata {
                carbon_intensity: 0,
                energy_source: EnergySource::Grid,
            },
        }
    }

    #[tokio::test]
    async fn static_provider_returns_all_nodes() {
        let nodes = vec![
            make_node("trainer", "http://trainer.example:8080"),
            make_node("graph", "http://graph.example:8080"),
        ];
        let provider = StaticConfigProvider::new(nodes.clone());
        let result = provider.poll_nodes().await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id.0, "trainer");
        assert_eq!(result[1].id.0, "graph");
    }

    #[tokio::test]
    async fn static_provider_empty_list() {
        let provider = StaticConfigProvider::new(vec![]);
        let result = provider.poll_nodes().await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn dynamic_registry_select_optimal_returns_first_after_poll() {
        let nodes = vec![
            make_node("trainer", "http://trainer.example:8080"),
            make_node("graph", "http://graph.example:8080"),
        ];
        let provider = Arc::new(StaticConfigProvider::new(nodes));
        // Seed the registry directly (bypass the background ticker interval)
        let registry = DynamicRegistry {
            nodes: Arc::new(RwLock::new(
                provider.poll_nodes().await.unwrap(),
            )),
        };
        let dummy_req = {
            use slm_core::{ChatMessage, Complexity, ModuleId, RequestId};
            use std::str::FromStr;
            ComputeRequest {
                request_id: RequestId::new(),
                module_id: ModuleId::from_str("foundry").unwrap(),
                model: None,
                messages: vec![ChatMessage { role: "user".into(), content: "hi".into() }],
                complexity: Complexity::High,
                tier_hint: None,
                stream: false,
                max_tokens: None,
                temperature: None,
                sanitised_outbound: true,
                tier_c_label: None,
                yoyo_label: None,
                grammar: None,
                speculation: None,
                graph_context_enabled: None,
            }
        };
        let selected = registry.select_optimal(&dummy_req).await;
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id.0, "trainer");
    }

    #[tokio::test]
    async fn dynamic_registry_empty_returns_none() {
        let registry = DynamicRegistry {
            nodes: Arc::new(RwLock::new(vec![])),
        };
        let dummy_req = {
            use slm_core::{ChatMessage, Complexity, ModuleId, RequestId};
            use std::str::FromStr;
            ComputeRequest {
                request_id: RequestId::new(),
                module_id: ModuleId::from_str("foundry").unwrap(),
                model: None,
                messages: vec![ChatMessage { role: "user".into(), content: "hi".into() }],
                complexity: Complexity::Medium,
                tier_hint: None,
                stream: false,
                max_tokens: None,
                temperature: None,
                sanitised_outbound: true,
                tier_c_label: None,
                yoyo_label: None,
                grammar: None,
                speculation: None,
                graph_context_enabled: None,
            }
        };
        let selected = registry.select_optimal(&dummy_req).await;
        assert!(selected.is_none());
    }
}

