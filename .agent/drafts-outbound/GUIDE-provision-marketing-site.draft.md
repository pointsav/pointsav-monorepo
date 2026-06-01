---
schema: foundry-draft-v1
type: GUIDE-provision-marketing-site
language_protocol: PROSE-GUIDE
title: "Provision a marketing landing site instance"
target_project: project-editorial
target_destination: customer/woodfine-fleet-deployment/media-marketing-landing/ | vendor/pointsav-fleet-deployment/media-marketing-landing/
created: 2026-06-01T00:00:00Z
author: totebox@project-marketing (claude-code / claude-sonnet-4-6)
research_trail:
  source_commits:
    - b49b4ae (cluster/project-marketing — ops shutdown, session-context updated)
    - 1686c35 (outputs/ established)
    - 8da56fc (session-context 2026-05-24 — SEO applied, binary-targets declared)
  prior_drafts: []
  citations:
    - conventions/compounding-substrate.md
    - conventions/datagraph-access-discipline.md
    - clones/project-marketing/.agent/manifest.md
  operator_inputs:
    - "Manifest: media-marketing-landing-1 + -2 both active on foundry-workspace"
    - "systemd unit: local-marketing.service (woodfine, 9102) + local-marketing-pointsav.service (pointsav, 9101)"
    - "nginx vhosts: home.woodfinegroup.com + home.pointsav.com"
    - "Tier 0 target: runs on $7/mo node with no AI tier"
  related_files:
    - deployments/media-marketing-landing-1/MANIFEST.md
    - deployments/media-marketing-landing-2/MANIFEST.md
    - /etc/systemd/system/local-marketing.service
    - /etc/nginx/sites-available/home.woodfinegroup.com
---

# Guide: Provision a marketing landing site instance

Operational runbook for provisioning a new `app-mediakit-marketing` instance from scratch. Covers infrastructure through first health verification. One instance = one tenant (one domain, one content directory, one systemd unit, one nginx vhost).

---

## Prerequisites

**Hardware (Tier 0 minimum):** 1 vCPU, 512 MB RAM, 10 GB storage. Compatible with any $7/mo VPS (Hetzner CX11, Oracle Free Tier, GCE e2-micro).

**OS:** Ubuntu 22.04 LTS or 24.04 LTS (tested). Debian 12 is compatible.

**Required packages:**

```sh
sudo apt-get update
sudo apt-get install -y nginx certbot python3-certbot-nginx
```

**Binary:** `app-mediakit-marketing` compiled for the target platform. Build via `bin/build-binary.sh app-mediakit-marketing` from the Command Session, or obtain from the software catalog at `data/software-catalog/app-mediakit-marketing/`.

---

## Step 1 — System user

```sh
sudo useradd --system --no-create-home --shell /usr/sbin/nologin local-marketing
sudo mkdir -p /var/lib/local-marketing
sudo chown local-marketing:local-marketing /var/lib/local-marketing
```

If provisioning multiple tenants on the same host, they share this user.

---

## Step 2 — Install binary

```sh
sudo cp app-mediakit-marketing /usr/local/bin/app-mediakit-marketing
sudo chmod 755 /usr/local/bin/app-mediakit-marketing
```

Verify:

```sh
/usr/local/bin/app-mediakit-marketing --version
```

---

## Step 3 — Content directory

Create the content directory for this instance. The directory holds Markdown pages, HTML overrides, and static files served by the binary.

```sh
export DEPLOY_NAME=media-marketing-landing-N   # substitute N
export CONTENT_DIR=/srv/foundry/deployments/${DEPLOY_NAME}/content

sudo mkdir -p ${CONTENT_DIR}/media
```

Minimum required files in `content/`:

| File | Purpose |
|---|---|
| `index.html` | Home page (served at `/`) |
| `robots.txt` | Crawler policy |
| `sitemap.xml` | Sitemap for search engines |

Copy or scaffold these from an existing instance (`media-marketing-landing-1` or `-2`) and update domain-specific values (canonical URL, site title, org name, `sameAs` links in JSON-LD).

---

## Step 4 — systemd unit

Create `/etc/systemd/system/local-marketing-TENANT.service`. Substitute `TENANT`, `PORT`, `CONTENT_DIR`, and `MODULE_ID` for this instance:

