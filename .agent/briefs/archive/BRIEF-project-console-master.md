---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-master
owner: project-console
archive: project-console
title: "project-console Master BRIEF"
status: archived
created: 2026-05-31
updated: 2026-05-31
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
supersedes:
  - BRIEF-leapfrog-2030-coding.md
  - BRIEF-pairing-ceremony.md
  - BRIEF-pairing-phase3-4.md
absorbs:
  - BRIEF-comprehensive-improvement-proposal.md (Gemini audit 2026-05-31)
companion: BRIEF-os-console-platform.md
---

# project-console Master BRIEF

> **Read this before any session.** Architecture detail lives in `BRIEF-os-console-platform.md`
> (full F-key map, Cartridge trait, MBA topology, platform targets). This brief is the
> living state tracker + forward roadmap. Both files together are the complete session context.
>
> **Note on briefs directory contamination:** ~18 BRIEFs from project-gis, project-knowledge,
> project-editorial, and project-intelligence also live in this directory. Ignore them here.
> project-console BRIEFs are: this file, `BRIEF-os-console-platform.md`, and the superseded
> files listed above (historical reference only).

---

## §1 — Live deployment state (as of 2026-05-31)

| Component | Binary | Port | Status |
|---|---|---|---|
| `service-proofreader` | `/usr/local/bin/service-proofreader` | 9092 | active |
| `app-console-proofreader` | `/usr/local/bin/app-console-proofreader` | — | active (Round 5; awaiting redeploy for Round 6 UX) |
| `local-doorman.service` | `slm-doorman-server` | 9080 | active |
| `pairing-server` | not yet deployed | 9201 | **pending — systemd unit missing** |
| LanguageTool 6.6 | Docker | 8010 | active |
| nginx + Let's Encrypt | `proofreader.pointsav.com` | 443 | active |

**Doorman endpoint:** `http://localhost:9080` (authoritative; confirmed from `local-doorman.service`
`SLM_BIND_ADDR`; code fixed in `009b2e04`).

---

## §2 — Completed phases

| Phase | Commit | What shipped |
|---|---|---|
| Phase 0 — Spike | first commit | russh + ratatui SSH TUI skeleton |
| Phase 1 — Chassis | `af462797`, `480dd105` | `app-console-keys` lib + `os-console` bin; Cartridge trait; F-key tab strip |
| Phase 2 — Auth + MBA | `0b8088c4` | `system-gateway-mba`; proofctl; MBA LINK ACTIVE status bar |
| Phase 3 — ContentCartridge | `a020a2cd` | Full proofread workflow; tui-textarea; nucleo fuzzy picker; similar diff; syntect |
| Phase 4 — InputCartridge (F12) | `ce6c6621` | F12 Anchor; file path modal; POST to `service-input`; audit log |
| Phase 5 — Draft mode | `5118ce77` | `/new` slash command; Doorman Tier B SSE; drafts-outbound write |
| Pairing Phase 1 | `d6267e39` | 8-char Crockford code; `pairing-server` tiny_http 9201; proofctl pair CLI; PairingState TUI screens |
| Pairing Phase 2 | `30874995` | Dense1x2 Unicode QR beside code pill; narrow fallback |
| Pairing Phase 3 | `11135186` | ratatui-image Kitty/Sixel pixel QR; Dense1x2 fallback |
| Pairing Phase 4 | `28000772` | F11 `app-console-system` cartridge; pending-pair list; Enter approve / D deny |
| Cross-platform Phase A | `009b2e04` | Doorman port 9080 fix everywhere; configurable endpoints; GitHub Actions Linux CI |
| Cross-platform Phase B | `6f21f580` | 4-target release matrix (Linux musl, macOS Intel, ARM, universal); rustls-tls; TerminalCaps probe |

---

## §3 — Stage 6 status

Staging mirrors (`origin-staging-j`, `origin-staging-p`) are at `009b2e04` (Phase A).
Commits needing push + promote:

