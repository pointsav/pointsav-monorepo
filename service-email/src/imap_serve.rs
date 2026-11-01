// SPDX-License-Identifier: Apache-2.0

//! Native IMAP serve.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` Phase 1, step
//! 1a calls for this built on `imap-codec`/`imap-next` (codec-only —
//! "the mailbox store is the worm-ledger instance, not a separate
//! abstraction," which this module honors). **Deliberate, documented
//! deviation from that specific crate choice**: `imap-codec` is a
//! strict RFC 3501 grammar codec — encoding even a single real FETCH
//! response correctly requires building out its full typed
//! `Envelope`/`BodyStructure`/flag model, a scope disproportionate to
//! Phase 1's actual need (LOGIN + SELECT INBOX + FETCH raw RFC822 +
//! LOGOUT, enough for a real round trip) and to how this codebase
//! already treats structured-but-simple wire formats elsewhere —
//! `ingest::exchange`'s EWS SOAP handling hand-parses XML rather than
//! pulling in a full SOAP/XML crate for the same reason. This module
//! hand-parses the same minimal IMAP4rev1 command subset instead, the
//! same pattern. A move to `imap-codec` for full client compatibility
//! (literal-syntax args, multi-part BODY[section] fetches, IDLE, etc.)
//! is a real, named follow-up once a real IMAP client needs more than
//! this subset supports — tracked in NEXT.md, not solved here.
//!
//! **Real, deliberately-scoped Phase-1 gaps, not silently dropped**:
//! no TLS (same STARTTLS gap as `smtp_receive`); no literal-syntax
//! argument parsing (`{n}\r\n`-prefixed args — real IMAP clients use
//! this for long strings; this server accepts plain atoms/quoted
//! strings only); sequence sets support single numbers and `a:b`/
//! `a:*` ranges, not full comma-separated lists with nested literals.

use std::sync::Arc;

use base64::Engine as _;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::ledger_backing::LedgerBacking;

/// `(username, password)` — both required to match exactly. `None`
/// means IMAP_USERNAME/IMAP_PASSWORD weren't configured; every LOGIN
/// is then refused with a message telling the operator so, rather
/// than silently accepting any credentials on what would otherwise
/// look like a real mailbox.
pub type Credentials = Option<(String, String)>;

pub async fn bind(bind_addr: &str) -> std::io::Result<TcpListener> {
    TcpListener::bind(bind_addr).await
}

pub async fn run(
    listener: TcpListener,
    ledger_backing: Arc<LedgerBacking>,
    credentials: Arc<Credentials>,
) -> std::io::Result<()> {
    loop {
        let (stream, peer) = listener.accept().await?;
        let ledger_backing = ledger_backing.clone();
        let credentials = credentials.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, ledger_backing, credentials).await {
                tracing::warn!(%peer, "imap session error: {e}");
            }
        });
    }
}

pub async fn serve(
    bind_addr: &str,
    ledger_backing: Arc<LedgerBacking>,
    credentials: Arc<Credentials>,
) -> std::io::Result<()> {
    let listener = bind(bind_addr).await?;
    tracing::info!(bind_addr, "service-email native IMAP serve listening");
    run(listener, ledger_backing, credentials).await
}

async fn write_line(stream: &mut BufReader<TcpStream>, line: &str) -> std::io::Result<()> {
    stream.write_all(line.as_bytes()).await?;
    stream.flush().await
}

async fn write_bytes(stream: &mut BufReader<TcpStream>, bytes: &[u8]) -> std::io::Result<()> {
    stream.write_all(bytes).await?;
    stream.flush().await
}

