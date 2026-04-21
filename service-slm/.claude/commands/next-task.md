---
description: Read TASKS.md, pick the highest-priority open task with context, and propose a plan.
---

Read `TASKS.md` at the workspace root. Identify the highest-priority
open task (p0 before p1 before p2) that:

- You have the context to complete.
- Is not blocked on another task being done first.
- Does not require decisions that belong to a human (licence changes,
  new ADRs with subjective rationale, anything in the "escalation"
  section of `CLAUDE.md`).

Then:

1. State the task number and title.
2. Read the relevant crate's `CLAUDE.md`.
3. Read the relevant section of `specs/SLM-STACK.md` or
   `specs/YOYO-COMPUTE.md`.
4. Propose a step-by-step plan with file paths and approximate line
   counts.
5. Stop and wait for human approval before writing any code.

Do not skip step 5. The plan is the contract for the session.
