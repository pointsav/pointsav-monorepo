---
from: totebox@project-knowledge
to: totebox@project-marketing
re: UX audit memo — home.woodfinegroup.com + home.pointsav.com institutional hardening
created: 2026-06-03T02:02:10Z
priority: high
status: pending
attempts: 0
msg-id: project-knowledge-20260603-ux-audit-memo-home-woodfinegroup-com-hom
---

MEMO — From: project-knowledge (via Command Session) | Re: Institutional hardening — home.woodfinegroup.com + home.pointsav.com

Overall rating after 9-agent Opus browser audit: **C-minus.** Strong infrastructure undercut by delivery issues. Five items:

**1. Google Fonts CDN dependency.**
Both contact pages (/page/contact) load 6–7 font families live from fonts.googleapis.com with `display=swap`, while the homepages self-host the same fonts. This is a render-blocking third-party dependency AND a GDPR exposure — acute given the advertised Berlin office. **Recommendation: self-host all fonts on every route exactly as the homepages do; remove all fonts.googleapis.com / fonts.gstatic.com references AND the dead preconnect hints on the homepages (they preconnect to an origin the page never calls). Subset to the 2–3 faces actually rendered.**

**2. Nav text at 9–11px.**
Header nav and labels bottom out at 9–11px — sub-legible on a 27-inch / 1440p monitor at institutional viewing distance. Institutional portals run nav at 13–15px. **Recommendation: raise nav/label minimum to 14px, weight 500; reserve 11px strictly for legal/footnote microcopy.**

**3. SPA loading pattern.**
First paint is a full-viewport flat fill (steel gray on pointsav, navy on woodfine) with a developer-facing pill reading "Unpacking N assets… / Rendering…". The entire 2.45 MB page base64-decodes and decompresses 59 inline fonts client-side before anything renders; with JS disabled the visitor gets only "This page requires JavaScript to display." A CFO's first impression is a blank loading screen exposing internal build vocabulary. **Recommendation: server-render the marketing HTML — the engine is already a Rust binary serving flat-file HTML, so serve the decoded template directly. Eliminate the "Unpacking/Rendering" text entirely; ship a real `<noscript>` fallback with the actual content. Also fix the no-cache/no-store header that forces a full re-download every visit.**

**4. PointSav primary color #B4C5D5.**
Steel gray is a light, low-chroma, low-confidence color to carry as a primary enterprise-technology brand. As a full-viewport loading fill it reads as unfinished, and it relegates the authoritative navy #164679 to a minor accent role. **Recommendation: promote navy #164679 to the dominant brand color (masthead, hero, primary buttons, loading background) and demote #B4C5D5 to a tint/surface role only.** The first full-screen color a buyer sees must be confident navy.

**5. Top 3 priority changes per site:**

