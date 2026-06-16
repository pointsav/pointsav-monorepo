---
artifact: guide
schema: foundry-draft-v1
title: "Deployment — media-knowledge-projects"
slug: guide-deployment
draft_id: project-editorial-20260616-mk-proj-deployment
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-projects/guide-deployment.md
route_to: command
action: replace
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: adapted from GUIDE-mk-documentation-deployment.draft.md; projects instance not yet deployed — service port and WIKI_CONTENT_DIR path require Command verification before use
  data_sources: guide-deployment.md (documentation reference), local-knowledge.service structure
  citations_pending: false
  forbidden_terms_cleared: true
---

## Deployment — media-knowledge-projects

This guide covers initial deployment of the `media-knowledge-projects` wiki instance, intended to serve project-specific and research documentation for active Woodfine operations. For day-to-day content operations, see `guide-content-operations.md`. For node provisioning, see `guide-provision-node.md`.

**Status:** This deployment is planned. The steps below document the intended procedure based on the pattern established by `media-knowledge-documentation`. Before executing, confirm the following with the workspace coordinator:

- The service port assigned to the projects instance
- The `WIKI_CONTENT_DIR` path on the target VM
- The public domain name and DNS configuration

---

## Prerequisites

- Node is provisioned per `guide-provision-node.md`.
- nginx is installed: `nginx -v` returns a version string.
- certbot is installed: `certbot --version` returns a version string.
- The `local-knowledge` system user and group exist.
- The `media-knowledge-projects` content repository is checked out on the target VM.
- DNS A record for the projects wiki domain resolves to the VM's public IP.

---

## Step 1 — Install the binary

The `app-mediakit-knowledge` binary is shared with the documentation instance — the same binary serves all three wiki deployments with different configuration. If the binary is already installed at `/usr/local/bin/app-mediakit-knowledge`, skip this step and verify the installed version matches the intended release.

```
/usr/local/bin/app-mediakit-knowledge --version
```

If installation is needed, follow the binary installation procedure in `media-knowledge-documentation/guide-deployment.md §Step 1`.

---

## Step 2 — Prepare the state directory

The projects instance uses a separate state directory from the documentation instance:

```
sudo mkdir -p /var/lib/local-knowledge-projects/state
sudo chown local-knowledge:local-knowledge /var/lib/local-knowledge-projects/state
sudo chmod 0750 /var/lib/local-knowledge-projects/state
```

---

## Step 3 — Install the systemd unit

A separate systemd unit is required for the projects instance. The unit file template is in the workspace at `infrastructure/local-knowledge-projects/local-knowledge-projects.service`. Install and verify:

```
sudo cp infrastructure/local-knowledge-projects/local-knowledge-projects.service \
  /etc/systemd/system/local-knowledge-projects.service
sudo chmod 0644 /etc/systemd/system/local-knowledge-projects.service
sudo systemctl daemon-reload
```

**Before enabling, confirm:**

- `WIKI_CONTENT_DIR` points to the actual checkout of `media-knowledge-projects`
- `WIKI_BIND` is set to `127.0.0.1:<port>` where `<port>` is the port assigned to this instance
- `WIKI_STATE_DIR` points to the state directory created in Step 2

---

## Step 4 — Enable and start the service

```
sudo systemctl enable local-knowledge-projects.service
sudo systemctl start local-knowledge-projects.service
sudo systemctl status local-knowledge-projects.service
```

If the unit shows `failed`, read the journal:

```
journalctl -u local-knowledge-projects.service -n 50
```

---

## Step 5 — Smoke test on the loopback

```
curl -s http://127.0.0.1:<port>/healthz
```

Expected: `{"status":"ok"}`. Substitute the confirmed port number.

---

## Step 6 — Configure nginx

Add a new nginx server block for the projects domain. The configuration is identical in structure to the documentation instance's configuration, with the domain name and proxy port substituted. See `media-knowledge-documentation/guide-deployment.md §Step 6` for the template.

---

## Step 7 — Open OS firewall ports

TCP/80 and TCP/443 are shared with the documentation instance if both run on the same VM — no additional firewall rule is needed if the documentation instance is already running. Verify:

```
sudo ufw status numbered | grep -E '80|443'
```

---

## Step 8 — Obtain a TLS certificate

```
sudo certbot --nginx -d <projects-wiki-domain>
```

---

## Step 9 — Verify the public URL

```
curl -sI https://<projects-wiki-domain>/
```

Expected: `HTTP/2 200`.

---

## Failure reference

See `media-knowledge-documentation/guide-deployment.md §Failure reference` — the failure modes and remediation steps are identical across all three wiki instances.

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
