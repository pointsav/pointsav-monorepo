# AGENTS.md — service-slm

This file is a framework-agnostic version of [CLAUDE.md](./CLAUDE.md), intended
for AI coding agents other than Claude Code (Cursor, Aider, Continue, Cline,
OpenHands, and anything else that reads an `AGENTS.md` convention file).

The rules, invariants, and session protocol are identical to CLAUDE.md. The
only reason this file exists separately is that different agent frameworks
look for different filenames, and we refuse to play the which-file-wins game.
Both files are kept in sync by [`.github/workflows/ci.yml`](./.github/workflows/ci.yml),
which fails the build if they diverge.

If you are an AI coding agent reading this file because your framework reads
`AGENTS.md` specifically, please:

1. Read [CLAUDE.md](./CLAUDE.md) in full — it is the canonical content.
2. Read [STATUS.md](./STATUS.md) and [TASKS.md](./TASKS.md).
3. Read the `CLAUDE.md` inside whichever crate you are working on.
4. Follow the session protocol in CLAUDE.md.

If you are a human wondering why this file is a redirect: it is, and the
single canonical file is [CLAUDE.md](./CLAUDE.md). The redundancy is
deliberate; the drift-prevention CI check is the insurance.
