---
artifact: brief
schema: foundry-brief-v1
brief-id: project-foodservice-platform-master
title: Foodservice Platform Master
status: active
owner: project-foodservice
created: 2026-06-24
updated: 2026-06-24
---

## Context

`foodservice.woodfinegroup.com` is a Woodfine Capital Projects domain currently
serving a Sourdough Tracker PWA (`local-bread.service`, port 9099) via nginx.
The domain is not publicly reachable as of 2026-06-24 (ERR_FAILED from browser;
nginx proxies to 127.0.0.1:9099 but the service binds on 10.8.0.9:9099).

The archive mission is to replace this with a proper `app-mediakit-*` marketing
engine following the pattern established by `app-mediakit-marketing`.

## Scope

- `app-mediakit-foodservice` crate inside `pointsav-monorepo`
- `content/` directory: flat-file YAML page manifests
- systemd unit: `local-foodservice.service` (future; Command Session deploys)
- nginx: update `/etc/nginx/sites-available/foodservice.woodfinegroup.com`
  to proxy to port 9103 (future; Command Session)

## Decisions locked

- Port: `127.0.0.1:9103`
- Env prefix: `SERVICE_FOODSERVICE_*`
- Module id: `woodfine`
- Follows `app-mediakit-marketing` pattern 1:1
- Content stubs: home, contact, disclaimer (EN + ES) — authored fresh;
  no live site content to migrate (site was unreachable at P1 scaffold time)

## Decisions open

- Full content authoring for `content/home/page.yaml` — stub only
- nginx cut-over: update proxy from 9099 → 9103; Command Session action
- systemd unit for `local-foodservice.service`
- woodfine-fleet-deployment catalog entry (outbox sent to Command)
- What happens to `local-bread.service` after cut-over?

## Work log

### Session 1 — 2026-06-24 (P1 scaffold)

- Crate `app-mediakit-foodservice` created; all source files (main, lib,
  config, content, server, mcp, pending)
- Workspace member added; release profile added
- Content stubs: home, contact, disclaimer (EN + ES)
- Tests: routes.rs (6 tests + shipped-manifest guard)
- README.md + README.es.md
- briefs/README.md + session-start.md initialized (were contaminated)
- Live site unreachable: `cargo check` passing; tests deferred to compile step

## Carry-forward

- [ ] Run `cargo test -p app-mediakit-foodservice` — confirm all 7 tests pass
- [ ] Outbox to Command: woodfine-fleet-deployment catalog entry
- [ ] Outbox to Command: nginx cut-over 9099 → 9103 for foodservice domain
- [ ] Outbox to Command: systemd unit `local-foodservice.service`
- [ ] Content authoring: home page (requires clarity on food service purpose)
