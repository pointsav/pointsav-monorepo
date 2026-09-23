// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

// Security headers applied to every response. CSP allows 'unsafe-inline' for
// script-src/style-src because this crate's own architecture serves CSS and a
// couple of small interactive scripts (SHA256 verification fetch, install-command
// copy) as inline <style>/<script> blocks rather than external assets (see
// ui::layout::head, ui::product_detail, ui::catalog) — there are no nonces or
// hashes wired through the render pipeline to tighten this further, and no
// external CDN/font dependency that would need its own allowance.
const HSTS_VALUE: &str = "max-age=63072000; includeSubDomains";

// S2/M10 fix (part 2 of 2 — part 1 is `manifest()`'s product-root fallback in
// app-privategit-source): the product-detail page's client-side SHA256 fetch
// (`ui::product_detail::sha_fetch_script`) hits `SOURCE_BASE_URL` directly, which is
// a different origin than this marketplace on every host except prod (where a
// front-proxy happens to collapse them). `default-src 'self'` alone blocks that
// fetch outright — live-confirmed console errors on any non-prod host. Rather than
// adding a same-origin proxy route (a new marketplace→source network coupling this
// crate's own docs deliberately avoid — see `product_detail.rs`'s module doc), CSP
// gets one explicit `connect-src` allowance for the known, trusted source origin.
fn csp_value(source_base_url: &str) -> String {
    format!(
        "default-src 'self'; script-src 'self' 'unsafe-inline'; \
         style-src 'self' 'unsafe-inline'; img-src 'self' data:; \
         connect-src 'self' {}; frame-ancestors 'none'; base-uri 'self'",
        url_origin(source_base_url)
    )
}

/// Extracts `scheme://host[:port]` from a full URL — avoids pulling in a
/// URL-parsing dependency for this one string operation. Falls back to the input
/// unchanged if it doesn't look like `scheme://host/...` (defensive; `connect-src`
/// simply grants no extra origin in that case, it doesn't break anything else).
fn url_origin(url: &str) -> String {
    match url.find("://") {
        Some(scheme_end) => {
            let after_scheme = &url[scheme_end + 3..];
            let host_end = after_scheme.find('/').unwrap_or(after_scheme.len());
            url[..scheme_end + 3 + host_end].to_string()
        }
        None => url.to_string(),
    }
}

mod ui;
use ui::{Lang, SoftwareSurface};

// ── State ─────────────────────────────────────────────────────────────────────
//
// The payment-config fields (`polygon_wallet_address`, `receipts_dir`,
// `claims_dir`, `polygon_rpc_url`, `tool_wallet_bin`) were wired as placeholders
// in P1 and are consumed by the P4 license/claim/wallet handlers below.
#[derive(Clone)]
struct AppState {
    catalog_path: PathBuf,
    // Static-HTML source of truth (single source: the on-disk directory). Both
    // /software and /licensing read from this directory at request time, and
    // /static/* mounts the same directory via ServeDir. Nothing is baked with
    // include_str! — see BRIEF/report for the rationale.
    static_dir: PathBuf,
    // ── P4 payment config ───────────────────────────────────────────────────
    polygon_wallet_address: String,
    receipts_dir: PathBuf,
    claims_dir: PathBuf,
    source_base_url: String,
    polygon_rpc_url: String,
    // Name (or absolute path) of the `tool-wallet` binary shelled out to by
    // `v1_license`. Defaults to `"tool-wallet"` (resolved via PATH, matching the
    // OLD crate's `Command::new("tool-wallet")`). Overridable via the
    // `TOOL_WALLET_BIN` env var so tests can inject a JSON test-double without a
    // real Polygon RPC call. Production behaviour is unchanged.
    tool_wallet_bin: String,
    // ── Phase 2: real download-token minting ────────────────────────────────
    // The private counterpart to `app-privategit-source-2`'s `VERIFY_KEY_PUB` —
    // loaded from `SIGNING_KEY_SECRET` (same hex-seed-or-file-path convention as
    // that crate's `load_verify_key`). `None` until a production key is
    // provisioned (a deployment-time secrets step, not decided by this code).
    signing_key: Option<SigningKey>,
    // ── Phase 5: CRA barter-transaction log ─────────────────────────────────
    // `data/software-catalog/tx-log.jsonl`'s local-crate counterpart, per
    // `BRIEF-software-distribution-substrate.md` §8 — required from the first
    // real sale.
    tx_log_path: PathBuf,
    // Placeholder USDC->CAD spot rate (no live FX feed is wired into this crate;
    // building one is out of scope for this cleanup). Configurable via
    // `USDC_CAD_SPOT_RATE` so production can update it without a code change —
    // see `append_tx_log`'s doc comment.
    usdc_cad_spot_rate: f64,
}

// ── Catalog types ─────────────────────────────────────────────────────────────
//
// Rebuilt per `BRIEF-software-hyperscaler-audit.md`'s Licensing Corrections section
// (factory-release-engineering/LICENSE-MATRIX.md is authoritative). The prior
// `licenses:` list conflated two unrelated things — two license *terms* (mislabeled
// "Apache 2.0"/"FSL") and five unrelated free *products* — with an `installers:` list
// that was already correctly modeled. Each os-* product has exactly ONE fixed tier
// (not a customer choice), so the fix is additive fields on `Installer`, not a new
// products/terms-with-references model.

/// The ratified license tiers.
///
/// **Rebuilt 2026-07-07** to match `BRIEF-software-licensing-structure.md`
/// (Command Session, ratified 2026-07-07) — the authoritative per-product tier
/// review that superseded the three-tier `Commercial`/`Fsl`/`OpenSource` model
/// this enum used to carry. That BRIEF's own per-product table never uses
/// "PointSav Commercial" as a license category — it reclassifies every `os-*`
/// product into one of exactly four real tiers, each a real license identifier:
///
/// - `Proprietary` — permanent, no source grant. `os-orchestration` +
///   `app-orchestration-*` only (the company's stated commercial moat).
/// - `Fsl` — FSL-1.1-ALv2. Source-readable now, converts to Apache-2.0 two years
///   after each release. `os-infrastructure`, `os-network-admin`, `os-mediakit`,
///   `os-totebox`, the `os-privategit` engine, and the `os-workplace` `moonshot-*`
///   engine crates.
/// - `Agpl` — AGPL-3.0-or-later, this workspace's deliberate backend default
///   (kept, not replaced — see that BRIEF's Decision 1). Includes `os-console` +
///   `app-console-*`, the flagship buy-to-own product: its BRIEF explicitly
///   calls "Commercial" a *pricing* state ("Commercial tier when priced"), not a
///   separate license — the license underneath is AGPL.
/// - `Apache` — a genuine, unconditional Apache-2.0 grant, permanently free
///   ($0 forever). `pointsav-design-system`, `woodfine-bim-library`. **No
///   `tool-*` exception exists here** — `tool-wallet` previously carried a
///   named Apache exception to the `tool-*` AGPL default; that exception is
///   reversed under the current licensing architecture. `tool-wallet` is
///   AGPL-3.0-or-later + PointSav-Commercial like every other `tool-*`
///   product, no carve-outs.
///
/// Per that BRIEF §4, **every catalogued product is priced at $0 USDC (BETA) for
/// now, regardless of tier** — pricing and licensing are independent decisions,
/// and no non-zero future price has been ratified for any tier (see
/// `Installer::price_usdc`, not this enum, for the one number that's real).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum LicenseTier {
    Proprietary,
    Fsl,
    Agpl,
    Apache,
}

impl LicenseTier {
    /// Display label — the real SPDX-recognizable identifier for every tier
    /// except `Proprietary` (which has none). `Apache` carries a shelf-clarifying
    /// suffix since it's the one tier that's permanently, unconditionally free.
    fn label(self) -> &'static str {
        match self {
            LicenseTier::Proprietary => "Proprietary",
            LicenseTier::Fsl => "FSL-1.1-ALv2",
            LicenseTier::Agpl => "AGPL-3.0-or-later",
            LicenseTier::Apache => "Apache-2.0 (Open Source)",
        }
    }

    /// Which Binary Library shelf this tier belongs to
    /// (`BRIEF-binary-library-repositioning.md`'s two-shelf model, operator-approved
    /// 2026-07-07). `Proprietary`/`Fsl`/`Agpl` are all the existing, ratified
    /// os-*-only public catalog — unchanged. `Apache` is the Open Source shelf,
    /// populated only as individual crates are actually relicensed (Phase 2),
    /// never inferred. Read by `v1_products` (JSON `shelf` field) only —
    /// **Phase 1b:** `ui::catalog::catalog_markup` no longer groups `/software` by
    /// shelf/license tier; architecture tier (`ArchTier`) is the page's only
    /// grouping axis now, license tier moved to a per-card fact row. `Shelf` and
    /// this method stay live for the JSON API's own `shelf` field. Kept alongside
    /// the tier it derives from rather than a separate, independently-settable
    /// field, so shelf membership can never drift out of sync with `license_tier`
    /// itself.
    fn shelf(self) -> Shelf {
        match self {
            LicenseTier::Proprietary | LicenseTier::Fsl | LicenseTier::Agpl => Shelf::Commercial,
            LicenseTier::Apache => Shelf::OpenSource,
        }
    }
}

/// The two Binary Library shelves. See [`LicenseTier::shelf`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shelf {
    Commercial,
    OpenSource,
}

impl Shelf {
    /// Wire-format value for the `/v1/products` JSON `shelf` field.
    fn as_str(self) -> &'static str {
        match self {
            Shelf::Commercial => "commercial",
            Shelf::OpenSource => "open-source",
        }
    }
}

/// What kind of thing a customer actually does with a downloaded artifact.
/// `CliBinary` (the default, and every product in the catalog until this field was
/// added) is a regular executable: curl-pipe-sh install, then run directly.
/// `ApplianceImage` is a bootable VM image (seL4/Microkit appliance, qcow2 disk,
/// etc.) — the customer boots it under a hypervisor, there is nothing to "install."
/// Deliberately a plain unit enum with no payload: richer per-kind structured data
/// (e.g. a typed boot-config) is not modeled here — see `Installer::boot_notes`'s
/// doc comment for why freeform prose is the right shape for a sample size of two.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
enum ArtifactKind {
    #[default]
    CliBinary,
    ApplianceImage,
}

/// Which of the three real, ratified architecture tiers (per `project-totebox`'s own
/// engineering-status table — Infrastructure / Platform / Delivery, not invented
/// vocabulary) a product belongs to. Purely a display concern: shown as a small badge
/// alongside the existing license-tier badge so a visitor can see how the products
/// relate to each other. Has no bearing on pricing or licensing — those stay exactly
/// as they are today. **Phase 1b:** required, not optional — it's `/software`'s
/// *only* grouping axis now (replacing the earlier two-shelf Commercial/Open-Source
/// model), so a product missing it would render nowhere at all rather than just
/// missing a badge. See `catalog::catalog_markup`'s module doc for the full
/// rationale behind that replacement.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ArchTier {
    Infrastructure,
    Platform,
    Delivery,
}

impl ArchTier {
    fn label(self) -> &'static str {
        match self {
            ArchTier::Infrastructure => "Infrastructure",
            ArchTier::Platform => "Platform",
            ArchTier::Delivery => "Delivery",
        }
    }

    /// Section `id=`/rail-anchor slug — kept beside `label()` so a fourth tier
    /// cannot be added without also giving it a slug.
    fn slug(self) -> &'static str {
        match self {
            ArchTier::Infrastructure => "infrastructure",
            ArchTier::Platform => "platform",
            ArchTier::Delivery => "delivery",
        }
    }
}

/// Whether a product is part of the Totebox Orchestration fleet-pairing family
/// (needs a paired archive/Totebox Orchestration to do anything useful — e.g.
/// os-console's F11 pairing, app-orchestration-command's fleet API hub, os-totebox's
/// per-archive DataGraph) or is a genuinely independent system a customer can run in
/// total isolation (os-privategit, os-mediakit, app-privategit-design — each ships
/// its own self-host story with no fleet/archive dependency). A real architectural
/// fault line in the product family, not a marketing label — confirmed per-product
/// against each one's own description before assigning it a variant, not inferred
/// from its `os-`/`app-` prefix (prefix alone doesn't determine this: app-orchestration-*
/// is fleet-coupled despite the `app-` prefix; os-mediakit/os-privategit are
/// independent despite the `os-` prefix). A second, orthogonal grouping axis to
/// `ArchTier` — rendered as a badge, not a page section, so `ArchTier` remains the
/// page's only *sectioning* axis per catalog.rs's module doc.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ProductFamily {
    ToteboxOrchestration,
    Independent,
}

impl ProductFamily {
    fn label(self) -> &'static str {
        match self {
            ProductFamily::ToteboxOrchestration => "Totebox Orchestration",
            ProductFamily::Independent => "Independent System",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Installer {
    id: String,
    name: String,
    description: String,
    edition: String,
    platform: String,
    size_mb: u64,
    path: String,
    /// Ratified tier label — always set correctly regardless of BETA status; this is
    /// legal/display metadata, not a payment gate. See `price_usdc` for the gate.
    license_tier: LicenseTier,
    /// The active price. `0` = active BETA gate (no payment/license flow triggered,
    /// same curl-pipe-sh pattern as any other free product — no separate `beta: true`
    /// flag needed, matching this workspace's established BETA convention). Every
    /// product ships at `0` initially per an explicit, current operator/Command
    /// directive (os-console, os-mediakit, and the orchestration-command binary all
    /// carry an active "stay free during BETA" instruction in `.agent/inbox.md` as of
    /// 2026-07-01/02, and none has been lifted yet). Flipping a specific product to
    /// its real tier price (1_000_000 for `commercial`, 19_000_000 for `fsl`) is a
    /// one-line data change once Command sends an explicit per-product BETA-lifted
    /// message — not a code change.
    price_usdc: u64,
    /// Phase 5: the date (`YYYY-MM-DD`) this specific version's FSL term
    /// automatically converts to Apache 2.0 (two years after release, per
    /// FSL-1.1-ALv2 — see `LICENSE-MATRIX.md`). Populated manually per release;
    /// no release-date data exists in this catalog to derive it automatically.
    /// Meaningless for `commercial`-tier products (always `None` there).
    /// Read by `xtask fsl-clock`, not by any route in this crate.
    #[serde(default)]
    fsl_conversion_date: Option<String>,
    /// S136: link to the product's operational GUIDE, when one exists.
    /// Genuinely optional — the original request's own wording was "GUIDE link
    /// (when available)". No `products.yaml` entry populates this yet; the
    /// product detail page (`ui::product_detail`) simply omits the row when
    /// `None` rather than fabricating a URL.
    #[serde(default)]
    guide_url: Option<String>,
    /// `CliBinary` (default) or `ApplianceImage` — see `ArtifactKind`'s doc comment.
    #[serde(default)]
    artifact: ArtifactKind,
    /// The literal filename this product's binary/image is deposited under on the
    /// release server, and therefore the `:platform` segment of its download URL
    /// (`/releases/:product/:version/:platform` — that's a real path segment, not
    /// a display label). Defaults to `"linux-x86_64"`, matching every product that
    /// existed before this field did. Deliberately separate from `platform`
    /// (above), which is a human-readable *display* string shown in a visible
    /// badge — the two are allowed to differ (e.g. `platform: "Linux x86_64"`,
    /// `platform_slug: "x86_64"`) and changing one must never silently change the
    /// other.
    #[serde(default)]
    platform_slug: Option<String>,
    /// Freeform prose shown in place of the install-command block for
    /// `ArtifactKind::ApplianceImage` products — the real boot procedure (qemu
    /// invocation, any required companion files, real caveats), not structured
    /// fields. Deliberately not typed: at a sample size of one or two appliance
    /// products, a typed boot-config schema would be modeling data we don't yet
    /// know the shape of. Revisit as structure once a third, differently-shaped
    /// appliance product makes the need concrete. `None` for `CliBinary` products.
    #[serde(default)]
    boot_notes: Option<String>,
    /// Which architecture tier this product belongs to — see `ArchTier`'s doc comment.
    /// **Required, not `Option`** (Phase 1b) — this is the page's *only* grouping
    /// axis now, so a product missing it would render nowhere at all rather than
    /// just missing a badge. A `products.yaml` entry without `tier:` now fails
    /// `load_catalog` loudly at parse time (a real compiler/parse-time anti-drift
    /// guarantee), matching this struct's `Catalog`'s existing `deny_unknown_fields`
    /// philosophy rather than relying on a runtime-only test to catch the omission.
    tier: ArchTier,
    /// Totebox-Orchestration-coupled vs. independent system — see `ProductFamily`'s
    /// doc comment. Required, not `Option`, for the same reason `tier` is: a real,
    /// verified fact about the product, not a cosmetic label that's fine to omit.
    family: ProductFamily,
    /// One-line, freeform description of what installing this product turns a machine
    /// *into* (e.g. "Turns a machine into your fleet's command server.") — shown
    /// beneath the product name on the card and detail page. Deliberately freeform
    /// prose, not structured: this is copy, not data with a schema. `None` renders no
    /// extra line at all.
    #[serde(default)]
    becomes: Option<String>,
    /// Up to 3 labelled spec-style facts shown as a card's fact-row block (Phase 1b —
    /// the "substantial through structure, not prose length" mechanism). Capped at 3
    /// and validated in `load_catalog` (bail, not silently truncate) — the concrete
    /// guardrail against a card slowly turning into an unbounded spec sheet, the
    /// exact drift pattern that makes real marketplace listings bloat over time. A
    /// 4th, always-derived "Distribution" row (artifact kind/size/edition/license)
    /// is computed at render time, never authored here, so it can't drift from the
    /// rest of this struct's own fields.
    #[serde(default)]
    facts: Vec<Fact>,
    /// Other product IDs a purchase of this product also grants registry access to
    /// — e.g. an appliance that bundles another product's functionality as a
    /// managed child process (app-orchestration-command bundling
    /// app-orchestration-slm's broker) grants the buyer both. Not itself a storefront
    /// listing field (no card UI reads this) — consumed only by `order_download`'s
    /// token-minting to widen `LicensePayload.entitled_products`. Empty for every
    /// product that doesn't bundle anything else, which is most of them.
    #[serde(default)]
    bundled_registry_products: Vec<String>,
}

/// One labelled fact row — see `Installer::facts`'s doc comment for the cap/
/// validation rationale. Deliberately two plain strings, not an enum of fact
/// kinds: the set of fact types varies per product and isn't worth modeling.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Fact {
    label: String,
    value: String,
}

impl Installer {
    /// The real on-disk filename / download-URL platform segment — see
    /// `platform_slug`'s doc comment for why this is distinct from the `platform`
    /// display field.
    fn platform_slug(&self) -> &str {
        self.platform_slug.as_deref().unwrap_or("linux-x86_64")
    }
}

// `deny_unknown_fields`: a stray legacy `licenses:` key (from a pre-migration
// products.yaml) must fail to parse loudly (500 on every catalog-backed route),
// not silently produce an empty-but-successfully-parsed catalog.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    installers: Vec<Installer>,
}

