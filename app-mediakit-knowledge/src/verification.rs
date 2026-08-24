// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Continuous citation verification (Phase 3.4 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! A background task re-fetches every cited external URL on an interval,
//! re-hashes the body, and records drift (a changed hash since the last
//! check) via `ClaimStore`. Drift is surfaced in the article chrome's
//! citation ribbon (`ui::layout`) by reading `ClaimStore::drifted_citations`.
//!
//! This module never blocks a request — it runs entirely on its own
//! `tokio::spawn`'d task, started once at `serve()` startup.

use std::net::IpAddr;
use std::time::Duration;

use futures_util::StreamExt;

use crate::citations::CitationRegistry;
use crate::claims_store::{CitationVerification, ClaimStore, VerificationStatus};

/// How often the scheduler sweeps every cited citation. A wiki's cited
/// sources change slowly; checking more than once a day is unnecessary load
/// on the sources themselves.
pub const VERIFY_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);

/// Per-fetch timeout — a slow/hanging source must not stall the whole sweep.
/// This is a TOTAL request timeout (reqwest semantics), so it also bounds
/// (though doesn't replace) the body-size cap below.
const FETCH_TIMEOUT: Duration = Duration::from_secs(15);

/// Hard cap on a citation response body. Citations are prose/reference pages,
/// not media — 10 MB is generous headroom while still bounding worst-case
/// memory use in the shared server process (this scheduler runs in the same
/// process as the HTTP server).
const MAX_BODY_BYTES: usize = 10 * 1024 * 1024;

/// Spawn the background re-verification loop. Fire-and-forget: the returned
/// `JoinHandle` is intentionally not awaited by the caller (the task runs
/// for the service's lifetime); callers that want graceful shutdown can
/// abort it explicitly via the handle.
pub fn spawn_scheduler(
    store: ClaimStore,
    registry: CitationRegistry,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        // Redirects disabled deliberately (SSRF hardening): a citation URL
        // that later starts 30x-redirecting to an internal/metadata host
        // must not be silently followed. Combined with the pre-fetch
        // is_safe_url() DNS check below, this closes the redirect-driven
        // SSRF gap a default reqwest client (10 auto-follows) would have.
        let client = match reqwest::Client::builder()
            .timeout(FETCH_TIMEOUT)
            .redirect(reqwest::redirect::Policy::none())
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("citation verification scheduler could not build HTTP client: {e}; scheduler disabled");
                return;
            }
        };
        let mut interval = tokio::time::interval(VERIFY_INTERVAL);
        // The first tick fires immediately; skip it so we don't hammer every
        // cited source at process start on every restart.
        interval.tick().await;
        loop {
            interval.tick().await;
            run_sweep(&client, &store, &registry).await;
        }
    })
}

/// Run one verification sweep now (also used directly by tests and by a
/// future on-demand admin trigger, without waiting for the interval).
pub async fn run_sweep(client: &reqwest::Client, store: &ClaimStore, registry: &CitationRegistry) {
    let cited_ids = match store.all_cited_citation_ids() {
        Ok(ids) => ids,
        Err(e) => {
            tracing::error!("citation verification sweep: could not list cited citation ids: {e}");
            return;
        }
    };
    tracing::info!(
        "citation verification sweep: checking {} cited source(s)",
        cited_ids.len()
    );
    for citation_id in cited_ids {
        let Some(entry) = registry.get(&citation_id) else {
            // Cited by a claim but not (or no longer) in citations.yaml —
            // an editorial-linter concern (convention §9), not this
            // scheduler's to fix; skip.
            continue;
        };
        let verification = verify_one(client, &citation_id, &entry.url, store).await;
        if let Err(e) = store.record_verification(&verification) {
            tracing::error!(
                "citation verification sweep: failed to record result for {citation_id}: {e}"
            );
        }
        if verification.status == VerificationStatus::Drifted {
            tracing::warn!("citation drift detected: {citation_id} ({})", entry.url);
        }
    }
}

