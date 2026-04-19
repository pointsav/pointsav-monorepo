# CLAUDE.md — service-extraction

You are Claude Code working on the `service-extraction` component of the PointSav Digital Systems platform. Read this file first at the start of every session in this directory.

---

## Project identity

`service-extraction` is one of the nine services inside a Totebox Archive, the core operating system PointSav Digital Systems builds for Woodfine Management Corp. This service reads raw `.eml` files and writes structured records. No AI in this service — AI work happens in `service-slm`, which is an optional upgrade tier.

Full platform context: see `../../USER_GUIDE.md` or the Development Memo in the monorepo root.

## Current state

- **v0.2** exists and works. 571-line `src/main.rs`, regex-based, validated qualitatively against 10 real `.eml` files in `samples/`.
- **v0.4** is in active development. Target: >90% extraction fidelity against `samples/*.eml`, adding four techniques to the v0.2 baseline:
  - Aho-Corasick gazetteer (loads `crm_contacts.csv` + embedded first-name and org-suffix lists)
  - Cognitive Gravitation Model classifier (Wen et al. 2013, physics-based)
  - Shannon entropy filter (rejects boilerplate)
  - Signature block detection (structural, not SVM)

Read `ROADMAP.md` for the full technique inventory and phase plan. Read `VALIDATION.md` for the test corpus and scoring methodology.

## Build and test

```
./build.sh
```

Produces `target/debug/service-extraction` (native) and `target/x86_64-unknown-linux-musl/release/service-extraction` (deploy to commodity cloud nodes).

**Quick test against the sample:**

```
mkdir -p /tmp/test-totebox
./target/release/service-extraction samples/sample.eml /tmp/test-totebox
ls /tmp/test-totebox/service-fs/data/service-people/source/
```

**Full corpus validation** — see `VALIDATION.md`. The scoring script is part of v0.4 development.

## File layout

```
service-extraction/
├── Cargo.toml         — crate manifest
├── build.sh           — native + MUSL build
├── README.md          — public-facing, English
├── README.es.md       — public-facing, Spanish
├── ROADMAP.md         — technique inventory + phase plan (read before adding features)
├── VALIDATION.md      — per-version measurements (update after every behaviour change)
├── CLAUDE.md          — this file
├── src/
│   └── main.rs        — v0.2 working baseline, 571 lines
├── samples/
│   ├── sample.eml           — synthetic business email
│   ├── *.eml                — 10 real test emails
│   └── expected.yaml        — ground-truth targets (to create during v0.4)
├── scripts/           — (to create) validation scoring scripts
└── target/            — cargo output, git-ignored
```

## Hard constraints — do not violate

1. **No AI, no LLM, no NER, no embeddings in this service.** AI processing is `service-slm`'s job. This service must run on the commodity $7 cloud node with zero AI dependency.

2. **Output goes to the WORM ledger pattern** under `<totebox_root>/service-fs/data/`:
   - Raw bytes → `service-fs/data/service-extraction/source/`
   - Extracted record index → `service-fs/data/service-extraction/ledger/`
   - Personnel records → `service-fs/data/service-people/source/`
   - Clean body text → `service-fs/data/service-content/source/`

3. **Point-in-time execution only.** One `.eml` in, records out, process exits. No daemon, no persistent state, no long-running connections.

4. **Crate licences must be Apache 2.0 or MIT compatible.** No GPL. No proprietary SDKs.

5. **Extractions must be deterministic.** Same input → same output byte-for-byte. No wall-clock timestamps in JSON record bodies. No random IDs — use content-addressed hashes or monotonic counters.

## Canonical names (do not substitute)

- `service-extraction` — this service. Not `service-parser` (legacy name persists in some older code but is retired for new work).
- `service-email`, `service-input` — the two ingress services that feed this one.
- `service-fs` — the WORM kernel. All durable output goes through it.
- `service-people`, `service-content` — downstream services we write to.
- `service-slm` — the AI Doorman. This service does NOT depend on it.
- `ToteboxOS`, `ToteboxArchive`, `PersonnelArchive`, `CorporateArchive`, `PropertyArchive` — platform terms.

## Language rules

When writing docs, comments, log messages, or error strings in this repository, these terms are explicitly out of bounds:

- **"Leapfrog 2030"** — AI-generated marketing from prior sessions. Retired.
- **"Cognitive Forge"** — same. Retired.
- **"Sovereign"** as a descriptive adjective (e.g. "sovereign pipeline"). The proper noun "Sovereign Data Foundation" is retained until formally renamed.
- **"Tri-State Vault"**, **"Unicode Enclave"**, **"Linguistic Air-Lock"**, **"Fiduciary target"**, **"Data Vault"** — all AI-generated grandiose prose from prior sessions. Retired.

Write plain, precise technical prose. Target audience: institutional investors, new developers. The Bloomberg standard — if it wouldn't appear in a Bloomberg article on financial infrastructure, rewrite it.

## BCSC disclosure rule

Woodfine Capital Projects Inc. ultimately owns both PointSav Digital Systems and Woodfine Management Corp. The Sovereign Data Foundation is an **intended** equity holder and planned overseer — this arrangement has **not** been formally executed. Any public-facing documentation written in this repo must reference the Foundation in planned/intended terms only, never as a current equity holder or active body.

## Working style in this directory

1. **Before adding any feature, read `ROADMAP.md`.** If the feature is in Core, add it. If it is in Next Tier or Deferred, do not add it without explicit user instruction and without first updating `VALIDATION.md` to document the gap the feature closes.

2. **After every behaviour change, measure.** Run the sample against the current corpus, update `VALIDATION.md` with a new version entry. Append, never overwrite.

3. **Commit frequently with small, focused messages.** One technique per commit where possible. Commit message format: `service-extraction: <what changed>` — e.g., `service-extraction: add Aho-Corasick gazetteer loader`.

4. **When uncertain about scope, ask the user.** Do not expand scope silently. The standing pattern in this project is: do the minimum that closes the documented gap, measure, then decide.

5. **Never delete sample `.eml` files.** The corpus is the ground truth. Extending the corpus is welcome; removing it is not.

6. **Never commit secrets.** No OAuth tokens, no API keys, no personal email content from real mailboxes unless explicitly anonymised. The `samples/` directory is the exception — those emails are the agreed test corpus.

## If you get stuck

- Build errors: read the full `cargo build` output carefully. Rust compiler errors are usually very specific.
- Behaviour unclear: check `ROADMAP.md` for the intended behaviour of each technique.
- Phase question: check `ROADMAP.md` *Phase plan* and *Stop conditions* sections. If the answer is not there, stop and ask the user.
- Naming question: check this file and the project instructions. If a term is not listed here as canonical or retired, default to the simplest plain-English term.
