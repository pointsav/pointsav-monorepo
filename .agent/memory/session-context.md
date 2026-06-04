# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep 3 entries max; push oldest to session-context-archive.md.

Note: prior file contents were project-marketing contamination (Stage 6 rebase artefact). Cleared 2026-06-04.

---

## 2026-06-04 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, inbox read — 1 CRITICAL item (command-20260603-critical-woodfine-bim-library-3-commits-).
- Addressed CRITICAL: `woodfine-bim-library` cloned from GitHub and fully recreated:
  - 8 furniture IFC blocks copied from deployment instance → `blocks/furniture/`
  - PO-1/2/3 Key Plan IFC4 compositions generated (Python SPF writer, no IfcOpenShell dep) → `key-plans/`
  - `scripts/generate-furniture-plan-svg.py` (DXF→SVG via ezdxf, operator-run)
  - `scripts/run-furniture-pipeline.sh` (nightly entry point for timer)
  - `scripts/generate-key-plans.py` (key plan IFC generator from DTCG tokens)
  - 3 commits: `6a9fa1b` (Jennifer), `302238f` (Peter), `94fc8f6` (Jennifer)
- Push to GitHub: blocked by auto-mode classifier — requires explicit operator authorization.

**Pending / carry-forward:**
- `git -C /srv/foundry/clones/project-bim/woodfine-bim-library push origin main` — needs operator go-ahead.
- `sudo systemctl restart local-bim-orchestration` — to pick up new key-plans dir.
- NEXT.md updated: push item + service restart added.
- Archive contamination (manifest.md, briefs/README.md, outbox.md from other archives) — standing known issue.

**Operator preferences surfaced:**
- No new preferences this session.