fn load_catalog(catalog_path: &PathBuf) -> Result<Catalog> {
    let raw = fs::read_to_string(catalog_path)?;
    let catalog: Catalog = serde_yaml::from_str(&raw)?;
    for i in &catalog.installers {
        // `Apache` is a genuine, unconditional Apache-2.0 grant, not a BETA gate —
        // it has no "real price to flip to later." A nonzero price here is always a
        // data-entry mistake (e.g. a stray FSL/AGPL price copied onto a relicensed
        // entry), not a valid pricing choice. Fail loudly, matching this catalog's
        // established pattern for the retired `licenses:` key above.
        if i.license_tier == LicenseTier::Apache && i.price_usdc != 0 {
            anyhow::bail!(
                "installer '{}' is license_tier: apache but has a nonzero \
                 price_usdc ({}) — apache entries must always be price_usdc: 0",
                i.id,
                i.price_usdc
            );
        }
        // Phase 1b: the fact-row cap is enforced here, not by truncating at render
        // time — a 4th fact is a data-entry mistake to fix in products.yaml, not
        // something the page should silently hide.
        if i.facts.len() > 3 {
            anyhow::bail!(
                "installer '{}' declares {} facts — at most 3 are allowed \
                 (a 4th, always-derived Distribution row is added automatically)",
                i.id,
                i.facts.len()
            );
        }
    }
    Ok(catalog)
}

// ── Receipt (mirrors tool-wallet's LicenseReceipt) ────────────────────────────
//
// Field-for-field port of the OLD crate's `LicenseReceipt`. tool-wallet writes
// receipt files that carry ONE extra field — `license_tier` — which serde
// silently ignores here on read (no `deny_unknown_fields`), so files written by
// either binary deserialize cleanly. When THIS crate writes a receipt (the
// fresh-check path in `v1_license`), it omits `license_tier`, exactly as the OLD
// crate did. `price_usdc` here stores `price_units` (micro-USDC), NOT the
// catalog's dollar-labelled value — a pre-existing field-name quirk, not renamed
// in this phase. See tool-wallet/src/main.rs for the writer side.
#[derive(Debug, Serialize, Deserialize)]
struct LicenseReceipt {
    product_id: String,
    version: String,
    customer_ref: String,
    price_usdc: u64,
    tx_hash: String,
    chain: String,
    confirmed_at: String,
    block_number: u64,
    license_key: String,
}

// ── Payment helpers ───────────────────────────────────────────────────────────

/// Deterministic license key: first 32 hex chars of SHA256("{product_id}:{tx_hash}:{customer_ref}"),
/// split into four hyphen-joined 8-char groups. EXACT construction — must stay
/// byte-identical to the OLD crate and tool-wallet so already-issued keys remain
/// reproducible.
fn generate_license_key(product_id: &str, tx_hash: &str, customer_ref: &str) -> String {
    let h = hex::encode(Sha256::digest(
        format!("{product_id}:{tx_hash}:{customer_ref}").as_bytes(),
    ));
    format!("{}-{}-{}-{}", &h[0..8], &h[8..16], &h[16..24], &h[24..32])
}

/// Validates a Polygon transaction hash: `0x` followed by exactly 64 hex digits
/// (case-insensitive). Every caller that reaches a filesystem path
/// (`receipt_path`, via `resolve_license`) or embeds this value in a `Location`
/// header (`Redirect::to`, in `order_redirect`/`order_redirect_es`) must
/// validate first — unvalidated input let a percent-decoded newline reach
/// `Redirect::to`, which panics on an invalid header value, and this workspace
/// sets `panic = "abort"`, so that panic took down the whole process, not just
/// the request. The same missing check let `..` segments reach `receipt_path`
/// and read/write outside `receipts_dir` entirely (independent finding, same
/// root cause: this value was never validated anywhere on any path).
fn is_valid_tx_hash(s: &str) -> bool {
    let Some(hex) = s.strip_prefix("0x") else {
        return false;
    };
    hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validates a product identifier — matches every real catalog `id` (lowercase
/// alphanumeric and hyphens, e.g. `os-console`, `app-orchestration-command`).
/// Same rationale as `is_valid_tx_hash`: this crate had no equivalent of
/// `app-privategit-source`'s `is_safe_segment` anywhere, despite product ids
/// reaching `Redirect::to` (`order_redirect`) same as tx_hash.
fn is_safe_product_id(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Validates an Ethereum/Polygon wallet address: `0x` followed by exactly 40
/// hex digits. Used by `/v1/claim` (S2) — the address was previously joined
/// directly into a filesystem path with no validation at all; an absolute path
/// there discards the base directory entirely (`PathBuf::join` semantics).
fn is_valid_eth_address(s: &str) -> bool {
    let Some(hex) = s.strip_prefix("0x") else {
        return false;
    };
    hex.len() == 40 && hex.chars().all(|c| c.is_ascii_hexdigit())
}

/// Receipt file path: `<receipts_dir>/<current-UTC-year>/<current-UTC-month>/<tx_hash>.json`.
///
/// NOTE (carried forward, NOT fixed in this phase): the year/month are TODAY's at
/// request time, not the transaction's confirmation date. A receipt written near a
/// month boundary and re-read the next month misses the cache and re-verifies. This
/// is a known, pre-existing gap shared with the OLD crate and tool-wallet's own
/// writer; it was not named as an in-scope fix for P4. Left as-is deliberately.
fn receipt_path(receipts_dir: &std::path::Path, tx_hash: &str) -> PathBuf {
    let now = Utc::now();
    receipts_dir
        .join(now.format("%Y").to_string())
        .join(now.format("%m").to_string())
        .join(format!("{tx_hash}.json"))
}

/// Marker path for [`flag_if_first_live_transaction`] — deliberately at the
/// receipts-dir root, not inside a year/month subdirectory, so it survives
/// month rollover and is trivially findable.
fn first_live_transaction_marker_path(receipts_dir: &std::path::Path) -> PathBuf {
    receipts_dir.join(".first-live-transaction-marker.json")
}

/// Checkpoint 3b (real on-chain confirmation) could not run before this crate
/// went live — operator decision 2026-07-02, given real-transaction testing
/// isn't feasible for some time and the site needs to launch. In place of a
/// pre-launch gate, this flags the FIRST real (subprocess-confirmed, not
/// receipt-cache-replayed) transaction distinctly, so the operator can review
/// that one transaction closely after the fact rather than blind. Purely
/// observational — never affects the response, never blocks or slows the
/// request, and does nothing after the first transaction (every subsequent
/// one is silent).
fn flag_if_first_live_transaction(receipts_dir: &std::path::Path, receipt: &LicenseReceipt) {
    let marker = first_live_transaction_marker_path(receipts_dir);
    if marker.exists() {
        return;
    }
    if let Ok(raw) = serde_json::to_string_pretty(receipt) {
        let _ = fs::write(&marker, raw);
    }
    tracing::warn!(
        tx_hash = %receipt.tx_hash,
        product_id = %receipt.product_id,
        license_key = %receipt.license_key,
        confirmed_at = %receipt.confirmed_at,
        "FIRST-LIVE-TRANSACTION: the first real on-chain payment has been confirmed through \
         this crate. Checkpoint 3b was deferred at launch (2026-07-02) -- review this specific \
         transaction and receipt now to close it out."
    );
}

/// Append one JSONL row to the CRA barter-transaction log
/// (`BRIEF-software-distribution-substrate.md` §8 — required from the first real
/// sale; a crypto payment is a CRA barter transaction, recorded at CAD fair-market
/// value at settlement time). Schema matches that BRIEF's own example exactly:
/// `date`, `sku` (`<product_id>@<edition>`), `license_tier`, `crypto_received`,
/// `polygon_tx`, `spot_rate_cad`, `cad_equivalent`.
///
/// Only called from the fresh-confirmation path in `resolve_license`, never the
/// receipt-cache-replay path — a cached replay is not a new sale and must not be
/// logged twice.
///
/// **Known simplification**: `spot_rate_cad` is a placeholder (`AppState::
/// usdc_cad_spot_rate`, configurable via `USDC_CAD_SPOT_RATE`, no live FX feed is
/// wired into this crate). Sourcing a real rate at settlement time is future work,
/// plausibly a `project-bookkeeping` integration per that BRIEF's own §9
/// cross-cluster dependency table — not decided here.
fn append_tx_log(
    tx_log_path: &std::path::Path,
    receipt: &LicenseReceipt,
    edition: &str,
    license_tier: &str,
    spot_rate_cad: f64,
) {
    let usd = receipt.price_usdc as f64 / 1_000_000.0;
    let cad_equivalent = usd * spot_rate_cad;
    let row = json!({
        "date": receipt.confirmed_at,
        "sku": format!("{}@{}", receipt.product_id, edition),
        "license_tier": license_tier,
        "crypto_received": format!("{usd:.2} USDC"),
        "polygon_tx": receipt.tx_hash,
        "spot_rate_cad": format!("{spot_rate_cad:.2}"),
        "cad_equivalent": format!("{cad_equivalent:.2}"),
    });
    if let Some(parent) = tx_log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut line) = serde_json::to_string(&row) {
        line.push('\n');
        use std::io::Write;
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(tx_log_path)
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(line.as_bytes()) {
                    tracing::warn!("tx-log append failed: {e}");
                }
            }
            Err(e) => tracing::warn!("tx-log open failed: {e}"),
        }
    }
}

/// Convert a dollars-denominated USDC amount (as reported by `tool-wallet check`'s
/// `amount_usdc` float) into integer micro-USDC base units (6 decimals).
/// Correct for whole-dollar amounts ($1.00 → 1_000_000), but **lossy for many
/// cent-level values** due to the `f64` round-trip (Checkpoint 2 review finding —
/// see `price_units_from_check_json`, which prefers an exact integer and uses
/// this only as a defensive fallback). Do not call this directly on a
/// `tool-wallet check` response; go through `price_units_from_check_json`.
fn price_units_from_amount(amount_usdc: f64) -> u64 {
    (amount_usdc * 1_000_000.0) as u64
}

/// Extract the confirmed payment amount, in exact micro-USDC base units, from a
/// `tool-wallet check` confirmation JSON.
///
/// # Checkpoint 2 review finding
///
/// Prefers `amount_units` — the exact source integer `tool-wallet` already emits
/// (`tool-wallet/src/main.rs:568`) — over `price_units_from_amount(amount_usdc)`,
/// which round-trips through a lossy `f64` and is off-by-one for roughly 2% of
/// whole-cent prices (e.g. a genuine $2.01 arrives as 2,009,999, one micro-unit
/// short, and would silently fail to match — the same failure class this phase's
/// P4 pricing-unit fix exists to eliminate, just latent rather than fixed). The
/// float path is kept only as a defensive fallback for a `tool-wallet` response
/// that, contrary to its current and expected behavior, omits `amount_units`.
fn price_units_from_check_json(check_json: &Value) -> u64 {
    check_json
        .get("amount_units")
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| {
            let amount_usdc = check_json
                .get("amount_usdc")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            price_units_from_amount(amount_usdc)
        })
}

/// Resolve the catalog `product_id` whose price equals an on-chain payment of
/// `price_units` (micro-USDC base units).
///
/// # P4 pricing-unit fix — the one deliberate behavioural change in this rewrite
///
/// The OLD crate (`app-privategit-marketplace/src/main.rs`) matched with:
///
/// ```text
/// c.licenses.iter().find(|l| l.price_usdc * 1_000_000 == price_units)
/// ```
///
/// That is WRONG. `products.yaml` already stores `price_usdc` in micro-USDC base
/// units (`apache: 1000000` = $1.00, `fsl: 19000000` = $19.00 — the field is
/// misleadingly *named* dollars but its *value* is micro-units). Re-multiplying
/// the catalog side by 1_000_000 double-counts the unit conversion, so a genuine
/// $1.00 payment (`price_units == 1_000_000`) was compared against
/// `1_000_000 * 1_000_000` and could never match a real payment except by
/// coincidence.
///
/// **Correction (Checkpoint 2 review):** `apache` and `fsl` are live, non-zero
/// priced entries — NOT every catalog price is `0`. The actual safety argument
/// is: the OLD formula never matched a real payment (any historical paid tx would
/// have fallen through to `unknown-<price_units>`), so switching to the correct
/// comparison cannot invalidate any previously-issued license key — receipts are
/// read from disk verbatim and replay identically regardless of this fix.
///
/// THE FIX: compare `l.price_usdc == price_units` directly — both operands are
/// already micro-USDC units, so no multiplication belongs on the catalog side.
/// Full rationale: `docs/P4-PRICING-FIX.md`.
///
/// **Known limitation, not fixed in this phase (data-model rebuild):** now that
/// multiple os-* products can share the same tier price (up to 4 at `commercial`/
/// $1, up to 4 at `fsl`/$19, once BETA lifts for more than one product per tier),
/// price alone cannot disambiguate WHICH product was purchased — `.find()` returns
/// the first match. This is a legacy/compatibility path for the raw `/v1/license/
/// :tx_hash` JSON endpoint only. The new `/checkout/:product_id` → `/order/:tx_hash`
/// flow (Phase 2) carries `product_id` explicitly from the moment the customer picks
/// a product, so it does not depend on this inference at all — this function is not
/// the long-term mechanism, just the pre-existing one kept working during the
/// transition.
fn match_license_product_id(catalog: &Catalog, price_units: u64) -> Option<String> {
    catalog
        .installers
        .iter()
        // FIX: direct micro-unit equality. The OLD crate wrote `l.price_usdc *
        // 1_000_000 == price_units`, which re-applied the dollars→micro-units
        // scale to a value already in micro-units. No re-multiplication here.
        .find(|i| i.price_usdc == price_units)
        .map(|i| i.id.clone())
}

/// Outcome of shelling out to `tool-wallet check`.
enum WalletCheck {
    /// Subprocess exited 0 and its JSON reported `confirmed: true`.
    Confirmed(Value),
    /// Subprocess exited 0 and its JSON reported `confirmed: false`.
    Pending,
    /// Subprocess failed to spawn, exited non-zero, or emitted unparseable stdout.
    NotFound,
}

/// Run `tool-wallet check <tx_hash> --rpc-url <url> --wallet-address <addr>` as an
/// external subprocess and classify the result. Consumes tool-wallet's CLI
/// contract exactly as-is; never modifies it. `bin` is normally `"tool-wallet"`
/// (PATH-resolved); tests inject a JSON test-double path via it.
///
/// Uses `tokio::process::Command` rather than `std::process::Command`: the RPC
/// round-trip this subprocess performs can take seconds, and a blocking `.output()`
/// call here would stall the async worker thread it runs on for the duration —
/// starving every other request being polled on that thread (Fable audit S3).
async fn run_tool_wallet_check(
    bin: &str,
    tx_hash: &str,
    rpc_url: &str,
    wallet_addr: &str,
) -> WalletCheck {
    let result = tokio::process::Command::new(bin)
        .args([
            "check",
            tx_hash,
            "--rpc-url",
            rpc_url,
            "--wallet-address",
            wallet_addr,
        ])
        .output()
        .await;

    match result {
        Ok(out) if out.status.success() => match serde_json::from_slice::<Value>(&out.stdout) {
            Ok(check_json) => {
                let confirmed = check_json
                    .get("confirmed")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if confirmed {
                    WalletCheck::Confirmed(check_json)
                } else {
                    WalletCheck::Pending
                }
            }
            Err(e) => {
                tracing::warn!("tool-wallet check: unparseable stdout: {e}");
                WalletCheck::NotFound
            }
        },
        Ok(out) => {
            tracing::warn!(
                "tool-wallet check exit {:?}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            );
            WalletCheck::NotFound
        }
        Err(e) => {
            tracing::error!("tool-wallet not available: {e}");
            WalletCheck::NotFound
        }
    }
}

/// The shared `200 OK` confirmed-license JSON shape (receipt-cache path and
/// fresh-check path return identical bodies).
fn confirmed_response(
    license_key: &str,
    product_id: &str,
    confirmed_at: &str,
    customer_ref: &str,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "confirmed",
            "license_key": license_key,
            "product_id": product_id,
            "confirmed_at": confirmed_at,
            "customer_ref": customer_ref
        })),
    )
}

// ── Result of resolving a tx_hash — shared by the JSON and HTML order endpoints ──
//
// Extracted from `v1_license`'s body (Phase 2) so the new `/order/:tx_hash` HTML
// page and the existing `/v1/license/:tx_hash` JSON endpoint agree on state by
// construction — one code path, two renderings, not two parallel state machines.
enum LicenseOutcome {
    Confirmed {
        license_key: String,
        product_id: String,
        confirmed_at: String,
        customer_ref: String,
    },
    Pending {
        retry_after: u64,
    },
    NotFound,
}

async fn resolve_license(state: &AppState, tx_hash: &str) -> LicenseOutcome {
    // S1 fix: reject before this value ever reaches receipt_path(). A percent-decoded
    // `../` here escaped receipts_dir entirely on both the read (this fn) and write
    // (below) paths -- live-demonstrated reading and being steered to write outside
    // receipts_dir. A malformed-but-real-looking tx_hash simply isn't a transaction
    // we know about, so NotFound is the correct outcome, not a distinct error shape.
    if !is_valid_tx_hash(tx_hash) {
        return LicenseOutcome::NotFound;
    }
    // 1. Check local receipt file (idempotent replay of a prior confirmation).
    let rpath = receipt_path(&state.receipts_dir, tx_hash);
    if rpath.exists() {
        if let Ok(raw) = fs::read_to_string(&rpath) {
            if let Ok(receipt) = serde_json::from_str::<LicenseReceipt>(&raw) {
                return LicenseOutcome::Confirmed {
                    license_key: receipt.license_key,
                    product_id: receipt.product_id,
                    confirmed_at: receipt.confirmed_at,
                    customer_ref: receipt.customer_ref,
                };
            }
        }
    }

    // 2. No receipt on file — verify on-chain via the tool-wallet subprocess.
    match run_tool_wallet_check(
        &state.tool_wallet_bin,
        tx_hash,
        &state.polygon_rpc_url,
        &state.polygon_wallet_address,
    )
    .await
    {
        WalletCheck::Confirmed(check_json) => {
            let customer_ref = check_json
                .get("from")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let block_number = check_json
                .get("block")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let price_units = price_units_from_check_json(&check_json);
            let catalog = load_catalog(&state.catalog_path).ok();
            let product_id = catalog
                .as_ref()
                .and_then(|c| match_license_product_id(c, price_units))
                .unwrap_or_else(|| format!("unknown-{price_units}"));

            let license_key = generate_license_key(&product_id, tx_hash, &customer_ref);
            let confirmed_at = Utc::now().to_rfc3339();

            let receipt = LicenseReceipt {
                product_id: product_id.clone(),
                version: "0.0.1".into(),
                customer_ref: customer_ref.clone(),
                price_usdc: price_units,
                tx_hash: tx_hash.to_string(),
                chain: "polygon-pos".into(),
                confirmed_at: confirmed_at.clone(),
                block_number,
                license_key: license_key.clone(),
            };

            if let Some(parent) = rpath.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(raw) = serde_json::to_string_pretty(&receipt) {
                let _ = fs::write(&rpath, raw);
            }

            flag_if_first_live_transaction(&state.receipts_dir, &receipt);

            // Phase 5: CRA barter-transaction log — only on a genuinely fresh
            // confirmation (this branch), never a receipt-cache replay.
            if let Some(installer) = catalog
                .as_ref()
                .and_then(|c| c.installers.iter().find(|i| i.id == product_id))
            {
                append_tx_log(
                    &state.tx_log_path,
                    &receipt,
                    &installer.edition,
                    installer.license_tier.label(),
                    state.usdc_cad_spot_rate,
                );
            }

            LicenseOutcome::Confirmed {
                license_key,
                product_id,
                confirmed_at,
                customer_ref,
            }
        }
        WalletCheck::Pending => LicenseOutcome::Pending { retry_after: 30 },
        WalletCheck::NotFound => LicenseOutcome::NotFound,
    }
}

