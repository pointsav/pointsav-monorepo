---
schema: foundry-plan-v1
archive: project-console
topic: pairing-ceremony-ux-design
created: 2026-05-21
updated: 2026-05-23
status: archived
cites: []
---

> **ARCHIVED 2026-05-23.** Pre-implementation research. Pairing Phases 1–2 shipped
> (d6267e39, 30874995). §6 (operator journey), §9 (error copy), and vocabulary table §4
> referenced by `BRIEF-pairing-phase3-4.md` and `BRIEF-pairing-ceremony.md`.

# Pairing Ceremony — UX Design Brief

> A device-pairing experience for `os-console`. A design brief for the UI designer who will implement the screens.
> Research by Opus UX design agent, 2026-05-21.

## 0. The one-sentence brief

**Make connecting a new computer to the workspace feel like pairing a pair of Bluetooth headphones — a short, calm, slightly delightful ceremony with a clear beginning, a patient middle, and a small celebration at the end — and never once make the user think about cryptography.**

---

## 1. The problem we are solving

`os-console` is a terminal application. Before anyone can use it, their computer must be cryptographically linked to the remote service (`os-totebox`). Today that link is established by hand: the user hunts for an SSH public key file, sends it to an administrator, the admin runs `proofctl user add jennifer --tenant woodfine --key-file jennifer.pub`, tells the user, the user restarts the app, and `MBA LINK ACTIVE` appears.

The current pairing screen (`app-console-keys/src/chassis.rs::render_pairing_screen`) literally prints the `proofctl` command and the words "fingerprint", "key-file", and "id_ed25519.pub" on screen, and asks the user to restart the app to find out whether it worked. It is honest, but it is an engineering artifact, not a product. **The pairing ceremony is the first thing every user experiences — it must be the most polished screen we ship, not the least.**

**Engineering invariants the design must respect:**
- A keypair exists on the user's machine and the public half must reach the operator
- The operator must take a deliberate approval action ("Pairing as Permission" — no separate roles DB)
- Link state has exactly three values — `Active`, `Pending`, `Inactive(reason)` (per `widgets/status_bar.rs::MbaStatus`)
- The `MBA LINK ACTIVE` status-bar label stays for operators but the user flow does not lead with the acronym

---

## 2. User personas

- **Mathew** — Linux Mint, technical, knows what SSH *is* but is not a security specialist; will tolerate one extra line of explanation, not a five-step manual procedure. Risk: will "help" by doing it the hard way if the easy way looks uncertain.
- **Jennifer** — macOS 13, professional, has never generated an SSH key and should never have to; expects guided, progress-bearing pairing like AirPods or bank-app Face ID. Risk: any raw command/path/acronym reads as "this is broken / not for me" and she stops.
- **Peter** — macOS current (Apple Silicon), same profile as Jennifer; may pair a second machine later. Risk: re-pairing must not feel like a fault.
- **The Operator** — separate administrator, runs `os-totebox`, technical but busy and not watching for requests. Needs to (a) know a request exists and (b) approve in seconds. Their friction is the user's frustration.

---

## 3. Design principles

1. Pairing is a ceremony, not a form — curtain-up, calm middle, curtain-down, each its own full screen.
2. The user never types a command and never touches a file path — the app moves values, or the user reads a short human code aloud.
3. No security vocabulary in the primary flow. Banned: *SSH, key, keyfile, public/private key, fingerprint, hash, cryptographic, ed25519, certificate, RSA, .pub, token, credential.* (Status bar keeps `MBA LINK ACTIVE`; F1 Help may carry one "Advanced / how this works" panel with real terms for Mathew.)
4. Waiting is designed, not defaulted — a first-class screen with copy, motion, reassurance.
5. The app tells the user when it worked — no "restart to find out"; it polls and transitions itself.
6. Every dead end has a way out — every error names one concrete action and a retry.
7. The operator's approval is one keystroke or one click.

---

## 4. Naming — the words we commit to

| Concept | We call it | We never call it |
|---|---|---|
| The act of linking a computer | **Connecting** / "connect this computer" | pairing ceremony, enrolment, provisioning |
| The established link | **Connected to your workspace** | MBA link (user-facing), SSH session |
| The remote service (`os-totebox`) | **your workspace** | the server, the host, os-totebox |
| The value the operator needs | **your Connection Code** | fingerprint, public key, SSH key |
| The waiting period | **waiting for approval** | pending, INACTIVE, handshake |
| The operator | **your administrator** | the operator, IT, sysadmin, root |
| The operator's approval | **approve** / "Approve this computer" | register key, run proofctl, add user |
| Success | **You're connected** | MBA LINK ACTIVE (user-facing) |
| Re-doing it on a new machine | **Connect a new computer** | re-pairing, key rotation, rotate-key |

**Ceremony name:** "Connect to your workspace" (recommended) or "Workspace Setup". The word "pairing" is fine in casual support speech but the *screens* say "connect / connected."

**The Connection Code** — present in two interchangeable forms, user picks:
- **A QR code** rendered via the Kitty graphics protocol (the AirPods/WhatsApp Web moment)
- **A six-character code** (e.g. `K7Q2-9XMT`) to read aloud or paste into chat

