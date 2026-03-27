use std::env;
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;
use warp::Filter;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Deserialize, Debug)] struct TranslateRequest { raw_input: String, }
#[derive(Deserialize, Serialize, Debug)] struct AuthorizedPayload { intent: String, target: String, }
#[derive(Serialize, Debug)] struct MeshPayload { sender_id: String, intent: String, target: String, timestamp: String, }
#[derive(Serialize)] struct TerminalResponse { status: String, message: String, data: Option<serde_json::Value>, }

const MESH_PORT: u16 = 8090;
const HTTP_PORT: u16 = 8085;
// The exact IPs of the PointSav Secure Tunnel (PSST)
const PEERS: &[&str] = &["10.50.0.1", "10.50.0.2", "10.50.0.3"];

#[tokio::main]
async fn main() {
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "F8-TERMINAL-GATEWAY".to_string());
    
    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("[FATAL] Hardware rejection. Could not open UDP socket.");
    let socket_arc = Arc::new(Mutex::new(udp_socket));

    let translate_route = warp::post()
        .and(warp::path("translate"))
        .and(warp::body::json())
        .and_then(handle_translation);

    let authorize_route = warp::post()
        .and(warp::path("authorize"))
        .and(warp::body::json())
        .and(with_socket(socket_arc.clone()))
        .and(with_node_id(node_id.clone()))
        .and_then(handle_authorization);

    let cors = warp::cors().allow_any_origin().allow_headers(vec!["Content-Type"]).allow_methods(vec!["POST"]);
    warp::serve(translate_route.or(authorize_route).with(cors)).run(([0, 0, 0, 0], HTTP_PORT)).await;
}

fn with_socket(socket: Arc<Mutex<UdpSocket>>) -> impl Filter<Extract = (Arc<Mutex<UdpSocket>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || socket.clone())
}

fn with_node_id(node_id: String) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || node_id.clone())
}

async fn handle_translation(req: TranslateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Absolute path execution required for Systemd
    let output = Command::new("/opt/pointsav/f8-gateway/system-slm")
        .arg(&req.raw_input).output().await;

    match output {
        Ok(out) => {
            let stdout_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr_str = String::from_utf8_lossy(&out.stderr).trim().to_string();
            if out.status.success() {
                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&stdout_str) {
                    Ok(warp::reply::json(&TerminalResponse { status: "PENDING_AUTHORIZATION".to_string(), message: "Awaiting Verification.".to_string(), data: Some(json_val) }))
                } else {
                    Ok(warp::reply::json(&TerminalResponse { status: "ERROR".to_string(), message: "Invalid JSON.".to_string(), data: None }))
                }
            } else {
                Ok(warp::reply::json(&TerminalResponse { status: "ERROR".to_string(), message: stderr_str, data: None }))
            }
        }
        Err(e) => Ok(warp::reply::json(&TerminalResponse { status: "FATAL".to_string(), message: e.to_string(), data: None }))
    }
}

async fn handle_authorization(auth: AuthorizedPayload, socket: Arc<Mutex<UdpSocket>>, node_id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let timestamp = Utc::now().to_rfc3339();
    let payload = MeshPayload { sender_id: node_id, intent: auth.intent.clone(), target: auth.target.clone(), timestamp: timestamp.clone() };
    let data = serde_json::to_string(&payload).unwrap();
    
    // WIREGUARD COMPATIBLE UNICAST ITERATION (Bypassing .255 limitation)
    let sock = socket.lock().await;
    let mut success_count = 0;
    let mut last_error = String::new();

    let target_ips: Vec<&str> = match auth.target.as_str() {
        "NODE-CLOUD-RELAY" => vec!["10.50.0.1"],
        "NODE-LAPTOP-A" => vec!["10.50.0.2"],
        "NODE-IMAC-12" => vec!["10.50.0.3"],
        "ALL" | _ => PEERS.to_vec(),
    };

    for ip in target_ips {
        let target_addr = format!("{}:{}", ip, MESH_PORT);
        match sock.send_to(data.as_bytes(), &target_addr) {
            Ok(_) => success_count += 1,
            Err(e) => last_error = e.to_string(),
        }
    }

    if success_count > 0 {
        Ok(warp::reply::json(&TerminalResponse { status: "SUCCESS".to_string(), message: format!("Payload injected to {} mesh nodes.", success_count), data: Some(serde_json::json!({"timestamp": timestamp})) }))
    } else {
        Ok(warp::reply::json(&TerminalResponse { status: "FATAL".to_string(), message: format!("Hardware rejection: {}", last_error), data: None }))
    }
}
