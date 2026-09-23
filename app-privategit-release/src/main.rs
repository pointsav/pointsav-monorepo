// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use anyhow::Result;
use axum::{
    body::Body,
    extract::{ConnectInfo, Path, Query, Request, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tower::ServiceExt;
use tower_http::services::ServeFile;
use tower_http::set_header::SetResponseHeaderLayer;

// Security headers applied to every response. No Content-Security-Policy here —
// this server returns only JSON and binary/file downloads, never HTML, so CSP has
// no meaningful surface; X-Content-Type-Options matters most, to stop a served
// binary from being MIME-sniffed and executed in a browser context.
const HSTS_VALUE: &str = "max-age=63072000; includeSubDomains";

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    releases_dir: String,
    verify_key: Option<VerifyingKey>,
    revocation_list_path: Option<String>,
    revoked_tokens: Arc<RwLock<HashSet<String>>>,
    // C2 fix: two SEPARATE limiters, not one shared bucket. The original single
    // limiter let an unauthenticated caller exhaust /verify-key's budget and
    // have that same exhaustion apply to /admin/reload-revocation-list --
    // live-demonstrated locking out the emergency license-revocation kill
    // switch with nothing but repeated public requests. `public_rate_limiter`
    // now also guards `binary()`'s per-request signature verification (the
    // audit's own point: that route, not the not-publicly-routed /verify-key,
    // is the one actually reachable through this host's nginx config).
    public_rate_limiter: Arc<RateLimiter>,
    admin_rate_limiter: Arc<RateLimiter>,
}

// ── Rate limiting (S6, revised) ─────────────────────────────────────────────
//
// In-memory per-IP sliding-window limiter — no new dependency, matching this
// crate's existing preference for small hand-rolled utilities over pulling in a
// crate for a narrow need.
//
// Revised after an independent audit found the original single-limiter design
// substantively ineffective: (1) it keyed on `ConnectInfo`'s peer address,
// which behind this host's nginx is always 127.0.0.1, not the real client --
// `client_ip()` below now prefers `X-Forwarded-For` (nginx confirmed to set it
// correctly in this vhost's config) and falls back to ConnectInfo only when
// absent (direct/test access); (2) `/verify-key` and the admin
// reload-revocation-list endpoint shared one bucket, so exhausting the public
// endpoint's budget also locked out the emergency revocation kill switch --
// `AppState` now carries two independent `RateLimiter`s (`public_rate_limiter`,
// `admin_rate_limiter`), so public traffic can never affect the admin bucket
// regardless of key computation; (3) the limiter was applied to `/verify-key`,
// which this host's nginx config does not actually proxy publicly at all
// (only `/releases/` and `/git/` are), while `binary()`'s own per-request
// `verify_license_key` call -- genuinely reachable, genuinely doing Ed25519
// verification work per request -- was unlimited. `public_rate_limiter` now
// also guards that call.
//
// Binary download/streaming itself is still deliberately NOT rate-limited --
// that's bundled with the separate Range/caching-headers work instead.
//
// Single-instance, in-process state: not distributed, resets on restart. That's
// an accepted tradeoff for defense-in-depth on a service that (per NEXT.md) has
// no confirmed front-proxy rate limiter in front of it today — this is not a
// substitute for one if it exists, just a floor if it doesn't.
struct RateLimiter {
    max_requests: usize,
    window: std::time::Duration,
    hits: std::sync::Mutex<
        std::collections::HashMap<std::net::IpAddr, std::collections::VecDeque<std::time::Instant>>,
    >,
}

impl RateLimiter {
    fn new(max_requests: usize, window: std::time::Duration) -> Self {
        Self {
            max_requests,
            window,
            hits: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Returns `Ok(())` if the request is allowed, `Err(retry_after_secs)` if the
    /// caller has exceeded `max_requests` within `window`. Prunes stale entries
    /// (and drops empty per-IP queues) on every call — bounded by actual distinct
    /// recent callers, not left to grow unbounded.
    fn check(&self, ip: std::net::IpAddr) -> Result<(), u64> {
        let now = std::time::Instant::now();
        let mut hits = self
            .hits
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let result = {
            let queue = hits.entry(ip).or_default();
            while queue
                .front()
                .is_some_and(|&t| now.duration_since(t) >= self.window)
            {
                queue.pop_front();
            }
            if queue.len() >= self.max_requests {
                // L1 fix: `queue` can be empty here when `max_requests == 0` (a
                // config that denies every request outright) -- `front()` was
                // unconditionally `.unwrap()`'d, which panicked (and, with
                // `panic = "abort"`, killed the whole process) on the very first
                // request. Fall back to the full window as the retry hint in that
                // case instead of assuming there's always a queued hit to measure from.
                let retry_after = queue
                    .front()
                    .map(|&t| self.window.saturating_sub(now.duration_since(t)))
                    .unwrap_or(self.window)
                    .as_secs()
                    .max(1);
                Err(retry_after)
            } else {
                queue.push_back(now);
                Ok(())
            }
        };
        // Opportunistic cleanup of other IPs' fully-expired queues, amortized
        // across calls rather than a separate background task.
        hits.retain(|_, q| !q.is_empty());
        result
    }
}

/// Resolves the real client IP for rate-limiting purposes. Prefers the LAST
/// (rightmost) entry in `X-Forwarded-For`, falling back to the direct TCP
/// peer otherwise. The rightmost entry specifically, not the first: nginx's
/// `$proxy_add_x_forwarded_for` (confirmed in this vhost's config) APPENDS the
/// real connecting peer to whatever the client already sent, producing
/// `"<client-supplied-or-empty>, <real-peer>"` — so the leftmost entry is
/// exactly the part a client CAN spoof, and the rightmost is exactly the part
/// only nginx itself writes. This service is only ever reachable through this
/// host's own nginx (confirmed single-hop topology), so trusting that
/// nginx-written rightmost value is safe; it would not be safe if an
/// untrusted intermediate proxy could sit between nginx and this process.
fn client_ip(headers: &HeaderMap, connect_addr: SocketAddr) -> IpAddr {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.rsplit(',').next())
        .map(str::trim)
        .and_then(|s| s.parse::<IpAddr>().ok())
        .unwrap_or_else(|| connect_addr.ip())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

// Reads RELEASES_DIR/<product>/MANIFEST.json (the PRODUCT-ROOT manifest — distinct
// from the version-dir MANIFEST.json served raw at /releases/:p/:v/MANIFEST) and
// returns the "requires_license" bool. Defaults to true (secure) on any failure:
// missing file, unreadable, unparsable JSON, absent field, or non-bool field.
fn product_requires_license(releases_dir: &str, product: &str) -> bool {
    let manifest = PathBuf::from(releases_dir)
        .join(product)
        .join("MANIFEST.json");
    let Ok(text) = fs::read_to_string(&manifest) else {
        return true; // default: license required
    };
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) else {
        return true;
    };
    val.get("requires_license")
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

/// Rejects any path segment capable of escaping `releases_dir` before it reaches
/// `release_path`'s `PathBuf::push` — a bare `..`, an absolute segment (leading `/`
/// or `\`), or a segment embedding a separator at all (axum percent-decodes `%2F`
/// into a literal `/` within what matchit treated as a single path parameter, so a
/// segment containing `/` is never legitimate product/version/platform input).
///
/// Also rejects control characters (`< 0x20`, e.g. a NUL from `%00`) directly —
/// live adversarial probing this session found `..%00`-style segments clear the
/// four-item blocklist above and reach `File::open`, relying only on Rust's
/// refusal of interior NUL at the syscall boundary (not this gate) to fail safe.
/// The gate itself must be the defense, not an accident of the standard library.
fn is_safe_segment(s: &str) -> bool {
    !s.is_empty()
        && !s.contains('/')
        && !s.contains('\\')
        && s != ".."
        && s != "."
        // L6 fix: the control-character band only excluded C0 (0x00-0x1F),
        // missing DEL (0x7F) -- also a control character, and one some
        // filesystems/shells treat specially. Not independently a traversal
        // vector, but there's no reason to admit it into a path segment either.
        && !s.chars().any(|c| (c as u32) < 0x20 || c == '\u{7f}')
}

fn release_path(releases_dir: &str, parts: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(releases_dir);
    for part in parts {
        p.push(part);
    }
    p
}

/// Error body carrying both a human-readable `error` message and a stable,
/// machine-readable `code` (S15: "5 inconsistent error shapes... no stable
/// machine-readable code field"). `code` is a kebab-case slug that won't change
/// even if `error`'s wording does — every plain error response in this crate goes
/// through this one helper now, instead of each call site hand-rolling its own
/// `json!({"error": "..."})`. `LicenseVerifyErr::reason()` already returns codes in
/// this same style; use it directly rather than duplicating a code table for it.
fn err_json(code: &'static str, message: impl Into<String>) -> Value {
    json!({"error": message.into(), "code": code})
}

/// A weak validator derived from the file's mtime + size, in the common
/// nginx/Apache "weak etag from stat()" style — not a content hash. Hashing every
/// release binary (some multi-hundred-MB) on every request would be its own
/// CPU-exhaustion vector; mtime+size changes whenever the file's replaced, which is
/// the property that matters here.
fn file_etag(meta: &std::fs::Metadata) -> Option<HeaderValue> {
    let secs = meta
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    HeaderValue::from_str(&format!("W/\"{:x}-{:x}\"", secs, meta.len())).ok()
}

/// True if any comma-separated token in an `If-None-Match` header value matches
/// `etag`, or is `*`. Weak comparison (plain string equality) per RFC 7232 §2.3.2 —
/// acceptable since `etag` is already a weak (`W/`-prefixed) validator.
fn if_none_match_hits(header_val: &HeaderValue, etag: &HeaderValue) -> bool {
    let (Ok(given), Ok(etag)) = (header_val.to_str(), etag.to_str()) else {
        return false;
    };
    given
        .split(',')
        .map(str::trim)
        .any(|tok| tok == "*" || tok == etag)
}

fn set_caching_headers(headers: &mut HeaderMap, etag: &HeaderValue) {
    headers.insert(header::ETAG, etag.clone());
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=300, must-revalidate"),
    );
}

/// Streams a file with Range/Last-Modified support via `tower_http::ServeFile`,
/// replacing a raw `ReaderStream` that had none of those — a dropped connection on
/// a large binary used to restart from byte 0. ETag and Cache-Control are added by
/// this function directly (M1 fix): `ServeFile` was previously assumed to supply
/// them too, but its vendored source (tower-http 0.5.2) shows it emits only
/// `Accept-Ranges` and `Last-Modified` — no `ETag`, no `Cache-Control` at all.
///
/// Also rejects any `path` that isn't a regular file before handing off to
/// `ServeFile` (M3 fix): `ServeFile`'s single-file mode never checks `is_dir()`
/// (that check only exists in `ServeDir`'s directory-listing mode) — opening a
/// directory fd as if it were a file succeeds on Unix and would otherwise have
/// streamed a bogus `200 OK` with an unreadable body instead of a clean 404.
///
/// `req_headers` forwards the incoming `Range`/`If-Modified-Since`/`If-Unmodified-Since`
/// headers through to `ServeFile` so it can actually honor them; `If-None-Match` is
/// handled here directly, since `ServeFile` doesn't understand it at all.
/// `content_type` overrides `ServeFile`'s extension-guessed MIME type (this crate's
/// release filenames don't reliably carry the right extension). `content_disposition`,
/// when set, forces a download with that filename rather than inline rendering.
async fn stream_file(
    req_headers: &HeaderMap,
    path: PathBuf,
    content_type: &'static str,
    content_disposition: Option<&str>,
) -> Response {
    let meta = match tokio::fs::metadata(&path).await {
        Ok(meta) if meta.is_file() => meta,
        _ => {
            // Covers both "doesn't exist" and "exists but isn't a regular file"
            // (M3) — never echo the real filesystem path to the client (live-
            // confirmed disclosure this session); log it instead.
            tracing::debug!(path = %path.display(), "stream_file: not found or not a regular file");
            return (
                StatusCode::NOT_FOUND,
                Json(err_json("not-found", "not found")),
            )
                .into_response();
        }
    };
    let etag = file_etag(&meta);

    if let (Some(etag), Some(inm)) = (&etag, req_headers.get(header::IF_NONE_MATCH)) {
        if if_none_match_hits(inm, etag) {
            let mut resp = StatusCode::NOT_MODIFIED.into_response();
            set_caching_headers(resp.headers_mut(), etag);
            return resp;
        }
    }

    let mut req = Request::new(Body::empty());
    *req.headers_mut() = req_headers.clone();
    // ServeDir/ServeFile's Error is Infallible (checked against tower-http 0.5's
    // source directly) — IO failures become a response (404/500), not a Service err.
    let resp = ServeFile::new(&path)
        .oneshot(req)
        .await
        .expect("ServeFile's Service::Error is Infallible");
    if resp.status() == StatusCode::NOT_FOUND {
        tracing::debug!(path = %path.display(), "stream_file: not found");
        return (
            StatusCode::NOT_FOUND,
            Json(err_json("not-found", "not found")),
        )
            .into_response();
    }
    let (mut parts, body) = resp.into_parts();
    // M2 fix: only stamp content-identity headers on responses that actually carry
    // the file's representation. The old code stamped Content-Type/Content-Disposition
    // unconditionally, including on 304/412/416/500 — meaningless there, since those
    // describe "nothing changed" or an error, not the file itself.
    if matches!(parts.status, StatusCode::OK | StatusCode::PARTIAL_CONTENT) {
        parts
            .headers
            .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
        if let Some(name) = content_disposition {
            if let Ok(v) = HeaderValue::from_str(&format!("attachment; filename=\"{name}\"")) {
                parts.headers.insert(header::CONTENT_DISPOSITION, v);
            }
        }
    }
    if let Some(etag) = &etag {
        if matches!(
            parts.status,
            StatusCode::OK | StatusCode::PARTIAL_CONTENT | StatusCode::NOT_MODIFIED
        ) {
            set_caching_headers(&mut parts.headers, etag);
        }
    }
    Response::from_parts(parts, Body::new(body))
}

fn load_verify_key(val: &str) -> Option<VerifyingKey> {
    // Accept either a 64-char hex string directly or a path to a file containing one.
    let hex = if val.len() == 64 && val.chars().all(|c| c.is_ascii_hexdigit()) {
        val.to_string()
    } else {
        fs::read_to_string(val).ok()?.trim().to_string()
    };
    let bytes = hex::decode(&hex).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&arr).ok()
}

// Deploy note: update this file atomically (write to .tmp then rename) to avoid
// partial reads during live reloads.
fn load_revocation_list(path: &str) -> std::io::Result<HashSet<String>> {
    let content = fs::read_to_string(path)?;
    let set = content
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let lower = l.to_lowercase();
            if lower.len() != 64 || !lower.chars().all(|c| c.is_ascii_hexdigit()) {
                tracing::warn!(
                    line = l,
                    "revocation list: skipping non-fingerprint line (expected 64-char SHA256 hex; \
                     did you paste a raw token instead of its fingerprint?)"
                );
                return None;
            }
            Some(lower)
        })
        .collect();
    Ok(set)
}