Both encode the same rendezvous token. The user is told *"Show this code to your administrator — by QR, or just read the code out,"* and never what it *is*.

---

## 5. The user's journey — four screens

### Screen 1 — Welcome (first launch, never connected)

Calm centred panel, workspace glyph, dimmed F-key strip.

```
                    Welcome to os-console

       Let's connect this computer to your workspace.

   It takes about a minute. You'll show a short code to
   your administrator, they'll approve it, and you're in.

                  [ Enter ]  Start

             New here? Press  ?  for help.
```

"About a minute" sets the duration expectation — the single most effective anti-frustration line in the flow.

### Screen 2 — Your Connection Code

QR rendered large and centred via Kitty graphics; code displayed prominently.

```
               Show this to your administrator

              [ QR CODE IMAGE — large, centred ]

        Can't scan it? Read this code instead:
                      K7Q2 · 9XMT

   Your administrator will approve this computer. As soon
   as they do, this screen will move on by itself —
   you don't need to do anything else.

            [ Enter ]  I've shared the code →
```

"This screen will move on by itself" kills the "restart the app" anti-pattern.

**Degradation:** no Kitty graphics → Unicode half-block QR; unreadable at terminal size → promote the code to hero, never show a broken image box.

### Screen 3 — Waiting for approval (Pending)

Gentle slow ambient animation (~1.5–2s cycle — slow reads as "patiently waiting," fast spinners read as "might fail"). Rotating reassurance copy (~8s/line) so the screen visibly breathes.

```
              Waiting for your administrator…

                       ◌  ◍  ◌

   We've let your administrator know. They'll approve
   this computer when they get a moment.

   • This usually takes a minute or two.
   • You can leave this screen open — we'll catch the
     approval the moment it happens.
   • Still here? You can show your code again with R.

            R  Show my code again
            ?  Help        Q  I'll come back later
```

After ~3 min add one calm (non-error) line about the admin being away; `Q` is "I'll come back later," not "Cancel" — quitting is safe and resumable.

**The only place the emotional journey can crater is the waiting state** — every Screen 3 decision exists to keep that line flat.

### Screen 4 — Connected (Active)

~1.5s success animation: checkmark draws in, workspace glyph lights up, F-key strip un-dims one key at a time left-to-right (handing the user their tools), then auto-advances into the app (F4).

```
                          ✓

                   You're connected.

          This computer is now linked to your
             workspace. Everything's ready.

            Opening your console…
```

No "click to continue" — the transition is the celebration. Status bar updates to `MBA LINK ACTIVE`; first-time users get a one-time auto-dismissing tooltip: *"Connected — this is your workspace link. Green means good."*

---

## 6. The operator's journey

Operator learns of a request via: (1) a pending-request card in their own `os-console` F11 / `app-console-system` cartridge with a status-bar badge `1 request waiting`; or (2) out-of-band notification (mailbox/email/chat).

```
  Pending connection requests                       1 waiting
  ┌──────────────────────────────────────────────────────┐
  │  Jennifer  ·  woodfine                                │
  │  Requested 2 minutes ago                              │
  │  Code:  K7Q2 · 9XMT                                  │
  │   [A] Approve     [V] View details     [X] Decline    │
  └──────────────────────────────────────────────────────┘
```

**Approve is one keystroke: `A`.** "View details" expands the technical view (real fingerprint, tenant, role, "this runs `proofctl user add`") — available, never required. The human cross-check (does the code Jennifer read me match the card?) is the actual security act and needs zero jargon.

Confirmation: *"Jennifer can now connect. We've told her console — her screen will move on by itself."*

Target: badge-to-approved in **under ten seconds**.

---

## 7. Waiting state design rules

- Always show proof of life (animation + rotating copy)
- **Never a fake progress bar** (indeterminate ambient motion only)
- Set the expectation early and reinforce gently
- Escalate slowly: 0–3min normal, 3–10min "admin may be away, code stays valid," >10min offer `N` to re-nudge
- Quitting mid-wait is safe and resumable — relaunch returns to Screen 3 if still pending, or straight to Screen 4 if approval landed while closed
- The app polls; "restart" never appears in the flow

---

## 8. The `MBA LINK ACTIVE` problem

Keep the label, teach it once, lead with colour + a plain word.

Status bar: `● Connected · MBA LINK ACTIVE` (green), `● Connecting…` (blue), `▲ Not connected` (yellow).

The success screen says only "You're connected." A one-time tooltip explains the bar token.

**Open question:** keep the acronym de-emphasised, or hide it behind an operator/advanced mode — needs an operator decision (recommendation: keep, de-emphasised). Colour must never be the only cue (see §12).

---

## 9. Error states (actual copy)

**E1 — Declined:**
> "Connection not approved"
> "Your administrator didn't approve this computer. This is usually a quick mix-up — the wrong code, or they want to check with you first. Talk to your administrator, then try again."
> → `Enter` Show my code again.

