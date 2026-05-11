---
archived: 2026-05-07T00:00Z by master@claude-code (Task #17 session sweep)
---

---
from: task@project-marketing
to: master@claude-code
re: Session-start check complete — services healthy, inbox cleared, README defect fixed
created: 2026-05-07T16:00:00Z
priority: normal
---

Session-start ritual complete. Summary:

**Inbox:** 1 message archived (2026-05-07T00:00Z "Go-ahead — commit v0.0.1"). Commits 3bc17061 + d4d3153c + d3ee2ceb were already present from the prior session; no action was needed beyond archiving.

**Services:** Both running and responding correctly.
- `local-marketing.service` (woodfine) — port 9102 ✓
- `local-marketing-pointsav.service` (pointsav) — port 9101 ✓

**Defect fixed:** `app-mediakit-marketing/README.md` + `README.es.md` were still the old 3-line scaffold placeholder. Replaced with v0.0.1 documentation covering routes, env vars, two-deployment pattern, and roadmap. Committed as `0ef09cf6` (Jennifer).

**DNS/certbot:** Pending operator action via DreamHost (home.woodfinegroup.com + home.pointsav.com → VM external IP). No action needed from this session.

**Next:** Awaiting operator direction for v0.0.2 (theme system via CSS tokens from pointsav-design-system; per-tenant nav + branding differentiation). Ready to begin when directed.

— task@project-marketing


---
archived: 2026-05-06T19:05Z by master@claude-code (Master sweep)
note: v0.0.1 MVP implementation complete (2026-05-06T02:35Z). cargo check passes (245 pkgs, 0 errors).
  7 source files across 5 modules. WordPress admin chrome + multi-tenant + DataGraph. Bootstrap needed.
  Code unstaged on cluster/project-marketing — operator confirmation requested for Task to commit.
---
---
from: task@project-marketing
to: master@claude-code
re: v0.0.1 MVP implementation complete — ready for bootstrap
created: 2026-05-06T02:35:00Z
priority: high
---

## Summary

`app-mediakit-marketing` v0.0.1 MVP is implemented and `cargo check` passes clean
(245 packages; 0 errors; 2m37s first run).

## What was built

**New source files** in `app-mediakit-marketing/`:
- `Cargo.toml` — v0.0.1 with standalone workspace marker; axum 0.8 + tokio + maud + comrak + reqwest (rustls-tls, no openssl-sys dep)
- `src/main.rs` — CLI binary; `serve` subcommand with `SERVICE_MARKETING_*` env vars
- `src/lib.rs` — module declarations (content, render, server)
- `src/render.rs` — frontmatter parsing + comrak Markdown → HTML
- `src/content.rs` — page scan (content/*.md) + media scan (content/media/)
- `src/server.rs` — axum router, AppState, all route handlers, WordPress admin chrome, public landing chrome
- `static/style.css` — WordPress admin approximation + public landing styles

**Routes served:**
- `GET /` — public landing (renders content/index.md or default placeholder)
- `GET /page/{slug}` — individual content page
- `GET /wp-admin` — redirect to /wp-admin/dashboard
- `GET /wp-admin/dashboard` — Dashboard (page count + optional graph entity count)
- `GET /wp-admin/pages` — Pages list (all .md files, Edit/View row actions)
- `GET /wp-admin/media` — Media library (content/media/ directory)
- `GET /wp-admin/themes` — Themes stub (PointSav Default; v0.0.2 ships theme system)
- `GET /wp-admin/settings` — Settings (read-only env var display)
- `GET /healthz` — JSON health check

**Multi-tenant:** `SERVICE_MARKETING_MODULE_ID=woodfine|pointsav` selects tenant.
Site title defaults to "Woodfine" or "PointSav" if `SERVICE_MARKETING_SITE_TITLE` is not set.

**DataGraph:** optional entity count on dashboard. `fetch_entity_count()` tries
`GET {graph_url}/v1/graph/context?q=*&module_id=...&limit=1` with 2s timeout.
Fails silently — Tier 0 fully functional without graph connectivity.

**Sample content** written to deployment instance directories:
- `deployments/media-marketing-landing-1/content/` — Woodfine index.md + contact.md
- `deployments/media-marketing-landing-2/content/` — PointSav index.md + architecture.md

## Bootstrap actions needed (Master scope)

To bring home.woodfinegroup.com and home.pointsav.com live, Master needs to:

1. **Build release binary:**
   ```
   cd /srv/foundry/clones/project-marketing/pointsav-monorepo/app-mediakit-marketing
   sudo -u mathew /home/mathew/.cargo/bin/cargo build --release
   ```
   Note: CARGO_BUILD_JOBS=1 is set in /etc/environment. Build will be slow (~10-15 min first time).

2. **Install binary:**
   ```
   sudo install -m 755 target/release/app-mediakit-marketing /usr/local/bin/app-mediakit-marketing
   ```

3. **Write systemd units** (two instances, different ports and env):
   - `local-marketing.service` → woodfine, port 9100, content-dir deployments/media-marketing-landing-1/content/
   - `local-marketing-pointsav.service` → pointsav, port 9101, content-dir deployments/media-marketing-landing-2/content/

4. **Write nginx vhosts** for home.woodfinegroup.com → 9100 and home.pointsav.com → 9101.

5. **DNS + certbot** (after DNS A records point to VM external IP).

Infrastructure directory `infrastructure/local-marketing/` does not yet exist — needs
bootstrap.sh, service unit, and nginx conf authored (Master scope, parallel to project-knowledge
pattern at `infrastructure/local-knowledge/`).

## Commit needed

All changes are unstaged on cluster/project-marketing branch. This is Task scope to commit
via `commit-as-next.sh`. Awaiting operator confirmation to proceed with commit.

## v0.0.2 next items

- Theme CSS token integration (pointsav-design-system)
- Per-tenant branding in nav (logo, colour scheme)
- Edit endpoint for pages (POST /wp-admin/pages/{slug}/edit)
- Audit-logged page edits via Doorman /v1/audit/capture

---
mailbox: outbox-archive
owner: task@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Outbox Archive — project-marketing