fn token_fingerprint(raw_b64: &str) -> String {
    use sha2::{Digest, Sha256};
    // Fingerprint is SHA256 of the canonical URL_SAFE_NO_PAD base64url token string.
    // Stored fingerprints must use the same encoding — see tool-wallet fingerprint subcommand.
    hex::encode(Sha256::digest(raw_b64.as_bytes()))
}

// ── Version helpers ───────────────────────────────────────────────────────────

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u64> { s.split('.').map(|p| p.parse().unwrap_or(0)).collect() };
    parse(a).cmp(&parse(b))
}

fn latest_version_with_platform(
    releases_dir: &str,
    product: &str,
    platform: &str,
) -> Option<String> {
    let product_dir = PathBuf::from(releases_dir).join(product);
    let mut versions: Vec<String> = fs::read_dir(&product_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|v| product_dir.join(v).join(platform).exists())
        .collect();
    versions.sort_by(|a, b| compare_versions(a, b));
    versions.into_iter().last()
}

// ── License verification ──────────────────────────────────────────────────────

enum LicenseVerifyErr {
    MalformedToken,
    TokenTooShort,
    InvalidSignature,
    InvalidPayload,
    WrongProduct,
    ChannelExpired(String),
    Revoked,
}

impl LicenseVerifyErr {
    fn status(&self) -> StatusCode {
        match self {
            Self::WrongProduct | Self::ChannelExpired(_) | Self::Revoked => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
    fn reason(&self) -> &'static str {
        match self {
            Self::MalformedToken => "malformed-token",
            Self::TokenTooShort => "token-too-short",
            Self::InvalidSignature => "invalid-signature",
            Self::InvalidPayload => "invalid-payload",
            Self::WrongProduct => "wrong-product",
            Self::ChannelExpired(_) => "channel-expired",
            Self::Revoked => "token-revoked",
        }
    }
}

fn verify_license_key(
    vk: &VerifyingKey,
    key_b64: &str,
    product_id: &str,
    revoked_tokens: &RwLock<HashSet<String>>,
) -> Result<LicensePayload, LicenseVerifyErr> {
    use LicenseVerifyErr::*;
    let token_bytes = URL_SAFE_NO_PAD
        .decode(key_b64)
        .map_err(|_| MalformedToken)?;
    if token_bytes.len() <= 64 {
        return Err(TokenTooShort);
    }
    let (sig_bytes, payload_bytes) = token_bytes.split_at(64);
    let sig_arr: [u8; 64] = sig_bytes.try_into().expect("exactly 64 bytes");
    let sig = Signature::from_bytes(&sig_arr);
    if vk.verify_strict(payload_bytes, &sig).is_err() {
        return Err(InvalidSignature);
    }
    let payload: LicensePayload =
        serde_json::from_slice(payload_bytes).map_err(|_| InvalidPayload)?;
    if payload.product != product_id && !payload.entitled_products.iter().any(|p| p == product_id) {
        return Err(WrongProduct);
    }
    // Revocation check before expiry: a revoked key returns Revoked regardless of expiry date.
    // Fingerprint is of the RAW base64url token STRING (not the decoded bytes).
    {
        // S11: recover from a poisoned lock rather than panicking. A prior panic while
        // holding the write lock (line ~626) would otherwise turn into a full-service
        // panic loop on every subsequent request — the revocation set itself is still
        // structurally valid after a panic mid-read/write, just possibly stale.
        let revoked = revoked_tokens
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !revoked.is_empty() && revoked.contains(&token_fingerprint(key_b64)) {
            return Err(Revoked);
        }
    }
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    if payload.channel_expiry < today {
        return Err(ChannelExpired(payload.channel_expiry.clone()));
    }
    Ok(payload)
}

// ── Request / payload types ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct VerifyKeyRequest {
    license_key_b64: String,
    product_id: String,
}

#[derive(Deserialize, Default)]
struct BinaryQuery {
    token: Option<String>,
}

#[derive(Deserialize)]
struct LicensePayload {
    product: String,
    channel_expiry: String,
    entitlements: Vec<String>,
    version_floor: Option<String>,
    /// Additional product IDs this token also authorizes, beyond `product` itself —
    /// e.g. a purchase of an appliance that bundles another product's functionality
    /// as a managed child process (app-orchestration-command bundling
    /// app-orchestration-slm) grants registry access to both. Absent/empty on any
    /// token minted before this field existed — `product` alone is always checked
    /// first, so those keep verifying exactly as before.
    #[serde(default)]
    entitled_products: Vec<String>,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "service": "app-privategit-release"}))
}

async fn releases_index(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    let base = PathBuf::from(&state.releases_dir);
    let products: Vec<String> = fs::read_dir(&base)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    (StatusCode::OK, Json(json!({"products": products})))
}

async fn product_index(
    State(state): State<Arc<AppState>>,
    Path(product): Path<String>,
) -> (StatusCode, Json<Value>) {
    if !is_safe_segment(&product) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json("invalid-identifier", "invalid product identifier")),
        );
    }
    let base = release_path(&state.releases_dir, &[&product]);
    if !base.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(err_json("product-not-found", "product not found")),
        );
    }
    let versions: Vec<String> = fs::read_dir(&base)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    (
        StatusCode::OK,
        Json(json!({"product": product, "versions": versions})),
    )
}

async fn manifest(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((product, version)): Path<(String, String)>,
) -> Response {
    if !is_safe_segment(&product) || !is_safe_segment(&version) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json(
                "invalid-identifier",
                "invalid product or version identifier",
            )),
        )
            .into_response();
    }
    // NO AUTH CHECK AT ALL — this route serves manifest JSON raw, whichever file it
    // resolves to.
    //
    // S2 fix: the per-version manifest (`<product>/<version>/MANIFEST.json`) is
    // never written by the deposit pipeline — this route 404s for every product on
    // every host today, breaking the marketplace product-detail page's SHA256
    // display. Per the audit's own suggested fix ("repoint the detail page at
    // product-root"), fall back to the product-root manifest
    // (`<product>/MANIFEST.json`) — the file that's actually always deposited, and
    // already trusted enough to drive the unauthenticated `requires_license` check
    // (`product_requires_license` above) — when no version-specific file exists.
    // A real per-version manifest, if the deposit pipeline is later extended to
    // write one, still takes priority.
    let versioned = release_path(&state.releases_dir, &[&product, &version, "MANIFEST.json"]);
    let path = if versioned.is_file() {
        versioned
    } else {
        release_path(&state.releases_dir, &[&product, "MANIFEST.json"])
    };
    // L9 (informational, not fixed here): this route serves whatever `sha256`
    // value the deposit pipeline (`xtask deposit`/`characterize`, a shared
    // cross-archive workspace member -- not this crate's code) happened to write
    // into MANIFEST.json at build time. Nothing on this side re-hashes the
    // release binary to confirm the two still agree. A stale or hand-edited
    // MANIFEST could serve an incorrect checksum without this route ever
    // noticing. Fixing it for real means either the deposit pipeline itself
    // (out of this crate's scope) or a separate periodic reconciliation job --
    // hashing every release binary on every manifest request is not viable for
    // multi-hundred-MB artifacts. Routed to Command as a NEXT.md item rather
    // than papered over here.
    stream_file(&headers, path, "application/json", None).await
}

