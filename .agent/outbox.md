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
re: housekeeping batch — corporate + projects wiki (2026-05-20)
created: 2026-05-20T17:45:00Z
priority: normal
status: pending
---

Two housekeeping commits:

| Commit | Repo | Author | What |
|---|---|---|---|
| `ebc2939` | content-wiki-corporate | Peter | YAML expansion (15-topic rotation + 9 DYK facts), about.md scope, NEXT.md, .agent/rules bootstrap |
| `bffe4e3` | content-wiki-projects | Jennifer | NEXT.md current-state update + CLAUDE.md created |

content-wiki-corporate is now 10 commits ahead of origin/main.
content-wiki-projects is now 6 commits ahead of origin/main.
Both content-only; no binary rebuild required for these batches.

---
from: totebox@project-knowledge
to: command@claude-code
re: G2/PJ2/C8-C10 complete — 3 new commits across 3 sub-clones; G2 canonical removal pending
created: 2026-05-19T17:30:00Z
priority: normal
status: pending
---

Three parallel work items committed 2026-05-19:

| Commit | Repo | Author | What |
|---|---|---|---|
| `a06f64f` | woodfine-fleet-deployment | Peter | G2: remove README-TOTEBOX-EGRESS.md from cluster-clone |
| `b138b99` | content-wiki-projects | Jennifer | PJ2: expand 5 country co-location index stubs (Italy, Mexico, Nordics, Poland, Spain) + fix ES frontmatter bug; 10 files |
| `cb53200` | content-wiki-corporate | Peter | C8-C10: 10 new corporate wiki topics + 10 ES bilingual pairs (20 files) |

**G2 follow-up required (Command Session):** The cluster-clone removal is done, but `guide_dir_2` for `local-knowledge-documentation.service` points to `/srv/foundry/customer/woodfine-fleet-deployment/` (canonical). That path still has `README-TOTEBOX-EGRESS.md`. Canonical removal requires admin-tier commit to `woodfine/woodfine-fleet-deployment`. Action:
```bash
cd /srv/foundry/customer/woodfine-fleet-deployment
git rm README-TOTEBOX-EGRESS.md
# then admin-tier commit + push
```

**Stage 6 for content-wiki-projects and content-wiki-corporate:** Both are now ahead of origin/main. Content-only changes; no binary rebuild required. See existing promote messages below for content-wiki-documentation.

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-documentation — 4 commits ahead; D3 + D6 complete (2026-05-19)
created: 2026-05-19T14:00:00Z
priority: normal
status: pending
---

Supersedes earlier documentation 3-commit message. Now 4 commits ahead of origin/main.

**content-wiki-documentation — 4 commits ahead of origin/main:**

| Commit | Author | What |
|---|---|---|
| `a07bdf5` | Peter | D6: governance category complete — 10 files, 4 articles rewritten/elevated, _index expanded |
| `cf72e67` | Jennifer | D3: substrate + patterns _index MOC expanded (7→32 and 3→10 articles, EN+ES) |
| `c8192fc` | Jennifer | D5: `short_description` added to all 162 EN+ES documentation wiki articles |
| `1d92e7c` | Peter | NEXT.md — D-items sprint close + open items update |

**content-wiki-projects — 4 commits ahead of origin/main (unchanged).**

All content-only. Stage 6 for both repos can proceed independently.

**Plan status:** D3, D5, D6, PJ1, PJ3, PJ4, PJ5, PJ6, PJ7, PJ8 — all complete.
Open: D10 (wikilink validation, blocked on Stage 6 binary rebuild), PJ2 (country index stubs — needs real data).

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-projects — 2 new commits (PJ3 + PJ7 fixes, 2026-05-19)
created: 2026-05-19T00:00:00Z
priority: normal
status: pending
---

`content-wiki-projects` main is now 2 commits ahead of `origin/main`:

| Commit | Author | What |
|---|---|---|
| `2ec3a8f` | Jennifer | PJ3: `short_description` added to all 26 remaining EN+ES articles |
| `78db55b` | Peter | PJ7: `leapfrog-facts.yaml` `link_slug` prefix fix — all 7 entries missing `topic-` prefix corrected |

