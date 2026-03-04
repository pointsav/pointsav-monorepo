//! Data Ingestion and Parsing for Fleet Wikis.

use crate::payload::ContextSnippet;
use std::error::Error;

pub fn parse_glossary_csv(csv_data: &str, source_id: &str) -> Result<Vec<ContextSnippet>, Box<dyn Error>> {
    // We set has_headers(false) and flexible(true) to handle raw data geometries 
    // without panicking on mismatched header names or empty trailing columns.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(csv_data.as_bytes());
        
    let mut snippets = Vec::new();

    for (line_number, result) in reader.records().enumerate() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                eprintln!("[PARSER WARNING] Dropping Row {}: {}", line_number + 1, e);
                continue;
            }
        };

        // Safely extract the columns by physical index
        let term = record.get(0).unwrap_or("").trim();
        let translation = record.get(1).unwrap_or("").trim();
        let definition = record.get(2).unwrap_or("").trim();

        // Auto-detect and skip the header row if it exists
        if line_number == 0 && (term.eq_ignore_ascii_case("term") || term.eq_ignore_ascii_case("word")) {
            continue;
        }

        // Enforce data integrity: skip empty terms or definitions
        if term.is_empty() || definition.is_empty() {
            continue;
        }

        // Format the output for the Linguistic Compiler
        let content = if translation.is_empty() {
            format!("{}: {}", term, definition)
        } else {
            format!("{} (ES: {}): {}", term, translation, definition)
        };
        
        snippets.push(ContextSnippet {
            source_id: source_id.to_string(),
            content,
            tags: vec!["glossary".to_string(), "nomenclature_lock".to_string()],
        });
    }

    if snippets.is_empty() {
        eprintln!("[PARSER CRITICAL] 0 records parsed. The data substrate is completely empty.");
    } else {
        println!("[PARSER SUCCESS] Successfully ingested {} records.", snippets.len());
    }

    Ok(snippets)
}
