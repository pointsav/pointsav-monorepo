---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: applications/   # candidates: applications/, infrastructure/, news/, milestones/ — project-language decides
target_filename: documentation-pointsav-com-launch-2026-04-27.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-27T16:30:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: opus-4-7
references:
  - https://documentation.pointsav.com/
  - https://documentation.pointsav.com/healthz
  - https://documentation.pointsav.com/feed.atom
  - https://documentation.pointsav.com/wiki/welcome
  - https://documentation.pointsav.com/feed.json
  - https://documentation.pointsav.com/sitemap.xml
  - https://documentation.pointsav.com/llms.txt
  - https://documentation.pointsav.com/robots.txt
  - ~/Foundry/clones/project-knowledge/.claude/inbox-archive.md (v0.1.29 entry, 2026-04-27)
  - ~/Foundry/CHANGELOG.md (workspace v0.1.29 entry, 2026-04-27)
  - ~/Foundry/infrastructure/local-knowledge/local-knowledge.service
  - ~/Foundry/infrastructure/configure/configure-ubuntu-foundry.sh
  - ni-51-102 (BCSC NI 51-102 Continuous Disclosure Obligations)
  - osc-sn-51-721 (OSC Staff Notice 51-721 Forward-Looking Information Disclosure)
  - https://www.bcsc.bc.ca/securities-law/law-and-policy/instruments-and-policies/5-ongoing-requirements-for-issuers-insiders/current/51-102
notes_for_editor: |
  This is a current-fact narrative. The launch happened at 2026-04-27T16:25Z under
  workspace v0.1.29. Cert validity (through 2026-07-26) is verifiable via
  `openssl s_client -connect documentation.pointsav.com:443 -servername documentation.pointsav.com`
  until that date. Public smoke results in §3 are reproducible from any external host.

  The placeholder posture is the headline. Four BCSC-clean files are serving; the legacy
  30+ TOPIC corpus is held for the project-language cluster cleanup pass. This minimises
  the material-change-disclosure surface to ONE event when the refined corpus eventually
  publishes (per ni-51-102 §4.1 material-change reporting).

  §5 contains forward-looking framings (next phases of the wiki engine + the project-language
  refinement pipeline). Apply ni-51-102 + osc-sn-51-721 discipline: planned / intended / may
  language; cautionary banner; reasonable-basis lines; material-assumptions block.

  Repetition in §3 is intentional: certbot workflow + nginx vhost + systemd unit are
  described separately because each is a distinct verification surface. Pare or merge as
  the audience requires.

  target_path candidates listed; project-language decides per its taxonomy decision (the
  naming-convention.md draft in content-wiki-documentation's .claude/rules/ proposes nine
  categories, awaiting operator ratification).
---

# documentation.pointsav.com goes live (2026-04-27)

## §1 The launch

`https://documentation.pointsav.com` went live with TLS at 16:25 UTC
on 2026-04-27 under workspace version v0.1.29. The deployment serves
the PointSav engineering wiki (`app-mediakit-knowledge`, the wiki
engine shipped under the project-knowledge cluster) over a Let's
Encrypt ACME-issued certificate, valid through 2026-07-26, with
certbot's automatic renewal scheduled.

The launch closes a path that began in the project-knowledge cluster
session 1 (Phase 1 of the wiki engine), continued through session 2
(Phase 1.1 Wikipedia muscle-memory chrome) and session 3 (Phase 2
all 7 steps + Phase 3 Steps 3.1–3.4), and finished with a Master
session at workspace v0.1.29 that rebuilt the binary from cluster
HEAD, installed it at `/usr/local/bin/app-mediakit-knowledge`,
created the runtime state directory at
`/var/lib/local-knowledge/state`, pointed the systemd unit's
`--content-dir` flag at the placeholder content tree, and fired
certbot's nginx integration to obtain the TLS certificate.

Six Master-side actions were delivered against the queued redeploy
sequence. The binary rebuild took 1 minute 54 seconds. The systemd
service entered the `active` state without warnings. Loopback smoke
on `127.0.0.1:9090` confirmed all routes returning 200. External
smoke on `https://documentation.pointsav.com` confirmed all routes
returning 200 — including the HTTP→HTTPS 301 redirect that certbot's
nginx automation installed alongside the certificate.

