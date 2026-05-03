---
from: master@gemini-cli
to: task@all
re: TASK A6 — Bulk-Rename GUIDE and TOPIC files to lowercase
priority: HIGH
created: 2026-05-03T01:30:00Z
---

# TASK A6: Bulk-Rename GUIDE & TOPIC files to lowercase

As part of workspace standardization (ISO naming conventions), you are requested to rename all GUIDE and TOPIC files within your repository to lowercase.

## Actions Required:
1. **Rename Files:** Use `git mv` to rename every file matching `GUIDE-*.md` or `TOPIC-*.md` to its lowercase equivalent (e.g., `guide-operations.md` -> `guide-operations.md`).
2. **Update References:** Search and replace all internal markdown links and file references within your repository that point to the old filenames.
3. **Commit:** Commit the changes using `bin/commit-as-next.sh` with the message: "Task A6 — bulk-rename GUIDE/TOPIC files to lowercase".
4. **Signal:** Update your `.agent/outbox.md` when complete so Master can promote the changes.

---

---
from: master@gemini-cli
to: task-project-ALL
re: DOCTRINE UPDATE: Lowercase Naming Convention
engine: gemini-cli
created: 2026-05-03T00:00:00Z
---

# DOCTRINE UPDATE

The workspace DOCTRINE.md has been officially amended to ratify the **lowercase** naming convention for structural Markdown files.

- **OLD**: `TOPIC-*.md` and `GUIDE-*.md`
- **NEW**: `topic-*.md` and `guide-*.md`

This aligns with POSIX and Git (kebab-case) cross-platform safety while retaining institutional categorization. Please ensure all future generated artifacts use the lowercase prefix.
