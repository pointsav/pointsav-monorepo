---
artifact: guide
schema: foundry-draft-v1
title: "Getting Started with os-console"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-49]
research_trail:
  sources: [BRIEF-os-console-active-dev.md, BRIEF-os-console-hypervisor.md, os-console-main-rs]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: os-console Cargo.toml + config.example.toml + F-key map verified against source
---

# Getting Started with os-console

os-console is the keyboard-driven operator interface to your Totebox. This guide covers
installation, machine pairing, and first use for an operator with no IT background.

---

## What You Need

- **A Totebox running on your network.** This is the server that holds your data. It runs
  on hardware you control — a small server, a mini-PC, or a cloud VM. If you do not have
  a Totebox yet, contact your Totebox administrator.
- **A computer running Linux or macOS.** os-console runs on the machine you use daily.
  It does not require a dedicated machine in the current release. Minimum: 512 MB free RAM.
- **Network access to the Totebox.** Your computer must be able to reach the Totebox by
  IP address or hostname. A local network is sufficient; no public internet access required.

---

## Installation

**Current release (SSH tunnel mode):**

1. Download the os-console binary for your platform from your Totebox administrator.
2. Make it executable: `chmod +x os-console`
3. Run it: `./os-console --host <totebox-ip> --ssh-port 2222`

The terminal clears. The os-console TUI appears. The title bar shows the Totebox address.
If the connection fails, verify the Totebox is running and port 2222 is accessible.

**Planned release (VM image, Phase H2):**

Download the os-console VM image. Run it with your local hypervisor:
- macOS: `os-console.img` runs as a virtual machine application.
- Linux: `qemu-system-x86_64 -enable-kvm -m 512 os-console.img`

The VM boots in under 5 seconds. The TUI appears without any configuration.
The VM auto-discovers Toteboxes on the local network via mDNS.

---

## Machine Pairing

Before you can use cartridges, your computer must be paired with the Totebox. Pairing
authorizes your computer — not just your account — as a trusted participant.

1. Press **F11** to open the SystemCartridge.
2. Select **Pair this machine**.
3. A QR code appears on screen.
4. Your Totebox administrator scans the QR code from the Totebox operator panel.
5. The administrator approves the pairing request.
6. The cartridges you have been granted access to activate automatically.

The QR code contains your machine's fingerprint. After approval, your computer holds
authorization tokens for the specific cartridges the administrator granted. No username
or password is used. The machine itself is the credential.

**First time:** pairing may take up to 60 seconds while the Totebox verifies the request.
After the first pairing, os-console reconnects automatically on subsequent launches.

---

## Navigating os-console

os-console uses function keys to switch between cartridges. Press the key to open a
cartridge; press it again or press **Esc** to return to the chassis view.

| Key | Cartridge | What it does |
|---|---|---|
| **F2** | People | View and search personnel records |
| **F3** | Email | Browse the email archive |
| **F4** | Content | Search the knowledge graph |
| **F6** | Bookkeeper | Financial records and ledger entries |
| **F9** | SLM | Query the AI assistant (Doorman) |
| **F11** | System | Machine pairing, connection status |
| **F12** | Input | Submit documents and structured data |
| **F1** | Help | This keyboard reference |

Within any cartridge:
- **Arrow keys / j / k** — navigate lists
- **Enter** — open selected item
- **Esc** — go back / close panel
- **Tab** — move focus between panels
- **/** — search (where available)

---

## Pasting Content from Your Computer

**Keyboard paste (current):**
- macOS: **Cmd+V** in any cartridge input field
- Linux: **Ctrl+V** in any cartridge input field

The clipboard content from your host machine appears in the cartridge field.

**File input (planned, Phase H2):**
When the VM image release is available, a **Totebox Watch Folder** will appear on your
desktop. Drop any file into this folder — a document, a spreadsheet, an email export — and
os-console routes it to the appropriate cartridge as a structured form submission. No copy
and paste required.

---

## Common Issues

**Cartridges appear greyed out after pairing:**
The administrator may not have granted access to that cartridge. Press F11 → Status to
see which cartridges your machine is authorized for. Contact your administrator to request
additional access.

**Connection drops or shows "reconnecting":**
os-console reconnects automatically with exponential backoff (2 seconds to 60 seconds).
If the Totebox is temporarily unavailable, wait for it to come back online. The reconnect
watchdog handles this without operator action.

**"Address already in use" error on port 9093:**
Another process is using port 9093. Run `lsof -i :9093` to identify it. If it is a
previous os-console instance that did not shut down cleanly, kill it before launching again.

**Screen displays garbled characters:**
os-console requires a terminal that supports truecolor (24-bit color) and UTF-8. If your
terminal does not support these, launch os-console with `--plain` flag for a simplified
display that works on any terminal.

---

## Shutting Down

Press **Ctrl+Q** or close the terminal window. os-console sends a graceful shutdown
signal to all cartridges before exiting. Your pairing remains valid; the next launch
reconnects automatically.

If os-console is running as a VM, shut it down from within: **F11 → Shutdown VM**.
This saves the connection state cleanly before the VM halts.
