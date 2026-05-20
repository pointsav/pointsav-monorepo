---
archive: project-console
updated: 2026-05-20
---

# NEXT — project-console

> Architecture plan: `.agent/plans/os-console-platform.md`
> Coding roadmap: `.agent/plans/leapfrog-2030-coding.md`
> Session orientation: `.agent/session-start.md`

---

## Direction (updated 2026-05-20)

**os-console platform — chassis-first.** Full Leapfrog 2030 pivot: os-console binary +
app-console-keys chassis + compiled-in cartridges (F1-F12). app-console-content (F4)
is Phase 3 cartridge. MBA peer-to-peer (system-gateway-mba).
Doorman endpoint: `http://localhost:8011`. **Phase 1 COMPLETE. Phase 2 = auth + MBA next.**

**Rename done:** project-proofreader → project-console (directory + branch). ✓

---

## Blocked — waiting on Command Session

- [ ] **[BLOCKER] Confirm clean-slate intent** — `pointsav-monorepo/` source tree is empty
  (filter-repo swept source). Last source SHA: `788b3722` in local reflog. Confirm:
  proceed clean-slate TUI rewrite (no restore of old web UI source). `[2026-05-16 totebox@claude-code]`

- [ ] **[BLOCKER] Backfill `local-proofreader-public.service`** — no canonical copy at
  `/srv/foundry/infrastructure/local-proofreader/`. Command Session: copy live unit file
  and commit BEFORE teardown. `[2026-05-16 totebox@claude-code]`

- [ ] **Teardown web UI** — requires sudo; Command Session action:
  stop + disable `local-proofreader-console` + `local-proofreader-public`, remove nginx
  vhost, certbot delete, rm binary. Checklist in plan §Part 6. `[2026-05-16 totebox@claude-code]`

- [ ] **Open GCE firewall port 2222** — needed before Phase 0 spike can be tested
  externally. Operator action. `[2026-05-16 totebox@claude-code]`

- [x] **Read `conventions/tui-corpus-producer.md`** — relayed inline via inbox msg `command-20260517-tui-pivot-relay`. Key specs: /feedback keybinding (G/R/B verdicts), adapter quality budget (200–500 interactions), per-tenant adapter ownership. `[2026-05-19 totebox@claude-code]`

- [ ] **slm-cli source review** — `pointsav-monorepo/service-slm/crates/slm-cli/` does NOT exist yet (Phase 4 item in project-slm). No reference implementation available. Use inbox relay spec (§5) as design guide for slash commands. `[2026-05-19 totebox@claude-code]`

- [x] **Domain migration commit `9ede81f` status** — RESOLVED. WFD sub-clone at `7fdf36b`; `git ls-tree` confirms no `proofreader` entries — filter-repo history rewrite cleaned the stale woodfinegroup catalog. No rebase needed. `[2026-05-19 totebox@claude-code]`

---

## Phase 0 — Spike `[ COMPLETE ]` ✓

- [x] New crate: `pointsav-monorepo/app-console-content/`
- [x] Workspace `pointsav-monorepo/Cargo.toml` created
- [x] `Cargo.toml` dependencies: `ratatui 0.30`, `crossterm 0.28`, `russh 0.60`, `tokio`, `rand 0.10`, `anyhow`
- [x] `src/main.rs` — russh 0.60 Handler impl (native async fn, no async_trait); TerminalHandle (sink+flush); per-session spawn_blocking render loop; `cargo build` green
- [x] **Gate passed (2026-05-17):** `ssh -p 2222 -i ~/.ssh/google_compute_engine mathew@localhost` → ratatui frame rendered; ANSI output confirmed in server log
- [x] Committed: `feat: Session 1 — russh + ratatui spike; SSH TUI skeleton on port 2222`

**Gate:** ✓ Passed. ratatui frame renders over SSH on port 2222.

---

## Phase 1 — Chassis `[ COMPLETE ]` ✓

- [x] `app-console-keys/` lib crate created — `Cartridge` trait, `FKey` enum, `AppConsoleKeys` chassis, F-key tab strip widget, status bar widget, `ConsoleConfig`
- [x] `os-console/` bin crate created — local PTY mode (`run_local`); SSH mode behind `#[cfg(feature = "ssh-server")]` stub
- [x] `app-console-content/src/cartridge.rs` — `ContentCartridge` implementing `Cartridge` (F4 placeholder)
- [x] Workspace `Cargo.toml` updated: 3 members (`app-console-keys`, `app-console-content`, `os-console`)
- [x] `cargo build`, `cargo build --release` green
- [x] Session 2 backfill committed: `auth.rs`, `db.rs`, `session.rs`, `ui/status_bar.rs`, `proofctl` CLI

**Gate:** ✓ Passed (`cargo build` green; `os-console` binary produces F-key tab strip + Content pane + status bar in local PTY mode).

