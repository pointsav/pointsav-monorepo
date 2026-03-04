//! Ingestion and Orchestration Logic for the Synthesis Engines.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use crate::payload::{PayloadBuilder, ContextSnippet};
use crate::verification::verify_artifact;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtocolManifest {
    pub protocol_name: String,
    pub capability_requests: Vec<String>,
    pub collision_priority: HashMap<u8, String>,
    pub operational_rules: OperationalRules,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OperationalRules {
    pub capex_posture: Option<bool>,
    pub plain_language: Option<bool>,
    pub structural_headers: Option<bool>,
    pub factuality_enforcement: Option<bool>,
    pub anti_puffery_verbs: Option<bool>,
    pub entity_precision: Option<bool>,
    pub jurisdictional_agnosticism: Option<bool>,
    pub trademark_first_use: Option<bool>,
    pub banned_buzzwords: Option<Vec<String>>,
}

pub trait SynthesisEngine {
    fn ingest_protocol(&mut self, path: &str) -> Result<(), String>;
    fn execute_synthesis(&self, theme: &str, raw_data: Vec<ContextSnippet>) -> Result<String, String>;
}

pub struct MemoEngine {
    pub manifest: Option<ProtocolManifest>,
}

impl MemoEngine {
    pub fn new() -> Self {
        Self { manifest: None }
    }
}

impl SynthesisEngine for MemoEngine {
    fn ingest_protocol(&mut self, _path: &str) -> Result<(), String> {
        Ok(())
    }

    fn execute_synthesis(&self, theme: &str, raw_data: Vec<ContextSnippet>) -> Result<String, String> {
        let manifest = self.manifest.as_ref().ok_or("Protocol Manifest not loaded. Engine halted.")?;

        let mut builder = PayloadBuilder::new(&manifest.operational_rules, theme);
        for snippet in raw_data {
            builder.add_context(snippet);
        }
        let prompt = builder.build();

        // Secure API Bridge (Gemini)
        let api_key = env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY environment variable not set. Secure outbound capability denied.")?;
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);

        let request_body = serde_json::json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });

        println!("[API BRIDGE] Transmitting payload to Linguistic Compiler...");
        
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .map_err(|e| format!("Failed to build network client: {}", e))?;

        let response = client.post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| format!("Network transmission failed: {}", e))?;

        // TELEMETRY INJECTED: Capture and print the exact Google JSON error
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "No detailed error body provided by Google.".to_string());
            return Err(format!("API rejected the payload. Status: {}\n[GOOGLE DIAGNOSTIC REPORT]:\n{}", status, error_text));
        }

        let response_json: serde_json::Value = response.json().map_err(|_| "Failed to parse API response.")?;
        let generated_text = response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Error: Empty response from compiler.")
            .to_string();

        println!("[VERIFICATION] Executing regex safety net against generated artifact...");
        let audit = verify_artifact(&generated_text, &manifest.operational_rules);

        if audit.is_compliant {
            Ok(generated_text)
        } else {
            Err(format!("Compliance Audit Failed. Violations: {:?}", audit.violations))
        }
    }
}
