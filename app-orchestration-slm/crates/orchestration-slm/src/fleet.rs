// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Fleet registry — in-memory map of registered Totebox Archives.
//!
//! The chassis is stateless: this map is built from `POST /v1/discovery/register`
//! calls made at Doorman startup. On chassis restart, Doormans re-register on
//! their next startup (SLM_ORCHESTRATION_ENDPOINT health retry — Phase 3).

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use orchestration_slm_core::{FleetMember, FleetMemberSummary, FleetResponse, RegistrationRequest};
use tokio::sync::RwLock;
use tracing::info;

use crate::error::{ChassisError, Result};

/// Thread-safe fleet registry.
#[derive(Debug, Default)]
pub struct FleetRegistry {
    inner: RwLock<HashMap<String, FleetMember>>,
}

impl FleetRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Register or update a Totebox Archive.
    pub async fn register(&self, req: RegistrationRequest) -> FleetMember {
        let member = FleetMember {
            module_id: req.module_id.clone(),
            archive_id: req.archive_id,
            doorman_endpoint: req.doorman_endpoint,
            tier_b_subscribed: req.tier_b_subscribed,
            registered_at: Utc::now(),
        };
        let mut map = self.inner.write().await;
        map.insert(req.module_id.clone(), member.clone());
        info!(module_id = %req.module_id, "fleet: Totebox registered");
        member
    }

    /// Look up a member by module-id. Returns None if not registered.
    pub async fn get(&self, module_id: &str) -> Option<FleetMember> {
        self.inner.read().await.get(module_id).cloned()
    }

    /// Return the current fleet as a summary listing.
    pub async fn list(&self) -> FleetResponse {
        let map = self.inner.read().await;
        let members: Vec<FleetMemberSummary> = map
            .values()
            .map(|m| FleetMemberSummary {
                module_id: m.module_id.clone(),
                archive_id: m.archive_id.clone(),
                tier_b_subscribed: m.tier_b_subscribed,
                registered_at: m.registered_at,
            })
            .collect();
        let total = members.len();
        FleetResponse { members, total }
    }

    pub async fn member_count(&self) -> usize {
        self.inner.read().await.len()
    }

    /// Authenticate a Yo-Yo proxy request.
    ///
    /// `bearer_module_id` — module-id extracted from `Authorization: Bearer <id>`
    /// `header_module_id` — value of `X-Foundry-Module-ID` request header
    ///
    /// Rules:
    /// 1. `bearer_module_id` must be registered in the fleet → else 401.
    /// 2. `header_module_id` (if present) must match `bearer_module_id` → else 403.
    /// 3. Registered member must be `tier_b_subscribed` → else 402.
    pub async fn authenticate_proxy(
        &self,
        bearer_module_id: &str,
        header_module_id: Option<&str>,
    ) -> Result<FleetMember> {
        let member = self
            .get(bearer_module_id)
            .await
            .ok_or(ChassisError::Unauthenticated)?;

        if let Some(hid) = header_module_id {
            if hid != bearer_module_id {
                return Err(ChassisError::ModuleIdMismatch);
            }
        }

        if !member.tier_b_subscribed {
            return Err(ChassisError::NotSubscribed(member.module_id.clone()));
        }

        Ok(member)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orchestration_slm_core::RegistrationRequest;

    fn make_req(module_id: &str, subscribed: bool) -> RegistrationRequest {
        RegistrationRequest {
            module_id: module_id.to_string(),
            archive_id: "test-archive".to_string(),
            doorman_endpoint: "http://127.0.0.1:9080".to_string(),
            tier_b_subscribed: subscribed,
        }
    }

    #[tokio::test]
    async fn register_and_list() {
        let reg = FleetRegistry::new();
        reg.register(make_req("op::a::slm", true)).await;
        reg.register(make_req("op::b::slm", false)).await;
        let fleet = reg.list().await;
        assert_eq!(fleet.total, 2);
    }

    #[tokio::test]
    async fn authenticate_proxy_unregistered_is_401() {
        let reg = FleetRegistry::new();
        let err = reg.authenticate_proxy("op::nobody::slm", None).await.unwrap_err();
        assert!(matches!(err, ChassisError::Unauthenticated));
    }

    #[tokio::test]
    async fn authenticate_proxy_spoofing_is_403() {
        let reg = FleetRegistry::new();
        reg.register(make_req("op::a::slm", true)).await;
        let err = reg
            .authenticate_proxy("op::a::slm", Some("op::b::slm"))
            .await
            .unwrap_err();
        assert!(matches!(err, ChassisError::ModuleIdMismatch));
    }

    #[tokio::test]
    async fn authenticate_proxy_not_subscribed_is_402() {
        let reg = FleetRegistry::new();
        reg.register(make_req("op::a::slm", false)).await;
        let err = reg.authenticate_proxy("op::a::slm", None).await.unwrap_err();
        assert!(matches!(err, ChassisError::NotSubscribed(_)));
    }

    #[tokio::test]
    async fn authenticate_proxy_subscribed_ok() {
        let reg = FleetRegistry::new();
        reg.register(make_req("op::a::slm", true)).await;
        let member = reg
            .authenticate_proxy("op::a::slm", Some("op::a::slm"))
            .await
            .unwrap();
        assert_eq!(member.module_id, "op::a::slm");
    }
}
