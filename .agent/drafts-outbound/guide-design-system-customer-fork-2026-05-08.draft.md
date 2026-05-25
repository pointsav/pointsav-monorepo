---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-customer-fork.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 2
open_questions_count: 1
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/developing/customer-fork/.
  BLOCKED pending license decision: this guide cannot be published until the
  license governing the pointsav-design-system vault fork is decided by master
  (MIT, Apache 2.0, or proprietary). See outbox message 2026-05-08 and the
  license placeholder section below. Do not remove the placeholder — leave it
  in place until the decision is recorded and the guide is updated.
  Language pass: Bloomberg standard. All environment variable names are uppercase
  and literal (DESIGN_VAULT_DIR etc.) — do not change capitalization.
  BCSC: "planned/intended" language for any forward-looking bearer token auth.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub developing/customer-fork.md] — skeleton content
- [tacit: app-privategit-design CLAUDE.md] — DESIGN_VAULT_DIR, DESIGN_TENANT,
  DESIGN_BIND env vars confirmed; single-tenant per process stated explicitly
- [tacit: main.rs Axum router] — substrate is a single binary, stateless

### Suggested — what project-editorial should consult
- [infrastructure/ systemd unit] — locate the exact systemd unit filename and
  ExecStart line for the deployment guide (high priority; guide references it)
- [pointsav-design-system vault/themes/] — verify the theme file format so the
  "edit themes" step is accurate (medium priority)

### Open questions — for future passes
- What license governs the forked vault? Guide cannot be published without this.
  → master decision required (see outbox 2026-05-08)

---

# Customer Fork Procedure

The PointSav Design System substrate is designed to be forked. When you fork, you
own the complete system: the token files, component recipes, research files, themes,
Git history, and the binary that serves it. There is no SaaS dependency and no
central authority over your instance.

---

## What you own after forking

- **A Git repository** — your vault, in your account, with your history. You commit
  changes directly; there is no pull request queue to a vendor platform.
- **A binary** — a single stateless binary that reads your vault from disk and
  serves it over HTTP. It runs as a systemd unit on your server.
- **A domain** — the binary binds to a local address; your nginx vhost proxies it.
  The domain is yours.
- **No recurring dependency** — the substrate does not phone home. Upgrades are
  voluntary; you control when and whether to pull vendor changes.

---

## Fork steps

### 1. Fork the repository on GitHub

Fork `pointsav/pointsav-design-system` to your GitHub account or organisation.
This creates your private copy of the vault directory structure.

### 2. Clone your fork to your server

```bash
git clone git@github.com:<your-org>/pointsav-design-system.git \
    /srv/vault
```

Replace `/srv/vault` with the path you want to use for your vault directory.
This path becomes the value of `DESIGN_VAULT_DIR`.

### 3. Edit your theme

Your brand's design decisions live in `vault/themes/<your-brand>.json`. This file
provides semantic token overrides that alias into the primitive layer:

```json
{
  "semantic": {
    "interactive": {
      "primary": { "$value": "{color.primary-60}", "$type": "color" }
    }
  }
}
```

To change the primary interactive colour, update the `$value` reference to point
at a different primitive, or define a new primitive in `vault/tokens/primitive.json`
and reference it here. The substrate resolves the alias chain at startup.

Name your theme file to match your tenant identifier
(`acme-brand.json` → `DESIGN_TENANT=acme-brand`).

### 4. Run the substrate binary

```bash
DESIGN_VAULT_DIR=/srv/vault \
DESIGN_TENANT=acme-brand \
DESIGN_BIND=127.0.0.1:9094 \
/usr/local/bin/app-privategit-design
```

In production, run this as a systemd unit. The unit file is at
`infrastructure/local-design.service` in this repository. Copy it to
`/etc/systemd/system/`, edit the environment variables, and enable it:

```bash
sudo systemctl daemon-reload
sudo systemctl enable local-design
sudo systemctl start local-design
```

### 5. Configure nginx and certbot

```nginx
server {
    server_name design.example.com;
    location / {
        proxy_pass http://127.0.0.1:9094;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

Obtain a certificate:

```bash
sudo certbot --nginx -d design.example.com
```

---

## Environment variables

| Variable | Purpose | Example |
|---|---|---|
| `DESIGN_VAULT_DIR` | Absolute path to your vault directory | `/srv/vault` |
| `DESIGN_TENANT` | Tenant identifier; matches `themes/<tenant>.json` | `acme-brand` |
| `DESIGN_BIND` | Address and port the binary binds to | `127.0.0.1:9094` |

The substrate is single-tenant per process. To serve multiple brands, run
multiple instances on different ports, each pointing at its own vault directory.

---

## Verifying your deployment

```bash
curl -sS http://127.0.0.1:9094/healthz | jq .
# {"status":"ok","service":"app-privategit-design","version":"0.1.0"}

curl -sS http://127.0.0.1:9094/readyz | jq .
# {"status":"ready"} when tokens and components are loaded
```

---

## License

**⚠ Placeholder — master decision pending.**

The license governing the forked vault is not yet determined. This guide will be
updated with the applicable license terms (MIT, Apache 2.0, or other) once the
vendor makes the decision. Do not publish this guide until this section is complete.
