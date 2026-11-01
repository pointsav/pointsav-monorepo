// SPDX-License-Identifier: Apache-2.0

//! Native SMTP receive server.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` Phase 1, step
//! 1a: "SMTP receive server built natively on smtp-proto + a new
//! session/queue layer over tokio/rustls." `smtp-proto` is a line
//! parser only (confirmed against its own source — the crate itself
//! says "not yet documented"); the session state machine below (line
//! buffering, MAIL/RCPT/DATA sequencing, dot-unstuffing) is this
//! crate's own, matching the "layer on top of a parser" shape every
//! other real SMTP/IMAP server built on these Stalwart Labs crates
//! uses.
//!
//! **Real, deliberately-scoped Phase-1 gap, not silently dropped**:
//! this server speaks plaintext SMTP only — no STARTTLS/implicit-TLS
//! (the `/rustls` half of "tokio/rustls" in the BRIEF's own wording).
//! Wiring TLS in is a follow-up once there's a real deployment that
//! needs it (this workspace's own "don't build ahead of demand"
//! convention) — tracked in NEXT.md, not solved here. No AUTH either:
//! this is a trusted-network intake surface (matching how the rest of
//! Ring 1 already operates — `X-Foundry-Module-ID` is the boundary,
//! not per-connection auth), not an internet-facing MX.
//!
//! **Also deferred, real not silent**: the message_id used for the
//! ledger record is a freshly generated UUID, not the message's own
//! `Message-ID:` header (MIME parsing via `mail-parser`, named in the
//! BRIEF for this exact purpose, isn't wired into the receive path
//! yet) — the raw EML body still carries the real header verbatim,
//! nothing is lost, just not yet surfaced as ledger metadata.

use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::ledger_backing::{Direction, LedgerBacking};

/// Bind the listening socket. Split from `run` so a caller (or a
/// test) can learn the actual bound address before the accept loop
/// starts — needed when binding to port 0 for a test server.
pub async fn bind(bind_addr: &str) -> std::io::Result<TcpListener> {
    TcpListener::bind(bind_addr).await
}

/// Accept loop — one spawned task per connection, all sharing the one
/// `LedgerBacking` instance (same pattern as the MCP server / EWS
/// poll loop in `main.rs`, all three write to the same ledger).
pub async fn run(
    listener: TcpListener,
    ledger_backing: Arc<LedgerBacking>,
    hostname: Arc<str>,
) -> std::io::Result<()> {
    loop {
        let (stream, peer) = listener.accept().await?;
        let ledger_backing = ledger_backing.clone();
        let hostname = hostname.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, ledger_backing, hostname).await {
                tracing::warn!(%peer, "smtp session error: {e}");
            }
        });
    }
}

/// Convenience wrapper combining `bind` + `run` for `main.rs`.
pub async fn serve(
    bind_addr: &str,
    ledger_backing: Arc<LedgerBacking>,
    hostname: Arc<str>,
) -> std::io::Result<()> {
    let listener = bind(bind_addr).await?;
    tracing::info!(bind_addr, "service-email native SMTP receive server listening");
    run(listener, ledger_backing, hostname).await
}

async fn write_line(stream: &mut BufReader<TcpStream>, line: &str) -> std::io::Result<()> {
    stream.write_all(line.as_bytes()).await?;
    stream.flush().await
}

