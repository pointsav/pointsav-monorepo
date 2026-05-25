# Cross-Cluster Cargo Dependency Visibility — Options Analysis

Prepared for Master Claude outbox. Context: `service-fs-anchor-emitter`
(in `~/Foundry/clones/project-data/`, on branch `cluster/project-data`)
needs `system_core::SignedCheckpoint` (defined in `~/Foundry/clones/project-system/`,
on branch `cluster/project-system`, not yet on `main`).

---

## 1. Problem Statement

`pointsav-monorepo` is worked via the cluster-clone pattern: each cluster is an
independent `git clone` checked out to its own feature branch. Branch
`cluster/project-system` holds `system-core` (version 0.1.4 at time of writing);
branch `cluster/project-data` does not — `system-core` is absent from that
branch's working tree. Both branches share the same upstream repo but have not
been merged; neither has been promoted to canonical `main`. A Cargo `path =`
dependency from `fs-anchor-emitter` pointing to a sibling path inside the
`project-data` clone would resolve to nothing, because `system-core/` does not
exist in that clone's checked-out state. The problem is not a Cargo limitation —
it is a branch-isolation fact: `system-core` lives on a branch the consumer clone
has never seen.

An additional wrinkle: `fs-anchor-emitter` declares its own `[workspace]` in
`service-fs/anchor-emitter/Cargo.toml`, making it a standalone workspace island
rather than a member of the surrounding `project-data` cluster workspace. Any
solution must either join it to a workspace that sees `system-core`, or supply
`system-core` through a mechanism that does not require workspace membership.

---

## 2. Resolution Options

### Option A — Cargo `[patch]` override pointing to the sibling clone's path

**Mechanism.** Add a `[patch.crates-io]` or `[patch."<path>"]` section to
`fs-anchor-emitter`'s `Cargo.toml` (or to a workspace root that includes it)
directing Cargo to resolve `system-core = "*"` against an absolute path on disk:
`/srv/foundry/clones/project-system/pointsav-monorepo/system-core`. The
consumer adds `system-core` as a normal dependency; the patch redirects
resolution to the live sibling checkout.

**When it fits.** Fast unblocking when the producer crate is already compiling
cleanly, both developers are working on the same machine, and the consumer needs
a type defined only in the producer. Useful for a short sprint where the two
clusters converge quickly.

**Blast radius.** Touches only `fs-anchor-emitter/Cargo.toml` (and optionally a
containing workspace root). The `[patch]` section is local; Cargo embeds the
resolved path into `Cargo.lock`, which will contain an absolute filesystem path.
That `Cargo.lock` must not be committed to staging remotes without stripping or
replacing the patch — it is machine-local state. Anyone cloning the consumer repo
on a different machine or path will get a build failure with no obvious error.

**Cost / friction.** Minimal setup. High ongoing friction: every operator must
maintain the sibling clone at the exact path the patch names, on the exact
branch. The patch must be removed before promoting the consumer crate to `main`.
If the patch is committed by accident, `promote.sh` / Stage-6 will land
machine-specific absolute paths in canonical history — a defect that requires a
fixup commit to clean.

---

### Option B — Git submodule pinning `cluster/project-system` commit inside the consumer clone

**Mechanism.** Add a Git submodule at (e.g.) `vendor/system-core/` inside the
`project-data` clone, pointing to the `pointsav-monorepo` upstream and pinned to
a specific commit on `cluster/project-system`. Reference it via
`path = "vendor/system-core"` in `fs-anchor-emitter/Cargo.toml`.

**When it fits.** When the producer crate is at a stable internal milestone and
the consumer needs a snapshot that does not track live changes. Analogous to
vendoring an unreleased library.

**Blast radius.** Adds a `.gitmodules` entry and a submodule directory to the
`project-data` clone. Other Task sessions that check out `project-data` must run
`git submodule update --init` after every pull. The submodule pinned commit is on
a non-`main` feature branch — if that branch is rebased or the cluster clone's
history is cleaned up during Stage-6 promotion, the submodule reference becomes a
dangling pointer. The consuming cluster's `Cargo.lock` now tracks a path inside
the submodule; both the submodule pin and the lock must be updated in tandem when
advancing the producer.

**Cost / friction.** Non-trivial setup; submodule state is a persistent source of
session confusion (new Claude Code sessions in the consumer clone will see a
detached HEAD inside the submodule). Advancing the producer requires both
a submodule pointer bump commit and a `Cargo.lock` update commit in the consumer.
Maintenance cost is disproportionate to the problem size. Submodule-to-feature-branch
pinning is fragile precisely because feature branches are not stable references.

---

### Option D — Feature-branch merge / cherry-pick: bring `system-core` commits into `cluster/project-data`

**Mechanism.** Merge `cluster/project-system` into `cluster/project-data` (or
cherry-pick only the commits introducing `system-core`). The consumer clone's
branch then literally contains the `system-core/` directory, and a normal
`path = "../system-core"` dependency works within that branch's workspace.

**When it fits.** When the two clusters are converging intentionally — both are
approaching a point where they will be promoted together — and the dependency is
expected to be long-lived. Also appropriate if `system-core` is nearing
Stage-6-readiness and a combined promotion is the intended path.

