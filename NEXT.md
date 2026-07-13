# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

Resolved items are moved to `.agent/rules/cleanup-log.md`, not struck in place —
this file should read as "what's actually left," not a mixed done/todo log.
Full 2026-07-09 plan-completeness audit + BRIEF consolidation: see cleanup-log.md.

---

## Hot — pick up here next session

- [ ] **foundry-prod static-asset gap — server-side fixed + verified twice; confirm operator's browser cache cleared** `[2026-07-13 totebox@claude-code]`
  - Server-side root cause found, escalated, and fixed by Command (see `.agent/rules/cleanup-log.md`
    2026-07-13 entry for full detail) — `push-to-prod.sh`'s `target_bim()` now sources static assets
    from the same place the binary comes from, closing the whole bug class, not just a one-time
    resync. Verified clean twice via independent full-browser audits (9 pages × 2 viewports ×
    2 languages, then a fresh/cache-busted re-check) — zero defects both times. Operator was still
    reporting it broken on their own device after the fix; likely explanation is browser heuristic
    caching (the 4 fixed assets send no `Cache-Control` header) — asked operator to confirm via
    hard-refresh/incognito, not yet confirmed as of shutdown. **Start here next session: check
    whether the operator confirmed it resolved.** If a genuine non-cache issue surfaces, that's a new
    investigation — the root cause here is closed.

- [ ] **SEO draft ready for bim.woodfinegroup.com — real, substantive work, not started** `[2026-07-12 totebox@project-editorial, logged 2026-07-13]`
  - project-editorial staged a ready-to-apply draft (`SEO-bim-woodfinegroup.draft.html`, their
    `.agent/drafts-outbound/`) as part of a 10-property cross-site SEO strategy
    (`BRIEF-seo-cross-site-strategy.md`). This property currently has zero of ~10 required SEO
    signals (no canonical, no OG, no Twitter, no JSON-LD, robots.txt/sitemap.xml both 404), plus a
    real bug: the meta description is hardcoded identical on every page (confirmed directly against
    `shell.rs` — `meta_desc` uses the same `t(lang, ..., ...)` call regardless of route). 3 open
    questions flagged in the draft (per-page description sourcing, sitemap URL inventory needs the
    real route table, og-image asset path) — do not silently resolve them. Deserves its own session.

- [ ] **`building-width-calculator.dtcg.json` still has unreconciled Private Office zone depths** `[2026-07-04 totebox@claude-code]`
  - A third token file, with yet another set of numbers, different from both the old wrong "confirmed"
    value and the real CAD-sourced one now correct in `key-plans.dtcg.json`. Same bug family as the fix
    made 2026-07-04 — not yet applied here. (Separate from the Academic Small / Business Width /
    End-cap fixes closed 2026-07-09 — see cleanup-log.md.)

- [ ] **"Swiss air requirement" — not found, don't fabricate** `[2026-07-04 totebox@claude-code]`
  - Operator mentioned this; searched all text-based methodology docs this session, no match. May exist in
    a PDF not yet read (check `inputs/Sketches/` more thoroughly), or may not be a real citable fact.

- [ ] **Deferred design work, re-scoped under the "Anatomy of a Key Plan" direction** `[2026-07-04 totebox@claude-code]`
  - Drafting-sheet layout system (sheet numbers, title blocks, cross-sheet references) for category pages.
  - GUID-as-visual-mark (IFC GlobalId as a recurring, owned visual signature).
  - Live constraint-composition tool — pick tokens, watch a real composition recompute. The most
    engineering-heavy of the deferred ideas; a stretch goal, not a first step.

- [ ] **3D-viewport decision gate — needs explicit operator sign-off, not a default** `[2026-07-03/04, reconfirmed still-open 2026-07-09]`
  - xeokit-sdk (AGPL-3.0, better BIM-viewer completeness + double-precision georeferencing, requires
    open-sourcing the client bundle or a paid Creoox commercial license) vs. `@thatopen/components`
    (MIT/MPL, license-safe, more integration work). Real sample IFC data already exists
    (`woodfine-bim-library`'s key-plan models); no conversion pipeline built yet either way. Feeds
    directly into `BRIEF-app-orchestration-bim.md`'s BIM Editor/Viewer scope (confirmed 2026-07-09
    as still active, just correctly sequenced behind current CMS/content work — see cleanup-log.md).

- [ ] **Search: doesn't index property-set/compliance text inside entity values** `[2026-07-03 totebox@claude-code]`
  - Only title/slug/IFC-class/top-level `$description` are indexed — a query like "fire door" can legitimately return 0 results even though both words exist in the corpus, if no single item's indexed fields contain both
  - Lower priority; noted as a known/accepted scope limit when built, not a bug