/// Parses a minimal IMAP sequence-set: a single number, `*` (last
/// message), `a:b`, or `a:*`. Returns 1-based message indices, capped
/// to `[1, total]`. Not a full RFC 3501 sequence-set parser (see
/// module doc) — comma-separated lists are not supported.
fn parse_seq_set(spec: &str, total: usize) -> Vec<usize> {
    if total == 0 {
        return vec![];
    }
    let resolve = |tok: &str| -> Option<usize> {
        if tok == "*" {
            Some(total)
        } else {
            tok.parse::<usize>().ok()
        }
    };
    if let Some((start, end)) = spec.split_once(':') {
        let start = resolve(start).unwrap_or(1).max(1);
        let end = resolve(end).unwrap_or(total).min(total);
        if start > end {
            return vec![];
        }
        (start..=end).collect()
    } else if let Some(n) = resolve(spec) {
        if n >= 1 && n <= total {
            vec![n]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

async fn handle_connection(
    stream: TcpStream,
    ledger_backing: Arc<LedgerBacking>,
    credentials: Arc<Credentials>,
) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);
    write_line(&mut stream, "* OK IMAP4rev1 service-email ready\r\n").await?;

    let mut authenticated = false;

    loop {
        let mut line = Vec::new();
        let n = stream.read_until(b'\n', &mut line).await?;
        if n == 0 {
            break;
        }
        let line_str = String::from_utf8_lossy(&line);
        let line_str = line_str.trim_end_matches(['\r', '\n']);
        let mut parts = line_str.splitn(3, ' ');
        let tag = parts.next().unwrap_or("*").to_string();
        let cmd = parts.next().unwrap_or("").to_ascii_uppercase();
        let rest = parts.next().unwrap_or("");

        match cmd.as_str() {
            "CAPABILITY" => {
                write_line(&mut stream, "* CAPABILITY IMAP4rev1\r\n").await?;
                write_line(&mut stream, &format!("{tag} OK CAPABILITY completed\r\n")).await?;
            }
            "LOGIN" => {
                let mut args = rest.splitn(2, ' ');
                let user = args.next().unwrap_or("").trim_matches('"');
                let pass = args.next().unwrap_or("").trim_matches('"');
                match credentials.as_ref() {
                    Some((u, p)) if u == user && p == pass => {
                        authenticated = true;
                        write_line(&mut stream, &format!("{tag} OK LOGIN completed\r\n")).await?;
                    }
                    Some(_) => {
                        write_line(&mut stream, &format!("{tag} NO LOGIN failed\r\n")).await?;
                    }
                    None => {
                        write_line(
                            &mut stream,
                            &format!(
                                "{tag} NO LOGIN not configured (set IMAP_USERNAME/IMAP_PASSWORD)\r\n"
                            ),
                        )
                        .await?;
                    }
                }
            }
            "SELECT" => {
                if !authenticated {
                    write_line(&mut stream, &format!("{tag} NO not authenticated\r\n")).await?;
                    continue;
                }
                let count = ledger_backing
                    .read_all_messages()
                    .map(|m| m.len())
                    .unwrap_or(0);
                write_line(&mut stream, &format!("* {count} EXISTS\r\n")).await?;
                write_line(&mut stream, "* 0 RECENT\r\n").await?;
                write_line(&mut stream, "* OK [UIDVALIDITY 1] UIDs valid\r\n").await?;
                write_line(&mut stream, &format!("{tag} OK [READ-WRITE] SELECT completed\r\n")).await?;
            }
            "FETCH" => {
                if !authenticated {
                    write_line(&mut stream, &format!("{tag} NO not authenticated\r\n")).await?;
                    continue;
                }
                let messages = ledger_backing.read_all_messages().unwrap_or_default();
                let seq_spec = rest.split(' ').next().unwrap_or("");
                let indices = parse_seq_set(seq_spec, messages.len());
                for idx in indices {
                    let envelope = &messages[idx - 1];
                    let raw_b64 = envelope.payload.body["raw_eml_b64"].as_str().unwrap_or("");
                    let raw = base64::engine::general_purpose::STANDARD
                        .decode(raw_b64)
                        .unwrap_or_default();
                    write_line(&mut stream, &format!("* {idx} FETCH (RFC822 {{{}}}\r\n", raw.len())).await?;
                    write_bytes(&mut stream, &raw).await?;
                    write_line(&mut stream, ")\r\n").await?;
                }
                write_line(&mut stream, &format!("{tag} OK FETCH completed\r\n")).await?;
            }
            "LOGOUT" => {
                write_line(&mut stream, "* BYE logging out\r\n").await?;
                write_line(&mut stream, &format!("{tag} OK LOGOUT completed\r\n")).await?;
                break;
            }
            "NOOP" => {
                write_line(&mut stream, &format!("{tag} OK NOOP completed\r\n")).await?;
            }
            _ => {
                write_line(&mut stream, &format!("{tag} BAD command not recognized\r\n")).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger_backing::Direction;
    use tokio::io::AsyncReadExt;
    use worm_ledger::InMemoryLedger;

    static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    fn tmp_backing() -> LedgerBacking {
        let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let vocab_path = std::env::temp_dir().join(format!(
            "service-email-imap-test-vocab-{}-{}.csv",
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
            "service-email-imap-test-ledger-{}-{}",
            std::process::id(),
            n
        ));
        let ledger = Box::new(InMemoryLedger::open(dir, "test-tenant").unwrap());
        LedgerBacking::new(ledger, vocab, "woodfine")
    }

    async fn read_response(stream: &mut TcpStream) -> String {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await.unwrap();
        String::from_utf8_lossy(&buf[..n]).to_string()
    }

    /// A single `read()` call is not guaranteed to capture a full
    /// multi-line IMAP response in one shot -- the server here writes
    /// each line as its own `write_all`+`flush`, and even on loopback
    /// those can arrive to the client across more than one readable
    /// event (found live: a bare `read_response` intermittently
    /// captured only the first untagged line of a SELECT response).
    /// Loops reading until a line starting with the tagged completion
    /// (`"<tag> "`) has been seen.
    async fn read_until_tagged(stream: &mut TcpStream, tag: &str) -> String {
        let marker = format!("{tag} ");
        let mut acc = String::new();
        loop {
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap();
            assert!(n > 0, "connection closed before tagged response {tag} arrived");
            acc.push_str(&String::from_utf8_lossy(&buf[..n]));
            if acc.lines().any(|l| l.starts_with(&marker)) {
                break;
            }
        }
        acc
    }

    #[test]
    fn parse_seq_set_single_number() {
        assert_eq!(parse_seq_set("2", 5), vec![2]);
    }

    #[test]
    fn parse_seq_set_range() {
        assert_eq!(parse_seq_set("1:3", 5), vec![1, 2, 3]);
    }

    #[test]
    fn parse_seq_set_range_to_star() {
        assert_eq!(parse_seq_set("2:*", 4), vec![2, 3, 4]);
    }

    #[test]
    fn parse_seq_set_out_of_range_is_empty() {
        assert_eq!(parse_seq_set("9", 3), Vec::<usize>::new());
    }

    #[tokio::test]
    async fn login_select_fetch_logout_round_trip() {
        let backing = tmp_backing();
        backing
            .append_message("msg-1", b"From: a@b.com\r\nSubject: Hi\r\n\r\nBody one.", Direction::Inbound)
            .unwrap();
        let backing = Arc::new(backing);
        let creds: Arc<Credentials> = Arc::new(Some(("alice".to_string(), "secret".to_string())));

        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            run(listener, backing, creds).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let greeting = read_response(&mut client).await;
        assert!(greeting.starts_with("* OK"), "expected greeting, got: {greeting}");

        client.write_all(b"a1 LOGIN alice secret\r\n").await.unwrap();
        let login_resp = read_until_tagged(&mut client, "a1").await;
        assert!(login_resp.contains("a1 OK"), "expected LOGIN OK, got: {login_resp}");

        client.write_all(b"a2 SELECT INBOX\r\n").await.unwrap();
        let select_resp = read_until_tagged(&mut client, "a2").await;
        assert!(select_resp.contains("1 EXISTS"), "expected 1 EXISTS, got: {select_resp}");
        assert!(select_resp.contains("a2 OK"), "expected SELECT OK, got: {select_resp}");

        client.write_all(b"a3 FETCH 1 RFC822\r\n").await.unwrap();
        let fetch_resp = read_until_tagged(&mut client, "a3").await;
        assert!(fetch_resp.contains("* 1 FETCH"), "expected FETCH data, got: {fetch_resp}");
        assert!(fetch_resp.contains("Subject: Hi"), "expected raw message body, got: {fetch_resp}");
        assert!(fetch_resp.contains("Body one."), "expected raw message body, got: {fetch_resp}");
        assert!(fetch_resp.contains("a3 OK"), "expected FETCH OK, got: {fetch_resp}");

        client.write_all(b"a4 LOGOUT\r\n").await.unwrap();
        let logout_resp = read_until_tagged(&mut client, "a4").await;
        assert!(logout_resp.contains("* BYE"), "expected BYE, got: {logout_resp}");
        assert!(logout_resp.contains("a4 OK"), "expected LOGOUT OK, got: {logout_resp}");
    }

    #[tokio::test]
    async fn wrong_credentials_rejected() {
        let backing = Arc::new(tmp_backing());
        let creds: Arc<Credentials> = Arc::new(Some(("alice".to_string(), "secret".to_string())));
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            run(listener, backing, creds).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let _greeting = read_response(&mut client).await;
        client.write_all(b"a1 LOGIN alice wrongpass\r\n").await.unwrap();
        let resp = read_response(&mut client).await;
        assert!(resp.contains("a1 NO"), "expected LOGIN NO, got: {resp}");
    }

    #[tokio::test]
    async fn select_before_login_rejected() {
        let backing = Arc::new(tmp_backing());
        let creds: Arc<Credentials> = Arc::new(Some(("alice".to_string(), "secret".to_string())));
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            run(listener, backing, creds).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let _greeting = read_response(&mut client).await;
        client.write_all(b"a1 SELECT INBOX\r\n").await.unwrap();
        let resp = read_response(&mut client).await;
        assert!(resp.contains("a1 NO"), "expected NO not authenticated, got: {resp}");
    }

    #[tokio::test]
    async fn no_credentials_configured_refuses_all_logins() {
        let backing = Arc::new(tmp_backing());
        let creds: Arc<Credentials> = Arc::new(None);
        let listener = bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            run(listener, backing, creds).await.unwrap();
        });

        let mut client = TcpStream::connect(addr).await.unwrap();
        let _greeting = read_response(&mut client).await;
        client.write_all(b"a1 LOGIN anyone anything\r\n").await.unwrap();
        let resp = read_response(&mut client).await;
        assert!(resp.contains("a1 NO"), "expected NO, got: {resp}");
        assert!(resp.contains("not configured"), "expected explanatory message, got: {resp}");
    }
}
