---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: pointsav-fleet-deployment
target_path: media-knowledge-documentation/
target_filename: GUIDE-operate-knowledge-wiki.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: PROSE-GUIDE
authored: 2026-04-27T16:35:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: opus-4-7
references:
  - ~/Foundry/infrastructure/local-knowledge/local-knowledge.service
  - ~/Foundry/infrastructure/configure/configure-ubuntu-foundry.sh
  - ~/Foundry/infrastructure/local-slm/local-slm.service (precedent)
  - vendor/pointsav-fleet-deployment/media-knowledge-documentation/guide-deployment.md (sibling — provision)
  - vendor/pointsav-fleet-deployment/media-knowledge-documentation/guide-provision-node.md (sibling — node)
  - https://certbot.eff.org/
  - https://man7.org/linux/man-pages/man5/systemd.exec.5.html
  - https://help.ubuntu.com/community/UFW
notes_for_editor: |
  English-only operational GUIDE per CLAUDE.md §14 (GUIDE files are not bilingual).
  Audience is the operator running the wiki node day-to-day, not the public.
  bcsc_class is no-disclosure-implication — this is operational documentation,
  not material claims about business state.

  Pairs with the existing guide-deployment.md (initial bring-up procedure) and
  guide-provision-node.md (node-tier provisioning); this GUIDE covers DAY-2 OPERATIONS
  — what to check, what to update, what gotchas the v0.1.29 launch surfaced. Don't
  duplicate provision content; cross-reference to it.

  Sections are heavy on bullets and concrete commands by design — operators reach
  for a GUIDE in the middle of an incident, not from a quiet desk. Pare prose, keep
  commands. Preserve all paths verbatim (typos in operational paths burn time).

  The libssl-dev / cargo-workspace-coupling section is forward-looking in the sense
  that it names a still-open install + cleanup task; not BCSC forward-looking
  (it's an internal workspace todo, not a business outcome statement). Frame as
  current-state operational gotcha + planned remediation.
---

# GUIDE — Operating the knowledge wiki node

This guide covers day-2 operations for the
`media-knowledge-documentation` deployment serving
`https://documentation.pointsav.com`. For initial node provisioning,
see `guide-provision-node.md`. For initial deployment bring-up
(systemd unit installation, content-tree configuration, first
service start), see `guide-deployment.md`.

The lessons in this GUIDE are derived from the v0.1.29 launch
(2026-04-27) and reflect real surfaces touched during that pass.

## §1 The surfaces you operate

The deployment runs on the workspace VM as a single binary
(`/usr/local/bin/app-mediakit-knowledge`) under a dedicated
`local-knowledge` system user. The runtime touches several surfaces
you may need to inspect or modify:

| Surface | Path | Owner |
|---|---|---|
| systemd unit | `/etc/systemd/system/local-knowledge.service` | root, mode 0644 |
| systemd unit (IaC) | `~/Foundry/infrastructure/local-knowledge/local-knowledge.service` | version-controlled |
| Binary | `/usr/local/bin/app-mediakit-knowledge` | root, mode 0755 |
| Content tree | `<as configured by --content-dir>` | local-knowledge readable |
| State dir | `/var/lib/local-knowledge/state` | local-knowledge:local-knowledge |
| Citations registry | `/srv/foundry/citations.yaml` | local-knowledge readable |
| nginx vhost | `/etc/nginx/sites-enabled/documentation.pointsav.com.conf` | root |
| TLS cert | `/etc/letsencrypt/live/documentation.pointsav.com/` | root |
| OS firewall | ufw (rules in `/etc/ufw/`) | root |
| GCP firewall | `allow-https-documentation` rule, `documentation-public` target tag | gcloud |
| DNS | DreamHost A record `documentation.pointsav.com` → VM public IP | external |

The unit, binary, citations registry, and IaC are version-controlled
in `~/Foundry`. The content tree is version-controlled in
`~/Foundry/clones/project-knowledge/content-wiki-documentation/`
(or whichever subdirectory is named in `--content-dir`). State
directory contents are non-canonical — the Tantivy search index
rebuilds on startup from the content tree, so wiping state is
non-destructive.

## §2 Firewall — both layers must be open

The launch surfaced a class of bug worth memorising: the wiki
serves on TCP/80 and TCP/443, and **both** the GCP firewall and
the OS firewall (ufw) must permit those ports.

The v0.1.21 deployment (2026-04-26) had only the GCP firewall open
on 80 and 443 (`allow-https-documentation` rule, `documentation-public`
target tag). The OS firewall on the workspace VM ran ufw with
`default deny incoming` and an `allow 22/tcp` rule for SSH only.
The deployment had passed loopback smoke from inside the VM, but
the public URL had never been reached from outside until certbot
attempted the HTTP-01 challenge during the v0.1.29 launch. The
challenge timed out; root cause was the OS firewall blocking 80
and 443 at the kernel level, behind the GCP firewall that was
already open.

The fix landed in the workspace IaC at v0.1.29 in
`infrastructure/configure/configure-ubuntu-foundry.sh`. New VM
provisioning inherits the rules. A live VM that was provisioned
before v0.1.29 needs the rules added once:

```
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw status numbered
```

To verify both layers from outside the VM:

```
# OS layer (any host with TCP connectivity to the VM IP):
nc -zv 34.53.65.203 80
nc -zv 34.53.65.203 443

# GCP layer (gcloud-authenticated host):
gcloud compute firewall-rules describe allow-https-documentation \
  --project <project-id>
```

If `nc` reports refused / timed out, the OS layer is the suspect
even if the GCP rule is in place. If the GCP rule is missing, the
GCP layer needs the rule added; that's a separate operation
described in `infrastructure/`'s GCP-side IaC.

The next vhost queued (`proofreader.woodfinegroup.com`) inherits
both layers' fix automatically as a consequence of the v0.1.29
ufw rules; no per-vhost firewall change is needed for a new
vhost on the same VM serving the same ports.

## §3 systemd unit and state directory

The unit is configured to run the binary as the unprivileged
`local-knowledge` system user with hardening flags appropriate to
a network-facing service that holds no secrets. Key sections:

```
[Service]
Type=exec
User=local-knowledge
Group=local-knowledge
ExecStart=/usr/local/bin/app-mediakit-knowledge serve \
  --content-dir /srv/foundry/clones/project-knowledge/content-wiki-documentation/launch-placeholder \
  --citations-yaml /srv/foundry/citations.yaml \
  --state-dir /var/lib/local-knowledge/state \
  --bind 127.0.0.1:9090

Environment=WIKI_CONTENT_DIR=/srv/foundry/clones/project-knowledge/content-wiki-documentation/launch-placeholder
Environment=WIKI_CITATIONS_YAML=/srv/foundry/citations.yaml
Environment=WIKI_STATE_DIR=/var/lib/local-knowledge/state

NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadWritePaths=/var/lib/local-knowledge/state
```

The CLI flags and the environment variables both convey the same
configuration; the explicit-in-unit form makes future edits trivial
(grep + sed) and the env-var form covers any future invocation that
bypasses the systemd unit (developer running the binary manually
for debugging, an `xargs`-style batch script, etc.).

To edit the unit, edit the IaC copy at
`~/Foundry/infrastructure/local-knowledge/local-knowledge.service`,
then propagate to `/etc/systemd/system/local-knowledge.service`
(symlink or copy as the workspace's IaC convention dictates), then
reload and restart:

```
sudo systemctl daemon-reload
sudo systemctl restart local-knowledge.service
sudo systemctl status local-knowledge.service
```

The state directory at `/var/lib/local-knowledge/state` must exist,
must be chowned to `local-knowledge:local-knowledge`, and must be
named in the unit's `ReadWritePaths=` list. Initial creation:

```
sudo mkdir -p /var/lib/local-knowledge/state
sudo chown -R local-knowledge:local-knowledge /var/lib/local-knowledge
```

The state directory currently holds the Tantivy search index
(rebuilt on startup, incrementally updated on edit). Phase 4 will
introduce redb databases for the wikilink graph, also living in
this directory. The directory is non-canonical state — wiping it
loses no information, but causes a rebuild-from-content-tree on
the next service start, which can take seconds to minutes
depending on content tree size.

## §4 Content directory and citations registry

Two paths configure what the wiki serves:

**`--content-dir` / `WIKI_CONTENT_DIR`** points at the markdown
tree the wiki renders. The current value (as of v0.1.29) is
`/srv/foundry/clones/project-knowledge/content-wiki-documentation/launch-placeholder/`,
the four-file placeholder subtree the project-knowledge cluster
authored to enable the public TLS launch without exposing the
legacy 30+ TOPIC corpus. To swap the served tree (e.g., once the
project-language cluster ratifies the refined corpus), edit the
unit, daemon-reload, restart. The wiki has no in-flight requests
to drain; restart is fast.

**`--citations-yaml` / `WIKI_CITATIONS_YAML`** points at the
workspace citation registry at `/srv/foundry/citations.yaml`. The
registry powers the editor's `[`-triggered citation autocomplete
and the inline `[citation-id]` resolution at render time. The
registry uses the workspace YAML-frontmatter convention — a
metadata block followed by a YAML document with a `citations:`
top-level key. The wiki engine handles the frontmatter strip
automatically; no special accommodation needed in the unit.

Editing either path follows the same pattern: edit the IaC, edit
the live unit, daemon-reload, restart. There is no
configuration-reload-without-restart path; the binary loads
configuration once at startup. A SIGHUP-reload story is open as
Phase 5 work and not yet implemented.

## §5 certbot lifecycle

The TLS certificate for `documentation.pointsav.com` is provisioned
via certbot's nginx integration:

```
sudo certbot --nginx -d documentation.pointsav.com \
  --email open.source@pointsav.com \
  --agree-tos --no-eff-email
```

The HTTP-01 challenge requires TCP/80 reachable from the public
internet (see §2 firewall). On the workspace VM, certbot's renewal
runs via a systemd timer installed by the `python3-certbot` package;
the timer fires twice daily and renews any cert with less than 30
days of validity remaining. To inspect:

```
sudo systemctl list-timers | grep certbot
sudo certbot certificates
sudo certbot renew --dry-run
```

The current cert is valid through 2026-07-26. Renewal will fire
automatically around 2026-06-26.

If renewal fails (e.g., the firewall regresses, or the nginx vhost
is rewritten incompatibly), the cert simply expires. There is no
hard-fail path that drops the wiki — nginx will continue serving
the expired cert and browsers will warn. To detect failure, check
the certbot timer's status weekly or wire a workspace-tier alert.
Today neither is automated; workspace-tier monitoring is open
work.

The HTTP→HTTPS 301 redirect on TCP/80 is installed by certbot's
nginx integration as part of the `--nginx` flow. To verify after
any nginx reconfiguration:

```
curl -I http://documentation.pointsav.com/
# expect: HTTP/1.1 301 Moved Permanently
# expect: Location: https://documentation.pointsav.com/
```

## §6 ProtectHome trade-off

The systemd unit currently sets `ProtectHome=true`, which makes
`/home`, `/root`, and `/run/user` inaccessible to the service.
This is the standard hardening posture for a network-facing
service that has no business reading user home directories.

The trade-off: if the content tree lives under a user home
directory (e.g., `/home/mathew/Foundry/clones/project-knowledge/...`),
the service cannot read it, and the wiki returns 404 for every
article on startup. This bit early development on alternative
workspace mounts.

Two paths to avoid the trade-off:

1. **Keep the content tree out of `/home`.** The current production
   path `/srv/foundry/clones/project-knowledge/...` is already
   compliant; this guide's recommended posture.
2. **Relax `ProtectHome` to `read-only`.** The unit can set
   `ProtectHome=read-only` instead of `true`; the service can read
   home-directory paths but cannot write. Adequate for a content
   tree that is canonically managed via git on the operator's
   side, not via the wiki's `POST /edit` route. Less hardened than
   `true` but acceptable when path 1 is impractical.

Path 1 is preferred. Path 2 is documented for the case where an
operator is running the wiki against a content tree under their
home directory for development purposes.

## §7 Build coupling — libssl-dev and the workspace cargo

A subtlety surfaces when rebuilding the binary from the cluster
HEAD. Running `cargo build --release` from the
`pointsav-monorepo` root pulls in `service-content`'s `reqwest`
dependency, which transitively requires `openssl-sys`, which
requires `libssl-dev` to be installed at the system level. On a
fresh VM without `libssl-dev`, the workspace-root build fails on
that crate.

The workaround during launch was to build from inside the wiki
crate's directory specifically, which keeps cargo's resolver
scoped to that crate's dependency graph (which uses only `rustls`,
not `openssl-sys`):

```
cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo install -m 755 target/release/app-mediakit-knowledge \
  /usr/local/bin/app-mediakit-knowledge
```

This pattern is *the* operational build path for the wiki node
until either (a) `libssl-dev` is installed on the workspace VM, or
(b) `service-content` switches its `reqwest` features from
`native-tls` to `rustls` and drops the openssl coupling, or (c)
the parent workspace's `[members]` declaration is tightened so
that `cargo build` from the monorepo root no longer pulls
`service-content` into the resolver for app-mediakit-knowledge.

Phase 4 of the wiki engine introduces `git2` as a dependency,
which requires `libgit2-dev` on the system at build time. The
crate-scoped build pattern continues to work for git2 the same way
it works for openssl-sys; from outside the crate, the
workspace-root build will need both libs installed. Both `apt
install libssl-dev` and `apt install libgit2-dev` are open items
queued for a Master-tier session on the workspace VM.

## §8 Day-2 operations checklist

A quick operator checklist for the recurring tasks:

- **Check service health.**
  `sudo systemctl status local-knowledge.service`. Look for
  `active (running)`. Check journal for errors:
  `sudo journalctl -u local-knowledge -n 100 --no-pager`.
- **Smoke the public URL.**
  `curl -I https://documentation.pointsav.com/healthz` from any
  external host. Expect 200.
- **Smoke the route surface.**
  `/`, `/wiki/welcome`, `/feed.atom`, `/feed.json`, `/sitemap.xml`,
  `/robots.txt`, `/llms.txt`, `/search?q=substrate`, `/git/welcome`.
  All should return 200.
- **Check the cert expiry.**
  `openssl s_client -connect documentation.pointsav.com:443
  -servername documentation.pointsav.com 2>/dev/null | openssl
  x509 -noout -dates`. The `notAfter` should be more than 30 days
  in the future; renewal is automatic but worth eyeballing
  monthly.
- **Check disk usage.**
  `du -sh /var/lib/local-knowledge/state`. The Tantivy index grows
  with content tree size; current placeholder content produces
  approximately 100 KB. A future legacy-corpus tree will produce
  a larger index — budget tens of MB per thousand articles.
- **Roll the content tree.** When swapping `--content-dir` (e.g.,
  for a new content release), follow the §4 procedure: edit IaC,
  edit live unit, daemon-reload, restart, smoke. Restart is fast;
  no maintenance window required.
- **Rebuild and redeploy the binary.** When pulling Phase 4+
  features into production, follow the §7 crate-scoped build
  pattern. Stop the service before installing the new binary if
  you want to be conservative, or rely on systemd's `Type=exec`
  semantics that re-exec the binary path on restart. After
  install, restart and smoke.

The wiki has no scheduled maintenance windows. Restarts are fast
(under a second between `systemctl restart` and the next 200
response on `/healthz`). The Tantivy index rebuild on startup is
non-blocking — the service answers requests during rebuild,
returning empty search results for queries that hit during the
rebuild window. For the placeholder content, rebuild is
millisecond-scale; for a larger corpus, it could be seconds.
