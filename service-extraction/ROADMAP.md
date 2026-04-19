# service-extraction — Roadmap

**Last updated:** 2026-04-18
**Author:** Mathew + Claude (design session)
**Read this before adding features.**

---

## Purpose of this document

This file defines the technique inventory for `service-extraction` and the phase gates that govern when each technique moves from research into production. It answers three questions for anyone picking up this codebase:

1. **What are we trying to extract?** — see *Gap analysis* below.
2. **Which techniques ship now, later, or never?** — see *Phase plan* and *Technique inventory*.
3. **When can we move the next technique out of deferred?** — see *Stop conditions* on each phase.

If you are Claude Code reading this at the start of a session, also read `CLAUDE.md` and `VALIDATION.md` before starting work.

---

## The gap `service-extraction` is closing

### What v0.2 already extracts (preserved in v0.4 unchanged)

- Email headers — `From`, `To`, `Cc`, `Date`, `Reply-To`, `Return-Path`, `Message-ID`
- Sender display name and email address
- Structured signature blocks (tested end-to-end on the PPCmetrics / Melina Herzig sample)
- Tracking URL filtering (125 dropped across the 10-email corpus)
- Newsletter classification (10/10 correctly flagged)
- Multi-language names: Czech "Mariana ze Stavby", German, French diacritics
- Body-mentioned email addresses when embedded in clean text (caught `manager.relations@ppcmetrics.ch` and `jobs@tldr.tech` that v0.1 missed)

### What v0.2 misses (the >90% target)

- **Names mentioned inside prose body** — the single biggest gap. Example: "Ryan Rumsey" (the body-prose guest) in the `ask_Ryan_anything.eml` interview thread, sent by Chris Nguyen of UX Playbook. v0.2's regex approach cannot distinguish a name from surrounding prose.
- **Multi-word company names in prose** — "DAAily Platforms AG", "Schloss Elmau" when referenced mid-sentence rather than in a signature.
- **Titles not adjacent to their person** — "...spoke with the CEO, who has been in the role since 2019, about..." where the title and name are separated by a clause.
- **Partial signatures** — senders who include a role but not a full formal block.

The >90% target is measured against the 10-email corpus in `samples/`. See `VALIDATION.md` for the ground-truth definition and scoring methodology.

---

## Phase plan

### Core — v0.4 — target >90% on the 10-email corpus

Techniques **#1 through #5** below. These are the techniques with the highest ratio of gap-closing power to implementation complexity against the current test set.

**Stop condition for v0.4 Core:** measure against `VALIDATION.md` corpus. If v0.4 Core hits >90%, freeze. Do not proceed to Next Tier until v0.4 has processed real `maildir` data for at least 30 days AND there is a documented gap that Next Tier closes.

### Next Tier — v0.5 — scale and disambiguation

Techniques **#6 through #9**. These are useful but not critical for the 10-email corpus. They become valuable at larger scale — Woodfine's 1,700-contact CRM plus production incoming mail.

**Stop condition for v0.5:** do not begin work on v0.5 until all of the following are true:
- v0.4 Core has been in production for at least 30 days.
- A documented gap exists that v0.5 closes, with example inputs from real `maildir` data.
- `VALIDATION.md` has been extended with a scale-tier test corpus of at least 200 `.eml` files drawn from actual production ingress.

### Deferred — scale optimisation, PDF pipeline, OCR

Techniques **#10 through #15**. These are either pure optimisation (XOR filters) or address document types not in the current test set (PDFs, scanned documents).

**Stop condition for Deferred:** do not begin any Deferred technique without a specific, documented production need. PDFs require an institutional customer's actual PDF workflow; OCR requires hardware above the $7 base node tier anyway.

---

## Technique inventory

### #1 — mailparse + html2text + regex
- **Status:** v0.2 baseline, preserved unchanged in v0.4 Core.
- **Crates:** `mailparse`, `html2text`, `regex`.
- **Gap closed:** headers, sender identity, tracking URL filtering, structured signature detection.
- **RAM:** <1 MB.

### #2 — Aho-Corasick gazetteer
- **Status:** v0.4 Core — to add.
- **Reference:** Aho & Corasick, *Efficient String Matching: An Aid to Bibliographic Search*, CACM 1975.
- **Crate:** `aho-corasick` (BurntSushi).
- **Gap closed:** body-prose name matching against the 1,700-row CRM + embedded ~100K first-name + ~10K org-suffix gazetteers. O(n) scan, matches millions of patterns simultaneously.
- **This is the largest single gap-closer for reaching >90%.**
- **RAM:** ~10 MB compiled automaton.
- **Gazetteer source:** `crm_contacts.csv` (real, 1,700 verified contacts). Embedded at build time with `include_bytes!`.

