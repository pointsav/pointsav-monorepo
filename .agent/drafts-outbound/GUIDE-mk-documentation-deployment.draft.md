---
artifact: guide
schema: foundry-draft-v1
title: "Deployment — media-knowledge-documentation"
slug: guide-deployment
draft_id: project-editorial-20260616-mk-doc-deployment
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-documentation/guide-deployment.md
route_to: command
action: create
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: adapted from project-knowledge/pointsav-fleet-deployment/media-knowledge-documentation/guide-deployment.md; deployment parameters verified against infrastructure/local-knowledge/local-knowledge.service; WIKI_CONTENT_DIR stale path flagged for Command verification
  data_sources: local-knowledge.service, guide-operate-knowledge-wiki.md reference
  citations_pending: false
  forbidden_terms_cleared: true
---

## Deployment — media-knowledge-documentation

This guide covers initial deployment of the `media-knowledge-documentation` wiki instance, serving `documentation.pointsav.com`. For day-to-day content operations once the instance is running, see `guide-content-operations.md`. For node provisioning (OS setup, user creation, package installation), see `guide-provision-node.md`.

---

## Prerequisites

Before starting this guide, confirm all of the following:

- Node is provisioned per `guide-provision-node.md`.
- nginx is installed: `nginx -v` returns a version string.
- certbot is installed: `certbot --version` returns a version string.
- The `local-knowledge` system user and group exist:

  ```
  id local-knowledge
  ```

- The `media-knowledge-documentation` content repository is checked out and the working tree is clean.
- DNS A record for `documentation.pointsav.com` resolves to this VM's public IP:

  ```
  dig +short documentation.pointsav.com
  ```

---

## Step 1 — Install the binary

The `app-mediakit-knowledge` binary is distributed through the PointSav private release channel. Obtain the release archive for the target version and verify the SHA-256 digest before installing.

```
sha256sum app-mediakit-knowledge-<version>-linux-amd64
# compare against the .sha256 file in the release
```

Install the binary:

```
sudo install -o root -g root -m 0755 \
  app-mediakit-knowledge-<version>-linux-amd64 \
  /usr/local/bin/app-mediakit-knowledge
```

Verify:

```
/usr/local/bin/app-mediakit-knowledge --version
```

---

## Step 2 — Prepare the state directory

The service stores its Tantivy search index in a state directory. The index rebuilds automatically from the content tree on startup, so wiping the state directory is non-destructive.

```
sudo mkdir -p /var/lib/local-knowledge/state
sudo chown local-knowledge:local-knowledge /var/lib/local-knowledge/state
sudo chmod 0750 /var/lib/local-knowledge/state
```

---

## Step 3 — Install the systemd unit

The unit file is version-controlled in the workspace. Copy the current version to the systemd directory:

```
sudo cp infrastructure/local-knowledge/local-knowledge.service \
  /etc/systemd/system/local-knowledge.service
sudo chmod 0644 /etc/systemd/system/local-knowledge.service
sudo systemctl daemon-reload
```

**Verify the WIKI_CONTENT_DIR path before enabling.** Open the installed unit file and confirm the `WIKI_CONTENT_DIR` environment variable points to the actual checkout path of the `media-knowledge-documentation` content repository:

```
grep WIKI_CONTENT_DIR /etc/systemd/system/local-knowledge.service
```

If the path is stale (does not exist on disk), update it to the correct path before proceeding. The service will fail to start if the content directory is absent.

---

## Step 4 — Enable and start the service

```
sudo systemctl enable local-knowledge.service
sudo systemctl start local-knowledge.service
sudo systemctl status local-knowledge.service
```

The unit should report `active (running)`. If it shows `failed`, read the journal:

```
journalctl -u local-knowledge.service -n 50
```

---

## Step 5 — Smoke test on the loopback

The service binds to `127.0.0.1:9090` by default. Verify it responds before exposing it publicly:

```
curl -s http://127.0.0.1:9090/healthz
```

A healthy instance returns `{"status":"ok"}`. If the health check fails or the connection is refused, check the journal output from Step 4 before proceeding.

Also verify the search index is populated:

```
curl -s "http://127.0.0.1:9090/search?q=documentation" | head -5
```

---

## Step 6 — Configure nginx

The nginx configuration proxies requests from the public HTTPS endpoint to the service's loopback port. A minimal configuration follows:

```nginx
server {
    listen 80;
    server_name documentation.pointsav.com;

    location / {
        proxy_pass http://127.0.0.1:9090;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Write this to `/etc/nginx/sites-available/documentation.pointsav.com.conf`, enable it, and reload nginx:

```
sudo ln -s \
  /etc/nginx/sites-available/documentation.pointsav.com.conf \
  /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## Step 7 — Open OS firewall ports

Both TCP/80 (HTTP, required for certbot ACME challenge) and TCP/443 (HTTPS) must be open at the OS firewall layer as well as the GCP network firewall layer.

```
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw status numbered
```

Verify from outside the VM that both ports are reachable before running certbot. A connection that succeeds from inside the VM but times out from outside indicates the OS firewall or GCP rule is not open — fix this before Step 8.

---

## Step 8 — Obtain a TLS certificate

```
sudo certbot --nginx -d documentation.pointsav.com
```

certbot modifies the nginx configuration to add HTTPS and redirect HTTP → HTTPS automatically. After the command completes:

```
sudo nginx -t
sudo systemctl reload nginx
```

---

## Step 9 — Verify the public URL

From a browser or external host, confirm:

```
curl -sI https://documentation.pointsav.com/
```

Expected response: `HTTP/2 200`.

Also confirm the wiki home page renders correctly and search returns results for a known article title.

---

## Failure reference

| Symptom | Likely cause | Action |
|---|---|---|
| `active (failed)` in systemctl status | WIKI_CONTENT_DIR missing or unreadable | Verify path; correct unit file; `systemctl daemon-reload` then restart |
| curl :9090/healthz → Connection refused | Service not started or wrong port | Check journal; confirm `WIKI_BIND` in unit file |
| certbot ACME challenge times out | OS firewall blocking port 80 | `sudo ufw allow 80/tcp` |
| nginx returns 502 | Service not running on :9090 | Restart service; check journal |
| Search returns no results | Index not built | Restart service to trigger index rebuild |

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
