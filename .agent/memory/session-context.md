---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-05-21b | totebox@claude-code | plan-bim-objects-v2-committed

**Done:**
- Merged Deliverable 1 spec (authoritative FIN.xlsx sizes, Q1–Q6, Decisions 1–4) into plan-bim-objects.md → status: draft-v2
- Committed: 667c5f2 "plan: BIM Objects draft-v2 — authoritative sizes + Q1-Q6 + Decisions 1-4"
- Copied v2 to `outputs/plan-bim-objects.md` — accessible via `fpull bim outputs/`

**Inbox state at session end:** unchanged from prior session.

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` — READY, all data in plan-bim-objects.md v2
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (HIGH)
- DTCG 6 missing files (unblocked by Decision 3)
- Rust crate scaffold (deferred until DTCG complete)
- Stage 6: 32+ commits ahead of origin

**Operator preferences surfaced:** will proceed with Deliverable 1 in next session.

---

## 2026-05-21 | totebox@claude-code | plan-bim-objects

**Done:**
- Read all 15+ source documents from `inputs/` (15 PDFs, XLSX, DOCX, 3 collaborator folders)
- Created `.agent/plans/plan-bim-objects.md` — first draft BIM Objects specification (Key Plans / Tiles / Floor Plates / Building Width Calculator / Rust engine data model)
- Created `outputs/` directory at `/srv/foundry/clones/project-bim/outputs/` — accessible via `fpull bim outputs/`
- Resolved Q1–Q6 with operator for Deliverable 1 (Key Plans registry)
- Read FIN.xlsx `Summary_Key Plans` tab — authoritative sizes for all professional office Key Plans
- Specified Deliverable 1 fully in plan: `woodfine-bim-library/key-plans/key-plans-registry.md`
- Read inbox: received 4 Key Plans foundation decisions from Command (all answered) + NOTAM permission resolved notification

**Inbox state at session end:**
- `command-20260520-bim-foundation-decisions` (HIGH / pending): 4 decisions received — naming, HTML BIM_TOKENS deletion, scope (all 3 building types), tile disambiguation. **Unblocks** DTCG work + Rust scaffold
- B5 (HIGH / in-progress): Rust source for app-orchestration-bim — not started this session; pivoted to plan-bim-objects
- DTCG accuracy errors (2 messages / operator-pending): unchanged

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` (READY — plan complete)
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (still HIGH)
- DTCG 6 missing files (now unblocked by Decision 3 scope confirmation)
- Rust crate scaffold (still deferred until DTCG complete)
- Stage 6: 31+ commits ahead of origin

**Operator preferences surfaced:**
- Pull pattern: `fpull bim outputs/` → `/srv/foundry/clones/project-bim/outputs/` on VM
- Eco-region variants (landscaping/parking) deferred to later iteration
- Corporate Office sizing deferred until Floor Plate dimensions confirmed

---

## 2026-05-20 | totebox@claude-code | startup-only

**Done:** Startup ritual only — confirmed role (Totebox Session), wrote session lock, read manifest + inbox + session-start + NEXT.md + plans/README.md. No code, no artifact, no commit work performed.

**Inbox state at session end:**
- B5 (HIGH / in-progress): write Rust source for app-orchestration-bim with "BIM Object Library" naming, build binary — still not started
- DTCG accuracy errors (2 messages, operator-pending): blocked pending citations — unchanged
- Key Plans 4 operator decisions: paused — unchanged

**Pending / carry-forward:**
- B5 website update + Rust source + binary rebuild (active, pick up first)
- Key Plans foundation — 4 operator decisions (paused; briefing in `.agent/plans/key-plans-foundation-briefing.md`)
- DTCG token files — 6 missing files (paused pending Decision 2)
- Rust crate scaffold — 5 crates (deferred until DTCG complete)
- Stage 6: 30 commits ahead of origin

**Flag:** NOTAM unreadable — `/srv/foundry/NOTAM.md` and `~/Foundry/NOTAM.md` both return permission denied. Flagged to Command via outbox.

**Operator preferences surfaced:** none this session.
