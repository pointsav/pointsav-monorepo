mod maildir;
mod graph_client;
mod auth;

use maildir::MaildirVault;
use graph_client::GraphBridge;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SYSTEM EVENT: Initializing Sovereign Email Bridge daemon (Production Recursive Mode).");

    let archive_path = env::var("TOTEBOX_ARCHIVE_PATH")
        .unwrap_or_else(|_| "/assets/personnel-maildir".to_string());
    let target_user = env::var("EXCHANGE_TARGET_USER")
        .expect("FATAL: EXCHANGE_TARGET_USER missing.");
    
    let tenant_id = env::var("AZURE_TENANT_ID").expect("FATAL: AZURE_TENANT_ID missing.");
    let client_id = env::var("AZURE_CLIENT_ID").expect("FATAL: AZURE_CLIENT_ID missing.");
    let secret = env::var("AZURE_CLIENT_SECRET").expect("FATAL: AZURE_CLIENT_SECRET missing.");

    let vault = MaildirVault::init(&archive_path)?;
    println!("SYSTEM EVENT: Totebox archive verified at {}", archive_path);
    println!("SYSTEM EVENT: Entering persistent polling loop. Target: {}", target_user);

    loop {
        match auth::negotiate_token(&tenant_id, &client_id, &secret).await {
            Ok(token) => {
                let bridge = GraphBridge::new(token);
                
                // Initialize physical query with $top=500 for maximum throughput
                let mut current_url = format!(
                    "https://graph.microsoft.com/v1.0/users/{}/messages?$filter=isRead eq false&$top=500",
                    target_user
                );

                // Recursive Pagination Loop
                loop {
                    match bridge.fetch_url(&current_url).await {
                        Ok(messages) => {
                            if let Some(msg_array) = messages["value"].as_array() {
                                if !msg_array.is_empty() {
                                    let mut mutation_count = 0;
                                    for msg in msg_array {
                                        let raw_json = msg.to_string();
                                        
                                        // Physical Extraction
                                        if let Ok(_) = vault.write_payload(&raw_json) {
                                            // State Mutation (Authorized)
                                            if let Some(msg_id) = msg["id"].as_str() {
                                                if let Ok(_) = bridge.mutate_state(&target_user, msg_id).await {
                                                    mutation_count += 1;
                                                } else {
                                                    eprintln!("SYSTEM ERROR: Failed to mutate state for {}", msg_id);
                                                }
                                            }
                                        }
                                    }
                                    println!("SYSTEM EVENT: Extracted and mutated {} payloads to local archive.", mutation_count);
                                }
                            }
                            
                            // Evaluate Microsoft Graph Pagination
                            if let Some(next_link) = messages["@odata.nextLink"].as_str() {
                                println!("SYSTEM EVENT: Network pagination detected. Bypassing throttle to recursively extract next batch.");
                                current_url = next_link.to_string();
                            } else {
                                break; // Mathematical exhaustion achieved. Break recursive loop.
                            }
                        }
                        Err(e) => {
                            eprintln!("SYSTEM ERROR: Graph Bridge extraction failed: {}", e);
                            break; // Break on error to prevent infinite fail loop
                        }
                    }
                }
            }
            Err(e) => eprintln!("SYSTEM ERROR: OAuth2 Negotiation Failed: {}", e),
        }
        
        // Physical Throttle (Only executes when the inbox is mathematically empty)
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
