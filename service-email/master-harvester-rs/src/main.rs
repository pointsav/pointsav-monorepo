use dotenv::from_path;
use futures::future::join_all;
use reqwest::{Client, header};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const BATCH_SIZE: usize = 50;
const GCP_TARGET: &str = "admin@136.117.130.104:/opt/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-email/personnel-maildir/new/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 💥 MASTER DESTRUCTIVE HARVESTER (RUST ASYNC BATCHING)");
    println!("========================================================");

    // 1. Load WORM Credentials
    from_path("/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-personnel/service-email/auth-credentials.env").ok();
    
    let tenant = env::var("AZURE_TENANT_ID").expect("Missing AZURE_TENANT_ID");
    let client_id = env::var("AZURE_CLIENT_ID").expect("Missing AZURE_CLIENT_ID");
    let secret = env::var("AZURE_CLIENT_SECRET").expect("Missing AZURE_CLIENT_SECRET");
    let user = env::var("EXCHANGE_TARGET_USER").expect("Missing EXCHANGE_TARGET_USER");

    let client = Client::new();
    let mut token = get_token(&client, &tenant, &client_id, &secret).await?;

    let folders = vec![
        ("Data_Ingest", "AAMkAGNiMzVmMDMxLTY1OTEtNGQzNC05YzE2LTM2YWMyOWMwOTkyMgAuAAAAAABUZZ-cFXcyR6WM1RpB-73bAQDF6l4UzZZOR6tUM0g2iU_BAAmpzIZlAAA="),
        ("OpenStack", "AAMkAGNiMzVmMDMxLTY1OTEtNGQzNC05YzE2LTM2YWMyOWMwOTkyMgAuAAAAAABUZZ-cFXcyR6WM1RpB-73bAQDF6l4UzZZOR6tUM0g2iU_BAAi_G0ETAAA="),
        ("PostgresSQL", "AAMkAGNiMzVmMDMxLTY1OTEtNGQzNC05YzE2LTM2YWMyOWMwOTkyMgAuAAAAAABUZZ-cFXcyR6WM1RpB-73bAQDF6l4UzZZOR6tUM0g2iU_BAAjT8448AAA="),
    ];

    let local_batch_dir = "/home/mathew/Desktop/NOSAVE_BATCH";
    let mut grand_total = 0;

    for (folder_name, folder_id) in folders {
        println!("\n[SYSTEM] Initiating Destructive Sweep on: {}", folder_name);
        let mut folder_total = 0;

        loop {
            // Fetch Batch
            let url = format!("https://graph.microsoft.com/v1.0/users/{}/mailFolders/{}/messages?$top={}&$select=id", user, folder_id, BATCH_SIZE);
            let res = client.get(&url).bearer_auth(&token).send().await?;
            
            if res.status() == 401 {
                println!("  -> [SYSTEM] Token expired. Renegotiating...");
                token = get_token(&client, &tenant, &client_id, &secret).await?;
                continue;
            }

            let msg_data: Value = res.json().await?;
            let messages = match msg_data["value"].as_array() {
                Some(arr) if !arr.is_empty() => arr,
                _ => {
                    println!("[SUCCESS] {} is mathematically empty. Total destroyed: {}", folder_name, folder_total);
                    break;
                }
            };

            // Recreate Local Directory
            let _ = fs::remove_dir_all(local_batch_dir);
            fs::create_dir_all(local_batch_dir)?;

            let mut download_futures = vec![];
            let mut batch_ids = vec![];

            println!("  -> Pulling async batch of {} assets...", messages.len());

            for (idx, msg) in messages.iter().enumerate() {
                let msg_id = msg["id"].as_str().unwrap().to_string();
                batch_ids.push(msg_id.clone());
                
                let local_file = format!("{}/NOSAVE_{}_{}.eml", local_batch_dir, folder_name, folder_total + idx + 1);
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

            // Transmit via PSST Airlock
            println!("  -> Pushing batch to GCP Node via WORM airlock...");
            let batch_path = format!("{}/", local_batch_dir);
            let _ = Command::new("rsync").args(["-avz", "-e", "ssh", &batch_path, GCP_TARGET]).output();

            // Issue Async Hard Deletes
            println!("  -> Issuing {} Async Hard Deletes to Microsoft...", batch_ids.len());
            let mut delete_futures = vec![];
            for msg_id in &batch_ids {
                let delete_url = format!("https://graph.microsoft.com/v1.0/users/{}/messages/{}", user, msg_id);
                let t_token = token.clone();
                let t_client = client.clone();
                delete_futures.push(tokio::spawn(async move {
                    let _ = t_client.delete(&delete_url).bearer_auth(t_token).send().await;
                }));
            }
            join_all(delete_futures).await;

            folder_total += batch_ids.len();
            grand_total += batch_ids.len();
            println!("     [+] {} assets processed in {} so far...", folder_total, folder_name);
        }
    }

    println!("\n========================================================");
    println!("[SUCCESS] BURN COMPLETE. Grand Total Extracted & Destroyed: {}", grand_total);
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
