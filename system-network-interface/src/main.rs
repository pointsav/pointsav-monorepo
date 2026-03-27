use std::env;
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;
use warp::Filter;
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// PointSav Digital Systems: F8 Terminal Gateway (HITL Enforced)
/// Standard: SYS-ADR-10 (The Fiduciary Interface)

#[derive(Deserialize, Debug)]
struct TranslateRequest {
    raw_input: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct AuthorizedPayload {
    intent: String,
    target: String,
}

#[derive(Serialize, Debug)]
struct MeshPayload {
    sender_id: String,
    intent: String,
    target: String,
    timestamp: String,
}

#[derive(Serialize)]
struct TerminalResponse {
    status: String,
    message: String,
    data: Option<serde_json::Value>,
}

const MESH_PORT: u16 = 8090;
const BROADCAST_ADDR: &str = "10.50.0.255";
const HTTP_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    println!("========================================================");
    println!(" 🛡️ SYSTEM-NETWORK-INTERFACE: HITL GATEWAY ACTIVE");
    println!("========================================================");
    
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "F8-TERMINAL-GATEWAY".to_string());
    println!("[SYSTEM] Node Identity Locked: {}", node_id);
    println!("[SYSTEM] Binding HTTP Listener to 0.0.0.0:{}", HTTP_PORT);

    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("[FATAL] Hardware rejection. Could not open UDP socket.");
    udp_socket.set_broadcast(true).expect("[FATAL] Kernel rejection. Could not enable broadcast physics.");
    let socket_arc = Arc::new(Mutex::new(udp_socket));

    // ROUTE 1: Translate (Human Intent -> Machine Proposal)
    let translate_route = warp::post()
        .and(warp::path("translate"))
        .and(warp::body::json())
        .and_then(handle_translation);

    // ROUTE 2: Authorize (Verified Proposal -> UDP Broadcast)
    let authorize_route = warp::post()
        .and(warp::path("authorize"))
        .and(warp::body::json())
        .and(with_socket(socket_arc.clone()))
        .and(with_node_id(node_id.clone()))
        .and_then(handle_authorization);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST"]);

    let routes = translate_route.or(authorize_route).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], HTTP_PORT)).await;
}

fn with_socket(socket: Arc<Mutex<UdpSocket>>) -> impl Filter<Extract = (Arc<Mutex<UdpSocket>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || socket.clone())
}

fn with_node_id(node_id: String) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || node_id.clone())
}

// STEP 1: Execute system-slm to parse the intent
async fn handle_translation(req: TranslateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[GATEWAY] Translation requested for: '{}'", req.raw_input);
    
    // Spawn the system-slm binary
    let output = Command::new("./system-slm") // Assumes binary is in the same directory path during execution
        .arg(&req.raw_input)
        .output()
        .await;

    match output {
        Ok(out) => {
            let stdout_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr_str = String::from_utf8_lossy(&out.stderr).trim().to_string();

            if out.status.success() {
                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&stdout_str) {
                    let response = TerminalResponse {
                        status: "PENDING_AUTHORIZATION".to_string(),
                        message: "Proposal generated. Awaiting Human-in-the-Loop verification.".to_string(),
                        data: Some(json_val),
                    };
                    Ok(warp::reply::json(&response))
                } else {
                    let response = TerminalResponse {
                        status: "ERROR".to_string(),
                        message: "SLM output invalid JSON.".to_string(),
                        data: None,
                    };
                    Ok(warp::reply::json(&response))
                }
            } else {
                let response = TerminalResponse {
                    status: "ERROR".to_string(),
                    message: stderr_str,
                    data: None,
                };
                Ok(warp::reply::json(&response))
            }
        }
        Err(e) => {
            let response = TerminalResponse {
                status: "FATAL".to_string(),
                message: format!("Failed to spawn system-slm compiler: {}", e),
                data: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// STEP 2: Broadcast the authorized payload
async fn handle_authorization(
    auth: AuthorizedPayload,
    socket: Arc<Mutex<UdpSocket>>,
    node_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let timestamp = Utc::now().to_rfc3339();

    let payload = MeshPayload {
        sender_id: node_id,
        intent: auth.intent.clone(),
        target: auth.target.clone(),
        timestamp: timestamp.clone(),
    };

    let data = serde_json::to_string(&payload).unwrap();
    let target_addr = format!("{}:{}", BROADCAST_ADDR, MESH_PORT);

    let sock = socket.lock().await;
    match sock.send_to(data.as_bytes(), &target_addr) {
        Ok(_) => {
            println!("[MESH INJECT] Authorized intent broadcast: {} -> {}", auth.intent, auth.target);
            let response = TerminalResponse {
                status: "SUCCESS".to_string(),
                message: "Payload broadcast to Sovereign Mesh.".to_string(),
                data: Some(serde_json::json!({"timestamp": timestamp})),
            };
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            eprintln!("[ERROR] Mesh injection failed: {}", e);
            let response = TerminalResponse {
                status: "FATAL".to_string(),
                message: format!("Hardware rejection: {}", e),
                data: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}
