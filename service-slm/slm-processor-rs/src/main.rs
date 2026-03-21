use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use uuid::Uuid;

const TRANSIENT_QUEUE: &str = "/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-slm/transient-queues";
const CONTENT_VAULT: &str = "/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-content/verified-ledger";
const LOCAL_SLM_URL: &str = "http://127.0.0.1:11434/api/generate";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 🤖 NANO-SLM PROCESSOR (135M PARAMETER ROUTER)");
    println!("========================================================");

    fs::create_dir_all(CONTENT_VAULT)?;
    let client = Client::new();
    
    let entries = fs::read_dir(TRANSIENT_QUEUE)?;
    let mut processed_count = 0;

    for entry in entries {
        let path = entry?.path();
        if path.is_file() && path.extension().unwrap_or_default() == "txt" {
            let filename = path.file_name().unwrap().to_string_lossy();
            println!("\n[SYSTEM] Initiating Sovereign Extraction on: {}", filename);
            
            let payload_text = fs::read_to_string(&path)?;
            
            let prompt = format!(
                "Analyze the email. Extract sender details into JSON: \"name\", \"title\", \"organization\", \"email\". If missing, use \"UNKNOWN\". Return ONLY raw JSON.\n\nTEXT:\n{}", 
                payload_text
            );

            // Targeted precisely at the 135M model with 1024 token memory clamp
            let request_body = json!({
                "model": "smollm2:135m", 
                "prompt": prompt,
                "stream": false,
                "format": "json",
                "options": {
                    "num_ctx": 1024,
                    "num_predict": 150
                }
            });

            println!("  -> Transmitting payload to local nano-SLM...");
            let res = match client.post(LOCAL_SLM_URL).json(&request_body).send().await {
                Ok(response) => response,
                Err(e) => {
                    println!("  -> [FATAL] Connection refused. SLM is offline: {}", e);
                    continue;
                }
            };

            let json_res: Value = res.json().await?;
            
            if let Some(content) = json_res["response"].as_str() {
                let clean_json = content.replace("```json", "").replace("```", "").trim().to_string();
                let ledger_id = Uuid::new_v4().to_string()[..8].to_string();
                let out_path = format!("{}/LEDGER-{}.json", CONTENT_VAULT, ledger_id);
                
                fs::write(&out_path, clean_json)?;
                println!("  -> [SUCCESS] Institutional Ledger updated: LEDGER-{}.json", ledger_id);
                
                fs::remove_file(path)?;
                processed_count += 1;
                
                if processed_count >= 3 { break; }
            }
        }
    }

    println!("========================================================");
    println!("[SUCCESS] Cognitive Phase Complete. Processed {} payloads.", processed_count);
    Ok(())
}
