// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Per-tenant cost metering.
//!
//! The metering ledger is in-process and rebuildable from per-Archive Doorman
//! ledgers on restart (stateless rule). Each Yo-Yo proxy request records its
//! cost here; the chassis serves a read-only rollup at GET /v1/audit/rollup
//! (Phase 2).
//!
//! Cost is computed from the `X-Foundry-Inference-Ms` response header emitted
//! by the Yo-Yo VM, multiplied by the configured hourly USD rate for that node.
//! If the header is absent (Yo-Yo down or timed out), cost is recorded as 0.0.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::debug;

#[derive(Debug, Default, Clone)]
pub struct TenantStats {
    pub total_requests: u64,
    pub total_cost_usd: f64,
    pub total_inference_ms: u64,
}

#[derive(Debug, Default)]
pub struct MeteringLedger {
    stats: Mutex<HashMap<String, TenantStats>>,
}

impl MeteringLedger {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn record(&self, module_id: &str, inference_ms: Option<u64>, hourly_usd_rate: f64) {
        let ms = inference_ms.unwrap_or(0);
        let cost = hourly_usd_rate * (ms as f64 / 3_600_000.0);

        let mut map = self.stats.lock().await;
        let entry = map.entry(module_id.to_string()).or_default();
        entry.total_requests += 1;
        entry.total_cost_usd += cost;
        entry.total_inference_ms += ms;

        debug!(
            module_id,
            inference_ms = ms,
            cost_usd = cost,
            "metering: request recorded"
        );
    }

    pub async fn get(&self, module_id: &str) -> Option<TenantStats> {
        self.stats.lock().await.get(module_id).cloned()
    }

    pub async fn all(&self) -> HashMap<String, TenantStats> {
        self.stats.lock().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn records_cost() {
        let ledger = MeteringLedger::new();
        // 1000 ms at $0.84/hr → $0.84/3600 ≈ $0.000233
        ledger.record("op::a::slm", Some(1000), 0.84).await;
        ledger.record("op::a::slm", Some(2000), 0.84).await;
        let stats = ledger.get("op::a::slm").await.unwrap();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.total_inference_ms, 3000);
        assert!(stats.total_cost_usd > 0.0);
    }
}
