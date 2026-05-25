---
name: project-ring2-ring3-coupling
description: service-content has a Ring 2/3 coupling defect — graph halts when Doorman is unavailable; 30-LOC fix approved
metadata:
  type: project
---

`service-content/src/main.rs` currently calls the Doorman (Ring 3) to extract entities before writing anything to the graph. If the Doorman is unavailable, Ring 2 writes nothing — graph stops growing. This violates DOCTRINE claim #54 (deterministic substrate must work without AI).

**Fix approved 2026-05-23**: write a deterministic `Source` node to the graph BEFORE calling the Doorman (~30 LOC at `main.rs:198`). Graph grows regardless; entities get enriched later when Doorman responds.

**Why:** Operator approved addition of §4.1 to BRIEF-flow-restructure.md on 2026-05-23. Fix is in BRIEF-vm-hardening-and-consolidation.md §2 Step 2 todo list.

**How to apply:** This is the top-priority service-content code change for the next code session. Full defect list: BRIEF-service-content-architecture.md.
