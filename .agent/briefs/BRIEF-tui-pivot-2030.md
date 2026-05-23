---
schema: foundry-plan-v1
archive: project-console
title: "Leapfrog 2030 — TUI Pivot Strategic Plan"
created: 2026-05-16
updated: 2026-05-23
status: archived
authors: [totebox@project-console, opus-research-audit-2026-05-16]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
---

> **ARCHIVED 2026-05-23.** Superseded by `BRIEF-os-console-platform.md` (architecture)
> and `BRIEF-leapfrog-2030-coding.md` (coding roadmap). **Part 6 (Teardown Plan — sudo
> commands for web UI services, nginx vhost, cert) remains the Command Session reference.**
> All other sections are superseded. Note: Doorman port in Part 1 audit table shows 9080
> (stale) — correct port is 8011 (see BRIEF-os-console-platform.md §10). Part 7 phase
> numbers are the old sequence — do not use them.

# Leapfrog 2030 — Project Proofreader TUI Pivot
## Strategic Plan & Long-Term Development Roadmap

> **One-paragraph statement of intent**
>
> We are replacing the web-based `app-console-proofreader` with a single Rust binary
> that speaks SSH and renders a keyboard-native TUI. Users type `ssh proof@host` and
> receive a full-featured write-assistant — proofread existing text, draft new content,
> accept/reject/edit suggestions, and stream from Tier B — in under 200 ms. The
> service-proofreader HTTP backend is unchanged. The web UI is taken offline. The SSH
> TUI becomes the `app-console-content` cartridge in the `os-console` chassis and the
> primary corpus-producer for the apprenticeship loop. This is Doctrine claim #45
> fulfilled, not a fallback.

---

## Part 1 — Audit Results: Current State

*Four Opus research agents executed 2026-05-16. Findings synthesized below.*

### 1.1 What is live right now

| Component | Binary | Port | Auth | Public |
|---|---|---|---|---|
| `local-proofreader.service` | `service-proofreader` | 127.0.0.1:9092 | none (loopback) | No |
| `local-proofreader-console.service` | `app-console-proofreader` | 127.0.0.1:9091 | session cookie | No (no nginx) |
| `local-proofreader-public.service` | `app-console-proofreader` | 127.0.0.1:9097 | none | **Yes** — via nginx |
| **nginx vhost** | `proofreader.pointsav.com` | :443 | — | HTTPS live |
| LanguageTool 6.6 | Docker container | 127.0.0.1:8010 | none | No |
| `local-doorman.service` | `slm-doorman-server` | 127.0.0.1:9080 | none (loopback) | No |
| `local-slm.service` | llama.cpp / OLMo | 127.0.0.1:8080 | none (loopback) | No |
| `local-content.service` | `service-content` | 127.0.0.1:9081 | none (loopback) | No |
| `local-fs.service` | WORM ledger | local | — | No |

**Domain status:**
- `proofreader.pointsav.com` — HTTPS live, nginx vhost at port 9097, Let's Encrypt cert issued
- `proofreader.woodfinegroup.com` — **no nginx vhost, no cert, never deployed**; this domain was planned but never wired

**SSH baseline:**
- `ssh.service` active on `0.0.0.0:22`, internet-exposed, under active scan
- Login-capable users: `mathew`, `jennifer`, `ubuntu`
- Foundry SSH config drop-in: `/etc/ssh/sshd_config.d/99-foundry.conf`
- Identity/signing model already established (`~/Foundry/identity/allowed_signers`)

### 1.2 CRITICAL: Source tree is empty

> **This is the most important finding from the audit.**

The `pointsav-monorepo/` sub-clone is on `cluster/project-proofreader` branch at `e24b778`.
The working tree contains **only `.agent/` metadata files** — no Rust source code for
`service-proofreader/` or `app-console-proofreader/` is present.

**Root cause:** The canonical branch at `origin/cluster/project-proofreader@e24b778` was reset
during the same filter-repo security sweeps that cleaned the WFD sub-clone (2026-05-15).
The pre-reset source-bearing commits are preserved in the local reflog:

```
788b3722  — "Decommission woodfine tenant" (last source-bearing SHA)
10f062a   — Phase D-lite chat-style UI
112166c   — Phase A dark mode
93137c7   — Phase B prompt fix (59 tests pass)
```

**What this means for the pivot:**
- The web UI source (`app-console-proofreader/`) is lost from the working tree but fully
  reconstructed from mailbox/outbox history — sufficient to inform the TUI design
- The service-proofreader source (`service-proofreader/`) is similarly absent but its API
  contract, pipeline architecture, and test suite are fully documented
- **The pivot is a clean-slate rewrite of the client layer** — this is net positive;
  there is nothing to port or migrate, only to build forward
- The running binaries at `/usr/local/bin/service-proofreader` and
  `/usr/local/bin/app-console-proofreader` are the compiled artifacts from `788b3722`

**Immediate action required (pre-development):**
Confirm with Command Session whether the canonical source was moved to a different branch
or must be restored from the local reflog. If the intent is clean-slate TUI, the source
tree can remain empty and development begins fresh on a new branch cut from `origin/main`.

### 1.3 What the web UI was (reconstruction)

The web UI delivered a 3-screen flow:

1. **Login** — POST to `/login`, session cookie, bcrypt password check
2. **Identity** — three buttons (J / P / M) to set the corpus contributor identity
3. **Paste** — textarea + 18-item protocol dropdown + SLM readiness banner
4. **Spinner** — "1–3 minutes is normal" CSS overlay during pipeline execution
5. **Diff** — side-by-side original / improved; per-flag severity badges (`Banned` / `Mechanical` / `Generative`); Apply-all; per-flag accept/reject; "explain why"
6. **Verdict** — POST to `/v1/verdict` closes the apprenticeship loop

