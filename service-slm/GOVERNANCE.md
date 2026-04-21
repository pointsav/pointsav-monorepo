# Governance

This document describes how decisions are made in the service-slm project.
It is deliberately short because the project is small, and it is explicit
about the decision process because small projects with implicit governance
do not scale.

---

## 1. Structure

service-slm is a benevolent-dictator project stewarded by **Woodfine
Capital Projects Inc.**, represented operationally by a small group of
**maintainers** listed in [MAINTAINERS.md](./MAINTAINERS.md).

- **Steward:** Woodfine Capital Projects Inc. owns the copyright, chooses
  the licence (currently AGPL-3.0-only), and appoints maintainers. Legal and
  commercial decisions rest with the steward.
- **Maintainers:** have commit access, review authority, and the
  responsibility to ensure this project remains healthy, secure, and
  faithful to its specifications. They are named in `MAINTAINERS.md` and
  reflected in [CODEOWNERS](./CODEOWNERS).
- **Contributors:** anyone who submits a patch, issue, or review comment.
  You do not need to be a maintainer to influence the project.

This is not a foundation-governed project. If and when it becomes one, this
document will be replaced by a matching charter.

---

## 2. How decisions are made

We distinguish four classes of decision, each with its own process.

### 2.1 Code changes

**Process: pull request + code review.**

- Small changes (bug fixes, documentation edits, dependency bumps inside the
  allow-list): one maintainer approval is sufficient.
- Medium changes (new features inside an existing crate, non-breaking API
  additions): one maintainer approval plus one non-author contributor approval,
  or two maintainer approvals.
- Large changes (new crates, breaking API changes, cross-crate refactors):
  require an ADR (see 2.2 below) merged first, then the code change referenced
  against the ADR.

CI must pass in all cases. Merge is by squash-commit.

### 2.2 Architecture decisions

**Process: Architecture Decision Record (ADR) pull request.**

ADRs live in [`docs/adr/`](./docs/adr/) and use the
[MADR](https://adr.github.io/madr/) template. Each ADR records a context, a
decision, its rationale, and its consequences. ADRs are numbered sequentially
and are immutable once merged — superseding is done by a new ADR that
explicitly references the one it supersedes.

Any contributor may propose an ADR. Merge requires:

- Two maintainer approvals.
- A 72-hour minimum comment window so non-maintainer contributors have a
  chance to weigh in.
- No outstanding "requesting changes" reviews.

ADRs govern any question that will shape the project for more than one
release — crate structure, dependency choices, protocol design, licence
policy, release cadence.

### 2.3 Spec changes

**Process: change at the monorepo source, then re-copy.**

The files in [`specs/`](./specs/) are verbatim copies of source documents
that live at the root of the PointSav monorepo. They are read-only in this
repository. If the specification needs to change:

1. Change the source document in the monorepo-root docs tree.
2. Open a PR in this repository that re-copies the updated file and updates
   any affected ADRs.
3. Normal code-review rules apply.

A spec change that conflicts with an existing ADR is a governance event:
either the ADR is superseded, or the spec change is rejected. Maintainers
decide.

### 2.4 Commercial, legal, trademark, release-gating

**Process: Woodfine Capital Projects Inc. decides.**

These are not community decisions. The steward holds authority over:

- Licence changes (require Woodfine board action).
- Trademark use of "PointSav" and "Woodfine".
- Release versioning and timing.
- Commercial relationships and support agreements.

The steward consults maintainers on these questions but is not bound to
follow their advice. Maintainers who disagree with a steward decision may
record their dissent in the relevant ADR and continue.

---

## 3. Becoming a maintainer

There is no fixed threshold, but the usual path is:

1. Sustained, high-quality contribution over at least six months.
2. Demonstrated understanding of the specifications and the invariants.
3. A record of thoughtful review comments on other contributors' PRs.
4. A nomination from an existing maintainer.
5. Unanimous approval of existing maintainers.
6. Confirmation from the steward.

Maintainership is recorded in [MAINTAINERS.md](./MAINTAINERS.md) and
reflected in [CODEOWNERS](./CODEOWNERS). It can be stepped down voluntarily
at any time, or removed by unanimous vote of remaining maintainers plus
the steward (in the event of persistent absence or violation of the Code
of Conduct).

---

## 4. Conflicts of interest

Maintainers who have a material conflict of interest on a specific PR or
ADR (for example, a direct commercial relationship with a party affected by
the outcome) should recuse themselves from review. Disclosure is sufficient
for minor conflicts; recusal is required for material ones.

When in doubt, disclose in a PR comment and ask another maintainer to weigh
in on whether recusal is warranted.

---

## 5. Changing this document

Governance changes follow the ADR process (2.2), with the additional
requirement that the steward explicitly approves. This document is, itself,
governed by the rules it describes.

---

## 6. Current state

At time of writing this project is **pre-1.0.0** and in active scaffolding.
Governance is lightweight; most decisions flow through the steward and a
small maintainer set. As the project matures and external contributors
arrive, these processes will be exercised and, where necessary, refined via
ADR.
