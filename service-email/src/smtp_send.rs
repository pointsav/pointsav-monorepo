// SPDX-License-Identifier: Apache-2.0

//! Outbound SMTP send.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` Phase 1, step
//! 1a: "SMTP send via mail-send/lettre, MIME via mail-parser... message
//! construction via mail-builder." Wraps `mail_send::SmtpClientBuilder`
//! (mail-send, Apache-2.0/MIT, Stalwart Labs) + `mail_builder::
//! MessageBuilder`. Every sent message is written to this service's
//! own ledger as an `email.message` Envelope with `direction:
//! "outbound"` (decision 8) — the same record either way an operator
//! or `service-minutebook` later queries email history.
//!
//! **Real, deliberately-scoped Phase-1 gaps, not silently dropped**:
//! no DKIM signing (`mail-auth`'s `send_signed` path is a named later-
//! phase item, not wired here) and no STARTTLS negotiation —
//! `connect_plain()` is used, appropriate for the trusted-network
//! relay case this crate's own tests exercise, not yet for sending to
//! an arbitrary external MX that requires TLS. Both are tracked in
//! NEXT.md.

// Use mail-send's own re-export of mail-builder, not a separate direct
// dependency -- mail-send's `IntoMessage for MessageBuilder` impl is
// only satisfied by the exact `mail_builder` version mail-send itself
// depends on; a second, independently-versioned `mail-builder` direct
// dependency produces two distinct `MessageBuilder` types that don't
// implement each other's traits (found live: E0277 "the trait bound
// ... is not satisfied ... multiple different versions of crate
// `mail_builder` in the dependency graph").
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

use crate::ledger_backing::{Direction, LedgerBacking, LedgerBackingError};

#[derive(Debug)]
pub enum SendError {
    Build(std::io::Error),
    ClientBuild(String),
    Smtp(mail_send::Error),
    Ledger(LedgerBackingError),
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SendError::Build(e) => write!(f, "message build error: {e}"),
            SendError::ClientBuild(e) => write!(f, "smtp client build error: {e}"),
            SendError::Smtp(e) => write!(f, "smtp error: {e}"),
            SendError::Ledger(e) => write!(f, "ledger error: {e}"),
        }
    }
}

impl std::error::Error for SendError {}

pub struct OutboundMessage<'a> {
    pub from: &'a str,
    pub to: Vec<&'a str>,
    pub subject: &'a str,
    pub text_body: &'a str,
}

/// Sends `msg` via plaintext SMTP to `host:port` (no STARTTLS — see
/// module doc), then appends the exact bytes that were sent to this
/// service's own ledger as an outbound `email.message` Envelope.
/// Returns the assigned ledger cursor.
pub async fn send_plain(
    host: &str,
    port: u16,
    msg: OutboundMessage<'_>,
    ledger_backing: &LedgerBacking,
) -> Result<u64, SendError> {
    let mut builder = MessageBuilder::new().from(msg.from).subject(msg.subject);
    for to in &msg.to {
        builder = builder.to(*to);
    }
    builder = builder.text_body(msg.text_body.to_string());

    let raw = builder.clone().write_to_vec().map_err(SendError::Build)?;

    // `SmtpClientBuilder::new` constructs a TLS connector eagerly even
    // for a later `connect_plain()` call, and panics at that point if
    // no process-level rustls `CryptoProvider` has been installed yet
    // (found live: "Could not automatically determine the process-
    // level CryptoProvider... exactly one of 'aws-lc-rs' and 'ring'" --
    // this crate's dependency graph carries both as available
    // backends, so rustls refuses to guess). Installing twice is a
    // safe no-op (`install_default` erroring on a second call is
    // expected and ignored here).
    let _ = rustls::crypto::ring::default_provider().install_default();

    let client_builder =
        SmtpClientBuilder::new(host, port).map_err(SendError::ClientBuild)?;
    let mut client = client_builder.connect_plain().await.map_err(SendError::Smtp)?;
    client.send(builder).await.map_err(SendError::Smtp)?;

    let message_id = uuid::Uuid::new_v4().to_string();
    ledger_backing
        .append_message(&message_id, &raw, Direction::Outbound)
        .map_err(SendError::Ledger)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smtp_receive;
    use std::sync::Arc;
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let vocab_path = std::env::temp_dir().join(format!(
            "service-email-smtp-send-test-vocab-{}-{}.csv",
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
            "service-email-smtp-send-test-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, vocab, "woodfine")
    }

    /// The real Phase-1 round trip: send via `send_plain` against this
    /// crate's own native SMTP receive server (`smtp_receive`),
    /// running on loopback — no external network or mocked SMTP
    /// server involved on either side.
    #[tokio::test]
    async fn send_plain_round_trips_through_native_receive_server() {
        let receiver_backing = Arc::new(tmp_backing());
        let listener = smtp_receive::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let receiver_backing_for_server = receiver_backing.clone();
        tokio::spawn(async move {
            smtp_receive::run(listener, receiver_backing_for_server, Arc::from("test.local"))
                .await
                .unwrap();
        });

        let sender_backing = tmp_backing();
        let cursor = send_plain(
            "127.0.0.1",
            addr.port(),
            OutboundMessage {
                from: "sender@example.com",
                to: vec!["recipient@example.com"],
                subject: "Round trip test",
                text_body: "This message travelled through the native SMTP receive server.",
            },
            &sender_backing,
        )
        .await
        .unwrap();
        assert_eq!(cursor, 1, "sender's own ledger gets the outbound record");

        // The sender's ledger has the outbound copy.
        let sent = sender_backing.read_all_messages().unwrap();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].payload.body["direction"], "outbound");

        // The receiver's ledger has the inbound copy, delivered over
        // real SMTP by mail-send talking to our own smtp_receive server.
        // Give the spawned server task a moment to finish processing
        // the just-closed connection before asserting.
        for _ in 0..50 {
            if !receiver_backing.read_all_messages().unwrap().is_empty() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        let received = receiver_backing.read_all_messages().unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].payload.body["direction"], "inbound");
        let raw_b64 = received[0].payload.body["raw_eml_b64"].as_str().unwrap();
        let raw = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, raw_b64).unwrap();
        let raw_str = String::from_utf8(raw).unwrap();
        assert!(raw_str.contains("Round trip test"));
        assert!(raw_str.contains("This message travelled through the native SMTP receive server."));
    }
}
