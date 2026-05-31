---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: woodfine/woodfine-fleet-deployment
target_path: node-console-operator/
target_filename: guide-os-console-operator.md
target_cluster: node-console-operator
audience: operators
bcsc_class: current-fact
language_protocol: PROSE-GUIDE
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: false
references:
  - woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
  - woodfine-fleet-deployment/node-console-operator/guide-console-operations.md
  - .agent/plans/os-console-platform.md
  - .agent/plans/leapfrog-2030-coding.md
research_trail:
  source_commits:
    - "guide-command-ledger.md — F-key map, MBA LINK ACTIVE, Zero-Form"
    - "guide-console-operations.md — Derivative HUD, pane structure"
    - "os-console-platform.md — F-key canonical map, status bar, config"
  prior_drafts: []
  citations:
    - SYS-ADR-10
  operator_inputs:
    - "F-key map is canonical WIP; F12 is immovable (2026-05-20)"
    - "Terminal requirements: Kitty/iTerm2/Ghostty/WezTerm for PDF; any VTE for everything else (2026-05-20)"
    - "Doorman at http://localhost:9080; service-proofreader at 127.0.0.1:9092 (2026-05-20)"
  related_files:
    - .agent/drafts-outbound/guide-mba-pairing-ceremony.md
    - .agent/drafts-outbound/topic-os-console-platform.md
    - .agent/drafts-outbound/topic-input-machine.md
notes_for_editor: |
  Comprehensive operator reference. Routes to node-console-operator/ in woodfine-fleet-deployment.
  Written for operators who use os-console daily — not for developers.

  Refinement priorities:
  - Apply foundry-doc-v1 schema header when promoting to woodfine-fleet-deployment
  - Update F-key map section when assignments are finalized (marked WIP)
  - Confirm cartridge installation instructions against Phase 1 implementation
  - Add copyright footer per woodfine-fleet-deployment convention
  - English only
---

# GUIDE: os-console Operator Reference

**Audience:** Daily operators of os-console
**Prerequisite:** os-console installed; MBA pairings established

---

## I. What os-console is

`os-console` is a keyboard-native terminal interface for working with the Totebox
Archive. It provides proofread, draft, governance, financial, and infrastructure
workflows through F-key-navigated cartridges in a single terminal session.

No web browser required. No mouse required. The keyboard is the entire interface.

---

## II. Terminal requirements

| Use case | Supported terminals |
|---|---|
| All workflows (text only) | Any VTE-based terminal; kitty, iTerm2, Ghostty, WezTerm, Alacritty |
| PDF viewing | kitty, iTerm2, Ghostty, WezTerm only |
| Full 24-bit color | Most modern terminals |

**Linux Mint:** kitty terminal recommended. Install: `sudo apt install kitty`
**macOS:** kitty (`brew install --cask kitty`), iTerm2, Ghostty, or WezTerm

---

## III. Starting os-console

```bash
# Start with default local profile
os-console

# Start with a specific profile
os-console --profile gce-native

# Offline mode (no backend services required)
os-console --profile offline
```

On startup, `os-console`:
1. Reads `~/.config/os-console/config.toml`
2. Initiates MBA connections to configured os-* peers
3. Renders the F-key tab strip and status bar
4. Activates the default cartridge (F4: Content)

---

## IV. The interface

### Status bar (bottom of screen)

```
jennifer@woodfine | MBA LINK ACTIVE | F4: Content | Tier A | 00:04:23
```

| Element | Meaning |
|---|---|
| `jennifer@woodfine` | Your identity (username@tenant) |
| `MBA LINK ACTIVE` | os-* service connection verified |
| `F4: Content` | Currently active cartridge |
| `Tier A` | SLM inference tier in use (A=local, B=cloud, C=frontier) |
| `00:04:23` | Session duration |

### F-key tab strip (top of screen)

One slot per F-key, F1–F12. The active cartridge is highlighted. Greyed slots are
not installed. Press the F-key to switch cartridges.

---

## V. F-key map

*Work in progress — assignments will evolve. F12 never changes.*

| F-key | Cartridge | What it does |
|---|---|---|
| F1 | Help | Key binding reference; press again to close |
| F2 | People | Identity and contact management |
| F3 | Email | Communications ledger |
| F4 | Content | Proofread existing text; draft new content |
| F5 | Minutebook | Governance: minutes, resolutions |
| F6 | Bookkeeper | Financial ledger entries |
| F7 | BIM | Building information management |
| F8 | GIS | Geographic information |
| F9 | SLM | AI adapter management and marketplace |
| F10 | Mesh | PPN network management |
| F11 | System | Live os-* service health; MBA pairing status |
| **F12** | **Input Machine** | **The Anchor** — mandatory ingest gate |

---

## VI. The Input Machine (F12)

F12 is The Anchor. Press it at any time, from any cartridge, to ingest a document.

