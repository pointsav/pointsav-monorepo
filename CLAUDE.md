# Mathew's Sandbox

This is your personal scratch space. **It is not Foundry.** None of
the rules in `/srv/foundry/CLAUDE.md` or `/srv/foundry/DOCTRINE.md`
apply here. There is no mailbox protocol, no scope geometry, no
versioning rules, no NOTAM to read. Do whatever the user asks.

## What lives here

- `CLAUDE.md` (this file) — orienting note. Brief.
- `NEXT.md` — your personal todo list. Add to it freely; remove items
  as you complete them. Not the Foundry NEXT.
- `MEMORY.md` — pointer to the auto-memory location for this sandbox.
- Any other files mathew chooses to keep here.

## Persistent memory

Claude Code auto-creates and maintains a per-project memory store at:

```
~/.claude/projects/-home-mathew-sandbox/memory/
```

That directory is **separate** from the Foundry-context memory at
`~/.claude/projects/-home-mathew/memory/`. Save sandbox-relevant
memories there. Keep an `MEMORY.md` index file inside the memory
directory listing the entries (one line each) for fast recall.

## Hard rule — keep Foundry separate

If the user asks you to touch anything under `/srv/foundry/`,
`~/Foundry/`, or any Foundry-tier identity, **stop**. Tell mathew
he should switch to a Foundry-rooted Claude session for that work
(open a new terminal, `cd ~/Foundry`, start `claude`). Sandbox is
the wrong place; the doctrine's geometric scope rule depends on
which directory the session opened in.

## Suggested uses

- Quick scratch code, one-off scripts.
- Research notes outside the Bloomberg-article-standard discipline.
- Personal todo tracking.
- Prototyping ideas before they earn a place in Foundry.
- Anything that does not need Foundry's audit trail, signed commits,
  or cross-realm coordination.