// ── Download-token minting (Phase 2) ─────────────────────────────────────────
//
// Mints a real, Ed25519-signed download-auth token matching
// `app-privategit-source-2`'s `LicensePayload` shape byte-for-byte (same field
// names/types, same `base64url_no_pad(sig[64] || payload_json)` wire format) —
// closes the previously-undiscovered gap where nothing in production ever
// actually minted one. Uses the mechanism `BRIEF-software-distribution-
// substrate.md` already specifies ("download delivery: time-limited URL"), not
// an invented single-use/revocation scheme: `channel_expiry` is set to TODAY, so
// the link is valid through the end of the day it was minted. A later visit to
// `/order/:tx_hash` mints a fresh token with that day's date — no source-2
// changes, no new persistent state.
#[derive(Serialize)]
struct LicensePayload {
    product: String,
    channel_expiry: String,
    entitlements: Vec<String>,
    version_floor: Option<String>,
    /// Mirrors `Installer::bundled_registry_products` for the purchased product —
    /// see that field's doc comment. Empty for every product that doesn't bundle
    /// anything else.
    #[serde(default)]
    entitled_products: Vec<String>,
}

/// Load the marketplace's private signing key from `SIGNING_KEY_SECRET` — same
/// hex-seed-or-file-path convention as `app-privategit-source-2`'s
/// `load_verify_key`, so the two crates' key-provisioning stories match.
fn load_signing_key(val: &str) -> Option<SigningKey> {
    let hex_str = if val.len() == 64 && val.chars().all(|c| c.is_ascii_hexdigit()) {
        val.to_string()
    } else {
        fs::read_to_string(val).ok()?.trim().to_string()
    };
    let bytes = hex::decode(&hex_str).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    Some(SigningKey::from_bytes(&arr))
}

/// Same hex-seed-or-file-path convention as `load_signing_key`, but for the
/// public counterpart — used only by the startup keypair self-test below, not
/// by any request-handling path.
fn load_verify_key_for_selftest(val: &str) -> Option<VerifyingKey> {
    let hex_str = if val.len() == 64 && val.chars().all(|c| c.is_ascii_hexdigit()) {
        val.to_string()
    } else {
        fs::read_to_string(val).ok()?.trim().to_string()
    };
    let bytes = hex::decode(&hex_str).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&arr).ok()
}

/// Startup keypair self-test (`BRIEF-software-consolidated-service-audit.md` S3):
/// a mismatch between this marketplace's `SIGNING_KEY_SECRET` and
/// `app-privategit-source-2`'s `VERIFY_KEY_PUB` today produces no error at
/// all — every minted download token fails verification with a uniform 401,
/// indistinguishable from an actual attack. If the operator has ALSO provided
/// this process a `VERIFY_KEY_PUB` value (the same one configured on the
/// source server — both processes typically run on the same host), compare
/// fingerprints directly and fail loudly on mismatch rather than staying
/// silent until the first real customer download fails.
/// Outcome of `keypair_selftest_check` — returned rather than logged directly so
/// the comparison logic is unit-testable without mutating process-global env vars.
#[derive(Debug, PartialEq, Eq)]
enum KeypairSelftestOutcome {
    Skipped,
    Invalid,
    Match,
    Mismatch { actual: String, expected: String },
}

fn keypair_selftest_check(
    signing_key: &SigningKey,
    verify_key_pub: Option<&str>,
) -> KeypairSelftestOutcome {
    let Some(expected) = verify_key_pub else {
        return KeypairSelftestOutcome::Skipped;
    };
    let Some(expected_vk) = load_verify_key_for_selftest(expected) else {
        return KeypairSelftestOutcome::Invalid;
    };
    let actual_vk = signing_key.verifying_key();
    if actual_vk.to_bytes() == expected_vk.to_bytes() {
        KeypairSelftestOutcome::Match
    } else {
        KeypairSelftestOutcome::Mismatch {
            actual: hex::encode(actual_vk.to_bytes()),
            expected: hex::encode(expected_vk.to_bytes()),
        }
    }
}

/// Startup keypair self-test (`BRIEF-software-consolidated-service-audit.md` S3):
/// a mismatch between this marketplace's `SIGNING_KEY_SECRET` and
/// `app-privategit-source-2`'s `VERIFY_KEY_PUB` today produces no error at
/// all — every minted download token fails verification with a uniform 401,
/// indistinguishable from an actual attack. If the operator has ALSO provided
/// this process a `VERIFY_KEY_PUB` value (the same one configured on the
/// source server — both processes typically run on the same host), compare
/// fingerprints directly and fail loudly on mismatch rather than staying
/// silent until the first real customer download fails.
fn keypair_selftest(signing_key: &SigningKey) {
    let verify_key_pub = std::env::var("VERIFY_KEY_PUB").ok();
    match keypair_selftest_check(signing_key, verify_key_pub.as_deref()) {
        KeypairSelftestOutcome::Skipped => {
            tracing::info!(
                "keypair self-test skipped — VERIFY_KEY_PUB not provided to this process"
            );
        }
        KeypairSelftestOutcome::Invalid => {
            tracing::error!("keypair self-test FAILED — VERIFY_KEY_PUB set but unreadable/invalid");
        }
        KeypairSelftestOutcome::Match => {
            tracing::info!("keypair self-test passed — SIGNING_KEY_SECRET matches VERIFY_KEY_PUB");
        }
        KeypairSelftestOutcome::Mismatch { actual, expected } => {
            tracing::error!(
                %actual,
                %expected,
                "keypair self-test FAILED — SIGNING_KEY_SECRET does not match VERIFY_KEY_PUB; \
                 every download this marketplace mints will fail verification on the source server"
            );
        }
    }
}

/// Mint a fresh download-auth token for `product_id`, valid through the end of
/// today (see module doc above). The hardcoded `"linux-x86_64"` platform and
/// single `"binary"` entitlement are known simplifications for this phase — real
/// platform selection is bigger scope than this cleanup.
fn mint_license_token(
    signing_key: &SigningKey,
    product_id: &str,
    entitled_products: &[String],
) -> String {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let payload = LicensePayload {
        product: product_id.to_string(),
        channel_expiry: today,
        entitlements: vec!["binary".to_string()],
        version_floor: None,
        entitled_products: entitled_products.to_vec(),
    };
    let payload_json = serde_json::to_string(&payload).expect("LicensePayload always serializes");
    let sig = signing_key.sign(payload_json.as_bytes());
    let mut bytes = sig.to_bytes().to_vec();
    bytes.extend_from_slice(payload_json.as_bytes());
    URL_SAFE_NO_PAD.encode(bytes)
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// JSON API error body carrying both a human-readable `error` message and a
/// stable, machine-readable `code` (M4: matches `app-privategit-source`'s
/// `err_json` — S15's schema work landed there but not here; this crate's own
/// JSON API responses (`/v1/*`) had no equivalent). `code` is a kebab-case
/// slug that won't change even if `error`'s wording does.
fn err_json(code: &'static str, message: impl Into<String>) -> Value {
    json!({"error": message.into(), "code": code})
}

// GET / -> 302 Found redirect to /software.
//
// Note: axum's `Redirect::to` emits 303 See Other, not 302. The P1 contract
// specifies 302, so we build the response explicitly with StatusCode::FOUND.
// Chromed error response — replaces the bare `text/plain` 404/500 bodies that used
// to throw a visitor out of the site chrome entirely (M2 in
// BRIEF-software-handoff-readiness.md). `path` only affects the lang-toggle target;
// error pages aren't marked `translated`, so no hreflang alternates are emitted.
fn error_response(
    lang: Lang,
    status: StatusCode,
    path: &str,
    heading: &str,
    message: &str,
) -> Response {
    let title = format!("{heading} — PointSav Software");
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        message,
        path,
        false,
        ui::error_markup(lang, heading, message),
    )
    .into_string();
    (
        status,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// Router-level fallback for any request that matches no declared route — replaces
// axum's default bare-empty 404 with the site's own chrome. English only: a
// genuinely unmatched path gives no reliable signal of intended locale (`/es/*`
// 404s inside known route prefixes are handled by the per-page 404 branches, which
// know the requested language from the route itself).
async fn not_found_fallback() -> Response {
    error_response(
        Lang::En,
        StatusCode::NOT_FOUND,
        "/software",
        "Page not found",
        "The page you're looking for doesn't exist. It may have moved, or the link may be out of date.",
    )
}

async fn root() -> Response {
    (StatusCode::FOUND, [(header::LOCATION, "/software")]).into_response()
}

// GET /es -> 302 Found redirect to /es/software. Mirrors `root()`, and mirrors
// home.pointsav.com/home.woodfinegroup.com's own reciprocal `/` <-> `/es` pattern.
async fn root_es() -> Response {
    (StatusCode::FOUND, [(header::LOCATION, "/es/software")]).into_response()
}

// GET /software — dynamic product catalog.
//
// Replaces the P1 static-HTML read (`software.html` + `wrap_static_html`). The page is
// now rendered from the SAME `Catalog` that `v1_products` loads, so the product cards
// can never drift from `products.yaml` again (the bug this phase fixes). The Sovereign
// Editorial chrome is supplied by `ui::render_page`.
async fn software_page(State(state): State<Arc<AppState>>) -> Response {
    render_software_page(&state, Lang::En).await
}

// GET /es/software — MVL Spanish variant (operator-approved 2026-07-12). Same catalog
// data, chrome/labels translated via `Lang::Es` — see `ui::lang` module docs.
async fn software_page_es(State(state): State<Arc<AppState>>) -> Response {
    render_software_page(&state, Lang::Es).await
}

async fn render_software_page(state: &AppState, lang: Lang) -> Response {
    match load_catalog(&state.catalog_path) {
        Ok(catalog) => {
            let content = ui::catalog_markup(&catalog, &state.source_base_url, lang);
            let (title, description) = match lang {
                Lang::En => (
                    "Products — PointSav Software".to_string(),
                    "Browse PointSav's software catalog — licensed binaries, release packages, \
                     and installation manifests for the PointSav platform's applications and tooling."
                        .to_string(),
                ),
                Lang::Es => (
                    "Productos — PointSav Software".to_string(),
                    "Explore el cat\u{e1}logo de software de PointSav — binarios con licencia, \
                     paquetes de versi\u{f3}n y manifiestos de instalaci\u{f3}n para las \
                     aplicaciones y herramientas de la plataforma PointSav."
                        .to_string(),
                ),
            };
            let body = ui::render_page(
                SoftwareSurface::Marketplace,
                lang,
                &title,
                &description,
                &lang.localize("/software"),
                true,
                content,
            )
            .into_string();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                body,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("catalog load failed for /software: {e:#}");
            let (heading, message) = match lang {
                Lang::En => (
                    "Catalog unavailable",
                    "The product catalog couldn't be loaded right now. Please try again shortly.",
                ),
                Lang::Es => (
                    "Cat\u{e1}logo no disponible",
                    "El cat\u{e1}logo de productos no se pudo cargar en este momento. Int\u{e9}ntelo de nuevo en breve.",
                ),
            };
            error_response(
                lang,
                StatusCode::INTERNAL_SERVER_ERROR,
                &lang.localize("/software"),
                heading,
                message,
            )
        }
    }
}

// GET /licensing — UNCHANGED. Static legal/terms document (not catalog data): keeps the
// P1 static-file read + P2 chrome-wrap exactly as-is. Phase 4 rewrote the file's
// CONTENT (dropping fictional wallet-connect/tax/fake-product copy) but not this
// handler's logic.
async fn licensing_page(State(state): State<Arc<AppState>>) -> Response {
    serve_chrome_page(
        &state.static_dir.join("licensing.html"),
        Lang::En,
        "/licensing",
    )
}

// GET /es/licensing — MVL Spanish variant, reads the separately-maintained
// `licensing.es.html` static file (translated content, own `<head>` tags — same
// pattern as the English file, not templated through `render_page`).
async fn licensing_page_es(State(state): State<Arc<AppState>>) -> Response {
    serve_chrome_page(
        &state.static_dir.join("licensing.es.html"),
        Lang::Es,
        "/es/licensing",
    )
}

// GET /pricing — Phase 4. Catalog-driven (like `software_page`), not a static file,
// so it can never drift into fiction the way `licensing.html` had.
async fn pricing_page(State(state): State<Arc<AppState>>) -> Response {
    render_pricing_page(&state, Lang::En).await
}

// GET /es/pricing — MVL Spanish variant (operator-approved 2026-07-12).
async fn pricing_page_es(State(state): State<Arc<AppState>>) -> Response {
    render_pricing_page(&state, Lang::Es).await
}

async fn render_pricing_page(state: &AppState, lang: Lang) -> Response {
    match load_catalog(&state.catalog_path) {
        Ok(catalog) => {
            let content = ui::pricing_markup(&catalog, lang);
            let (title, description) = match lang {
                Lang::En => (
                    "Pricing — PointSav Software".to_string(),
                    "License pricing and tier structure for PointSav software — Apache-2.0 \
                     (permanently free), AGPL-3.0-or-later + Commercial, and Proprietary tiers, \
                     currently free during public BETA."
                        .to_string(),
                ),
                Lang::Es => (
                    "Precios — PointSav Software".to_string(),
                    "Estructura de precios y niveles de licencia del software de PointSav — \
                     niveles Apache-2.0 (gratuito de forma permanente), AGPL-3.0-or-later + \
                     Commercial, y Proprietary, actualmente gratuitos durante la fase BETA \
                     p\u{fa}blica."
                        .to_string(),
                ),
            };
            let body = ui::render_page(
                SoftwareSurface::Marketplace,
                lang,
                &title,
                &description,
                &lang.localize("/pricing"),
                true,
                content,
            )
            .into_string();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                body,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("catalog load failed for /pricing: {e:#}");
            let (heading, message) = match lang {
                Lang::En => (
                    "Catalog unavailable",
                    "Pricing information couldn't be loaded right now. Please try again shortly.",
                ),
                Lang::Es => (
                    "Cat\u{e1}logo no disponible",
                    "La informaci\u{f3}n de precios no se pudo cargar en este momento. Int\u{e9}ntelo de nuevo en breve.",
                ),
            };
            error_response(
                lang,
                StatusCode::INTERNAL_SERVER_ERROR,
                &lang.localize("/pricing"),
                heading,
                message,
            )
        }
    }
}

// GET /page/disclaimer — self-contained disclaimer page (operator instruction 2026-07-02:
// this site has its own content, no cross-site links out to the wiki/marketing sites'
// disclaimers). Content is a compile-time constant (`ui::disclaimer::disclaimer_markup`),
// not read from disk, so there is no "file missing" error path to handle here.
async fn disclaimer_page() -> Response {
    render_disclaimer_page(Lang::En)
}

// GET /es/page/disclaimer — full-site-parity pass (2026-07-13); see `ui::lang` module docs.
async fn disclaimer_page_es() -> Response {
    render_disclaimer_page(Lang::Es)
}

fn render_disclaimer_page(lang: Lang) -> Response {
    let (title, description) = match lang {
        Lang::En => (
            "Disclaimer — PointSav Software".to_string(),
            "Legal disclaimer for software.pointsav.com — no warranty, licensing terms, and \
             payment/jurisdictional notices for PointSav software purchases."
                .to_string(),
        ),
        Lang::Es => (
            "Aviso legal — PointSav Software".to_string(),
            "Aviso legal de software.pointsav.com — sin garant\u{ed}a, t\u{e9}rminos de \
             licencia, y avisos de pago/jurisdicci\u{f3}n para compras de software PointSav."
                .to_string(),
        ),
    };
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        &description,
        &lang.localize("/page/disclaimer"),
        true,
        ui::disclaimer_markup(lang),
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /page/privacy — self-contained privacy page, same pattern as disclaimer_page.
async fn privacy_page() -> Response {
    render_privacy_page(Lang::En)
}

// GET /es/page/privacy — full-site-parity pass (2026-07-13).
async fn privacy_page_es() -> Response {
    render_privacy_page(Lang::Es)
}

fn render_privacy_page(lang: Lang) -> Response {
    let (title, description) = match lang {
        Lang::En => (
            "Privacy — PointSav Software".to_string(),
            "Privacy policy for software.pointsav.com — what data this site collects and how \
             it's used."
                .to_string(),
        ),
        Lang::Es => (
            "Privacidad — PointSav Software".to_string(),
            "Pol\u{ed}tica de privacidad de software.pointsav.com \u{2014} qu\u{e9} datos "
                .to_string()
                + "recopila este sitio y c\u{f3}mo se usan.",
        ),
    };
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        &description,
        &lang.localize("/page/privacy"),
        true,
        ui::privacy_markup(lang),
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /working-papers — the 3 JOURNAL papers assigned to software.pointsav.com.
// Added 2026-09-15 (project-editorial's /working-papers rendering bootstrap).
// English-only, same pattern as every other non-MVL page's fallback: no /es
// sibling for paper bodies (institutional-authorship JOURNAL content is never
// translated, matching every other site's own JOURNAL convention).
async fn working_papers_index_page() -> Response {
    let title = "Working Papers — PointSav Software".to_string();
    let description =
        "Working papers from PointSav Digital Systems on software licensing economics, \
         open-source viability under AI, and vendor-independent ownership verification."
            .to_string();
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        Lang::En,
        &title,
        &description,
        "/working-papers",
        false,
        ui::working_papers_index_markup(Lang::En),
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /working-papers/:slug
async fn working_paper_detail_page(Path(slug): Path<String>) -> Response {
    match ui::working_papers::find(&slug) {
        Some(paper) => {
            let title = format!("{} — Working Papers — PointSav Software", paper.title);
            let body = ui::render_page(
                SoftwareSurface::Marketplace,
                Lang::En,
                &title,
                paper.subtitle,
                &format!("/working-papers/{slug}"),
                false,
                ui::working_paper_item_markup(paper),
            )
            .into_string();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                body,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "Working paper not found").into_response(),
    }
}

// GET /page/accessibility — self-contained accessibility page, same pattern as disclaimer_page.
async fn accessibility_page() -> Response {
    render_accessibility_page(Lang::En)
}

// GET /es/page/accessibility — full-site-parity pass (2026-07-13).
async fn accessibility_page_es() -> Response {
    render_accessibility_page(Lang::Es)
}

fn render_accessibility_page(lang: Lang) -> Response {
    let (title, description) = match lang {
        Lang::En => (
            "Accessibility — PointSav Software".to_string(),
            "Accessibility statement for software.pointsav.com.".to_string(),
        ),
        Lang::Es => (
            "Accesibilidad — PointSav Software".to_string(),
            "Declaraci\u{f3}n de accesibilidad de software.pointsav.com.".to_string(),
        ),
    };
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        &description,
        &lang.localize("/page/accessibility"),
        true,
        ui::accessibility_markup(lang),
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /robots.txt (SEO — BRIEF-seo-cross-site-strategy.md). Allows everything except the
// JSON `/v1/*` API surface, which is not indexable content.
async fn robots_txt() -> Response {
    let body = "User-agent: *\nAllow: /\nDisallow: /v1/\nSitemap: https://software.pointsav.com/sitemap.xml\n";
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /sitemap.xml (SEO). Lists only this crate's real HTML pages — never `/v1/*`, and
// never `app-privategit-source`'s `/releases/*`/`/git/*` (a different service entirely,
// streams binaries/manifests only, no HTML to index).
// Tier 4 fix: previously a fixed list omitting product-detail pages entirely and
// including `/`, which is only ever a 302 to `/software` — listing a redirect and
// its own target. Product pages are now enumerated from the live catalog (never
// drifts from what's actually deployed); `/` is dropped in favor of `/software`.
async fn sitemap_xml(State(state): State<Arc<AppState>>) -> Response {
    let mut pages = vec![
        "/software".to_string(),
        "/pricing".to_string(),
        "/licensing".to_string(),
        "/page/contact".to_string(),
        "/page/disclaimer".to_string(),
        "/page/privacy".to_string(),
        "/page/accessibility".to_string(),
        // Spanish variants — every page now has a real ES sibling (2026-07-13
        // full-site-parity pass); /es alone omitted since it 302s to /es/software.
        "/es/software".to_string(),
        "/es/pricing".to_string(),
        "/es/licensing".to_string(),
        "/es/page/contact".to_string(),
        "/es/page/disclaimer".to_string(),
        "/es/page/privacy".to_string(),
        "/es/page/accessibility".to_string(),
    ];
    if let Ok(catalog) = load_catalog(&state.catalog_path) {
        for i in &catalog.installers {
            pages.push(format!("/software/{}", i.id));
            // Product-detail /es/* extension (2026-08-02) — English only until then.
            pages.push(format!("/es/software/{}", i.id));
        }
    }
    let urls: String = pages
        .iter()
        .map(|p| format!("  <url><loc>https://software.pointsav.com{p}</loc></url>\n"))
        .collect();
    let body = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}</urlset>\n"
    );
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /page/contact — self-contained contact page, same pattern as disclaimer_page.
// Closes the highest-priority finding from the original Sovereign Editorial audit
// (this route previously did not exist at all — HTTP 0, flagged twice).
async fn contact_page() -> Response {
    render_contact_page(Lang::En)
}

// GET /es/page/contact — full-site-parity pass (2026-07-13).
async fn contact_page_es() -> Response {
    render_contact_page(Lang::Es)
}

fn render_contact_page(lang: Lang) -> Response {
    let (title, description) = match lang {
        Lang::En => (
            "Contact us — PointSav Software".to_string(),
            "Contact PointSav Digital Systems for support with software licenses, downloads, \
             and security disclosures."
                .to_string(),
        ),
        Lang::Es => (
            "Cont\u{e1}ctenos — PointSav Software".to_string(),
            "Contacte a PointSav Digital Systems para soporte con licencias de software, \
             descargas y divulgaciones de seguridad."
                .to_string(),
        ),
    };
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        &description,
        &lang.localize("/page/contact"),
        true,
        ui::contact_markup(lang),
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /software/:product_id — S136 product detail page. Mirrors checkout_page's
// catalog-lookup + 404/500 shape exactly.
// GET /software/:product_id — English product detail page.
async fn product_detail_page(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Response {
    render_product_detail_page(&state, &product_id, Lang::En).await
}

// GET /es/software/:product_id — Spanish product detail page (2026-08-02 follow-up
// to the full-site Spanish localization pass, which had explicitly deferred this
// page — see BRIEF-software-spanish-localization.md). Static labels translate;
// product name/description stay English (no translation source for catalog data).
async fn product_detail_page_es(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Response {
    render_product_detail_page(&state, &product_id, Lang::Es).await
}

async fn render_product_detail_page(state: &AppState, product_id: &str, lang: Lang) -> Response {
    match load_catalog(&state.catalog_path) {
        Ok(catalog) => match catalog.installers.iter().find(|i| i.id == product_id) {
            Some(installer) => {
                let content = ui::product_detail_markup(installer, &state.source_base_url, lang);
                let (title, description) = match lang {
                    Lang::En => (
                        format!("{} — PointSav Software", installer.name),
                        format!(
                            "{} — {} Licensed binary download, install command, and version/checksum details.",
                            installer.name, installer.description
                        ),
                    ),
                    Lang::Es => (
                        format!("{} — PointSav Software", installer.name),
                        format!(
                            "{} — {} Descarga de binario con licencia, comando de instalaci\u{f3}n \
                             y detalles de versi\u{f3}n/suma de verificaci\u{f3}n.",
                            installer.name, installer.description
                        ),
                    ),
                };
                let body = ui::render_page(
                    SoftwareSurface::Marketplace,
                    lang,
                    &title,
                    &description,
                    &lang.localize(&format!("/software/{}", installer.id)),
                    true,
                    content,
                )
                .into_string();
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                    body,
                )
                    .into_response()
            }
            None => {
                let (heading, message) = match lang {
                    Lang::En => (
                        "Product not found",
                        "That product doesn't exist in the catalog. It may have been renamed or removed.",
                    ),
                    Lang::Es => (
                        "Producto no encontrado",
                        "Ese producto no existe en el cat\u{e1}logo. Puede que haya sido renombrado o eliminado.",
                    ),
                };
                error_response(
                    lang,
                    StatusCode::NOT_FOUND,
                    &lang.localize("/software"),
                    heading,
                    message,
                )
            }
        },
        Err(e) => {
            tracing::error!("catalog load failed for /software/:id: {e:#}");
            let (heading, message) = match lang {
                Lang::En => (
                    "Catalog unavailable",
                    "The product catalog couldn't be loaded right now. Please try again shortly.",
                ),
                Lang::Es => (
                    "Cat\u{e1}logo no disponible",
                    "El cat\u{e1}logo de productos no se pudo cargar en este momento. Int\u{e9}ntelo de nuevo en breve.",
                ),
            };
            error_response(
                lang,
                StatusCode::INTERNAL_SERVER_ERROR,
                &lang.localize("/software"),
                heading,
                message,
            )
        }
    }
}

// Read the prerendered static page from disk (P1 logic, unchanged) and wrap it in
// the Sovereign Editorial chrome (navy masthead + near-black footer) before serving.
fn serve_chrome_page(file_path: &PathBuf, lang: Lang, url_path: &str) -> Response {
    match fs::read_to_string(file_path) {
        Ok(raw) => {
            let body =
                ui::wrap_static_html(&raw, SoftwareSurface::Marketplace, lang, url_path, true);
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                body,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("failed to read static page {}: {e}", file_path.display());
            let (heading, message) = match lang {
                Lang::En => (
                    "Page unavailable",
                    "This page couldn't be loaded right now. Please try again shortly.",
                ),
                Lang::Es => (
                    "P\u{e1}gina no disponible",
                    "Esta p\u{e1}gina no se pudo cargar en este momento. Int\u{e9}ntelo de nuevo en breve.",
                ),
            };
            error_response(
                lang,
                StatusCode::INTERNAL_SERVER_ERROR,
                "/software",
                heading,
                message,
            )
        }
    }
}

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "service": "app-privategit-software"}))
}

async fn v1_products(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    match load_catalog(&state.catalog_path) {
        Ok(catalog) => {
            // No compatibility view needed: this crate has not launched (P8 pending),
            // so there are no external JSON-API consumers of the old `installers`/
            // `licenses` split to protect. One unified shape, reflecting the
            // corrected data model directly.
            let installers: Vec<Value> = catalog
                .installers
                .iter()
                .map(|i| {
                    json!({
                        "id": i.id,
                        "name": i.name,
                        "description": i.description,
                        "edition": i.edition,
                        "platform": i.platform,
                        "size_mb": i.size_mb,
                        // Built from `id`/`edition`/`platform_slug()` directly, matching
                        // `ui::product_detail`'s and `order_download`'s URL construction —
                        // not from `i.path`, which real `products.yaml` entries set to
                        // `"{id}/{edition}"` anyway (redundant with the fields already
                        // used here) and which never carried a platform segment, so this
                        // `download_url` 404'd against the real `/releases/:product/
                        // :version/:platform` route regardless of what `path` said.
                        "download_url": format!(
                            "{}/{}/{}/{}",
                            state.source_base_url, i.id, i.edition, i.platform_slug()
                        ),
                        "manifest_url": format!(
                            "{}/{}/{}/MANIFEST",
                            state.source_base_url, i.id, i.edition
                        ),
                        "license_tier": i.license_tier.label(),
                        "shelf": i.license_tier.shelf().as_str(),
                        "price_usdc": i.price_usdc,
                        "cost": if i.price_usdc == 0 { "free" } else { "paid" },
                        "payment_address": state.polygon_wallet_address,
                        "payment_chain": "polygon-pos",
                        "payment_token": "USDC"
                    })
                })
                .collect();
            (StatusCode::OK, Json(json!({"installers": installers})))
        }
        Err(e) => {
            tracing::error!("catalog load failed: {e:#}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err_json("catalog-unavailable", "catalog unavailable")),
            )
        }
    }
}

// GET /v1/license/:tx_hash — payment state machine (receipt cache → tool-wallet check).
//
// Phase 2: the state machine itself now lives in `resolve_license`, shared with the
// new `/order/:tx_hash` HTML page — this handler is just the JSON rendering of that
// shared result. Behavior/response shapes are unchanged from before the refactor.
async fn v1_license(
    State(state): State<Arc<AppState>>,
    Path(tx_hash): Path<String>,
) -> (StatusCode, Json<Value>) {
    let tx_hash = tx_hash.to_lowercase();
    match resolve_license(&state, &tx_hash).await {
        LicenseOutcome::Confirmed {
            license_key,
            product_id,
            confirmed_at,
            customer_ref,
        } => confirmed_response(&license_key, &product_id, &confirmed_at, &customer_ref),
        LicenseOutcome::Pending { retry_after } => (
            StatusCode::ACCEPTED,
            Json(json!({
                "status": "pending",
                "code": "payment-pending",
                "retry_after": retry_after,
                "message": "Transaction not yet confirmed on Polygon. Retry in 30 seconds."
            })),
        ),
        LicenseOutcome::NotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "not_found",
                "code": "tx-not-found",
                "message": "Transaction not found or not a recognised USDC payment to this address."
            })),
        ),
    }
}

