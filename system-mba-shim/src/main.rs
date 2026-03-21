use std::fs;
use std::io::Read;
use tiny_http::{Server, Response, Method, Header};

const QUEUE_DIR: &str = "/opt/woodfine/cluster-totebox-personnel/service-people/discovery-queue";
const LEDGER_DIR: &str = "/opt/woodfine/cluster-totebox-personnel/service-people/extracted-ledgers";

fn main() {
    let server = Server::http("0.0.0.0:8080").expect("[ERROR] Failed to bind to port 8080");
    println!("[SYSTEM] Sovereign API Bridge (Rust Native) active on port 8080...");

    // Ensure the operational boundaries exist
    fs::create_dir_all(QUEUE_DIR).unwrap();
    fs::create_dir_all(LEDGER_DIR).unwrap();

    for mut request in server.incoming_requests() {
        match (request.method(), request.url()) {
            (&Method::Get, "/next") => {
                let mut found_data = None;
                let mut target_file = None;

                // Scan the discovery queue
                if let Ok(entries) = fs::read_dir(QUEUE_DIR) {
                    for entry in entries.flatten() {
                        if entry.path().extension().map_or(false, |ext| ext == "json") {
                            if let Ok(data) = fs::read_to_string(entry.path()) {
                                found_data = Some(data);
                                target_file = Some(entry.path());
                                break; // Lock onto the first valid identity
                            }
                        }
                    }
                }

                // Transmit and physically pop from the queue
                if let (Some(data), Some(path)) = (found_data, target_file) {
                    fs::remove_file(path).unwrap(); // Destroy local queue artifact after reading
                    let mut response = Response::from_string(data);
                    response.add_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    request.respond(response).unwrap();
                } else {
                    request.respond(Response::empty(404)).unwrap();
                }
            },
            (&Method::Post, "/verify") => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).unwrap();

                // Vault the cryptographically verified payload
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(sov_id) = json.get("sovereign_id").and_then(|v| v.as_str()) {
                        let out_path = format!("{}/{}.json", LEDGER_DIR, sov_id);
                        fs::write(out_path, content).unwrap();
                        request.respond(Response::empty(200)).unwrap();
                        continue;
                    }
                }
                request.respond(Response::empty(400)).unwrap();
            },
            _ => {
                request.respond(Response::empty(404)).unwrap();
            }
        }
    }
}