Content-only changes. Promote via `bin/promote.sh` from within `~/Foundry/clones/project-knowledge/content-wiki-projects/` (or the registered staging path).

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-documentation — 2 new commits (D-items sprint, 2026-05-18)
created: 2026-05-18T12:00:00Z
priority: normal
status: pending
---

`content-wiki-documentation` main is 2 commits ahead of `origin/main`:

| Commit | Author | What |
|---|---|---|
| `c8192fc` | Jennifer | D5: `short_description` added to all 162 EN+ES documentation wiki articles |
| `1d92e7c` | Peter | NEXT.md — D-items sprint close + open items update |

Both are content-only (no engine changes). Stage 6 for this repo can proceed independently of the monorepo Stage 6 below. Promote via `bin/promote.sh` from within `~/Foundry/clones/project-knowledge/content-wiki-documentation/` (or whichever the registered staging path is).

No binary rebuild required for this batch — content changes are picked up on the next wiki server restart or immediately from git if the engine reads from the working directory.

---
from: totebox@project-knowledge
to: command@claude-code
re: Stage 6 + binary rebuild — Sprints R through AE (16 commits)
created: 2026-05-18T00:00:00Z
priority: normal
status: pending
---

Cluster `project-knowledge` monorepo sub-clone is 16 commits ahead of
`origin/main`. All commits are on the `main` branch of
`~/Foundry/clones/project-knowledge/pointsav-monorepo/`.

**Sprints in this batch:**

| Sprint | Commit | Author | What |
|---|---|---|---|
| R | `3351c1f2` | Jennifer | Institutional quality — trademark/copyright, TOC fix, Woodfine theme |
| S | `5294f8e8` | Peter | Home-page chrome blending — mono-uppercase, IP footer, lede border |
| S.2 | `72a327b0` | Jennifer | Trademark text, lede :first-of-type fix, border-radius 2px sweep |
| T+U | `6453a7a9` | Jennifer | Print stylesheet + `--accent` token |
| V | `45f2985b` | Peter | Search CSS + blockquote/pre/table quality |
| W | `8ec12687` | Jennifer | History/blame inline-style purge + cite CSS |
| X | `f2cedd69` | Peter | Error pages, pageinfo, whatlinkshere CSS |
| Y | `e19e462b` | Peter | Semantic color tokens + full dark-mode variable migration |
| Z | `4fadfa3f` | Jennifer | Typography pass |
| AA | `99938103` | Peter | Focus-visible ring, skip-to-content, sticky animation |
| AB | `b28396ce` | Peter | Mobile polish: WCAG 2.5.5 44px touch targets, drawer slide-in animation, trapFocus, focus management; 8 new tests |
| AC | `35f787e3` | Jennifer | Infobox title/image support, main hatnote fenced block, r#unsafe=true fix; 7 new tests |
| AD | `dc0d3af3` | Peter | Engine P0 bug-fix: AGENT.md system-file filter, .git dir walk skip, per-article `<title>` tag |
| AD.2 | `3514904e` | Jennifer | Engine P0-C: wire bare-slug resolver call site in wiki_page — 301 redirects fix 280+ broken wikilinks |
| AE | `ecd6b74a` | Jennifer | Engine P0-E/F: tagline from site_title, search index excludes system/hidden files + test |

**Action required from Master:**
1. `echo "y" | ~/Foundry/bin/promote.sh` — promote cluster branch to canonical main
2. `cd ~/Foundry/vendor/pointsav-monorepo/app-mediakit-knowledge && cargo build --release`
3. `sudo cp target/release/app-mediakit-knowledge /usr/local/bin/app-mediakit-knowledge`
4. `sudo systemctl restart local-knowledge-documentation local-knowledge-projects local-knowledge-corporate`
5. Verify all 3 services healthy: `curl -s http://localhost:9090/healthz && curl -s http://localhost:9093/healthz && curl -s http://localhost:9095/healthz`

Tests: 206+ passing through AD.2; AE adds system-file search exclusion test. No operator decision gates — can proceed immediately.
