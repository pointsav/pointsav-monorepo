use std::fs;
use std::process::Command;

const TRANSIENT_QUEUE: &str = "/opt/woodfine/cluster-totebox-personnel-1/service-slm/transient-queues";
const CONTENT_VAULT: &str = "/opt/woodfine/cluster-totebox-personnel-1/knowledge-graph";
const LLAMAFILE_BIN: &str = "/opt/woodfine/cluster-totebox-personnel-1/service-slm/vendor-micro-slm/llamafile";
const MODEL_WEIGHTS: &str = "/opt/woodfine/cluster-totebox-personnel-1/service-slm/vendor-micro-slm/weights/qwen2-0.5b.gguf";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================================");
    println!(" 🤖 NANO-SLM PROCESSOR (SUB-PROCESS EXECUTION ENGINE)");
    println!("========================================================");

    fs::create_dir_all(CONTENT_VAULT)?;
    
    let entries = fs::read_dir(TRANSIENT_QUEUE)?;
    let mut processed = 0;

    for entry in entries {
        let path = entry?.path();
        
        if path.is_file() && path.extension().unwrap_or_default() == "txt" {
            let filename = path.file_name().unwrap().to_string_lossy();
            let overlay_name = filename.replace("_payload.txt", "_overlay.md");
            let out_path = format!("{}/{}", CONTENT_VAULT, overlay_name);
            
            println!("[SYSTEM] Initiating Sub-Process Extraction on: {}", filename);
            let payload_content = fs::read_to_string(&path)?;
            
            // Frame the payload with the exact Qwen Chat Template for absolute instruction adherence
            let formatted_prompt = format!("<|im_start|>system\nYou are an Institutional Data Extraction AI. Extract the core facts from the text provided.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", payload_content);
            
            // Execute the AI directly on the CPU
            let output = Command::new("/bin/sh").arg(LLAMAFILE_BIN)
                .arg("-m").arg(MODEL_WEIGHTS)
                .arg("-p").arg(&formatted_prompt)
                .arg("-n").arg("256") // Strict token cap to prevent infinite loop
                .arg("-t").arg("1")   // 1 Thread for stability
                .arg("-c").arg("512").arg("-b").arg("64")// Context limit
                .arg("--temp").arg("0.1") // Cold, factual logic
                .arg("--log-disable").arg("--no-display-prompt") // Silence internal engine noise
                .output()?;
                
            if output.status.success() {
                let extracted_text = String::from_utf8_lossy(&output.stdout);
                // Strip out the prompt text from the final output for clean markdown
                let clean_text = extracted_text.split("<|im_start|>assistant\n").last().unwrap_or(&extracted_text).trim();
                
                fs::write(&out_path, clean_text)?;
                println!("   -> [SUCCESS] Staging Overlay forged: {}", overlay_name);
                
                // Erase the transient payload
                fs::remove_file(&path)?;
                processed += 1;
            } else {
                let err_text = String::from_utf8_lossy(&output.stderr);
                println!("   -> [FATAL] Engine crashed: {}", err_text);
            }
        }
    }
    
    println!("========================================================");
    println!("[SUCCESS] Cognitive Phase Complete. Processed {} payloads.", processed);
    Ok(())
}