### #3 — Cognitive Gravitation Model
- **Status:** v0.4 Core — to add. Python prototype (`gravity_engine_prototype.py` from the design session) validated the approach against the old `people.db`.
- **Reference:** Wen, Wei, Wang, Zhou, Chen — *Cognitive Gravitation Model for Classification on Small Noisy Data*, Neurocomputing 2013. Newton's `F = G·m₁·m₂/d²` with self-information `I(x) = -log₂ P(x)` replacing mass.
- **Implementation:** in-tree, ~300 LOC.
- **Gap closed:** replaces v0.2's ad-hoc title-case + verb heuristic with self-information-weighted physics. Disambiguates "Peter Woodfine" (real name, high mass) from "Across The" (not a name, low mass). Every decision produces a traceable force diagram suitable for audit.
- **RAM:** <1 MB.

### #4 — Shannon entropy boilerplate filter
- **Status:** v0.4 Core — to add.
- **Reference:** Shannon, *A Mathematical Theory of Communication*, Bell System Technical Journal 1948.
- **Implementation:** in-tree, ~100 LOC.
- **Gap closed:** rejects boilerplate — "unsubscribe", privacy policies, legal footers, cookie notices — by penalising low-entropy regions. Multiplies signal mass by `(1 − boilerplate_penalty)`. Real human prose has higher per-byte entropy than repetitive legal boilerplate.
- **RAM:** negligible.

### #5 — Signature block detection (structural, not SVM)
- **Status:** v0.4 Core — to add.
- **Reference:** Mailgun Talon library (approach reference; we implement a simpler structural version without SVM training in v0.4).
- **Implementation:** in-tree, ~150 LOC. Structural heuristics (consecutive short lines after a sign-off token, repeated punctuation patterns, contact-information density) plus tightened regex.
- **Gap closed:** precise signature block boundaries. v0.2 uses a simpler heuristic that occasionally cuts too short or too long.
- **RAM:** <1 MB.
- **Why not the full Talon SVM in v0.4:** SVM training requires a labelled corpus. We don't have one yet. Structural rules will get us close enough for the 10-email test set. SVM is v0.5 or v0.6.

---

### #6 — Mosteller-Wallace function-word stylometry
- **Status:** Next Tier (v0.5) — deferred.
- **Reference:** Mosteller & Wallace, *Inference and Disputed Authorship: The Federalist*, Addison-Wesley 1964.
- **Implementation:** in-tree, ~200 LOC.
- **Gap closed:** 25-dimensional author fingerprint built from function-word frequencies. Detects "this message is probably from Peter" even when the signature differs or is missing. Cosine similarity in 25-dim space.
- **Why v0.5 not v0.4:** the 10-email test corpus does not stress author fingerprinting — each email is from a different sender. Stylometry becomes valuable with repeated senders and larger volume.

### #7 — Normalized Compression Distance (NCD)
- **Status:** Next Tier (v0.5) — deferred.
- **Reference:** Cilibrasi & Vitányi, *Clustering by Compression*, IEEE Transactions on Information Theory 2005.
- **Crate:** `flate2` (any compressor works; gzip is sufficient).
- **Gap closed:** parameter-free similarity. Near-duplicate email detection. Same-sender detection across address changes. No training data needed. Language-agnostic.
- **Why v0.5 not v0.4:** more valuable at scale. The 10-email corpus has no duplicates.

### #8 — Betweenness centrality for merge veto
- **Status:** Next Tier (v0.5) — deferred.
- **Reference:** Freeman, *A Set of Measures of Centrality Based on Betweenness*, Sociometry 1977.
- **Crate:** `petgraph`.
- **Gap closed:** prevents false entity merges. Before merging two candidate identities, compute the betweenness of the proposed edge on the entity graph. If the edge bridges otherwise-separate clusters (high betweenness), the merge is rejected and flagged for F12 operator review. Stops "Peter Woodfine" from fusing with "Peter Wuolfe" because they share one mistyped email.
- **Why v0.5:** matters at thousand-entity scale; the 10-email test does not stress it.

### #9 — Smith-Waterman fuzzy matching
- **Status:** Next Tier (v0.5) — deferred.
- **Reference:** Smith & Waterman, *Identification of Common Molecular Subsequences*, Journal of Molecular Biology 1981 (adapted from bioinformatics).
- **Crate:** `nucleo-matcher`.
- **Gap closed:** "Peter Wood fine" ≈ "Peter Woodfine". Handles spacing errors, minor typos, and transcription noise.
- **Why v0.5:** useful but not critical for the 10-email corpus.

---

### #10 — XOR filters
- **Status:** Deferred — optimisation only.
- **Crate:** `xorf`.
- **Gap closed:** compact gazetteer storage at 1.23 bits per key. 10M names fit in ~1.5 MB with 1% false-positive rate.
- **Why deferred:** at the current ~100K-name gazetteer scale, Aho-Corasick fits in ~10 MB without optimisation. Revisit when gazetteer size exceeds 1M entries.

