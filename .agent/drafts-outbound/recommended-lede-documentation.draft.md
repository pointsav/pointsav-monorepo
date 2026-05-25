---
schema: foundry-draft-v1
state: draft-recommended-lede
originating_cluster: project-editorial
target_repo: content-wiki-documentation
target_path: ./
target_filename: index.md
target_owner: project-knowledge
ownership_note: "Main Page artifact is project-knowledge's (KNOWLEDGE-PLATFORM-VISION §5). This is a recommended lede, not an editorial-owned rewrite."
audience: vendor-public
bcsc_class: public-disclosure-safe
language_protocol: PROSE-TOPIC
authored: 2026-05-21
authored_by: totebox@project-editorial
authored_with: claude-opus-4-7
research_done_count: 6
research_suggested_count: 2
open_questions_count: 1
research_provenance: direct-consultation
research_inline: true
---

# Recommended lede — documentation.pointsav.com Main Page

> **For project-knowledge.** project-knowledge owns the documentation Main Page
> (Vision §5). This is a recommended lede — starting material, not an
> editorial-owned rewrite. It applies the Gate-0 editorial standard
> (editorial plan §2): a Bloomberg four-paragraph nut graf, Crisis-first, with
> accordion rhythm. project-editorial reviews the lede prose when
> project-knowledge branches the Main Page.

## Recommended lede text

A regulated business rarely owns the systems it runs on. Its data sits in a vendor's cloud, its AI calls leave its network, and its records depend on infrastructure it cannot inspect. When a regulator, an auditor, or an acquirer asks who controls those systems, the honest answer is: someone else.

PointSav builds operating systems and services that close that gap. The platform runs on the customer's own hardware. Every record it produces is continuous-disclosure-grade by structure — append-only, and reconstructable from the ledger rather than from a vendor's assurance. For a buyer that requires an air gap, it operates fully without AI.

The mechanism is ownership by construction, not by contract. A record cannot be altered after it is written. Every AI request passes through a single access-control gateway that logs the call to a per-tenant audit ledger; a request that resolves on local hardware never leaves the customer's infrastructure. The governance commitments that bind future development are written down and versioned alongside the code.

This wiki is the engineering library for that platform, maintained against the published [[style-guide-topic|editorial standard]]. It documents the architecture, the services, the operating systems, and the design rationale, and is written for the institutional reader — the auditor, the technical due-diligence reviewer, the procurement evaluator — and for the engineers who build on the platform. Where the monorepo holds the code, this wiki holds the reasoning.

Forward-looking statements in this wiki carry planned, intended, or target language per the [[style-guide-topic|editorial standard]].

## Editorial note

- **Structure.** Paragraph 1 opens on the crisis (a regulated business does not control its own systems); paragraph 2 supplies the verifiable structural facts; paragraph 3 is the mechanism; paragraph 4 states why it matters and what the wiki is. This is the Bloomberg nut graf — the news survives a stand-alone read of the first four paragraphs.
- **Register.** No adjectives of self-description (gold-standard guide anti-pattern 4.3). No SaaS-marketing register (Gate-0 rule 5). Sentence length varies for accordion rhythm (Gate-0 rule 1).
- **Against the current lede.** The current `index.md` lede states the platform's properties directly but does not open on the problem. Crisis-first framing gives a first-time reader the stakes before the solution.

## Research trail

- **Research done (6):** current `content-wiki-documentation/index.md`; `guide-keep-the-home-page-the-gold-standard.md` (format invariants + anti-patterns); `reference/style-guide-topic.md` (Bloomberg lede structure, Gate-0 standard); `reference/editorial-language-registers.md` (Register 2); editorial plan §4 (Track A method); the Gate-0 standard as encoded in A0.
- **Research suggested (2):** project-knowledge to check the rendered lede against the live home-page chrome (welcome banner + featured panel) for length and visual balance; confirm the `[[style-guide-topic]]` wikilink resolves post-rename.
- **Open question (1):** none material to documentation — the lede uses only structural facts already present in the current `index.md`.
