# NEXT.md — project-console

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-12 [totebox@claude-code]

---

## State file cleanup — 2026-06-12 (this session)

- [x] **NEXT.md contamination** — was project-data + project-knowledge content; replaced.
- [x] **manifest.md YAML frontmatter** — was project-knowledge fields; rewritten for project-console.
- [x] **BRIEF contamination sweep** — 3 active contaminated BRIEFs (knowledge-platform, os-totebox-ppn, project-intelligence-active-work) archived in place; outbox sent to Command for redistribution.
- [x] **BRIEF schema fixes** — cross-platform-release, dev-env-mcp-expansion, project-console-master corrected.
- [x] **archive/BRIEF-project-intelligence-master.md** — status renamed from 'relocated' to 'archived'.
- [x] **New active BRIEF** — `BRIEF-os-console-active-dev.md` created for Phases 8–10.
- [x] **README.md active-briefs table** — populated.
- [x] **Outbox** — Command notified of contaminated BRIEF redistribution + Stage 6 Phase B promote request.

---

## Stage 6 — Cross-platform Phase B

Commits awaiting push to staging mirrors + canonical promote:

| SHA | Subject |
|---|---|
| `6f21f580` | feat(release): Phase B — CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | ops(session): Phase B complete |
| `d58960b4` | ops(brief): mark Phase B complete |

Force-push to staging mirrors authorized by Command 2026-05-28 (inbox `command-20260528-console-answers`).
Outbox sent this session requesting Command action. See `BRIEF-os-console-active-dev.md` §Stage 6.

---

## Phase 8 — Polish (in progress)

See `BRIEF-os-console-active-dev.md` §Phase 8 for full scope.

- [ ] OSC 8 hyperlinks in ContentCartridge
- [x] Truecolor Rgb variants — email/slm/system/input cartridges (2026-06-12)
- [ ] Multi-tab support
- [ ] Session persistence across reconnect
- [x] `/audit` log viewer — already implemented in InputCartridge (confirmed 2026-06-12)
- [x] F2 People cartridge — scaffold complete; pending cargo check + commit (2026-06-12)

**Pending commit group (blocked on cargo check --workspace clean):**
- `feat(truecolor): extend Rgb color variants to email/slm/system/input cartridges`
- `feat(f2): scaffold PeopleCartridge — read-only contact list from service-people`
- `ops(systemd): draft local-console.service + local-pairing-server.service units`
- `docs(topic): TOPIC-os-console-architecture EN+ES draft → drafts-outbound`
- `ops(brief): Phase 8 progress — truecolor + F2 + systemd units + TOPIC draft`

---

## Phase 9 — Operations (drafts ready; deployment blocked)

See `BRIEF-os-console-active-dev.md` §Phase 9 for full scope.

- [x] `local-console.service` systemd unit — drafted (2026-06-12)
- [x] `local-pairing-server.service` systemd unit — drafted (2026-06-12)
- [ ] Prometheus metrics exporter
- [ ] fail2ban config for port 2222
- [ ] Graceful SIGTERM handling

Blocked on: GCE firewall port 2222 (operator action); vm-intelligence WireGuard provisioning (project-infrastructure).

---

## Operator-blocked (no Totebox action until input received)

- GCE firewall port 2222 — required for external SSH connections to os-console
- Peter SSH key — generate Ed25519; add via `proofctl user add peter --tenant woodfine --role editor`
- Tag `v0.1.0` on pointsav-monorepo — triggers GitHub Actions release build
- Branch rename `cluster/project-proofreader → cluster/project-console` on GitHub