async fn handle_connection(
    stream: TcpStream,
    ledger_backing: Arc<LedgerBacking>,
    hostname: Arc<str>,
) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);
    write_line(&mut stream, &format!("220 {hostname} service-email ESMTP\r\n")).await?;

    let mut mail_from: Option<String> = None;
    let mut rcpt_to: Vec<String> = Vec::new();

    loop {
        let mut line = Vec::new();
        let n = stream.read_until(b'\n', &mut line).await?;
        if n == 0 {
            break; // peer closed the connection
        }

        match smtp_proto::Request::parse(&mut line.iter()) {
            Ok(smtp_proto::Request::Ehlo { .. }) | Ok(smtp_proto::Request::Helo { .. }) => {
                write_line(&mut stream, &format!("250 {hostname}\r\n")).await?;
            }
            Ok(smtp_proto::Request::Mail { from }) => {
                mail_from = Some(from.address.to_string());
                write_line(&mut stream, "250 2.1.0 OK\r\n").await?;
            }
            Ok(smtp_proto::Request::Rcpt { to }) => {
                rcpt_to.push(to.address.to_string());
                write_line(&mut stream, "250 2.1.5 OK\r\n").await?;
            }
            Ok(smtp_proto::Request::Data) => {
                if mail_from.is_none() || rcpt_to.is_empty() {
                    write_line(&mut stream, "503 5.5.1 bad sequence of commands\r\n").await?;
                    continue;
                }
                write_line(&mut stream, "354 start mail input; end with <CRLF>.<CRLF>\r\n").await?;
                let body = read_data_body(&mut stream).await?;
                let message_id = uuid::Uuid::new_v4().to_string();
                match ledger_backing.append_message(&message_id, &body, Direction::Inbound) {
                    Ok(cursor) => {
                        tracing::debug!(message_id, cursor, "smtp receive: appended to ledger");
                        write_line(&mut stream, &format!("250 2.0.0 OK: queued as {message_id}\r\n")).await?;
                    }
                    Err(e) => {
                        tracing::error!("smtp receive: ledger append failed: {e}");
                        write_line(&mut stream, "451 4.3.0 requested action aborted\r\n").await?;
                    }
                }
                mail_from = None;
                rcpt_to.clear();
            }
            Ok(smtp_proto::Request::Rset) => {
                mail_from = None;
                rcpt_to.clear();
                write_line(&mut stream, "250 2.0.0 OK\r\n").await?;
            }
            Ok(smtp_proto::Request::Quit) => {
                write_line(&mut stream, "221 2.0.0 Bye\r\n").await?;
                break;
            }
            Ok(smtp_proto::Request::Noop { .. }) => {
                write_line(&mut stream, "250 2.0.0 OK\r\n").await?;
            }
            Ok(_other) => {
                write_line(&mut stream, "502 5.5.1 command not implemented\r\n").await?;
            }
            Err(_e) => {
                write_line(&mut stream, "500 5.5.2 syntax error\r\n").await?;
            }
        }
    }

    Ok(())
}

