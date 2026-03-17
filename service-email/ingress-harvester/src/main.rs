use std::env;
use std::fs;
use std::io::Write;
use native_tls::TlsConnector;

fn main() {
    println!("========================================================");
    println!(" 📡 INGRESS HARVESTER: DESTRUCTIVE M365 AIR-BRIDGE");
    println!("========================================================");

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
            eprintln!("  -> [ERROR] M365 Authentication Failed. ({})", e.0);
            std::process::exit(1);
        }
    };

    println!("  -> Accessing restricted folder: totebox-ingress...");
    if imap_session.select("totebox-ingress").is_err() {
        if imap_session.select("INBOX/totebox-ingress").is_err() {
            eprintln!("  -> [ERROR] Target folder not found. Halting execution.");
            std::process::exit(1);
        }
    }

    let messages = imap_session.search("ALL").unwrap_or_default();
    if messages.is_empty() {
        println!("  -> [SYSTEM] The ingress folder is mathematically empty.");
        imap_session.logout().unwrap();
        return;
    }

    let target_batch: Vec<u32> = messages.into_iter().take(10).collect();
    println!("  -> [SYSTEM] Detected {} payloads. Extracting {}...", messages.len(), target_batch.len());

    let mut deletion_count = 0;

    for sequence_number in target_batch {
        let seq_str = sequence_number.to_string();
        let mut download_success = false;
        
        // PHYSICAL MEMORY BOUNDARY: Scope the IMAP read lock
        {
            if let Ok(msg_stream) = imap_session.fetch(&seq_str, "RFC822") {
                for message in msg_stream.iter() {
                    if let Some(body) = message.body() {
                        let out_path = format!("{}/M365_PAYLOAD_{}.eml", ingress_dir, sequence_number);
                        if let Ok(mut file) = fs::File::create(&out_path) {
                            if file.write_all(body).is_ok() {
                                println!("     [SUCCESS] Downloaded: {}", out_path);
                                download_success = true;
                            }
                        }
                    }
                }
            }
        } // Read lock is mathematically released here

        // Now we safely assert a write lock to flag for deletion
        if download_success {
            let _ = imap_session.store(&seq_str, "+FLAGS (\\Deleted)");
            deletion_count += 1;
        }
    }

    if deletion_count > 0 {
        println!("  -> [SYSTEM] Executing IMAP Expunge. Permanently obliterating {} emails...", deletion_count);
        let _ = imap_session.expunge();
    }

    imap_session.logout().unwrap();
    println!("--------------------------------------------------------");
    println!("[SUCCESS] Destructive extraction cycle complete.");
}
