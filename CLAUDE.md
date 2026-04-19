# CLAUDE.md — pointsav-monorepo

This file loads into Claude Code's context at session start. Read it before acting. If a rule here conflicts with your instinct, follow the rule.

---

## What this repo is

The engineering source for the PointSav platform. Contains all `system-*`, `service-*`, `os-*`, `app-*`, and `moonshot-*` crates. Primary language is C at Layer 0 (legacy seL4). The declared direction is 100% `no_std` Rust. Every new crate is Rust unless there is a documented reason otherwise.

This repo does not contain deployment manifests, content wikis, or marketing sites. Those live in separate repos under the `pointsav` and `woodfine` GitHub orgs.

---

## The three pillars test

Every component traces to one of three business purposes. If it cannot, it needs a strong reason to exist. Ask this question before adding a new crate.

1. Business Administration — daily property, tenant, service-provider, and reporting operations
2. Record Keeping — legally required, auditable, lifetime-of-asset records
3. Cyber-physical Connectivity — IoT, BIM, digital twins, building management

---

## Authoritative references, in priority order

1. **Nomenclature Matrix** — authoritative for all names, prefixes, conventions
2. **Development Overview MEMO** — authoritative for architecture
3. **Live code in this repo** — authoritative for what is actually built
4. **User Guide** — context and ADR rationale only. Contains known legacy naming. Not a naming authority.

When the MEMO and live code disagree, surface the conflict. Do not silently reconcile.

---

## Living cleanup log

A companion file at `.claude/rules/cleanup-log.md` holds the current state of in-flight cleanup work, open questions pending confirmation, and a chronological record of significant changes.

**At session start:** read `.claude/rules/cleanup-log.md`. Apply its guidance throughout the session — it reflects the most current state of naming migrations, deprecations, and open questions.

**At session end:** if the session included meaningful cleanup — renames across multiple files, deprecated code removal, resolving an open question, or surfacing a new one — append a dated entry to the top of the session entries section in the log. Entry format:

```
## YYYY-MM-DD
- What changed (files touched, counts, rationale)
- What was left pending and why
- New open questions surfaced
```

Do not log trivial edits or single-file changes. The log is a record of decisions, not of every keystroke.

---

## Naming prefixes — never substitute

Every prefix carries architectural meaning. Prefixes define execution boundaries, not stylistic choices.

| Prefix | Rule |
|---|---|
| `os-*` | Foundation. Below the application layer. Zero UI. |
| `app-[os]-*` | Interface crate. Runs on `os-console` or `os-orchestration`. Infix identifies the host OS. |
| `service-*` | Autonomous daemon inside a Totebox Archive. Async. Zero UI. One job per service. |
| `tool-*` | Manual or operator-triggered. Never a permanent daemon. |
| `vendor-*` | Quarantined third-party. Cryptographically isolated from core logic. |
| `moonshot-*` | R&D. Production-prohibited until formally verified at parity with the `vendor-*` it replaces. |

Do not create a new top-level prefix without updating the Nomenclature Matrix first.

---

## Canonical OS names — never substitute with synonyms

The platform has a precise vocabulary. Substitution creates drift.

- ToteboxOS — not ArchiveOS, VaultOS, DataNode
- OrchestrationOS — not InterfaceOS, LogicLayer, MiddleTier, AggregationService
- ConsoleOS — not ClientApp, Frontend, Terminal
- InfrastructureOS — not HypervisorLayer, VMSubstrate
- NetworkAdminOS — not MeshController, NodeManager
- PersonnelArchive — not HRArchive, PeopleNode
- PropertyArchive — not RealPropertyArchive, BuildingArchive, AssetNode
- CorporateArchive — not FinanceArchive, BusinessNode
- CommandCentre — not OrchestratorHub, AccessGateway
- FKeysConsole — not MainTerminal, AdminConsole

---

## Active legacy-to-canonical renames

These substitutions are in flight. When you encounter a legacy name, flag it and propose the canonical rename. Do not leave legacy names in new code. Check the cleanup log for current migration status.