Phase A (dark mode toggle) and Phase D-lite (chat-style layout) were the final shipped state.
The UI was ~1,500–2,500 lines of Rust/HTML across `auth.rs`, `handlers.rs`, `ui.rs`,
`upstream.rs`, `types.rs`, `main.rs`.

**None of this survives the pivot. All of it is discarded.**

### 1.4 What the service-proofreader pipeline does (survives intact)

Three-stage deterministic + AI pipeline, all server-side, UI-agnostic:

```
POST /v1/proofread {text, protocol, tenant, user?}
  │
  ├── Stage 1: banned-vocabulary scan (deterministic, <1ms)
  │   └── Flags: Severity::Banned, span + replacement-hint
  │
  ├── Stage 2: LanguageTool 6.6 (HTTP to 127.0.0.1:8010)
  │   └── Flags: Severity::Mechanical, LT rule IDs normalised to Finding type
  │
  └── Stage 3: Doorman generative pass (HTTP to 127.0.0.1:9080)
      ├── compact-plaintext findings embedded in system message (Phase B fix)
      ├── anti-CoT directive: "Do NOT reason aloud. Output ONLY the improved text."
      ├── reasoning-prefix stripping for OLMo output
      └── Flags: Severity::Generative (whole-text rewrite at present)

Response: {improved_text, diff[], findings[{severity, span, message, replacement?}],
           metadata: {tier_used, model, inference_ms, banned_vocab_count,
                      lt_flag_count, degraded:[...]}, generative_pass: bool}

POST /v1/verdict {draft_id, disposition: accepted|rejected|edited, final_text?}
  └── Writes creative-edited event pair to corpus (PP.1 / Doctrine claim #35 §7A)

GET  /v1/templates — 18 GenreTemplate variants (PROSE/COMMS/LEGAL/TRANSLATE families)
GET  /v1/health/ready — {ready, has_doorman, has_lt} for TUI status-bar
GET  /health — liveness probe
```

**This API surface is the stable substrate the TUI consumes. It changes minimally.**

### 1.5 service-slm (Doorman) API — what the TUI needs to know

| Endpoint | Purpose | TUI use |
|---|---|---|
| `POST /v1/chat/completions` | Inference (OpenAI wire) | Draft mode: direct LLM calls |
| `GET /healthz` | Liveness | Status-bar |
| `GET /readyz` | `{ready, has_local, has_yoyo, has_external}` | Tier-availability display |
| `GET /v1/contract` | Doorman version | Startup validation |
| `POST /v1/brief` | Apprenticeship — senior attempt | Automatic during proofread |
| `POST /v1/verdict` | Apprenticeship — verdict submission | On user accept/reject/edit |

Routing headers: `X-Foundry-Module-ID: proofreader`, `X-Foundry-Tier-C-Label: <purpose>`
(Tier C only, allowlist-gated).

**Doorman wire quirk:** response carries `.content`, not `.choices[0].message.content`.
The TUI's draft-mode SSE consumer must parse the Doorman shape, not the raw OpenAI shape.

### 1.6 service-content — what it provides for draft mode

- **`local-content.service`** on `127.0.0.1:9081`
- DataGraph (LadybugDB persistence) — entity-relationship store
- Tantivy inverted index at `/search-index/` — offline-capable keyword search
- Ontology CSVs — domain glossaries (topics, domains, guides)
- Draft mode calls: `GET /graph/entity/<id>`, `GET /graph/neighborhood/<id>` for RAG
  context before invoking Doorman

### 1.7 os-console / Console-OS doctrine

> **F4 is ours.**

`os-console` is the chassis OS for unified operator terminals. It hot-swaps Cartridges via
F-keys. The relevant F-keys for this project:

| F-Key | Cartridge | Territory |
|---|---|---|
| F4 | Content | **Project Proofreader scope — proofread + draft** |
| F12 | Input Machine | **Mandatory per SYS-ADR-10 — human-authorized ingest** |
| F2 | People | Identity Ledger (separate project) |
| F3 | Email | Sovereign Comm Diode (separate project) |

The TUI is the `app-console-content` Cartridge. It must implement both F4 (the
proofread/draft workspace) and F12 (file/text injection gate) per SYS-ADR-10.

Machine-Based Authorization (MBA) is the doctrine-established auth model: access is granted
via cryptographic pairing between operator hardware and the archive, not via username/password.
SSH public key = hardware pairing. This is doctrine-native.

---

## Part 2 — The New Direction

### 2.1 Strategic statement

