---
schema: foundry-draft-v1
type: GUIDE-deployment-marketing-site
language_protocol: PROSE-GUIDE
title: "Deploy and update app-mediakit-marketing on a running instance"
target_project: project-editorial
target_destination: customer/woodfine-fleet-deployment/media-marketing-landing/ | vendor/pointsav-fleet-deployment/media-marketing-landing/
created: 2026-06-01T00:00:00Z
author: totebox@project-marketing (claude-code / claude-sonnet-4-6)
state: language-cleared
language_pass_date: 2026-06-03
language_pass_by: totebox@project-editorial
research_trail:
  source_commits:
    - b49b4ae (cluster/project-marketing — ops shutdown)
    - 8da56fc (session-context 2026-05-24 — binary-targets declared, SEO applied)
  prior_drafts: []
  citations:
    - conventions/compounding-substrate.md
    - conventions/worm-ledger-design.md
    - clones/project-marketing/.agent/binary-targets.yaml
  operator_inputs:
    - "Binary at /usr/local/bin/app-mediakit-marketing, run as local-marketing user"
    - "Units: local-marketing.service (woodfine/9102), local-marketing-pointsav.service (pointsav/9101)"
    - "H-9 guardrail: deploy only from clean working tree; commit source before deploying"
    - "H-1: canonical build entry point is bin/build-binary.sh"
  related_files:
    - .agent/binary-targets.yaml
    - deployments/media-marketing-landing-1/MANIFEST.md
    - deployments/media-marketing-landing-2/MANIFEST.md
    - /etc/systemd/system/local-marketing.service
    - /etc/systemd/system/local-marketing-pointsav.service
---

# Guide: Deploy and update app-mediakit-marketing on a running instance

Operational runbook for deploying a new build of `app-mediakit-marketing` to an existing instance. Covers zero-downtime binary swap, content updates, rollback, and health verification.

For provisioning a new instance from scratch, see `guide-provision-marketing-site`.

---

## Pre-deployment checklist

Before deploying:

1. **Source committed.** The source commit for the build must be on a promoted canonical branch. H-9 guardrail: if the working tree is dirty at build time, the binary ledger records `working_tree_clean: false` and `bin/foundry-fsck.sh` flags CRITICAL on the next health check. Commit first.

2. **Binary built.** Use `bin/build-binary.sh app-mediakit-marketing` (Command Session) — not bare `cargo build --release`. Build log goes to `data/build-logs/app-mediakit-marketing-<ts>.log`. The script refuses to claim "deployed" if the sha256 of the new binary matches the installed binary (no change; no deploy needed).

3. **Binary ledger updated.** `bin/deploy-binary.sh` (Command Session) updates the ledger entry with `source_commit`, `source_tree_sha`, and `working_tree_clean`. Do not manually overwrite the binary without running `deploy-binary.sh`.

---

## Deployment — binary swap

The running binary is at `/usr/local/bin/app-mediakit-marketing`. The swap procedure avoids a race between the old binary exiting and the new one starting.

### Step 1 — Stage new binary

Copy the new binary to a staging path:

```sh
sudo cp app-mediakit-marketing-NEW /usr/local/bin/app-mediakit-marketing.next
sudo chmod 755 /usr/local/bin/app-mediakit-marketing.next
```

Verify the new binary version:

```sh
/usr/local/bin/app-mediakit-marketing.next --version
```

### Step 2 — Swap and restart each tenant

For each tenant using this binary:

```sh
# Woodfine instance
sudo systemctl stop local-marketing.service
sudo mv /usr/local/bin/app-mediakit-marketing /usr/local/bin/app-mediakit-marketing.prev
sudo mv /usr/local/bin/app-mediakit-marketing.next /usr/local/bin/app-mediakit-marketing
sudo systemctl start local-marketing.service
sudo systemctl status local-marketing.service

# PointSav instance (if running same host)
sudo systemctl stop local-marketing-pointsav.service
sudo systemctl start local-marketing-pointsav.service
sudo systemctl status local-marketing-pointsav.service
```

The `.prev` file is the rollback binary. Remove after confirming health:

```sh
sudo rm /usr/local/bin/app-mediakit-marketing.prev
```

### Step 3 — Health verification