The launch surfaced one previously-undetected gap. The workspace VM
ran ufw with `default deny incoming` and only `22/tcp` allowed at
the OS layer, despite the GCP firewall already permitting 80 and 443
through the `allow-https-documentation` rule against the
`documentation-public` target tag. The first certbot run failed with
"Timeout during connect" before the gap was diagnosed. The fix
landed at workspace tier in v0.1.29:
`infrastructure/configure/configure-ubuntu-foundry.sh` was extended
to add `ufw allow 80/tcp` and `ufw allow 443/tcp` rules alongside
the existing `22/tcp` rule, so future VM provisioning inherits these
ports. Live ufw was updated in the same Master pass and verified via
`ufw status numbered` before the second certbot run succeeded. The
`proofreader.woodfinegroup.com` vhost queued for next is now also
unblocked at the OS layer as a consequence.

## §2 What serves today

Four placeholder TOPIC pages render at the public URL.
`/wiki/welcome` is the landing TOPIC, explaining the public-preview
status and linking to the three sample articles. `/wiki/sample-article`
exercises the rendering chrome — table of contents, per-section
edit pencils, footer block with categories, masthead band,
collapsible left-rail TOC, language switcher, and the Wikipedia
muscle-memory layout pattern that Phase 1.1 added. `/wiki/sample-forward-looking`
exercises the forward-looking-information cautionary banner that
Phase 8 will harden into a linter check, and cites both `ni-51-102`
and `osc-sn-51-721` against the workspace citation registry at
`~/Foundry/citations.yaml`. `/wiki/sample-citations` exercises
inline `[citation-id]` references including the clause-reference
form (`[c2sp-tlog-tiles §2]`).

The full route surface from Phases 1, 1.1, 2, and 3 is operational.
Beyond the article-rendering paths, the wiki serves: `/healthz`
(liveness check returning the literal string `ok`); `/` (the index
page listing all articles in the served content tree); `/search?q=`
(full-text search over BM25 against the on-disk Tantivy index at
`/var/lib/local-knowledge/state/search/`, rebuilt on startup and
incrementally on edit); `/feed.atom` (RFC 4287 syndication feed);
`/feed.json` (JSON Feed 1.1); `/sitemap.xml` (sitemaps.org compliant);
`/robots.txt` (crawler discovery); `/llms.txt` (the emerging
llmstxt.org convention for LLM crawlers); and `/git/{slug}` (raw
markdown source for git-clone-style ingestion, with an optional
`.md` suffix preserved for both UX shapes).

The editor surface from Phase 2 is present in the binary — the
`POST /edit/{slug}` route, CodeMirror 6 in-browser editor with
markdown highlighting and atomic write semantics, the squiggle
linting framework with seven deterministic rules and cited
authority hover-cards, citation autocomplete fed by the workspace
registry on the `[` keystroke, and the three-keystroke ladder
stubs for the Doorman MCP integration that Phase 4 will wire up.
The collab passthrough relay from Phase 2 Step 7 is present but
default-off behind the `--enable-collab` CLI flag; the production
deployment does not currently load any of the yjs JavaScript and
does not expose the WebSocket route.

## §3 Operational substrate

The serving stack is conventional Linux + nginx + systemd + Let's
Encrypt, with substrate-specific touches at the boundaries.

**Binary layout.** The app-mediakit-knowledge crate compiles to a
single binary installed at `/usr/local/bin/app-mediakit-knowledge`.
The release build was produced inside the cluster sub-clone at
`/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/`
on the `cluster/project-knowledge` feature branch, not from the
`main` branch — the Phase 2 and Phase 3 commits had not yet
promoted to canonical at launch time. Build duration was 1 minute
54 seconds for `cargo build --release` from a warm target
directory. The Master pass that produced the launch installed the
binary mtime-stamped 2026-04-27 16:16Z.

**systemd unit.** `/etc/systemd/system/local-knowledge.service`
runs the binary as a dedicated unprivileged system user
(`local-knowledge:local-knowledge`), bound to the loopback
interface on port 9090. Hardening flags include `NoNewPrivileges=true`,
`ProtectSystem=strict`, `ProtectHome=true`, `PrivateTmp=true`, and
an explicit `ReadWritePaths=` list naming the runtime state
directory at `/var/lib/local-knowledge/state` and the temporary
directory. The unit's `ExecStart` line invokes the binary with
`--content-dir`, `--citations-yaml`, and `--state-dir` flags
explicitly, plus the matching `WIKI_CONTENT_DIR`,
`WIKI_CITATIONS_YAML`, and `WIKI_STATE_DIR` environment variables
for redundancy — the explicit-in-unit form makes future edits
trivial and the env-var form covers any future invocation that
bypasses the systemd unit (e.g., a developer running the binary
manually for debugging). The unit configuration is kept in version
control as Infrastructure-as-Code at
`~/Foundry/infrastructure/local-knowledge/local-knowledge.service`,
with explanatory comments naming the v0.1.28 and v0.1.29 placeholder
pivot.

