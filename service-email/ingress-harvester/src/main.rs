use std::env;
use std::fs;
use std::io::Write;
use native_tls::TlsConnector;

fn main() {
    println!("========================================================");
    println!(" 📡 INGRESS HARVESTER: DESTRUCTIVE M365 AIR-BRIDGE");
    println!("========================================================");

    // Load local air-gapped credentials
    dotenv::dotenv().ok();
    let email_user = env::var("M365_USER").expect("[ERROR] M365_USER missing in .env");
    let email_pass = env::var("M365_APP_PASS").expect("[ERROR] M365_APP_PASS missing in .env");
    let totebox_root = env::var("TOTEBOX_ROOT").expect("[ERROR] TOTEBOX_ROOT missing in .env");

    let ingress_dir = format!("{}/service-people/raw-ingress", totebox_root);
    let _ = fs::create_dir_all(&ingress_dir);

    println!("  -> Establishing TLS encrypted diode to outlook.office365.com...");
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect(("outlook.office365.com", 993), "outlook.office365.com", &tls).unwrap();

    let mut imap_session = match client.login(&email_user, &email_pass) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  -> [ERROR] M365 Authentication Failed. Verify App Password. ({})", e.0);
            std::process::exit(1);
        }
    };

    println!("  -> Accessing restricted folder: totebox-ingress...");
    // Strict enforcement: Only operate within this specific folder
    match imap_session.select("totebox-ingress") {
        Ok(_) => {},
        Err(_) => {
            println!("  -> [SYSTEM] 'totebox-ingress' not found at root. Attempting 'INBOX/totebox-ingress'...");
            if let Err(e) = imap_session.select("INBOX/totebox-ingress") {
                eprintln!("  -> [ERROR] Target folder not found. Halting execution to protect data. ({})", e);
                std::process::exit(1);
            }
        }
    }

    // Search for ALL messages in this specific folder
    let messages = imap_session.search("ALL").unwrap_or_default();
    
    if messages.is_empty() {
        println!("  -> [SYSTEM] The ingress folder is mathematically empty.");
        imap_session.logout().unwrap();
        return;
    }

    // Apply the 10-Batch Throttle
    let target_batch: Vec<u32> = messages.into_iter().take(10).collect();
    println!("  -> [SYSTEM] Detected {} payloads. Destructively extracting {} for this cycle.", messages.len(), target_batch.len());

    let mut deletion_count = 0;

    for sequence_number in target_batch {
        if let Ok(msg_stream) = imap_session.fetch(sequence_number.to_string(), "RFC822") {
            for message in msg_stream.iter() {
                if let Some(body) = message.body() {
                    let out_path = format!("{}/M365_PAYLOAD_{}.eml", ingress_dir, sequence_number);
                    let mut file = fs::File::create(&out_path).unwrap();
                    file.write_all(body).unwrap();
                    println!("     [SUCCESS] Downloaded: {}", out_path);
                    
                    // DESTRUCTIVE EXTRACTION: Flag for permanent deletion
                    let _ = imap_session.store(sequence_number.to_string(), "+FLAGS (\\Deleted)");
                    deletion_count += 1;
                }
            }
        }
    }

    if deletion_count > 0 {
        println!("  -> [SYSTEM] Executing IMAP Expunge. Permanently obliterating {} emails from Microsoft Servers...", deletion_count);
        let _ = imap_session.expunge();
    }

    imap_session.logout().unwrap();
    println!("--------------------------------------------------------");
    println!("[SUCCESS] Destructive extraction cycle complete.");
}