### #11 — MinHash + LSH
- **Status:** Deferred.
- **Crate:** `probabilistic-collections`.
- **Gap closed:** candidate deduplication at scale.
- **Why deferred:** not needed at 10-email scale.

### #12 — Union-find entity resolution
- **Status:** Deferred.
- **Crate:** `petgraph`.
- **Gap closed:** deterministic entity merging across email/phone/name overlap.
- **Why deferred:** 10-email corpus does not stress entity resolution.

### #13 — BLAST seed-and-extend
- **Status:** Deferred.
- **Implementation:** in-tree, ~200 LOC.
- **Gap closed:** fast screening at million-candidate scale before alignment.
- **Why deferred:** not needed for current scale.

### #14 — PDF pipeline (pdf-inspector + pdf-extract + lopdf)
- **Status:** Deferred.
- **Crates:** `pdf-extract`, `lopdf`, Firecrawl `pdf-inspector` port.
- **Gap closed:** PDF text extraction, table detection, Markdown conversion.
- **Why deferred:** no PDFs in the 10-email test set. Revisit when an institutional customer starts routing PDF workflows (Woodfine lease documents, contracts, etc.).

### #15 — OCR fallback (pdfium-render + tesseract-rs + paddleocr)
- **Status:** Deferred.
- **Crates:** `pdfium-render`, `tesseract-rs`, `paddleocr` via subprocess.
- **Gap closed:** scanned PDFs and image-based documents.
- **Why deferred:** no scanned docs in test set. OCR requires hardware above the $7 base node tier regardless — sits with `service-slm` upgrade tier.

---

## Resource budget — v0.4 Core target

| Component | Size on disk | RAM at runtime |
|---|---|---|
| Core binary (Rust MUSL static) | ~10 MB | ~25 MB |
| Gazetteer (CRM + first names + org suffixes, Aho-Corasick automaton) | ~12 MB | ~10 MB |
| CGM coefficients (in-tree, static) | included | <1 MB |
| Entropy filter tables | included | negligible |
| Signature regex set | included | negligible |
| **Total v0.4 Core** | **~22 MB** | **~36 MB peak** |

Target node: commodity cloud instance at approximately $7/month, 1 GB RAM. v0.4 Core fits comfortably with substantial headroom.

---

## What v0.4 does NOT do

- No AI, no LLM, no NER model, no embeddings, no vector database.
- No PDF extraction (Deferred #14).
- No OCR (Deferred #15).
- No HTTPS calls. No external APIs.
- No persistent daemon. One `.eml` in, records out, process exits.
- No proprietary dependencies. All crates Apache 2.0 or MIT compatible.

If the test corpus ever requires something in this list, pull the corresponding technique out of Deferred, document the production need, and re-measure. Do not silently add it.

---

## Integration contract (stable across versions)

**Input:** a single `.eml` file path + a `totebox_root` path.

**Output:** four write locations under `<totebox_root>/service-fs/data/`:

| Path | Contents |
|---|---|
| `service-extraction/source/<txid>.eml` | Raw `.eml`, SHA-256 sealed before any processing (SYS-ADR-07 forensic chain of custody) |
| `service-extraction/ledger/<txid>.yaml` | Transaction record: timestamp, source hash, extracted record IDs |
| `service-people/source/<PSN>.json` | One JSON per person extracted, anchored to PSN identifier |
| `service-content/source/<txid>.md` | Clean body text as Markdown |

**Exit code:** 0 on success. Non-zero only on unrecoverable error (corrupt `.eml`, missing totebox root, permission denied). Partial extractions are logged to the ledger but do not fail the process.

**Determinism:** same input must produce same output byte-for-byte. No timestamps inside the JSON bodies (use transaction metadata in the ledger instead). No random IDs — use content-addressed hashes or monotonic counters.

---

## Do not do

- Do not add any technique above the phase line without first updating `VALIDATION.md` with a measurement that proves the gap exists and justifies the scope change.
- Do not introduce new crate dependencies beyond the Core list without documenting the gap they close in this file.
- Do not add an AI dependency to this service. AI processing happens in `service-slm`, which is an optional upgrade. This service must run on the base $7 tier with zero AI component.
- Do not rename this service to `service-parser`. The canonical name is `service-extraction`; the legacy name persists in some older code but is not used for new work.
- Do not use the terms "Leapfrog 2030", "Cognitive Forge", "Sovereign" as a descriptive adjective, "Tri-State Vault", "Unicode Enclave", or "Linguistic Air-Lock" in docs, comments, or error messages. These are old AI-generated artefacts from prior sessions and are explicitly retired.