---

## Phase 2 — Proofread core (est. 2 weeks) `[ NOT STARTED ]`

- [ ] `tui-textarea` integration for paste input
- [ ] `Event::Paste` → atomic buffer insert
- [ ] Protocol picker (18 GenreTemplate variants via `nucleo` fuzzy filter)
- [ ] HTTP client to `service-proofreader /v1/proofread` (300s timeout; spinner during wait)
- [ ] Status bar feedback during pipeline stages (poll `/v1/health/ready`)
- [ ] `similar::TextDiff` → `Vec<Suggestion>` with severity from `findings`
- [ ] `syntect` 24-bit colorization for diff panes
- [ ] `tui-scrollview` for long documents
- [ ] Per-suggestion verdict keybindings (`a`/`r`/`e`/`A`/`R`)
- [ ] POST `/v1/verdict` on session complete → corpus event

**Gate:** Full proofread workflow over SSH, feature-equivalent to the former web UI.

---

## Phase 3 — F12 gate + offline mode (est. 1 week) `[ NOT STARTED ]`

- [ ] `src/ui/f12_gate.rs` — SYS-ADR-10 ingest gate widget (file path input + confirm)
- [ ] All file/text ingest routes through F12; cannot be bypassed from other panes
- [ ] Offline detection: poll `/v1/health/ready`; switch to deterministic-only mode
- [ ] Disabled-state UX: greyed inference UI; `/status` shows what's offline
- [ ] Tantivy search: `/search <query>` → service-content Tantivy index at 9081

**Gate:** SYS-ADR-10 and Doctrine claim #54 compliant; offline mode functional.

---

## Phase 4 — Draft mode (est. 2 weeks) `[ NOT STARTED ]`