**E2 — Timed out:**
> "Still waiting — let's pause here"
> "We haven't heard back yet. They may be away from their desk. Your code is still valid."
> "Nothing was lost — pick this up whenever it suits you."
> → `R` try again, `N` reminder, `Q` later.

**E3 — Unreachable:**
> "Can't reach your workspace"
> "We couldn't connect right now. This is almost always a network hiccup, not a problem with your computer. Check you're online; if on a VPN make sure it's connected."
> → `R` try again.

**E4 — Connection removed:**
> "This computer needs to reconnect"
> "Your access from this computer was reset — maybe a new login, or your administrator refreshed things. It's a two-minute fix."
> → routes into re-pairing; must not look like a bug.

**E5 — Catch-all:**
> "Something went wrong"
> "We hit an unexpected problem while connecting. It's not something you did."
> → `R` retry, plus a copy-pasteable reference code (`CONN-7F3A`); never a stack trace.

Rules: never display raw errors, paths, `proofctl`, or security words; always one sentence of *what*, one reassuring *why*, one *concrete action*; every screen has a retry; every screen carries "it's not something you did."

---

## 10. Re-pairing and new computers

Re-pairing **reuses the exact same four screens** as first-time pairing — one ceremony, learned once.

A brand-new computer is simply a first launch (Screen 1); if we can tell the person already has a connected computer, the subtitle warms to "Welcome back, Peter. Let's connect this computer too — same quick steps as last time."

A lost connection shows E4, which routes into the normal flow framed as maintenance.

The F11 System cartridge offers "Connect a new computer" / "Reconnect this computer" as plain, unalarming settings entries. The operator's card tags returning users `↻ returning` with one-line context ("Peter already has 1 connected computer") so they make a fast informed call.

**Do not build a separate "recovery" flow** — a parallel recovery flow is itself what makes re-pairing feel like a bug.

---

## 11. Analogues studied

| Analogue | Steal | Avoid |
|---|---|---|
| **Bluetooth / AirPods** | AirPods' one-tap simplicity; slide-up card as a discrete ceremony moment | Generic device-name ambiguity (we name the *person*); silent ambiguous success of a list entry changing |
| **WhatsApp Web QR** | QR as hero; display-here / approve-there split (our exact model); auto-transition into the app on success | Silent QR expiry — our code stays valid; if it must expire, say so loudly + offer refresh |
| **Nintendo Switch local play** | Full-screen, friendly, *this-is-an-event* framing; the satisfying "click" → our Screen 4 | Open-ended "searching" with no escalation guidance |
| **Banking app Face ID enrolment** | Reassuring plain-language "why"; explicit "You're all set" closer; trust-building tone | Wordiness; consent-screen sprawl — our flow is four screens, not ten |
| **Chromecast / Apple Watch pairing** | Human-named targets; pairing can carry genuine craft + delight | Chromecast's dead-end "nothing found"; Apple Watch pairing's *length* — keep ours short |

Cross-cutting principles from all analogues:
- Display device ≠ approving device; name the target as a human/place not a hex string
- Success must be a felt moment
- Waiting needs motion + an escalation path
- Silently-expiring codes kill trust
- Short "why" then act; never a dead-end failure

---

## 12. Accessibility

- **Colour:** never rely on hue alone (~8% of men cannot separate red/green) — every state also carries a glyph (`●`/`◌`/`▲`) and a word (`Connected`/`Connecting…`/`Not connected`). Test against default dark, default light, Solarized Light, and 16-colour terminals.
- **Screen readers:** the QR is invisible to a screen reader — the **8-character code must always be present as real text**, never QR-only. Announce state transitions in text — Screen 4 must render "You're connected" as text the instant it appears.
- **Low vision:** legible from 80×24 up to large terminals; if the QR cannot render at a scannable size, auto-promote the code and drop the QR; generous whitespace, one focal point per screen; respect `NO_COLOR` / monochrome.
- **Motion sensitivity:** honour a reduced-motion preference — static `Waiting…` with gentle text rotation; success screen shows final `✓` state directly.
- **Input:** every action keyboard-reachable and labelled on screen (`Enter`, `R`, `N`, `Q`, `?`, `A`, `X`, `Ctrl-C`); never require timed input.

---

## 13. Open questions for engineering and design

1. `MBA LINK ACTIVE` in the user build — keep de-emphasised or hide behind an operator/advanced mode? (operator decision)
2. Connection Code word-list / format — who owns vocabulary (no homophones, phone-safe)? Bilingual concern given workspace bilingual rules.
3. Can the workspace emit a "request seen/claimed" signal so Screen 2 auto-advances to Screen 3 without a button press?
4. Operator notification channel — is the in-console F11 badge enough, or also out-of-band?
5. QR rendering quality across Kitty / iTerm2 / Ghostty / WezTerm / VTE — prototype at 80×24; confirm the code-fallback is always usable.
6. Does silent first-run keypair generation ever need a "Preparing this computer…" beat?
7. Sound — a single soft success bell on Screen 4? Default: no sound.
8. Multi-workspace future — "your workspace" (singular) is a deliberate v1 choice; will need revisiting.
