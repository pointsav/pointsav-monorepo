---
artifact: guide
schema: foundry-draft-v1
title: "Deployment — media-knowledge-corporate"
slug: guide-deployment
draft_id: project-editorial-20260616-mk-corp-deployment
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-corporate/guide-deployment.md
route_to: command
action: replace
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: adapted from GUIDE-mk-documentation-deployment.draft.md; corporate instance not yet deployed — service port and WIKI_CONTENT_DIR path require Command verification before use
  data_sources: guide-deployment.md (documentation reference), local-knowledge.service structure
  citations_pending: false
  forbidden_terms_cleared: true
---

## Deployment — media-knowledge-corporate

This guide covers initial deployment of the `media-knowledge-corporate` wiki instance, intended to serve internal corporate documentation for Woodfine Management Corp. For day-to-day content operations, see `guide-content-operations.md`. For node provisioning, see `guide-provision-node.md`.

**Status:** This deployment is planned. The steps below document the intended procedure based on the pattern established by `media-knowledge-documentation`. Before executing, confirm the following with the workspace coordinator:

- The service port assigned to the corporate instance
- The `WIKI_CONTENT_DIR` path on the target VM
- The public domain name and DNS configuration
- Whether the corporate instance requires authentication (the service supports `EDITOR_ENABLED` and MBA auth; confirm access control requirements before exposing publicly)

---

## Prerequisites

- Node is provisioned per `guide-provision-node.md`.
- nginx is installed.
- certbot is installed.
- The `local-knowledge` system user and group exist.
- The `media-knowledge-corporate` content repository is checked out on the target VM.
- DNS A record for the corporate wiki domain resolves to the VM's public IP.

---

## Step 1 — Install the binary

The `app-mediakit-knowledge` binary is shared across all three wiki deployments. If the binary is already installed at `/usr/local/bin/app-mediakit-knowledge`, verify the version and skip this step. If installation is needed, follow `media-knowledge-documentation/guide-deployment.md §Step 1`.

---

## Step 2 — Prepare the state directory

```
sudo mkdir -p /var/lib/local-knowledge-corporate/state
sudo chown local-knowledge:local-knowledge /var/lib/local-knowledge-corporate/state
sudo chmod 0750 /var/lib/local-knowledge-corporate/state
```

---

## Step 3 — Install the systemd unit

A separate unit is required for the corporate instance:

```
sudo cp infrastructure/local-knowledge-corporate/local-knowledge-corporate.service \
  /etc/systemd/system/local-knowledge-corporate.service
sudo chmod 0644 /etc/systemd/system/local-knowledge-corporate.service
sudo systemctl daemon-reload
```

**Before enabling, confirm:**

- `WIKI_CONTENT_DIR` points to the actual checkout of `media-knowledge-corporate`
- `WIKI_BIND` is set to `127.0.0.1:<port>` where `<port>` is the port assigned to this instance
- `WIKI_STATE_DIR` points to the state directory created in Step 2
- If access control is required: confirm with the workspace coordinator whether `EDITOR_ENABLED` should be set and what authentication configuration is needed

---

## Step 4 — Enable and start the service

```
sudo systemctl enable local-knowledge-corporate.service
sudo systemctl start local-knowledge-corporate.service
sudo systemctl status local-knowledge-corporate.service
```

If the unit shows `failed`:

```
journalctl -u local-knowledge-corporate.service -n 50
```

---

## Step 5 — Smoke test on the loopback

```
curl -s http://127.0.0.1:<port>/healthz
```

Expected: `{"status":"ok"}`.

---

## Steps 6–9 — nginx, firewall, TLS, and verification

Follow the same procedure as `media-knowledge-documentation/guide-deployment.md §Steps 6–9`, substituting the corporate domain name and port number.

---

## Failure reference

See `media-knowledge-documentation/guide-deployment.md §Failure reference` — the failure modes and remediation steps are identical across all three wiki instances.

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