/// Reads the DATA body up to (and consuming) the `<CRLF>.<CRLF>`
/// terminator, undoing RFC 5321 §4.5.2 leading-dot transparency
/// stuffing along the way. Returns the raw message bytes without the
/// terminator line.
async fn read_data_body(stream: &mut BufReader<TcpStream>) -> std::io::Result<Vec<u8>> {
    let mut body = Vec::new();
    loop {
        let mut line = Vec::new();
        let n = stream.read_until(b'\n', &mut line).await?;
        if n == 0 {
            // Peer closed mid-body -- treat what we have as the message
            // rather than hanging forever waiting for a terminator that
            // will never arrive.
            break;
        }
        if line == b".\r\n" || line == b".\n" {
            break;
        }
        if line.starts_with(b"..") {
            body.extend_from_slice(&line[1..]);
        } else {
            body.extend_from_slice(&line);
        }
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let vocab_path = std::env::temp_dir().join(format!(
            "service-email-smtp-recv-test-vocab-{}-{}.csv",
            std::process::id(),
            n
        ));
        std::fs::write(
            &vocab_path,
            "class,duration_days,regime,notes\nemail-message,,test,test\n",
        )
        .unwrap();
        let vocab = worm_ledger::retention::load_retention_classes(vocab_path.to_str().unwrap());
        let dir = std::env::temp_dir().join(format!(
            "service-email-smtp-recv-test-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, vocab, "woodfine")
    }

    async fn read_response(stream: &mut TcpStream) -> String {
        let mut buf = [0u8; 512];
        let n = stream.read(&mut buf).await.unwrap();
        String::from_utf8_lossy(&buf[..n]).to_string()
    }

    #[tokio::test]
    async fn full_session_stores_message_in_ledger() {
        let backing = Arc::new(tmp_backing());
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let backing_for_server = backing.clone();
        tokio::spawn(async move {
            run(listener, backing_for_server, Arc::from("test.local")).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let greeting = read_response(&mut client).await;
        assert!(greeting.starts_with("220 "), "expected greeting, got: {greeting}");

        client.write_all(b"EHLO client.example.com\r\n").await.unwrap();
        assert!(read_response(&mut client).await.starts_with("250"));

        client.write_all(b"MAIL FROM:<alice@example.com>\r\n").await.unwrap();
        assert!(read_response(&mut client).await.starts_with("250"));

        client.write_all(b"RCPT TO:<bob@example.com>\r\n").await.unwrap();
        assert!(read_response(&mut client).await.starts_with("250"));

        client.write_all(b"DATA\r\n").await.unwrap();
        let data_resp = read_response(&mut client).await;
        assert!(data_resp.starts_with("354"), "expected 354, got: {data_resp}");

        client
            .write_all(b"From: alice@example.com\r\nTo: bob@example.com\r\nSubject: Hi\r\n\r\nHello, Bob.\r\n.\r\n")
            .await
            .unwrap();
        let queued_resp = read_response(&mut client).await;
        assert!(queued_resp.starts_with("250"), "expected 250 queued, got: {queued_resp}");

        client.write_all(b"QUIT\r\n").await.unwrap();
        let bye = read_response(&mut client).await;
        assert!(bye.starts_with("221"), "expected 221 Bye, got: {bye}");

        let messages = backing.read_all_messages().unwrap();
        assert_eq!(messages.len(), 1);
        let raw_b64 = messages[0].payload.body["raw_eml_b64"].as_str().unwrap();
        let raw = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, raw_b64).unwrap();
        let raw_str = String::from_utf8(raw).unwrap();
        assert!(raw_str.contains("Subject: Hi"));
        assert!(raw_str.contains("Hello, Bob."));
    }

    #[tokio::test]
    async fn data_before_rcpt_is_rejected() {
        let backing = Arc::new(tmp_backing());
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            run(listener, backing, Arc::from("test.local")).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let _greeting = read_response(&mut client).await;

        client.write_all(b"DATA\r\n").await.unwrap();
        let resp = read_response(&mut client).await;
        assert!(resp.starts_with("503"), "expected 503 bad sequence, got: {resp}");
    }

    #[tokio::test]
    async fn dot_stuffed_line_is_unstuffed() {
        let backing = Arc::new(tmp_backing());
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let backing_for_server = backing.clone();
        tokio::spawn(async move {
            run(listener, backing_for_server, Arc::from("test.local")).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let _greeting = read_response(&mut client).await;
        client.write_all(b"EHLO c\r\n").await.unwrap();
        let _ = read_response(&mut client).await;
        client.write_all(b"MAIL FROM:<a@b.com>\r\n").await.unwrap();
        let _ = read_response(&mut client).await;
        client.write_all(b"RCPT TO:<c@d.com>\r\n").await.unwrap();
        let _ = read_response(&mut client).await;
        client.write_all(b"DATA\r\n").await.unwrap();
        let _ = read_response(&mut client).await;
        // A body line starting with ".." must be unstuffed to a single
        // leading "." (RFC 5321 4.5.2) rather than terminating early.
        client
            .write_all(b"Subject: dot test\r\n\r\n..this line starts with a literal dot\r\n.\r\n")
            .await
            .unwrap();
        let _ = read_response(&mut client).await;
        client.write_all(b"QUIT\r\n").await.unwrap();
        let _ = read_response(&mut client).await;

        let messages = backing.read_all_messages().unwrap();
        assert_eq!(messages.len(), 1);
        let raw_b64 = messages[0].payload.body["raw_eml_b64"].as_str().unwrap();
        let raw = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, raw_b64).unwrap();
        let raw_str = String::from_utf8(raw).unwrap();
        assert!(
            raw_str.contains(".this line starts with a literal dot"),
            "expected unstuffed line, got: {raw_str:?}"
        );
    }
}
