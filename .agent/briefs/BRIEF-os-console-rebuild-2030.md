---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-os-console-rebuild-2030
title: os-console 10× Rebuild — Demo Vertical Slice
status: active
owner: project-console
created: 2026-06-20
updated: 2026-06-24
---

## Context

os-console was a working prototype assessed as "~2012 TUI standards": keyboard-only,
single-cartridge-at-a-time, hard-coded 3-zone layout, no shared state, no mouse, no
persistence, search not a cartridge. Investors (WPP, Goldman Sachs) require a 10×
demonstration of the platform's security thesis.

## Scope

Four layers: (1) deployment keystone — host-native binary + MBA over internet;
(2) interaction core — intent registry + mouse; (3) cinematic shell — motion engine;
(4) visible moat — capability DAG + Merkle ledger. Demo vertical slice proves all
four end-to-end through four cartridges: F4 Proofreader, F5 Search, F11 Admin, F12
Input Machine.

## Decisions locked

- Build order: I-1 → I-2 → C-1 → M-1 → D-A (foundation-first → demo milestone)
- ssh_server.rs retired; host-native binary + service-ingress is the transport
- service-ingress: mTLS over 443, self-signed cert, path routing to localhost services
- SearchCartridge is F5 (first-class cartridge tab)
- Commitment: all four moat items shipped in one session (2026-06-23)

## Work log

### Phase I-1 — Intent registry (complete, committed)
- `console-core` crate: `IntentRegistry`, `IntentId`, `IntentSpec`, parity gate
- 10 integration tests green; parity test enforced at compile time

### Phase I-2 — Mouse handling (complete, committed)
- SGR 1006 capture wired in chassis; basic click/scroll dispatched via intent

### Phase C-1b — Cinematic (complete, committed)
- `motion.rs`: `Anim` + `Ease`; patience ring (F4 submit), verdict-pop

### Phase F12-L — Merkle ledger ribbon (complete, committed)
- F12 Input shows `ledger_root` advancing on every ingest; F12 anchor gate preserved

### Phase K — Ctrl-`?` capability overlay (complete, committed)
- `render_cap_overlay` overlay on any cartridge; `cap_verdicts()` trait method
- Live seL4 capability verdicts from `system-ledger`

### Phase F5 — Search cartridge (complete, committed)
- `app-console-search` crate: `SearchCartridge`; result list + redacted rows
- `SendToContent` cross-cartridge bus: S key on result → F4 with text pre-loaded

### Phase D-A — TOFU auto-pin (complete, committed, 56b30973)
- `mba_client.rs` rewritten: `MbaHandler` verifies server fingerprint; `pin_server_key()`
- Closes the `check_server_key → Ok(true)` MITM hole

### Phase M-1 — Four moat items (complete, committed, a5ad7c54)
- **Search → Proofreader send-to**: `CartridgeAction::SendToContent`; chassis intercepts;
  `accept_transfer()` in ContentCartridge
- **F4 egress-witness strip**: `"⬡ Witnessed by Doorman · Local · HH:MM"` footer;
  `witness_at: String` in `ContentState::Results`
- **F11 revocation cascade**: `cascade_queue: Vec<usize>` + 12-tick timer; amber tint
  + `⟲` glyph for pending; real `apply_revocation()` at each step
- **service-ingress crate**: rcgen self-signed cert, fingerprint to stderr, axum-server
  HTTPS on 8443, proxies all paths to localhost services

### Phase D-A mTLS routing (complete, committed, a6464faf)
- `app-console-keys/src/tls.rs`: `build_http_client(cert_pem, timeout)` shared helper
- `tls_endpoint` + `tls_cert_pem_path` in `ProfileConfig`
- When `tls_endpoint` is set: all HTTP calls in ContentCartridge + SearchCartridge route
  through HTTPS; custom cert loaded for pinning

## Decisions open

- **service-ingress GCE deploy** — binary not deployed; port 8443 firewall rule not created;
  os-console config not pointed at it. Command Session work (~30 min).
- **F11 identicon approval** — TOPOLOGY tab renders pending approvals; Kitty-rendered
  identicons before approval not implemented (Goldman demo moment)
- **I-3 recursive pane tree** — chassis still hardcodes 3-zone layout; tiling deferred
- **I-4 full data bus + Desk** — SendToContent is the precursor; full typed Payload/Envelope
  bus not started
- **C-1 full integration** — motion.rs exists but Patience-ring/Verdict-pop not wired into
  every cartridge render path
- **Kitty GRAPH** — capability DAG is a text list; Sugiyama-layout Kitty-rendered image
  version not started
- **D-B/D-C** — WireGuard PPN underlay + device-cert identity; explicitly future phases

## Carry-forward

- Stage 6 pending for commits a5ad7c54 + a6464faf → Command Session outbox
- service-ingress GCE deployment → Command Session (gcloud + systemd unit)
- README.md + README.es.md for service-ingress/ (required by repo-layout.md)
