use mailparse::*;
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("[ERROR] Invalid execution. Usage: mime-splinter <EML_FILE_PATH> <TOTEBOX_ROOT>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let totebox_root = &args[2];

    println!("[SYSTEM] Initiating Forensic Splinter on: {}", file_path);

    let raw_email = fs::read(file_path).expect("[ERROR] Failed to read immutable .eml file.");
    let parsed_mail = parse_mail(&raw_email).expect("[ERROR] Failed to parse MIME boundaries.");

    let route_slm = format!("{}/service-slm/transient-queues", totebox_root);
    let route_people = format!("{}/service-people/discovery-queue", totebox_root);
    let route_assets = format!("{}/assets/inert-media", totebox_root);

    fs::create_dir_all(&route_slm).unwrap();
    fs::create_dir_all(&route_people).unwrap();
    fs::create_dir_all(&route_assets).unwrap();

    let transaction_id = Uuid::new_v4().to_string()[..8].to_string();
    traverse_and_route(&parsed_mail, &transaction_id, &route_slm, &route_people, &route_assets);

    println!("[SUCCESS] Transaction {} complete.", transaction_id);
}

fn traverse_and_route(part: &ParsedMail, tx_id: &str, route_slm: &str, route_people: &str, route_assets: &str) {
    let ctype = part.ctype.mimetype.clone();
    let disposition = part.get_content_disposition().disposition;
    
    if disposition != DispositionType::Attachment && (ctype == "text/plain" || ctype == "text/html") {
        let body_text = part.get_body().unwrap_or_default();
        if !body_text.trim().is_empty() {
            let out_path = format!("{}/TX-{}_body.txt", route_slm, tx_id);
            fs::write(&out_path, body_text).expect("Failed to write to SLM queue");
            println!("  -> [ROUTED] Core Body Text --> service-slm");
        }
    }

    if disposition == DispositionType::Attachment {
        let filename = part.get_content_disposition().params.get("filename").cloned().unwrap_or_else(|| "unnamed_artifact.dat".to_string());
        let binary_payload = part.get_body_raw().unwrap_or_default();
        let ext = Path::new(&filename).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

        match ext.as_str() {
            "csv" | "xlsx" | "xls" => {
                let out_path = format!("{}/TX-{}_{}", route_people, tx_id, filename);
                fs::write(&out_path, binary_payload).expect("Failed to write to People queue");
                println!("  -> [ROUTED] {} --> service-people", filename);
            },
            "pdf" | "docx" | "txt" => {
                let out_path = format!("{}/TX-{}_{}", route_slm, tx_id, filename);
                fs::write(&out_path, binary_payload).expect("Failed to write to SLM queue");
                println!("  -> [ROUTED] {} --> service-slm", filename);
            },
            "jpg" | "jpeg" | "png" | "svg" | "gif" => {
                let out_path = format!("{}/TX-{}_{}", route_assets, tx_id, filename);
                fs::write(&out_path, binary_payload).expect("Failed to write to Asset vault");
                println!("  -> [ROUTED] {} --> inert-media", filename);
            },
            _ => println!("  -> [IGNORED] {} (Unsupported extension)", filename)
        }
    }

    for subpart in &part.subparts {
        traverse_and_route(subpart, tx_id, route_slm, route_people, route_assets);
    }
}
