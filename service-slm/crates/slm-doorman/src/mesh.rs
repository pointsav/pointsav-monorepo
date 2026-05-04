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