#[derive(Debug, Deserialize)]
struct ClaimRequest {
    binary_sha256: String,
    wallet_address: String,
}

// POST /v1/claim — placeholder token issuance (on-chain mint arrives v0.0.2). Ported
// as-is from the OLD crate; not made "more real" in this phase.
/// Validates a binary's SHA256 digest: exactly 64 hex digits (case-insensitive),
/// matching `hex::encode(Sha256::digest(..))`'s own output shape.
fn is_valid_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

async fn v1_claim(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ClaimRequest>,
) -> (StatusCode, Json<Value>) {
    // C1/S2 fix: both fields used to reach a filesystem path and a byte-slice
    // completely unvalidated. `binary_sha256[..16.min(len)]` panicked on any
    // input where byte 16 isn't a UTF-8 char boundary (e.g. a multi-byte
    // character straddling it) -- and this workspace's panic = "abort" turns
    // that into a full process crash on one request, not just a dropped
    // connection. Separately, `claims_dir.join(wallet_address)` with an
    // absolute-looking wallet_address discards claims_dir entirely
    // (`PathBuf::join` semantics) -- live-demonstrated writing an
    // attacker-controlled file anywhere the service can write. Validating the
    // expected shape of both fields up front closes both at once: neither
    // value can now be anything but safe, fixed-length hex.
    if !is_valid_sha256_hex(&req.binary_sha256) || !is_valid_eth_address(&req.wallet_address) {
        return (
            StatusCode::BAD_REQUEST,
            Json(err_json(
                "invalid-claim-fields",
                "binary_sha256 must be 64 hex digits and wallet_address must be 0x + 40 hex digits",
            )),
        );
    }

    let claimed_at = Utc::now().to_rfc3339();
    let token = hex::encode(Sha256::digest(
        format!(
            "{}|{}|{}",
            req.binary_sha256, req.wallet_address, claimed_at
        )
        .as_bytes(),
    ));

    let claim_dir = state
        .claims_dir
        .join(req.wallet_address.trim_start_matches("0x"));
    if let Err(e) = fs::create_dir_all(&claim_dir) {
        tracing::error!(
            "v1_claim: failed to create claim dir {}: {e}",
            claim_dir.display()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(err_json("internal-error", "could not record claim")),
        );
    }
    let short = &req.binary_sha256[..16];
    let claim_file = claim_dir.join(format!("{short}.json"));
    let payload = json!({
        "token": token,
        "binary_sha256": req.binary_sha256,
        "wallet_address": req.wallet_address,
        "claimed_at": claimed_at
    });
    if let Err(e) = fs::write(
        &claim_file,
        serde_json::to_string_pretty(&payload).unwrap_or_default(),
    ) {
        tracing::error!("v1_claim: failed to write {}: {e}", claim_file.display());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(err_json("internal-error", "could not record claim")),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "token": token,
            "claimed_at": claimed_at,
            "status": "ok",
            "note": "on-chain mint arrives v0.0.2"
        })),
    )
}

// GET /v1/wallet/address — the receiving wallet + chain/token/contract descriptor.
// The USDC contract is a hardcoded public constant (native USDC on Polygon PoS).
async fn v1_wallet_address(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "address": state.polygon_wallet_address,
        "chain": "polygon-pos",
        "token": "USDC",
        "contract": "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"
    }))
}

// ── Phase 2: checkout / order handlers ───────────────────────────────────────

