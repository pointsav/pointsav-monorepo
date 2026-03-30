use mailparse::*;
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { std::process::exit(1); }

    let file_path = &args[1];
    let totebox_root = &args[2];
    let filename = Path::new(file_path).file_name().unwrap_or_default().to_string_lossy().to_string();

    let raw_email = fs::read(file_path).unwrap();
    let parsed_mail = parse_mail(&raw_email).unwrap();

    let route_spatial = format!("{}/service-people/spatial-queue", totebox_root);
    fs::create_dir_all(&route_spatial).unwrap();

    let transaction_id = Uuid::new_v4().to_string()[..8].to_string().to_uppercase();
    traverse_and_route(&parsed_mail, &transaction_id, &filename, &route_spatial);
}

fn traverse_and_route(part: &ParsedMail, tx_id: &str, filename: &str, route_spatial: &str) {
    let ctype = part.ctype.mimetype.clone();
    let disposition = part.get_content_disposition().disposition;
    
    if disposition != DispositionType::Attachment && (ctype == "text/plain" || ctype == "text/html") {
        let raw_body = part.get_body().unwrap_or_default();
        if !raw_body.trim().is_empty() {
            let clean_text = if ctype == "text/html" {
                html2text::from_read(raw_body.as_bytes(), 100)
            } else { raw_body };

            // PURE TEXT DUMP. NO YAML. NO CHUNKING.
            let out_path = format!("{}/TX-{}_{}_raw.txt", route_spatial, tx_id, filename);
            fs::write(&out_path, clean_text).expect("Failed to write to spatial-queue");
        }
    }

    for subpart in &part.subparts { traverse_and_route(subpart, tx_id, filename, route_spatial); }
}
