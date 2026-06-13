# BRIEF discipline — project-infrastructure

Rules for BRIEF-*.md files in this archive.
Full specification: `/srv/foundry/conventions/brief-discipline.md`

## Quick reference

- **Soft cap:** 5 active BRIEFs (status: active) at one time
- **New vs update:** New BRIEF for orthogonal ideas (use `parent:`); update existing for same-scope work
- **Status values:** active | reference | archived | superseded | stub (only these five)
- **Required frontmatter:** artifact, schema, brief-id, title, status, owner, created, updated
- **Body sections (in order):** Context → Scope → Decisions locked → Decisions open → Work log → Carry-forward
- **README.md:** Keep `.agent/briefs/README.md` active-briefs table current at every session
- **Never delete:** Supersede via status field or git mv to briefs/archive/
