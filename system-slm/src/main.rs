use std::env;
use reqwest::Client;
use serde_json::{json, Value};
use std::process;

/// PointSav Digital Systems: Semantic Router
/// Standard: TOPIC-SYSTEM-SLM (Human-in-the-Loop Translation)

const SLM_SERVER_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(r#"{{"error": "Missing input. Usage: system-slm '<intent>'"}}"#);
        process::exit(1);
    }

    let user_intent = &args[1];

    // The strict systemic prompt forcing deterministic JSON output
    let system_prompt = "You are a network command translator. \
    Translate the user's natural language request into a strict JSON payload. \
    You must output ONLY valid JSON. No markdown formatting, no explanations. \
    The JSON must contain exactly two keys: 'intent' (a single uppercase ACTION verb like ISOLATE, PING, REBOOT, QUERY) \
    and 'target' (the specific node identifier like NODE-LAPTOP-A, or ALL).";

    let payload = json!({
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_intent}
        ],
        "temperature": 0.1, // Near-zero temperature ensures deterministic outputs
        "max_tokens": 150
    });

    let client = Client::new();
    match client.post(SLM_SERVER_URL).json(&payload).send().await {
        Ok(res) => {
            if let Ok(json_res) = res.json::<Value>().await {
                if let Some(content) = json_res["choices"][0]["message"]["content"].as_str() {
                    // Strip any stray markdown code blocks the SLM might inject
                    let clean_content = content.replace("```json", "").replace("```", "").trim().to_string();
                    
                    // Verify it is actually valid JSON before outputting
                    if serde_json::from_str::<Value>(&clean_content).is_ok() {
                        println!("{}", clean_content);
                    } else {
                        eprintln!(r#"{{"error": "SLM generated invalid JSON structure.", "raw": "{}"}}"#, clean_content);
                        process::exit(1);
                    }
                } else {
                    eprintln!(r#"{{"error": "Malformed response from SLM server."}}"#);
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            // Fallback for when the local SLM server is offline (useful for development)
            eprintln!(r#"{{"error": "SLM Server unreachable at {}. Ensure Qwen2-0.5B is ignited."}}"#, SLM_SERVER_URL);
            process::exit(1);
        }
    }
}
