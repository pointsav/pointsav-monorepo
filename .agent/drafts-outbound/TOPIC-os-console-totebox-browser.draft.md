---
artifact: topic
schema: foundry-draft-v1
title: "os-console: The Totebox Orchestration Browser"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-49, claim-43, claim-34]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, BRIEF-OS-FAMILY.md, BRIEF-sovereign-os-family-master-plan.md]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: agent-research + os-console Cargo.toml + cartridge source review
---

# os-console: The Totebox Orchestration Browser

os-console is the operator terminal surface for Totebox Orchestration. It runs on the
operator's host machine — a personal computer, workstation, or thin client — and presents
a keyboard-native interface to the services running on the operator's Totebox.

The closest analogy is a web browser. The analogy is structural, not metaphorical:
os-console and a browser solve the same architectural problem by the same means.

---

## The Browser Analogy

| Web Browser | os-console |
|---|---|
| Renders HTML from web servers | Renders TUI cartridge views from Totebox services |
| Browser tabs — isolated renderer processes | F-key cartridges — seL4 Protection Domains (planned Phase H2) |
| Certificate store — trusted server identities | Machine pairing (F11) — host machine as trust anchor |
| HTTP + DNS — universal transport protocol | Totebox service protocol — cartridge-to-service contract |
| Service Workers — offline cache | Offline cartridge cache (planned Phase H3) |
| Same-origin policy — tab isolation | seL4 capability boundary between cartridge PDs (planned) |
| Your ISP's network | Your Totebox — sovereign, on-premises, no cloud dependency |
| Browser as installed application | os-console as bootable VM image (planned Phase H2) |
| Browser extensions | Future: operator-registered cartridges |

The key distinction from a web browser: os-console connects to hardware under the
operator's physical control. The Totebox is not a cloud service. The data does not
leave the operator's premises.

---

## F-key Cartridges as Browser Tabs

A browser tab is a renderer process with a defined origin. It can reach resources from
its origin and cannot read memory from other tabs. The browser enforces this boundary.

In os-console, each F-key cartridge is a distinct application:

| Key | Cartridge | Service |
|---|---|---|
| F2 | People | service-people (:9091) |
| F3 | Email | service-email |
| F4 | Content | service-content (:9081) |
| F6 | Bookkeeper | service-bookkeeper |
| F9 | SLM | service-slm (:9080 Doorman) |
| F11 | System | pairing-server |
| F12 | Input | service-input (audit anchor) |

In the planned seL4 Microkit substrate (Phase H2), each cartridge runs as a seL4
Protection Domain. A PD cannot read another PD's memory. The seL4 kernel enforces
this boundary with the same guarantee a browser enforces same-origin: formally, not
by convention.

A compromised F9 (SLM) cartridge cannot read the People cartridge's data. Not because
there is a firewall between them — because there is no seL4 capability path.

---

## Machine Pairing as the Certificate Store

A browser's certificate store establishes which servers are trusted. When you connect to
a server over HTTPS, your browser checks the server's certificate against the trust store.

Machine pairing (F11) is the Totebox equivalent. The F11 SystemCartridge presents a QR
code. The Totebox operator scans it. The Totebox issues capability tokens for the
authorized cartridge services. From that point, the host machine is authorized — not the
user account, the machine.

This is the intended machine-level identity anchor (planned Phase H3 capability model).
The current implementation uses SSH key fingerprints. The seL4 capability model replaces
this with formally bounded capability tokens.

Revocation works the same way as certificate revocation, but faster: removing a machine
pairing on the Totebox propagates immediately. The next request from that os-console
instance returns an authorization failure. There is no grace period.

---

## Beyond Clipboard: Structured Interaction

A web browser does not use clipboard to interact with a website. You submit a form. You
attach a file. You drag a component. The interaction is structured.

os-console is designed toward the same model. The clipboard (Cmd+V / Ctrl+V via arboard,
planned Phase H1) is a baseline. The intended structured interaction surface is VirtIO-fs:

1. The operator's host machine has a **Totebox Watch Folder** — a local directory mounted
   into the os-console VM via VirtIO-fs.
2. The operator drops a file into the Watch Folder.
3. The appropriate cartridge (Proofreader, Content, Input) detects the file, reads it as
   structured input, and presents it as a form submission rather than raw text paste.
4. The file is never transmitted as bytes through the clipboard. It is addressed as a
   capability-gated input to a specific cartridge form.

This is planned for Phase H2. Phase H1 delivers VirtIO clipboard (host→guest paste) as
the baseline for SMB operators who need to copy content from their host applications.

---

## One Totebox or Many: os-orchestration as Multiple Websites

A browser can display content from multiple websites simultaneously. os-orchestration
is the Totebox equivalent of a multi-site aggregator.

When an operator's organization operates multiple Toteboxes — one per office, or one per
legal entity — os-orchestration federates them. The operator's os-console session can
display cartridge views from multiple Toteboxes without switching connections. Each
Totebox is an independent data vault with its own capability namespace. os-orchestration
holds derived capability tokens from each.

This is analogous to a browser showing multiple websites in different tabs. Each website
is independent. The browser holds session credentials for each. The browser cannot let
Website A read Website B's cookies.

---

## The Bootable Browser

A web browser is not an OS. But it runs as a privileged process, manages its own process
tree, enforces its own security model, and interprets its own protocol. For most
operators, the browser is the primary application surface.

The intended final form of os-console (planned Phase H2) is a bootable VM image. The
operator boots it — on a dedicated machine, inside their existing OS as a VM, or as a
virtual appliance. The image contains the seL4 Microkit kernel, the os-console cartridge
PDs, the VirtIO drivers, and nothing else. There is no general-purpose OS underneath it.
No shell. No package manager. No attack surface that the operator did not approve.

For the SMB operator with no IT department: download the image, run it, scan the QR code.
The Totebox browser is running.

---

## Non-Technical Operator Experience

The design intention for os-console is that it requires no technical knowledge to operate:

1. **Boot**: the image launches (on hardware or as a VM); the TUI appears within seconds.
2. **Discover**: os-console auto-discovers Toteboxes on the local network or connects to
   a configured address.
3. **Pair**: F11 shows a QR code; the Totebox admin scans it; cartridges activate.
4. **Use**: F-keys navigate between People, Email, Content, Bookkeeper, SLM.
5. **Paste**: Cmd+V / Ctrl+V works in any cartridge. Drop files into the Watch Folder.
6. **No configuration**: no IP addresses to type, no ports to remember, no SSH keys to
   manage. The machine pairing token handles authentication.

The Totebox operator does not need to understand seL4 capability theory. The operator
needs to know: scan the QR code to pair, press F11 to manage pairings, press F-keys to
use cartridges.
