# Phase 9 — Deploy Checklist

**Read NIGHT-BUILD-STATUS.md first — all phases must be green before deploying.**

Current status: Phases 0–5 and 8 PASS. Phases 6 and 7 are deferred
(not blockers — functional equivalents from prior build are in production).
`cargo build --release` PASS (7m 24s; 3 pre-existing warnings; no errors).

---

## Pre-deploy

- [ ] Resolve Q3: confirm public URL — `documentation.pointsav.com` or
  `documentation.woodfinegroup.com`? Update BRIEF §2 DNS-status field
  and NEXT.md before DNS cutover. (L28)
- [ ] Run: `cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge && cargo build --release`
  (binary already built 2026-06-04; re-run if any source has changed since)
- [ ] Run: `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-documentation` (must pass)
- [ ] Run: `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-projects` (must pass)
- [ ] Run: `cargo xtask check-content /srv/foundry/clones/project-knowledge/content-wiki-corporate`
  (expected: may fail due to 4 articles missing `last_edited:` + 2 stub stubs — note remaining issues;
  deploy of corporate instance is provisional until content gate passes)
- [ ] Confirm DESIGN-TOKEN-CHANGE draft has received master_cosign from project-design
  (file: `.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md`)

---

## Install binary

```bash
sudo cp /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/target/release/app-mediakit-knowledge \
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

Review each .toml after copy — confirm `path =` points to correct content-repo checkouts
for this host. The templates use `/srv/foundry/clones/project-knowledge/media-knowledge-*`
which matches the current archive layout.

---

## Systemd units update

For each of the three service unit files (`local-knowledge-documentation.service`,
`local-knowledge-projects.service`, `local-knowledge-corporate.service`):

- [ ] Add environment variable: `Environment="WIKI_KNOWLEDGE_TOML=/etc/local-knowledge/{instance}.toml"`
  substituting `documentation`, `projects`, or `corporate` for `{instance}`
- [ ] Remove obsolete lines: `Environment="WIKI_CONTENT_DIR=..."` and any `WIKI_GUIDE_DIR` lines
  (these are superseded by the knowledge.toml mount declaration)
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

Wait 3 seconds after each restart before running verification curl.

---

## Verify

```bash
# Documentation instance — check title
curl -s http://127.0.0.1:9090/ | grep "<title>"
# Expected: <title>PointSav Documentation</title>

# Projects instance — check title
curl -s http://127.0.0.1:9093/ | grep "<title>"
# Expected: <title>Woodfine Projects</title>

# Corporate instance — check title
curl -s http://127.0.0.1:9095/ | grep "<title>"
# Expected: <title>Woodfine Corporate</title>

# Search works on documentation
curl -s 'http://127.0.0.1:9090/api/search?q=substrate' | head -5
# Expected: JSON array with result objects

# Font preloads in head (L23 — exactly 2)
curl -s http://127.0.0.1:9090/ | grep 'rel="preload"' | wc -l
# Expected: 2

# editor.js NOT on article pages (L25 — route-gated)
curl -s http://127.0.0.1:9090/wiki/slm-tiered-substrate | grep 'editor.js'
# Expected: no output (empty)

# Mobile — verify on physical phone
# Open http://<vm-ip>:9090/ on phone (via SSH tunnel or direct)
# Check: no Home Indicator overlap on bottom chrome
# Press Cmd+K: verify command palette opens
# Navigate to /es/: verify navigation labels are in Spanish
```

---

## Post-deploy

- [ ] Update BRIEF §2 DNS status for documentation instance after Q3 resolved
- [ ] Route DESIGN-TOKEN-CHANGE draft to project-design for master_cosign
  (file: `.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md`)
- [ ] Update binary ledger: add entry to `data/binary-ledger/app-mediakit-knowledge.jsonl`
  with sha256 of installed binary
  ```bash
  sha256sum /usr/local/bin/app-mediakit-knowledge
  # Record SHA + install date + version in ledger
  ```
- [ ] Update BRIEF §14 Phase 9 status to "deployed: 2026-06-04" (or actual deploy date)
- [ ] Create session-end brief entry in `.agent/briefs/`

---

## All done?

Update BRIEF status field to `deployed: 2026-06-05` (or actual date) and commit a
session-end brief. Route DESIGN-TOKEN-CHANGE to project-design outbox.
