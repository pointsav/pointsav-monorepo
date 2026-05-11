use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::{Utc, Local};
use webbrowser;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
struct EntityBundle {
    sovereign_id: String,
    display_name: String,
    entity_type: String,
    status: String,
    claims: serde_json::Value,
    provenance: serde_json::Value,
    verified_data: Option<VerifiedData>,
}

#[derive(Serialize, Deserialize, Clone)]
struct VerifiedData {
    domain: String,
    theme: String,
    archetype: String,
    chart_of_accounts: String,
    verified_url: String,
    verified_by: String,
    timestamp: String,
}

#[derive(Deserialize)] struct SeedArchetypes { archetypes: Vec<Archetype> }
#[derive(Deserialize)] struct Archetype { name: String }

#[derive(Deserialize)] struct SeedCOA { chart_of_accounts: Vec<COA> }
#[derive(Deserialize)] struct COA { sub_domain: String }

#[derive(Deserialize)] struct SeedDomains { domains: Vec<Domain> }
#[derive(Deserialize)] struct Domain { domain_name: String }

#[derive(Deserialize)] struct SeedThemes { themes: Vec<Theme> }
#[derive(Deserialize)] struct Theme { name: String }

const MAX_DAILY_VERIFICATIONS: usize = 10;
const THROTTLE_LOG: &str = ".surveyor_throttle.log";

fn check_throttle() -> bool {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut count = 0;

    if let Ok(content) = fs::read_to_string(THROTTLE_LOG) {
        if content.starts_with(&today) {
            count = content.split(',').nth(1).unwrap_or("0").trim().parse().unwrap_or(0);
        }
    }

    if count >= MAX_DAILY_VERIFICATIONS {
        println!("\n[SYSTEM LOCK] Cognitive Throttle Reached.");
        println!("You have completed your {} verifications for {}.", MAX_DAILY_VERIFICATIONS, today);
        println!("The system is locked to ensure data fidelity. Return tomorrow.\n");
        return false;
    }
    true
}

fn increment_throttle() {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut count = 0;

    if let Ok(content) = fs::read_to_string(THROTTLE_LOG) {
        if content.starts_with(&today) {
            count = content.split(',').nth(1).unwrap_or("0").trim().parse().unwrap_or(0);
        }
    }
    
    count += 1;
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(THROTTLE_LOG).unwrap();
    writeln!(file, "{},{}", today, count).unwrap();
}

