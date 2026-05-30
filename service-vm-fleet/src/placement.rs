use system_vm_fleet_types::NodeId;

use crate::fleet::NodeRegistry;

/// RAM headroom reserved above the requested allocation.
const SAFETY_MARGIN_MB: u64 = 512;

/// Advisory placement: select the node with the most available RAM above the safety margin.
///
/// When `prefer_kvm` is true, a KVM-capable node is chosen if one has sufficient RAM;
/// otherwise falls back to any node (including TCG-only). This allows VM-Totebox and
/// VM-PrivateGit to land on Laptop A/B while VM-MediaKit tolerates GCP TCG.
///
/// auto_rebalance is permanently false — this function is called once per CreateVm request
/// and that decision is final. No background rebalancing occurs.
pub fn select_node(registry: &NodeRegistry, ram_mb: u64, prefer_kvm: bool) -> Option<NodeId> {
    let required = ram_mb + SAFETY_MARGIN_MB;

    if prefer_kvm {
        let kvm_choice = registry
            .nodes_iter()
            .filter(|n| n.ram_available_mb >= required && n.kvm_available)
            .max_by_key(|n| n.ram_available_mb)
            .map(|n| n.node_id.clone());
        if kvm_choice.is_some() {
            return kvm_choice;
        }
    }

    // Fall back to any eligible node (TCG included)
    registry
        .nodes_iter()
        .filter(|n| n.ram_available_mb >= required)
        .max_by_key(|n| n.ram_available_mb)
        .map(|n| n.node_id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fleet::NodeRegistry;
    use chrono::Utc;
    use system_vm_fleet_types::NodeHeartbeat;

    fn make_heartbeat(node_id: &str, ram_total: u64, ram_used: u64) -> NodeHeartbeat {
        make_heartbeat_kvm(node_id, ram_total, ram_used, true)
    }

    fn make_heartbeat_kvm(node_id: &str, ram_total: u64, ram_used: u64, kvm: bool) -> NodeHeartbeat {
        NodeHeartbeat {
            node_id: node_id.to_string(),
            wg_ip: "10.8.0.9".to_string(),
            hostname: node_id.to_string(),
            ram_total_mb: ram_total,
            ram_used_mb: ram_used,
            cpu_cores: 4,
            cpu_load_pct: 5.0,
            kvm_available: kvm,
            vms: vec![],
            boot_id: "boot-1".to_string(),
            timestamp_utc: Utc::now(),
        }
    }

    #[test]
    fn selects_node_with_most_ram() {
        let mut reg = NodeRegistry::new();
        reg.update_node(&make_heartbeat("node-a", 8192, 6144)); // 2048 available
        reg.update_node(&make_heartbeat("node-b", 16384, 4096)); // 12288 available
        // Request 1024 MB — both qualify; node-b has more RAM
        let result = select_node(&reg, 1024, false);
        assert_eq!(result, Some("node-b".to_string()));
    }

    #[test]
    fn returns_none_when_all_nodes_below_threshold() {
        let mut reg = NodeRegistry::new();
        reg.update_node(&make_heartbeat("node-a", 4096, 3800)); // 296 available < 1024+512
        let result = select_node(&reg, 1024, false);
        assert_eq!(result, None);
    }

    #[test]
    fn safety_margin_enforced() {
        let mut reg = NodeRegistry::new();
        // Exactly at request + safety margin (512 + 512 = 1024 available)
        reg.update_node(&make_heartbeat("node-a", 4096, 3072)); // 1024 available
        // Requesting 512 MB — needs 512 + 512 = 1024 MB available — exactly meets threshold
        assert!(select_node(&reg, 512, false).is_some());
        // Requesting 513 MB — needs 513 + 512 = 1025 MB — exceeds available
        assert!(select_node(&reg, 513, false).is_none());
    }

    #[test]
    fn empty_registry_returns_none() {
        let reg = NodeRegistry::new();
        assert_eq!(select_node(&reg, 1024, false), None);
    }

    #[test]
    fn prefer_kvm_selects_kvm_node_when_available() {
        let mut reg = NodeRegistry::new();
        // laptop-a: KVM, 8192 MB total, 2048 used → 6144 available
        reg.update_node(&make_heartbeat_kvm("laptop-a", 8192, 2048, true));
        // gcp-tcg: no KVM, same RAM available
        reg.update_node(&make_heartbeat_kvm("gcp-tcg", 8192, 2048, false));
        let result = select_node(&reg, 1024, true);
        assert_eq!(result, Some("laptop-a".to_string()));
    }

    #[test]
    fn prefer_kvm_falls_back_to_tcg_when_no_kvm_nodes() {
        let mut reg = NodeRegistry::new();
        reg.update_node(&make_heartbeat_kvm("gcp-tcg", 8192, 2048, false));
        // prefer_kvm=true but no KVM node available — should still return gcp-tcg
        let result = select_node(&reg, 1024, true);
        assert_eq!(result, Some("gcp-tcg".to_string()));
    }
}
