use std::collections::HashMap;

pub struct TenantConfig {
    pub tenant_id: String,
    pub max_vms: u32,
    pub max_ram_mb: u64,
}

pub struct TenantRegistry {
    tenants: HashMap<String, TenantConfig>,
}

impl TenantRegistry {
    /// Build from environment variables.
    ///
    /// TENANT_IDS=alice,bob
    /// TENANT_ALICE_MAX_VMS=3          (optional; default 5)
    /// TENANT_ALICE_MAX_RAM_MB=4096    (optional; default 8192)
    pub fn from_env() -> Self {
        let ids_raw = std::env::var("TENANT_IDS").unwrap_or_default();
        let mut tenants = HashMap::new();

        for id in ids_raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let upper = id.to_uppercase();
            let max_vms = std::env::var(format!("TENANT_{upper}_MAX_VMS"))
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5u32);
            let max_ram_mb = std::env::var(format!("TENANT_{upper}_MAX_RAM_MB"))
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8192u64);

            tenants.insert(
                id.to_string(),
                TenantConfig {
                    tenant_id: id.to_string(),
                    max_vms,
                    max_ram_mb,
                },
            );
            tracing::info!(tenant_id = id, max_vms, max_ram_mb, "registered tenant");
        }

        TenantRegistry { tenants }
    }

    /// Validate a Bearer token and return the matching config.
    /// In Phase 1 the token IS the tenant_id — opaque from the caller's perspective
    /// (caller supplies it, the registry accepts or rejects it).
    pub fn authenticate<'a>(&'a self, bearer: &str) -> Option<&'a TenantConfig> {
        self.tenants.get(bearer)
    }

    pub fn is_empty(&self) -> bool {
        self.tenants.is_empty()
    }
}

/// Extract the bearer token from an Authorization header value.
/// Accepts "Bearer <token>" or just "<token>".
pub fn extract_bearer(auth_header: &str) -> &str {
    auth_header
        .strip_prefix("Bearer ")
        .unwrap_or(auth_header)
        .trim()
}
