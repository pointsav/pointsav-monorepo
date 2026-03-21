use futures::future::join_all;
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const SPOOL_DIR: &str = "/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-email/personnel-maildir/new";
const LEDGER_PATH: &str = "/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-email/downloaded_ledger.txt";
const TARGET_FOLDER: &str = "totebox-ingress";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 🌐 SELF-HEALING GRAPH HARVESTER (DYNAMIC INGRESS)");
    println!("========================================================");

    let env_content = fs::read_to_string("/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-email/auth-credentials.env")
        .expect("[FATAL] Missing auth-credentials.env");
        
    let mut tenant = String::new();
    let mut client_id = String::new();
    let mut secret = String::new();
    let mut user = String::new();

    for line in env_content.lines() {
        if line.starts_with("AZURE_TENANT_ID=") { tenant = line.replace("AZURE_TENANT_ID=", "").replace("\"", ""); }
        if line.starts_with("AZURE_CLIENT_ID=") { client_id = line.replace("AZURE_CLIENT_ID=", "").replace("\"", ""); }
        if line.starts_with("AZURE_CLIENT_SECRET=") { secret = line.replace("AZURE_CLIENT_SECRET=", "").replace("\"", ""); }
        if line.starts_with("EXCHANGE_TARGET_USER=") { user = line.replace("EXCHANGE_TARGET_USER=", "").replace("\"", ""); }
    }

    let client = Client::new();
    let token = get_token(&client, &tenant, &client_id, &secret).await?;
    println!("[SYSTEM] Graph API OAuth2 Token Negotiated.");

    // 1. Dynamically Hunt for the Folder
    println!("[SYSTEM] Hunting for Target Folder: {}", TARGET_FOLDER);
    let folders_url = format!("https://graph.microsoft.com/v1.0/users/{}/mailFolders?$top=250", user);
    let folders_res = client.get(&folders_url).bearer_auth(&token).send().await?;
    let folders_data: Value = folders_res.json().await?;
    
    let mut target_id = String::new();
    if let Some(folders) = folders_data["value"].as_array() {
        for folder in folders {
            if folder["displayName"].as_str().unwrap_or("") == TARGET_FOLDER {
                target_id = folder["id"].as_str().unwrap().to_string();
                break;
            }
        }
    }

    if target_id.is_empty() {
        println!("[FATAL] Folder '{}' not found in the primary mailbox.", TARGET_FOLDER);
        return Ok(());
    }
    println!("[SUCCESS] Live Folder ID Acquired: {}...", &target_id[..15]);

    // 2. Prepare Vaults and Ledger
    fs::create_dir_all(SPOOL_DIR)?;
    let mut seen_ids = std::collections::HashSet::new();
    if let Ok(file) = fs::File::open(LEDGER_PATH) {
        for line in BufReader::new(file).lines().flatten() { seen_ids.insert(line); }
    }
    let mut ledger_file = OpenOptions::new().create(true).append(true).open(LEDGER_PATH)?;

    // 3. Extract Message IDs
    println!("[SYSTEM] Sweeping for unvaulted assets...");
    let msg_url = format!("https://graph.microsoft.com/v1.0/users/{}/mailFolders/{}/messages?$top=50&$select=id", user, target_id);
    let msg_res = client.get(&msg_url).bearer_auth(&token).send().await?;
    let msg_data: Value = msg_res.json().await?;
    
    let messages = match msg_data["value"].as_array() {
        Some(arr) if !arr.is_empty() => arr,
        _ => {
            println!("  -> [STATUS] Folder is mathematically empty.");
            return Ok(());
        }
    };

    let mut download_futures = vec![];
    let mut batch_ids = vec![];

    for msg in messages {
        let msg_id = msg["id"].as_str().unwrap().to_string();
        if seen_ids.contains(&msg_id) { continue; }
        
        batch_ids.push(msg_id.clone());
        let local_file = format!("{}/NOSAVE_totebox_{}.eml", SPOOL_DIR, batch_ids.len());
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

    if batch_ids.is_empty() {
        println!("  -> [STATUS] No new assets detected.");
        return Ok(());
    }

    println!("  -> Directly vaulting {} NEW MIME payloads to Spool Directory...", batch_ids.len());
    join_all(download_futures).await;

    for id in &batch_ids {
        writeln!(ledger_file, "{}", id)?;
    }

    println!("\n========================================================");
    println!("[SUCCESS] GRAPH SYNC COMPLETE. Total New Assets: {}", batch_ids.len());
    Ok(())
}

async fn get_token(client: &Client, tenant: &str, client_id: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant);
    let params = [("client_id", client_id), ("scope", "https://graph.microsoft.com/.default"), ("client_secret", secret), ("grant_type", "client_credentials")];
    let res = client.post(&url).form(&params).send().await?;
    let json: Value = res.json().await?;
    Ok(json["access_token"].as_str().unwrap().to_string())
}
