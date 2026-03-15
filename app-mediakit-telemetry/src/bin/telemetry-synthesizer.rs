use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration, Datelike, TimeZone};

struct TelemetryRecord {
    received_at: DateTime<Utc>,
    masked_ip: String,
    uri: String,
    referrer: String,
    viewport: String,
    timezone: String,
    device_memory: String,
    hardware_cores: String,
    dwell_seconds: i64,
    scroll_depth: i64,
    intent_clicks: String,
}

fn get_distribution(items: Vec<String>) -> String {
    let mut counts = HashMap::new();
    for item in items {
        let clean = item.trim().to_string();
        if clean != "unknown" && clean != "" && !clean.contains("***.***.***.***") {
            *counts.entry(clean).or_insert(0) += 1;
        }
    }
    
    if counts.is_empty() { 
        return "  - *No verifiable data points recorded*\n".to_string(); 
    }
    
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    sorted.into_iter()
        .map(|(k, v)| format!("  - **{}**: {} instances", k, v))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn main() {
    let assets_dir = Path::new("./assets/");
    let report_path = format!("./outbox/REPORT_TELEMETRY_{}.md", Utc::now().format("%Y%m%d_%H%M%S"));
    let mut records: Vec<TelemetryRecord> = Vec::new();

    // 1. Shard Ingestion Engine
    if let Ok(entries) = fs::read_dir(assets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(OsStr::to_str) == Some("csv") {
                if let Ok(file) = File::open(&path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        let cols: Vec<&str> = line.split(',').collect();
                        
                        let (masked_ip, uri, referrer, viewport, timezone, memory, cores, dwell, scroll, clicks);
                        
                        // V4 Architecture (15 Columns)
                        if cols.len() >= 15 {
                            masked_ip = cols[2].to_string();
                            uri = cols[3].to_string();
                            referrer = cols[5].to_string();
                            viewport = cols[6].to_string();
                            timezone = cols[7].to_string();
                            memory = cols[8].to_string();
                            cores = cols[9].to_string();
                            dwell = cols[10].parse().unwrap_or(0);
                            scroll = cols[11].parse().unwrap_or(0);
                            clicks = cols[12].to_string();
                        } 
                        // Legacy V3 Architecture (14 Columns)
                        else if cols.len() == 14 {
                            masked_ip = "***.***.***.*** (Legacy)".to_string();
                            uri = cols[2].to_string();
                            referrer = cols[4].to_string();
                            viewport = cols[5].to_string();
                            timezone = cols[6].to_string();
                            memory = cols[7].to_string();
                            cores = cols[8].to_string();
                            dwell = cols[9].parse().unwrap_or(0);
                            scroll = cols[10].parse().unwrap_or(0);
                            clicks = cols[11].to_string();
                        } else {
                            continue; // Bypass heavily fragmented rows
                        }

                        if let Ok(received) = DateTime::parse_from_rfc3339(cols[0]) {
                            records.push(TelemetryRecord {
                                received_at: received.with_timezone(&Utc),
                                masked_ip, uri, referrer, viewport, timezone,
                                device_memory: memory, hardware_cores: cores,
                                dwell_seconds: dwell, scroll_depth: scroll, intent_clicks: clicks,
                            });
                        }
                    }
                }
            }
        }
    }

    if records.is_empty() {
        println!("No parseable data found in ./assets/. Exiting synthesis.");
        return;
    }

    // 2. Report Generation
    let mut out = File::create(&report_path).expect("Failed to create report.");
    
    writeln!(out, "# INSTITUTIONAL TELEMETRY INTELLIGENCE LEDGER").unwrap();
    writeln!(out, "**Generated:** {}  \n", Utc::now().to_rfc3339()).unwrap();
    writeln!(out, "*This report complies with the Sovereign Data Protocol. PII is scrubbed via /24 Subnet Masking.* \n\n").unwrap();
    writeln!(out, "---\n").unwrap();

    let now = Utc::now();
    let ytd_start = Utc.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();

    let windows = vec![
        ("1 DAY (24H)", now - Duration::days(1)),
        ("1 WEEK (7D)", now - Duration::days(7)),
        ("30 DAYS", now - Duration::days(30)),
        ("60 DAYS", now - Duration::days(60)),
        ("90 DAYS", now - Duration::days(90)),
        ("YEAR TO DATE (YTD)", ytd_start),
        ("INCEPTION", Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap()),
    ];

    for (label, start_time) in windows {
        let subset: Vec<&TelemetryRecord> = records.iter().filter(|r| r.received_at >= start_time).collect();
        
        writeln!(out, "## {}\n", label).unwrap();
        
        if subset.is_empty() {
            writeln!(out, "> **No operations recorded in this window.**\n\n---\n").unwrap();
            continue;
        }

        let total_loads = subset.len();
        let avg_dwell = subset.iter().map(|r| r.dwell_seconds).sum::<i64>() / total_loads as i64;
        let max_dwell = subset.iter().map(|r| r.dwell_seconds).max().unwrap_or(0);
        let avg_scroll = subset.iter().map(|r| r.scroll_depth).sum::<i64>() / total_loads as i64;
        
        let clicks_raw: Vec<String> = subset.iter()
            .filter(|r| r.intent_clicks != "none")
            .flat_map(|r| r.intent_clicks.split(" | ").map(|s| s.to_string()))
            .collect();
        
        let subnets = get_distribution(subset.iter().map(|r| r.masked_ip.clone()).collect());
        let viewports = get_distribution(subset.iter().map(|r| r.viewport.clone()).collect());
        let timezones = get_distribution(subset.iter().map(|r| r.timezone.clone()).collect());
        let referrers = get_distribution(subset.iter().map(|r| r.referrer.clone()).collect());
        let memory = get_distribution(subset.iter().map(|r| format!("{} GB", r.device_memory)).collect());
        let cores = get_distribution(subset.iter().map(|r| format!("{} Cores", r.hardware_cores)).collect());
        let clicks_dist = get_distribution(clicks_raw);
            
        writeln!(out, "### 1. Core Engagement Volume").unwrap();
        writeln!(out, "- **Total Asset Renderings:** {}", total_loads).unwrap();
        writeln!(out, "- **Average Dwell Time:** {} seconds", avg_dwell).unwrap();
        writeln!(out, "- **Maximum Dwell Time:** {} seconds", max_dwell).unwrap();
        writeln!(out, "- **Average Scroll Depth:** {}%\n", avg_scroll).unwrap();
        
        writeln!(out, "### 2. Geographic Theaters (Masked IPs & Timezones)").unwrap();
        writeln!(out, "{}", subnets).unwrap();
        writeln!(out, "{}", timezones).unwrap();

        writeln!(out, "### 3. Traffic Origins (Referrers)").unwrap();
        writeln!(out, "{}", referrers).unwrap();

        writeln!(out, "### 4. Hardware Profiles (Processing & Memory)").unwrap();
        writeln!(out, "{}", cores).unwrap();
        writeln!(out, "{}", memory).unwrap();

        writeln!(out, "### 5. Viewport Topologies (Device Geometry)").unwrap();
        writeln!(out, "{}", viewports).unwrap();

        writeln!(out, "### 6. High-Intent Physical Actions (Clicks)").unwrap();
        writeln!(out, "{}", clicks_dist).unwrap();
        
        writeln!(out, "---\n").unwrap();
    }

    println!("Synthesizer complete. Ledger generated at: {}", report_path);
}