- [ ] `/new` command → fuzzy protocol picker
- [ ] Entity context: fuzzy search → `service-content /graph/neighborhood/<id>` RAG fetch
- [ ] Doorman Tier B request with RAG context + protocol scaffolding
- [ ] SSE consumer for streaming token output (`.content` field, not `.choices[0]`)
- [ ] Streaming render into draft pane at 60Hz
- [ ] `/regenerate` — cancel + retry at same or higher tier
- [ ] `/tier b|c` switching with cost-cap awareness
- [ ] Draft accept → stage to `.agent/drafts-outbound/` with `foundry-draft-v1` frontmatter
  (5 mandatory research-trail fields, Doctrine claim #39)
- [ ] PROSE-TOPIC: optional bilingual `.es.md` pair generation

**Gate:** Draft mode functional; project-editorial pipeline can pick up output.

---

## Phase 5 — Leapfrog polish (est. 1 week) `[ NOT STARTED ]`

- [ ] OSC 8 hyperlinks on TOPIC/protocol references, citations
- [ ] 24-bit truecolor detection + application throughout
- [ ] Kitty graphics protocol probe + inline heatmap (feature-flagged; degrade to ASCII)
- [ ] Sixel fallback
- [ ] Multi-tab editing (`Ctrl-w n`, `Ctrl-w h/l`)
- [ ] Session persistence via SQLite (re-open last draft on reconnect)
- [ ] `/audit` verdict log viewer
- [ ] `/export` write buffer to file

---

## Phase 6 — Operations (est. 3–5 days) `[ NOT STARTED ]`

- [ ] `local-proofreader-tui.service` systemd unit (replaces both console units)
- [ ] Unit source at `infrastructure/local-proofreader/local-proofreader-tui.service`
- [ ] Prometheus metrics endpoint (separate HTTP port)
- [ ] Fail2ban rule for port 2222
- [ ] Key-rotation runbook in `guide-provision-node.md`
- [ ] Graceful shutdown: flush corpus WAL on SIGTERM
- [ ] Update deployment instance MANIFEST.md

**Gate:** `local-proofreader-tui.service active`. Monitoring live. Runbook written.

---

## Phase 7 — os-console chassis (future, separate milestone)

- [ ] `os-console/` crate in `pointsav-monorepo` — F-key navigator chassis
- [ ] Move SSH server from `app-console-content` into `os-console`
- [ ] `app-console-content` becomes F4 Cartridge loaded by chassis
- [ ] MBA pairing (cryptographic hardware-to-archive binding)
- [ ] Other Cartridges: F2 People, F3 Email loadable alongside F4

---

## Open questions (unresolved)

| # | Question | Status |
|---|---|---|
| Q1 | Clean-slate intent on `pointsav-monorepo/` source? | Waiting Command |
| Q2 | service-content REST contract for `/graph/*` endpoints? | Waiting Command/project-content |
| Q3 | `prose-draft` task-type string in corpus directory tree? | Waiting Command |
| Q4 | `local-proofreader-public.service` source backfilled? | Waiting Command |
| Q5 | GCE firewall port 2222 — who opens it? | Waiting operator |
| Q6 | `conventions/tui-corpus-producer.md` — task-type taxonomy? | **Resolved** — relayed inline 2026-05-17 |
| Q7 | `slm-cli` verdict-signing mechanism? | **Blocked** — slm-cli crate not yet written (project-slm Phase 4) |
| Q8 | russh FIDO2 (`sk-ssh-ed25519`) key support? | Engineering research Phase 1 |

---

## Completed (2026-05-16 — session 1)

- [x] 4-agent Opus research audit (codebase, deployment, architecture, TUI technology)
- [x] Strategic plan written: `.agent/plans/tui-pivot-2030.md`
- [x] `session-start.md` updated to reflect TUI pivot
- [x] `NEXT.md` created (this file)
- [x] Outbox message queued for Command Session (8 action items)
- [x] Inbox messages status-updated (3 messages actioned/operator-pending)
- [x] Memory index updated with pivot direction
- [x] 10-session autonomous development plan created: `~/.claude/plans/can-you-make-a-deep-naur.md`

## Completed (2026-05-17 — session 2)

- [x] Workspace `pointsav-monorepo/Cargo.toml` created
- [x] `app-console-content/Cargo.toml` created (ratatui 0.30, crossterm 0.28, russh 0.60, rand 0.10, tokio, anyhow)
- [x] `app-console-content/src/main.rs` — full russh 0.60 Handler + ratatui spike; `cargo build` green
- [x] `app-console-content/src/bin/proofctl.rs` — stub binary
- [x] **Phase 0 gate passed** — ratatui frame renders over SSH port 2222; committed

## Completed (2026-05-20 — session 3)

- [x] Full architecture Q&A resolved: os-console/app-console-keys/cartridge hierarchy; MBA peer-to-peer vs PPN isolation; F-key map canonical; service-input Ring 1; PDF=pdfium-render Kitty/Sixel
- [x] Plan written: `.agent/plans/os-console-platform.md` — consolidated architecture reference (supersedes tui-pivot-2030.md Phase 7 deferral + wrong port)
- [x] Plan written: `.agent/plans/leapfrog-2030-coding.md` — chassis-first phased coding roadmap (9 phases, Phase 0 done, Phase 1 next)
- [x] Draft TOPIC: `topic-machine-based-authorization.md` — Geometric Security, peer-to-peer, pairing ceremony
- [x] Draft TOPIC: `topic-pointsav-private-network.md` — hub-and-spoke WireGuard, Mesh Fusion, PPN vs MBA isolation
- [x] Draft TOPIC: `topic-os-console-platform.md` — os-console binary, app-console-keys chassis, Cartridge architecture
- [x] Draft TOPIC: `topic-input-machine.md` — F12 The Anchor, service-input Ring 1, SYS-ADR-10, Zero-Form
- [x] Draft GUIDE: `guide-mba-pairing-ceremony.md` — proofctl user add/list/rotate-key/disable + pairings.yaml
- [x] Draft GUIDE: `guide-os-console-operator.md` — terminal requirements, F-key map, slash commands, config
- [x] Outbox message to Command: rename project-proofreader→project-console, add content-wiki-documentation sub-clone, add app-console-gis/slm/system to catalog, fix Doorman port in manifest.md
- [x] `session-start.md` updated to reflect chassis-first architecture and new plans
- [x] All artifacts committed: `f7ad7dc`

## Completed (2026-05-20 — session 4)

- [x] **Rename actioned by Command** — directory + branch now `project-console` ✓
- [x] **Phase 1 COMPLETE** — `app-console-keys` lib crate (Cartridge trait, FKey enum, chassis, tab strip, status bar, config); `os-console` bin crate (local PTY mode, SSH feature-gated stub); `ContentCartridge` in `app-console-content`; workspace updated to 3 members
- [x] Session 2 backfill committed to monorepo: `auth.rs`, `db.rs`, `session.rs`, `ui/status_bar.rs`, full `proofctl` CLI — monorepo SHA `13848313`
- [x] Phase 1 chassis committed to monorepo — `8d02bd56`; `cargo build` + `cargo build --release` both green

---

## Codebase notes

- **`pointsav-monorepo/` source tree:** 3 crates: `app-console-keys` (lib), `app-console-content` (lib + 2 bins), `os-console` (bin). `cargo build` green as of 2026-05-20.
- **`woodfine-fleet-deployment/` sub-clone:** intact at `7fdf36b` (post-security-cleanup canonical HEAD). ✓
- **`os-console/`:** EXISTS. Phase 1 binary. Local PTY mode. SSH server: Phase 2.
- **`service-proofreader/`:** not on disk; API fully documented. Backend runs from compiled binary at `/usr/local/bin/service-proofreader`.
