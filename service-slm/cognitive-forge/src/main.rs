use std::env;
use std::fs;
use std::process::Command;

fn main() {
    println!("========================================================");
    println!(" 🧠 COGNITIVE FORGE: SLM DISTILLATION EXTRACTION");
    println!("========================================================");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[ERROR] Usage: cognitive-forge <TOTEBOX_ROOT>");
        std::process::exit(1);
    }

    let totebox_root = &args[1];
    let queue_dir = format!("{}/service-slm/transient-queues", totebox_root);
    let graph_dir = format!("{}/service-content/knowledge-graph", totebox_root);

    let _ = fs::create_dir_all(&queue_dir);
    let _ = fs::create_dir_all(&graph_dir);

    let paths = fs::read_dir(&queue_dir).expect("[ERROR] Cannot read transient queue.");
    let mut files_processed = 0;

    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.extension().and_then(|s| s.to_str()) != Some("txt") {
            continue;
        }

        let filename = file_path.file_name().unwrap().to_str().unwrap();
        println!("  -> Distilling {}...", filename);

        let mut content = fs::read_to_string(&file_path).unwrap_or_default();
        
        // Mathematical Truncation: Protect the 1GB RAM constraint
        if content.len() > 1500 {
            content.truncate(1500);
            println!("     [SYSTEM] Payload distilled to 1500 bytes.");
        }

        let system_prompt = "You are a strict data extraction engine. Read the text and extract: 1. Primary Entity Name. 2. Entity Type (Person or Company). 3. A 1-sentence summary. Output ONLY valid JSON.";
        
        let payload = serde_json::json!({
            "model": "qwen2-0.5b-instruct",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": content}
            ],
            "temperature": 0.1,
            "max_tokens": 200
        });

        println!("     [SYSTEM] Igniting Qwen2-0.5B Inference via OS Diode...");

        // Execute the physical API call using native curl
        let payload_str = payload.to_string();
        let output = Command::new("curl")
            .args(&[
                "-s",
                "-X", "POST",
                "http://127.0.0.1:8080/v1/chat/completions",
                "-H", "Content-Type: application/json",
                "-d", &payload_str
            ])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let res_str = String::from_utf8_lossy(&out.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&res_str) {
                    if let Some(text) = json["choices"][0]["message"]["content"].as_str() {
                        let clean_json = text.replace("```json", "").replace("```", "").trim().to_string();
                        
                        let out_path = format!("{}/EXTRACTED_{}.json", graph_dir, filename);
                        let _ = fs::write(&out_path, clean_json);
                        println!("     [SUCCESS] Forged Identity:");
                        println!("       {}", text.replace('\n', " "));
                        
                        let _ = fs::remove_file(&file_path);
                        files_processed += 1;
                        continue;
                    }
                }
                println!("     [ERROR] Sovereign AI returned an unexpected format.");
            }
            _ => println!("     [ERROR] SLM Inference failed. Is the engine running?"),
        }
    }
    
    if files_processed == 0 {
        println!("  -> [SYSTEM] The queue is mathematically empty.");
    }
    
    println!("--------------------------------------------------------");
    println!("[SUCCESS] Cognitive Forge extraction cycle complete.");
}