**Workflow:**
1. Press F12 — input modal appears
2. Type the file path; press Enter
3. Confirm submission when prompted
4. The document is sent to `service-input` on the Totebox Archive
5. `service-input` classifies and routes it
6. The active cartridge resumes with the document in context

**F12 cannot be bypassed.** All document ingest goes through F12. This is a compliance
requirement (SYS-ADR-10). There is no drag-and-drop, no paste-without-confirm.

---

## VII. Content cartridge (F4) — core workflows

### Proofread
1. Press F12 to submit the document for proofreading
2. Or: switch to F4 and paste text directly into the input pane
3. Select a protocol from the fuzzy picker (type to filter; Enter to confirm)
4. Wait for the pipeline (300s timeout; spinner shows progress)
5. Review the diff pane: original left, improved right
6. Per-suggestion actions: `a`=accept, `r`=reject, `e`=edit, `A`=accept-all, `R`=reject-all
7. Session closes the apprenticeship loop automatically

### Draft new content
1. Press F4 to activate Content cartridge
2. Type `/new` to start a draft
3. Select a protocol from the fuzzy picker
4. Optionally type `/search <query>` to add entity context from the knowledge graph
5. Tier B generation begins; output streams in real time
6. Accept the draft: it is staged to `.agent/drafts-outbound/` with full research trail

### Slash commands (Content cartridge)
| Command | Action |
|---|---|
| `/new` | Start a new draft |
| `/search <query>` | Search the knowledge graph for entity context |
| `/regenerate` | Cancel current generation and retry |
| `/tier b` | Force Tier B (cloud burst) for next generation |
| `/tier c` | Force Tier C (frontier API) for next generation |
| `/status` | Show service health and current tier availability |
| `/audit` | View the verdict log for this session |
| `/export` | Write the current buffer to a file |

---

## VIII. MBA connection management

### When MBA LINK INACTIVE

If the status bar shows `MBA LINK INACTIVE`, os-console is in local-only mode.
Locally-cached content is accessible; backend requests will fail.

Common reasons and fixes:

| Message | Fix |
|---|---|
| `key not registered` | Run `proofctl user add` on the target service |
| `service unreachable` | Check that the target os-* service is running |
| `fingerprint mismatch` | Run `proofctl user rotate-key` |

For full pairing setup, see GUIDE: MBA Pairing Ceremony.

### Checking individual service states (F11)

Press F11 (System cartridge) for a dashboard showing:
- Each configured os-* peer and its MBA state
- Service health for ring services (service-proofreader, service-content, service-slm)
- Doorman tier availability

---

## IX. Configuration

Configuration file: `~/.config/os-console/config.toml`

```toml
[profile.default]
mode = "local"

[profile.local]
# Linux Mint / macOS — local terminal
totebox_endpoint = "http://localhost:9000"
slm_endpoint = "http://localhost:9080"

[profile.offline]
mode = "offline"
# No backend connections; deterministic-only mode
```

**Critical endpoint values (do not modify):**
- Doorman (SLM): `http://localhost:9080`
- service-proofreader: `http://127.0.0.1:9092`

---

## X. Keyboard reference

### Global (any cartridge)
| Key | Action |
|---|---|
| F1–F12 | Switch cartridge |
| F12 | Input Machine (ingest document) |
| F1 | Help overlay |
| `Ctrl-c` | Quit os-console |
| `Ctrl-l` | Redraw screen |

### Content cartridge (F4)
| Key | Action |
|---|---|
| `Tab` | Switch between input / diff panes |
| `a` | Accept current suggestion |
| `r` | Reject current suggestion |
| `e` | Edit current suggestion |
| `A` | Accept all suggestions |
| `R` | Reject all suggestions |
| `j` / `k` or `↓` / `↑` | Navigate suggestions |
| `PgDn` / `PgUp` | Scroll long documents |
| `/` | Enter slash command |

---

## XI. Troubleshooting

### The screen is garbled or blank
- Verify the terminal supports 24-bit color: `echo $COLORTERM`
- Try a different terminal from the supported list
- Press `Ctrl-l` to force a full redraw

### PDF viewing shows an error
- Your terminal does not support Kitty graphics protocol or Sixel
- Switch to kitty, iTerm2, Ghostty, or WezTerm
- There is no text-extraction fallback by design

### Slash commands have no effect
- Check service health: press F11 (System cartridge)
- Doorman may be offline; Tier A inference will be unavailable
- Check: `systemctl status local-doorman` on the GCE VM

### MBA LINK PENDING for more than 30 seconds
- The target os-* service may be starting up; wait and retry
- Check: `systemctl status local-<service>` on the target machine
- If the issue persists, run `proofctl user list` to verify the key is registered

---

## See also

- TOPIC: os-console and app-console-keys — platform architecture
- TOPIC: Input Machine (F12) — The Anchor; full architectural context
- TOPIC: Machine-Based Authorization — how MBA connections work
- GUIDE: MBA Pairing Ceremony — setting up os-console ↔ os-* connections
- `node-console-operator/guide-command-ledger.md` — the original operational reference
