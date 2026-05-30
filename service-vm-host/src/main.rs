use chrono::Utc;
use std::time::Duration;
use system_vm_fleet_types::{NodeHeartbeat, VmRecord};
use tokio::time::sleep;

mod host_stats;
mod qemu_monitor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "service_vm_host=info".into()),
        )
        .init();

    let fleet_endpoint = std::env::var("VM_FLEET_ENDPOINT")
        .expect("VM_FLEET_ENDPOINT must be set (e.g. http://10.8.0.9:9203)");
    let node_id = std::env::var("VM_NODE_ID")
        .expect("VM_NODE_ID must be set (e.g. gcp-cloud-1)");
    let wg_ip = std::env::var("VM_WG_IP")
        .expect("VM_WG_IP must be set (e.g. 10.8.0.9)");
    let interval_secs: u64 = std::env::var("VM_HEARTBEAT_INTERVAL_S")
        .unwrap_or_else(|_| "10".into())
        .parse()
        .expect("VM_HEARTBEAT_INTERVAL_S must be a positive integer");

    let hostname = hostname();
    let boot_id = read_boot_id();
    let client = reqwest::Client::new();
    let heartbeat_url = format!("{}/v1/nodes/heartbeat", fleet_endpoint.trim_end_matches('/'));

    tracing::info!(
        node_id = %node_id,
        wg_ip = %wg_ip,
        fleet_endpoint = %fleet_endpoint,
        interval_secs,
        "service-vm-host starting"
    );

    loop {
        let stats = host_stats::read_host_stats();
        let vms: Vec<VmRecord> = qemu_monitor::poll_running_vms();

        let hb = NodeHeartbeat {
            node_id: node_id.clone(),
            wg_ip: wg_ip.clone(),
            hostname: hostname.clone(),
            ram_total_mb: stats.ram_total_mb,
            ram_used_mb: stats.ram_used_mb,
            cpu_cores: stats.cpu_cores,
            cpu_load_pct: stats.cpu_load_pct,
            kvm_available: stats.kvm_available,
            vms,
            boot_id: boot_id.clone(),
            timestamp_utc: Utc::now(),
        };

        match client.post(&heartbeat_url).json(&hb).send().await {
            Ok(resp) if resp.status().is_success() => {
                tracing::debug!("heartbeat sent ok");
            }
            Ok(resp) => {
                tracing::warn!(status = %resp.status(), "fleet controller returned error");
            }
            Err(e) => {
                tracing::warn!(error = %e, "failed to send heartbeat");
            }
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

fn read_boot_id() -> String {
    std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}
