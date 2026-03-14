use warp::Filter;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::env;
use std::sync::Arc;
use maxminddb::Reader;

// DS-ADR-06: The V3 Telemetry Payload Contract
#[derive(Deserialize, Debug)]
struct V3Payload {
    uri: String,
    timestamp: String,
    user_agent: String,
    referrer: String,
    viewport: String,
    dwell_time: u32,
    scroll_depth: u32,
    intent_clicks: String,
    intent_copied: bool,
    lang_browser: String,
    lang_toggled: String,
    network: String,
}

#[tokio::main]
async fn main() {
    println!("📡 [SYSTEM] Initiating PointSav V3 Sovereign Telemetry Daemon...");

    // Dynamic Database Pathing: Check Production Vault first, fallback to Foundry Vault
    let prod_path = "./assets/GeoLite2-City.mmdb";
    let dev_path = "../vendors-maxmind/GeoLite2-City.mmdb";
    
    let db_path = if std::path::Path::new(prod_path).exists() {
        prod_path
    } else {
        dev_path
    };

    let reader = match Reader::open_readfile(db_path) {
        Ok(r) => {
            println!("✅ [SUCCESS] Offline Metro Routing DB loaded from: {}", db_path);
            Arc::new(r)
        },
        Err(_) => {
            println!("⚠️ [WARNING] GeoLite2-City.mmdb not found. Metro routing will default to 'Unknown'.");
            Arc::new(Reader::from_source(vec![]).unwrap())
        }
    };

    // Ensure the ledger file exists with headers
    let ledger_path = "./assets/ledger_telemetry.csv";
    if let Some(parent) = std::path::Path::new(ledger_path).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    
    if !std::path::Path::new(ledger_path).exists() {
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(ledger_path) {
            let _ = writeln!(file, "IP_Masked,Timestamp,URI,Referrer,Viewport,Dwell_Sec,Scroll_Pct,Intent_Clicks,Intent_Copied,Lang_Browser,Lang_Toggled,Network,Metro_Region");
        }
    }

    let db_filter = warp::any().map(move || Arc::clone(&reader));

    // The Ingestion Route
    let telemetry_route = warp::post()
        .and(warp::path("telemetry-endpoint"))
        .and(warp::addr::remote())
        .and(warp::body::json())
        .and(db_filter)
        .map(|addr: Option<SocketAddr>, payload: V3Payload, db: Arc<Reader<Vec<u8>>>| {
            
            // 1. IP Masking (Truncate final octet)
            let raw_ip = addr.map(|a| a.ip()).unwrap_or_else(|| "0.0.0.0".parse().unwrap());
            let masked_ip = match raw_ip {
                IpAddr::V4(ipv4) => {
                    let octets = ipv4.octets();
                    format!("{}.{}.{}.0", octets[0], octets[1], octets[2])
                }
                IpAddr::V6(ipv6) => {
                    let segments = ipv6.segments();
                    format!("{:x}:{:x}:{:x}:{:x}::", segments[0], segments[1], segments[2], segments[3])
                }
            };

            // 2. Offline Metro Routing
            let mut metro_region = String::from("Unknown_Region");
            if let Ok(city_data) = db.lookup::<maxminddb::geoip2::City>(raw_ip) {
                if let Some(city) = city_data.city {
                    if let Some(names) = city.names {
                        if let Some(name) = names.get("en") {
                            metro_region = name.to_string();
                        }
                    }
                }
            }

            // 3. Write to Immutable CSV Ledger
            if let Ok(mut file) = OpenOptions::new().append(true).open("./assets/ledger_telemetry.csv") {
                let log_entry = format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    masked_ip,
                    payload.timestamp,
                    payload.uri,
                    payload.referrer,
                    payload.viewport,
                    payload.dwell_time,
                    payload.scroll_depth,
                    payload.intent_clicks.replace(",", ";"), 
                    payload.intent_copied,
                    payload.lang_browser,
                    payload.lang_toggled,
                    payload.network,
                    metro_region
                );
                let _ = file.write_all(log_entry.as_bytes());
            }

            warp::reply::with_status("200 OK", warp::http::StatusCode::OK)
        })
        .with(warp::cors().allow_any_origin().allow_methods(vec!["POST"]).allow_headers(vec!["Content-Type"]));

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8081".to_string()).parse().unwrap_or(8081);
    
    println!("🔒 [SECURE] V3 Diode Active. Listening on 127.0.0.1:{}", port);
    warp::serve(telemetry_route).run(([127, 0, 0, 1], port)).await;
}