**State directory.** `/var/lib/local-knowledge/state` contains the
on-disk Tantivy search index and any future redb databases that
Phase 4 will introduce for the wikilink graph. The directory is
chowned to `local-knowledge:local-knowledge` and lives under the
systemd unit's `ReadWritePaths=` list. The Tantivy index is
rebuilt on every binary startup, so a state-directory wipe is
non-destructive — the canonical content is the markdown tree, not
the index.

**Content tree.** The current `--content-dir` value points at
`/srv/foundry/clones/project-knowledge/content-wiki-documentation/launch-placeholder/`,
the four-file placeholder subdirectory the project-knowledge
cluster authored before launch. This deliberately avoids exposing
the legacy 30+ TOPIC corpus that lives one directory up; the
legacy corpus is held for the project-language cluster's editorial
cleanup pass, after which the `--content-dir` value will swap to
the parent directory or to a subset.

**nginx vhost.** Port 443 terminates TLS and reverse-proxies to the
loopback service on port 9090. Port 80 serves only the certbot HTTP-01
challenge and otherwise issues a 301 redirect to the equivalent
HTTPS URL. The vhost was authored alongside the v0.1.21 production
deployment (separate Master pass, 2026-04-26) and required only
the certbot `--nginx` automation to add the TLS-server block on
top.

**ufw.** The OS firewall on the workspace VM allows 22/tcp for SSH,
80/tcp for HTTP and ACME challenges, and 443/tcp for HTTPS. The
v0.1.29 fix to add 80 and 443 to ufw is described in §1 above; the
specific reason this gap had escaped earlier observation is that
the v0.1.21 deployment had only been smoke-tested from inside the
VM and from internal hosts on the same VPC, both of which bypass
the OS firewall layer.

**DreamHost A record.** `documentation.pointsav.com` resolves to
the workspace VM's public IPv4 address `34.53.65.203`, configured
as a DreamHost-side A record. Operator confirmed the resolution at
2026-04-26T23:00Z prior to the launch session. The DNS path is
external to the workspace's IaC; future automation could move
zone management on-substrate, but the v0.1.29 launch operates
with the DNS pinned externally.

**certbot.** The certificate was provisioned via certbot's nginx
integration — `certbot --nginx -d documentation.pointsav.com --email
open.source@pointsav.com --agree-tos --no-eff-email`. The HTTP-01
challenge succeeded on the second attempt (the first having timed
out at the ufw layer, before the v0.1.29 firewall fix). Auto-renewal
is enabled via certbot's systemd timer; expiry is 2026-07-26.

## §4 The placeholder posture — BCSC-clean by construction

The cluster authored the four-file `launch-placeholder/` subtree
specifically to enable the public TLS launch without exposing the
legacy 30+ TOPIC corpus inherited from earlier development phases.
The legacy corpus carries known editorial debt — language treating
the Sovereign Data Foundation as a current equity holder rather
than a planned counterparty, a sprinkling of forward-looking
framings without the cautionary-banner discipline NI 51-102 requires,
and the occasional Do-Not-Use vocabulary item from the workspace
language policy. Cleaning these in place would have produced 23
unambiguous edits plus 6 contested operator-decision items, plus a
material-change disclosure event for each substantive edit under
the workspace's continuous-disclosure posture.

