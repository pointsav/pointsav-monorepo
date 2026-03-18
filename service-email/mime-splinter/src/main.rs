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

    let raw_email = fs::read(file_path).expect("[ERROR] Failed to read immutable .eml file.");
    let parsed_mail = parse_mail(&raw_email).expect("[ERROR] Failed to parse MIME boundaries.");

    // THE PATCH: Extract and preserve critical identity metadata
    let mut metadata = String::new();
    for header in &parsed_mail.headers {
        let k = header.get_key().to_lowercase();
        if k == "from" || k == "to" || k == "cc" || k == "date" || k == "subject" {
            metadata.push_str(&format!("{}: {}\n", header.get_key(), header.get_value()));
        }
    }
    metadata.push_str("---\n\n");

    let route_slm = format!("{}/service-slm/transient-queues", totebox_root);
    let route_people = format!("{}/service-people/discovery-queue", totebox_root);
    let route_assets = format!("{}/assets/inert-media", totebox_root);

    fs::create_dir_all(&route_slm).unwrap();
    fs::create_dir_all(&route_people).unwrap();
    fs::create_dir_all(&route_assets).unwrap();

    let tx_id = Uuid::new_v4().to_string()[..8].to_string();

    // ALWAYS write the metadata block so the ACS Engine can extract the identity
    let metadata_path = format!("{}/TX-{}_metadata.txt", route_slm, tx_id);
    fs::write(&metadata_path, &metadata).expect("Failed to write metadata anchor");

    traverse_and_route(&parsed_mail, &tx_id, &route_slm, &route_people, &route_assets);
}

fn traverse_and_route(
    part: &ParsedMail, 
    tx_id: &str, 
    route_slm: &str, 
    route_people: &str, 
    route_assets: &str
) {
    let ctype = part.ctype.mimetype.clone();
    let disposition = part.get_content_disposition().disposition;
    
    if disposition != DispositionType::Attachment && ctype == "text/plain" {
        let body_text = part.get_body().unwrap_or_default();
        if !body_text.trim().is_empty() {
            let out_path = format!("{}/TX-{}_body.txt", route_slm, tx_id);
            fs::write(&out_path, body_text).ok();
        }
    }

    if disposition != DispositionType::Attachment && ctype == "text/html" {
         let body_text = part.get_body().unwrap_or_default();
         if !body_text.trim().is_empty() {
            let out_path = format!("{}/TX-{}_body.html", route_slm, tx_id);
            fs::write(&out_path, body_text).ok();
         }
    }

    if disposition == DispositionType::Attachment {
        let filename = part.get_content_disposition().title.unwrap_or_else(|| "unnamed_artifact.dat".to_string());
        let binary_payload = part.get_body_raw().unwrap_or_default();
        let ext = Path::new(&filename).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

        match ext.as_str() {
            "csv" | "xlsx" | "xls" => {
                fs::write(format!("{}/TX-{}_{}", route_people, tx_id, filename), binary_payload).ok();
            },
            "pdf" | "docx" | "txt" => {
                fs::write(format!("{}/TX-{}_{}", route_slm, tx_id, filename), binary_payload).ok();
            },
            _ => {}
        }
    }

    for subpart in &part.subparts {
        traverse_and_route(subpart, tx_id, route_slm, route_people, route_assets);
    }
}
