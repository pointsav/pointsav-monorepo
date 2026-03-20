use aho_corasick::AhoCorasick;
use serde_json::Value;
use std::env;
use std::fs;

/// Recursively hunts through any JSON structure for the "gravity_keywords" array.
fn extract_keywords(v: &Value, keywords: &mut Vec<String>) {
    match v {
        Value::Object(map) => {
            for (key, val) in map {
                if key == "gravity_keywords" {
                    if let Value::Array(arr) = val {
                        for item in arr {
                            if let Value::String(s) = item {
                                keywords.push(s.to_string());
                            }
                        }
                    }
                } else {
                    extract_keywords(val, keywords);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                extract_keywords(item, keywords);
            }
        }
        _ => {}
    }
}

/// Loads all JSON files in the Seed Vault and extracts the intelligence baseline.
fn load_gravity_seeds(seed_dir: &str) -> Vec<String> {
    let mut keywords = Vec::new();
    if let Ok(entries) = fs::read_dir(seed_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(json_val) = serde_json::from_str::<Value>(&content) {
                        extract_keywords(&json_val, &mut keywords);
                    }
                }
            }
        }
    }
    keywords.sort();
    keywords.dedup();
    keywords
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("[ERROR] Usage: service-content <SEED_DIRECTORY> <WORM_PAYLOAD>");
        std::process::exit(1);
    }

    let seed_dir = &args[1];
    let file_path = &args[2];

    // 1. Ingest the Customer's JSON Seed Vault
    let gravity_keywords = load_gravity_seeds(seed_dir);
    if gravity_keywords.is_empty() {
        eprintln!("[ERROR] Seed Vault is empty or unreadable.");
        std::process::exit(1);
    }

    // 2. Read the Raw Payload
    let raw_text = fs::read_to_string(file_path)
        .expect("[ERROR] Could not read WORM payload.");

    let clean_text = raw_text.replace("<br>", "\n").replace("<p>", "\n");
    let lines: Vec<&str> = clean_text.split('\n').collect();

    // 3. Build the High-Speed Automaton
    let ac = AhoCorasick::new(&gravity_keywords).expect("Failed to build Automaton");

    let mut gravity_mass = 0;
    let mut condensed_vector: Vec<String> = Vec::new();

    // 4. Scan for Identity Mass
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        let mut matched = false;
        for mat in ac.find_iter(trimmed) {
            let keyword = &gravity_keywords[mat.pattern()];
            gravity_mass += 10; 
            
            if !matched {
                condensed_vector.push(format!("[HIT: {}] {}", keyword, trimmed));
                matched = true;
            }
        }
    }

    // 5. Output the 99% Digested Payload for the SLM Gatekeeper
    if gravity_mass > 0 {
        println!("--- GRAVITY VECTOR ESTABLISHED ---");
        println!("TOTAL MASS: {}", gravity_mass);
        println!("CONDENSED PAYLOAD FOR SLM:\n");
        for vector in condensed_vector.iter().take(5) {
            println!("{}", vector);
        }
    } else {
        println!("[SYSTEM] Zero Gravity Detected. Payload Ignored.");
    }
}