| SHA | Subject |
|---|---|
| `6f21f580` | feat(release): Phase B — CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | ops(session): Phase B complete |
| `d58960b4` | ops(brief): mark Phase B complete |

Force-push to staging mirrors was **authorized by Command 2026-05-28** (inbox
`command-20260528-console-answers`). After push: write `promote-queue.jsonl` entry;
Command runs `bin/promote.sh`.

---

## §4 — Next coding phases (COMPLETE as of 2026-05-31)

### Phase C — Email cartridge (F3) ✓

- `app-console-email/` converted to lib crate with `EmailCartridge` implementing `Cartridge`
- Inbox list + message read + compose/send; `j/k` navigate, `Enter` open, `N` compose, `R` refresh
- Backend: `service-email` at `email_endpoint` (default `localhost:9093`)
- Registered in `os-console/src/main.rs` at F3; workspace member
- Plain mode: `plain: bool` passed from `ConsoleConfig.plain_mode`; ASCII borders, text labels

### Phase D — SLM cartridge (F9) ✓

- `app-console-slm/` converted to lib crate with `SlmCartridge`
- Doorman health dashboard: circuit state, tier routing, `ai_available`, entity count
- Background poller (10s interval) + `R` manual refresh; `?` help overlay
- Backend: `slm_endpoint` (default `localhost:9080`) via `/readyz`
- Registered in `os-console/src/main.rs` at F9; workspace member

### Phase E — Orchestration wiring ✓

- `mba_client.rs` audited: clean, connects via `totebox_host:totebox_ssh_port`; no stale refs
- Zero `app-orchestration-command` references in any Rust source
- `orchestration_host: String` added to `ConsoleConfig` (default `127.0.0.1`)
- `email_endpoint` + `plain_mode` also added to `ConsoleConfig` (Phase C prerequisite)
- `BRIEF-os-console-platform.md` §5 updated with full peer-field table

## §4b — Next coding phases

### Phase 6 — Offline mode + Tantivy search

- Offline detection: poll `slm_endpoint/v1/health/ready`; switch to deterministic-only mode
- Greyed inference widgets in `ContentCartridge` when offline
- `/search <query>` → `service-content` port 9081 Tantivy index

---

## §5 — Deferred phases

| Phase | What |
|---|---|
| ~~Phase 6~~ | **DONE 2026-05-31** — Offline mode (Doorman `/readyz` poll) + `/search` Tantivy command |
| ~~Phase 7~~ | **DONE 2026-05-31** — PDF viewing via pdfium-render → Kitty/Sixel pixel render; text fallback on unsupported terminals; `/pdf <path>` command |
| Phase 8 | Polish — OSC 8 hyperlinks; truecolor; multi-tab; session persistence; `/audit` log viewer; F2 People |
| Phase 9 | Operations — `local-console.service` systemd unit; Prometheus metrics; fail2ban for port 2222; graceful SIGTERM |
| Phase 10–13 | F7 BIM, F10 mesh, chassis auto-reconnect watchdog |

**Phase 7 note:** `Cartridge` trait gained `set_graphics_caps(kitty, sixel, font_size)`, called once
by the chassis after the terminal probe. The `image` dependency in `app-console-content` is pinned
to `default-features = false` (matches `app-console-keys`) to avoid pulling AVIF/rav1e codecs.

---

## §6 — UX contract

Rules applied uniformly across ALL cartridges. New in this BRIEF; extracted from Gemini audit.

**1. `--plain` mode**
Every cartridge must function fully in `--plain` mode: no terminal graphics, no 24-bit color,
no ratatui-image calls. Enables screen readers and degraded SSH sessions. `TerminalCaps` probe
already exists (`6f21f580`); cartridges read `caps.plain` to branch render paths. Greyscale
ASCII borders are acceptable; blank is not.

