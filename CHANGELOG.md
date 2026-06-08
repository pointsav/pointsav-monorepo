# CHANGELOG — project-proforma

> One line per PATCH version per workspace doctrine. Versions tie to BRIEF
> frontmatter `version:` and to `Version: M.m.P` trailers on commits.

## v0.15.13 — 2026-06-02
feat(spv-budget): SPV operating budget v10 — surgical edits for dual-asset Bencal SPV2 + PCLP 1 canonical rename + Flag 15 path (b). Title block updated; remaining "PCLP1" references replaced with "Professional Centres Canada LP" (line 56 Structure Overview, lines 75/82/106/110 Commission Revenue tables); new Notes section appended (Note A — Bencal SPV2 founding capital endowment with FMV/contributed-surplus accounting and ITA s.69 mechanic; Note B — $10 nominal GP-share capital account; Note C — Bencal Management FOFI Y1-Y3 scope confirmation). Numerical values unchanged from v9 (operating budget reserves are commission-funded, not affected by the dual-asset Bencal SPV2 change).

## v0.15.12 — 2026-06-02
ops(outputs): restore v9 SPV operating budget HTML baseline (377 lines) extracted from workspace-git commit 0c9d15e into the archive's own tree. MD mirror intentionally NOT restored (lagged HTML from v6.1 onward; treated as obsolete).

## v0.15.11 — 2026-06-02
ops(flags): lock Flag 15 path (b) capital contribution (Bencal SPV2 600K WCP at FMV vs contributed surplus equity; no Y0 IS at SPV2); REVERSE Flag 9 to publish Bencal Management FOFI Y1–Y3 only (was Y1–Y5); lock Block F MOIC presentation as side-by-side per-share + aggregate columns with dilution-mechanics header note; lock D4 Phase A full scope (schema + outputs + Block A-F dual-asset column); lock SPV operating budget v10 sequence (v9 restore + surgical v10 edits). BRIEF v0.15.9.

## v0.15.10 — 2026-06-02
ops(stage6): archive + monorepo cluster/project-proforma ready for Command promotion; pre-Stage-6 sweep complete; project-documents informed; deferred items tracked in NEXT.md.

## v0.15.9 — 2026-06-02
ops(housekeeping): pre-Stage-6 sweep — CHANGELOG created; .agent/briefs/ synced with canonical briefs/; project-documents outbox dispatched; J1 inbox archived; deferred items added to NEXT.md.

## v0.15.8 — 2026-06-02
feat(d4-bencal-spv2): dual-asset Bencal SPV2 — adds 600,000 WCP founding-bonus shares at $0.00033223/share nominal ($199.34 total) on completion of minimum CAD 13,000,000 investment in Professional Centres Canada LP. Strategic Partner block reduces from 1,800,000 (18.0%) to 1,200,000 (12.0%); WCP total outstanding unchanged at 10,000,000. Bencal Group aggregate WCP exposure rises to 9.0%. Canonical legal name lock: PCLP 1 → Professional Centres Canada LP ("PC Canada LP"); engine struct identifiers retained for code stability. Opens Flag 15 (IFRS 2 / IAS 12 / Section 69 accounting treatment of Strategic Partner → Bencal SPV2 transfer).

## v0.15.7 — 2026-06-02
ops(flags): resolves all 19 open decision flags spanning Bencal Management §5f (Flags 2/3/4/6/7/8/9/10/11/12/13/14), Snapshot §3e (S1–S4), D7 Legacy JV §5h (D7-1 through D7-5), and Reporting Issuer §3f (P1–P4). Adopts FSL-1.1-Apache-2.0 licensing across all product tiers at $19.00 CAD one-time perpetual; replaces prior $299/$499/month SaaS pricing. LICENSE file added at archive root.

## v0.15.6 — 2026-05-26
Prior — Bencal restructure to net $25M PCLP1 + additive OpexBudget reserve; commission waterfall v0.15.6 with 2% Work Fee constraint; director fee $4,373.37/yr/director. See commit history for full pre-v0.15.6 entries.
