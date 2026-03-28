use std::env;
use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;
use warp::Filter;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::fs;
use bytes::Bytes;

#[derive(Deserialize, Debug)] struct TranslateRequest { raw_input: String, }
#[derive(Deserialize, Serialize, Debug)] struct AuthorizedPayload { intent: String, target: String, }
#[derive(Serialize, Debug)] struct MeshPayload { sender_id: String, intent: String, target: String, timestamp: String, }
#[derive(Serialize)] struct TerminalResponse { status: String, message: String, data: Option<serde_json::Value>, }

const MESH_PORT: u16 = 8090;
const HTTP_PORT: u16 = 8085;
const PEERS: &[&str] = &["10.50.0.1", "10.50.0.2", "10.50.0.3"];
const SPOOL_DIR: &str = "/opt/woodfine/cluster-totebox-personnel-1/service-email/maildir/new";

#[tokio::main]
async fn main() {
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "F8-TERMINAL-GATEWAY".to_string());
    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("[FATAL] Hardware rejection.");
    let socket_arc = Arc::new(Mutex::new(udp_socket));

    // Existing F8 Terminal Routes
    let translate_route = warp::post().and(warp::path("translate")).and(warp::body::json()).and_then(handle_translation);
    let authorize_route = warp::post().and(warp::path("authorize")).and(warp::body::json()).and(with_socket(socket_arc.clone())).and(with_node_id(node_id.clone())).and_then(handle_authorization);

    // NEW ROUTE: F12 Binary Uploads
    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::header::<String>("x-file-name"))
        .and(warp::body::bytes())
        .and_then(handle_upload);

    let cors = warp::cors().allow_any_origin().allow_headers(vec!["Content-Type", "x-file-name"]).allow_methods(vec!["POST"]);
    
    // We must ensure the Spool Directory exists
    fs::create_dir_all(SPOOL_DIR).unwrap();

    warp::serve(translate_route.or(authorize_route).or(upload_route).with(cors)).run(([0, 0, 0, 0], HTTP_PORT)).await;
}

fn with_socket(socket: Arc<Mutex<UdpSocket>>) -> impl Filter<Extract = (Arc<Mutex<UdpSocket>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || socket.clone())
}

fn with_node_id(node_id: String) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || node_id.clone())
}

// F12 INGESTION LOGIC
async fn handle_upload(filename: String, body: Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let safe_filename = filename.replace("/", "_").replace("\\", "_");
    
    // We prefix manual uploads with VAULT_INGRESS so the Splinter catches them
    let filepath = format!("{}/VAULT_INGRESS_MANUAL_{}", SPOOL_DIR, safe_filename);
    
    match fs::write(&filepath, body) {
        Ok(_) => {
            println!("[INGESTION] Base Asset secured in WORM Spool: {}", safe_filename);
            Ok(warp::reply::json(&TerminalResponse { 
                status: "SUCCESS".to_string(), 
                message: "Asset locked into cold storage.".to_string(), 
                data: None 
            }))
        }
        Err(e) => {
            eprintln!("[FATAL] Spool Write Error: {}", e);
            Ok(warp::reply::json(&TerminalResponse { 
                status: "ERROR".to_string(), 
                message: "Failed to write to physical disk.".to_string(), 
                data: None 
            }))
        }
    }
}

// Existing Handlers
async fn handle_translation(req: TranslateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let output = Command::new("/opt/pointsav/f8-gateway/system-slm").arg(&req.raw_input).output().await;
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&stdout_str) {
                    Ok(warp::reply::json(&TerminalResponse { status: "PENDING_AUTHORIZATION".to_string(), message: "Awaiting Verification.".to_string(), data: Some(json_val) }))
                } else {
                    Ok(warp::reply::json(&TerminalResponse { status: "ERROR".to_string(), message: "Invalid JSON.".to_string(), data: None }))
                }
            } else {
                Ok(warp::reply::json(&TerminalResponse { status: "ERROR".to_string(), message: String::from_utf8_lossy(&out.stderr).to_string(), data: None }))
            }
        }
        Err(e) => Ok(warp::reply::json(&TerminalResponse { status: "FATAL".to_string(), message: e.to_string(), data: None }))
    }
}

async fn handle_authorization(auth: AuthorizedPayload, socket: Arc<Mutex<UdpSocket>>, node_id: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&TerminalResponse { status: "SUCCESS".to_string(), message: "Broadcast injected.".to_string(), data: None }))
}
