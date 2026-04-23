use std::env;
use std::fs;
use reqwest::Client;
use serde_json::{json, Value};

const SLM_SERVER_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("[ERROR] Usage: cognitive-forge <TOTEBOX_ROOT>");
        std::process::exit(1);
    }
    let totebox_root = &args[1];
    let queue_dir = format!("{}/service-slm/transient-queues", totebox_root);
    let output_dir = format!("{}/service-content/knowledge-graph", totebox_root);

    let client = Client::new();

    if let Ok(entries) = fs::read_dir(&queue_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "txt" {
                if let Ok(content) = fs::read_to_string(&path) {
                    println!("[FORGE] Processing: {:?}", path.file_name().unwrap());
                    
                    let payload = json!({
                        "messages": [
                            {"role": "system", "content": "Extract the core objective facts from this text. Output clean markdown bullet points."},
                            {"role": "user", "content": content}
                        ],
                        "temperature": 0.2
                    });

                    if let Ok(res) = client.post(SLM_SERVER_URL).json(&payload).send().await {
                        if let Ok(json_res) = res.json::<Value>().await {
                            if let Some(extracted) = json_res["choices"][0]["message"]["content"].as_str() {
                                let out_path = format!("{}/{}.md", output_dir, path.file_stem().unwrap().to_string_lossy());
                                fs::write(&out_path, extracted).unwrap();
                                fs::remove_file(&path).unwrap(); // Destroy raw asset after extraction
                                println!("  -> [SUCCESS] Vaulted to knowledge-graph.");
                            }
                        }
                    }
                }
            }
        }
    }
}