**Blast radius.** The `cluster/project-data` branch history now contains commits
from `cluster/project-system`. Undoing the merge requires a revert commit; the
branch diverges significantly from `main` in both directions. If `project-system`
later rebases or amends its branch before promotion, the merge base becomes
inconsistent and the next merge will have conflicts. The workspace `Cargo.toml`
on the merged branch will need to declare `system-core` as a member or exclude it
explicitly.

**Cost / friction.** One `git merge` command. Ongoing cost: every subsequent
advancement of `system-core` in `cluster/project-system` requires a re-merge (or
cherry-pick) into `cluster/project-data`. This is manageable if both clusters
advance slowly; it becomes high-friction if `project-system` is iterating rapidly
(Phase 1A alone produced 13 commits). The merge strategy creates a tight coupling
between two clusters that the Foundry pattern is designed to keep loosely coupled.

---

### Option E — Promote-then-consume: block until Stage-6 lands `system-core` on `main`

**Mechanism.** Hold `fs-anchor-emitter`'s dependency on `system-core` until
`cluster/project-system` is promoted to `main` via Stage-6. Once `system-core`
is on `main`, the `project-data` clone pulls `main` and gains the crate as a
normal workspace path dependency.

**When it fits.** When the producer crate (`system-core`) is substantively
complete and Stage-6 promotion is imminent — days, not weeks. Also the correct
choice when the dependency is on a stable API surface that is unlikely to change
significantly after promotion.

**Blast radius.** Zero — no changes to any working branch, no tooling, no
submodules, no patches. The only cost is schedule: `fs-anchor-emitter`
development that depends on `system-core` types is blocked until promotion.
Development that does not depend on those types can proceed independently.

**Cost / friction.** No setup friction. The friction is entirely schedule-driven:
if `project-system` is weeks from promotion (e.g., Phase 1B seL4 integration is
in-flight), the consumer waits or uses a bridge. If promotion is days away, the
wait is low cost. The mechanism is reversible in the trivial sense — there is
nothing to undo.

---

## 3. Comparison Matrix

| Criterion | A — Cargo patch | B — Submodule | D — Merge/cherry-pick | E — Promote-then-consume |
|---|---|---|---|---|
| Setup cost | Low | Medium-high | Low | None |
| Maintenance cost | Medium (patch must be stripped pre-promotion) | High (submodule pin + lock updates per producer advance) | Medium (re-merge per producer advance) | None |
| Consistency guarantee | Tracks live producer checkout — can drift silently if producer branch advances without consumer noticing | Pinned to a specific commit — consumer is intentionally stale until manually advanced | Merged state is consistent at time of merge; diverges if either branch advances | Strong — consumer always sees what canonical `main` declares |
| Blast radius | `Cargo.lock` embeds machine-local absolute path | `.gitmodules` + detached submodule HEAD; dangling pointer risk on branch rebase | `cluster/project-data` branch history entangled with `cluster/project-system` | None |
| Reversibility | Remove patch entry + regenerate `Cargo.lock` — straightforward | `git submodule deinit` + remove `.gitmodules` entry — multi-step | Revert commit — possible but produces noisy history | Trivially reversible (nothing was done) |
| Safe to commit to staging remote | No — `Cargo.lock` contains absolute path | Conditional — submodule pointer is portable but submodule target is a non-`main` branch | Yes | Yes |

---

## 4. Recommendation for `fs-anchor-emitter`

The cleanest fit depends on one unknown: how soon is `cluster/project-system`
likely to reach Stage-6 promotion readiness? Phase 1A is structurally complete
(13 commits, 35 + 44 tests, benchmarks done). If Master judges `system-core` as
promotion-ready — meaning no further breaking changes are expected before
Stage-6 — then **Option E** is the correct choice: hold the `fs-anchor-emitter`
dependency stub (the crate already has `ed25519-dalek` and `sha2` directly, so
it can implement a local `SignedCheckpoint` shim or leave the verification hook
unimplemented) until `main` gains `system-core`, then add the path dependency
normally. If `project-system` is weeks from promotion because Phase 1B (seL4
integration) is in-flight and will produce breaking changes to `system-core`'s
API, then **Option A** (Cargo patch) is the pragmatic bridge: it unblocks
`fs-anchor-emitter` development immediately, costs one `[patch]` entry that
must be removed before any promotion, and leaves no permanent entanglement in
either branch's history. Option A requires discipline — the patch must not be
committed to a staging remote in its current form — but that constraint is
enforceable via a pre-promotion checklist item, which Master already holds in
the Stage-6 flow.

Options B and D are not recommended for this case. Submodule pinning against a
live feature branch is structurally fragile, and branch merging between two
clusters that are intended to advance independently creates entanglement that is
expensive to undo and contrary to the cluster-isolation design principle.

---

## 5. Open Questions for Master

1. **Is `system-core` API-stable for Stage-6 purposes?** If Phase 1B
   (moonshot-toolkit seL4 integration, or any other scheduled work) will
   introduce breaking changes to `system-core`'s public types before promotion,
   Option A (local patch) is the correct short-term bridge. If `system-core` is
   effectively frozen pending promotion, Option E (block on Stage-6) costs
   nothing and avoids any bridging machinery.

2. **Should `fs-anchor-emitter` be promoted on the same Stage-6 run as
   `system-core`, or independently?** If they are intended to promote together,
   a single combined promotion makes Option E the natural choice with no waiting
   penalty. If they are intended to promote at different cadences, the dependency
   relationship needs an explicit resolution (patch, copy, or interface
   abstraction) before either promotion can proceed cleanly.