- [ ] **Verify corporate wiki instance (port 9095) CSS is fixed** `[2026-07-03 totebox@claude-code]`
  - 4 of 5 stylesheets 404'd during a same-session recheck (`projects`/`documentation` instances on 9093/9090 were fine)
  - Not project-bim's service (`app-mediakit-knowledge-2`, likely project-knowledge's territory) — flagged verbally to operator only, not yet escalated via mailbox

- [ ] **PO-1 furniture_refs data source — needs a real decision, not a routing fix** `[2026-05-28 totebox@claude-code, re-investigated 2026-07-03, re-scoped 2026-07-09]`
  - `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` PO-1 entry still has the old unstructured
    `furniture_program` string array instead of structured `furniture_refs` + `bounding_box_mm` +
    `circulation_ref` + `compliance`.
  - The original plan assumed a source file in `pointsav-design-system` that **does not exist**
    (confirmed via `find`, zero matches — `furniture_refs` only appears in `tool-keyplan`'s Rust
    generator source, never in a live JSON data file). This is not a routing fix, it's an open
    decision: **regenerate PO-1 via `tool-keyplan` (the Composition compiler), or hand-author the
    structured fields directly?** Needs an operator or engineering call before any code runs.

- [ ] **Corporate Office SVG diagrams** `[2026-05-22 totebox@claude-code]`
  - Currently `_ => {}` — no furniture; zone structure only
  - Blocked on zone depth data for Corporate Office sizes

- [ ] **Rust crate scaffold** — DTCG files are complete, this is unblocked `[2026-05-17 totebox@claude-code, unblocked 2026-07-03]`
  - 5 crates: `bim-units`, `bim-tokens`, `bim-furniture`, `tool-buildingwidth`, `tool-floorplates`
  - Full architecture in `.agent/plans/tool-buildingwidth-architecture.md`
  - This is a genuinely large undertaking (new Rust workspace, ILP solver via `good_lp`, bidirectional adjustment logic) — deliberately held back pending its own dedicated planning session, not attempted opportunistically

- [ ] **Amenity + Common Area Key Plan categories — real open design questions, not yet resolved** `[2026-07-11 totebox@claude-code]`
  - Found while auditing `inputs/Sketches/*.pdf` for a "color version" of the Key Plans: two entire
    categories exist in source material but have no confirmed numbers and are not on the site —
    "Amenities" (Tenant Lounge, Lobby Atrium, Building Manager Office, Coffee/Bread, Main Floor Retail)
    and "Common Area, Auxiliary Structures, Upper Floors" (Loading/Recycling, Public Lobby Washroom,
    Garbage Enclosure, Building Core, Corridor).
  - Real, specific unresolved design questions survive in the source, addressed to a named collaborator
    ("Brandaan"): should there be a Mop Room in Loading/Recycling? should a Service Elevator have access
    to Loading/Recycling for move-ins/move-outs? refuse bins or compactors, and how many for a
    4-floor vs. 9-floor building?
  - A short, honest forward-reference ("Amenity and Common Area categories are in active design") was
    added to `about.md`'s "Key Plans and Tiles" section 2026-07-11 — no fabricated Key Plan entries.
    Needs real operator decisions on the open questions above before either category can ship with
    real numbers.

- [ ] **V1/V2/V3 furniture-count discrepancies in source tables — not reconciled against `key-plans.dtcg.json`** `[2026-07-11 totebox@claude-code]`
  - Found alongside the Amenity/Common Area sketches: several category source tables carry multiple
    dated revisions (V1/V2/V3) with real furniture-count differences between them (e.g. Academic's
    Reception furniture is zeroed out in V3 vs. non-zero in V1 in `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key
    Plans_Academic.pdf`). Not checked category-by-category against what's actually live in
    `key-plans.dtcg.json` today — a real data-accuracy question, deferred out of Round 10's voice/diagram
    scope per operator decision. Needs a dedicated reconciliation pass.

- [ ] **WBL key-plans IFC files — 18 uncommitted modifications** `[2026-06-25 totebox@claude-code]`
  - `woodfine-bim-library/key-plans/` has 18 modified `.ifc` files (academic, business, civic, laboratory, medical, private-office × 3 variants each), confirmed still present and untouched as of 2026-07-09.
  - Detected at startup 2026-06-25; origin unknown — review diff before committing. Not a Command
    dependency — ours to review.
  - `git -C woodfine-bim-library diff key-plans/` to inspect

---

