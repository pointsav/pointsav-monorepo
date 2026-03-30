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
    println!(" 🤖 SEMANTIC ALU: SYNTHETIC MEMORY MODE (V4.8)");
    println!("========================================================");

    fs::create_dir_all(KNOWLEDGE_GRAPH)?;
    
    // THE FIX: Expanding the temporal runway to 180 seconds for the e2-micro vCPU
    let client = Client::builder().timeout(Duration::from_secs(180)).build()?;
    let mut processed = 0;

    if let Ok(entries) = fs::read_dir(TRANSIENT_QUEUE) {
        let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        files.sort();

        for path in files {
            if path.is_file() && path.extension().unwrap_or_default() == "txt" {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let overlay_name = filename.replace("_skeleton.txt", ".md");
                let out_path = format!("{}/{}", KNOWLEDGE_GRAPH, overlay_name);
                
                println!("[SYSTEM] Routing Skeleton to Semantic ALU: {}", filename);
                let payload_content = fs::read_to_string(&path).unwrap_or_default();
                
                let cutoff_index = payload_content.find("status: \"PENDING_COGNITIVE_STATE\"\n---").unwrap_or(0);
                let frontmatter_top = &payload_content[0..cutoff_index];
                
                let body_start = payload_content.find("---\n\n").map(|i| i + 5).unwrap_or(0);
                let raw_text = &payload_content[body_start..];
                
                let sanitized_text: String = raw_text.chars().take(400).collect();

                let payload = json!({
                    "messages": [
                        { 
                            "role": "system", 
                            "content": "You are a rigid classification engine. Reply ONLY with three words. Choose exactly one from each list.\nArchetypes: EXECUTIVE, GUARDIAN, FIDUCIARY, ENVOY, CONSTRUCTOR\nDomains: CORPORATE, PROJECTS, DOCUMENTATION, EXTERNAL\nSentiments: POSITIVE, NEGATIVE, NEUTRAL" 
                        },
                        { 
                            "role": "user", 
                            "content": "Text: We need to finalize the real estate permits immediately. The city is waiting." 
                        },
                        { 
                            "role": "assistant", 
                            "content": "CONSTRUCTOR PROJECTS NEUTRAL" 
                        },
                        { 
                            "role": "user", 
                            "content": format!("Text: {}", sanitized_text)
                        }
                    ],
                    "temperature": 0.0,
                    "max_tokens": 10
                });

                match client.post(SLM_URL).json(&payload).send().await {
                    Ok(res) => {
                        if let Ok(json_res) = res.json::<Value>().await {
                            if let Some(content) = json_res["choices"][0]["message"]["content"].as_str() {
                                
                                let clean_output = content.trim().to_uppercase();
                                println!("   -> [RAW AI OUTPUT] {}", clean_output.replace('\n', " | "));
                                
                                let final_archetype = if clean_output.contains("EXECUTIVE") { "EXECUTIVE" }
                                else if clean_output.contains("GUARDIAN") { "GUARDIAN" }
                                else if clean_output.contains("FIDUCIARY") { "FIDUCIARY" }
                                else if clean_output.contains("ENVOY") { "ENVOY" }
                                else if clean_output.contains("CONSTRUCTOR") { "CONSTRUCTOR" }
                                else { "UNKNOWN" };

                                let final_domain = if clean_output.contains("CORPORATE") { "CORPORATE" }
                                else if clean_output.contains("PROJECTS") { "PROJECTS" }
                                else if clean_output.contains("DOCUMENTATION") { "DOCUMENTATION" }
                                else if clean_output.contains("EXTERNAL") { "EXTERNAL" }
                                else { "UNKNOWN" };

                                let final_sentiment = if clean_output.contains("POSITIVE") { "POSITIVE" }
                                else if clean_output.contains("NEGATIVE") { "NEGATIVE" }
                                else if clean_output.contains("NEUTRAL") { "NEUTRAL" }
                                else { "UNKNOWN" };

                                let token_assembly = format!(
                                    "{}status: \"VERIFIED\"\n\
                                    Archetype: \"{}\"\n\
                                    Domain: \"{}\"\n\
                                    Sentiment: \"{}\"\n\
                                    ---\n\n\
                                    {}",
                                    frontmatter_top,
                                    final_archetype, final_domain, final_sentiment,
                                    raw_text.trim()
                                );

                                fs::write(&out_path, token_assembly)?;
                                println!("   -> [SUCCESS] 3D Token Forged: {} [{} | {} | {}]", overlay_name, final_archetype, final_domain, final_sentiment);
                                fs::remove_file(&path)?;
                                processed += 1;
                            }
                        }
                    },
                    Err(e) => println!("   -> [FAULT] API Timeout/Refusal: {}", e)
                }
            }
        }
    }
    
    println!("[SUCCESS] Cognitive Phase Complete. Processed {} payloads.", processed);
    Ok(())
}