/// Fetch, hash, and compare one citation's URL against its previously
/// recorded hash (if any). Never panics on network failure — an
/// unreachable source is recorded as `Unreachable`, not a crash.
///
/// SSRF-hardened: rejects non-http(s) schemes and any URL whose host
/// resolves to a private/loopback/link-local/metadata address *before*
/// fetching (the client itself also has redirects disabled — see
/// `spawn_scheduler` — so a same-host response can't smuggle a second hop
/// past this check).
async fn verify_one(
    client: &reqwest::Client,
    citation_id: &str,
    url: &str,
    store: &ClaimStore,
) -> CitationVerification {
    let now = now_iso();
    let previous_hash = store
        .get_verification(citation_id)
        .ok()
        .flatten()
        .and_then(|v| v.content_hash);

    let unreachable = |now: String| CitationVerification {
        citation_id: citation_id.to_string(),
        url: url.to_string(),
        last_checked: now,
        content_hash: None,
        status: VerificationStatus::Unreachable,
    };

    if !is_safe_url(url).await {
        tracing::warn!(
            "citation verification: rejecting unsafe/unresolvable URL for {citation_id}: {url}"
        );
        return unreachable(now);
    }

    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => match read_capped_body(resp).await {
            Some(body) => {
                let content_hash = blake3::hash(&body).to_hex().to_string();
                let status = match &previous_hash {
                    Some(prev) if *prev != content_hash => VerificationStatus::Drifted,
                    _ => VerificationStatus::Ok,
                };
                CitationVerification {
                    citation_id: citation_id.to_string(),
                    url: url.to_string(),
                    last_checked: now,
                    content_hash: Some(content_hash),
                    status,
                }
            }
            None => unreachable(now),
        },
        _ => unreachable(now),
    }
}

/// Read a response body up to `MAX_BODY_BYTES`, aborting (returning `None`)
/// if the source keeps sending past the cap — prevents a single large or
/// endlessly-streaming citation response from ballooning memory in the
/// shared server process.
async fn read_capped_body(resp: reqwest::Response) -> Option<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.ok()?;
        if buf.len() + chunk.len() > MAX_BODY_BYTES {
            return None;
        }
        buf.extend_from_slice(&chunk);
    }
    Some(buf)
}

/// `true` iff `url` is `http`/`https` and every IP its host resolves to is
/// a public (non-private, non-loopback, non-link-local, non-metadata)
/// address. Resolution failure (including an unparseable URL) is unsafe.
async fn is_safe_url(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url) else {
        return false;
    };
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return false;
    }
    let Some(host) = parsed.host_str().map(str::to_owned) else {
        return false;
    };
    let port = parsed.port_or_known_default().unwrap_or(443);
    let lookup = tokio::net::lookup_host((host.as_str(), port)).await;
    let result = match lookup {
        Ok(addrs) => {
            let addrs: Vec<IpAddr> = addrs.map(|a| a.ip()).collect();
            !addrs.is_empty() && addrs.iter().all(|ip| is_safe_ip(*ip))
        }
        Err(_) => false,
    };
    result
}

/// `true` iff `ip` is a publicly-routable address — excludes loopback,
/// RFC1918 private ranges, link-local (which also covers the cloud metadata
/// endpoint `169.254.169.254/16`), broadcast, documentation ranges, and the
/// unspecified address. IPv6 unique-local/link-local are checked manually
/// via the segment prefix rather than `Ipv6Addr::is_unique_local()` /
/// `is_unicast_link_local()`, whose stabilization varies across toolchains
/// and this crate's MSRV is pinned at 1.80.
fn is_safe_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            !(v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified())
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() || v6.is_unspecified() {
                return false;
            }
            let segments = v6.segments();
            let is_unique_local = (segments[0] & 0xfe00) == 0xfc00; // fc00::/7
            let is_link_local = (segments[0] & 0xffc0) == 0xfe80; // fe80::/10
            !(is_unique_local || is_link_local)
        }
    }
}