| Legacy | Canonical |
|---|---|
| `service-parser` | `service-extraction` |
| `service-llm` | `service-slm` |
| `cluster-totebox-real-property` | `cluster-totebox-property` |
| `os-interface`, `os-integration` | `os-orchestration` |
| `RealPropertyArchive` | `PropertyArchive` |
| `fleet-command-authority` | deprecated — remove |

---

## ADR hard rules — no exceptions

Three rules apply to every change in this repo. Violations are rejected.

- **SYS-ADR-07** — structured data (CSV, JSON, XLSX, telemetry) never routes through AI. Deterministic parsers only. AI is reserved for unstructured human text.
- **SYS-ADR-10** — F12 is the only path for base-asset ingestion. No batch import, no AI auto-commit. A human operator selects destination archive, service, and Chart of Accounts.
- **SYS-ADR-19** — automated AI publishing to verified ledgers is prohibited. Totebox Orchestration is isolated from Independent Systems. Independent Systems consume F12-verified ledgers only.

---

## Do Not Use — vocabulary list

These terms are inappropriate for institutional, legal, or investor-facing language. Do not introduce them into new code comments, docs, crate names, or README content.

- Cognitive Forge
- Linguistic Air-Lock
- Data Vault
- Content Forge
- Quantum Collapse
- Sovereign (in descriptive use)

"Sovereign" is retained only in proper nouns — Sovereign Data Foundation, Sovereign Data Protocol. Do not use it as a descriptor.

Writing test for any user-facing string: would this sentence appear in a Bloomberg article about a financial infrastructure company?

---

## BCSC / Sovereign Data Foundation disclosure

The Sovereign Data Foundation's intended equity and oversight role has not been formally executed. All language that could leave this repo — READMEs, inline docs, code comments with user-facing strings — must refer to the Foundation only in planned or intended terms. Never as a current equity holder. Never as an active governance body.

If you encounter language that describes the Foundation as current or active, flag it.

---

## READMEs are bilingual

Every `README.md` in this repo has English and Spanish. If you add or modify a README, update both language blocks. If Spanish is missing, flag it — do not silently ship English-only.

This rule applies to README files only. Source code, inline docs, and this CLAUDE.md are English.

---

## Repo structure

```
pointsav-monorepo/
├── system-substrate/         Legacy seL4 C core + hardware bridges (freebsd, broadcom)
├── system-foundation/        Rust no_std hardware primitives
├── system-core/              Universal structs, execution parameters, errors
├── system-security/          Capability monitor (replaces third-party frameworks)
├── system-interface/         Software rasterizer / layout math
├── system-mba-shim/          Active bridge to conventional auth (Azure AD)
├── os-*/                     Foundation OS crates (no UI)
├── app-*/                    Interface crates — console, orchestration, infrastructure variants
├── service-*/                Autonomous archive services
├── tool-*/                   Operator scripts
├── vendor-*/                 Quarantined third-party
├── moonshot-*/               R&D native replacements
└── moonshot-toolkit/         Rust-only build orchestrator
```

---

## Build and toolchain

Use `moonshot-toolkit` for builds. It orchestrates crate compilation, links `system-substrate`, and produces bootable OS images. Do not introduce Python or CMake into the build path without documenting the reason — the direction is a Rust-only toolchain.

---

## What to ask before acting

- **Verification Surveyor daily throttle** — the specific number is under operational review. Do not cite a number. Refer to it as "a system-enforced daily limit" until confirmed.
- **Any structural change to `os-*` or `service-*` boundaries** — surface for review, do not execute silently.
- **Any modification to `vendor-*/`** — requires a corresponding entry in the Moonshot Register and an explicit reason.
- **Any new crate** — confirm it passes the three pillars test before scaffolding.

---

## The test for any change

Before committing, check:

1. Does the name match the Nomenclature Matrix?
2. Does the prefix match the execution boundary?
3. Is there a legacy term still present that should have been renamed?
4. Does any user-facing string treat the Sovereign Data Foundation as current rather than intended?
5. If a README was touched, does it have both English and Spanish?

If any answer is no or unclear, stop and surface it.