```sh
# Woodfine — port 9102
curl -s http://127.0.0.1:9102/ | head -5

# PointSav — port 9101
curl -s http://127.0.0.1:9101/ | head -5

# Public HTTPS
curl -sI https://home.woodfinegroup.com/ | head -3
curl -sI https://home.pointsav.com/    | head -3
```

Expected: HTTP 200, valid HTML, response within 2 seconds.

Check journal for errors:

```sh
sudo journalctl -u local-marketing.service -u local-marketing-pointsav.service --since "5 minutes ago" --no-pager
```

---

## Content update (flat-file, no binary change)

Content files live in `deployments/INSTANCE_NAME/content/`. Because the content directory is served directly by the binary, changes take effect on the next request with no service restart.

### Updating a page

Edit files in `deployments/INSTANCE_NAME/content/`. The deployment directory is gitignored on the workspace. Content is instance-local.

Structured content in `index.html` for media-marketing-landing-1/2 is stored as a JSON string inside a `<script type="__bundler/template">` block. Edits to these files require Python to parse the JSON before modifying HTML inside it:

```python
import json, re

path = "/srv/foundry/deployments/media-marketing-landing-N/content/index.html"

with open(path) as f:
    raw = f.read()

m = re.search(r'(<script type="__bundler/template">)(.*?)(</script>)', raw, re.DOTALL)
inner_html = json.loads(m.group(2))
# ... make edits to inner_html string ...
new_script = m.group(1) + json.dumps(inner_html) + m.group(3)

with open(path, "w") as f:
    f.write(raw[:m.start()] + new_script + raw[m.end():])
```

Verify the change is live:

```sh
curl -s http://127.0.0.1:PORT/ | grep -i "YOUR_EDIT"
```

### Updating robots.txt / sitemap.xml

These are served directly by nginx from `CONTENT_DIR/robots.txt` and `CONTENT_DIR/sitemap.xml`. Edit in place; nginx serves immediately (no reload needed).

---

## Rollback — binary

If the new binary is unhealthy:

```sh
sudo systemctl stop local-marketing.service
sudo mv /usr/local/bin/app-mediakit-marketing /usr/local/bin/app-mediakit-marketing.bad
sudo mv /usr/local/bin/app-mediakit-marketing.prev /usr/local/bin/app-mediakit-marketing
sudo systemctl start local-marketing.service
sudo systemctl status local-marketing.service
```

Update the binary ledger (`bin/deploy-binary.sh`) to record the rollback.

---

## Configuration change

Environment variables are set in the systemd unit file at `/etc/systemd/system/local-marketing-TENANT.service`. After any edit:

```sh
sudo systemctl daemon-reload
sudo systemctl restart local-marketing-TENANT.service
sudo systemctl status local-marketing-TENANT.service
```

**Key environment variables:**

| Variable | Purpose | Example |
|---|---|---|
| `SERVICE_MARKETING_CONTENT_DIR` | Absolute path to content directory | `/srv/foundry/deployments/media-marketing-landing-1/content` |
| `SERVICE_MARKETING_BIND` | Listen address | `127.0.0.1:9102` |
| `SERVICE_MARKETING_MODULE_ID` | Tenant identifier for DataGraph | `woodfine` |
| `SERVICE_MARKETING_SITE_TITLE` | Brand name used in page titles | `Woodfine` |
| `SERVICE_MARKETING_GRAPH_URL` | Doorman endpoint for DataGraph | `http://127.0.0.1:9081` |

---

## nginx reload (after vhost change)

```sh
sudo nginx -t          # test config
sudo systemctl reload nginx
```

A reload (not restart) keeps existing connections alive during the transition.

---

## TLS renewal

Certbot auto-renews via systemd timer. Manual force-renew (for testing):

```sh
sudo certbot renew --dry-run
sudo certbot renew
```

Certificates expire every 90 days; auto-renewal fires at 60 days. Monitor:

```sh
sudo certbot certificates
```

---

## Logs

| Source | Command |
|---|---|
| Service journal | `sudo journalctl -u local-marketing.service -f` |
| nginx access | `sudo tail -f /var/log/nginx/home.woodfinegroup.com.access.log` |
| nginx error | `sudo tail -f /var/log/nginx/home.woodfinegroup.com.error.log` |
| certbot | `sudo journalctl -u certbot.service` |
