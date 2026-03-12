use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use std::net::IpAddr;
use chrono::{Utc, DateTime, Duration, Datelike};
use maxminddb::Reader;

fn main() {
    println!("[SYSTEM] Initiating Sovereign Telemetry Synthesis...");

    let fleet_id = env::var("FLEET_ID").unwrap_or_else(|_| "UNKNOWN_FLEET".to_string());
    let current_time = Utc::now();
    let current_date = current_time.format("%Y-%m-%d").to_string();
    
    let db_path = "assets/GeoLite2-City.mmdb";
    let ledger_path = "assets/ledger_telemetry.csv";
    let report_path = format!("outbox/REPORT_{}_{}.md", fleet_id, current_date);

    let _ = fs::create_dir_all("outbox");

    // Initialize Time Buckets
    let mut count_yesterday = 0;
    let mut count_7d = 0;
    let mut count_30d = 0;
    let mut count_60d = 0;
    let mut count_90d = 0;
    let mut count_ytd = 0;
    let mut count_inception = 0;

    let mut metro_counts: HashMap<String, usize> = HashMap::new();
    let geo_reader = Reader::open_readfile(db_path).ok();

    // Parse the Flat-File Ledger
    if let Ok(contents) = fs::read_to_string(ledger_path) {
        for line in contents.lines() {
            // Strip quotes and split: "IP","TIMESTAMP","URI","USER_AGENT"
            let clean_line = line.replace("\"", "");
            let parts: Vec<&str> = clean_line.split(',').collect();
            
            if parts.len() >= 2 {
                let ip_str = parts[0];
                let time_str = parts[1];

                if let Ok(event_time) = DateTime::parse_from_rfc3339(time_str).map(|dt| dt.with_timezone(&Utc)) {
                    count_inception += 1;
                    
                    let duration = current_time.signed_duration_since(event_time);
                    let days_ago = duration.num_days();

                    if days_ago <= 1 && days_ago >= 0 { count_yesterday += 1; }
                    if days_ago <= 7 { count_7d += 1; }
                    if days_ago <= 30 { count_30d += 1; }
                    if days_ago <= 60 { count_60d += 1; }
                    if days_ago <= 90 { count_90d += 1; }
                    
                    if event_time.year() == current_time.year() { count_ytd += 1; }

                    // Metro Region Resolution (Offline)
                    let metro_name = if let Some(ref reader) = geo_reader {
                        if let Ok(ip) = ip_str.parse::<IpAddr>() {
                            if let Ok(city) = reader.lookup::<maxminddb::geoip2::City>(ip) {
                                city.city.and_then(|c| c.names).and_then(|n| n.get("en").map(|&s| s.to_string()))
                                    .unwrap_or_else(|| "Unknown_Region".to_string())
                            } else {
                                "Unresolved_IP".to_string()
                            }
                        } else {
                            "Malformed_IP".to_string()
                        }
                    } else {
                        "[OFFLINE_DB_REQUIRED]".to_string()
                    };

                    *metro_counts.entry(metro_name).or_insert(0) += 1;
                }
            }
        }
    }

    // Sort Metro Density
    let mut sorted_metros: Vec<(&String, &usize)> = metro_counts.iter().collect();
    sorted_metros.sort_by(|a, b| b.1.cmp(a.1));

    // Construct Brutalist Markdown Report
    let mut report = String::new();
    report.push_str(&format!("# 📊 FLEET TELEMETRY LEDGER | {}\n", fleet_id));
    report.push_str(&format!("**Generated:** {}\n", current_time.to_rfc3339()));
    report.push_str("**Standard:** Sovereign Data Protocol (DS-ADR-06)\n\n---\n\n");

    if geo_reader.is_none() {
        report.push_str("> [!WARNING]\n> Offline GeoLite2-City.mmdb not found in /assets/. Metro parsing bypassed.\n\n");
    }

    report.push_str("## ⏱️ TIME MATRIX\n");
    report.push_str("| Interval | Unique Events |\n| :--- | :--- |\n");
    report.push_str(&format!("| Yesterday | {} |\n", count_yesterday));
    report.push_str(&format!("| 7 Days | {} |\n", count_7d));
    report.push_str(&format!("| 30 Days | {} |\n", count_30d));
    report.push_str(&format!("| 60 Days | {} |\n", count_60d));
    report.push_str(&format!("| 90 Days | {} |\n", count_90d));
    report.push_str(&format!("| YTD | {} |\n", count_ytd));
    report.push_str(&format!("| Inception | {} |\n\n", count_inception));

    report.push_str("## 🌍 METRO REGION MATRIX\n");
    report.push_str("| Metro Area | Volume | Density % |\n| :--- | :--- | :--- |\n");
    
    for (metro, &count) in sorted_metros.into_iter().take(15) {
        let density = if count_inception > 0 { (count as f64 / count_inception as f64) * 100.0 } else { 0.0 };
        report.push_str(&format!("| {} | {} | {:.1}% |\n", metro, count, density));
    }

    // Write to Outbox
    let mut file = File::create(&report_path).expect("[ERROR] Cannot write to outbox.");
    file.write_all(report.as_bytes()).expect("[ERROR] Disk write failure.");

    println!("[SUCCESS] Synthesized Report anchored at: {}", report_path);
}
