# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

---

## 2026-05-20 session 2 | Totebox | claude-code

**Done this session:**
- Stage sovereign-mesh TOPIC: expanded one-sentence stub to full PPN architecture topic
  (hub-spoke topology, WireGuard overlay, ppn0, 16-byte binary protocol on port 8090,
  Genesis Protocol + Diode Standard integration, three node roles). EN + ES drafts staged
  to `.agent/drafts-outbound/`. Outbox message sent to project-editorial.
- Fixed `os-infrastructure/Makefile` — wrong script name (`forge_infrastructure_iso.sh` → `forge_iso.sh`)
- Fixed `os-infrastructure/forge_iso.sh` — old monorepo path → `/srv/foundry/vendor/pointsav-monorepo`
- Gitignored build artifacts in `os-infrastructure/` and `os-network-admin/` — 14 tracked binaries
  removed from index; `.gitignore` created in both crates
- Created `app-infrastructure-onprem/`, `app-infrastructure-leased/`, `app-infrastructure-cloud/`
  Reserved-folder scaffolds with bilingual READMEs
- Split `system-network-interface` — extracted F8 Terminal Gateway binary to new `app-network-admin/`
  crate; `system-network-interface` is now a pure lib with no std deps; both compile clean

**Commits this session:**
- `88831f63` — topic: stage sovereign-mesh EN+ES drafts
- `d3c6a7c8` — fix: Makefile, forge_iso.sh, gitignore build artifacts
- `a958b217` — scaffold: app-infrastructure-onprem, -leased, -cloud
- `b2eb755c` — refactor: split system-network-interface → app-network-admin

**Pending / carry-forward:**
- 4 operator decisions still blocking code and guide work (EAPOL vs Genesis Protocol;
  subnet `10.50.0.0/24` ratification; GCP static IP; Laptop A/B IPs + DNS status)
- `os-infrastructure/src/main.rs` missing symbols — blocked on above decision
- sovereign-mesh drafts need project-editorial pickup → commit to content-wiki-documentation
- GUIDE fix: `guide-deploy-vpn.md` path (cross-repo, Command Session admin-tier scope)

**Operator preferences surfaced:**
- Works through NEXT.md items systematically — confirmed each section before proceeding
- No notable new preferences beyond what was recorded in session 1

---

## 2026-05-20 session 1 | Totebox | claude-code

**Done this session:**
- Startup sequence completed (role confirmed, lock written, manifest + inbox + NOTAM read)
- Full archive read-through: focus crates (`os-infrastructure`, `os-network-admin`,
  `system-network-interface`, `system-substrate-broadcom`), all infrastructure TOPICs in
  `content-wiki-documentation`, and all infrastructure GUIDEs in `woodfine-fleet-deployment`
- Comprehensive TODO written to `.agent/plans/project-infrastructure-todo.md` (339 lines,
  5 sections: housekeeping, TOPICs, GUIDEs, code, operator decisions)
- Section 1 housekeeping completed:
  - `session-start.md` corrected (was `archive: project-intelligence`)
  - `manifest.md` planned_topics slugs corrected to match published wiki filenames
  - `NEXT.md` replaced with infrastructure-scoped items
  - `.agent/memory/` created and seeded
  - 6 project-intelligence plan files removed from `.agent/plans/`
  - 11 project-intelligence drafts removed from `.agent/drafts-outbound/`
  - `CLAUDE.md` + `manifest.md` committed together

**Pending / carry-forward:**
- Section 2: expand `sovereign-mesh.md` stub → full PPN architecture topic
- Section 4a: fix broken build in `os-infrastructure` — blocked on EAPOL vs Genesis Protocol decision
- Section 4b: split `system-network-interface` crate
- Section 5: four operator decisions needed (EAPOL vs Genesis Protocol; subnet; GCP IP; Laptop IPs)

**Operator preferences surfaced:**
- Wants comprehensive TODO plans saved to `.agent/plans/` as markdown before starting work
- Working on PPN / network OS layer (os-infrastructure + os-network-admin as primary focus)
