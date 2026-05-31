---
schema: foundry-draft-v1
draft_id: guide-deploy-bim-substrate
language_protocol: PROSE-GUIDE
state: ready-for-sweep
target_path: customer/woodfine-fleet-deployment/gateway-orchestration-bim/guide-deploy-bim-substrate.md
created: 2026-05-06T19:10:00Z
author: task@project-bim
cites: [axum-0-7, nginx, certbot, systemd]
research_done_count: 2
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Plan Part 5 (woodfine-design-bim + app-orchestration-bim config): /home/mathew/.claude/plans/1-we-need-to-frolicking-taco.md
  App structure: pointsav-monorepo/apps/app-orchestration-bim/ (main.rs, vault.rs)
research_inline: false
---

# Guide: Deploying the BIM Token Substrate

This guide covers setting up a sovereign BIM token vault (`woodfine-design-bim`) and deploying `app-orchestration-bim` to serve it at a public URL. All commands run from the deployment host as a user with `sudo` access and `systemd` unit permissions.

All paths below are relative to the GitHub repository root where appropriate, or absolute on the deployment host.

---

## Part 1 — Provision the woodfine-design-bim Token Vault

The BIM token vault is a GitHub repository in the `woodfine` org. It holds all DTCG token files, regulatory overlays, and climate zone data for the deployment.

### Prerequisites

- GitHub account with write access to `woodfine` org (`mcorp-administrator` identity)
- `git` configured with SSH key for `mcorp-administrator`

### Steps

1. Create the repository on GitHub (one-time, Master action via mcorp-administrator):
   ```
   Repository: woodfine/woodfine-design-bim
   Visibility: Private (default)
   License: EUPL-1.2
   ```

2. Clone to the deployment host:
   ```bash
   git clone git@github.com-mcorp:woodfine/woodfine-design-bim.git \
       /opt/foundry/vaults/woodfine-design-bim
   ```

3. Verify the token directory is populated:
   ```bash
   ls /opt/foundry/vaults/woodfine-design-bim/tokens/bim/
   # Expected: spatial.dtcg.json  elements.dtcg.json  systems.dtcg.json
   #           materials.dtcg.json  assemblies.dtcg.json  performance.dtcg.json
   #           identity-codes.dtcg.json  relationships.dtcg.json  climate-zones.dtcg.json
   ```

---

## Part 2 — Configure app-orchestration-bim

`app-orchestration-bim` reads its token vault path from an environment variable at startup.

### Environment variable

```
BIM_DESIGN_SYSTEM_DIR=/opt/foundry/vaults/woodfine-design-bim
```

This variable tells the vault loader in `vault.rs` where to find the `tokens/bim/`, `components/bim-*/`, and `research/bim-*.md` directories.

### Systemd unit configuration

The service unit file is at:
```
pointsav-monorepo/infrastructure/app-orchestration-bim.service
```

Set the environment variable in the unit's `[Service]` section:

```ini
[Service]
Environment=BIM_DESIGN_SYSTEM_DIR=/opt/foundry/vaults/woodfine-design-bim
Environment=PORT=9096
ExecStart=/opt/foundry/bin/app-orchestration-bim
User=foundry
Restart=on-failure
```

After editing the unit file:
```bash
sudo systemctl daemon-reload
sudo systemctl enable app-orchestration-bim
sudo systemctl start app-orchestration-bim
```

Verify the service is running:
```bash
systemctl status app-orchestration-bim
curl http://127.0.0.1:9096/readyz
# Expected: {"status":"ok"}
```

---

## Part 3 — nginx Vhost Setup

`app-orchestration-bim` listens on `127.0.0.1:9096`. nginx proxies public traffic to it.

### nginx site configuration

Create `/etc/nginx/sites-available/bim.woodfinegroup.com`:

```nginx
server {
    listen 80;
    server_name bim.woodfinegroup.com;

    location / {
        proxy_pass http://127.0.0.1:9096;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/bim.woodfinegroup.com \
           /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Part 4 — TLS via Certbot

Issue a TLS certificate for `bim.woodfinegroup.com` using certbot with the nginx plugin:

```bash
sudo certbot --nginx -d bim.woodfinegroup.com \
    --non-interactive --agree-tos \
    --email open.source@pointsav.com
```

Certbot modifies the nginx site configuration to add TLS. Verify the updated configuration:

```bash
sudo nginx -t && sudo systemctl reload nginx
```

Certbot auto-renewal is configured by the certbot systemd timer installed at `/etc/systemd/system/certbot.timer`. Verify:
```bash
systemctl status certbot.timer
```

---

## Part 5 — DNS

The `bim.woodfinegroup.com` DNS A record must point to the deployment host's public IP.
This is a workspace-tier operation (Master session) using the DNS management console for
the `woodfinegroup.com` domain. Confirm the record is active:

```bash
dig +short bim.woodfinegroup.com
# Expected: <public-IP-of-deployment-host>
```

---

## Part 6 — Smoke Test

After DNS resolves and TLS is active, run the full smoke test:

```bash
# 1. Health check
curl -s https://bim.woodfinegroup.com/readyz
# Expected: {"status":"ok"}

# 2. Token catalog loads
curl -s https://bim.woodfinegroup.com/tokens | grep -c "bim-token-card"
# Expected: 8 (one per token category)

# 3. Machine surface
curl -s https://bim.woodfinegroup.com/tokens.json | jq '.["elements.IfcWall"] | .ifc_class'
# Expected: "IfcWall" (or similar key from elements.dtcg.json)

# 4. API endpoint
curl -s https://bim.woodfinegroup.com/api/climate-zones | jq 'keys | length'
# Expected: number of registered climate zone tokens
```

When all four checks pass, update `woodfine-fleet-deployment/gateway-orchestration-bim/MANIFEST.md`:
```yaml
deployments:
  - id: gateway-orchestration-bim-1
    status: active
    url: https://bim.woodfinegroup.com
    smoke_check: pass
    smoke_check_date: <YYYY-MM-DD>
```

---

## Open Questions

1. The `app-orchestration-bim` binary path (`/opt/foundry/bin/`) assumes a specific deployment
   layout. The systemd unit file in the repo should be reviewed against the actual deployment
   host layout before first deploy.
