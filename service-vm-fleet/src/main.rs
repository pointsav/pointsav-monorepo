use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use std::{net::SocketAddr, sync::Arc};
use system_vm_fleet_types::{
    CreateVmRequest, FleetStatus, NodeHeartbeat, NodeId, NodeRecord, VmId, VmRecord, VmState,
};
use tokio::sync::RwLock;

mod fleet;
mod placement;
mod vm_spawn;

use fleet::NodeRegistry;

#[derive(Clone)]
struct AppState {
    registry: Arc<RwLock<NodeRegistry>>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "service_vm_fleet=info".into()),
        )
        .init();

    let state = AppState {
        registry: Arc::new(RwLock::new(NodeRegistry::new())),
    };

    let app = Router::new()
        .route("/v1/nodes/heartbeat", post(heartbeat_handler))
        .route("/v1/fleet", get(fleet_handler))
        .route("/v1/nodes", get(nodes_handler))
        .route("/v1/nodes/:node_id", get(node_handler))
        .route("/v1/vms", post(create_vm_handler))
        .route("/v1/vms/:vm_id", delete(destroy_vm_handler))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:9203".parse().unwrap();
    tracing::info!("service-vm-fleet listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn heartbeat_handler(
    State(state): State<AppState>,
    Json(hb): Json<NodeHeartbeat>,
) -> StatusCode {
    let mut reg = state.registry.write().await;
    reg.update_node(&hb);
    StatusCode::OK
}

async fn fleet_handler(State(state): State<AppState>) -> Json<FleetStatus> {
    let mut reg = state.registry.write().await;
    reg.evict_stale();
    Json(reg.fleet_status())
}

async fn node_handler(
    State(state): State<AppState>,
    Path(node_id): Path<NodeId>,
) -> Result<Json<NodeRecord>, StatusCode> {
    let mut reg = state.registry.write().await;
    reg.evict_stale();
    match reg.get_node(&node_id) {
        Some(n) => Ok(Json(n)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_vm_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateVmRequest>,
) -> Result<Json<VmRecord>, (StatusCode, String)> {
    let mut reg = state.registry.write().await;
    reg.evict_stale();

    let target = if let Some(pref) = &req.preferred_node {
        // Caller-specified node (required for VmTotebox)
        if reg.get_node(pref).is_none() {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("preferred_node '{pref}' is not registered"),
            ));
        }
        pref.clone()
    } else {
        placement::select_node(&reg, req.ram_mb, req.prefer_kvm).ok_or_else(|| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "insufficient RAM on all registered nodes".to_string(),
            )
        })?
    };

    // Capture kvm_available before releasing the lock.
    let kvm = reg
        .get_node(&target)
        .map(|n| n.kvm_available)
        .unwrap_or(false);

    let vm_id = format!("{}-{}", req.vm_type.to_lowercase(), Utc::now().timestamp());
    let record = VmRecord {
        vm_id: vm_id.clone(),
        vm_type: req.vm_type.clone(),
        state: VmState::Provisioning,
        ram_alloc_mb: req.ram_mb,
        vcpu_count: req.vcpu_count,
        started_at: None,
    };

    reg.register_vm(&target, record.clone());
    tracing::info!(
        vm_id = %vm_id,
        node = %target,
        ram_mb = req.ram_mb,
        kvm,
        "VM provisioning dispatched"
    );

    // Release the lock before the blocking QEMU spawn.
    drop(reg);

    let spawn_record = record.clone();
    let disk_size_gb = (req.ram_mb / 1024).max(8) as u32;
    let _ = tokio::task::spawn_blocking(move || {
        match vm_spawn::create_blank_disk(&spawn_record.vm_id, disk_size_gb) {
            Ok(_) => {
                if let Err(e) = vm_spawn::spawn_qemu(&spawn_record, kvm) {
                    tracing::warn!(vm_id = %spawn_record.vm_id, error = %e, "QEMU spawn failed");
                }
            }
            Err(e) => {
                tracing::warn!(vm_id = %spawn_record.vm_id, error = %e, "disk creation failed");
            }
        }
    });

    Ok(Json(record))
}

async fn destroy_vm_handler(State(state): State<AppState>, Path(vm_id): Path<VmId>) -> StatusCode {
    let mut reg = state.registry.write().await;
    if reg.remove_vm(&vm_id) {
        tracing::info!(vm_id = %vm_id, "VM destroyed");
        drop(reg);
        let id = vm_id.clone();
        let _ = tokio::task::spawn_blocking(move || vm_spawn::kill_qemu(&id));
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn nodes_handler(State(state): State<AppState>) -> Json<Vec<NodeRecord>> {
    let mut reg = state.registry.write().await;
    reg.evict_stale();
    Json(reg.all_nodes())
}