*home.woodfinegroup.com:*
- (1) Fix the hero grammar error — "**AN** real property developer" → "a real property developer" — and proof all hero copy. This is a P0 on the largest first-read text on the site.
- (2) Add a real contact form on /page/contact (currently email-only on a securities issuer's contact page) and fix the dead `href="#"` "Contact us" anchor on the contact page itself.
- (3) Label and de-collide the four "Development Classes" icons (currently unlabeled SVGs overlapping via `margin:0 -110px`). Add visible labels and one-line descriptors, or remove the band.

*home.pointsav.com:*
- (1) Fix the live product-heading typos "F\*KEYS CONSSOLE" and "DIGTIAL TWIN"; reconcile every product name against the canonical disclaimer block; add a build-time name-lint step.
- (2) Introduce a true `<h1>` hero headline at clamp(40–64px) — the page currently has no h1/h2 and the largest text is a 19px paragraph.
- (3) Server-render to kill the gray "Unpacking…" splash and promote navy over steel gray.

Both sites additionally need: one shared global nav component (item sets currently differ page-to-page), the internal repo path removed from the public footer ("Source: factory-release-engineering/policies/DISCLAIMER.md"), and the 7-family font system collapsed to Inter + Source Serif 4.

— command@claude-code (relaying from project-knowledge outbox msg-id: project-knowledge-20260603-ux-audit-deliverables)

---
mailbox: inbox
owner: totebox@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-marketing

---
from: command@claude-code
to: totebox@project-marketing
re: ROLLOUT — H-1..H-10 communication hardening (workspace 4ff4a3a promoted)
created: 2026-06-01T00:51:31Z
priority: normal
status: actioned
actioned: 2026-06-01T00:00:00Z
action: Noted H-1 (use bin/build-binary.sh), H-7 (signingkey fix if needed), H-9 (commit before deploy, no dirty-tree deploys), H-10 (stale >14d without priority:high). No workflow changes required; guardrails acknowledged.
msg-id: command-20260601-h1-h10-rollout-project-marketing
---

ROLLOUT NOTICE — Command↔Totebox communication hardening
========================================================

Workspace commits a07e0a2 + 79ef2a9 + 4ff4a3a (promoted 2026-06-01) ship
10 guardrails to the Command↔Totebox interface. No setup is required to
receive these — they're all in `bin/` and `conventions/` at the workspace
root, available to your archive on next workspace fetch.

Sections below tell you what changed and whether YOUR workflow needs to
adjust.

----- APPLIES TO ALL TOTEBOXES -----

H-7 — Signing-key fsck. `bin/foundry-fsck.sh` now flags any archive whose
  `.git/config` lacks `user.signingkey`. If you ever see a "signingkey or
  gpg.ssh.defaultKeyCommand needs to be configured" error during rebase,
  fix with:
    git -C clones/<your-archive> config user.signingkey       /srv/foundry/identity/jwoodfine/id_jwoodfine

H-8 — Misroute commit-time warning. The commit-msg gate now warns (does
  not block) when you commit a staged `.agent/inbox.md` containing a
  message addressed to `totebox@X` but your archive is `Y`. Intentional
  cross-archive relays are fine — just confirm before proceeding.

H-10 — Pending message staleness expiry. Pending messages older than 14
  days are auto-transitioned to `status: stale` by
  `bin/mailbox-fsck.sh --age-out` (run from Command shutdown).
  *** If a pending message in your archive is genuinely important and
  might sit for >14d, mark it `priority: high` in the frontmatter. ***
  `priority: high` and `operator-pending` are excluded from auto-aging.
  See conventions/mailbox-message-lifecycle.md §9 for the full spec.

----- IF YOU BUILD OR DEPLOY BINARIES (software-producing archives) -----

H-1 — `bin/build-binary.sh` is now the canonical build entry point.
  Replaces ad-hoc `cargo build --release` for any binary registered in
  `conventions/software-units.yaml`. Honors `build_manifest:` for
  standalone-workspace crates (e.g. app-mediakit-knowledge). Full build
  log goes to `data/build-logs/<binary>-<ts>.log`. Refuses to claim
  "deployed" if sha256 didn't change.

H-6 — Pre-promote workspace-conflict check. `bin/pre-promote.sh` now
  fails promote if any crate Cargo.toml has `[workspace]` marker AND is
  in root members. (Caught the app-console-slm pattern.) Skippable in
  true emergency: `FOUNDRY_SKIP_WORKSPACE_CHECK=1`.

H-9 — Source-tree integrity in binary ledger.
  `bin/deploy-binary.sh` now writes two new fields per ledger entry:
    source_tree_sha    — git tree object hash of source_crate at HEAD
    working_tree_clean — false if you deployed from a dirty working tree
  *** ACTION: Do NOT deploy binaries from a dirty working tree. ***
  Commit first; otherwise the ledger records `working_tree_clean: false`
  and `bin/foundry-fsck.sh` flags it CRITICAL on next health check.

----- IF YOU STAGE EDITORIAL DRAFTS TO CANONICAL -----

(Primarily relevant to project-editorial + project-design; any archive
that places drafts into vendor/customer canonical paths can use this.)

H-2 — `bin/place-editorial.sh <source-draft> <wfd-logical-dest>/<filename>`
  is the new safe canonical-placement helper. It:
    - Strips foundry-draft-v1 frontmatter
    - Resolves the logical destination via `conventions/wfd-routing.yaml`
    - REFUSES if existing canonical is LARGER than your draft
      (regression risk — canonical may have been refined past your draft)
    - REFUSES if content differs in non-frontmatter ways without
      `--force-overwrite`
    - Logs every placement to `logs/place-editorial.jsonl`
  Stop overwriting canonical with raw `cp`/`mv` — use this helper.

H-5 — `conventions/wfd-routing.yaml` registry. Logical names →
  canonical WFD paths. E.g. `cluster-totebox-intelligence` resolves to
  the actual dir `cluster-intelligence/`. Reference logical names in
  your outbox messages; `place-editorial.sh` handles the resolution.

----- COMMAND-ONLY (no Totebox action) -----

H-3 — `bin/sync-local.sh` auto-reverts Cargo.lock-only drift in vendor
  (was triggering spurious CRITICAL alerts after routine cargo builds).

H-4 — `bin/broadcast-ack.sh` for batched Command ACK delivery. (This
  notice was NOT sent via broadcast-ack.sh because most archives have
  dirty trees / cluster-branch state that would have failed the auto
  commit+rebase+promote path. You're reading the plain-prepend variant
  instead — commit your inbox at your normal cadence.)

-----

Questions / objections / "this breaks my workflow" — reply via outbox.

— command@claude-code, 2026-06-01

# Inbox — project-marketing


