---
schema: foundry-artifact-registry-v1
project: project-design
last_updated: 2026-06-20
---

# project-design Artifact Registry

Persistent record of artifacts produced by this archive.
Updated at session end when new artifacts are staged or committed.

Routing:
- TOPIC / GUIDE / COMMS / PROSE-RESEARCH → `drafts-outbound/` → project-editorial
- DESIGN-* / ASSET-* → `drafts-outbound/` → project-design
- CODE-* (monorepo) → commit to `pointsav-monorepo/` sub-clone; Stage 6 request to Command

---

## CODE — Monorepo artifacts

Commits to `pointsav-monorepo/` sub-clone. Stage 6 pending as of 2026-06-19.
Full promote range: `6f21f580` through `fc4d0978`.

### Phase B — Cross-platform release (complete; Stage 6 pending)

| Commit | Crate(s) | Description |
|--------|----------|-------------|
| `6f21f580` | os-console | CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | — | ops: session close |
| `d58960b4` | — | ops: BRIEF update |

### Phase 8 — Polish (complete; Stage 6 pending)

| Commit | Crate(s) | Description |
|--------|----------|-------------|
| `7259d2a0` | app-console-keys | people_endpoint default :9091 added to ProfileConfig |
| `ac7eb500` | app-console-keys, app-console-email, app-console-slm, app-console-system, app-console-input, app-console-people | Truecolor Rgb variants across cartridges; F2 PeopleCartridge scaffold (contact list, detail view, :9091 fetch loop) |

### Phase 9 — Operations (in progress; Stage 6 pending)

| Commit | Crate(s) | Description |
|--------|----------|-------------|
| `d2afebfe` | os-console/infrastructure | local-console.service + local-pairing-server.service systemd units |
| `78941244` | os-console | Graceful SIGTERM handling — terminal restored on systemd stop |

### Phase 10 — Sprint (complete; Stage 6 pending)

| Commit | Crate(s) | Description |
|--------|----------|-------------|
| `bc95acfa` | app-console-keys, app-console-content | Rgb color helpers (all cartridges); content search session persistence |
| `abd8d019` | app-console-people | F2 PeopleCartridge full scaffold — contact list + CRUD stubs; service-people :9091 |
| `fc4d0978` | app-console-keys | Chassis auto-reconnect watchdog (exponential backoff 2s→60s cap) |

---

## TOPIC / GUIDE — Editorial drafts (staged)

Drafts staged in `.agent/drafts-outbound/`. Routing message sent to project-editorial 2026-06-19.

| File | Type | Status |
|------|------|--------|
| `TOPIC-os-console-architecture.draft.md` | TOPIC | staged; project-editorial pickup pending |
| `TOPIC-os-console-architecture.es.draft.md` | TOPIC (ES) | staged; project-editorial pickup pending |

Committed in `7d70bb6a`.

---

## DESIGN — Design drafts (staged in drafts-outbound — non-console, routed out)

The following DESIGN artifacts were staged here from prior sessions spanning
multiple domains. Routing message sent to project-design 2026-06-19.

These are NOT project-console-originated; they arrived via session contamination
(M-17 era). Files remain in drafts-outbound until project-design picks them up.

16 DESIGN files (orgchart tokens, knowledge platform, wireframes, doc components).
See outbox message `project-console-20260619-*-design-drafts-pickup` for full list.

---

## Note on prior artifact-registry.md content

The previous content of this file (dated 2026-06-14) contained JOURNAL artifact
tables (J1–J8) and project-system/project-gis/project-editorial entries with no
project-console content. This appears to be contaminated registry content from
another archive propagated during M-17 cleanup. Replaced 2026-06-19 with
project-console's own artifact record.

If the JOURNAL paper registry (J1–J8) belongs to another archive, the Command
Session should route it to the correct artifact-registry.md.
