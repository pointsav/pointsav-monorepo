use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use std::time::Duration;

const TRANSIENT_QUEUE: &str = "/opt/woodfine/cluster-totebox-personnel-1/service-slm/transient-queues";
const KNOWLEDGE_GRAPH: &str = "/opt/woodfine/cluster-totebox-personnel-1/knowledge-graph";
const SLM_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 🤖 NANO-SLM PROCESSOR (STRICT ENUM GUARDRAIL)");
    println!("========================================================");

    fs::create_dir_all(KNOWLEDGE_GRAPH)?;
    let client = Client::builder().timeout(Duration::from_secs(300)).build()?;
    let mut processed = 0;

    if let Ok(entries) = fs::read_dir(TRANSIENT_QUEUE) {
        let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        files.sort();

        for path in files {
            if path.is_file() && path.extension().unwrap_or_default() == "txt" {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let overlay_name = filename.replace(".txt", "_overlay.md").replace("_raw", "");
                let out_path = format!("{}/{}", KNOWLEDGE_GRAPH, overlay_name);
                
                println!("[SYSTEM] Requesting Sentiment for: {}", filename);
                let payload_content = fs::read_to_string(&path).unwrap_or_default();
                
                let body_start = payload_content.find("[RAW CONTEXT SUMMARY]").map(|i| i + 21).unwrap_or(0);
                let actual_body = &payload_content[body_start..];
                
                let sanitized: String = actual_body
                    .chars()
                    .filter(|c| c.is_ascii() && (*c == '\n' || !c.is_control()))
                    .skip_while(|c| c.is_whitespace())
                    .take(400)
                    .collect();
                
                let user_prompt = format!("Text:\n{}\n\nSentiment:", sanitized);
                
                // Brutalist Prompting
                let payload = json!({
                    "messages": [
                        {
                            "role": "system", 
                            "content": "You are a strict classification engine. You must output exactly one word from this array: [POSITIVE, NEGATIVE, NEUTRAL]. Do not explain. Do not output any other text."
                        },
                        {
                            "role": "user", 
                            "content": user_prompt
                        }
                    ],
                    "temperature": 0.0,
                    "max_tokens": 5 
                });

                match client.post(SLM_URL).json(&payload).send().await {
                    Ok(res) => {
                        if let Ok(json_res) = res.json::<Value>().await {
                            if let Some(content) = json_res["choices"][0]["message"]["content"].as_str() {
                                // THE GUARDRAIL: Force to uppercase and check for exact substring matches
                                let clean_content = content.trim().to_uppercase();
                                
                                let final_sentiment = if clean_content.contains("POSITIVE") {
                                    "POSITIVE"
                                } else if clean_content.contains("NEGATIVE") {
                                    "NEGATIVE"
                                } else if clean_content.contains("NEUTRAL") {
                                    "NEUTRAL"
                                } else {
                                    "UNKNOWN"
                                };

                                fs::write(&out_path, format!("Sentiment: {}", final_sentiment))?;
                                println!("   -> [SUCCESS] Logged Sentiment: {}", final_sentiment);
                                fs::remove_file(&path)?;
                                processed += 1;
                            }
                        }
                    },
                    Err(e) => println!("   -> [FAULT] API Timeout/Error: {}", e)
                }
            }
        }
    }
    println!("[SUCCESS] Cognitive Phase Complete. Processed {} payloads.", processed);
    Ok(())
}
