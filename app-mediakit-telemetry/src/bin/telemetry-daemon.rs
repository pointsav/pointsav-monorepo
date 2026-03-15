use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use serde::{Deserialize, Serialize};
use chrono::Utc;

// [DS-ADR-06] The V4 Sovereign Telemetry Payload
// Option<T> ensures absolute backward compatibility with cached V3 payloads.
#[derive(Debug, Deserialize, Serialize)]
struct TelemetryPayload {
    uri: String,
    timestamp: String,
    user_agent: String,
    referrer: Option<String>,
    viewport: Option<String>,
    timezone: Option<String>,
    device_memory: Option<String>,
    hardware_cores: Option<String>,
    dwell_seconds: Option<i64>,
    scroll_depth: Option<i64>,
    intent_clicks: Option<String>,
    lang_browser: Option<String>,
    network: Option<String>,
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let ledger_path = Arc::new(Mutex::new("./assets/ledger_telemetry.csv".to_string()));

    println!("SOVEREIGN DIODE ACTIVE: Binding to {}", addr);

    let telemetry_route = warp::post()
        .and(warp::path("telemetry-endpoint"))
        .and(warp::body::json())
        .and(with_ledger(ledger_path.clone()))
        .and_then(handle_telemetry);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST"])
        .allow_headers(vec!["Content-Type"]);

    warp::serve(telemetry_route.with(cors))
        .run(addr)
        .await;
}

fn with_ledger(
    ledger: Arc<Mutex<String>>,
) -> impl Filter<Extract = (Arc<Mutex<String>>,), Error = Infallible> + Clone {
    warp::any().map(move || ledger.clone())
}

async fn handle_telemetry(
    payload: TelemetryPayload,
    ledger: Arc<Mutex<String>>,
) -> Result<impl warp::Reply, Infallible> {
    let path = ledger.lock().await;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*path)
        .expect("CRITICAL FAILURE: Cannot access immutable ledger.");

    let safe_clicks = payload.intent_clicks.unwrap_or_else(|| "none".to_string()).replace(",", ";");
    let safe_ua = payload.user_agent.replace(",", ";");
    let safe_ref = payload.referrer.unwrap_or_else(|| "direct".to_string()).replace(",", ";");
    
    let received_at = Utc::now().to_rfc3339();

    let record = format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        received_at,
        payload.timestamp,
        payload.uri,
        safe_ua,
        safe_ref,
        payload.viewport.unwrap_or_else(|| "unknown".to_string()),
        payload.timezone.unwrap_or_else(|| "unknown".to_string()),
        payload.device_memory.unwrap_or_else(|| "unknown".to_string()),
        payload.hardware_cores.unwrap_or_else(|| "unknown".to_string()),
        payload.dwell_seconds.unwrap_or(0),
        payload.scroll_depth.unwrap_or(0),
        safe_clicks,
        payload.lang_browser.unwrap_or_else(|| "unknown".to_string()),
        payload.network.unwrap_or_else(|| "unknown".to_string())
    );

    if let Err(e) = file.write_all(record.as_bytes()) {
        eprintln!("LEDGER WRITE ERROR: {}", e);
        return Ok(warp::reply::with_status("500 Internal Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(warp::reply::with_status("200 OK", warp::http::StatusCode::OK))
}