## Waiting on Command (no project-bim decision needed — visibility only)

- [ ] **`local-bim.service` unit file had wrong `BIM_DESIGN_SYSTEM_DIR` — fixed but uncommitted** `[2026-07-12 totebox@claude-code]`
  - `/srv/foundry/infrastructure/local-bim/local-bim.service` (workspace-root git, Command territory)
    pointed `BIM_DESIGN_SYSTEM_DIR` at `pointsav-monorepo` instead of `woodfine-bim-library` — silently
    zeroed out every category/token/entity on the live `local-bim.service` (port 9096) sitewide, not just
    the new `/es/*` routes. Triggered by running the documented `bootstrap.sh` self-service deploy.
    Corrected locally and service restarted (verified healthy — 72 category cards, no warnings), but
    left uncommitted since `/srv/foundry/` is Command's git scope. Flagged via outbox
    `command-20260713-local-bim-service-unit-file-bim-design-s`. Also noted: `bootstrap.sh`'s
    `systemctl enable --now` doesn't restart an already-running service — needed a manual
    `systemctl restart` to pick up a rebuilt binary.

- [ ] **`pointsav-monorepo` Cargo.lock is stale relative to Cargo.toml workspace members** `[2026-07-06 totebox@claude-code, recurred 2026-07-07/08]`
  - Working tree has an uncommitted +6035/-1453 line `Cargo.lock` diff. Root cause: the committed
    `Cargo.lock` at current HEAD (`45039f1f`) was last content-updated at old commit `82e10457`,
    well before many current workspace members (`app-console-*`, `app-mediakit-*`, etc.) existed in
    `Cargo.toml`. Workspace-wide lockfile-hygiene gap, not BIM-specific — deliberately NOT committed
    from here (a diff this size risks merge noise for other archives' pending Stage 6 promotes).
    Needs Command to decide who owns regenerating + committing this.

- [ ] **Command: admin-tier push for `woodfine-bim-library` `ee089b2`** `[2026-07-04 totebox@claude-code, updated 2026-07-06]`
  - `pointsav-monorepo` `0d72def7` half of this item is DONE — confirmed merged to canonical as
    `58fa91c0` and deployed live 2026-07-04.
  - `woodfine-bim-library` `ee089b2` (zone-depth data-bug fix, legal text rewrite, home.md content)
    is still committed locally only — remote is correctly configured but has no staging mirror, so
    it still needs Command's admin-tier push directly.

- [ ] **woodfine-bim-library has no staging-mirror remotes at all** `[2026-07-04 totebox@claude-code]`
  - Only a single `origin` (woodfine-administrator identity) — every real content/data fix here gets
    stuck waiting on Command's admin-tier push. Flagged twice now (`cleanup-log.md` 2026-05-16;
    2026-07-04). Worth Command setting up a proper staging mirror for this repo rather than a
    recurring one-off unblock.

- [ ] **Binary ledger — app-orchestration-bim v0.0.3** `[2026-05-22 totebox@claude-code]`
  - Command Session: update `data/binary-ledger/app-orchestration-bim.jsonl`
  - sha256 `/usr/local/bin/app-orchestration-bim` must match ledger entry

- [ ] **NOTAM still permission denied — fix not applied** `[2026-06-22 totebox@claude-code]`
  - `ls -la /srv/foundry/NOTAM.md` shows `-rw------- 1 mathew mathew` as of last check
  - inbox `command-20260520-notam-permission-resolved` was inaccurate; flagged to Command via outbox `project-bim-20260622-notam-permission-still-denied`
  - Command: re-apply `chmod 644 /srv/foundry/NOTAM.md`

---

## Operator-pending (blocked — do not touch)

- [ ] **DTCG accuracy errors** — 3 files pending source citations from operator `[2026-05-13 command@claude-code]`
  - `climate-zones.dtcg.json` — ASHRAE 90.1 zones + valid bSDD URIs needed
  - `performance.dtcg.json` — IFC4 Pset_DoorCommon.IsFireExit property name
  - `materials.dtcg.json` — IFC/ISO 10077 material vs. assembly thermal property distinction
  - **Do not edit without operator-confirmed citations**

- [ ] **Civic zone depths** — still synthesised; no real source exists `[2026-05-17 totebox@claude-code, reconfirmed no source available 2026-07-09]`
  - No DISCOVERY sketch or other real source document for Civic use-type zone depths. Operator
    confirmed 2026-07-09 there's no source to give yet — stays flagged/synthesised rather than
    fabricated. Needs a dedicated field-research pass, not a guess.