The web stack is not the right substrate for a write-assistant that:
- needs a 300s long-poll without stalling the UI
- receives 50KB paste events atomically
- requires per-suggestion keyboard-native accept/reject
- streams LLM output token-by-token into an editing buffer
- produces signed apprenticeship verdicts
- must work AI-offline (Doctrine claim #54)
- must implement F12 ingest

These are terminal-native problems. The browser solves all of them expensively and
incompletely. The terminal solves all of them cheaply and correctly.

**The SSH TUI is not a downgrade. It is a better tool.**

Evidence from the field: lazygit is more productive than any web git UI. k9s is more
productive than the Kubernetes dashboard. helix is a viable daily driver. These tools
outperform their web counterparts not despite being terminal-native but because of it.

The strategic bet: SSH, ANSI escape sequences, and public-key cryptography age slower than
JavaScript frameworks, OAuth flows, and browser bundlers. Building on the slower-aging
substrate means we make this choice once.

### 2.2 What changes

| Before | After |
|---|---|
| Web browser required | Any SSH client (built into every OS) |
| Cookie session auth | SSH public-key auth (Ed25519 / FIDO2) |
| bcrypt password in env | Application-managed key database (SQLite) |
| 9091 / 9097 HTTP | 2222 SSH |
| nginx vhost | No web ingress |
| Let's Encrypt cert | No TLS needed (SSH encrypts natively) |
| `app-console-proofreader` binary | `app-console-content` binary (new) |
| DOM + CSS diff rendering | ratatui two-pane + syntect colorization |
| Form POST verdict | Keyboard verdict → POST /v1/verdict + SSH-signed |
| No draft mode | Draft mode: protocol selector + streaming generation |
| Web-only corpus events | SSH-signed corpus events (stronger provenance) |

### 2.3 What does NOT change

| Component | Status |
|---|---|
| `service-proofreader` (9092) | Unchanged — all API endpoints reused |
| LanguageTool (8010) | Unchanged |
| Doorman (9080) | Unchanged |
| local-slm (8080) | Unchanged |
| service-content (9081) | Unchanged — added as RAG source for draft mode |
| local-fs WORM ledger | Unchanged — corpus capture path stays |
| PP.1 corpus schema | Unchanged — verdict POST is the same JSON |
| 18 GenreTemplate variants | Unchanged — exposed as TUI protocol picker |
| Long-poll timeout discipline | Unchanged — 300s on /proofread, 30s everywhere else |
| Severity discriminant (B/M/G) | Unchanged — mapped to terminal colour codes |

---

## Part 3 — Technology Stack

### 3.1 Core TUI stack

| Layer | Crate | Rationale |
|---|---|---|
| Rendering | **`ratatui`** ≥ 0.29 | De facto standard; immediate-mode; tui-rs successor; ~20 MB RAM |
| Terminal backend | **`crossterm`** | Cross-platform; `Event::Paste` for bracketed-paste (non-negotiable for large pastes); active maintenance |
| SSH server | **`russh`** | Async, tokio-native; production-used by kartoffels/sandhole; `ratatui_app.rs` example in tree is exactly the shape we need |
| Multi-line input | **`tui-textarea`** | Multi-line editor, yank buffer, crossterm key passthrough; the only credible option |
| Diff engine | **`similar`** | Myers + Patience; word/line/char granularity; used by `insta`; battle-tested |
| Highlighting | **`syntect`** (24-bit ANSI) | Banned-vocab inline underlines + LanguageTool span overlays + diff colorization |
| Async runtime | **`tokio`** + `tokio::select!` | Tick stream + SSE stream + crossterm events; canonical async-ratatui pattern |
| Scroll | **`tui-scrollview`** | Offset-based; avoids render stalls on large documents |

**Rejected:**
- `cursive` — archived March 2026 by maintainer, explicitly deprecated
- `tui-rs` — dead (ratatui is the fork)
- `termion` — Unix-only, slower maintenance cadence
- Go/charm stack — wrong language; `charmed_rust` is a hobby port, not a foundation

### 3.2 SSH delivery model

**Recommendation: russh embedded in the TUI binary, listening on port 2222.**

The binary IS the SSH server. No `sshd` involvement. No Unix user provisioning.
No `authorized_keys` management. No `ForceCommand` bash wrapper.

```
GCE VM — current                    GCE VM — after pivot
─────────────────────               ─────────────────────
nginx (443) ──→ 9097               (nginx vhost removed)
nginx (443) ──→ 9091               (nginx vhost: never existed)
service-proofreader (9092)  ──────→ service-proofreader (9092) [unchanged]
local-doorman (9080)        ──────→ local-doorman (9080)        [unchanged]
local-slm (8080)            ──────→ local-slm (8080)            [unchanged]
LanguageTool (8010)         ──────→ LanguageTool (8010)         [unchanged]
service-content (9081)      ──────→ service-content (9081)      [unchanged]
                                    app-console-content (2222)  [NEW — russh]
```

**Why not `sshd` + `ForceCommand`:**
- Every user needs a local Unix account or you lose per-user audit trail
- `authorized_keys` mutation becomes part of the deployment pipeline
- Tenant separation (pointsav / woodfine) requires a bash wrapper or multiple Unix users
- Cannot hot-reload the user database without restarting sshd
- Inherits the full `sshd` attack surface and update cadence

**Why russh-in-binary:**
- One process owns auth, session lifecycle, and rendering — debuggable from a single log stream
- Public keys live in SQLite; rotation is an SQL UPDATE, no filesystem mutation
- Tenant binding is a struct field on the session
- Per-session state (open document, draft buffer, undo stack) lives in tokio tasks scoped to the channel
- The russh `auth_publickey` handler is the natural authorization choke point
- F12 ingest gate is implementable as a trait on the session type
- No `sshd` upgrades break you

### 3.3 Authentication model

**SSH public keys, application-managed.**

```
Bootstrap:    proofctl user add jennifer --tenant woodfine --key-file jennifer.pub
              → fingerprint + username + tenant + role → SQLite `users` table

Connect:      ssh -p 2222 proof@host.example.com
              → russh auth_publickey handler queries table
              → Auth::Accept + tenant binding, or Auth::Reject

Rotation:     proofctl user rotate-key jennifer --key-file new.pub
              → SQL UPDATE, no sshd reload, no filesystem mutation
```

**"Easy to log in" means: generate a key once, register it once, then `ssh proof@host` works.**

- **No passwords.** SSH keys are the floor.
- **No PAM.** Not writing a PAM stack for one application.
- **No TOTP** (for now). If a second factor is needed, prefer FIDO2-backed SSH keys
  (`sk-ssh-ed25519`) — the factor is in the key hardware, not in a code prompt.
- **Later enhancement:** OIDC bridge via short-lived SSH certificates (step-ca pattern).
  User authenticates to Google/GitHub once daily, gets an 8-hour SSH cert. Russh validates
  certs via the same `auth_publickey` path. Right architecture for multi-tenant fleet
  scale; overkill until headcount crosses ~20 users.

**The connection command users will run:**

```bash
# One-time setup
ssh-keygen -t ed25519 -C "jennifer@woodfine" -f ~/.ssh/id_proofreader

# Register (operator action)
proofctl user add jennifer --tenant woodfine --key-file ~/.ssh/id_proofreader.pub

# Every session
ssh -p 2222 proof@host.example.com
```

**Or with an `~/.ssh/config` entry:**
```
Host proof
    HostName host.example.com
    Port 2222
    IdentityFile ~/.ssh/id_proofreader
```
```bash
ssh proof   # done
```

---

## Part 4 — New Architecture

### 4.1 `app-console-content` binary structure

```
app-console-content/
├── Cargo.toml
├── src/
│   ├── main.rs              — russh server startup; bind 2222; dispatch sessions
│   ├── auth.rs              — russh auth_publickey handler; SQLite user lookup
│   ├── session.rs           — per-connection state: user, tenant, mode, buffers
│   ├── app.rs               — top-level ratatui app loop (tokio::select! over events)
│   ├── ui/
│   │   ├── layout.rs        — Layout::horizontal / Layout::vertical composition
│   │   ├── status_bar.rs    — tier status + user + tenant + connection state
│   │   ├── paste_pane.rs    — tui-textarea for input; bracketed-paste handler
│   │   ├── diff_pane.rs     — two-column diff; similar + syntect; suggestion list
│   │   ├── draft_pane.rs    — protocol picker (fuzzy) + streaming generation buffer
│   │   ├── explain_pane.rs  — collapsible bottom pane for "explain why"
│   │   ├── help_overlay.rs  — F1 / ? keybinding reference
│   │   └── f12_gate.rs      — Input Machine (SYS-ADR-10 mandatory)
│   ├── pipeline.rs          — HTTP client to service-proofreader (9092); long-poll 300s
│   ├── doorman.rs           — HTTP client to Doorman (9080); SSE consumer; .content parser
│   ├── content.rs           — HTTP client to service-content (9081); RAG context fetch
│   ├── verdict.rs           — POST /v1/verdict; SSH-signed payload builder
│   ├── corpus.rs            — local apprenticeship event staging (WAL, SQLite)
│   ├── proofctl.rs          — admin CLI subcommand (user add/list/rotate-key)
│   └── offline.rs           — deterministic-only mode (Doctrine claim #54)
├── migrations/
│   └── 001_users.sql        — SQLite schema: users, sessions, verdict_log
├── ARCHITECTURE.md
├── README.md
└── README.es.md
```

### 4.2 UX flow — proofread mode

```
$ ssh proof

┌─ PROOFREADER — jennifer@woodfine ─────────────── Tier A (OLMo 3 7B) • Ready ─┐
│ F1:Help  F4:Content  F12:Ingest  ?:Keys  /tier  /adapters  /status            │
├────────────────────────────────────────────────────────────────────────────────┤
│ Protocol: [ PROSE-README ▼ ]   F12 to ingest file                             │
│                                                                                │
│ ┌─ Input (paste or type) ────────────────────────────────────────────────────┐ │
│ │ _                                                                          │ │
│ └────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                │
│ <Tab> to switch panes  <Enter> to run  /proof to run in command palette        │
└────────────────────────────────────────────────────────────────────────────────┘

→ User pastes large text (bracketed paste → single Event::Paste)
→ User selects protocol from fuzzy picker (18 GenreTemplate variants)
→ <Enter> to run

┌─ Running pipeline ─────────────────────────────────────────────────────────────┐
│ Stage 1: banned vocabulary  ✓ (3 flags found)                                  │
│ Stage 2: LanguageTool 6.6  ✓ (7 mechanical flags)                             │
│ Stage 3: OLMo 3 7B ⠿ generating… 47s (long operations are normal)             │
└────────────────────────────────────────────────────────────────────────────────┘

→ Pipeline completes; diff view renders

┌─ Original ──────────────────┬─ Improved ─────────────────────────────────────┐
│ The company has been        │ The company has been operating since 2019. Its  │
│ operating since 2019. It's  │ operations span three provinces.                │
│ operations span 3 provinces │ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~                  │
│ ▲[B] "It's" → "Its"        │                                                 │
│ ▲[M] "3" → "three"         │                                                 │
│                             │                                                 │
├─────────────────────────────┴─────────────────────────────────────────────────┤
│ Suggestion 1/10  [B] Banned vocab: "It's" → "Its"        a:accept  r:reject   │
│ Suggestion 2/10  [M] Grammar: "3" → "three"              e:edit    ?:explain  │
│                                                          A:all     R:reject-all│
└────────────────────────────────────────────────────────────────────────────────┘

→ j/k navigate suggestions
→ a/r per-suggestion verdict
→ A/R accept-all / reject-all
→ e to edit inline (tui-textarea modal)
→ ? to open explain pane (30% height at bottom)
→ <Enter> when done → POST /v1/verdict → corpus event written
```

### 4.3 UX flow — draft mode

```
/new                     — opens fuzzy protocol picker
/draft topic-readme      — shorthand

Protocol picker:
  > PROSE-README
    PROSE-TOPIC
    PROSE-GUIDE
    PROSE-MEMO
    COMMS-EMAIL
    COMMS-MEETING-NOTES
    TRANSLATE-EN-ES
    … (all 18 variants)

→ User selects PROSE-TOPIC
→ "Entity context?" prompt → fuzzy search over service-content DataGraph
→ Entity card fetched (RAG) → Doorman Tier B request issued

┌─ Drafting: PROSE-TOPIC — jennifer@woodfine ────────── Tier B (OLMo 32B) ⠿ ──┐
│ Entity: Foundry Sovereign Data Platform                                         │
│ Domain: engineering-pointsav                                                    │
│                                                                                 │
│ ┌─ Draft ───────────────────────────────────────────────────────────────────┐  │
│ │ The Foundry Sovereign Data Platform is a structured framework for         │  │
│ │ managing enterprise information assets in a compliance-ready manner.       │  │
│ │ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ streaming… ▒▒▒▒▒▒▒▒▒▒▒▒                              │  │
│ └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│ Elapsed: 34s  Model: OLMo-32B-Think  Cost: ~$0.003 (Yo-Yo)                    │
│ <Esc>:cancel  /regenerate:try again  /tier b|c:switch tier                     │
└─────────────────────────────────────────────────────────────────────────────────┘

→ Tokens stream into buffer at 60Hz render cadence
→ Generation complete → enters proofread mode automatically
→ Accept → stages to .agent/drafts-outbound/ with foundry-draft-v1 frontmatter
```

### 4.4 Keybinding vocabulary

Following `slm-cli` conventions (Doctrine claim #45 — consistent fleet-wide TUI vocabulary):

| Key / Command | Action |
|---|---|
| `j` / `k` | Navigate suggestion list |
| `a` | Accept current suggestion |
| `r` | Reject current suggestion |
| `e` | Edit current suggestion inline |
| `A` | Accept all suggestions |
| `R` | Reject all suggestions |
| `?` | Explain why (collapsible pane) |
| `<Tab>` | Switch panes |
| `<Enter>` | Confirm / submit |
| `<Esc>` | Cancel / close overlay |
| `F1` | Help overlay |
| `F4` | Content mode (this cartridge) |
| `F12` | Input Machine gate (SYS-ADR-10) |
| `/proof` | Run proofreader on current buffer |
| `/new` | Start new draft (opens protocol picker) |
| `/tier a|b|c|auto` | Switch inference tier |
| `/feedback good|bad|refine <text>` | Submit apprenticeship verdict |
| `/adapters` | List / install adapter |
| `/status` | Show all service health |
| `/audit` | Recent verdict log |
| `/export` | Save buffer to file |
| `/help` | Show command palette |
| `/quit` | Exit session |

### 4.5 Offline mode (Doctrine claim #54)

When `local-slm.service` is inactive or unreachable:

```
Status bar: [ ⚠ AI-disabled — deterministic operations only ]
```

What continues to work without inference:
- File open / save (`:load <path>`, `/export`)
- Stage 1 banned-vocabulary scan (deterministic, sub-ms)
- Stage 2 LanguageTool grammar check (if LanguageTool Docker is up)
- Diff display of any two buffers (manual edit baseline)
- Tantivy keyword search (`/search <query>`)
- Audit ledger view (`/audit`)
- Verdict log navigation
- History navigation (prior sessions via SQLite)
- proofctl admin commands

What requires inference (disabled in offline mode):
- Stage 3 generative rewrite
- Draft mode (all generation)
- "Explain why" (LLM rationale)
- Tier B / Tier C escalation
- `/feedback refine` training capture (queued locally, submitted when back online)

### 4.6 ADR compliance

| ADR | Rule | Implementation |
|---|---|---|
| SYS-ADR-07 | No structured data through AI | TUI pipeline routes prose only; CSV/JSON/YAML bypasses the LLM path entirely; `/export` writes raw buffer |
| SYS-ADR-10 | F12 mandatory | `F12` keybinding calls `f12_gate.rs`; all file/text ingest routes through it; cannot be bypassed from other panes |
| SYS-ADR-19 | No automated AI publishing to verified ledgers | Draft accept stages to `drafts-outbound/` only; no TUI path writes directly to `content-wiki-*` or `factory-release-engineering`; project-editorial is the gateway |

---

## Part 5 — Leapfrog 2030 Capabilities

These are the five capabilities that make this genuinely better than the web UI.

### 5.1 Instant cold-start (≤200ms)

`ssh proof` returns a working editor in under 200ms. No bundler, no hydration, no JS parse
cost, no TTFB. This is a felt advantage every session.

### 5.2 Bracketed-paste-aware large-text ingestion

Crossterm's `Event::Paste` collapses a 50KB paste into a single event. The TUI inserts the
buffer atomically, kicks off the pipeline in the background, and streams Stage 1 results
into the suggestion panel while the user is still reading. No web textarea matches this for
documents above ~100KB.

### 5.3 Per-suggestion keyboard-native accept/reject

Each `similar::Change` becomes a `Suggestion { id, range, original, proposed, source, severity }`.
Navigate with `j`/`k`, verdict with `a`/`r`/`e`. Verdicts stream straight into the corpus.
This UX requires a CodeMirror integration in the browser; in ratatui it is ~400 lines.

### 5.4 Token-by-token streaming with responsive input

`tokio::select!` over (render tick @ 60Hz, SSE stream from Doorman, keyboard events).
Tokens land in a String buffer; the render loop redraws the panel each tick. Typing
`<Esc>` cancels mid-stream without blocking. The browser can do SSE streaming too, but
the TUI does it with 2MB RSS instead of 200MB.

### 5.5 Truecolor + OSC 8 hyperlinks + optional Kitty graphics

- **OSC 8 hyperlinks:** Every TOPIC reference, protocol document, citation — `\x1b]8;;url\x1b\\text\x1b]8;;\x1b\\`. Cmd-clickable in modern terminals. Zero implementation cost after the first helper.
- **24-bit truecolor:** Detect via `COLORTERM=truecolor` env (passed through SSH `SendEnv`). Use for syntect output, severity coding, accepted/rejected state.
- **Kitty graphics protocol:** Inline error-density heatmaps, diff thumbnails. Detect via `XTGETTCAP`; degrade to ASCII when absent. Sixel as secondary fallback.
- **Degrade gracefully:** Must work in plain `xterm`. Richness is additive, not required.

---

## Part 6 — Teardown Plan

### 6.1 What gets taken down

**Before tear-down, backfill `local-proofreader-public.service` source-of-truth.** The
deployment agent found no canonical copy at `/srv/foundry/infrastructure/local-proofreader/`.
Retrieve the live unit from `/etc/systemd/system/local-proofreader-public.service` and
commit it to `infrastructure/local-proofreader/` before removing it.

```bash
# Step 1: Stop and disable web UI services (Command Session — sudo required)
sudo systemctl stop local-proofreader-console.service local-proofreader-public.service
sudo systemctl disable local-proofreader-console.service local-proofreader-public.service

# Step 2: Remove unit files
sudo rm /etc/systemd/system/local-proofreader-console.service
sudo rm /etc/systemd/system/local-proofreader-public.service
sudo systemctl daemon-reload

# Step 3: Remove nginx vhost and rate-limit zone
sudo rm /etc/nginx/sites-enabled/proofreader.pointsav.com
sudo rm /etc/nginx/sites-available/proofreader.pointsav.com
sudo rm /etc/nginx/conf.d/proofreader-ratelimit.conf
sudo nginx -t && sudo systemctl reload nginx

# Step 4: Retire the TLS cert
sudo certbot delete --cert-name proofreader.pointsav.com

# Step 5: Remove the web UI binary
sudo rm /usr/local/bin/app-console-proofreader

# Step 6: DNS — leave proofreader.pointsav.com A record for a 410 redirect,
# or redirect it to documentation. Do not just let it go dark.
```

**proofreader.woodfinegroup.com** — no nginx vhost, no cert, no service. No teardown needed.
DNS record (if one exists) can be removed.

### 6.2 What stays running

Everything in the 9092 backend stack is unchanged and continues to serve the TUI.

| Keep | Reason |
|---|---|
| `local-proofreader.service` (9092) | TUI HTTP backend — all /v1/* endpoints |
| LanguageTool Docker (8010) | Stage 2 grammar check |
| `local-doorman.service` (9080) | Tier A/B/C inference routing |
| `local-slm.service` (8080) | Tier A local inference |
| `local-content.service` (9081) | RAG context for draft mode |
| `local-fs.service` | Corpus WORM ledger |
| `/usr/local/bin/service-proofreader` | Backend binary |

---

## Part 7 — Development Plan

### Phase 0 — Spike (1–2 days)

**Goal:** Validate the russh + ratatui over SSH foundation works on this VM.

- Fork the russh `ratatui_app.rs` example
- Bind on port 2222, accept an Ed25519 key, render a "Hello proof" widget
- Validate: PTY resize (`window_change_request`), bracketed paste (`Event::Paste`),
  crossterm key events, crossterm backend writing to russh channel
- Confirm GCE firewall rule for port 2222

**Deliverable:** A binary that responds to `ssh -p 2222 mathew@localhost` with a working
ratatui frame. Nothing else.

### Phase 1 — Auth & session management (1 week)

**Goal:** Users can log in with their SSH key; tenant is bound to the session.

- SQLite schema: `users (id, fingerprint, username, tenant, role, created_at, active)`
- russh `auth_publickey` handler: lookup fingerprint → `Auth::Accept { remaining_methods: [] }`
- Session struct: `user: User, tenant: Tenant, mode: Mode, buffers: HashMap<TabId, Buffer>`
- `proofctl` admin CLI: `user add`, `user list`, `user disable`, `user rotate-key`
- Status bar: show `username@tenant` and session duration
- Basic pane structure with F1 help overlay

**Deliverable:** `ssh -p 2222 proof@host` with registered key → TUI with identity in status bar.

### Phase 2 — Proofread core (2 weeks)

**Goal:** Full proofread flow in the TUI, closing the apprenticeship loop.

- `tui-textarea` integration for paste input
- `Event::Paste` → atomic buffer insert
- Protocol picker (static list of 18 GenreTemplate variants; fuzzy filter via `nucleo`)
- HTTP client to service-proofreader `/v1/proofread` (300s timeout; spinner during wait)
- Status-bar feedback during pipeline stages (via `/v1/health/ready`)
- `similar::TextDiff` → `Vec<Suggestion>` with severity from `findings` field
- `syntect` 24-bit colorization for diff panes
- `tui-scrollview` for long documents
- Per-suggestion verdict keybindings (`a`/`r`/`e`/`A`/`R`)
- POST `/v1/verdict` on session complete → corpus event

**Deliverable:** Full proofread workflow, feature-equivalent to the web UI, over SSH.

### Phase 3 — F12 gate + offline mode (1 week)

**Goal:** SYS-ADR-10 compliance; Doctrine claim #54 compliance.

- F12 keybinding → `f12_gate.rs` widget
- File ingest: path input → read + insert into paste buffer (only path via F12)
- Text ingest: paste via F12 gate with explicit confirmation step
- Offline detection: poll `/v1/health/ready`; switch to deterministic-only mode
- Disabled-state UX: greyed inference-dependent UI; `/status` shows what's offline
- Tantivy search integration: `/search <query>` → calls service-content Tantivy index

**Deliverable:** SYS-ADR-10 and claim #54 compliant; offline mode functional.

### Phase 4 — Draft mode (2 weeks)

**Goal:** "Draft new content" — protocol-driven generation with RAG context.

- `/new` command → fuzzy protocol picker
- Entity context search → service-content `/graph/neighborhood/<id>` RAG fetch
- Doorman Tier B request with RAG context + protocol scaffolding
- SSE consumer for streaming token output (Doorman `.content` shape)
- Streaming render into draft pane at 60Hz (throttle at high token rate)
- `/regenerate` cancels current stream, retries at same or higher tier
- `/tier b|c` switching with cost-cap awareness (read `SLM_YOYO_HOURLY_USD`)
- Accept → stage to `.agent/drafts-outbound/` with `foundry-draft-v1` frontmatter
  (five mandatory research-trail fields per Doctrine claim #39)
- Bilingual prompt: PROSE-TOPIC drafts optionally generate `.es.md` pair

**Deliverable:** Draft mode functional; project-editorial pipeline can pick up output.

### Phase 5 — Leapfrog polish (1 week)

**Goal:** The features that make this genuinely excellent.

- OSC 8 hyperlinks on every TOPIC/protocol reference, citation, `[citation-id]`
- 24-bit truecolor detection and application throughout
- Kitty graphics protocol probe + inline heatmap thumbnail (feature-flagged)
- Sixel fallback
- Multi-tab editing (`Ctrl-w n`, `Ctrl-w h/l`)
- Session persistence via SQLite (re-open last draft on reconnect)
- `/audit` verdict log viewer
- `/export` write buffer to file

**Deliverable:** All Leapfrog capabilities live; degrades gracefully to basic ANSI.

### Phase 6 — Operations (3–5 days)

**Goal:** Production-ready deployment.

- Systemd unit: `local-proofreader-tui.service` — `app-console-content`, port 2222,
  `Restart=on-failure`, `LimitNOFILE=65536`
- GCE firewall rule: allow TCP 2222 (restrict to known IP ranges if desired)
- Log rotation for TUI session logs
- Prometheus metrics endpoint (separate HTTP port) for Doorman-style observability
- Fail2ban rule for port 2222 (supplement the existing port 22 rule)
- Key-rotation runbook in `guide-provision-node.md`
- Graceful shutdown: flush corpus WAL on SIGTERM before exit

**Deliverable:** `local-proofreader-tui.service` active; monitoring live; runbook written.

### Total estimate

~7–9 weeks for Phases 0–6, single Rust engineer.
Phase 0 and Phase 1 can start immediately after the source tree is reconstituted.

---

## Part 8 — os-console & Project Scope Expansion

### 8.1 This project becomes the os-console TUI home

`project-proofreader` cluster expands scope to own:

1. **`app-console-content/`** — the F4 Content Cartridge (proofreader + draft)
2. **`os-console/`** — the chassis binary that loads Cartridges via F-keys

The chassis is a small binary: it handles SSH auth, renders the F-key navigation strip,
loads the active Cartridge binary (or Cartridge as a Rust module), and owns the F12 gate
globally. Other Cartridges (`app-console-keys`, `app-console-email`, `app-console-people`)
remain in their respective clusters but are discovered and loaded by `os-console`.

### 8.2 Cartridge boundary: standalone vs. embedded

**Phase 0–3 recommendation: Standalone binary, not cartridge.**

Ship `app-console-content` as a standalone SSH TUI that does proofread + draft. This validates
the entire stack without the cartridge-loading complexity. The F4 framing is satisfied: the
Content cartridge exists, it's just not yet embedded in the chassis.

**Phase 7 (future): os-console chassis development.**

Once `app-console-content` is stable, develop `os-console` as the chassis that:
- Owns the SSH server (moves from `app-console-content` to `os-console`)
- Renders the F-key strip and routes to active Cartridge
- Loads `app-console-content` as a compiled-in module (not a subprocess)
- Implements MBA pairing (cryptographic hardware-to-archive binding)
- Manages the session lifecycle across Cartridge switches

This is a separate engineering milestone, at least one quarter after Phase 6.

### 8.3 Manifest updates required

The cluster manifest (`.agent/manifest.md`) tetrad section needs updating:

```yaml
# Change in manifest.md:
focus: app-console-content/ (NEW Rust TUI — replaces app-console-proofreader/)
        os-console/ (NEW chassis — F-key navigator; Cartridge loader)
        service-proofreader/ (existing; API unchanged; serves TUI)
```

The `app_console_proofreader` references in `runtime_artifacts` update to:
- `/usr/local/bin/app-console-content` (replaces `app-console-proofreader`)
- `local-proofreader-tui.service` (replaces `local-proofreader-console.service` + `local-proofreader-public.service`)

---

## Part 9 — Open Questions

These require resolution before or during development.

| # | Question | Owner | Urgency |
|---|---|---|---|
| Q1 | Can the canonical `cluster/project-proofreader` branch at `origin/cluster/project-proofreader@e24b778` be restored with source? Or is clean-slate rewrite the intent? | Command Session | **Pre-development blocker** |
| Q2 | What is the service-content REST contract? Need `GET /graph/entity/<id>`, `GET /graph/neighborhood/<id>` for draft-mode RAG. Where is the canonical contract? | project-content / Command | Phase 4 |
| Q3 | What task-type string does `prose-draft` use in the corpus directory tree? Is `proofread-prose-draft` the right label or does it align with the existing `proofread-prose-*` taxonomy in the manifest? | Command Session | Phase 2 |
| Q4 | Is `local-proofreader-public.service` unit file present at `/srv/foundry/infrastructure/local-proofreader/`? If not, copy from `/etc/systemd/system/` before teardown. | Totebox (this session) | Before teardown |
| Q5 | Which GCE firewall rules govern inbound to this VM? Who can open port 2222? | Command Session / operator | Phase 6 |
| Q6 | Does the `tui-corpus-producer.md` convention (`conventions/tui-corpus-producer.md` — referenced from `guide-tier-a-sysadmin-tui.md` but inaccessible from this cluster) specify which task-types are TUI-corpus-eligible by default? | Command Session reads conventions/ | Phase 2 |
| Q7 | `slm-cli` source is at `pointsav-monorepo/service-slm/crates/slm-cli/`. Read it — this is the reference implementation for slash-command patterns and verdict signing. What is the SSH-signing mechanism for verdicts? | project-slm or this session | Phase 2 |
| Q8 | For FIDO2 (`sk-ssh-ed25519`) key support: does russh handle the FIDO2 key format in `auth_publickey`, or does it require additional handling? | Engineering research | Phase 1 |

---

## Part 10 — Doctrine Alignment Summary

| Doctrine claim | Relevance to pivot |
|---|---|
| **#45 — TUI-as-Corpus-Producer** | The doctrinal anchor. The TUI is the preferred corpus producer. The pivot fulfils this claim explicitly. |
| **#49 — Tier 0 Sovereign Specialist** | The adapter trained from proofreader verdicts becomes the Tier 0 specialist for `prose-edit` and `prose-draft`. TUI verdicts are the training source. |
| **#54 — Substrate-Without-Inference Base Case** | Offline mode required. File IO, diff, search, audit must work without service-slm. |
| **#35 §7A** | Creative-edited event-pair schema for corpus capture. `/v1/verdict` POST is the mechanism. |
| **#39** | Five mandatory research-trail fields on `foundry-draft-v1` frontmatter. Draft mode must produce compliant frontmatter. |
| **SYS-ADR-07** | No structured data through AI. TUI pipeline accepts prose only. |
| **SYS-ADR-10** | F12 mandatory. All ingest routes through the F12 gate widget. |
| **SYS-ADR-19** | No automated AI publishing to verified ledgers. Draft accept → `drafts-outbound/` only. |
| **MBA model** | SSH key = hardware pairing. The russh key-lookup model is doctrine-native. |

---

## Part 11 — Immediate Next Actions

In priority order:

1. **[OPERATOR/Command] Resolve source tree status** — confirm whether `origin/cluster/project-proofreader` source recovery is desired or clean-slate is the intent. This unblocks all engineering work.

2. **[OPERATOR/Command] Backfill `local-proofreader-public.service` unit file** — copy from `/etc/systemd/system/local-proofreader-public.service` to `/srv/foundry/infrastructure/local-proofreader/` and commit. Do this before teardown.

3. **[OPERATOR] Open GCE firewall port 2222** — needed for Phase 0 spike testing from outside the VM.

4. **[This Totebox] Phase 0 spike** — fork russh `ratatui_app.rs`; build and test the SSH foundation on port 2222. No auth, no pipeline — just validates the substrate.

5. **[This Totebox] Update cluster manifest** — revise tetrad focus lines, runtime_artifacts, and deployment section to reflect the TUI direction.

6. **[This Totebox] Archive old plans** — `TODO-ui-distillation-rewrite.md` and any web-UI-era plan documents move to `.agent/plans/archive/` or are deleted. They are superseded by this document.

7. **[Command] Update session-start.md** — replace "Phase B prompt fix is next" with the TUI pivot orientation.

8. **[Command] Send outbox message to project-slm** — request read access to `slm-cli` source for TUI vocabulary reference (slash commands, verdict signing, status bar patterns).

---

## Appendix A — Files to archive / delete

These web-UI-era documents are superseded by the TUI pivot:

| File | Action |
|---|---|
| `pointsav-monorepo/.agent/TODO-ui-distillation-rewrite.md` | Archive or delete — web redesign plan, obsolete |
| Any WFD `media-proofreader-woodfinegroup/` references | Already migrated; clean up stale references |

## Appendix B — Key reference paths

| Path | Purpose |
|---|---|
| `woodfine-fleet-deployment/vault-privategit-source/guide-tier-a-sysadmin-tui.md` | slm-cli TUI design pattern to mirror |
| `woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md` | F-key taxonomy and cartridge vocabulary |
| `woodfine-fleet-deployment/vault-privategit-source/guide-doorman.md` | Doorman wire format + verdict endpoints |
| `woodfine-fleet-deployment/cluster-intelligence/guide-yo-yo-nightly-pipeline.md` | Nightly adapter training pipeline |
| `woodfine-fleet-deployment/media-knowledge-documentation/guide-editorial-content-sweep.md` | Draft pipeline reference (draft-mode RAG pattern) |
| `/srv/foundry/conventions/tui-corpus-producer.md` | Doctrine claim #45 implementation spec (read from Command Session) |
| `pointsav-monorepo/service-slm/crates/slm-cli/` | Reference TUI implementation (slash commands, verdict signing) |

---

*Document authored 2026-05-16 by totebox@project-proofreader via 4-agent Opus research audit.*
*Next review: after Phase 0 spike completes (est. 2026-05-18).*
