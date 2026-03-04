use service_content::engines::{MemoEngine, SynthesisEngine, ProtocolManifest};
use service_content::parser::parse_glossary_csv;
use service_content::payload::ContextSnippet;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: service-content <protocol_yaml_path> <target_theme> <output_directory>");
        eprintln!("       service-content merge-glossary <staging_file_path> <protocol_yaml_path>");
        process::exit(1);
    }

    // =========================================================================
    // PHASE 3: THE DETERMINISTIC ROUTER (No AI / Strict File Operations)
    // =========================================================================
    if args[1] == "merge-glossary" {
        if args.len() < 4 {
            eprintln!("Fatal: Missing arguments for merge-glossary.");
            process::exit(1);
        }
        let staging_path = &args[2];
        let protocol_path = &args[3];

        println!("[DETERMINISTIC ROUTER] Initiating Sovereign Glossary Merge...");
        
        let protocol_yaml = fs::read_to_string(protocol_path).expect("Fatal: Unable to read Protocol Manifest.");
        let manifest: ProtocolManifest = serde_yaml::from_str(&protocol_yaml).expect("Fatal: Invalid Protocol YAML.");

        let staging_data = fs::read_to_string(staging_path).expect("Fatal: Unable to read Staging File.");
        
        let mut corp_appends = Vec::new();
        let mut proj_appends = Vec::new();
        let mut doc_appends = Vec::new();

        for line in staging_data.lines() {
            if line.trim().is_empty() || line.starts_with("Term,") || line.starts_with("```") { continue; }
            
            // Isolate the Target_Silo by splitting only the final comma from the right
            let parts: Vec<&str> = line.rsplitn(2, ',').collect();
            if parts.len() == 2 {
                let silo = parts[0].trim().to_lowercase();
                let payload = parts[1].trim(); // Retains the "Term,Definition,Spanish_Equivalent" structure
                
                if silo.contains("corporate") { corp_appends.push(payload); }
                else if silo.contains("project") { proj_appends.push(payload); }
                else if silo.contains("documentation") { doc_appends.push(payload); }
                else { println!("[WARNING] Unrecognized Silo: {}", line); }
            }
        }

        // Route payloads strictly to the physical capabilities defined in the YAML
        for req_path in &manifest.capability_requests {
            let req_lower = req_path.to_lowercase();
            let payload_ref = if req_lower.contains("corporate") { &corp_appends }
                              else if req_lower.contains("project") { &proj_appends }
                              else if req_lower.contains("documentation") { &doc_appends }
                              else { continue; };

            if !payload_ref.is_empty() {
                let mut file = OpenOptions::new().append(true).open(req_path).expect("Fatal: Cannot open target glossary.");
                for row in payload_ref {
                    writeln!(file, "{}", row).expect("Fatal: Write failed.");
                }
                println!("[SUCCESS] Appended {} rows to {}", payload_ref.len(), req_path);
            }
        }
        
        println!("[DETERMINISTIC ROUTER] Merge Complete. You may now delete the staging file.");
        process::exit(0);
    }

    // =========================================================================
    // PHASE 1: THE STATELESS SYNTHESIS ENGINE (RAG & Linguistic Compilation)
    // =========================================================================
    if args.len() < 4 {
        eprintln!("Usage: service-content <protocol_yaml_path> <target_theme> <output_directory>");
        process::exit(1);
    }
    let protocol_path = &args[1];
    let target_theme = &args[2];
    let output_dir = &args[3];

    println!("[SERVICE-CONTENT] Initiating Stateless Synthesis Engine...");
    let mut engine = MemoEngine::new();

    println!("[SERVICE-CONTENT] Ingesting Protocol: {}", protocol_path);
    let protocol_yaml = fs::read_to_string(protocol_path).unwrap_or_else(|_| {
        eprintln!("Fatal: Unable to read Protocol Manifest.");
        process::exit(1);
    });
    
    let manifest: ProtocolManifest = serde_yaml::from_str(&protocol_yaml).unwrap_or_else(|_| {
        eprintln!("Fatal: Invalid Protocol YAML schema.");
        process::exit(1);
    });
    
    let mut context_snippets = Vec::new();

    println!("[SERVICE-CONTENT] Mounting Capability Requests...");
    for req_path in &manifest.capability_requests {
        if let Ok(csv_data) = fs::read_to_string(req_path) {
            if let Ok(mut parsed) = parse_glossary_csv(&csv_data, req_path) {
                context_snippets.append(&mut parsed);
                println!("  -> [SUCCESS] Ingested Substrate: {}", req_path);
            }
        } else {
            println!("  -> [WARNING] Capability Denied or File Not Found: {}", req_path);
        }
    }
    engine.manifest = Some(manifest);

    let base_dir = Path::new(output_dir).parent().unwrap();
    let sub_dirs = ["research", "artifacts", "themes"];
    for dir in sub_dirs.iter() {
        let target_dir = base_dir.join(dir);
        if target_dir.exists() && target_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&target_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if ext == "txt" || ext == "md" {
                                let content = fs::read_to_string(&path).unwrap_or_default();
                                let source_id = path.file_name().unwrap().to_str().unwrap();
                                context_snippets.push(ContextSnippet {
                                    source_id: source_id.to_string(),
                                    content,
                                    tags: vec![dir.to_string()],
                                });
                                println!("[SERVICE-CONTENT] Ingested Flat File: {} [{}]", source_id, dir);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("[SERVICE-CONTENT] Executing Synthesis for Theme...");
    match engine.execute_synthesis(target_theme, context_snippets) {
        Ok(artifact) => {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let file_name = format!("{}/ARTIFACT_{}.md", output_dir, timestamp);
            
            fs::write(&file_name, artifact).unwrap_or_else(|_| {
                eprintln!("Fatal: Unable to write artifact to output directory.");
                process::exit(1);
            });
            println!("\n[SUCCESS] Artifact generated, verified, and saved to:\n{}", file_name);
        }
        Err(e) => {
            eprintln!("\n[HALT] Synthesis aborted due to compliance failure:\n{}", e);
            process::exit(1);
        }
    }
}
