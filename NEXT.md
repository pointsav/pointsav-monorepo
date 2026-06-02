# NEXT.md — project-marketing

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.

---

## Viewport / zoom fix — 2026-06-02

- [x] **iOS Safari 30% zoom** — `documentElement.replaceWith()` dropped viewport-meta re-evaluation; iOS fell back to 980px desktop width → `max-width:1440px` layout at ~27% on 390px screens. Fix: swap `<head>` content and `<body>` separately (never replace `documentElement`). Applied to both deployed bundles 2026-06-02. Verified: `body_w == viewport_w` at 375/768/1280/1440px on both tenants.
- [x] **Regen guard** — `scripts/fix-viewport.sh` created 2026-06-02; idempotent patch script re-applies the body-only swap to both deployment `index.html` files after any bundle rebuild. Run `bash scripts/fix-viewport.sh` before restarting services. Detects already-patched files safely. [2026-06-02 totebox@claude-code]
- [x] **Monitoring** — daily remote agent `trig_01P7iwnuwpPShgaivbg4m2gq` created 2026-06-02; fires 07:00 UTC; checks HTTP 200 + viewport meta + no replaceWith regression on both tenants. Dashboard: https://claude.ai/code/routines/trig_01P7iwnuwpPShgaivbg4m2gq [2026-06-02 totebox@claude-code]

---

## SEO — brand search experience

- [ ] Apply SEO head block to `home.pointsav.com` index [2026-05-20 totebox@claude-code]
      Plan: `.agent/plans/seo-home-pages.md`
      File: `/srv/foundry/deployments/media-marketing-landing-2/content/index.html`
- [ ] Apply SEO head block to `home.woodfinegroup.com` index [2026-05-20 totebox@claude-code]
      Plan: `.agent/plans/seo-home-pages.md`
      File: `/srv/foundry/deployments/media-marketing-landing-1/content/index.html`
- [ ] Confirm: any LinkedIn or social profiles to add to `sameAs` in JSON-LD? [2026-05-20 operator-pending]
- [ ] Decide scope: also update disclaimer/contact subpages + software.pointsav.com pages in same pass? [2026-05-20 operator-pending]
- [ ] Follow-up (after home pages done): add `robots.txt` + `sitemap.xml` to both sites [2026-05-20 totebox@claude-code]

---

## Deployment (pre-existing)

- [ ] On next bundle rebuild for `home.pointsav.com`: fix `<title>` to read `PointSav — Home` (not `PointSav, Inc. — Home`) — direct fix applied to live `deployments/media-marketing-landing-2/content/index.html` 2026-05-21 but gitignored; source fix needed at build time [2026-05-21 totebox@claude-code]
- [ ] Bootstrap deploy: `media-marketing-landing-1` (home.woodfinegroup.com) — operator-gated
- [ ] Bootstrap deploy: `media-marketing-landing-2` (home.pointsav.com) — operator-gated
- [ ] Certbot TLS for both deployments — operator-gated