async fn latest_redirect(
    State(state): State<Arc<AppState>>,
    Path((product, platform)): Path<(String, String)>,
    Query(query): Query<BinaryQuery>,
) -> Response {
    if !is_safe_segment(&product) || !is_safe_segment(&platform) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json(
                "invalid-identifier",
                "invalid product or platform identifier",
            )),
        )
            .into_response();
    }
    // C1 fix: `query.token` used to reach `Redirect::temporary()` completely
    // unvalidated. axum's Redirect constructors panic if the target isn't a
    // valid HTTP header value (a percent-decoded newline, say) -- and this
    // workspace sets `panic = "abort"` in every profile, so that panic took
    // down the whole process. This route is the one nginx actually proxies
    // publicly (`location ^~ /releases/`), unlike the equivalent bug class in
    // the marketplace crate -- reusing `is_safe_segment` since a license token
    // never legitimately contains `/`, `\`, or control characters either.
    if let Some(tok) = &query.token {
        if !is_safe_segment(tok) {
            return (
                StatusCode::BAD_REQUEST,
                Json(err_json("invalid-token", "invalid token")),
            )
                .into_response();
        }
    }
    match latest_version_with_platform(&state.releases_dir, &product, &platform) {
        Some(version) => {
            let target = match &query.token {
                Some(tok) => format!("/releases/{product}/{version}/{platform}?token={tok}"),
                None => format!("/releases/{product}/{version}/{platform}"),
            };
            Redirect::temporary(&target).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "no binary available for this platform",
                "code": "no-binary-for-platform",
                "hint": "The formal build pipeline has not produced a release for this platform yet.",
            })),
        )
            .into_response(),
    }
}

async fn binary(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((product, version, platform)): Path<(String, String, String)>,
    Query(query): Query<BinaryQuery>,
) -> Response {
    if !is_safe_segment(&product) || !is_safe_segment(&version) || !is_safe_segment(&platform) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json(
                "invalid-identifier",
                "invalid product, version, or platform identifier",
            )),
        )
            .into_response();
    }
    // 1. Detached .sig files are unauthenticated — no license required at all.
    if let Some(base_platform) = platform.strip_suffix(".sig") {
        let path = release_path(
            &state.releases_dir,
            &[&product, &version, &format!("{base_platform}.sig")],
        );
        return stream_file(&headers, path, "application/octet-stream", None).await;
    }

    // 2. Open products (requires_license: false in PRODUCT-ROOT MANIFEST.json) — serve without auth.
    if !product_requires_license(&state.releases_dir, &product) {
        tracing::info!(product_id = %product, result = "ok-open", "binary-download");
        let path = release_path(&state.releases_dir, &[&product, &version, &platform]);
        let filename = format!("{product}-{version}-{platform}");
        return stream_file(&headers, path, "application/octet-stream", Some(&filename)).await;
    }

    // 3. License required — accept Authorization: Bearer <token> header OR ?token= query param
    // (header takes precedence).
    let key_b64 = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or_else(|| query.token.clone());

    let key_b64 = match key_b64 {
        Some(k) => k,
        None => {
            tracing::info!(product_id = %product, result = "unauthorized", reason = "missing-auth", "binary-download");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "license key required",
                    "code": "license-key-required",
                    "header": "Authorization: Bearer <license_key_b64>",
                    "query": "?token=<license_key_b64>",
                })),
            )
                .into_response();
        }
    };

    // 4. Server verify key never configured -> 503.
    let Some(vk) = &state.verify_key else {
        tracing::warn!(product_id = %product, result = "service-unavailable", "binary-download: VERIFY_KEY_PUB not set");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(err_json(
                "service-unavailable",
                "license verification not configured",
            )),
        )
            .into_response();
    };

    // 5. Rate limit before the CPU-expensive part (C2 fix): this is the route
    // genuinely reachable through this host's nginx config and doing
    // per-request Ed25519 verification on every call — the audit's own point
    // that the original limiter protected an unreachable route while this one
    // stayed unlimited. Shares the PUBLIC bucket with /verify-key, never the
    // admin bucket.
    let ip = client_ip(&headers, addr);
    if let Err(retry_after) = state.public_rate_limiter.check(ip) {
        tracing::warn!(product_id = %product, %ip, retry_after, "binary-download: rate limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(
                header::RETRY_AFTER,
                HeaderValue::from_str(&retry_after.to_string()).unwrap(),
            )],
            Json(json!({
                "error": "rate limited",
                "code": "rate-limited",
                "retry_after_seconds": retry_after,
            })),
        )
            .into_response();
    }

    // 6. Verify the license key.
    let key_fp = hex::encode(&vk.as_bytes()[..4]);
    match verify_license_key(vk, &key_b64, &product, &state.revoked_tokens) {
        Err(e) => {
            let log_result = if e.status() == StatusCode::UNAUTHORIZED {
                "unauthorized"
            } else {
                "forbidden"
            };
            tracing::info!(product_id = %product, key_fp = %key_fp, result = log_result, reason = e.reason(), "binary-download");
            // `e.reason()` is already a stable kebab-case code — used as both
            // fields here since binary-download errors don't have a separate
            // longer human message today (unlike the other err_json() sites).
            return (e.status(), Json(err_json(e.reason(), e.reason()))).into_response();
        }
        Ok(_payload) => {
            tracing::info!(product_id = %product, key_fp = %key_fp, result = "ok", "binary-download");
        }
    }

    // 7. On success: stream the binary.
    let path = release_path(&state.releases_dir, &[&product, &version, &platform]);
    let filename = format!("{product}-{version}-{platform}");
    stream_file(&headers, path, "application/octet-stream", Some(&filename)).await
}

async fn git_stub() -> (StatusCode, Json<Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "error": "smart-HTTP Git not yet enabled",
            "code": "not-yet-enabled",
            "see": "https://github.com/pointsav/pointsav-monorepo",
            "arriving": "v0.0.2"
        })),
    )
}

// Token format: base64url( sig[64] || payload_json )
// sig is Ed25519 over payload_json bytes.
// 200: valid, authorized, not expired
// 401: bad signature or malformed token
// 403: valid sig but wrong product or channel expired
async fn verify_key_endpoint(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<VerifyKeyRequest>,
) -> (StatusCode, Json<Value>) {
    // S6: this endpoint is a free, unbounded-CPU signature-verification oracle —
    // rate-limited before doing any real work. Uses the PUBLIC limiter (shared
    // with binary()'s per-request verify_license_key call, never with the
    // admin endpoint's limiter) and the real client IP, not nginx's own
    // 127.0.0.1 peer address (C2 fix).
    let ip = client_ip(&headers, addr);
    if let Err(retry_after) = state.public_rate_limiter.check(ip) {
        tracing::warn!(%ip, retry_after, "verify-key: rate limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "error": "rate limited",
                "code": "rate-limited",
                "retry_after_seconds": retry_after,
            })),
        );
    }
    let Some(vk) = &state.verify_key else {
        tracing::warn!(
            result = "service-unavailable",
            "verify-key: VERIFY_KEY_PUB not set"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(err_json(
                "service-unavailable",
                "verify key not configured — set VERIFY_KEY_PUB",
            )),
        );
    };
    let key_fp = hex::encode(&vk.as_bytes()[..4]);

    match verify_license_key(
        vk,
        &req.license_key_b64,
        &req.product_id,
        &state.revoked_tokens,
    ) {
        Err(ref e @ LicenseVerifyErr::ChannelExpired(ref expired)) => {
            tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "forbidden", reason = "channel-expired", expired = %expired, "verify-key");
            // S15: was hardcoded "channel expired" (space) here while every other
            // arm below uses e.reason() ("channel-expired", hyphenated) — the exact
            // inconsistency the audit named by example. Same stable code now, both
            // arms.
            (
                e.status(),
                Json(json!({"valid": false, "reason": e.reason(), "expired": expired})),
            )
        }
        Err(e) => {
            let log_result = if e.status() == StatusCode::UNAUTHORIZED {
                "unauthorized"
            } else {
                "forbidden"
            };
            tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = log_result, reason = e.reason(), "verify-key");
            (
                e.status(),
                Json(json!({"valid": false, "reason": e.reason()})),
            )
        }
        Ok(payload) => {
            tracing::info!(product_id = %payload.product, key_fp = %key_fp, result = "ok", "verify-key");
            (
                StatusCode::OK,
                Json(json!({
                    "valid": true,
                    "product": payload.product,
                    "version_floor": payload.version_floor,
                    "channel_expiry": payload.channel_expiry,
                    "entitlements": payload.entitlements,
                })),
            )
        }
    }
}

async fn reload_revocation_list(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> Response {
    if !addr.ip().is_loopback() {
        return (
            StatusCode::FORBIDDEN,
            Json(err_json(
                "forbidden-non-loopback",
                "admin endpoints are localhost-only",
            )),
        )
            .into_response();
    }
    // S6: on-demand disk read, checked after the loopback gate (no point rate
    // limiting a request that's about to be rejected anyway). Uses the
    // dedicated ADMIN limiter (C2 fix) -- never shared with public traffic,
    // regardless of key computation. Keys on the direct TCP peer, not
    // X-Forwarded-For: this endpoint is reached by direct loopback access
    // only (nginx doesn't proxy it at all), so addr.ip() is already correct
    // and trusting a client-supplied header here would be actively wrong.
    if let Err(retry_after) = state.admin_rate_limiter.check(addr.ip()) {
        tracing::warn!(ip = %addr.ip(), retry_after, "reload-revocation-list: rate limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(
                header::RETRY_AFTER,
                HeaderValue::from_str(&retry_after.to_string()).unwrap(),
            )],
            Json(json!({
                "error": "rate limited",
                "code": "rate-limited",
                "retry_after_seconds": retry_after,
            })),
        )
            .into_response();
    }
    let Some(ref path) = state.revocation_list_path else {
        return (
            StatusCode::NOT_FOUND,
            Json(err_json("not-configured", "no revocation list configured")),
        )
            .into_response();
    };
    match load_revocation_list(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            StatusCode::NOT_FOUND,
            Json(err_json("not-found", "revocation list file not found")),
        )
            .into_response(),
        Err(e) => {
            tracing::warn!("reload_revocation_list failed: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err_json("internal-error", e.to_string())),
            )
                .into_response()
        }
        Ok(fresh) => {
            let count = fresh.len();
            *state
                .revoked_tokens
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = fresh;
            Json(json!({"reloaded": count})).into_response()
        }
    }
}

async fn verify_key_pub(State(state): State<Arc<AppState>>) -> Response {
    match &state.verify_key {
        Some(vk) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            hex::encode(vk.to_bytes()),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(err_json("not-configured", "verify key not configured")),
        )
            .into_response(),
    }
}

async fn install_script(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(product): Path<String>,
) -> Response {
    if !is_safe_segment(&product) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json("invalid-identifier", "invalid product identifier")),
        )
            .into_response();
    }
    let path = release_path(&state.releases_dir, &[&product, "install.sh"]);
    stream_file(&headers, path, "text/x-shellscript", None).await
}

// ── Router ────────────────────────────────────────────────────────────────────

