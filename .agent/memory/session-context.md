# Session Context — project-knowledge Totebox

Format spec: `~/Foundry/conventions/session-context-format.md`.
Keep only the 5 most recent entries; push oldest to `session-context-archive.md`.

---

## Operator preference digest

- Auto mode preferred: batch all in-scope work, minimal interruptions, shut down clean.
- "Keep going" = proceed with all remaining in-scope items without pausing.
- NEXT.md scope: Totebox-executable items only; cross-archive items route to outbox immediately.
- Stale toggle lock: if `.toggle.lock` held by dead PID, operator must remove manually (`rm -f`); agent cannot self-authorize `FOUNDRY_CONFIRM_DESTRUCTIVE=1`.

---

## Cross-archive carry-forward

- [ ] Command: promote archive ops commits (tip `a2e79f9e`) + promote sub-clone `/health` fix (`69095f85`) + binary rebuild + redeploy to 9090/9093/9095.
- [ ] project-jennifer: MCP tasks blocked on jennifer:jennifer filesystem ownership.
- [ ] project-console: manifest contamination — needs project-console Totebox.
- [ ] project-bim: woodfine-bim-library Stage 6 — needs Command.
- [ ] project-intelligence: residual commit fix — needs project-intelligence Totebox.
- [ ] Phase E: TOKEN-CHANGE cosign propagation to editorial copy — Command-scope.

---

## Rolling entries (newest first)

### 2026-06-12 | totebox | claude-code | continuation after context compaction

**Done this session:**
- Added `/health` alias to `server/mod.rs` (1-line route — reuses `healthz` handler); 129 tests pass; committed `69095f85` (sub-clone).
- Stripped project-data contamination from NEXT.md (lines 1–92 were project-data content); inbox message `command-20260613-app-mediakit-knowledge-nightly-smoke-tes` marked actioned; committed `f7295cf8` (archive).
- Updated BRIEF tip hashes, session log, gate status — committed `a2e79f9e` (archive).
- Stage 6 READY sent to Command for sub-clone `69095f85` (msg-id: `command-20260613-stage-6-ready-project-knowledge-sub-clon`).

**Pending / carry-forward:**
- Command: promote archive ops + `/health` sub-clone commit → rebuild binary → redeploy 9090/9093/9095.
- All other carry-forward: cross-archive; routed to outbox.

**Operator preferences surfaced:** none new.

---

### 2026-06-12 | totebox | claude-code | close-out

**Done this session:**
- Phase 0 gate verified: 689 articles / 0 dead links / 0 missing fields (exit 0). Dead link fixed by project-editorial.
- Stage 6 confirmed: origin/main advanced to `9a1326df`.
- Archive ops commits (4e2ddf95 → 76671ddd) committed; Stage 6 READY sent (msg-id: `command-20260612-stage-6-ready-project-knowledge-archive-`).
- BRIEF Phase 0 → Complete 2026-06-12; completion test all three conditions DONE.

**Pending / carry-forward:**
- Command: promote archive ops + binary rebuild + redeploy.

---

### 2026-06-12 | totebox | claude-code

**Done this session:**
- Phase 0 gate commit `9a1326df`: `scripts/stage6-gate.sh` wired; `wikilink-unresolved` span removed (L18 complete); cross-mount resolution confirmed.
- Gate reported 1 dead link; fix sent to project-editorial.
- Pre-promote code-fix items closed (Doorman stubs, navigation portlet test — both already implemented).
- BRIEF §2 updated; L21/L25/M8 marked done.

**Pending:** project-editorial content fix (1 dead link → fixed in follow-on session).