fn get_selection(prompt: &str, options: &[String]) -> String {
    println!("\n{}", prompt);
    for (i, opt) in options.iter().enumerate() {
        println!("  {}) {}", i + 1, opt);
    }
    
    loop {
        print!("Select an option (1-{}): ", options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(idx) = input.trim().parse::<usize>() {
            if idx > 0 && idx <= options.len() {
                return options[idx - 1].clone();
            }
        }
        println!("[ERROR] Invalid selection.");
    }
}

fn load_options<T, F>(path: &str, extractor: F) -> Vec<String> 
where 
    T: for<'de> Deserialize<'de>,
    F: Fn(T) -> Vec<String>
{
    let content = fs::read_to_string(path).unwrap_or_else(|_| format!("{{}}"));
    let data: T = serde_json::from_str(&content).unwrap_or_else(|_| panic!("Failed to parse {}", path));
    extractor(data)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("[ERROR] Usage: app-console-content <SEED_VAULT_DIR> <DISCOVERY_QUEUE_DIR> <VERIFIED_LEDGER_DIR>");
        std::process::exit(1);
    }

    let seed_dir = &args[1];
    let queue_dir = &args[2];
    let output_dir = &args[3];

    println!("========================================================");
    println!(" 🏛️ POINT SAV SOVEREIGN SURVEYOR (CLI INTERFACE)");
    println!("========================================================");

    if !check_throttle() { std::process::exit(0); }

    let mut target_file = None;
    if let Ok(entries) = fs::read_dir(queue_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                target_file = Some(entry.path());
                break;
            }
        }
    }

    let file_path = match target_file {
        Some(p) => p,
        None => {
            println!("[SYSTEM] The Discovery Queue is empty. No identities require verification.");
            std::process::exit(0);
        }
    };

    let content = fs::read_to_string(&file_path).unwrap();
    let mut entity: EntityBundle = serde_json::from_str(&content).expect("Failed to parse Entity Bundle");

    // 1. Enhanced TUI Display
    println!("\n[TARGET IDENTITY]: {}", entity.display_name.to_uppercase());
    println!("--------------------------------------------------------");
    println!("| TYPE: {}", entity.entity_type);
    println!("| SOURCE: {}", entity.provenance.get("source_file").and_then(|v| v.as_str()).unwrap_or("Unknown"));
    
    let mut search_string = format!("site:linkedin.com/in/ OR site:linkedin.com/company/ \"{}\"", entity.display_name);
    
    if let Some(claims) = entity.claims.as_object() {
        if let Some(company) = claims.get("company").and_then(|v| v.as_str()) {
            println!("| COMPANY: {}", company);
            search_string.push_str(&format!(" \"{}\"", company));
        }
        if let Some(position) = claims.get("position").and_then(|v| v.as_str()) {
            println!("| POSITION: {}", position);
            // We don't always add position to the search string as it can be too restrictive, 
            // but displaying it helps the human operator.
        }
        if let Some(email) = claims.get("email").and_then(|v| v.as_str()) {
            println!("| EMAIL: {}", email);
        }
    }
    println!("--------------------------------------------------------");

    // 2. The Laser Query
    println!("\n[SYSTEM] Launching targeted Google query...");
    
    let search_query = format!("https://www.google.com/search?q={}", search_string.replace(" ", "+"));
    
    if webbrowser::open(&search_query).is_err() {
        println!("[WARNING] Could not auto-launch browser. Please search manually.");
    }
    
    thread::sleep(Duration::from_secs(2));

    let archetypes = load_options::<SeedArchetypes, _>(&format!("{}/Archetypes.json", seed_dir), |data| data.archetypes.into_iter().map(|a| a.name).collect());
    let coa = load_options::<SeedCOA, _>(&format!("{}/ChartOfAccounts.json", seed_dir), |data| data.chart_of_accounts.into_iter().map(|c| c.sub_domain).collect());
    let domains = load_options::<SeedDomains, _>(&format!("{}/Domains.json", seed_dir), |data| data.domains.into_iter().map(|d| d.domain_name).collect());
    let themes = load_options::<SeedThemes, _>(&format!("{}/Themes.json", seed_dir), |data| data.themes.into_iter().map(|t| t.name).collect());

    print!("\nPaste the verified LinkedIn or Corporate URL (or type 'skip'/'delete'): ");
    io::stdout().flush().unwrap();
    let mut verified_url = String::new();
    io::stdin().read_line(&mut verified_url).unwrap();
    
    let trimmed_url = verified_url.trim().to_lowercase();
    if trimmed_url == "delete" {
        fs::remove_file(&file_path).unwrap();
        println!("\n[SYSTEM] Entity '{}' permanently destroyed. Returning to pool.", entity.display_name);
        std::process::exit(0);
    } else if trimmed_url == "skip" {
        println!("\n[SYSTEM] Entity '{}' skipped. Returning to pool.", entity.display_name);
        std::process::exit(0);
    }

    let sel_domain = get_selection("Which DOMAIN (Glossary) does this entity belong to?", &domains);
    let sel_theme = get_selection("Which THEME (Concept/Trend) drives their operation?", &themes);
    let sel_coa = get_selection("Where do they slot into our CHART OF ACCOUNTS (Categorization)?", &coa);
    let sel_archetype = get_selection("What is their ARCHETYPE (Logo/Persona)?", &archetypes);

    entity.status = "Verified".to_string();
    entity.verified_data = Some(VerifiedData {
        domain: sel_domain,
        theme: sel_theme,
        archetype: sel_archetype,
        chart_of_accounts: sel_coa,
        verified_url: verified_url.trim().to_string(),
        verified_by: "OPERATOR_LOCAL".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    });

    fs::create_dir_all(output_dir).unwrap();
    let out_path = format!("{}/{}.json", output_dir, entity.sovereign_id);
    let out_json = serde_json::to_string_pretty(&entity).unwrap();
    fs::write(&out_path, out_json).unwrap();
    
    fs::remove_file(file_path).unwrap();

    increment_throttle();

    println!("\n========================================================");
    println!("✅ [SUCCESS] Entity '{}' mathematically verified and vaulted.", entity.display_name);
    println!("========================================================");
}