- [ ] **Professional Office Z2/Z3** — V12 carries TBD placeholders (3.0/3.0) `[2026-05-17 totebox@claude-code, reconfirmed still TBD 2026-07-09]`
  - No real figures available as of 2026-07-09. Confirm or specify when real data exists.

---

## Deployment

- `cluster-totebox-property-1` (per-property IFC archive) is **explicitly deprioritized for now**
  per operator decision `[2026-07-09 totebox@claude-code]` — not abandoned, just not the current
  focus while the CMS + content-accuracy work is live. See `.agent/manifest.md` tetrad block for
  the canonical status note.

---

## Deferred

- [ ] **Add `Cache-Control` headers to `app-privategit-bim`'s static assets** `[2026-07-13 totebox@claude-code]`
  - Found while investigating the foundry-prod static-asset gap (see cleanup-log.md): `bim-planroom.css`/
    `bim-layout.css`/`bim-components.css`/`bim.js` send no `Cache-Control`/`Expires` header, only
    `Last-Modified`. A browser that loads the site right before a deploy can keep serving stale
    cached CSS/JS via heuristic freshness (RFC 7234) well after the server itself is fixed, with no
    way to force a refresh short of a hard-reload. Low priority, but worth a short `max-age` or
    content-hashed filenames so future deploys don't leave this same silent tail for real visitors.

- [ ] **Spanish translation — Research essay body content ("the Journals")** `[2026-07-12, re-scoped 2026-07-13 totebox@claude-code]`
  - Round 11 deferred all of Objects/Key-Plans/Research/Search; Round 12 brought Objects, Search, and
    the full DTCG entity-description dataset into scope; Round 13 (2026-07-13) added chrome translation
    for `/es/research` + `/es/research/{slug}` (nav, breadcrumb, kicker) per operator clarification —
    the essay *content itself* is the one thing still explicitly excluded, not the whole page family.
    The 4 `research/*.md` essay titles/bodies (~3,480 words) stay English verbatim; no `.es.md` siblings
    exist or are planned unless the operator revisits this.

- [x] ~~Spanish translation Tier 3 — DTCG token `$description` fields~~ — **done 2026-07-13.** 221
  `$description_es` fields added across 22 `tokens/bim/*.json` files (3 parallel drafting passes).
  `key-plans.dtcg.json` and `amenity-key-plan.dtcg.json` deliberately excluded (Key Plans stay
  English-only); `building-width-calculator.dtcg.json`'s `bim.key-plan.*` subtree was translated then
  reverted for the same reason — see cleanup-log.md for the full boundary-case note. `design_notes`
  fields (a different, non-rendered field) were correctly left untouched by the drafting passes.

- [x] ~~Spanish translation — Key Plans page chrome~~ — **done 2026-07-13 (Round 13).**
  `/es/key-plans`, `/es/key-plans/{slug}`, `/es/key-plans/{slug}/o/{object}` all live — nav, footer,
  breadcrumbs, section headings ("Zone allocation"→"Asignación de zonas", "Parts list"→"Lista de
  componentes", bill-of-materials linked/unlinked status labels) translate. Fixed a real dead-end bug
  in the process: clicking Key Plans from the Spanish nav used to strand the visitor on an all-English
  page with the language switch entirely absent.
  - **Still and permanently out of scope:** the Key Plan/Composition data itself — dimensions, bill-of-
    materials item names, `$description` prose — per explicit operator decision. Not a "not yet gotten
    to" item.

- [ ] **Spanish translation — SVG diagram inner labels on /es/method** `[2026-07-12 totebox@claude-code]`
  - The two Method-page diagrams' `<figcaption>`s are fully translated (Round 11 Tier 1), but the labels
    drawn *inside* the SVGs themselves (Building, FACADE, DAYLIGHT PERIMETER, etc. — `render/svg.rs`'s
    `render_containment_model_svg()`/`render_method_zone_svg()`) stay English on `/es/method`, since only
    the figcaptions were in this round's scope. Confirmed via browser check 2026-07-12 — visually fine,
    not a bug, but a real gap if full diagram translation is ever wanted. Would need `lang` threaded into
    `render/svg.rs`, not attempted this round.

- [x] ~~Spanish translation — Objects/Search pages~~ — **done 2026-07-13 (Round 12).** Originally
  scoped out of Round 11 as "data-heavy, no lede/prose equivalent"; operator asked to bring them into
  scope. `/es/objects`, `/es/objects/{slug}`, `/es/objects/compare`, `/es/search` all live — chrome
  translated, entity descriptions pull from the new `$description_es` DTCG field. Key Plans and
  Research remain the only two excluded page families (see above).
