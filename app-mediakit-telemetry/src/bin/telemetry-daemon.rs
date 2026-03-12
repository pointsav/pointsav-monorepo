use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::SocketAddr;
use serde::Deserialize;
use warp::Filter;

// DS-ADR-06: Strict Payload Definition
#[derive(Deserialize, Debug)]
struct TelemetryPayload {
    uri: String,
    timestamp: String,
    user_agent: String,
}

#[tokio::main]
async fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let port: u16 = port_str.parse().expect("[ERROR] Invalid Port assignment.");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    let telemetry_route = warp::post()
        .and(warp::path("telemetry-endpoint"))
        .and(warp::addr::remote())
        .and(warp::body::json())
        .map(|addr: Option<SocketAddr>, payload: TelemetryPayload| {
            let raw_ip = addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "0.0.0.0".to_string());
            let masked_ip = mask_ip_address(&raw_ip);
            
            let ledger_entry = format!("\"{}\",\"{}\",\"{}\",\"{}\"\n", 
                masked_ip, payload.timestamp, payload.uri, payload.user_agent);
            
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("assets/ledger_telemetry.csv") {
                let _ = file.write_all(ledger_entry.as_bytes());
            } else {
                eprintln!("[ERROR] Vault access denied: Cannot write to ledger.");
            }
            
            warp::reply::json(&"ACK_SOVEREIGN")
        })
        .with(cors);

    println!("[SYSTEM] Sovereign Telemetry Daemon (TLS ENABLED) active on port {}", port);
    
    // STRICT ENFORCEMENT: Native Rust TLS Termination
    warp::serve(telemetry_route)
        .tls()
        .cert_path("assets/cert.pem")
        .key_path("assets/key.pem")
        .run(([0, 0, 0, 0], port)).await;
}

fn mask_ip_address(ip: &str) -> String {
    let mut parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        parts[3] = "0"; 
        return parts.join(".");
    }
    "Masked-IPv6".to_string() 
}
