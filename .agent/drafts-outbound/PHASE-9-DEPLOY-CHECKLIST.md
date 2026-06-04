# Phase 9 — Deploy Checklist

**Read NIGHT-BUILD-STATUS.md (git-verified) first.**

Current status (git-verified 2026-06-04): Phases 1–8 committed (HEAD `e94bfa9d`).
Auth/edit/CodeMirror removed (`0184fb16`, git-only workflow per Q1); openapi
regenerated (`e94bfa9d`). `cargo build --release` PASS (8m 28s; 3 warnings; 0
errors); binary 12M.

**No authentication setup is required.** Auth was removed entirely (Q1 = git-only).
There is no SQLite database to initialise, no admin user to seed, and no
`--admin-username` / `WIKI_ADMIN_*` step. The binary serves read-only;
content is edited in git, not in-browser.

---

## Pre-deploy

- [x] Q3 resolved: public URL is **documentation.pointsav.com** (port 9090,
  `local-knowledge-documentation.service`, brand PointSav, TOPIC + GUIDE).
  Confirmed per BRIEF §15 (2026-06-04).
- [ ] Run: `cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge && cargo build --release`
  (binary already built 2026-06-04 at `/srv/foundry/cargo-target/mathew/release/`; re-run if source changed since)
- [ ] **Dead-link gate must pass before promote (L18/L29).** Current state FAILS:
  - `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-documentation` → 4,568 dead links (MUST resolve)
  - `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-projects` → 396 dead links (MUST resolve)
  - `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-corporate` → 290 dead links (known-issue repo per BRIEF §9; deploy of corporate instance is provisional until its gate passes)
- [ ] Confirm DESIGN-TOKEN-CHANGE draft has received master_cosign from project-design
  (file: `.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md`)
- [ ] Stage 6 promote of `app-mediakit-knowledge` (Command Session) once gates clear

---

## Install binary

The resolved workspace cargo target dir is `/srv/foundry/cargo-target/mathew/`
(NOT a per-clone `app-mediakit-knowledge/target/`).

```bash
sudo cp /srv/foundry/cargo-target/mathew/release/app-mediakit-knowledge \
    /usr/local/bin/app-mediakit-knowledge
sudo chmod 755 /usr/local/bin/app-mediakit-knowledge
```

---

## Install knowledge.toml config files

```bash
sudo mkdir -p /etc/local-knowledge/
sudo cp /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/config/documentation.toml \
    /etc/local-knowledge/documentation.toml
sudo cp /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/config/projects.toml \
    /etc/local-knowledge/projects.toml
sudo cp /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/config/corporate.toml \
    /etc/local-knowledge/corporate.toml
```

Per BRIEF §15. Review each `.toml` after copy — confirm `[[mount]] path =`
points to the correct content-repo checkout on this host (templates use
`/srv/foundry/clones/project-knowledge/media-knowledge-*`). Bind ports:
documentation 9090, projects 9093, corporate 9095 (nginx reverse-proxies).

---

## Systemd units update

For each of the three service unit files (`local-knowledge-documentation.service`,
`local-knowledge-projects.service`, `local-knowledge-corporate.service`):

- [ ] Add: `Environment="WIKI_KNOWLEDGE_TOML=/etc/local-knowledge/{instance}.toml"`
  substituting `documentation`, `projects`, or `corporate` for `{instance}`
- [ ] Remove obsolete lines: `Environment="WIKI_CONTENT_DIR=..."` and any
  `WIKI_GUIDE_DIR=...` lines — these legacy vars are superseded by the
  knowledge.toml mount declaration (and are ignored when knowledge.toml is set)
- [ ] No `WIKI_ADMIN_USERNAME` / `WIKI_ADMIN_PASSWORD_HASH` — auth removed; do
  not add any admin env vars
- [ ] Run: `sudo systemctl daemon-reload`

Unit file locations: `/srv/foundry/infrastructure/local-knowledge-{documentation,projects,corporate}/`

Example final unit `[Service]` block (documentation instance):
```ini
[Service]
ExecStart=/usr/local/bin/app-mediakit-knowledge serve
Environment="WIKI_KNOWLEDGE_TOML=/etc/local-knowledge/documentation.toml"
Restart=always
RestartSec=5
```

---

## Restart services

```bash
sudo systemctl restart local-knowledge-documentation
sudo systemctl restart local-knowledge-projects
sudo systemctl restart local-knowledge-corporate
```

The server builds its Tantivy search index before binding the port (~10s on
the documentation corpus). Wait ~12 seconds after each restart before running
verification curls.

---

## Verify

```bash
# Health
curl -s http://127.0.0.1:9090/healthz
# Expected: ok

# Documentation instance — title
curl -s http://127.0.0.1:9090/ | grep -o '<title>[^<]*</title>'
# Expected: <title>PointSav Documentation</title>

# Projects instance — title
curl -s http://127.0.0.1:9093/ | grep -o '<title>[^<]*</title>'

# Corporate instance — title
curl -s http://127.0.0.1:9095/ | grep -o '<title>[^<]*</title>'

# Font preloads in head (L23 — exactly 2). Both tags are on one line,
# so use grep -o (NOT grep -c, which reports 1):
curl -s http://127.0.0.1:9090/ | grep -o 'rel="preload"' | wc -l
# Expected: 2

# Search — endpoint is /search (HTML page) or /api/complete (JSON autocomplete).
# There is NO /api/search route in this build.
curl -s -o /dev/null -w "%{http_code}\n" 'http://127.0.0.1:9090/search?q=substrate'
# Expected: 200
curl -s 'http://127.0.0.1:9090/api/complete?q=substrate' | head -c 200
# Expected: JSON array of {slug,title} autocomplete results

# Article page — git-only workflow means NO editor surface.
# Note /wiki/<slug> 301-redirects to the path-qualified canonical URL.
curl -sL http://127.0.0.1:9090/wiki/slm-tiered-substrate | grep -c 'editor.js'
# Expected: 0
curl -sL http://127.0.0.1:9090/wiki/slm-tiered-substrate | grep -c 'cm-saa'
# Expected: 0
curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:9090/wiki/slm-tiered-substrate
# Expected: 301 (redirect to /wiki/substrate/slm-tiered-substrate, which serves 200)

# Mobile — verify on physical phone
# Open http://<vm-ip>:9090/ on phone (via SSH tunnel or direct).
# Check: no Home Indicator overlap on bottom chrome (L24 safe-area-inset).
# Press Cmd+K: verify command palette opens. Navigate /es/: Spanish labels.
```

---

## Post-deploy

- [ ] Update BRIEF §15 DNS status for documentation instance after cutover
- [ ] Route DESIGN-TOKEN-CHANGE draft to project-design for master_cosign
  (file: `.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md`)
- [ ] Update binary ledger: add entry to `data/binary-ledger/app-mediakit-knowledge.jsonl`
  with sha256 of installed binary
  ```bash
  sha256sum /usr/local/bin/app-mediakit-knowledge
  ```
- [ ] Update BRIEF Phase 9 status to "deployed: 2026-06-04" (or actual deploy date)
- [ ] Create session-end brief entry in `.agent/briefs/`

---

## All done?

Update BRIEF status field to `deployed: <date>` and commit a session-end brief.
Route DESIGN-TOKEN-CHANGE to project-design outbox.
