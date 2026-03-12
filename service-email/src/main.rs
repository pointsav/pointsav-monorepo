mod maildir;
mod graph_client;
mod auth;

use maildir::MaildirVault;
use graph_client::GraphBridge;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SYSTEM EVENT: Initializing Sovereign Email Bridge daemon.");

    // Structural Parameter Extraction
    let archive_path = env::var("TOTEBOX_ARCHIVE_PATH")
        .unwrap_or_else(|_| "/assets/personnel-maildir".to_string());
    let target_user = env::var("EXCHANGE_TARGET_USER")
        .expect("FATAL: EXCHANGE_TARGET_USER missing.");
    
    // Quarantined Commercial Identity Extraction
    let tenant_id = env::var("AZURE_TENANT_ID")
        .expect("FATAL: AZURE_TENANT_ID missing.");
    let client_id = env::var("AZURE_CLIENT_ID")
        .expect("FATAL: AZURE_CLIENT_ID missing.");
    let secret = env::var("AZURE_CLIENT_SECRET")
        .expect("FATAL: AZURE_CLIENT_SECRET missing.");

    let vault = MaildirVault::init(&archive_path)?;
    println!("SYSTEM EVENT: Totebox archive verified at {}", archive_path);
    println!("SYSTEM EVENT: Entering persistent polling loop (60s interval). Target: {}", target_user);

    // Continuous Extraction Loop
    loop {
        // Dynamic negotiation prevents OAuth2 token expiration
        match auth::negotiate_token(&tenant_id, &client_id, &secret).await {
            Ok(token) => {
                let bridge = GraphBridge::new(token);
                match bridge.fetch_inbox(&target_user).await {
                    Ok(messages) => {
                        if let Some(msg_array) = messages["value"].as_array() {
                            if !msg_array.is_empty() {
                                for msg in msg_array {
                                    let raw_json = msg.to_string();
                                    let _ = vault.write_payload(&raw_json);
                                }
                                println!("SYSTEM EVENT: Extracted {} payloads to local archive.", msg_array.len());
                            }
                        }
                    }
                    Err(e) => eprintln!("SYSTEM ERROR: Graph Bridge extraction failed: {}", e),
                }
            }
            Err(e) => eprintln!("SYSTEM ERROR: OAuth2 Negotiation Failed: {}", e),
        }
        
        // Physical Throttle
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