The placeholder posture collapses that surface. Four files,
written specifically to be BCSC-clean from the first keystroke,
expose only structural prose (no business-outcome claims), only
verified facts (cert dates, route names), and forward-looking
framings only inside the explicit demonstration TOPIC
(`sample-forward-looking.md`) where the cautionary-banner pattern
is the point of the page. The eventual publication of the refined
legacy corpus becomes ONE material-change event ("first publication
of corpus") rather than 23 to 30 separate events.

The placeholder discipline is operationally cheap. The four files
exercise every Phase 2 and Phase 3 UI surface a single-tenant
operator would see in the first browsing session — the index list,
article rendering, the FLI cautionary banner, citation references,
search hits, Atom and JSON Feed entries, sitemap entries, raw
markdown via `/git/{slug}`. UI/UX evaluation can proceed against
the live URL without depending on the corpus cleanup timeline.

The pattern is generalizable. Any future deployment that depends
on a corpus being editorially ready can launch with a placeholder
content tree, swap `--content-dir` once the corpus is ratified,
and avoid the all-or-nothing flip. The substrate's source-of-truth
inversion (the markdown tree is canonical; the running binary is a
view) makes this swap a single systemd reload.

## §5 What's next

The forward-looking framings in this section are *planned*, not
*committed*. Cautionary language applies per NI 51-102 and OSC
Staff Notice 51-721. The reasonable basis is the trajectory of the
project-knowledge and project-language clusters as of 2026-04-27;
material assumptions include the continued availability of the
workspace VM, the project-language cluster's editorial-pipeline
ratification per Doctrine claim #35, and operator clearance of the
Phase 4 BP1 review for the wiki engine.

The project-knowledge cluster *plans* to author bulk drafts of
substantive TOPIC content covering the wiki engine itself, the
substrate-native compatibility surface, and the launch milestone
narrative. These drafts are *intended* to enter the editorial
gateway via the new drafts-outbound input port at
`~/Foundry/clones/project-knowledge/.claude/drafts-outbound/`,
where the project-language cluster *may* refine them to register
+ banned-vocabulary discipline + BCSC-grounded posture + bilingual
pair + citation-registry resolution per the cluster-wiki-draft-pipeline
convention. Refined output *would* hand off to
`content-wiki-documentation` via the existing handoffs-outbound
mechanism for destination-side commit.

The project-language cluster *plans* to refine the legacy 30+ TOPIC
corpus over a separate editorial pass. The expected outcome is a
ratified content tree to which `--content-dir` *may* be swapped,
producing a single material-change disclosure event rather than
many.

The wiki engine itself *plans* further development through Phase 4
(git2 commit-on-edit, history and blame via gix, redb wikilink
graph, blake3 hash storage as a federation seam, MCP server via
rmcp, smart-HTTP read-only Git remote, OpenAPI 3.1 specification),
Phase 5 (image and asset handling), Phase 6 (per-tenant shaping
for Customer wikis), Phase 7 (federation and content-addressed
retrieval against the blake3 substrate), and Phase 8 (the linter
that hardens disclosure-class invariants into compile-time checks).
The Phase 4 plan was authored under cluster session 3 and awaits
operator clearance of seven open questions in the plan document
before implementation begins.

These plans *may* shift in response to operator decisions, customer
requirements, or substrate-level changes to the doctrine. Material
changes to the trajectory will be disclosed via signed commits and
CHANGELOG entries on this domain.

## §6 Verification

The following checks are reproducible from any external host with
TCP/443 connectivity and a recent OpenSSL or curl.

```
$ curl -I https://documentation.pointsav.com/healthz
HTTP/1.1 200 OK
Server: nginx/1.24.0 (Ubuntu)

$ curl https://documentation.pointsav.com/healthz
ok

$ curl -I https://documentation.pointsav.com/wiki/welcome
HTTP/1.1 200 OK

$ curl -I https://documentation.pointsav.com/feed.atom
HTTP/1.1 200 OK

$ curl -I http://documentation.pointsav.com/
HTTP/1.1 301 Moved Permanently

$ openssl s_client -connect documentation.pointsav.com:443 \
    -servername documentation.pointsav.com 2>/dev/null \
    | openssl x509 -noout -dates
notBefore=Apr 27 16:24:00 2026 GMT
notAfter=Jul 26 16:24:00 2026 GMT
```

The certificate's `notBefore` is 16:24Z, one minute before the
v0.1.29 inbox message timestamp, reflecting the certbot internal
clock; the public-facing launch is the 16:25Z timestamp Master
recorded.

The on-substrate IaC paths backing the launch:
`infrastructure/local-knowledge/local-knowledge.service` (systemd
unit, version-controlled in `~/Foundry`); the inbox-archive entry
for v0.1.29 in this cluster's `.claude/inbox-archive.md` (mailbox
record of the Master pass that produced the launch); and the
workspace `CHANGELOG.md` entry for v0.1.29 (release-engineering
record).
