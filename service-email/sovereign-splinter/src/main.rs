use mailparse::*;
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use serde_json::json;
use chrono::Utc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("[ERROR] Invalid execution. Usage: sovereign-splinter <EML_FILE_PATH> <TOTEBOX_ROOT>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let totebox_root = &args[2];
    let filename = Path::new(file_path).file_name().unwrap_or_default().to_string_lossy().to_string();

    println!("[SYSTEM] Initiating Sovereign Splinter (v2.1) on: {}", filename);

    let raw_email = fs::read(file_path).expect("[ERROR] Failed to read immutable .eml file.");
    let parsed_mail = parse_mail(&raw_email).expect("[ERROR] Failed to parse MIME boundaries.");

    let route_slm = format!("{}/service-slm/transient-queues", totebox_root);
    let route_people = format!("{}/service-people/discovery-queue", totebox_root);
    let route_assets = format!("{}/assets/inert-media", totebox_root);

    fs::create_dir_all(&route_slm).unwrap();
    fs::create_dir_all(&route_people).unwrap();
    fs::create_dir_all(&route_assets).unwrap();

    let transaction_id = Uuid::new_v4().to_string()[..8].to_string().to_uppercase();

    let mut sender = String::new();
    let mut subject = String::new();
    
    for header in parsed_mail.headers.iter() {
        let key = header.get_key().to_lowercase();
        if key == "from" { sender = header.get_value(); }
        if key == "subject" { subject = header.get_value(); }
    }

    let identity_payload = json!({
        "transaction_id": transaction_id,
        "source_artifact": filename,
        "timestamp": Utc::now().to_rfc3339(),
        "extracted_signals": {
            "sender": sender,
            "subject": subject
        },
        "status": "AWAITING_HITL_CHECKOUT"
    });

    let identity_path = format!("{}/TX-{}_identity.json", route_people, transaction_id);
    fs::write(&identity_path, serde_json::to_string_pretty(&identity_payload).unwrap()).expect("Failed to route Identity Ledger");

    traverse_and_route(&parsed_mail, &transaction_id, &filename, &route_slm, &route_assets);

    println!("[SUCCESS] Transaction {} complete.", transaction_id);
}

fn traverse_and_route(part: &ParsedMail, tx_id: &str, filename: &str, route_slm: &str, route_assets: &str) {
    let ctype = part.ctype.mimetype.clone();
    let disposition = part.get_content_disposition().disposition;
    
    if disposition != DispositionType::Attachment && (ctype == "text/plain" || ctype == "text/html") {
        let raw_body = part.get_body().unwrap_or_default();
        if !raw_body.trim().is_empty() {
            
            // THE HTML PURIFIER
            let clean_text = if ctype == "text/html" {
                println!("  -> [PURIFIER] Stripping HTML/CSS noise...");
                html2text::from_read(raw_body.as_bytes(), 100)
            } else {
                raw_body
            };

            // THE DRIP FEED (Chunking into 1500 character limits)
            let max_chars = 1500;
            let chars: Vec<char> = clean_text.chars().collect();
            let chunks: Vec<&[char]> = chars.chunks(max_chars).collect();

            for (i, chunk_chars) in chunks.iter().enumerate() {
                let chunk_str: String = chunk_chars.iter().collect();
                
                let framed_payload = format!(
                    "---\n\
                    system_directive: \"Extract objective facts from this payload. Map them against the Domain Glossaries to generate an Overlay Proposal.\"\n\
                    ontological_pillars: [\"Archetypes\", \"Chart of Accounts\", \"Themes\", \"Domains\"]\n\
                    source_artifact: \"{}_PART_{}\"\n\
                    ---\n\n\
                    {}", filename, i + 1, chunk_str
                );

                let out_path = format!("{}/TX-{}_PT{}_payload.txt", route_slm, tx_id, i + 1);
                fs::write(&out_path, framed_payload).expect("Failed to route to SLM Staging");
            }
            println!("  -> [ROUTED] Core Body Text ({} chunks) --> service-slm", chunks.len());
        }
    }

    if disposition == DispositionType::Attachment {
        let att_filename = part.get_content_disposition().params.get("filename").cloned().unwrap_or_else(|| format!("unnamed_{}.dat", tx_id));
        let binary_payload = part.get_body_raw().unwrap_or_default();
        let out_path = format!("{}/TX-{}_{}", route_assets, tx_id, att_filename);
        fs::write(&out_path, binary_payload).expect("Failed to write to Asset vault");
        println!("  -> [ROUTED] {} --> inert-media (Assets)", att_filename);
    }

    for subpart in &part.subparts {
        traverse_and_route(subpart, tx_id, filename, route_slm, route_assets);
    }
}
