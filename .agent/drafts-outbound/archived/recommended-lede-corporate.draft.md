---
schema: foundry-draft-v1
state: draft-recommended-lede
originating_cluster: project-editorial
target_repo: content-wiki-corporate
target_path: ./
target_filename: index.md
target_owner: project-knowledge
ownership_note: "Main Page artifact is project-knowledge's (KNOWLEDGE-PLATFORM-VISION §5). This is a recommended lede, not an editorial-owned rewrite."
audience: customer-public
bcsc_class: public-disclosure-safe
language_protocol: PROSE-TOPIC
authored: 2026-05-21
authored_by: totebox@project-editorial
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 2
open_questions_count: 1
research_provenance: direct-consultation
research_inline: true
---

# Recommended lede — corporate.woodfinegroup.com Main Page

> **For project-knowledge.** project-knowledge owns the corporate Main Page
> (Vision §5). This is a recommended lede — starting material, not an
> editorial-owned rewrite. Bloomberg four-paragraph nut graf, Crisis-first,
> accordion rhythm; BCSC continuous-disclosure posture observed throughout.

## Recommended lede text

An investor in a pooled real-estate fund does not own real estate. They own a unit in a fund, and the fund owns the property. Between the investor and the asset sits a fund layer that can blend performance across holdings, gate redemptions, and wind down on a schedule the investor did not set.

Woodfine Management Corp. acts as Principal Manager for a portfolio of direct-hold structures that remove that layer. Each structure holds commercial real estate directly and is owned through freely transferable investment units. Each is organized as a regulated reporting entity, with mandatory disclosure to securities regulators under NI 51-102.

The mechanism is perpetual equity ownership. There is no fund wind-down date and no pool to redeem against; an investor exits by transferring units, not by forcing a sale of the underlying asset. Interest-coverage constraints and fiduciary data obligations are defined in the structure itself rather than left to manager discretion.

This encyclopedia is the corporate reference for that architecture. It documents the ownership model, the equity-transfer mechanics, the fiduciary data obligations, the interest-coverage constraints, and the redemption-elimination framework — written for the banker, the asset manager, and the auditor evaluating a Woodfine holding.

Forward-looking statements in this wiki reflect current intentions and are subject to risk and uncertainty; they are framed using planned, intended, or target language per the firm's continuous-disclosure practice under NI 51-102 and OSC SN 51-721.

## Editorial note

- **Structure.** Paragraph 1 opens on the crisis (a pooled-fund investor does not own the asset); paragraph 2 names what Woodfine does and the verifiable structural fact; paragraph 3 is the mechanism; paragraph 4 states the audience and what the encyclopedia is.
- **Register.** Bloomberg / FT institutional register (editorial-language-registers Register 1). No marketing vocabulary. No competitor named.
- **BCSC.** No forward-looking claim is asserted as present fact. The Sovereign Data Foundation is not referenced. The forward-looking note is retained verbatim from the current `index.md`.

## Research trail

- **Research done (5):** current `content-wiki-corporate/index.md`; `content-wiki-corporate/CLAUDE.md` (audience, voice, BCSC rule); `reference/style-guide-topic.md` (Bloomberg lede + Gate-0); `reference/editorial-language-registers.md` (Register 1); editorial plan §4 A1.
- **Research suggested (2):** project-knowledge to confirm the lede against the live corporate home-page chrome; a corporate-vocabulary cross-check once the DataGraph is reachable (see open question).
- **Open question (1):** editorial plan §4 A1.2 directs a check of the `cluster-totebox-jennifer` DataGraph for corporate vocabulary before drafting. The DataGraph is not present on disk and `service-content` is runtime-hung (see E1) — per plan §3 the draft proceeds in degraded mode, drawing vocabulary from the on-disk `content-wiki-corporate` corpus. project-knowledge or a later pass should reconcile against the DataGraph when it returns.