```ini
[Unit]
Description=Marketing Landing — TENANT tenant (app-mediakit-marketing)
Documentation=file:///srv/foundry/deployments/DEPLOY_NAME/MANIFEST.md
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=local-marketing
Group=local-marketing
WorkingDirectory=/var/lib/local-marketing

Environment="SERVICE_MARKETING_CONTENT_DIR=CONTENT_DIR"
Environment="SERVICE_MARKETING_BIND=127.0.0.1:PORT"
Environment="SERVICE_MARKETING_MODULE_ID=MODULE_ID"
Environment="SERVICE_MARKETING_SITE_TITLE=SITE_TITLE"
Environment="SERVICE_MARKETING_GRAPH_URL=http://127.0.0.1:9081"

ExecStart=/usr/local/bin/app-mediakit-marketing serve

Restart=on-failure
RestartSec=5s

NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadWritePaths=/var/lib/local-marketing

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

**Port allocation:** `9100` + instance index (woodfine = 9102, pointsav = 9101; next instance gets 9103, etc.). Ports must not conflict across tenants on the same host.

**MODULE_ID values:** `woodfine` (Woodfine Management Corp. tenant), `pointsav` (PointSav vendor reference tenant), or any custom string matching the DataGraph module_id for this tenant.

Add crash-loop guard:

```sh
sudo mkdir -p /etc/systemd/system/local-marketing-TENANT.service.d/
```

Create `/etc/systemd/system/local-marketing-TENANT.service.d/crash-loop-guard.conf`:

```ini
[Unit]
StartLimitBurst=5
StartLimitIntervalSec=300
```

Enable and start:

```sh
sudo systemctl daemon-reload
sudo systemctl enable local-marketing-TENANT.service
sudo systemctl start local-marketing-TENANT.service
sudo systemctl status local-marketing-TENANT.service
```

Verify the service is listening:

```sh
curl -s http://127.0.0.1:PORT/ | head -5
```

---

## Step 5 — nginx vhost

Create `/etc/nginx/sites-available/DOMAIN`:

```nginx
server {
    server_name DOMAIN;

    location /.well-known/acme-challenge/ {
        root /var/www/letsencrypt;
        try_files $uri =404;
    }

    location = /robots.txt {
        alias CONTENT_DIR/robots.txt;
        add_header Content-Type text/plain;
    }

    location = /sitemap.xml {
        alias CONTENT_DIR/sitemap.xml;
        add_header Content-Type application/xml;
    }

    location / {
        proxy_pass http://127.0.0.1:PORT;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_http_version 1.1;
        proxy_read_timeout 30s;
    }

    access_log /var/log/nginx/DOMAIN.access.log;
    error_log  /var/log/nginx/DOMAIN.error.log;

    listen 80;
    listen [::]:80;
}
```

Enable and test:

```sh
sudo ln -s /etc/nginx/sites-available/DOMAIN /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

Verify HTTP:

```sh
curl -s http://DOMAIN/ | head -5
```

---

## Step 6 — DNS

Point the domain's A record (and AAAA if IPv6) to the host's public IP. Allow up to 24 hours for propagation, though most registrars resolve within minutes.

Verify:

```sh
dig +short A DOMAIN
```

---

## Step 7 — TLS via certbot

Once DNS is resolving and HTTP is verified:

```sh
sudo certbot --nginx -d DOMAIN
```

Certbot will amend the nginx vhost to add the `:443` block and HTTP→HTTPS redirect automatically.

Verify:

```sh
curl -sI https://DOMAIN/ | head -5
```

Auto-renewal is configured by certbot's systemd timer. Confirm:

```sh
sudo systemctl status certbot.timer
```

---

## Step 8 — Deployment MANIFEST

Author a `MANIFEST.md` in the deployment instance directory using the `templates/deployment-manifest.md.tmpl` template (or copy from an existing instance). Fields to update:

- `deployment:` — unique instance name
- `tenant:` — module_id value
- `public_url:` — `https://DOMAIN`
- `guide:` — fleet-deployment catalog path for this deployment
- `state: active`
- `created:` — ISO 8601 date

Commit the MANIFEST via `bin/commit-as-next.sh` from the correct Totebox session.

---

## Step 9 — Register in fleet catalog

Add a row for this instance to the fleet catalog at:
- `customer/woodfine-fleet-deployment/media-marketing-landing/` (customer-tier instances)
- `vendor/pointsav-fleet-deployment/media-marketing-landing/` (vendor-tier instances)

This is an admin-tier write; route via outbox to Command Session.

---

## Verification checklist

- [ ] `systemctl status local-marketing-TENANT.service` → active (running)
- [ ] `curl -s http://127.0.0.1:PORT/` returns HTML
- [ ] `curl -sI https://DOMAIN/` returns HTTP 200 with valid TLS
- [ ] `https://DOMAIN/robots.txt` returns crawler policy
- [ ] `https://DOMAIN/sitemap.xml` returns sitemap
- [ ] MANIFEST.md committed and fleet catalog updated
