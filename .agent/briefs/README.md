# .agent/briefs/ — Durable project briefs & planning artifacts

`BRIEF-*.md` files are **permanent git-tracked artifacts** — not temp files.
Engine-agnostic: all engines (Claude Code, Gemini CLI) read and write here.

## Rules

- **Never delete a brief.** Supersede by editing `status: archived`, or
  `git mv` to `archive/`.
- **Frontmatter required:** `artifact: brief`, `status: active|archived`.
- **Filename:** `BRIEF-<topic>.md` — uppercase prefix signals permanence.
- Save planning files HERE — not `~/.claude/plans/` or `~/.gemini/tmp/`.
- AGENT.md startup step 7 reads this file; shutdown step 1 writes `BRIEF-<topic>.md`.

## Active briefs

| File | Subject | Status |
|---|---|---|
| BRIEF-substrate-phd-thesis-2026-05-27.md | JOURNAL J2 / ASPLOS — Trustworthy Systems from Verified Primitives; seL4/NetBSD; system-ledger benchmarks | active |
| BRIEF-totebox-transformation.md | VM fabric implementation — hypervisor setup, os-infrastructure, Genesis Protocol | active |

## Archived briefs

| File | Archived | Notes |
|---|---|---|
| archive/BRIEF-LEAPFROG-2030.md | — | os-* resource targets, Phase 2/3 disk/RAM targets |
| archive/BRIEF-OS-FAMILY.md | — | os-* family reference — five os-* types, placement principle |
| archive/BRIEF-VM-ARCHITECTURE.md | — | VM-* to os-* canonical mapping, deployment model |
| archive/BRIEF-PPN-ARCHITECTURE.md | — | PPN PhD-thesis-quality architecture — formally-isolated virtualisation |
| archive/BRIEF-PPN-DEV-BOOTSTRAP.md | — | PPN dev-environment bootstrap — dogfood principle, WireGuard mesh |

## Non-brief files kept here

| File | Role |
|---|---|
| domain-map.tsv | Domain-map data (cross-archive) |
| vocabulary-baseline.tsv | Vocabulary-baseline data (cross-archive) |
| archive/audit-foundry-wide-2026-05-16.md | Foundry-wide vocabulary/trademark/file-hygiene audit (one-time artifact; archived 2026-06-10) |
| archive/ | Archived and superseded briefs |

## Contamination cleanup — 2026-06-08

A bulk `.agent/` provisioning pass had copied ~42 foreign briefs and ~22 foreign
drafts-outbound files into this archive. All were removed this session and routed to
their home archives via outbox messages:

| Destination | msg-id |
|---|---|
| project-intelligence | project-infrastructure-20260608-brief-relocation-5-orphaned-briefs-ai-au |
| project-console | project-infrastructure-20260608-brief-relocation-10-orphaned-briefs-4-dr |
| project-knowledge | project-infrastructure-20260608-brief-relocation-9-orphaned-briefs-4-wik |
| project-gis | project-infrastructure-20260608-brief-relocation-2-orphaned-briefs-8-gis |
| project-editorial | project-infrastructure-20260608-brief-relocation-15-orphaned-archive-bri |

Content recoverable from git history: `git show HEAD~1:.agent/briefs/<filename>`
