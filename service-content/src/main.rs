use service_content::engines::{MemoEngine, SynthesisEngine, ProtocolManifest};
use service_content::parser::parse_glossary_csv;
use std::env;
use std::fs;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("[SERVICE-CONTENT] Initiating Stateless Synthesis Engine...");

    // 1. Argument Parsing
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: service-content <protocol_yaml_path> <glossary_csv_path> <target_theme> <output_directory>");
        process::exit(1);
    }

    let protocol_path = &args[1];
    let glossary_path = &args[2];
    let target_theme = &args[3];
    let output_dir = &args[4];

    // 2. Initialize the Engine
    let mut engine = MemoEngine::new();

    // 3. Load the Fleet Protocol
    println!("[SERVICE-CONTENT] Ingesting Protocol: {}", protocol_path);
    let protocol_yaml = fs::read_to_string(protocol_path).unwrap_or_else(|_| {
        eprintln!("Fatal: Unable to read Protocol Manifest.");
        process::exit(1);
    });
    
    let manifest: ProtocolManifest = serde_yaml::from_str(&protocol_yaml).unwrap_or_else(|_| {
        eprintln!("Fatal: Invalid Protocol YAML schema.");
        process::exit(1);
    });
    engine.manifest = Some(manifest);

    // 4. Ingest the Data Substrate
    println!("[SERVICE-CONTENT] Ingesting Glossary Substrate: {}", glossary_path);
    let csv_data = fs::read_to_string(glossary_path).unwrap_or_else(|_| {
        eprintln!("Fatal: Unable to read CSV Substrate.");
        process::exit(1);
    });

    let context_snippets = parse_glossary_csv(&csv_data, "content-wiki-corporate").unwrap_or_else(|_| {
        eprintln!("Fatal: CSV Parsing Failure.");
        process::exit(1);
    });

    // 5. Execute the RAG Pipeline and Route Output
    println!("[SERVICE-CONTENT] Executing Synthesis for Theme: {}", target_theme);
    match engine.execute_synthesis(target_theme, context_snippets) {
        Ok(artifact) => {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let sanitized_theme = target_theme.replace(" ", "_").replace("-", "_");
            let file_name = format!("{}/{}_{}.md", output_dir, sanitized_theme, timestamp);
            
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
