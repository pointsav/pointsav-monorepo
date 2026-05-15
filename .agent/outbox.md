---
mailbox: outbox
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-knowledge cluster

---
from: totebox@project-knowledge
to: command@claude-code
re: Sprint M shipped — binary rebuild required (all 3 wiki services)
created: 2026-05-15T00:00:00Z
priority: high
status: pending
msg-id: project-knowledge-20260515-sprint-m-rebuild
---

Sprint M committed at `c7958a68` (Peter Woodfine) on cluster branch `main`.

All 170 tests pass. `cargo check` clean.

**Engine changes in this commit:**
- Four-box Wikipedia-style home page (TFA / DYK / ITN / OTD)
- Coloured box headers: `--mw-color-box-tfa` (#cef2e0 green), `--mw-color-box-dyk` (#cedff2 blue)
- Welcome banner with comma-formatted article count and trailing comma
- Sister surfaces 3×3 grid, language tier footer
- `humanize_category()` replaces old `capitalise()` (fixes "Design-system" → "Design System")
- `RATIFIED_CATEGORIES` already has `substrate` + `patterns` from a prior session
- Anti-FOUT inline script; `wiki.js` now loaded on home page

**Please execute after Stage-6 promote of cluster branch:**

```
cd ~/Foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

**Verification after restart:**
- Home page at `documentation.pointsav.com` shows four coloured boxes, not the old two-panel layout
- Category headings show "Design System" not "Design-system"
- `substrate` and `patterns` categories visible in Browse by area (no catch-all needed)
- Sister surfaces grid visible below category section

Note: Stage-6 promotion of monorepo cluster branch to `pointsav/pointsav-monorepo` main is also pending (Sprint L + Sprint M are both on cluster `main`, 5 commits ahead of origin).

