use warp::Filter;
use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let ws_route = warp::path("terminal")
        .and(warp::path("ws"))
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_connection)
        });

    println!("[SYSTEM] PointSav Mesh CLI listening for BINARY FRAMES on 0.0.0.0:8088");
    warp::serve(ws_route).run(([0, 0, 0, 0], 8088)).await;
}

async fn handle_connection(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    
    let cyan = "\x1b[1;36m";
    let reset = "\x1b[0m";
    let prompt = format!("\r\n{}pwoodfine@woodfine-ppn{}:~$ ", cyan, reset);

    let boot_msg = format!(
        "\r\n[SYSTEM] Machine Based Authorization: SUCCESS.\r\n[SYSTEM] Welcome to the PointSav Private Network (PPN).\r\n[SYSTEM] Genesis Node (N=1) Sovereign Matrix is LIVE.{}", 
        prompt
    );
    
    if tx.send(Message::binary(boot_msg.into_bytes())).await.is_err() { return; }

    // THE STATEFUL BUFFER
    let mut input_buffer = String::new();

    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => break,
        };

        let bytes = if msg.is_binary() {
            msg.as_bytes()
        } else if msg.is_text() {
            msg.as_bytes()
        } else {
            continue;
        };

        for &b in bytes {
            match b {
                // Enter / Return Key (\r or \n)
                13 | 10 => {
                    let command = input_buffer.trim().to_string();
                    input_buffer.clear(); // Reset the buffer

                    let response = match command.as_str() {
                        "ppn telemetry --status" => "\r\n[TELEMETRY] Cryptographic Mesh: SECURE.\r\n[TELEMETRY] Active Genesis Nodes: 1\r\n[TELEMETRY] Ledger Synchronization: PARITY.",
                        "ppn query service-content --all" => "\r\n[LEDGER] Querying Sovereign Data Store...\r\n[LEDGER] Assets found: 0. Matrix is pristine.",
                        "whoami" => "\r\n[AUTH] Fiduciary Key: pwoodfine (Executive Officer)",
                        "" => "", // Handle empty enter key cleanly
                        _ => "\r\n[FAULT] Unrecognized Mesh Command. Try: 'ppn telemetry --status'",
                    };

                    let reply = format!("{}{}", response, prompt);
                    if tx.send(Message::binary(reply.into_bytes())).await.is_err() { return; }
                },
                // Backspace Key (\x7f or \x08)
                127 | 8 => {
                    if !input_buffer.is_empty() {
                        input_buffer.pop();
                        // Send ANSI sequence to visually delete the character on the screen:
                        // Move cursor back (\x08), overwrite with space, move cursor back again
                        let backspace_seq = "\x08 \x08".as_bytes();
                        if tx.send(Message::binary(backspace_seq.to_vec())).await.is_err() { return; }
                    }
                },
                // Standard Printable Characters
                32..=126 => {
                    let c = b as char;
                    input_buffer.push(c);
                    // ECHO the character back to the UI so the user sees it
                    if tx.send(Message::binary(vec![b])).await.is_err() { return; }
                },
                // Ignore other unhandled control codes
                _ => {}
            }
        }
    }
}