/// Extracted from `main()` so router-level integration tests can drive real
/// requests through actual routing (matchit segment matching + percent-decoding)
/// instead of calling handler functions directly — the only way to prove a
/// path-traversal fix actually holds through the real request path.
fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/releases/", get(releases_index))
        .route("/releases/:product/", get(product_index))
        .route("/releases/:product/install.sh", get(install_script))
        .route("/releases/:product/:version/MANIFEST", get(manifest))
        .route("/releases/:product/latest/:platform", get(latest_redirect))
        .route("/releases/:product/:version/:platform", get(binary))
        .route("/git/*path", get(git_stub).post(git_stub))
        .route("/verify-key", post(verify_key_endpoint))
        .route("/verify-key.pub", get(verify_key_pub))
        .route(
            "/admin/reload-revocation-list",
            post(reload_revocation_list),
        )
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static(HSTS_VALUE),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .with_state(state)
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    // Same env var names as the old crate, except SOURCE_BIND defaults to a test
    // port (19201) so the ground-up rewrite never collides with the live
    // app-privategit-release service on 9201.
    let bind_addr = std::env::var("SOURCE_BIND").unwrap_or_else(|_| "127.0.0.1:19201".into());
    let releases_dir =
        std::env::var("RELEASES_DIR").unwrap_or_else(|_| "/var/lib/local-software/releases".into());

    let verify_key = std::env::var("VERIFY_KEY_PUB")
        .ok()
        .and_then(|path| load_verify_key(&path));
    if verify_key.is_none() {
        tracing::warn!("VERIFY_KEY_PUB not set — /verify-key will return 503");
    }

    let revocation_list_path = std::env::var("REVOCATION_LIST_PATH")
        .ok()
        .filter(|p| !p.is_empty());
    let revoked_tokens = Arc::new(RwLock::new(match &revocation_list_path {
        None => HashSet::new(),
        Some(path) => match load_revocation_list(path) {
            Ok(set) => {
                tracing::info!("loaded {} revoked token fingerprints", set.len());
                set
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::warn!("REVOCATION_LIST_PATH file not found: {e}");
                HashSet::new()
            }
            Err(e) => {
                tracing::warn!("REVOCATION_LIST_PATH unreadable: {e}");
                HashSet::new()
            }
        },
    }));

    // 20 requests/60s per IP on each bucket — conservative default, not tuned
    // against real traffic yet. Two independent limiters (C2 fix): public
    // traffic against /verify-key or binary()'s signature check can never
    // exhaust the admin endpoint's budget.
    let public_rate_limiter = Arc::new(RateLimiter::new(20, std::time::Duration::from_secs(60)));
    let admin_rate_limiter = Arc::new(RateLimiter::new(20, std::time::Duration::from_secs(60)));

    let state = Arc::new(AppState {
        releases_dir,
        verify_key,
        revocation_list_path,
        revoked_tokens,
        public_rate_limiter,
        admin_rate_limiter,
    });

    let app = build_router(state);

    tracing::info!("app-privategit-release listening on {bind_addr}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// SAFETY: no test binds a TCP port (handlers are called directly with hand-built
// extractors) and every test writes ONLY under a unique scratch dir inside
// `std::env::temp_dir()` (`/tmp`). Nothing here touches `/var/lib/local-software/`
// or ports 9201/9202. Ed25519 keypairs are generated in-test from fixed seeds —
// no production keys are read. House style matches app-privategit-marketplace-2.
#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SEQ: AtomicUsize = AtomicUsize::new(0);

    /// Fresh, unique scratch directory under /tmp for one test.
    fn scratch_dir(tag: &str) -> PathBuf {
        let n = SEQ.fetch_add(1, Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "src2-test-{tag}-{}-{n}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Deterministic in-test Ed25519 keypair. Never a production key.
    fn test_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[42u8; 32])
    }

    /// A second, different keypair for wrong-key tests.
    fn other_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[43u8; 32])
    }

    /// Token format: base64url_no_pad( sig[64] || payload_json ), sig over payload_json.
    fn make_token(sk: &SigningKey, payload_json: &str) -> String {
        let sig = sk.sign(payload_json.as_bytes());
        let mut bytes = sig.to_bytes().to_vec();
        bytes.extend_from_slice(payload_json.as_bytes());
        URL_SAFE_NO_PAD.encode(bytes)
    }

    fn payload_json(product: &str, expiry: &str) -> String {
        format!(
            r#"{{"product":"{product}","channel_expiry":"{expiry}","entitlements":["source"]}}"#
        )
    }

    fn today_str() -> String {
        // Same formatting as verify_license_key uses internally.
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    }

    fn no_revocations() -> RwLock<HashSet<String>> {
        RwLock::new(HashSet::new())
    }

    fn test_state(
        releases_dir: &std::path::Path,
        verify_key: Option<VerifyingKey>,
        revocation_list_path: Option<String>,
    ) -> Arc<AppState> {
        Arc::new(AppState {
            releases_dir: releases_dir.to_string_lossy().into_owned(),
            verify_key,
            revocation_list_path,
            revoked_tokens: Arc::new(RwLock::new(HashSet::new())),
            // Effectively unlimited — existing tests hit handlers repeatedly from
            // the same IP; rate-limit-specific tests construct their own state
            // with a tight limit via `test_state_with_admin_rate_limit`/
            // `test_state_with_public_rate_limit` instead.
            public_rate_limiter: Arc::new(RateLimiter::new(
                100_000,
                std::time::Duration::from_secs(60),
            )),
            admin_rate_limiter: Arc::new(RateLimiter::new(
                100_000,
                std::time::Duration::from_secs(60),
            )),
        })
    }

    fn test_state_with_admin_rate_limit(
        releases_dir: &std::path::Path,
        max_requests: usize,
    ) -> Arc<AppState> {
        let mut state = (*test_state(releases_dir, None, None)).clone();
        state.admin_rate_limiter = Arc::new(RateLimiter::new(
            max_requests,
            std::time::Duration::from_secs(60),
        ));
        Arc::new(state)
    }

    fn test_state_with_public_rate_limit(
        releases_dir: &std::path::Path,
        verify_key: Option<VerifyingKey>,
        max_requests: usize,
    ) -> Arc<AppState> {
        let mut state = (*test_state(releases_dir, verify_key, None)).clone();
        state.public_rate_limiter = Arc::new(RateLimiter::new(
            max_requests,
            std::time::Duration::from_secs(60),
        ));
        Arc::new(state)
    }

    async fn body_json(resp: Response) -> Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    // ── product_requires_license ──────────────────────────────────────────────

    #[test]
    fn requires_license_missing_manifest_defaults_true() {
        let scratch = scratch_dir("reqlic-missing");
        // Product dir exists but has no MANIFEST.json — secure default.
        fs::create_dir_all(scratch.join("prod")).unwrap();
        assert!(product_requires_license(scratch.to_str().unwrap(), "prod"));
        // Product dir does not exist at all.
        assert!(product_requires_license(
            scratch.to_str().unwrap(),
            "no-such-product"
        ));
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── err_json / unified error schema (S15) ──────────────────────────────────

    #[test]
    fn err_json_carries_both_error_and_code() {
        let v = err_json("invalid-identifier", "invalid product identifier");
        assert_eq!(v["error"], "invalid product identifier");
        assert_eq!(v["code"], "invalid-identifier");
    }

    #[tokio::test]
    async fn manifest_rejects_unsafe_segments_carries_stable_code() {
        let scratch = scratch_dir("manifest-code");
        let state = test_state(&scratch, None, None);
        let resp = manifest(
            State(state),
            HeaderMap::new(),
            Path(("..".to_string(), "1.0.0".to_string())),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = body_json(resp).await;
        assert_eq!(body["code"], "invalid-identifier");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn verify_key_endpoint_channel_expired_reason_is_hyphenated_not_spaced() {
        // The exact inconsistency the audit named by example: this arm used to
        // hardcode "channel expired" (space) while every other reason in this same
        // function used LicenseVerifyErr::reason()'s "channel-expired" (hyphen).
        let scratch = scratch_dir("vke-channel-expired-code");
        let sk = test_signing_key();
        let state = test_state(&scratch, Some(sk.verifying_key()), None);
        let token = make_token(&sk, &payload_json("prod", "2020-01-01"));
        let (_, Json(body)) = verify_key_endpoint(
            loopback(),
            State(state),
            HeaderMap::new(),
            Json(VerifyKeyRequest {
                license_key_b64: token,
                product_id: "prod".into(),
            }),
        )
        .await;
        assert_eq!(body["reason"], "channel-expired");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_list_rate_limited_carries_stable_code() {
        let scratch = scratch_dir("rrl-code");
        let list = scratch.join("revoked.txt");
        fs::write(&list, "").unwrap();
        let state = test_state_with_admin_rate_limit(&scratch, 1);
        let mut state = (*state).clone();
        state.revocation_list_path = Some(list.to_string_lossy().into_owned());
        let state = Arc::new(state);

        let _ = reload_revocation_list(loopback(), State(state.clone())).await;
        let resp = reload_revocation_list(loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
        let body = body_json(resp).await;
        assert_eq!(body["code"], "rate-limited");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── stream_file (S10: Range/ETag/Last-Modified/Cache-Control) ─────────────

    #[tokio::test]
    async fn stream_file_serves_full_content_with_accept_ranges() {
        let scratch = scratch_dir("stream-full");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let resp = stream_file(
            &HeaderMap::new(),
            file_path,
            "application/octet-stream",
            None,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers().get(header::ACCEPT_RANGES).unwrap(), "bytes");
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&bytes[..], b"0123456789");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_honors_range_header_with_206_partial_content() {
        let scratch = scratch_dir("stream-range");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(header::RANGE, HeaderValue::from_static("bytes=2-4"));
        let resp = stream_file(&headers, file_path, "application/octet-stream", None).await;
        assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);
        assert!(resp.headers().get(header::CONTENT_RANGE).is_some());
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        // bytes=2-4 is inclusive: '2','3','4'.
        assert_eq!(&bytes[..], b"234");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_content_type_override_wins_over_extension_guess() {
        let scratch = scratch_dir("stream-ctype");
        // No extension at all -- ServeFile would otherwise guess
        // application/octet-stream or fail to guess; the explicit override must win.
        let file_path = scratch.join("install");
        fs::write(&file_path, b"#!/bin/sh\necho hi\n").unwrap();

        let resp = stream_file(&HeaderMap::new(), file_path, "text/x-shellscript", None).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/x-shellscript"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_sets_content_disposition_when_requested() {
        let scratch = scratch_dir("stream-disposition");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"data").unwrap();

        let resp = stream_file(
            &HeaderMap::new(),
            file_path,
            "application/octet-stream",
            Some("os-console-1.0.0-linux-x86_64"),
        )
        .await;
        let disposition = resp
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(disposition.contains("attachment"));
        assert!(disposition.contains("os-console-1.0.0-linux-x86_64"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_omits_content_disposition_when_not_requested() {
        let scratch = scratch_dir("stream-no-disposition");
        let file_path = scratch.join("payload.json");
        fs::write(&file_path, b"{}").unwrap();

        let resp = stream_file(&HeaderMap::new(), file_path, "application/json", None).await;
        assert!(resp.headers().get(header::CONTENT_DISPOSITION).is_none());
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_404_never_echoes_real_filesystem_path() {
        let scratch = scratch_dir("stream-missing");
        let file_path = scratch.join("does-not-exist.bin");

        let resp = stream_file(
            &HeaderMap::new(),
            file_path.clone(),
            "application/octet-stream",
            None,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        let body_str = body.to_string();
        assert!(!body_str.contains(&file_path.to_string_lossy().to_string()));
        assert_eq!(body["error"], "not found");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── stream_file: M1 (ETag/Cache-Control actually present) ─────────────────

    #[tokio::test]
    async fn stream_file_sets_etag_and_cache_control_on_200() {
        let scratch = scratch_dir("stream-etag-200");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let resp = stream_file(
            &HeaderMap::new(),
            file_path,
            "application/octet-stream",
            None,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get(header::ETAG).is_some());
        assert_eq!(
            resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=300, must-revalidate"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_matching_if_none_match_short_circuits_to_304_without_content_type() {
        let scratch = scratch_dir("stream-etag-304");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let first = stream_file(
            &HeaderMap::new(),
            file_path.clone(),
            "application/octet-stream",
            None,
        )
        .await;
        let etag = first.headers().get(header::ETAG).unwrap().clone();

        let mut headers = HeaderMap::new();
        headers.insert(header::IF_NONE_MATCH, etag.clone());
        let second = stream_file(&headers, file_path, "application/octet-stream", None).await;
        assert_eq!(second.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(second.headers().get(header::ETAG).unwrap(), &etag);
        assert!(second.headers().get(header::CONTENT_TYPE).is_none());
        let bytes = axum::body::to_bytes(second.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(bytes.is_empty());
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn stream_file_mismatched_if_none_match_serves_full_content() {
        let scratch = scratch_dir("stream-etag-mismatch");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(
            header::IF_NONE_MATCH,
            HeaderValue::from_static("W/\"stale-etag\""),
        );
        let resp = stream_file(&headers, file_path, "application/octet-stream", None).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── stream_file: M2 (no content-identity headers on non-content statuses) ──

    #[tokio::test]
    async fn stream_file_range_not_satisfiable_omits_our_content_disposition_override() {
        let scratch = scratch_dir("stream-416");
        let file_path = scratch.join("payload.bin");
        fs::write(&file_path, b"0123456789").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(header::RANGE, HeaderValue::from_static("bytes=9999-10999"));
        let resp = stream_file(
            &headers,
            file_path,
            "application/x-marketplace-override",
            Some("payload.bin"),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        // Our custom override must NOT be stamped on an error response -- ServeFile's
        // own guessed type stands instead of a misleading claim that the body is the
        // binary (it's the plain-text "range not satisfiable" message).
        assert_ne!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/x-marketplace-override"
        );
        // A 416 body is an error message, not the file -- forcing a download
        // disposition on it would be actively misleading.
        assert!(resp.headers().get(header::CONTENT_DISPOSITION).is_none());
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── stream_file: M3 (directory-as-path rejected instead of a bogus 200) ────

    #[tokio::test]
    async fn stream_file_rejects_a_directory_path_as_not_found() {
        let scratch = scratch_dir("stream-dir-as-file");
        let dir_path = scratch.join("0.0.1");
        fs::create_dir_all(&dir_path).unwrap();

        let resp = stream_file(
            &HeaderMap::new(),
            dir_path,
            "application/octet-stream",
            None,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "not found");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── latest_redirect (C1: unauthenticated crash on the one publicly-proxied route) ──

    #[tokio::test]
    async fn latest_redirect_302s_to_the_highest_version_with_token() {
        let scratch = scratch_dir("latest-ok");
        fs::create_dir_all(scratch.join("prod/1.0.0")).unwrap();
        fs::create_dir_all(scratch.join("prod/2.0.0")).unwrap();
        fs::write(scratch.join("prod/1.0.0/linux-x86_64"), b"old").unwrap();
        fs::write(scratch.join("prod/2.0.0/linux-x86_64"), b"new").unwrap();
        let state = test_state(&scratch, None, None);
        let resp = latest_redirect(
            State(state),
            Path(("prod".to_string(), "linux-x86_64".to_string())),
            Query(BinaryQuery {
                token: Some("valid-token-abc123".to_string()),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            resp.headers().get(header::LOCATION).unwrap(),
            "/releases/prod/2.0.0/linux-x86_64?token=valid-token-abc123"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    // C1 regression test: a malformed token used to reach Redirect::temporary()
    // unvalidated, which panics on an invalid header value -- and panic = abort
    // in this workspace turns that into a full process crash on one request to
    // the single route nginx actually proxies publicly. Must reject before ever
    // calling Redirect::temporary, not just return something.
    #[tokio::test]
    async fn latest_redirect_rejects_malformed_token_instead_of_panicking() {
        let scratch = scratch_dir("latest-bad-token");
        fs::create_dir_all(scratch.join("prod/1.0.0")).unwrap();
        fs::write(scratch.join("prod/1.0.0/linux-x86_64"), b"data").unwrap();
        let state = test_state(&scratch, None, None);
        let resp = latest_redirect(
            State(state),
            Path(("prod".to_string(), "linux-x86_64".to_string())),
            Query(BinaryQuery {
                token: Some("bad\ntoken\rwith-control-chars".to_string()),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn latest_redirect_404_when_no_version_has_the_platform() {
        let scratch = scratch_dir("latest-404");
        fs::create_dir_all(scratch.join("prod")).unwrap();
        let state = test_state(&scratch, None, None);
        let resp = latest_redirect(
            State(state),
            Path(("prod".to_string(), "linux-x86_64".to_string())),
            Query(BinaryQuery { token: None }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── manifest (S2 fix: per-version MANIFEST route) ─────────────────────────

    #[tokio::test]
    async fn manifest_falls_back_to_product_root_when_version_manifest_missing() {
        let scratch = scratch_dir("manifest-fallback");
        fs::create_dir_all(scratch.join("prod/1.0.0")).unwrap();
        // Only the product-root manifest is deposited — matches every real host today.
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false, "sha256": "root-sha"}"#,
        )
        .unwrap();
        let state = test_state(&scratch, None, None);
        let resp = manifest(
            State(state),
            HeaderMap::new(),
            Path(("prod".to_string(), "1.0.0".to_string())),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        assert_eq!(body["sha256"], "root-sha");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn manifest_prefers_version_specific_manifest_when_present() {
        let scratch = scratch_dir("manifest-versioned");
        fs::create_dir_all(scratch.join("prod/1.0.0")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false, "sha256": "root-sha"}"#,
        )
        .unwrap();
        fs::write(
            scratch.join("prod/1.0.0/MANIFEST.json"),
            r#"{"sha256": "versioned-sha"}"#,
        )
        .unwrap();
        let state = test_state(&scratch, None, None);
        let resp = manifest(
            State(state),
            HeaderMap::new(),
            Path(("prod".to_string(), "1.0.0".to_string())),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        assert_eq!(body["sha256"], "versioned-sha");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn manifest_404s_when_neither_manifest_exists() {
        let scratch = scratch_dir("manifest-neither");
        fs::create_dir_all(scratch.join("prod/1.0.0")).unwrap();
        let state = test_state(&scratch, None, None);
        let resp = manifest(
            State(state),
            HeaderMap::new(),
            Path(("prod".to_string(), "1.0.0".to_string())),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn manifest_rejects_unsafe_segments() {
        let scratch = scratch_dir("manifest-unsafe");
        let state = test_state(&scratch, None, None);
        let resp = manifest(
            State(state),
            HeaderMap::new(),
            Path(("..".to_string(), "1.0.0".to_string())),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn requires_license_malformed_json_defaults_true() {
        let scratch = scratch_dir("reqlic-malformed");
        fs::create_dir_all(scratch.join("prod")).unwrap();
        fs::write(scratch.join("prod/MANIFEST.json"), "{ not json !!!").unwrap();
        assert!(product_requires_license(scratch.to_str().unwrap(), "prod"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn requires_license_field_semantics() {
        let scratch = scratch_dir("reqlic-field");
        let dir = scratch.join("prod");
        fs::create_dir_all(&dir).unwrap();
        let manifest = dir.join("MANIFEST.json");

        // Explicit false — the ONLY way to open a product.
        fs::write(&manifest, r#"{"requires_license": false}"#).unwrap();
        assert!(!product_requires_license(scratch.to_str().unwrap(), "prod"));

        // Explicit true.
        fs::write(&manifest, r#"{"requires_license": true}"#).unwrap();
        assert!(product_requires_license(scratch.to_str().unwrap(), "prod"));

        // Field absent — secure default.
        fs::write(&manifest, r#"{"name": "prod"}"#).unwrap();
        assert!(product_requires_license(scratch.to_str().unwrap(), "prod"));

        // Field present but not a bool — secure default.
        fs::write(&manifest, r#"{"requires_license": "false"}"#).unwrap();
        assert!(product_requires_license(scratch.to_str().unwrap(), "prod"));

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Version helpers ───────────────────────────────────────────────────────

    #[test]
    fn compare_versions_is_numeric_per_segment() {
        use std::cmp::Ordering::*;
        // Numeric-per-segment, NOT string compare: "0.0.9" < "0.0.10"
        // (a plain string compare would say "0.0.10" < "0.0.9").
        assert_eq!(compare_versions("0.0.9", "0.0.10"), Less);
        assert_eq!(compare_versions("1.2.3", "1.10.0"), Less);
        assert_eq!(compare_versions("2.0.0", "1.99.99"), Greater);
        assert_eq!(compare_versions("1.2.3", "1.2.3"), Equal);
        // Differing segment counts: shorter prefix sorts first.
        assert_eq!(compare_versions("1.2", "1.2.0"), Less);
    }

    #[test]
    fn compare_versions_non_numeric_segment_parses_to_zero() {
        use std::cmp::Ordering::*;
        // Documented quirk: a non-numeric dir name like "beta" parses to [0].
        assert_eq!(compare_versions("beta", "0"), Equal);
        assert_eq!(compare_versions("beta", "0.0.1"), Less);
        assert_eq!(compare_versions("1.0.0", "beta"), Greater);
    }

    #[test]
    fn latest_version_picks_numeric_max_with_platform_present() {
        let scratch = scratch_dir("latest");
        let prod = scratch.join("prod");
        for v in ["0.0.1", "0.0.9", "0.0.10"] {
            let d = prod.join(v).join("linux-x86_64");
            fs::create_dir_all(d.parent().unwrap()).unwrap();
            fs::write(&d, b"bin").unwrap();
        }
        // Newer version that LACKS the platform must be skipped.
        fs::create_dir_all(prod.join("0.0.11")).unwrap();
        // Non-numeric dir with the platform present parses to [0] — never the max here.
        let beta = prod.join("beta").join("linux-x86_64");
        fs::create_dir_all(beta.parent().unwrap()).unwrap();
        fs::write(&beta, b"bin").unwrap();
        // A stray FILE in the product dir must be filtered (dirs only).
        fs::write(prod.join("README.txt"), b"x").unwrap();

        assert_eq!(
            latest_version_with_platform(scratch.to_str().unwrap(), "prod", "linux-x86_64"),
            Some("0.0.10".to_string())
        );
        // No version has this platform.
        assert_eq!(
            latest_version_with_platform(scratch.to_str().unwrap(), "prod", "darwin-arm64"),
            None
        );
        // Product dir missing entirely.
        assert_eq!(
            latest_version_with_platform(scratch.to_str().unwrap(), "ghost", "linux-x86_64"),
            None
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── load_verify_key ───────────────────────────────────────────────────────

    #[test]
    fn load_verify_key_accepts_raw_hex_and_file_path() {
        let sk = test_signing_key();
        let hex_key = hex::encode(sk.verifying_key().to_bytes());

        // Form 1: raw 64-char hex string.
        let vk = load_verify_key(&hex_key).expect("raw hex form must load");
        assert_eq!(vk.to_bytes(), sk.verifying_key().to_bytes());

        // Form 2: path to a file containing the hex (with surrounding whitespace).
        let scratch = scratch_dir("vkey");
        let key_file = scratch.join("verify.pub");
        fs::write(&key_file, format!("  {hex_key}\n")).unwrap();
        let vk2 = load_verify_key(key_file.to_str().unwrap()).expect("file form must load");
        assert_eq!(vk2.to_bytes(), sk.verifying_key().to_bytes());
        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn load_verify_key_rejects_bad_inputs() {
        // Not hex and not an existing file.
        assert!(load_verify_key("/nonexistent/path/to/key").is_none());
        // File exists but contains non-hex garbage.
        let scratch = scratch_dir("vkey-bad");
        let bad = scratch.join("bad.pub");
        fs::write(&bad, "this is not hex at all\n").unwrap();
        assert!(load_verify_key(bad.to_str().unwrap()).is_none());
        // File contains hex of the wrong length (16 bytes, not 32).
        fs::write(&bad, "deadbeefdeadbeefdeadbeefdeadbeef\n").unwrap();
        assert!(load_verify_key(bad.to_str().unwrap()).is_none());
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── load_revocation_list ──────────────────────────────────────────────────

    #[test]
    fn load_revocation_list_skips_comments_blanks_and_malformed_lines() {
        let scratch = scratch_dir("revlist");
        let fp_a = token_fingerprint("token-a");
        let fp_b_upper = token_fingerprint("token-b").to_uppercase();
        let list = scratch.join("revoked.txt");
        fs::write(
            &list,
            format!(
                "# comment line\n\
                 \n\
                 {fp_a}\n\
                 not-a-fingerprint\n\
                 {fp_b_upper}\n\
                 zzzz0000zzzz0000zzzz0000zzzz0000zzzz0000zzzz0000zzzz0000zzzz0000\n"
            ),
        )
        .unwrap();
        // Malformed lines warn-and-skip — they must NOT error the whole load.
        let set = load_revocation_list(list.to_str().unwrap()).expect("load must succeed");
        assert_eq!(set.len(), 2);
        assert!(set.contains(&fp_a));
        // Uppercase hex is normalised to lowercase.
        assert!(set.contains(&token_fingerprint("token-b")));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn load_revocation_list_missing_file_is_not_found() {
        let err = load_revocation_list("/nonexistent/revoked.txt").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    // ── token_fingerprint ─────────────────────────────────────────────────────

    #[test]
    fn token_fingerprint_hashes_encoded_string_not_decoded_bytes() {
        use sha2::{Digest, Sha256};
        let decoded = b"some raw token payload bytes";
        let encoded = URL_SAFE_NO_PAD.encode(decoded);

        let fp = token_fingerprint(&encoded);
        // Concrete before/after: the fingerprint IS sha256 of the base64url STRING…
        assert_eq!(fp, hex::encode(Sha256::digest(encoded.as_bytes())));
        // …and is NOT sha256 of the decoded bytes.
        assert_ne!(fp, hex::encode(Sha256::digest(decoded)));
        // Shape: 64 lowercase hex chars.
        assert_eq!(fp.len(), 64);
        assert!(fp
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }

    // ── verify_license_key — every branch of the auth state machine ───────────

    #[test]
    fn verify_rejects_malformed_base64() {
        let vk = test_signing_key().verifying_key();
        let err = verify_license_key(&vk, "!!!not base64url!!!", "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "malformed-token");
        assert_eq!(err.status(), StatusCode::UNAUTHORIZED);
        // Standard-base64 padding is also rejected by URL_SAFE_NO_PAD.
        let err = verify_license_key(&vk, "aGVsbG8=", "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "malformed-token");
    }

    #[test]
    fn verify_rejects_token_too_short() {
        let vk = test_signing_key().verifying_key();
        // Exactly 64 decoded bytes: signature with an EMPTY payload — still too short.
        let exactly_64 = URL_SAFE_NO_PAD.encode([0u8; 64]);
        let err = verify_license_key(&vk, &exactly_64, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "token-too-short");
        assert_eq!(err.status(), StatusCode::UNAUTHORIZED);
        // Far shorter than a signature.
        let tiny = URL_SAFE_NO_PAD.encode(b"tiny");
        let err = verify_license_key(&vk, &tiny, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "token-too-short");
    }

    #[test]
    fn verify_rejects_invalid_signature() {
        let sk = test_signing_key();
        let vk = sk.verifying_key();
        let payload = payload_json("prod", "9999-99-99");

        // Signed by a DIFFERENT key.
        let forged = make_token(&other_signing_key(), &payload);
        let err = verify_license_key(&vk, &forged, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "invalid-signature");
        assert_eq!(err.status(), StatusCode::UNAUTHORIZED);

        // Valid signature but payload tampered after signing.
        let good = make_token(&sk, &payload);
        let mut bytes = URL_SAFE_NO_PAD.decode(&good).unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;
        let tampered = URL_SAFE_NO_PAD.encode(bytes);
        let err = verify_license_key(&vk, &tampered, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "invalid-signature");
    }

    #[test]
    fn verify_rejects_invalid_json_payload() {
        let sk = test_signing_key();
        // Correctly signed, but the payload is not LicensePayload JSON.
        let token = make_token(&sk, "this is not json");
        let err = verify_license_key(&sk.verifying_key(), &token, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "invalid-payload");
        assert_eq!(err.status(), StatusCode::UNAUTHORIZED);

        // Valid JSON but missing required fields is also invalid-payload.
        let token = make_token(&sk, r#"{"product":"prod"}"#);
        let err = verify_license_key(&sk.verifying_key(), &token, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "invalid-payload");
    }

    #[test]
    fn verify_rejects_wrong_product() {
        let sk = test_signing_key();
        let token = make_token(&sk, &payload_json("product-a", "9999-99-99"));
        let err = verify_license_key(&sk.verifying_key(), &token, "product-b", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "wrong-product");
        assert_eq!(err.status(), StatusCode::FORBIDDEN);
    }

    /// A token minted for the bundling product (e.g. app-orchestration-command)
    /// must also verify for a product it lists in entitled_products (e.g.
    /// app-orchestration-slm, folded into the registry rather than sold
    /// separately) — this is the actual new capability, not just "old tokens
    /// still work" (verify_rejects_wrong_product already covers that).
    #[test]
    fn verify_accepts_product_named_in_entitled_products() {
        let sk = test_signing_key();
        let payload = r#"{"product":"app-orchestration-command","channel_expiry":"9999-99-99","entitlements":["binary"],"version_floor":null,"entitled_products":["app-orchestration-command","app-orchestration-slm"]}"#;
        let token = make_token(&sk, payload);
        let result = verify_license_key(
            &sk.verifying_key(),
            &token,
            "app-orchestration-slm",
            &no_revocations(),
        );
        assert!(result.is_ok(), "expected entitled product to verify");
    }

    /// A token's entitled_products list is a real allowlist, not a rubber stamp —
    /// requesting a product absent from both `product` and `entitled_products`
    /// must still fail.
    #[test]
    fn verify_rejects_product_absent_from_entitled_products() {
        let sk = test_signing_key();
        let payload = r#"{"product":"app-orchestration-command","channel_expiry":"9999-99-99","entitlements":["binary"],"version_floor":null,"entitled_products":["app-orchestration-command","app-orchestration-slm"]}"#;
        let token = make_token(&sk, payload);
        let err = verify_license_key(&sk.verifying_key(), &token, "os-totebox", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "wrong-product");
    }

    #[test]
    fn verify_checks_revocation_before_expiry() {
        // A token that is BOTH revoked AND expired must return Revoked, not
        // ChannelExpired — revocation is the stronger, permanent state.
        let sk = test_signing_key();
        let token = make_token(&sk, &payload_json("prod", "2020-01-01")); // long expired
        let revoked = RwLock::new(HashSet::from([token_fingerprint(&token)]));
        let err = verify_license_key(&sk.verifying_key(), &token, "prod", &revoked)
            .err()
            .unwrap();
        assert_eq!(err.reason(), "token-revoked");
        assert_eq!(err.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn verify_revocation_fingerprint_is_of_raw_b64_string_not_decoded_bytes() {
        use sha2::{Digest, Sha256};
        let sk = test_signing_key();
        let token = make_token(&sk, &payload_json("prod", "9999-99-99"));

        // Revoking the fingerprint of the DECODED bytes must NOT revoke the token.
        let decoded_bytes = URL_SAFE_NO_PAD.decode(&token).unwrap();
        let wrong_fp = hex::encode(Sha256::digest(&decoded_bytes));
        let revoked_wrong = RwLock::new(HashSet::from([wrong_fp]));
        assert!(
            verify_license_key(&sk.verifying_key(), &token, "prod", &revoked_wrong).is_ok(),
            "decoded-bytes fingerprint must not match — revocation keys on the encoded string"
        );

        // Computing it the documented way — SHA256 of the raw base64url STRING —
        // does revoke it.
        let right_fp = hex::encode(Sha256::digest(token.as_bytes()));
        assert_eq!(right_fp, token_fingerprint(&token));
        let revoked_right = RwLock::new(HashSet::from([right_fp]));
        let err = verify_license_key(&sk.verifying_key(), &token, "prod", &revoked_right)
            .err()
            .unwrap();
        assert_eq!(err.reason(), "token-revoked");
    }

    #[test]
    fn verify_expiry_is_string_lexicographic_not_date_parsing() {
        let sk = test_signing_key();
        let vk = sk.verifying_key();

        // "9999-99-99" is NOT a parseable date (month 99) — under date parsing it
        // would error, but lexicographically it exceeds any real YYYY-MM-DD, so
        // the token is accepted. Proves string compare, not chrono parsing.
        let token = make_token(&sk, &payload_json("prod", "9999-99-99"));
        assert!(verify_license_key(&vk, &token, "prod", &no_revocations()).is_ok());

        // "0000-00-00" is likewise unparseable but lexicographically below any
        // real date — always expired.
        let token = make_token(&sk, &payload_json("prod", "0000-00-00"));
        let err = verify_license_key(&vk, &token, "prod", &no_revocations())
            .err()
            .unwrap();
        assert_eq!(err.reason(), "channel-expired");
        assert_eq!(err.status(), StatusCode::FORBIDDEN);

        // Expiring exactly today is still valid (strict `<` comparison).
        let token = make_token(&sk, &payload_json("prod", &today_str()));
        assert!(verify_license_key(&vk, &token, "prod", &no_revocations()).is_ok());

        // Numeric-vs-lexicographic divergence: yesterday's date written with an
        // UNPADDED month (e.g. "2026-7-01") is in the past under date semantics,
        // but lexicographically "7" > "0…" makes it sort AFTER today — so the
        // string compare treats it as unexpired. Only constructible when
        // yesterday is in the same calendar year.
        let now = chrono::Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        if yesterday.format("%Y").to_string() == now.format("%Y").to_string() {
            let unpadded = format!(
                "{}-{}-{}",
                yesterday.format("%Y"),
                yesterday.format("%-m"),
                yesterday.format("%d")
            );
            let token = make_token(&sk, &payload_json("prod", &unpadded));
            assert!(
                verify_license_key(&vk, &token, "prod", &no_revocations()).is_ok(),
                "unpadded past date {unpadded} must be treated as unexpired under \
                 lexicographic compare — if this fails, expiry semantics changed"
            );
        }
    }

    #[test]
    fn verify_success_path_returns_payload() {
        let sk = test_signing_key();
        let payload = r#"{"product":"prod","channel_expiry":"9999-12-31","entitlements":["source","updates"],"version_floor":"0.0.2"}"#;
        let token = make_token(&sk, payload);
        let Ok(p) = verify_license_key(&sk.verifying_key(), &token, "prod", &no_revocations())
        else {
            panic!("valid unexpired non-revoked token for the correct product must verify");
        };
        assert_eq!(p.product, "prod");
        assert_eq!(p.channel_expiry, "9999-12-31");
        assert_eq!(p.entitlements, vec!["source", "updates"]);
        assert_eq!(p.version_floor.as_deref(), Some("0.0.2"));
    }

    #[test]
    fn verify_version_floor_is_optional() {
        let sk = test_signing_key();
        let token = make_token(&sk, &payload_json("prod", "9999-12-31"));
        let Ok(p) = verify_license_key(&sk.verifying_key(), &token, "prod", &no_revocations())
        else {
            panic!("payload without version_floor must verify");
        };
        assert_eq!(p.version_floor, None);
    }

    // ── Route-shape tests (direct handler invocation — no TCP port) ───────────

    #[tokio::test]
    async fn healthz_shape() {
        let Json(body) = healthz().await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["service"], "app-privategit-release");
    }

    #[tokio::test]
    async fn install_script_404_on_missing_file() {
        let scratch = scratch_dir("install");
        let state = test_state(&scratch, None, None);
        let resp = install_script(State(state), HeaderMap::new(), Path("ghost".to_string())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "not found");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn install_script_streams_when_present() {
        let scratch = scratch_dir("install-ok");
        fs::create_dir_all(scratch.join("prod")).unwrap();
        fs::write(scratch.join("prod/install.sh"), "#!/bin/sh\necho hi\n").unwrap();
        let state = test_state(&scratch, None, None);
        let resp = install_script(State(state), HeaderMap::new(), Path("prod".to_string())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()[header::CONTENT_TYPE].to_str().unwrap(),
            "text/x-shellscript"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn git_stub_returns_503_shape() {
        let (status, Json(body)) = git_stub().await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(body["error"], "smart-HTTP Git not yet enabled");
        assert_eq!(body["code"], "not-yet-enabled");
        assert!(body["see"].is_string());
        assert!(body["arriving"].is_string());
    }

    // ── /admin/reload-revocation-list ─────────────────────────────────────────

    fn loopback() -> ConnectInfo<SocketAddr> {
        ConnectInfo("127.0.0.1:54321".parse().unwrap())
    }

    fn non_loopback() -> ConnectInfo<SocketAddr> {
        ConnectInfo("203.0.113.9:54321".parse().unwrap())
    }

    #[tokio::test]
    async fn reload_revocation_rejects_non_loopback() {
        let scratch = scratch_dir("reload-remote");
        let list = scratch.join("revoked.txt");
        fs::write(&list, "").unwrap();
        let state = test_state(&scratch, None, Some(list.to_string_lossy().into_owned()));
        let resp = reload_revocation_list(non_loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "admin endpoints are localhost-only");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_404_when_not_configured() {
        let scratch = scratch_dir("reload-nopath");
        let state = test_state(&scratch, None, None);
        let resp = reload_revocation_list(loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "no revocation list configured");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_enoent_is_404_other_io_error_is_500() {
        let scratch = scratch_dir("reload-errs");

        // ENOENT — configured path does not exist -> 404, distinct message.
        let missing = scratch.join("ghost.txt");
        let state = test_state(&scratch, None, Some(missing.to_string_lossy().into_owned()));
        let resp = reload_revocation_list(loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "revocation list file not found");

        // Other I/O error — path is a DIRECTORY, read_to_string fails but not
        // with NotFound -> 500.
        let dir_path = scratch.join("a-directory");
        fs::create_dir_all(&dir_path).unwrap();
        let state = test_state(
            &scratch,
            None,
            Some(dir_path.to_string_lossy().into_owned()),
        );
        let resp = reload_revocation_list(loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_success_replaces_state() {
        let scratch = scratch_dir("reload-ok");
        let list = scratch.join("revoked.txt");
        let fp = token_fingerprint("some-token");
        fs::write(&list, format!("# header\n{fp}\n")).unwrap();
        let state = test_state(&scratch, None, Some(list.to_string_lossy().into_owned()));
        // Pre-seed with a stale entry that must be REPLACED, not merged.
        state.revoked_tokens.write().unwrap().insert("f".repeat(64));

        let resp = reload_revocation_list(loopback(), State(state.clone())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        assert_eq!(body["reloaded"], 1);
        let now = state.revoked_tokens.read().unwrap();
        assert_eq!(now.len(), 1);
        assert!(now.contains(&fp));
        let _ = fs::remove_dir_all(&scratch);
    }

    // S11: a panic while holding the write lock must not turn every subsequent
    // request into a panic loop. Poison the lock deliberately, then prove both the
    // read path (verify_license_key's revocation check) and the write path
    // (reload_revocation_list) still function afterward.
    #[tokio::test]
    async fn revocation_lock_recovers_after_poison() {
        let scratch = scratch_dir("poison-recover");
        let list = scratch.join("revoked.txt");
        let fp = token_fingerprint("some-token");
        fs::write(&list, format!("{fp}\n")).unwrap();
        let state = test_state(&scratch, None, Some(list.to_string_lossy().into_owned()));

        // Poison the lock: panic on another thread while holding the write guard.
        let lock = state.revoked_tokens.clone();
        let _ = std::thread::spawn(move || {
            let _guard = lock.write().unwrap();
            panic!("intentional poison for revocation_lock_recovers_after_poison");
        })
        .join();
        assert!(state.revoked_tokens.is_poisoned());

        // Read path recovers instead of panicking.
        let revoked = state
            .revoked_tokens
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        drop(revoked);

        // Write path (the actual production call site) also recovers.
        let resp = reload_revocation_list(loopback(), State(state.clone())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let now = state
            .revoked_tokens
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(now.contains(&fp));
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── POST /verify-key endpoint ─────────────────────────────────────────────

    #[tokio::test]
    async fn verify_key_endpoint_503_when_key_not_configured() {
        let scratch = scratch_dir("vke-503");
        let state = test_state(&scratch, None, None);
        let (status, Json(body)) = verify_key_endpoint(
            loopback(),
            State(state),
            HeaderMap::new(),
            Json(VerifyKeyRequest {
                license_key_b64: "anything".into(),
                product_id: "prod".into(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert!(body["error"].as_str().unwrap().contains("VERIFY_KEY_PUB"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn verify_key_endpoint_success_and_expired_shapes() {
        let scratch = scratch_dir("vke-ok");
        let sk = test_signing_key();
        let state = test_state(&scratch, Some(sk.verifying_key()), None);

        // Success shape.
        let token = make_token(&sk, &payload_json("prod", "9999-12-31"));
        let (status, Json(body)) = verify_key_endpoint(
            loopback(),
            State(state.clone()),
            HeaderMap::new(),
            Json(VerifyKeyRequest {
                license_key_b64: token,
                product_id: "prod".into(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["valid"], true);
        assert_eq!(body["product"], "prod");
        assert_eq!(body["channel_expiry"], "9999-12-31");
        assert_eq!(body["entitlements"], json!(["source"]));

        // Expired shape carries the expired date back.
        let token = make_token(&sk, &payload_json("prod", "2020-01-01"));
        let (status, Json(body)) = verify_key_endpoint(
            loopback(),
            State(state),
            HeaderMap::new(),
            Json(VerifyKeyRequest {
                license_key_b64: token,
                product_id: "prod".into(),
            }),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body["valid"], false);
        // S15 fix: was "channel expired" (space) — now matches LicenseVerifyErr's
        // stable kebab-case code, same as every other reason value this endpoint
        // returns.
        assert_eq!(body["reason"], "channel-expired");
        assert_eq!(body["expired"], "2020-01-01");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Rate limiting (S6) ──────────────────────────────────────────────────

    #[test]
    fn rate_limiter_allows_up_to_max_then_blocks() {
        let rl = RateLimiter::new(3, std::time::Duration::from_secs(60));
        let ip: std::net::IpAddr = "203.0.113.9".parse().unwrap();
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_ok());
        let err = rl.check(ip).unwrap_err();
        assert!(err >= 1, "retry_after should be at least 1 second");
    }

    #[test]
    fn rate_limiter_tracks_ips_independently() {
        let rl = RateLimiter::new(1, std::time::Duration::from_secs(60));
        let a: std::net::IpAddr = "203.0.113.1".parse().unwrap();
        let b: std::net::IpAddr = "203.0.113.2".parse().unwrap();
        assert!(rl.check(a).is_ok());
        assert!(rl.check(a).is_err());
        // A different IP is unaffected by A's limit.
        assert!(rl.check(b).is_ok());
    }

    #[test]
    fn rate_limiter_zero_max_requests_denies_without_panicking() {
        // L1: a `max_requests: 0` config used to panic on the very first `check()`
        // call -- `queue.front().unwrap()` on a queue that had never had anything
        // pushed to it. With `panic = "abort"` that would have killed the process.
        let rl = RateLimiter::new(0, std::time::Duration::from_secs(60));
        let ip: std::net::IpAddr = "203.0.113.9".parse().unwrap();
        let err = rl.check(ip).unwrap_err();
        assert!(err >= 1, "retry_after should be at least 1 second");
    }

    #[tokio::test]
    async fn verify_key_endpoint_429_when_rate_limited() {
        let scratch = scratch_dir("vke-429");
        let sk = test_signing_key();
        let state = test_state_with_public_rate_limit(&scratch, Some(sk.verifying_key()), 1);

        let req = || VerifyKeyRequest {
            license_key_b64: "anything".into(),
            product_id: "prod".into(),
        };
        let (first_status, _) = verify_key_endpoint(
            loopback(),
            State(state.clone()),
            HeaderMap::new(),
            Json(req()),
        )
        .await;
        assert_ne!(first_status, StatusCode::TOO_MANY_REQUESTS);

        let (status, Json(body)) =
            verify_key_endpoint(loopback(), State(state), HeaderMap::new(), Json(req())).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(body["error"], "rate limited");
        assert!(body["retry_after_seconds"].as_u64().unwrap() >= 1);
        let _ = fs::remove_dir_all(&scratch);
    }

    // C2 regression: binary()'s own signature-verification step (the route
    // genuinely reachable through this host's nginx, unlike /verify-key) must
    // also be rate limited, sharing the PUBLIC bucket -- not the admin bucket.
    #[tokio::test]
    async fn binary_429_when_rate_limited_before_signature_verification() {
        let scratch = scratch_dir("binary-429");
        let sk = test_signing_key();
        fs::create_dir_all(scratch.join("prod")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": true}"#,
        )
        .unwrap();
        let state = test_state_with_public_rate_limit(&scratch, Some(sk.verifying_key()), 1);

        let call = |state: Arc<AppState>| {
            binary(
                loopback(),
                State(state),
                HeaderMap::new(),
                Path((
                    "prod".to_string(),
                    "1.0.0".to_string(),
                    "linux-x86_64".to_string(),
                )),
                Query(BinaryQuery {
                    token: Some("anything".to_string()),
                }),
            )
        };
        let first = call(state.clone()).await;
        assert_ne!(first.status(), StatusCode::TOO_MANY_REQUESTS);

        let resp = call(state).await;
        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
        let body = body_json(resp).await;
        assert_eq!(body["code"], "rate-limited");
        let _ = fs::remove_dir_all(&scratch);
    }

    // C2 regression: exhausting the PUBLIC bucket must never affect the ADMIN
    // bucket -- this is the exact bug the audit demonstrated live (20x
    // /verify-key calls made the admin revocation-reload return 429 too).
    #[tokio::test]
    async fn public_rate_limit_never_affects_admin_bucket() {
        let scratch = scratch_dir("c2-buckets-independent");
        let list = scratch.join("revoked.txt");
        fs::write(&list, "").unwrap();
        let sk = test_signing_key();
        let mut state =
            (*test_state_with_public_rate_limit(&scratch, Some(sk.verifying_key()), 1)).clone();
        state.revocation_list_path = Some(list.to_string_lossy().into_owned());
        let state = Arc::new(state);

        let req = || VerifyKeyRequest {
            license_key_b64: "anything".into(),
            product_id: "prod".into(),
        };
        // Exhaust the public bucket.
        let _ = verify_key_endpoint(
            loopback(),
            State(state.clone()),
            HeaderMap::new(),
            Json(req()),
        )
        .await;
        let (exhausted, _) = verify_key_endpoint(
            loopback(),
            State(state.clone()),
            HeaderMap::new(),
            Json(req()),
        )
        .await;
        assert_eq!(exhausted, StatusCode::TOO_MANY_REQUESTS);

        // Admin endpoint must be entirely unaffected.
        let admin_resp = reload_revocation_list(loopback(), State(state)).await;
        assert_ne!(admin_resp.status(), StatusCode::TOO_MANY_REQUESTS);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_list_429_when_rate_limited() {
        let scratch = scratch_dir("rrl-429");
        let list = scratch.join("revoked.txt");
        fs::write(&list, "").unwrap();
        let state = test_state_with_admin_rate_limit(&scratch, 1);
        // Patch in the revocation list path the generic helper doesn't set.
        let mut state = (*state).clone();
        state.revocation_list_path = Some(list.to_string_lossy().into_owned());
        let state = Arc::new(state);

        let first = reload_revocation_list(loopback(), State(state.clone())).await;
        assert_ne!(first.status(), StatusCode::TOO_MANY_REQUESTS);

        let resp = reload_revocation_list(loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(resp.headers().get(header::RETRY_AFTER).is_some());
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn reload_revocation_list_non_loopback_rejected_before_rate_limit_check() {
        // Loopback gate must fire first -- an attacker spoofing a non-loopback
        // source shouldn't be able to probe the rate limiter's state at all.
        let scratch = scratch_dir("rrl-loopback-first");
        let state = test_state_with_admin_rate_limit(&scratch, 1);
        let resp = reload_revocation_list(non_loopback(), State(state)).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── GET /verify-key.pub ───────────────────────────────────────────────────

    #[tokio::test]
    async fn verify_key_pub_shapes() {
        let scratch = scratch_dir("vkp");
        let sk = test_signing_key();

        let state = test_state(&scratch, Some(sk.verifying_key()), None);
        let resp = verify_key_pub(State(state)).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(
            String::from_utf8(bytes.to_vec()).unwrap(),
            hex::encode(sk.verifying_key().to_bytes())
        );

        let state = test_state(&scratch, None, None);
        let resp = verify_key_pub(State(state)).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── GET /releases/:p/:v/:platform (binary handler auth flow) ──────────────

    #[tokio::test]
    async fn binary_requires_auth_when_license_required() {
        let scratch = scratch_dir("bin-401");
        let sk = test_signing_key();
        let state = test_state(&scratch, Some(sk.verifying_key()), None);
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64".to_string(),
            )),
            Query(BinaryQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "license key required");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn binary_503_when_verify_key_unconfigured() {
        let scratch = scratch_dir("bin-503");
        let state = test_state(&scratch, None, None);
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64".to_string(),
            )),
            Query(BinaryQuery {
                token: Some("sometoken".into()),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn binary_open_product_serves_without_auth() {
        let scratch = scratch_dir("bin-open");
        let prod = scratch.join("prod");
        fs::create_dir_all(prod.join("0.0.1")).unwrap();
        fs::write(prod.join("MANIFEST.json"), r#"{"requires_license": false}"#).unwrap();
        fs::write(prod.join("0.0.1/linux-x86_64"), b"binary-bytes").unwrap();
        let state = test_state(&scratch, None, None);
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64".to_string(),
            )),
            Query(BinaryQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()[header::CONTENT_TYPE].to_str().unwrap(),
            "application/octet-stream"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn binary_sig_files_are_unauthenticated() {
        let scratch = scratch_dir("bin-sig");
        // Product REQUIRES a license, but detached .sig files are served openly.
        let prod = scratch.join("prod");
        fs::create_dir_all(prod.join("0.0.1")).unwrap();
        fs::write(prod.join("0.0.1/linux-x86_64.sig"), b"detached-sig").unwrap();
        let state = test_state(&scratch, None, None);
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64.sig".to_string(),
            )),
            Query(BinaryQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn binary_valid_token_but_missing_file_is_404() {
        let scratch = scratch_dir("bin-404");
        let sk = test_signing_key();
        let state = test_state(&scratch, Some(sk.verifying_key()), None);
        let token = make_token(&sk, &payload_json("prod", "9999-12-31"));
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64".to_string(),
            )),
            Query(BinaryQuery { token: Some(token) }),
        )
        .await;
        // Auth passed; the file simply is not there.
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = body_json(resp).await;
        // Error shape now comes from the shared stream_file() 404, not a
        // route-specific message (S10 refactor onto tower_http::ServeFile).
        assert_eq!(body["error"], "not found");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn binary_revoked_token_is_403() {
        let scratch = scratch_dir("bin-revoked");
        let sk = test_signing_key();
        let state = test_state(&scratch, Some(sk.verifying_key()), None);
        let token = make_token(&sk, &payload_json("prod", "9999-12-31"));
        state
            .revoked_tokens
            .write()
            .unwrap()
            .insert(token_fingerprint(&token));
        let resp = binary(
            loopback(),
            State(state),
            HeaderMap::new(),
            Path((
                "prod".to_string(),
                "0.0.1".to_string(),
                "linux-x86_64".to_string(),
            )),
            Query(BinaryQuery { token: Some(token) }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let body = body_json(resp).await;
        assert_eq!(body["error"], "token-revoked");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Router-level: proves the traversal fix holds through REAL routing
    // (matchit segment matching + axum's percent-decoding), not just the handler
    // function called directly — this is the gap S16 flagged as untested.

    async fn oneshot_get(state: Arc<AppState>, uri: &str) -> Response {
        use http_body_util::BodyExt;
        use tower::ServiceExt;
        let router = build_router(state);
        let request = axum::http::Request::builder()
            .uri(uri)
            .extension(loopback())
            .body(axum::body::Body::empty())
            .unwrap();
        let resp = router.oneshot(request).await.unwrap();
        let (parts, body) = resp.into_parts();
        let bytes = body.collect().await.unwrap().to_bytes();
        Response::from_parts(parts, axum::body::Body::from(bytes))
    }

    #[tokio::test]
    async fn router_rejects_dot_dot_traversal_in_binary_route() {
        let scratch = scratch_dir("router-traversal-dotdot");
        // A file OUTSIDE releases_dir a traversal attempt would try to read.
        let secret_path = scratch.parent().unwrap().join("secret-router-test.txt");
        fs::write(&secret_path, b"top secret").unwrap();
        fs::create_dir_all(scratch.join("prod/0.0.1")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false}"#,
        )
        .unwrap();
        let state = test_state(&scratch, None, None);

        let resp = oneshot_get(
            state,
            "/releases/prod/..%2f..%2fsecret-router-test.txt/linux-x86_64",
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let _ = fs::remove_file(&secret_path);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn router_rejects_encoded_slash_in_product_segment() {
        let scratch = scratch_dir("router-traversal-slash");
        fs::create_dir_all(scratch.join("prod/0.0.1")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false}"#,
        )
        .unwrap();
        let state = test_state(&scratch, None, None);

        // %2F decodes to a literal '/' inside what matchit treated as one segment.
        let resp = oneshot_get(state, "/releases/..%2Fetc/0.0.1/passwd").await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn router_rejects_nul_byte_in_segment() {
        let scratch = scratch_dir("router-traversal-nul");
        fs::create_dir_all(scratch.join("prod/0.0.1")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false}"#,
        )
        .unwrap();
        let state = test_state(&scratch, None, None);

        let resp = oneshot_get(state, "/releases/prod/..%00/linux-x86_64").await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn router_serves_legitimate_open_binary_unaffected() {
        let scratch = scratch_dir("router-legit");
        fs::create_dir_all(scratch.join("prod/0.0.1")).unwrap();
        fs::write(
            scratch.join("prod/MANIFEST.json"),
            r#"{"requires_license": false}"#,
        )
        .unwrap();
        fs::write(
            scratch.join("prod/0.0.1/linux-x86_64"),
            b"real-binary-bytes",
        )
        .unwrap();
        let state = test_state(&scratch, None, None);

        let resp = oneshot_get(state, "/releases/prod/0.0.1/linux-x86_64").await;
        assert_eq!(resp.status(), StatusCode::OK);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── is_safe_segment: direct table, not just exercised through router tests ────
    // (gap flagged by this session's live adversarial re-verification pass)

    #[test]
    fn is_safe_segment_accepts_ordinary_identifiers() {
        for s in ["prod", "0.0.1", "linux-x86_64", "os-console", "beta"] {
            assert!(is_safe_segment(s), "{s:?} should be accepted");
        }
    }

    #[test]
    fn is_safe_segment_rejects_traversal_and_separators() {
        for s in ["..", ".", "../etc", "a/b", "a\\b", "", "/etc"] {
            assert!(!is_safe_segment(s), "{s:?} should be rejected");
        }
    }

    #[test]
    fn is_safe_segment_rejects_control_characters_including_nul() {
        // Live-confirmed gap this session: a NUL-bearing segment (from `%00`) cleared
        // the separator/`..`-only blocklist and reached `File::open`; only the OS
        // syscall boundary's rejection of interior NUL prevented a real leak.
        for s in ["..\u{0}", "prod\u{0}", "\u{0}", "a\tb", "a\nb", "a\rb"] {
            assert!(!is_safe_segment(s), "{s:?} should be rejected");
        }
    }

    #[test]
    fn is_safe_segment_rejects_del() {
        // L6: the control-char band excluded 0x00-0x1F but not 0x7F (DEL) --
        // also a control character, just outside the C0 block.
        for s in ["\u{7f}", "prod\u{7f}", "a\u{7f}b"] {
            assert!(!is_safe_segment(s), "{s:?} should be rejected");
        }
    }
}