/// Current UTC time as an ISO 8601 string, without pulling in a full
/// date/time crate — matches this crate's existing style of plain string
/// dates (`history.rs`'s `date_iso` is likewise a hand-formatted string).
fn now_iso() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    // Minimal civil-from-days conversion (Howard Hinnant's algorithm),
    // avoiding a new chrono/time dependency for one timestamp format.
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let (y, mo, d) = civil_from_days(days as i64);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Days-since-epoch to (year, month, day). Public-domain algorithm, Howard
/// Hinnant, "chrono-Compatible Low-Level Date Algorithms".
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_iso_is_well_formed() {
        let s = now_iso();
        // "YYYY-MM-DDTHH:MM:SSZ" — 20 chars.
        assert_eq!(s.len(), 20);
        assert!(s.ends_with('Z'));
        assert_eq!(&s[4..5], "-");
        assert_eq!(&s[7..8], "-");
        assert_eq!(&s[10..11], "T");
    }

    #[test]
    fn civil_from_days_epoch_is_1970_01_01() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn civil_from_days_known_date() {
        // 2000-03-01 is day 11017 since epoch (well-known reference point
        // for the Hinnant algorithm's test suite).
        assert_eq!(civil_from_days(11017), (2000, 3, 1));
    }

    #[tokio::test]
    async fn run_sweep_with_no_cited_ids_does_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let registry = CitationRegistry::default();
        let client = reqwest::Client::new();
        // Must not panic or hang with an empty store.
        run_sweep(&client, &store, &registry).await;
        assert!(store.drifted_citations().unwrap().is_empty());
    }

    #[tokio::test]
    async fn verify_one_records_unreachable_for_a_bad_host() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()
            .unwrap();
        let v = verify_one(
            &client,
            "test-cite",
            "http://127.0.0.1.invalid.example/nope",
            &store,
        )
        .await;
        assert_eq!(v.status, VerificationStatus::Unreachable);
        assert!(v.content_hash.is_none());
    }

    #[test]
    fn is_safe_ip_rejects_loopback_private_and_link_local_v4() {
        assert!(!is_safe_ip("127.0.0.1".parse().unwrap()));
        assert!(!is_safe_ip("10.0.0.1".parse().unwrap()));
        assert!(!is_safe_ip("192.168.1.1".parse().unwrap()));
        assert!(!is_safe_ip("172.16.0.1".parse().unwrap()));
        // The GCP/AWS/Azure cloud metadata endpoint — link-local range.
        assert!(!is_safe_ip("169.254.169.254".parse().unwrap()));
        assert!(!is_safe_ip("0.0.0.0".parse().unwrap()));
    }

    #[test]
    fn is_safe_ip_accepts_public_v4() {
        assert!(is_safe_ip("8.8.8.8".parse().unwrap()));
        assert!(is_safe_ip("1.1.1.1".parse().unwrap()));
    }

    #[test]
    fn is_safe_ip_rejects_loopback_and_link_local_v6() {
        assert!(!is_safe_ip("::1".parse().unwrap()));
        assert!(!is_safe_ip("fe80::1".parse().unwrap()));
        assert!(!is_safe_ip("fc00::1".parse().unwrap()));
        assert!(!is_safe_ip("::".parse().unwrap()));
    }

    #[test]
    fn is_safe_ip_accepts_public_v6() {
        assert!(is_safe_ip("2606:4700:4700::1111".parse().unwrap())); // Cloudflare DNS
    }

    #[tokio::test]
    async fn is_safe_url_rejects_non_http_scheme() {
        assert!(!is_safe_url("file:///etc/passwd").await);
        assert!(!is_safe_url("ftp://example.com/x").await);
    }

    #[tokio::test]
    async fn is_safe_url_rejects_unparseable() {
        assert!(!is_safe_url("not a url").await);
    }

    #[tokio::test]
    async fn is_safe_url_rejects_loopback_by_ip_literal() {
        // No DNS involved — resolves directly to a loopback literal.
        assert!(!is_safe_url("http://127.0.0.1/admin").await);
    }

    #[tokio::test]
    async fn verify_one_rejects_unsafe_url_without_fetching() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        // A loopback target must be rejected by the pre-fetch safety check,
        // not merely time out or connection-refuse.
        let v = verify_one(&client, "test-cite", "http://127.0.0.1:1/whatever", &store).await;
        assert_eq!(v.status, VerificationStatus::Unreachable);
    }

    /// Spawn a minimal one-shot raw-TCP HTTP/1.1 server that returns `body`
    /// with a correct `Content-Length`, then closes. Returns the address to
    /// fetch. Used to exercise `read_capped_body` against a real
    /// `reqwest::Response` without a mock-HTTP dev-dependency.
    async fn spawn_one_shot_http_server(body: Vec<u8>) -> std::net::SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut discard = [0u8; 1024];
            let _ = socket.read(&mut discard).await;
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            socket.write_all(header.as_bytes()).await.unwrap();
            socket.write_all(&body).await.unwrap();
            socket.shutdown().await.ok();
        });
        addr
    }

    #[tokio::test]
    async fn read_capped_body_accepts_within_cap() {
        let addr = spawn_one_shot_http_server(b"hello world".to_vec()).await;
        let client = reqwest::Client::builder().build().unwrap();
        let resp = client.get(format!("http://{addr}/")).send().await.unwrap();
        assert_eq!(
            read_capped_body(resp).await.unwrap(),
            b"hello world".to_vec()
        );
    }

    #[tokio::test]
    async fn read_capped_body_rejects_oversized_stream() {
        let oversized = vec![b'x'; MAX_BODY_BYTES + 1024];
        let addr = spawn_one_shot_http_server(oversized).await;
        let client = reqwest::Client::builder().build().unwrap();
        let resp = client.get(format!("http://{addr}/")).send().await.unwrap();
        assert!(
            read_capped_body(resp).await.is_none(),
            "oversized body should be rejected, not buffered"
        );
    }
}