**2. Keyboard ergonomics contract**
Uniform across all cartridges — document in `Cartridge` trait:
- `Esc` = cancel / close modal / back to previous state
- `Enter` = confirm / submit / select
- `Tab` / `Shift-Tab` = focus next / previous within a cartridge
- `F1`–`F12` = cartridge switch (always routed by chassis; cartridge does not intercept)
- Cartridge-specific bindings go in a local help overlay (accessible via `?`)

**3. Semantic color states**
Apply consistently; do not rely on color alone (also use border style or prefix symbol):
- Error / failure: red (`Color::Red`) + `✗` prefix or `[ERROR]` label
- Success / done: green (`Color::Green`) + `✓` prefix
- Warning / pending: yellow (`Color::Yellow`) + `⚠` or `[WAIT]` label
- Inactive / greyed: `Color::DarkGray`
- Active / streaming: `Color::Yellow` border (existing convention — keep)

---

## §7 — Architecture quick-reference

Full detail: `BRIEF-os-console-platform.md`. Summary for session-start orientation:

**Binary:** `os-console` (single process; crossterm PTY local; `--features ssh-server` for GCE)

**Base chassis:** `app-console-keys` (always installed; defines `Cartridge` trait; owns F-key tab
strip, status bar, MBA client, profile config)

**F-key map (canonical):**

| F | Cartridge | State | Notes |
|---|---|---|---|
| F1 | `app-console-help` | Reserved | Help overlay |
| F2 | `app-console-people` | Scaffold-coded | Identity, contacts |
| F3 | `app-console-email` | Scaffold-coded → **Phase C** | Communications |
| F4 | `app-console-content` | Active | Proofread + draft |
| F5 | `app-console-minutebook` | Reserved | Governance |
| F6 | `app-console-bookkeeper` | Active | Financial ledger |
| F7 | `app-console-bim` | Reserved | BIM |
| F8 | `app-console-gis` | Reserved | GIS |
| F9 | `app-console-slm` | Reserved → **Phase D** | SLM management |
| F10 | `app-console-mesh` | Reserved | PPN mesh |
| F11 | `app-console-system` | Active | System status, pairing panel |
| F12 | `app-console-input` | Active | **The Anchor** — SYS-ADR-10 |

**Doorman endpoint:** `http://localhost:9080`  
**Doorman response field:** `.content` (not `.choices[0].message.content`)  
**Long-poll timeout:** 300s on `/v1/proofread`; 30s elsewhere  

---

## §8 — Operator-gated items (unchanged; awaiting action)

- [ ] GCE firewall port 2222 — required for external MBA connections
- [ ] `pairing-server` systemd unit — deploy alongside SSH on VM; listens `0.0.0.0:9201`
- [ ] Peter SSH key — generate Ed25519; `proofctl user add peter --tenant woodfine --role editor`
- [ ] Tag `v0.1.0` on pointsav-monorepo — triggers GitHub Actions release build
- [ ] Branch rename `cluster/project-proofreader → cluster/project-console` on GitHub

---

## §9 — Absorbed BRIEFs (historical record)

| File | Absorbed | What was in it |
|---|---|---|
| `BRIEF-leapfrog-2030-coding.md` | 2026-05-31 | Phase 0–9 full spec; key crate deps; gate criteria per phase |
| `BRIEF-pairing-ceremony.md` | 2026-05-31 | Pairing Phases 1–2 implementation detail; zero-jargon vocabulary |
| `BRIEF-pairing-phase3-4.md` | 2026-05-31 | ratatui-image Kitty/Sixel Phase 3; F11 operator panel Phase 4 |
| `BRIEF-comprehensive-improvement-proposal.md` | 2026-05-31 | Gemini audit; extracted: --plain mode, keyboard contract, semantic colors |

Files remain on disk for git history. `superseded_by` / `absorbed_by` frontmatter added.
For deep phase detail (crate deps, gate criteria, implementation snippets) read the original
superseded files directly.
