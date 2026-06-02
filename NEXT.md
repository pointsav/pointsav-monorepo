# NEXT.md — project-marketing

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.

---

## Viewport / zoom fix — 2026-06-02

- [x] **iOS Safari 30% zoom** — `documentElement.replaceWith()` dropped viewport-meta re-evaluation; iOS fell back to 980px desktop width → `max-width:1440px` layout at ~27% on 390px screens. Fix: swap `<head>` content and `<body>` separately (never replace `documentElement`). Applied to both deployed bundles 2026-06-02. Verified: `body_w == viewport_w` at 375/768/1280/1440px on both tenants.
- [ ] **Regen guard** — fix applied directly to gitignored content files; must be re-applied if bundles are regenerated from source. Source bundler JS lives in the binary build pipeline (not yet in this archive). Flag as blocker before any rebuild. [2026-06-02 totebox@claude-code]
- [ ] **Monitoring** — `/schedule` daily check pending setup (Step 5 of viewport fix plan). [2026-06-02 totebox@claude-code]

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