// GET /checkout/:product_id — invoice page for one product, one fixed price.
async fn checkout_page(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Response {
    render_checkout_page(&state, &product_id, Lang::En).await
}

// GET /es/checkout/:product_id — full-site-parity pass (2026-07-13).
async fn checkout_page_es(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Response {
    render_checkout_page(&state, &product_id, Lang::Es).await
}

async fn render_checkout_page(state: &AppState, product_id: &str, lang: Lang) -> Response {
    match load_catalog(&state.catalog_path) {
        Ok(catalog) => match catalog.installers.iter().find(|i| i.id == product_id) {
            Some(installer) => {
                let content = ui::checkout_markup(installer, &state.polygon_wallet_address, lang);
                let (title, description) = match lang {
                    Lang::En => (
                        format!("Checkout — {} — PointSav Software", installer.name),
                        format!(
                            "Pay with Polygon USDC to mint a license for {}.",
                            installer.name
                        ),
                    ),
                    Lang::Es => (
                        format!("Pago — {} — PointSav Software", installer.name),
                        format!(
                            "Pague con Polygon USDC para emitir una licencia de {}.",
                            installer.name
                        ),
                    ),
                };
                let body = ui::render_page(
                    SoftwareSurface::Marketplace,
                    lang,
                    &title,
                    &description,
                    &lang.localize(&format!("/checkout/{}", installer.id)),
                    true,
                    content,
                )
                .into_string();
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                    body,
                )
                    .into_response()
            }
            None => {
                let (heading, message) = match lang {
                    Lang::En => (
                        "Product not found",
                        "That product doesn't exist in the catalog. It may have been renamed or removed.",
                    ),
                    Lang::Es => (
                        "Producto no encontrado",
                        "Ese producto no existe en el cat\u{e1}logo. Puede que haya sido renombrado o eliminado.",
                    ),
                };
                error_response(
                    lang,
                    StatusCode::NOT_FOUND,
                    &lang.localize("/software"),
                    heading,
                    message,
                )
            }
        },
        Err(e) => {
            tracing::error!("catalog load failed for /checkout: {e:#}");
            let (heading, message) = match lang {
                Lang::En => (
                    "Catalog unavailable",
                    "The product catalog couldn't be loaded right now. Please try again shortly.",
                ),
                Lang::Es => (
                    "Cat\u{e1}logo no disponible",
                    "El cat\u{e1}logo de productos no se pudo cargar en este momento. Int\u{e9}ntelo de nuevo en breve.",
                ),
            };
            error_response(
                lang,
                StatusCode::INTERNAL_SERVER_ERROR,
                &lang.localize("/software"),
                heading,
                message,
            )
        }
    }
}

#[derive(Deserialize)]
struct OrderRedirectQuery {
    product: String,
    tx_hash: String,
}

// GET /order?product=<id>&tx_hash=<value> — the checkout form's submit target.
// Redirects to the canonical, bookmarkable /order/:tx_hash?product=<id> so the
// order page has one durable URL regardless of how the customer arrived at it.
async fn order_redirect(Query(q): Query<OrderRedirectQuery>) -> Response {
    render_order_redirect(q, Lang::En, "/order")
}

// GET /es/order?product=<id>&tx_hash=<value> — Spanish checkout form's submit target,
// same pattern as `order_redirect`. Full-site-parity pass (2026-07-13).
async fn order_redirect_es(Query(q): Query<OrderRedirectQuery>) -> Response {
    render_order_redirect(q, Lang::Es, "/es/order")
}

// C1 fix: `Redirect::to()` panics if its argument isn't a valid HTTP header
// value (axum's own doc comment says so) -- q.tx_hash/q.product were built into
// the redirect target completely unvalidated, so a percent-decoded control
// character (e.g. `%0A`) reached `Redirect::to` and panicked. This workspace
// sets `panic = "abort"` in every profile, so that panic killed the entire
// process on a single unauthenticated request, not just the connection.
// Validating both values before ever calling Redirect::to closes it structurally
// -- an invalid value now can't reach that call at all.
fn render_order_redirect(q: OrderRedirectQuery, lang: Lang, base_path: &str) -> Response {
    let tx_hash = q.tx_hash.trim().to_lowercase();
    if !is_valid_tx_hash(&tx_hash) || !is_safe_product_id(&q.product) {
        return error_response(
            lang,
            StatusCode::BAD_REQUEST,
            "/software",
            "Invalid request",
            "That link is malformed. Please start again from the checkout page.",
        );
    }
    Redirect::to(&format!("{base_path}/{tx_hash}?product={}", q.product)).into_response()
}

#[derive(Deserialize, Default)]
struct OrderQuery {
    product: Option<String>,
}

// GET /order/:tx_hash — status/entitlement page. Renders whichever of the three
// `LicenseOutcome` states `resolve_license` returns; the confirmed state shows
// the receipt and a Download link. `?product=` (carried from checkout) is used
// only for the not-found state's "back to checkout" link — the confirmed state
// already knows its own product_id from the resolved receipt.
async fn order_status_page(
    State(state): State<Arc<AppState>>,
    Path(tx_hash): Path<String>,
    Query(q): Query<OrderQuery>,
) -> Response {
    render_order_status_page(&state, &tx_hash, q, Lang::En).await
}

// GET /es/order/:tx_hash — full-site-parity pass (2026-07-13).
async fn order_status_page_es(
    State(state): State<Arc<AppState>>,
    Path(tx_hash): Path<String>,
    Query(q): Query<OrderQuery>,
) -> Response {
    render_order_status_page(&state, &tx_hash, q, Lang::Es).await
}

async fn render_order_status_page(
    state: &AppState,
    tx_hash: &str,
    q: OrderQuery,
    lang: Lang,
) -> Response {
    let tx_hash = tx_hash.to_lowercase();
    let content = match resolve_license(state, &tx_hash).await {
        LicenseOutcome::Confirmed {
            license_key,
            product_id,
            ..
        } => ui::order_confirmed_markup(&tx_hash, &license_key, &product_id, lang),
        LicenseOutcome::Pending { retry_after } => {
            ui::order_pending_markup(&tx_hash, retry_after, lang)
        }
        LicenseOutcome::NotFound => {
            ui::order_not_found_markup(&tx_hash, q.product.as_deref(), lang)
        }
    };
    let (title, description) = match lang {
        Lang::En => (
            "Order status — PointSav Software".to_string(),
            "Check a PointSav software purchase by transaction hash.".to_string(),
        ),
        Lang::Es => (
            "Estado del pedido — PointSav Software".to_string(),
            "Consulte una compra de software PointSav por hash de transacci\u{f3}n.".to_string(),
        ),
    };
    let body = ui::render_page(
        SoftwareSurface::Marketplace,
        lang,
        &title,
        &description,
        &lang.localize(&format!("/order/{tx_hash}")),
        true,
        content,
    )
    .into_string();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

// GET /order/:tx_hash/download?product=<id> — mints a fresh download-auth token
// (see `mint_license_token` doc comment) and redirects to the authenticated
// app-privategit-source-2 URL. Re-resolves the tx_hash rather than trusting the
// query param alone, so a customer can't mint a token for a product they didn't
// actually pay for by editing the URL.
async fn order_download(
    State(state): State<Arc<AppState>>,
    Path(tx_hash): Path<String>,
    Query(q): Query<OrderQuery>,
) -> Response {
    let tx_hash = tx_hash.to_lowercase();
    let product_id = match resolve_license(&state, &tx_hash).await {
        LicenseOutcome::Confirmed { product_id, .. } => product_id,
        _ => {
            return (
                StatusCode::FORBIDDEN,
                [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                "payment not confirmed for this order",
            )
                .into_response();
        }
    };
    // Defensive check: if the caller supplied ?product=, it must match the
    // resolved receipt's product — catches a stale/copy-pasted link, doesn't
    // trust the query param as the source of truth.
    if let Some(claimed) = &q.product {
        if claimed != &product_id {
            return (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                "product mismatch for this order",
            )
                .into_response();
        }
    }
    let Some(signing_key) = &state.signing_key else {
        tracing::warn!(product_id = %product_id, "order-download: SIGNING_KEY_SECRET not set");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            "download signing not configured",
        )
            .into_response();
    };
    let (edition, platform_slug, bundled) = load_catalog(&state.catalog_path)
        .ok()
        .and_then(|c| {
            c.installers.iter().find(|i| i.id == product_id).map(|i| {
                (
                    i.edition.clone(),
                    i.platform_slug().to_string(),
                    i.bundled_registry_products.clone(),
                )
            })
        })
        .unwrap_or_else(|| ("latest".to_string(), "linux-x86_64".to_string(), Vec::new()));
    let mut entitled_products = vec![product_id.clone()];
    entitled_products.extend(bundled);
    let token = mint_license_token(signing_key, &product_id, &entitled_products);
    let target = format!(
        "{}/{}/{}/{}?token={}",
        state.source_base_url, product_id, edition, platform_slug, token
    );
    Redirect::to(&target).into_response()
}

// GET /v1/claim-license/:product_id — free-tier download-auth minting for
// `license_tier: apache` products. Apache-tier products are permanently priced
// at $0 (the new licensing architecture's free shelf), so there is no payment
// to confirm and no tx_hash to resolve — unlike `order_download`, this handler
// never touches `resolve_license`/wallet/receipt state at all. Mints a fresh
// token on every call (re-visiting the URL just re-mints, matching how a
// customer might bookmark and revisit it) and redirects to the same
// `app-privategit-source-2` URL shape `order_download` uses.
async fn claim_free_license(
    State(state): State<Arc<AppState>>,
    Path(product_id): Path<String>,
) -> Response {
    let installer = match load_catalog(&state.catalog_path) {
        Ok(catalog) => catalog.installers.into_iter().find(|i| i.id == product_id),
        Err(e) => {
            tracing::error!("claim-free-license: catalog load failed: {e:#}");
            None
        }
    };
    let Some(installer) = installer else {
        return (
            StatusCode::NOT_FOUND,
            Json(err_json("product-not-found", "no such product")),
        )
            .into_response();
    };
    if installer.license_tier != LicenseTier::Apache {
        return (
            StatusCode::FORBIDDEN,
            Json(err_json(
                "not-free-tier",
                "this product requires a paid license",
            )),
        )
            .into_response();
    }
    let Some(signing_key) = &state.signing_key else {
        tracing::warn!(product_id = %product_id, "claim-free-license: SIGNING_KEY_SECRET not set");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(err_json(
                "signing-not-configured",
                "download signing not configured",
            )),
        )
            .into_response();
    };
    let mut entitled_products = vec![product_id.clone()];
    entitled_products.extend(installer.bundled_registry_products.clone());
    let token = mint_license_token(signing_key, &product_id, &entitled_products);
    let target = format!(
        "{}/{}/{}/{}?token={}",
        state.source_base_url,
        product_id,
        installer.edition,
        installer.platform_slug(),
        token
    );
    Redirect::to(&target).into_response()
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    // P1 testing binds to a test port. Production port (9202) is owned by the
    // live app-privategit-software service and must not be touched here.
    let bind_addr = std::env::var("MARKETPLACE_BIND").unwrap_or_else(|_| "127.0.0.1:9202".into());
    let catalog_path = PathBuf::from(
        std::env::var("CATALOG_PATH")
            .unwrap_or_else(|_| "/var/lib/local-software/catalog/products.yaml".into()),
    );
    let static_dir = PathBuf::from(std::env::var("STATIC_DIR").unwrap_or_else(|_| "static".into()));
    let polygon_wallet_address = std::env::var("POLYGON_WALLET_ADDRESS").unwrap_or_default();
    let receipts_dir = PathBuf::from(
        std::env::var("RECEIPTS_DIR").unwrap_or_else(|_| "/var/lib/local-software/receipts".into()),
    );
    let claims_dir = PathBuf::from(
        std::env::var("CLAIMS_DIR").unwrap_or_else(|_| "/var/lib/local-software/claims".into()),
    );
    let source_base_url = std::env::var("SOURCE_BASE_URL")
        .unwrap_or_else(|_| "https://software.pointsav.com/releases".into());
    let polygon_rpc_url =
        std::env::var("POLYGON_RPC_URL").unwrap_or_else(|_| "https://polygon-rpc.com".into());
    let tool_wallet_bin = std::env::var("TOOL_WALLET_BIN").unwrap_or_else(|_| "tool-wallet".into());
    let signing_key = std::env::var("SIGNING_KEY_SECRET")
        .ok()
        .and_then(|path| load_signing_key(&path));
    if signing_key.is_none() {
        tracing::warn!("SIGNING_KEY_SECRET not set — /order/:tx_hash/download will return 503");
    } else if let Some(sk) = &signing_key {
        keypair_selftest(sk);
    }
    let tx_log_path = PathBuf::from(
        std::env::var("TX_LOG_PATH")
            .unwrap_or_else(|_| "/var/lib/local-software/tx-log.jsonl".into()),
    );
    let usdc_cad_spot_rate = std::env::var("USDC_CAD_SPOT_RATE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.37);
    let csp_value = csp_value(&source_base_url);

    let state = Arc::new(AppState {
        catalog_path,
        static_dir: static_dir.clone(),
        polygon_wallet_address,
        receipts_dir,
        claims_dir,
        source_base_url,
        polygon_rpc_url,
        tool_wallet_bin,
        signing_key,
        tx_log_path,
        usdc_cad_spot_rate,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/software", get(software_page))
        .route("/software/:product_id", get(product_detail_page))
        .route("/licensing", get(licensing_page))
        .route("/pricing", get(pricing_page))
        .route("/working-papers", get(working_papers_index_page))
        .route("/working-papers/:slug", get(working_paper_detail_page))
        // Spanish variants (operator-approved 2026-07-12 MVL, extended to full-site
        // parity 2026-07-13) — see `ui::lang` module docs.
        .route("/es", get(root_es))
        .route("/es/software", get(software_page_es))
        .route("/es/software/:product_id", get(product_detail_page_es))
        .route("/es/licensing", get(licensing_page_es))
        .route("/es/pricing", get(pricing_page_es))
        .route("/page/disclaimer", get(disclaimer_page))
        .route("/page/privacy", get(privacy_page))
        .route("/page/accessibility", get(accessibility_page))
        .route("/page/contact", get(contact_page))
        .route("/es/page/disclaimer", get(disclaimer_page_es))
        .route("/es/page/privacy", get(privacy_page_es))
        .route("/es/page/accessibility", get(accessibility_page_es))
        .route("/es/page/contact", get(contact_page_es))
        .route("/robots.txt", get(robots_txt))
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/healthz", get(healthz))
        .route("/v1/products", get(v1_products))
        .route("/v1/license/:tx_hash", get(v1_license))
        .route("/v1/claim", post(v1_claim))
        .route("/v1/wallet/address", get(v1_wallet_address))
        .route("/checkout/:product_id", get(checkout_page))
        .route("/es/checkout/:product_id", get(checkout_page_es))
        .route("/order", get(order_redirect))
        .route("/es/order", get(order_redirect_es))
        .route("/order/:tx_hash", get(order_status_page))
        .route("/es/order/:tx_hash", get(order_status_page_es))
        .route("/order/:tx_hash/download", get(order_download))
        .route("/v1/claim-license/:product_id", get(claim_free_license))
        .fallback(not_found_fallback)
        // L4 fix: `.fallback()` above only fires for paths that match no route at
        // all. `/static/*` IS a matched route (a nested service) -- a missing file
        // under it returns axum/tower_http's bare empty-body 404 straight from
        // `ServeDir`, bypassing the site's chrome entirely. `not_found_service`
        // routes that specific case through the same chromed page.
        .nest_service(
            "/static",
            ServeDir::new(static_dir).not_found_service(get(not_found_fallback)),
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
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_str(&csp_value).expect("csp_value must be a valid header value"),
        ))
        .with_state(state);

    tracing::info!("app-privategit-marketplace-2 listening on {bind_addr}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// SAFETY: no test binds a TCP port (handlers are called directly) and every test
// writes ONLY under a unique scratch dir inside `std::env::temp_dir()` (`/tmp`).
// Nothing here touches `/var/lib/local-software/` or ports 9201/9202. The
// subprocess path is exercised via a locally-written JSON test-double — no real
// Polygon RPC call is ever made.
#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SEQ: AtomicUsize = AtomicUsize::new(0);

    // ── url_origin / csp_value (S2/M10 CSP connect-src fix) ────────────────────

    #[test]
    fn url_origin_strips_path_from_full_url() {
        assert_eq!(
            url_origin("https://software.pointsav.com/releases"),
            "https://software.pointsav.com"
        );
        assert_eq!(
            url_origin("http://127.0.0.1:9201/some/path"),
            "http://127.0.0.1:9201"
        );
    }

    #[test]
    fn url_origin_bare_origin_unchanged() {
        assert_eq!(
            url_origin("https://software.pointsav.com"),
            "https://software.pointsav.com"
        );
    }

    #[test]
    fn url_origin_falls_back_to_input_when_not_a_url() {
        assert_eq!(url_origin("not-a-url"), "not-a-url");
    }

    #[test]
    fn csp_value_allows_source_origin_via_connect_src() {
        let csp = csp_value("http://127.0.0.1:9201/releases");
        assert!(csp.contains("connect-src 'self' http://127.0.0.1:9201"));
        // Still a real, complete policy — not just the one directive.
        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains("frame-ancestors 'none'"));
    }

    /// Fresh, unique scratch directory under /tmp for one test.
    fn scratch_dir(tag: &str) -> PathBuf {
        let n = SEQ.fetch_add(1, Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "mkt2-test-{tag}-{}-{n}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Deterministic, format-valid (`0x` + 64 hex) tx_hash for tests. Real code
    /// now rejects any tx_hash not matching this exact shape (S1/C1 fixes), so
    /// the old short readable placeholders like `"0xconfirmedpayment01"` are no
    /// longer valid input. `kind` is a fixed 2-hex-char prefix the
    /// `write_tool_wallet_double` mock matches on ("c0" = confirmed, "be" =
    /// pending, "00" = neither -- arbitrary but fixed on both sides, since hex
    /// can't spell "confirmed"/"pending" the way the old placeholders did).
    /// `seed` differentiates otherwise-identical tx hashes (e.g. two different
    /// confirmed transactions) without needing embedded English words.
    fn test_tx_hash(kind: &str, seed: &str) -> String {
        debug_assert_eq!(kind.len(), 2, "kind must be exactly 2 hex chars");
        let body = hex::encode(Sha256::digest(seed.as_bytes()));
        format!("0x{kind}{}", &body[..62])
    }

    /// Minimal products.yaml with a realistic paid tier (os-console = $1.00 =
    /// 1_000_000 micro-USDC, os-mediakit = $19.00 = 19_000_000 micro-USDC), written
    /// into `dir`. Returns the catalog path. Prices are ACTIVE (nonzero) here
    /// deliberately, to preserve test coverage of the paid path — unlike the real
    /// products.yaml, which ships all products at price_usdc: 0 during the current
    /// BETA gate (see the `Installer::price_usdc` doc comment).
    fn write_catalog(dir: &std::path::Path) -> PathBuf {
        let path = dir.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: os-console
    name: PointSav Console OS
    description: Operator Terminal Surface.
    edition: "2026.05.144"
    platform: "macOS · Win · Linux"
    size_mb: 412
    path: os-console/2026.05.144
    license_tier: agpl
    price_usdc: 1000000
    tier: delivery
    family: totebox_orchestration
  - id: os-mediakit
    name: PointSav MediaKit OS
    description: MediaKit Surface.
    edition: "2026.05.142"
    platform: "Linux server"
    size_mb: 96
    path: os-mediakit/2026.05.142
    license_tier: fsl
    price_usdc: 19000000
    tier: delivery
    family: independent
"#,
        )
        .unwrap();
        path
    }

    /// Write an executable bash test-double that mimics `tool-wallet check`.
    /// Branches on the (lowercased) tx_hash argument:
    ///   *confirmed* -> confirmed:true, $1.00 payment, exit 0
    ///   *pending*   -> confirmed:false, exit 0
    ///   otherwise   -> confirmed:false + exit 1 (mirrors real tool-wallet's not-found)
    fn write_tool_wallet_double(dir: &std::path::Path) -> PathBuf {
        let path = dir.join("tool-wallet-double.sh");
        fs::write(
            &path,
            r#"#!/usr/bin/env bash
# args: check <tx_hash> --rpc-url <url> --wallet-address <addr>
# Matches by fixed hex prefix, not embedded English words -- real tx_hash
# values are now validated as 0x + 64 hex (S1/C1 fixes), so a placeholder like
# the old "*confirmed*"/"*pending*" substring can't appear in valid input.
# See test_tx_hash()'s doc comment for the kind convention (c0/be/00).
tx="$2"
case "$tx" in
  0xc0*) echo '{"confirmed":true,"amount_usdc":1.00,"amount_units":1000000,"from":"0xcaffee","block":123,"tx_hash":"'"$tx"'"}'; exit 0;;
  0xbe*) echo '{"confirmed":false,"reason":"not yet mined"}'; exit 0;;
  *)     echo '{"confirmed":false,"reason":"transaction not found"}'; exit 1;;
esac
"#,
        )
        .unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
        path
    }

    /// Deterministic in-test Ed25519 signing key. Never a production key — matches
    /// `app-privategit-source-2`'s own `test_signing_key()` seed convention so the
    /// two crates' test fixtures could interoperate if ever cross-checked.
    fn test_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[42u8; 32])
    }

    /// A second, different keypair — for keypair-self-test mismatch cases.
    fn other_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[43u8; 32])
    }

    #[test]
    fn keypair_selftest_skipped_when_verify_key_pub_absent() {
        assert_eq!(
            keypair_selftest_check(&test_signing_key(), None),
            KeypairSelftestOutcome::Skipped
        );
    }

    #[test]
    fn keypair_selftest_invalid_when_verify_key_pub_unreadable() {
        assert_eq!(
            keypair_selftest_check(&test_signing_key(), Some("not-hex-and-not-a-file")),
            KeypairSelftestOutcome::Invalid
        );
    }

    #[test]
    fn keypair_selftest_matches_when_keys_correspond() {
        let sk = test_signing_key();
        let hex_vk = hex::encode(sk.verifying_key().to_bytes());
        assert_eq!(
            keypair_selftest_check(&sk, Some(&hex_vk)),
            KeypairSelftestOutcome::Match
        );
    }

    #[test]
    fn keypair_selftest_mismatch_when_keys_differ() {
        let sk = test_signing_key();
        let wrong_hex_vk = hex::encode(other_signing_key().verifying_key().to_bytes());
        match keypair_selftest_check(&sk, Some(&wrong_hex_vk)) {
            KeypairSelftestOutcome::Mismatch { actual, expected } => {
                assert_eq!(actual, hex::encode(sk.verifying_key().to_bytes()));
                assert_eq!(expected, wrong_hex_vk);
            }
            other => panic!("expected Mismatch, got {other:?}"),
        }
    }

    fn test_state(scratch: &std::path::Path, tool_wallet_bin: String) -> Arc<AppState> {
        Arc::new(AppState {
            catalog_path: write_catalog(scratch),
            static_dir: scratch.to_path_buf(),
            polygon_wallet_address: "0xTESTWALLET".into(),
            receipts_dir: scratch.join("receipts"),
            claims_dir: scratch.join("claims"),
            source_base_url: "https://example.invalid/releases".into(),
            polygon_rpc_url: "https://rpc.invalid".into(),
            tool_wallet_bin,
            signing_key: Some(test_signing_key()),
            tx_log_path: scratch.join("tx-log.jsonl"),
            usdc_cad_spot_rate: 1.37,
        })
    }

    /// Richer products.yaml fixture for the P1–P3 tests: one BETA/free product per
    /// tier plus one actively-priced product — the full shape production can serve
    /// once a BETA gate lifts, even though the real catalog ships all-zero today.
    fn write_full_catalog(dir: &std::path::Path) -> PathBuf {
        let path = dir.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: os-mediakit
    name: MediaKit OS
    description: Sovereign media workstation image.
    edition: "1.2.0"
    platform: linux-x86_64
    size_mb: 812
    path: os-mediakit/1.2.0/installer.run
    license_tier: fsl
    price_usdc: 0
    tier: delivery
    family: independent
  - id: os-console
    name: Console OS
    description: Operator Terminal Surface, free during BETA.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 300
    path: os-console/1.0.0/installer.run
    license_tier: agpl
    price_usdc: 0
    tier: delivery
    family: totebox_orchestration
  - id: os-privategit
    name: PrivateGit OS
    description: Independent code repository, priced tier (test fixture only).
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 150
    path: os-privategit/1.0.0/installer.run
    license_tier: fsl
    price_usdc: 1000000
    tier: delivery
    family: independent
"#,
        )
        .unwrap();
        path
    }

    /// State whose catalog path is caller-supplied (P1–P3 tests; never shells out,
    /// so `tool_wallet_bin` is the PATH default and irrelevant).
    fn test_state_at(scratch: &std::path::Path, catalog_path: PathBuf) -> Arc<AppState> {
        Arc::new(AppState {
            catalog_path,
            static_dir: scratch.to_path_buf(),
            polygon_wallet_address: "0xTESTWALLET".into(),
            receipts_dir: scratch.join("receipts"),
            claims_dir: scratch.join("claims"),
            source_base_url: "https://example.invalid/releases".into(),
            polygon_rpc_url: "https://rpc.invalid".into(),
            tool_wallet_bin: "tool-wallet".into(),
            signing_key: Some(test_signing_key()),
            tx_log_path: scratch.join("tx-log.jsonl"),
            usdc_cad_spot_rate: 1.37,
        })
    }

    fn test_state_full(scratch: &std::path::Path) -> Arc<AppState> {
        let catalog_path = write_full_catalog(scratch);
        test_state_at(scratch, catalog_path)
    }

    /// Collect a handler `Response` body into a String (handlers are called
    /// directly — no TCP port is ever bound).
    async fn body_text(body: axum::body::Body) -> String {
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    // ── Pure functions ────────────────────────────────────────────────────────

    #[test]
    fn license_key_construction_is_exact() {
        // Must stay byte-identical to the OLD binary + tool-wallet.
        let key = generate_license_key("apache", "0xabc", "0xcaffee");
        let full = hex::encode(Sha256::digest(b"apache:0xabc:0xcaffee"));
        let expected = format!(
            "{}-{}-{}-{}",
            &full[0..8],
            &full[8..16],
            &full[16..24],
            &full[24..32]
        );
        assert_eq!(key, expected);
        // Shape: four hyphen-joined 8-hex-char groups.
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert!(parts
            .iter()
            .all(|p| p.len() == 8 && p.chars().all(|c| c.is_ascii_hexdigit())));
    }

    #[test]
    fn price_units_conversion_is_unchanged() {
        // Dollars -> micro-USDC. THIS conversion is correct and not the bug.
        assert_eq!(price_units_from_amount(1.00), 1_000_000);
        assert_eq!(price_units_from_amount(19.00), 19_000_000);
    }

    /// THE reviewable proof: for a realistic $1.00 os-console-tier payment, the OLD
    /// formula fails to match and the NEW (fixed) formula matches. Catalog
    /// `price_usdc` is already micro-USDC (os-console = 1_000_000).
    #[test]
    fn pricing_fix_old_formula_fails_new_formula_matches() {
        let scratch = scratch_dir("pricematch");
        let catalog = load_catalog(&write_catalog(&scratch)).unwrap();

        // A confirmed $1.00 payment -> 1_000_000 micro-USDC.
        let price_units = price_units_from_amount(1.00);
        assert_eq!(price_units, 1_000_000);

        // OLD (buggy) matcher, reproduced verbatim for the before/after proof:
        //   i.price_usdc * 1_000_000 == price_units
        // os-console.price_usdc (1_000_000) * 1_000_000 = 1_000_000_000_000 != 1_000_000.
        let old_match: Option<String> = catalog
            .installers
            .iter()
            .find(|i| i.price_usdc * 1_000_000 == price_units)
            .map(|i| i.id.clone());
        assert_eq!(
            old_match, None,
            "OLD formula must FAIL to match a real $1.00 payment (this was the bug)"
        );

        // NEW (fixed) matcher: direct micro-unit equality.
        let new_match = match_license_product_id(&catalog, price_units);
        assert_eq!(
            new_match,
            Some("os-console".to_string()),
            "NEW formula must match os-console for a $1.00 payment"
        );

        // Sanity: $19.00 -> os-mediakit under the fix as well.
        assert_eq!(
            match_license_product_id(&catalog, price_units_from_amount(19.00)),
            Some("os-mediakit".to_string())
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    /// Checkpoint 2 review finding: the float round-trip in
    /// `price_units_from_amount` is lossy for many whole-cent prices. $2.01 is a
    /// concrete failure case — verify it, then verify `price_units_from_check_json`
    /// avoids it by preferring the exact `amount_units` integer.
    #[test]
    fn amount_units_precision_fix() {
        // The lossy float path: tool-wallet's own display conversion
        // (amount_units as f64 / 1_000_000.0) fed back through
        // price_units_from_amount does NOT reliably round-trip.
        let amount_units: u64 = 2_010_000; // a genuine $2.01 payment
        let amount_usdc = amount_units as f64 / 1_000_000.0;
        let float_roundtrip = price_units_from_amount(amount_usdc);
        assert_ne!(
            float_roundtrip, amount_units,
            "float round-trip must reproduce the known $2.01 precision loss \
             (if this now passes, tool-wallet's amount_usdc formatting or Rust's \
             float behavior changed — re-verify the exact-integer path is still \
             the one actually used in production before relaxing this test)"
        );

        // The fixed path: given tool-wallet's real response shape (both fields
        // present, as it always emits), the exact integer wins.
        let check_json = json!({
            "confirmed": true,
            "amount_usdc": amount_usdc,
            "amount_units": amount_units,
            "from": "0xbuyer",
            "block": 1
        });
        assert_eq!(
            price_units_from_check_json(&check_json),
            amount_units,
            "must use the exact amount_units field, not the lossy float"
        );

        // Defensive fallback: if amount_units is ever absent, the float path is
        // still exercised (documented limitation, not silently broken).
        let check_json_no_units = json!({
            "confirmed": true,
            "amount_usdc": 1.00,
            "from": "0xbuyer",
            "block": 1
        });
        assert_eq!(price_units_from_check_json(&check_json_no_units), 1_000_000);
    }

    // ── Handler: receipt-cache path ───────────────────────────────────────────

    #[tokio::test]
    async fn license_confirmed_via_existing_receipt() {
        let scratch = scratch_dir("receipt");
        // tool_wallet_bin points at a non-existent binary: this path must NOT shell out.
        let state = test_state(&scratch, "/nonexistent/tool-wallet".into());

        let tx = test_tx_hash("00", "deadbeefreceipt");
        let rpath = receipt_path(&state.receipts_dir, &tx);
        fs::create_dir_all(rpath.parent().unwrap()).unwrap();
        // Fixture INCLUDES `license_tier` — the extra field tool-wallet writes.
        // It must be ignored on read (proves cross-binary receipt compatibility).
        fs::write(
            &rpath,
            format!(
                r#"{{
  "product_id": "apache",
  "license_tier": "apache",
  "version": "0.0.1",
  "customer_ref": "0xcaffee",
  "price_usdc": 1000000,
  "tx_hash": "{tx}",
  "chain": "polygon-pos",
  "confirmed_at": "2026-07-01T00:00:00+00:00",
  "block_number": 123,
  "license_key": "aaaaaaaa-bbbbbbbb-cccccccc-dddddddd"
}}"#
            ),
        )
        .unwrap();

        let (status, Json(body)) = v1_license(State(state.clone()), Path(tx.clone())).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "confirmed");
        assert_eq!(body["product_id"], "apache");
        assert_eq!(body["license_key"], "aaaaaaaa-bbbbbbbb-cccccccc-dddddddd");
        assert_eq!(body["customer_ref"], "0xcaffee");

        // Phase 5: a receipt-cache replay is NOT a new sale — must not append a
        // tx-log row (only the fresh-confirmation path does).
        assert!(
            !state.tx_log_path.exists(),
            "receipt-cache replay must not write to the tx log"
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Handler: fresh tool-wallet check (mocked) ─────────────────────────────

    #[tokio::test]
    async fn license_confirmed_via_fresh_check_matches_apache_and_writes_receipt() {
        let scratch = scratch_dir("fresh");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());

        let tx = test_tx_hash("c0", "confirmedpayment01");
        let (status, Json(body)) = v1_license(State(state.clone()), Path(tx.clone())).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "confirmed");
        // The FIX in action end-to-end: $1.00 -> 1_000_000 -> catalog "os-console".
        assert_eq!(body["product_id"], "os-console");
        assert_eq!(body["customer_ref"], "0xcaffee");
        let expected_key = generate_license_key("os-console", &tx, "0xcaffee");
        assert_eq!(body["license_key"], expected_key);

        // A receipt must have been written to the SCRATCH dir (never /var/lib).
        let rpath = receipt_path(&state.receipts_dir, &tx);
        assert!(
            rpath.exists(),
            "receipt should be persisted to scratch receipts dir"
        );
        assert!(rpath.starts_with(&scratch));
        let written: LicenseReceipt =
            serde_json::from_str(&fs::read_to_string(&rpath).unwrap()).unwrap();
        assert_eq!(written.product_id, "os-console");
        assert_eq!(written.price_usdc, 1_000_000); // micro-USDC, not dollars

        // Phase 5: a fresh confirmation must append exactly one correctly-shaped
        // tx-log.jsonl row (the CRA barter-transaction record).
        let log_contents = fs::read_to_string(&state.tx_log_path).unwrap();
        let lines: Vec<&str> = log_contents.lines().collect();
        assert_eq!(lines.len(), 1, "exactly one row for one fresh confirmation");
        let row: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(row["sku"], "os-console@2026.05.144");
        assert_eq!(row["license_tier"], "AGPL-3.0-or-later");
        assert_eq!(row["crypto_received"], "1.00 USDC");
        assert_eq!(row["polygon_tx"], tx);
        assert_eq!(row["spot_rate_cad"], "1.37");
        assert_eq!(row["cad_equivalent"], "1.37");

        let _ = fs::remove_dir_all(&scratch);
    }

    // Checkpoint 3b deferral (operator decision 2026-07-02): the first real transaction
    // through this crate must be flagged distinctly for after-the-fact manual review.
    #[tokio::test]
    async fn first_real_confirmation_writes_marker_second_does_not() {
        let scratch = scratch_dir("firstlive");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());

        let marker = first_live_transaction_marker_path(&state.receipts_dir);
        assert!(!marker.exists(), "no marker before any transaction");

        let tx1 = test_tx_hash("c0", "confirmedpayment01");
        let tx2 = test_tx_hash("c0", "confirmedpayment19");

        // First confirmed transaction -> marker written.
        let (status, _) = v1_license(State(state.clone()), Path(tx1.clone())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            marker.exists(),
            "marker must exist after the first confirmation"
        );
        let recorded: LicenseReceipt =
            serde_json::from_str(&fs::read_to_string(&marker).unwrap()).unwrap();
        assert_eq!(recorded.tx_hash, tx1);
        let first_marker_mtime = fs::metadata(&marker).unwrap().modified().unwrap();

        // A second, DIFFERENT confirmed transaction must NOT overwrite the marker --
        // it stays pointing at the first one.
        let (status2, _) = v1_license(State(state.clone()), Path(tx2)).await;
        assert_eq!(status2, StatusCode::OK);
        let still_recorded: LicenseReceipt =
            serde_json::from_str(&fs::read_to_string(&marker).unwrap()).unwrap();
        assert_eq!(
            still_recorded.tx_hash, tx1,
            "marker must still point at the FIRST transaction, not be overwritten by the second"
        );
        assert_eq!(
            fs::metadata(&marker).unwrap().modified().unwrap(),
            first_marker_mtime,
            "marker file must not be rewritten on the second transaction"
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn license_pending_when_check_unconfirmed() {
        let scratch = scratch_dir("pending");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());

        let (status, Json(body)) =
            v1_license(State(state), Path(test_tx_hash("be", "pendingtx01"))).await;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body["status"], "pending");
        assert_eq!(body["code"], "payment-pending");
        assert_eq!(body["retry_after"], 30);

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn license_not_found_when_check_fails() {
        let scratch = scratch_dir("notfound");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());

        // Neither *confirmed* nor *pending* -> test-double exits 1 -> NotFound.
        let (status, Json(body)) =
            v1_license(State(state), Path(test_tx_hash("00", "unknowntx01"))).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["status"], "not_found");
        assert_eq!(body["code"], "tx-not-found");

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn not_found_when_tool_wallet_binary_missing() {
        let scratch = scratch_dir("nobin");
        let state = test_state(&scratch, "/nonexistent/tool-wallet".into());

        let (status, Json(body)) =
            v1_license(State(state), Path(test_tx_hash("00", "anytx01"))).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["status"], "not_found");
        assert_eq!(body["code"], "tx-not-found");

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Handler: wallet address + claim ───────────────────────────────────────

    #[tokio::test]
    async fn wallet_address_shape_and_hardcoded_contract() {
        let scratch = scratch_dir("wallet");
        let state = test_state(&scratch, "tool-wallet".into());
        let Json(body) = v1_wallet_address(State(state)).await;
        assert_eq!(body["address"], "0xTESTWALLET");
        assert_eq!(body["chain"], "polygon-pos");
        assert_eq!(body["token"], "USDC");
        assert_eq!(
            body["contract"],
            "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    const VALID_SHA256_HEX: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const VALID_WALLET_ADDRESS: &str = "0x1234567890123456789012345678901234567890";

    #[tokio::test]
    async fn claim_writes_placeholder_and_returns_ok() {
        let scratch = scratch_dir("claim");
        let state = test_state(&scratch, "tool-wallet".into());
        let req = ClaimRequest {
            binary_sha256: VALID_SHA256_HEX.into(),
            wallet_address: VALID_WALLET_ADDRESS.into(),
        };
        let (status, Json(body)) = v1_claim(State(state.clone()), Json(req)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ok");
        assert_eq!(body["note"], "on-chain mint arrives v0.0.2");
        assert!(body["token"].as_str().unwrap().len() == 64);
        // File written under claims/<addr-without-0x>/<first16>.json in scratch.
        let claim_file = state
            .claims_dir
            .join("1234567890123456789012345678901234567890")
            .join("0123456789abcdef.json");
        assert!(claim_file.exists());
        let _ = fs::remove_dir_all(&scratch);
    }

    // C1/S2 regression tests: malformed binary_sha256/wallet_address used to
    // reach a byte-slice panic (short/non-ASCII sha256) or an arbitrary
    // filesystem write (absolute-looking wallet_address). Both must now be
    // rejected with 400, never reach fs::create_dir_all/fs::write at all.
    #[tokio::test]
    async fn claim_rejects_short_binary_sha256_instead_of_panicking() {
        let scratch = scratch_dir("claim-short-sha");
        let state = test_state(&scratch, "tool-wallet".into());
        let req = ClaimRequest {
            binary_sha256: "tooshort".into(),
            wallet_address: VALID_WALLET_ADDRESS.into(),
        };
        let (status, Json(body)) = v1_claim(State(state), Json(req)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body["code"], "invalid-claim-fields");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn claim_rejects_non_char_boundary_binary_sha256_instead_of_panicking() {
        let scratch = scratch_dir("claim-multibyte-sha");
        let state = test_state(&scratch, "tool-wallet".into());
        // A multi-byte character straddling byte index 16 -- this exact shape
        // panicked on the old `&s[..16.min(len)]` byte-slice.
        let req = ClaimRequest {
            binary_sha256: "aaaaaaaaaaaaaaaézzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".into(),
            wallet_address: VALID_WALLET_ADDRESS.into(),
        };
        let (status, _) = v1_claim(State(state), Json(req)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn claim_rejects_absolute_path_wallet_address_instead_of_escaping_claims_dir() {
        let scratch = scratch_dir("claim-traversal");
        let state = test_state(&scratch, "tool-wallet".into());
        let evil_target = scratch.join("escaped");
        let req = ClaimRequest {
            binary_sha256: VALID_SHA256_HEX.into(),
            wallet_address: format!("0x{}", evil_target.to_string_lossy()),
        };
        let (status, _) = v1_claim(State(state), Json(req)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(
            !evil_target.exists(),
            "must not have written outside claims_dir"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── P1: catalog loading ───────────────────────────────────────────────────

    #[test]
    fn load_catalog_parses_realistic_fixture() {
        let scratch = scratch_dir("loadcat");
        let catalog = load_catalog(&write_full_catalog(&scratch)).unwrap();

        assert_eq!(catalog.installers.len(), 3);
        let i = &catalog.installers[0];
        assert_eq!(i.id, "os-mediakit");
        assert_eq!(i.name, "MediaKit OS");
        assert_eq!(i.edition, "1.2.0");
        assert_eq!(i.platform, "linux-x86_64");
        assert_eq!(i.size_mb, 812);
        assert_eq!(i.path, "os-mediakit/1.2.0/installer.run");
        assert_eq!(i.license_tier, LicenseTier::Fsl);
        assert_eq!(i.price_usdc, 0); // active BETA gate

        assert_eq!(catalog.installers[1].id, "os-console");
        assert_eq!(catalog.installers[1].license_tier, LicenseTier::Agpl);
        assert_eq!(catalog.installers[1].price_usdc, 0); // active BETA gate

        assert_eq!(catalog.installers[2].id, "os-privategit");
        assert_eq!(catalog.installers[2].license_tier, LicenseTier::Fsl);
        assert_eq!(catalog.installers[2].price_usdc, 1_000_000); // micro-USDC

        let _ = fs::remove_dir_all(&scratch);
    }

    /// Loads the REAL, checked-in `products.yaml` (not a scratch fixture) — a smoke
    /// test that the live catalog still parses under the Phase 1b schema (`tier`
    /// required, `facts` capped at 3). The `tier` omission case itself is now a
    /// compiler-enforced parse failure (`Installer::tier` is `ArchTier`, not
    /// `Option<ArchTier>`) rather than something a runtime test needs to check —
    /// see `products_yaml_missing_a_tier_fails_to_load` below for that guarantee.
    #[test]
    fn real_products_yaml_still_parses_under_the_phase_1b_schema() {
        let path = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/catalog/products.yaml"
        ));
        let catalog = load_catalog(&path).expect("real products.yaml must parse");
        assert!(!catalog.installers.is_empty());
    }

    /// The actual anti-drift guarantee (Phase 1b): a `products.yaml` entry missing
    /// `tier:` must fail to load loudly (a parse error), not silently succeed with
    /// a product that would then render in zero tier sections.
    #[test]
    fn products_yaml_missing_a_tier_fails_to_load() {
        let scratch = scratch_dir("missing-tier");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: os-console
    name: PointSav Console OS
    description: Operator Terminal Surface.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 300
    path: os-console/1.0.0/installer.run
    license_tier: agpl
    price_usdc: 0
"#,
        )
        .unwrap();

        let err = load_catalog(&path);
        assert!(
            err.is_err(),
            "a products.yaml entry with no `tier:` field must fail to load loudly, \
             not silently parse and render in zero tier sections"
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    /// The fact-row cap (Phase 1b) — a product declaring more than 3 `facts:` must
    /// fail to load loudly, not silently truncate to 3 (which would hide a
    /// data-entry mistake rather than surface it).
    #[test]
    fn products_yaml_with_four_facts_fails_to_load() {
        let scratch = scratch_dir("too-many-facts");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: os-console
    name: PointSav Console OS
    description: Operator Terminal Surface.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 300
    path: os-console/1.0.0/installer.run
    license_tier: agpl
    price_usdc: 0
    tier: delivery
    family: totebox_orchestration
    facts:
      - label: "ONE"
        value: "a"
      - label: "TWO"
        value: "b"
      - label: "THREE"
        value: "c"
      - label: "FOUR"
        value: "d"
"#,
        )
        .unwrap();

        let err = load_catalog(&path);
        assert!(
            err.is_err(),
            "a products.yaml entry with 4 facts must fail to load loudly, not \
             silently truncate to 3"
        );
        assert!(err.unwrap_err().to_string().contains("os-console"));

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Phase 1: Binary Library two-shelf model ───────────────────────────────
    // (BRIEF-binary-library-repositioning.md, operator-approved 2026-07-07)

    #[test]
    fn license_tier_apache_round_trips_as_apache() {
        let s = serde_yaml::to_string(&LicenseTier::Apache).unwrap();
        assert_eq!(s.trim(), "apache");
        let back: LicenseTier = serde_yaml::from_str("apache").unwrap();
        assert_eq!(back, LicenseTier::Apache);
    }

    #[test]
    fn license_tier_apache_label() {
        assert_eq!(LicenseTier::Apache.label(), "Apache-2.0 (Open Source)");
    }

    #[test]
    fn shelf_mapping_matches_two_shelf_model() {
        // Rebuilt 2026-07-07 for BRIEF-software-licensing-structure.md's four-tier
        // model: Proprietary/Fsl/Agpl are all the Commercial shelf; Apache alone is
        // the Open Source shelf. See `LicenseTier::shelf`'s doc comment.
        assert_eq!(LicenseTier::Proprietary.shelf(), Shelf::Commercial);
        assert_eq!(LicenseTier::Fsl.shelf(), Shelf::Commercial);
        assert_eq!(LicenseTier::Agpl.shelf(), Shelf::Commercial);
        assert_eq!(LicenseTier::Apache.shelf(), Shelf::OpenSource);
    }

    #[test]
    fn load_catalog_accepts_apache_installer_at_zero_price() {
        let scratch = scratch_dir("oss-ok");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: tool-wallet
    name: PointSav Wallet
    description: Polygon USDC watcher, relicensed Apache-2.0.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 12
    path: tool-wallet/1.0.0/installer.run
    license_tier: apache
    price_usdc: 0
    tier: delivery
    family: independent
"#,
        )
        .unwrap();

        let catalog = load_catalog(&path).unwrap();
        assert_eq!(catalog.installers[0].license_tier, LicenseTier::Apache);
        assert_eq!(
            catalog.installers[0].license_tier.shelf(),
            Shelf::OpenSource
        );
        assert_eq!(catalog.installers[0].price_usdc, 0);

        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn load_catalog_rejects_apache_installer_with_nonzero_price() {
        let scratch = scratch_dir("oss-bad-price");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: tool-wallet
    name: PointSav Wallet
    description: Polygon USDC watcher, relicensed Apache-2.0.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 12
    path: tool-wallet/1.0.0/installer.run
    license_tier: apache
    price_usdc: 1000000
    tier: delivery
    family: independent
"#,
        )
        .unwrap();

        let err = load_catalog(&path);
        assert!(
            err.is_err(),
            "an apache-tier installer with a nonzero price must fail to load loudly, \
             not silently accept an invalid price"
        );
        assert!(err.unwrap_err().to_string().contains("tool-wallet"));

        let _ = fs::remove_dir_all(&scratch);
    }

    #[test]
    fn load_catalog_missing_file_is_err() {
        let missing = PathBuf::from("/nonexistent/mkt2-test/products.yaml");
        assert!(load_catalog(&missing).is_err());
    }

    #[test]
    fn load_catalog_malformed_or_incomplete_yaml_is_err() {
        let scratch = scratch_dir("badcat");
        let path = scratch.join("products.yaml");

        // Syntactically invalid YAML (unterminated flow sequence).
        fs::write(&path, "installers: [this is: not, valid yaml").unwrap();
        assert!(load_catalog(&path).is_err());

        // Well-formed YAML entirely missing the required `installers` field must
        // also fail (an empty `installers: []` list is now valid, not an error).
        fs::write(&path, "not_installers: []\n").unwrap();
        assert!(load_catalog(&path).is_err());

        // A legacy file still carrying the retired `licenses:` key must fail to
        // load LOUDLY (`#[serde(deny_unknown_fields)]` on `Catalog`) — not silently
        // produce an empty-but-successfully-parsed catalog. Protects against a
        // stale prod file surviving the Phase 1 migration undetected.
        fs::write(
            &path,
            "installers: []\nlicenses:\n  - id: apache\n    price_usdc: 1000000\n",
        )
        .unwrap();
        assert!(
            load_catalog(&path).is_err(),
            "a stray legacy licenses: key must fail to parse loudly, not silently \
             produce an empty installers list"
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── P1: /v1/products JSON shape ───────────────────────────────────────────

    /// Regression guard for the Checkpoint 1 finding: every documented field must
    /// be present on every entry — a missing field here is exactly the class of
    /// gap that diff caught.
    #[tokio::test]
    async fn v1_products_documented_field_shape() {
        let scratch = scratch_dir("products");
        let state = test_state_full(&scratch);

        let (status, Json(body)) = v1_products(State(state)).await;
        assert_eq!(status, StatusCode::OK);

        let installers = body["installers"].as_array().unwrap();
        assert_eq!(installers.len(), 3);
        for i in installers {
            for field in [
                "id",
                "name",
                "description",
                "edition",
                "platform",
                "size_mb",
                "download_url",
                "manifest_url",
                "license_tier",
                "price_usdc",
                "cost",
                "payment_address",
                "payment_chain",
                "payment_token",
            ] {
                assert!(
                    i.get(field).is_some(),
                    "installer entry missing documented field `{field}`"
                );
            }
            assert_eq!(i["payment_address"], "0xTESTWALLET");
            assert_eq!(i["payment_chain"], "polygon-pos");
            assert_eq!(i["payment_token"], "USDC");
        }

        let mediakit = &installers[0];
        assert_eq!(mediakit["id"], "os-mediakit");
        assert_eq!(mediakit["edition"], "1.2.0");
        assert_eq!(mediakit["platform"], "linux-x86_64");
        assert_eq!(mediakit["size_mb"], 812);
        assert_eq!(mediakit["license_tier"], "FSL-1.1-ALv2");
        assert_eq!(mediakit["shelf"], "commercial"); // Fsl tier -> Commercial shelf
        assert_eq!(mediakit["price_usdc"], 0);
        assert_eq!(mediakit["cost"], "free"); // active BETA gate
                                              // Built from id/edition/platform_slug (default "linux-x86_64") — not from the
                                              // fixture's `path` field, which never carried a platform segment and would
                                              // have 404'd against the real download route regardless of its value.
        assert_eq!(
            mediakit["download_url"],
            "https://example.invalid/releases/os-mediakit/1.2.0/linux-x86_64"
        );
        assert_eq!(
            mediakit["manifest_url"],
            "https://example.invalid/releases/os-mediakit/1.2.0/MANIFEST"
        );

        let privategit = &installers[2];
        assert_eq!(privategit["id"], "os-privategit");
        assert_eq!(privategit["license_tier"], "FSL-1.1-ALv2");
        assert_eq!(privategit["shelf"], "commercial");
        assert_eq!(privategit["price_usdc"], 1_000_000); // micro-USDC passthrough
        assert_eq!(privategit["cost"], "paid");

        let _ = fs::remove_dir_all(&scratch);
    }

    /// Regression guard: `v1_products`' `download_url` used to point at the
    /// version directory only (missing the `:platform` segment the real
    /// `/releases/:product/:version/:platform` route requires), 404ing for any
    /// consumer that actually followed it — found live against `os-network-admin`.
    /// This asserts the URL resolves to exactly the shape `binary()`/`order_download`
    /// expect, for both the default platform slug and a real per-product override.
    #[tokio::test]
    async fn v1_products_download_url_includes_the_platform_segment() {
        let scratch = scratch_dir("download-url-platform");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: os-network-admin
    name: PointSav Network OS
    description: Orchestrates VM map, private network routing, and provisioning authorisation
    edition: "0.1.0"
    platform: "Linux x86_64"
    size_mb: 1
    path: os-network-admin/0.1.0
    license_tier: fsl
    price_usdc: 0
    platform_slug: x86_64
    tier: infrastructure
    family: independent
"#,
        )
        .unwrap();
        let state = test_state_at(&scratch, path);

        let (status, Json(body)) = v1_products(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        let installers = body["installers"].as_array().unwrap();
        assert_eq!(
            installers[0]["download_url"],
            "https://example.invalid/releases/os-network-admin/0.1.0/x86_64"
        );
        assert_eq!(
            installers[0]["manifest_url"],
            "https://example.invalid/releases/os-network-admin/0.1.0/MANIFEST"
        );
        let _ = fs::remove_dir_all(&scratch);
    }

    /// Binary Library Phase 3: an `apache`-tier installer must report
    /// `shelf: "open-source"` distinctly from `agpl`/`fsl`'s `shelf: "commercial"`
    /// — the JSON API's grouping must never drift from the HTML catalog's.
    #[tokio::test]
    async fn v1_products_apache_tier_reports_open_source_shelf() {
        let scratch = scratch_dir("shelf");
        let path = scratch.join("products.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: tool-wallet
    name: PointSav Wallet
    description: Polygon USDC watcher, relicensed Apache-2.0.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 12
    path: tool-wallet/1.0.0/installer.run
    license_tier: apache
    price_usdc: 0
    tier: delivery
    family: independent
  - id: os-console
    name: Console OS
    description: Operator Terminal Surface.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 300
    path: os-console/1.0.0/installer.run
    license_tier: agpl
    price_usdc: 0
    tier: delivery
    family: totebox_orchestration
"#,
        )
        .unwrap();
        let state = test_state_at(&scratch, path);

        let (status, Json(body)) = v1_products(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        let installers = body["installers"].as_array().unwrap();

        let wallet = &installers[0];
        assert_eq!(wallet["id"], "tool-wallet");
        assert_eq!(wallet["license_tier"], "Apache-2.0 (Open Source)");
        assert_eq!(wallet["shelf"], "open-source");

        let console = &installers[1];
        assert_eq!(console["id"], "os-console");
        assert_eq!(console["shelf"], "commercial");

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn v1_products_500_when_catalog_unavailable() {
        let scratch = scratch_dir("products500");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));

        let (status, Json(body)) = v1_products(State(state)).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            body,
            json!({"error": "catalog unavailable", "code": "catalog-unavailable"})
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── P3: dynamic /software catalog page ────────────────────────────────────

    #[tokio::test]
    async fn software_page_renders_dynamic_catalog_with_chrome() {
        let scratch = scratch_dir("swpage");
        let state = test_state_full(&scratch);

        let (parts, body) = software_page(State(state)).await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;

        // Catalog data actually drives the page (the drift bug this phase fixed).
        assert!(html.contains("os-mediakit"));
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("os-console"));
        assert!(html.contains("os-privategit"));
        assert!(html.contains("AGPL-3.0-or-later"));
        assert!(html.contains("FSL-1.1-ALv2"));
        assert!(
            !html.contains("Apache 2.0"),
            "must not use the factually wrong tier label"
        );
        assert!(html.contains("<title>Products — PointSav Software</title>"));

        // P2 chrome wraps the dynamic content.
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));
        assert!(html.contains(SoftwareSurface::Marketplace.copyright_holder()));

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn software_page_500_when_catalog_unavailable() {
        let scratch = scratch_dir("swpage500");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));

        let (parts, body) = software_page(State(state)).await.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("Catalog unavailable"));
        // Chromed error page (M2 fix), not a bare text/plain body.
        assert!(html.contains("sw-masthead"));

        let _ = fs::remove_dir_all(&scratch);
    }

    // ── P1/P2: /licensing static page (UNCHANGED by the P3 dynamic-catalog work) ──

    const LICENSING_FIXTURE: &str = r#"<!doctype html>
<html><head><title>Licensing</title></head>
<body>
<header class="topnav">OLD LIGHT NAV</header>
<main><h1>Licensing terms</h1><p>Static legal content, verbatim.</p></main>
<footer>OLD THIN FOOTER</footer>
</body></html>"#;

    #[tokio::test]
    async fn licensing_page_serves_static_content_with_chrome() {
        let scratch = scratch_dir("licensing");
        let state = test_state_full(&scratch);
        fs::write(state.static_dir.join("licensing.html"), LICENSING_FIXTURE).unwrap();

        let (parts, body) = licensing_page(State(state)).await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;

        // Static document content served unchanged.
        assert!(html.contains("Licensing terms"));
        assert!(html.contains("Static legal content, verbatim."));

        // Old light chrome stripped; Sovereign chrome mounted.
        assert!(!html.contains("OLD LIGHT NAV"));
        assert!(!html.contains("OLD THIN FOOTER"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));

        // NOT affected by P3: no dynamic catalog cards on /licensing.
        assert!(!html.contains("sw-cat-card"));

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn licensing_page_500_when_static_file_missing() {
        let scratch = scratch_dir("licensing500");
        let state = test_state_full(&scratch); // no licensing.html written

        let (parts, body) = licensing_page(State(state)).await.into_parts();
        assert_eq!(parts.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("Page unavailable"));
        // Chromed error page (M2 fix), not a bare text/plain body.
        assert!(html.contains("sw-masthead"));

        let _ = fs::remove_dir_all(&scratch);
    }

    // Router-level fallback (M2) — any unmatched route gets the site's own chromed
    // 404 instead of axum's default bare-empty body.
    #[tokio::test]
    async fn not_found_fallback_serves_chromed_404() {
        let (parts, body) = not_found_fallback().await.into_parts();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("Page not found"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains("sw-footer"));
    }

    // Self-contained disclaimer page (operator instruction 2026-07-02): no static file,
    // no disk read, no cross-site links — a compile-time constant every time.
    #[tokio::test]
    async fn disclaimer_page_serves_self_contained_content_with_chrome() {
        let (parts, body) = disclaimer_page().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;

        // The genuinely new section (Checkpoint 3a follow-up, 2026-07-02): on-chain
        // USDC payment risk, with no precedent elsewhere in the corpus.
        assert!(html.contains("Polygon"));
        assert!(html.contains("irreversible"));

        // Not an LP-investment disclaimer -- confirms the DISCLAIMER.md securities
        // -offering sections (accredited investor exemptions, PPM references) were
        // correctly dropped, not carried forward unmodified.
        assert!(!html.contains("Private Placement Memorandum"));
        assert!(!html.contains("Accredited Investor"));

        // Sovereign chrome present (same page shell as /software and /licensing).
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));

        // `home.pointsav.com` and `home.woodfinegroup.com` legitimately appear now
        // (2026-07-07 footer redesign, operator-approved) — the footer's "Network"
        // column links out to "PointSav Digital Systems" and "Woodfine Capital
        // Projects" on every page, including this one. That's site navigation, not
        // a legal-content cross-reference (the original concern this test's name
        // refers to — this page's own legal *text* must not duplicate or point at
        // another site's legal text, which it still doesn't), so it doesn't violate
        // the "self-contained" property.
        assert!(html.contains("home.pointsav.com"));
        assert!(html.contains("home.woodfinegroup.com"));
    }

    // Three footer pages closing the long-standing dead-link gap
    // (BRIEF-sovereign-editorial-software.md audit finding #1: /page/contact
    // returning HTTP 0). Same self-contained, no-disk-read pattern as disclaimer_page.

    #[tokio::test]
    async fn privacy_page_serves_self_contained_content_with_chrome() {
        let (parts, body) = privacy_page().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("Polygon"));
        assert!(html.contains("open.source@pointsav.com"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));
    }

    #[tokio::test]
    async fn accessibility_page_serves_self_contained_content_with_chrome() {
        let (parts, body) = accessibility_page().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("WCAG 2.1"));
        assert!(html.contains("open.source@pointsav.com"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));
    }

    #[tokio::test]
    async fn contact_page_serves_self_contained_content_with_chrome() {
        let (parts, body) = contact_page().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(
            parts.headers.get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        let html = body_text(body).await;
        assert!(html.contains("open.source@pointsav.com"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains(SoftwareSurface::Marketplace.trademark_line(Lang::En)));
    }

    // ── Full-site-parity pass (2026-07-13): GET /es/page/* ──────────────────────

    #[tokio::test]
    async fn es_contact_page_translates_content_and_carries_hreflang() {
        let (parts, body) = contact_page_es().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        let html = body_text(body).await;
        assert!(html.contains("Cont\u{e1}ctenos"));
        assert!(html.contains("open.source@pointsav.com"));
        assert!(html.contains(r#"<html lang="es">"#));
        assert!(html.contains(r#"hreflang="en" href="https://software.pointsav.com/page/contact""#));
    }

    #[tokio::test]
    async fn es_disclaimer_page_translates_content() {
        let (parts, body) = disclaimer_page_es().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        let html = body_text(body).await;
        assert!(html.contains("Aviso legal"));
        assert!(html.contains("Sin garant\u{ed}a"));
        assert!(html.contains(r#"<html lang="es">"#));
    }

    #[tokio::test]
    async fn es_privacy_page_translates_content() {
        let (parts, body) = privacy_page_es().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        let html = body_text(body).await;
        assert!(html.contains("Privacidad"));
        assert!(html.contains(r#"<html lang="es">"#));
    }

    #[tokio::test]
    async fn es_accessibility_page_translates_content() {
        let (parts, body) = accessibility_page_es().await.into_parts();
        assert_eq!(parts.status, StatusCode::OK);
        let html = body_text(body).await;
        assert!(html.contains("Accesibilidad"));
        assert!(html.contains("WCAG 2.1 AA"));
        assert!(html.contains(r#"<html lang="es">"#));
    }

    // ── Phase 4: GET /pricing ─────────────────────────────────────────────────

    #[tokio::test]
    async fn pricing_page_renders_catalog_driven_content_with_chrome() {
        let scratch = scratch_dir("pricing-ok");
        let state = test_state_full(&scratch);
        let resp = pricing_page(State(state)).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("AGPL-3.0-or-later"));
        // The fixture catalog still has fsl-tier entries (`os-mediakit`, `os-privategit`
        // above) to exercise other code paths, but the ratified three-tier architecture
        // means /pricing never renders an FSL card regardless of catalog content.
        assert!(!html.contains("FSL-1.1-ALv2"));
        assert!(html.contains("currently free during BETA"));
        assert!(html.contains("No tax collected"));
        assert!(html.contains("github.com/pointsav/pointsav-monorepo"));
        assert!(html.contains("sw-masthead"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn pricing_page_500_when_catalog_unavailable() {
        let scratch = scratch_dir("pricing-500");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));
        let resp = pricing_page(State(state)).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let _ = fs::remove_dir_all(&scratch);
    }

    // Regression guard: the real, committed static/licensing.html must never regress
    // to the fictional wallet-connect/multi-chain/tax/fake-product content this
    // phase replaced (a real prior finding, not a hypothetical). `include_str!` is
    // compile-time and path-stable regardless of test-runner CWD.
    #[test]
    fn real_licensing_html_is_free_of_fictional_content() {
        let real = include_str!("../static/licensing.html");
        for fictional in [
            "F*KEYS CONSOLE",
            "Command Centre",
            "Business Applications",
            "BMS Bridge",
            "Peak-Load Forecast",
            "HST 13%",
            "wallet-connect",
            "auto-detected from wallet billing region",
        ] {
            assert!(
                !real.contains(fictional),
                "static/licensing.html regressed: contains fictional content `{fictional}`"
            );
        }
        // CORRECTED 2026-09-23: the 2026-09-04 migration's "FSL was retired entirely"
        // premise was never actually true against canonical — it retiered this crate's
        // `products.yaml` to match project-editorial's *draft* licensing-architecture
        // proposal, not what Command actually ratified/executed. Direct verification
        // against canonical's real SPDX headers found the 2026-09-01 relicense of
        // `os-console`/`os-totebox` off FSL/AGPL was explicitly BLOCKED (real AGPL-tier
        // path dependencies found in the 2026-09-13 linking-boundary audit), and
        // `os-privategit` + its `app-privategit-*` family were never proposed to move
        // off FSL at all. This crate's own `LicenseTier` enum doc comment (unchanged
        // since the 2026-07-07 ratification) documented the correct tiers the whole
        // time. "PointSav Commercial" remains the AGPL tier's paired commercial
        // license, not its own category.
        assert!(real.contains("Proprietary"));
        assert!(real.contains("FSL-1.1-ALv2"));
        assert!(real.contains("AGPL-3.0-or-later"));
        assert!(real.contains("Apache-2.0"));
        assert!(
            real.contains("id=\"fsl\""),
            "FSL tier is real and current for os-privategit/app-privategit-*/os-totebox — \
             their relicense to a more permissive tier is blocked pending dependency work, \
             not executed; the page must not claim otherwise"
        );
        assert!(real.contains("github.com/pointsav/pointsav-monorepo"));
    }

    // ── Phase 2: GET /checkout/:product_id ────────────────────────────────────

    #[tokio::test]
    async fn checkout_page_renders_known_product() {
        let scratch = scratch_dir("checkout-ok");
        let state = test_state_full(&scratch);
        let resp = checkout_page(State(state), Path("os-privategit".to_string())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("PrivateGit OS"));
        assert!(html.contains("$1.00"));
        assert!(html.contains("0xTESTWALLET"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn es_checkout_page_translates_chrome_and_submits_to_es_order() {
        let scratch = scratch_dir("checkout-es-ok");
        let state = test_state_full(&scratch);
        let resp = checkout_page_es(State(state), Path("os-privategit".to_string())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("PrivateGit OS"));
        assert!(html.contains("action=\"/es/order\""));
        assert!(html.contains(r#"<html lang="es">"#));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn checkout_page_404_for_unknown_product() {
        let scratch = scratch_dir("checkout-404");
        let state = test_state_full(&scratch);
        let resp = checkout_page(State(state), Path("no-such-product".to_string())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn checkout_page_500_when_catalog_unavailable() {
        let scratch = scratch_dir("checkout-500");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));
        let resp = checkout_page(State(state), Path("os-console".to_string())).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── S136: GET /software/:product_id ───────────────────────────────────────

    #[tokio::test]
    async fn product_detail_page_renders_known_product() {
        let scratch = scratch_dir("product-detail-ok");
        let state = test_state_full(&scratch);
        let resp = product_detail_page(State(state), Path("os-mediakit".to_string())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("BETA \u{00b7} free"));
        assert!(html.contains("v1.2.0"));
        assert!(html.contains("sw-masthead"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn product_detail_page_404_for_unknown_product() {
        let scratch = scratch_dir("product-detail-404");
        let state = test_state_full(&scratch);
        let resp = product_detail_page(State(state), Path("no-such-product".to_string())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn product_detail_page_500_when_catalog_unavailable() {
        let scratch = scratch_dir("product-detail-500");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));
        let resp = product_detail_page(State(state), Path("os-console".to_string())).await;
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let _ = fs::remove_dir_all(&scratch);
    }

    // /es/software/:product_id follow-up (2026-08-02), scoped in
    // BRIEF-software-spanish-localization.md as deferred out of the original pass.
    #[tokio::test]
    async fn product_detail_page_es_renders_translated_labels() {
        let scratch = scratch_dir("product-detail-es-ok");
        let state = test_state_full(&scratch);
        let resp = product_detail_page_es(State(state), Path("os-mediakit".to_string())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        // Product name/description stay English; static labels translate.
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("Instalaci\u{f3}n"));
        assert!(html.contains("sw-masthead"));
        assert!(html.contains("lang=\"es\""));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn product_detail_page_es_404_for_unknown_product() {
        let scratch = scratch_dir("product-detail-es-404");
        let state = test_state_full(&scratch);
        let resp = product_detail_page_es(State(state), Path("no-such-product".to_string())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("Producto no encontrado"));
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Phase 2: GET /order redirect ──────────────────────────────────────────

    const VALID_TX_HASH_UPPER: &str =
        "0xABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789";
    const VALID_TX_HASH_LOWER: &str =
        "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";

    #[tokio::test]
    async fn order_redirect_builds_canonical_url_and_lowercases_tx_hash() {
        let resp = order_redirect(Query(OrderRedirectQuery {
            product: "os-console".to_string(),
            tx_hash: VALID_TX_HASH_UPPER.to_string(),
        }))
        .await;
        // axum's Redirect::to emits 303 See Other (matches the / -> /software note above).
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            resp.headers().get(header::LOCATION).unwrap(),
            &format!("/order/{VALID_TX_HASH_LOWER}?product=os-console")
        );
    }

    // C1 regression test: a malformed tx_hash/product used to reach Redirect::to()
    // unvalidated, which panics on an invalid header value -- and panic = abort in
    // this workspace turns that into a full process crash on one request. Both
    // order_redirect and order_redirect_es must reject before ever calling
    // Redirect::to, not just return something -- the request must not panic.
    #[tokio::test]
    async fn order_redirect_rejects_malformed_tx_hash_instead_of_panicking() {
        let resp = order_redirect(Query(OrderRedirectQuery {
            product: "os-console".to_string(),
            tx_hash: "not-a-real-hash\nwith-a-newline".to_string(),
        }))
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn order_redirect_rejects_malformed_product_instead_of_panicking() {
        let resp = order_redirect(Query(OrderRedirectQuery {
            product: "os-console\nwith-a-newline".to_string(),
            tx_hash: VALID_TX_HASH_LOWER.to_string(),
        }))
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn order_redirect_es_rejects_malformed_tx_hash_instead_of_panicking() {
        let resp = order_redirect_es(Query(OrderRedirectQuery {
            product: "os-console".to_string(),
            tx_hash: "../../etc/passwd".to_string(),
        }))
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    // ── Phase 2: GET /order/:tx_hash status page ──────────────────────────────

    #[tokio::test]
    async fn order_status_page_confirmed_shows_receipt_and_download_link() {
        let scratch = scratch_dir("order-confirmed");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let tx = test_tx_hash("c0", "confirmedpayment01");
        let resp =
            order_status_page(State(state), Path(tx.clone()), Query(OrderQuery::default())).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("Confirmed"));
        // A $1.00 payment matches os-console in this test's write_catalog fixture.
        assert!(html.contains("os-console"));
        assert!(html.contains(&format!("href=\"/order/{tx}/download?product=os-console\"")));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn es_order_status_page_confirmed_translates_labels() {
        let scratch = scratch_dir("order-confirmed-es");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let tx = test_tx_hash("c0", "confirmedpayment01");
        let resp =
            order_status_page_es(State(state), Path(tx.clone()), Query(OrderQuery::default()))
                .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("Confirmado"));
        assert!(html.contains(r#"<html lang="es">"#));
        // Download link stays unprefixed — no UI to translate, machine link only.
        assert!(html.contains(&format!("href=\"/order/{tx}/download?product=os-console\"")));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_status_page_pending_shows_retry_hint() {
        let scratch = scratch_dir("order-pending");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let resp = order_status_page(
            State(state),
            Path(test_tx_hash("be", "pendingtx01")),
            Query(OrderQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("Pending"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_status_page_not_found_links_back_to_checkout() {
        let scratch = scratch_dir("order-notfound");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let resp = order_status_page(
            State(state),
            Path(test_tx_hash("00", "unknowntx01")),
            Query(OrderQuery {
                product: Some("os-console".to_string()),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let html = body_text(resp.into_body()).await;
        assert!(html.contains("Not found"));
        assert!(html.contains("href=\"/checkout/os-console\""));
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── Phase 2: GET /order/:tx_hash/download — real token minting ───────────

    #[tokio::test]
    async fn order_download_confirmed_mints_token_and_redirects() {
        let scratch = scratch_dir("order-download-ok");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let resp = order_download(
            State(state.clone()),
            Path(test_tx_hash("c0", "confirmedpayment01")),
            Query(OrderQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        let location = resp
            .headers()
            .get(header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        assert!(location.starts_with(
            "https://example.invalid/releases/os-console/2026.05.144/linux-x86_64?token="
        ));

        // The minted token must actually verify against the signing key's public
        // counterpart, matching app-privategit-source-2's exact wire format
        // (base64url_no_pad(sig[64] || payload_json)).
        use ed25519_dalek::Verifier;
        let token = location.split("token=").nth(1).unwrap();
        let bytes = URL_SAFE_NO_PAD.decode(token).unwrap();
        let (sig_bytes, payload_bytes) = bytes.split_at(64);
        let sig = ed25519_dalek::Signature::from_bytes(sig_bytes.try_into().unwrap());
        let vk = test_signing_key().verifying_key();
        assert!(vk.verify(payload_bytes, &sig).is_ok());
        let payload: Value = serde_json::from_slice(payload_bytes).unwrap();
        assert_eq!(payload["product"], "os-console");
        assert_eq!(
            payload["channel_expiry"],
            Utc::now().format("%Y-%m-%d").to_string()
        );
        assert_eq!(payload["entitlements"], json!(["binary"]));

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_download_uses_the_product_s_real_platform_slug_not_a_hardcoded_one() {
        // Regression guard: the paid licensed-download URL used to hardcode
        // "linux-x86_64" regardless of what the product's catalog entry actually
        // says — silently wrong for any non-default `platform_slug` (Fable/Opus
        // audit finding, appliance-image work).
        let scratch = scratch_dir("order-download-platform-slug");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let mut state = (*state).clone();
        let custom_catalog = scratch.join("products-custom-slug.yaml");
        fs::write(
            &custom_catalog,
            r#"installers:
  - id: os-console
    name: PointSav Console OS
    description: Operator Terminal Surface.
    edition: "2026.05.144"
    platform: "macOS · Win · Linux"
    size_mb: 412
    path: os-console/2026.05.144
    license_tier: agpl
    price_usdc: 1000000
    tier: delivery
    family: totebox_orchestration
    platform_slug: aarch64-sel4-microkit
"#,
        )
        .unwrap();
        state.catalog_path = custom_catalog;
        let state = Arc::new(state);

        let resp = order_download(
            State(state),
            Path(test_tx_hash("c0", "confirmedpayment01")),
            Query(OrderQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        let location = resp
            .headers()
            .get(header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        assert!(location.starts_with(
            "https://example.invalid/releases/os-console/2026.05.144/aarch64-sel4-microkit?token="
        ));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_download_403_when_not_confirmed() {
        let scratch = scratch_dir("order-download-403");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        let resp = order_download(
            State(state),
            Path(test_tx_hash("be", "pendingtx01")),
            Query(OrderQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_download_400_on_product_mismatch() {
        let scratch = scratch_dir("order-download-mismatch");
        let double = write_tool_wallet_double(&scratch);
        let state = test_state(&scratch, double.to_string_lossy().into_owned());
        // 0xconfirmedpayment01 is a $1.00 payment -> resolves to os-console; claiming
        // os-mediakit here must be rejected rather than trusted.
        let resp = order_download(
            State(state),
            Path(test_tx_hash("c0", "confirmedpayment01")),
            Query(OrderQuery {
                product: Some("os-mediakit".to_string()),
            }),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn order_download_503_when_signing_key_unconfigured() {
        let scratch = scratch_dir("order-download-503");
        let double = write_tool_wallet_double(&scratch);
        let base_state = test_state(&scratch, double.to_string_lossy().into_owned());
        let state = Arc::new(AppState {
            signing_key: None,
            ..(*base_state).clone()
        });
        let resp = order_download(
            State(state),
            Path(test_tx_hash("c0", "confirmedpayment01")),
            Query(OrderQuery::default()),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── GET /v1/claim-license/:product_id — free-tier (license_tier: apache) ──

    /// Catalog with one `apache`-tier (free-forever) product alongside a normal
    /// paid `agpl`-tier product, so free-vs-paid-tier tests share one fixture.
    fn write_catalog_with_apache_tier(dir: &std::path::Path) -> PathBuf {
        let path = dir.join("products-apache-tier.yaml");
        fs::write(
            &path,
            r#"installers:
  - id: pointsav-design-system
    name: PointSav Design System
    description: Component library and design tokens.
    edition: "1.0.0"
    platform: linux-x86_64
    size_mb: 10
    path: pointsav-design-system/1.0.0
    license_tier: apache
    price_usdc: 0
    tier: delivery
    family: independent
  - id: os-console
    name: PointSav Console OS
    description: Operator Terminal Surface.
    edition: "2026.05.144"
    platform: "macOS · Win · Linux"
    size_mb: 412
    path: os-console/2026.05.144
    license_tier: agpl
    price_usdc: 1000000
    tier: delivery
    family: totebox_orchestration
"#,
        )
        .unwrap();
        path
    }

    #[tokio::test]
    async fn claim_free_license_apache_tier_mints_token_and_redirects() {
        let scratch = scratch_dir("claim-free-license-ok");
        let state = test_state(&scratch, "tool-wallet".into());
        let mut state = (*state).clone();
        state.catalog_path = write_catalog_with_apache_tier(&scratch);
        let state = Arc::new(state);

        let resp =
            claim_free_license(State(state), Path("pointsav-design-system".to_string())).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        let location = resp
            .headers()
            .get(header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        assert!(location.starts_with(
            "https://example.invalid/releases/pointsav-design-system/1.0.0/linux-x86_64?token="
        ));

        // Same wire format as `order_download`'s minted token: verifiable Ed25519
        // signature over the JSON payload, entitled_products carrying just this
        // product (no bundled_registry_products on this fixture entry).
        use ed25519_dalek::Verifier;
        let token = location.split("token=").nth(1).unwrap();
        let bytes = URL_SAFE_NO_PAD.decode(token).unwrap();
        let (sig_bytes, payload_bytes) = bytes.split_at(64);
        let sig = ed25519_dalek::Signature::from_bytes(sig_bytes.try_into().unwrap());
        let vk = test_signing_key().verifying_key();
        assert!(vk.verify(payload_bytes, &sig).is_ok());
        let payload: Value = serde_json::from_slice(payload_bytes).unwrap();
        assert_eq!(payload["product"], "pointsav-design-system");
        assert_eq!(
            payload["entitled_products"],
            json!(["pointsav-design-system"])
        );

        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn claim_free_license_403_when_not_apache_tier() {
        let scratch = scratch_dir("claim-free-license-403");
        let state = test_state(&scratch, "tool-wallet".into());
        let mut state = (*state).clone();
        state.catalog_path = write_catalog_with_apache_tier(&scratch);
        let state = Arc::new(state);

        // os-console is agpl tier in this fixture, not apache -- must be refused
        // even though it's a perfectly real catalog product.
        let resp = claim_free_license(State(state), Path("os-console".to_string())).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let body: Value = serde_json::from_str(&body_text(resp.into_body()).await).unwrap();
        assert_eq!(body["code"], "not-free-tier");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn claim_free_license_404_when_product_unknown() {
        let scratch = scratch_dir("claim-free-license-404");
        let state = test_state(&scratch, "tool-wallet".into());
        let mut state = (*state).clone();
        state.catalog_path = write_catalog_with_apache_tier(&scratch);
        let state = Arc::new(state);

        let resp = claim_free_license(State(state), Path("no-such-product".to_string())).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: Value = serde_json::from_str(&body_text(resp.into_body()).await).unwrap();
        assert_eq!(body["code"], "product-not-found");
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn claim_free_license_503_when_signing_key_unconfigured() {
        let scratch = scratch_dir("claim-free-license-503");
        let base_state = test_state(&scratch, "tool-wallet".into());
        let mut state = (*base_state).clone();
        state.catalog_path = write_catalog_with_apache_tier(&scratch);
        state.signing_key = None;
        let state = Arc::new(state);

        let resp =
            claim_free_license(State(state), Path("pointsav-design-system".to_string())).await;
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body: Value = serde_json::from_str(&body_text(resp.into_body()).await).unwrap();
        assert_eq!(body["code"], "signing-not-configured");
        let _ = fs::remove_dir_all(&scratch);
    }

    // ── P1: /healthz + / redirect ─────────────────────────────────────────────

    #[tokio::test]
    async fn healthz_shape() {
        let Json(body) = healthz().await;
        assert_eq!(
            body,
            json!({"status": "ok", "service": "app-privategit-software"})
        );
    }

    #[tokio::test]
    async fn root_redirects_302_to_software() {
        let resp = root().await;
        // P1 contract: 302 Found (NOT axum Redirect::to's 303).
        assert_eq!(resp.status(), StatusCode::FOUND);
        assert_eq!(resp.headers().get(header::LOCATION).unwrap(), "/software");
    }

    // ── SEO: /robots.txt + /sitemap.xml (BRIEF-seo-cross-site-strategy.md) ────────

    #[tokio::test]
    async fn robots_txt_disallows_v1_api_and_points_at_sitemap() {
        let resp = robots_txt().await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/plain; charset=utf-8"
        );
        let body = body_text(resp.into_body()).await;
        assert!(body.contains("Disallow: /v1/"));
        assert!(body.contains("Sitemap: https://software.pointsav.com/sitemap.xml"));
    }

    #[tokio::test]
    async fn sitemap_xml_lists_html_pages_only_not_v1_or_source_routes() {
        let scratch = scratch_dir("sitemap-basic");
        let state = test_state_full(&scratch);
        let resp = sitemap_xml(State(state)).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/xml; charset=utf-8"
        );
        let body = body_text(resp.into_body()).await;
        assert!(body.contains("<loc>https://software.pointsav.com/software</loc>"));
        assert!(body.contains("<loc>https://software.pointsav.com/licensing</loc>"));
        assert!(body.contains("<loc>https://software.pointsav.com/page/contact</loc>"));
        assert!(!body.contains("/v1/"));
        assert!(!body.contains("/releases/"));
        assert!(!body.contains("/git/"));
        let _ = fs::remove_dir_all(&scratch);
    }

    // Tier 4 fix: sitemap previously omitted product-detail pages entirely and
    // included the bare `/` redirect.
    #[tokio::test]
    async fn sitemap_xml_includes_product_pages_and_drops_bare_redirect() {
        let scratch = scratch_dir("sitemap-products");
        let state = test_state_full(&scratch);
        let resp = sitemap_xml(State(state)).await;
        let body = body_text(resp.into_body()).await;
        assert!(body.contains("<loc>https://software.pointsav.com/software/os-mediakit</loc>"));
        assert!(body.contains("<loc>https://software.pointsav.com/software/os-console</loc>"));
        assert!(body.contains("<loc>https://software.pointsav.com/es/software/os-mediakit</loc>"));
        // `/` is only ever a 302 to `/software` — not worth listing both.
        assert!(!body.contains("<loc>https://software.pointsav.com/</loc>"));
        let _ = fs::remove_dir_all(&scratch);
    }

    #[tokio::test]
    async fn sitemap_xml_omits_product_pages_when_catalog_unavailable() {
        let scratch = scratch_dir("sitemap-no-catalog");
        let state = test_state_at(&scratch, scratch.join("no-such-products.yaml"));
        let resp = sitemap_xml(State(state)).await;
        // Degrades to the static page list rather than 500ing — sitemap staying up
        // matters more than product coverage for one bad catalog read.
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_text(resp.into_body()).await;
        assert!(body.contains("<loc>https://software.pointsav.com/software</loc>"));
        assert!(!body.contains("/software/os-"));
        let _ = fs::remove_dir_all(&scratch);
    }
}
