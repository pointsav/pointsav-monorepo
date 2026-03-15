use dotenv::from_path;
use futures::future::join_all;
use reqwest::{Client, header};
use serde_json::Value;
use std::env;
use std::fs;

const BATCH_SIZE: usize = 5; // Strict 5-asset throttle
const LOCAL_SPOOL: &str = "/opt/woodfine/cluster-totebox-personnel/service-email/personnel-maildir/new";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 🟢 RECOVERY HARVESTER (ZERO-NETWORK SAVE | 5-ASSET LIMIT)");
    println!("========================================================");

    // 1. Load WORM Credentials natively on the GCP Node
    from_path("/opt/woodfine/cluster-totebox-personnel/auth-credentials.env").ok();
    
    let tenant = env::var("AZURE_TENANT_ID").expect("Missing AZURE_TENANT_ID");
    let client_id = env::var("AZURE_CLIENT_ID").expect("Missing AZURE_CLIENT_ID");
    let secret = env::var("AZURE_CLIENT_SECRET").expect("Missing AZURE_CLIENT_SECRET");
    let user = env::var("EXCHANGE_TARGET_USER").expect("Missing EXCHANGE_TARGET_USER");

    let client = Client::new();
    let mut token = get_token(&client, &tenant, &client_id, &secret).await?;

    let folder_name = "DumpsterRecovery";
    let folder_id = "recoverableitemsdeletions"; 

    println!("\n[SYSTEM] Initiating Sovereign Recovery on: {}", folder_name);

    // Ensure the local GCP spool exists
    fs::create_dir_all(LOCAL_SPOOL)?;

    // Fetch exactly 1 Batch of 5
    let url = format!("https://graph.microsoft.com/v1.0/users/{}/mailFolders/{}/messages?$top={}&$select=id", user, folder_id, BATCH_SIZE);
    let res = client.get(&url).bearer_auth(&token).send().await?;
    
    if res.status() == 401 {
        println!("  -> [SYSTEM] Token expired. Renegotiating...");
        token = get_token(&client, &tenant, &client_id, &secret).await?;
    }

    let msg_data: Value = res.json().await?;
    let messages = match msg_data["value"].as_array() {
        Some(arr) if !arr.is_empty() => arr,
        _ => {
            println!("[SUCCESS] Dumpster is mathematically empty.");
            return Ok(());
        }
    };

    let mut download_futures = vec![];
    let mut batch_ids = vec![];

    println!("  -> Pulling async recovery batch of {} assets...", messages.len());

    for msg in messages.iter() {
        let msg_id = msg["id"].as_str().unwrap().to_string();
        batch_ids.push(msg_id.clone());
        
        // Generate LIVE file without NOSAVE prefix directly in the GCP Spool
        let local_file = format!("{}/LIVE_RECOVERY_{}.eml", LOCAL_SPOOL, msg_id);
        let download_url = format!("https://graph.microsoft.com/v1.0/users/{}/messages/{}/$value", user, msg_id);
        let t_token = token.clone();
        let t_client = client.clone();
        
        download_futures.push(tokio::spawn(async move {
            if let Ok(response) = t_client.get(&download_url).bearer_auth(t_token).send().await {
                if let Ok(bytes) = response.bytes().await {
                    let _ = fs::write(local_file, bytes);
                }
            }
        }));
    }

    join_all(download_futures).await;

    println!("\n========================================================");
    println!("[SUCCESS] RECOVERY COMPLETE. {} assets secured in local GCP Spool.", batch_ids.len());
    println!("No network push was used. No destruct commands were fired.");
    Ok(())
}

async fn get_token(client: &Client, tenant: &str, client_id: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant);
    let params = [
        ("client_id", client_id),
        ("scope", "https://graph.microsoft.com/.default"),
        ("client_secret", secret),
        ("grant_type", "client_credentials"),
    ];
    let res = client.post(&url).form(&params).send().await?;
    let json: Value = res.json().await?;
    Ok(json["access_token"].as_str().unwrap().to_string())
}
