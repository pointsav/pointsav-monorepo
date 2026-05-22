# service-slm + service-content — Testing & Hardening
> Updated: 2026-05-18 totebox@project-intelligence  
> Prior plan (crash recovery) superseded — Blocks 1 & 2 fully shipped.
> Session paused: compute pressure. Resume here.

---

## Shipped this session (do NOT redo)

| Commit | What |
|---|---|
| `561b74ce` | fix(content): preserve created_at on upsert + entity_count in healthz + deferred-extraction retry fix |
| `ae653cdb` | feat(slm): D5 Sprint 1 — CanonicalMessage + ContentBlock replaces flat ChatMessage in ComputeRequest |
| `c67bb284` | fix(slm,content): drain worker flag-check before dequeue + remove dead taxonomy helpers |

**211 tests pass. Clippy clean. service-content cargo check clean.**

**7B model (OLMo 2 1124 7B Instruct Q4_K_M):**
- Running via `local-slm.service` with `--parallel 1 --no-repack`
- MemoryMax=7G, MemoryHigh=6500M (drop-in at `/etc/systemd/system/local-slm.service.d/memory.conf`)
- Infrastructure files updated and git-tracked in `~/Foundry/infrastructure/local-slm/`
- Performance: ~5.5s short requests, 80-120s long prompts (CPU-only)

---

## Stage 6 pending

Commits `561b74ce`, `ae653cdb`, `c67bb284`, `9915eddf`, `e365e10e` are on `main`
in `project-intelligence` but not yet promoted to canonical.

```bash
# From Command Session:
cp .agent/engines/claude-code/session.lock /tmp/  # stash lock
echo "y" | ~/Foundry/bin/promote.sh
mv /tmp/session.lock .agent/engines/claude-code/session.lock
~/Foundry/bin/sync-local.sh --all
```

---

## Task 2 — Re-enable apprenticeship (do first on next session, ~5 min)

The drain worker spin-loop fix (task 1) is committed but Doorman has not
been restarted. Do this before restoring briefs.

```bash
# 1. Rebuild and deploy Doorman with new binary
cd /srv/foundry/clones/project-intelligence/service-slm
cargo build --release -p slm-doorman-server
sudo cp target/release/slm-doorman-server /usr/local/bin/local-doorman

# 2. Re-enable apprenticeship in service file
sudo sed -i 's/SLM_APPRENTICESHIP_ENABLED=false/SLM_APPRENTICESHIP_ENABLED=true/' \
  /etc/systemd/system/local-doorman.service
sudo systemctl daemon-reload
sudo systemctl restart local-doorman

# 3. Verify it starts with apprenticeship_enabled=true
sudo journalctl -u local-doorman -n 20

# 4. Restore held briefs from the paused hold area
sudo mv /srv/foundry/data/apprenticeship/queue-paused/*.brief.jsonl \
        /srv/foundry/data/apprenticeship/queue/
# Note: queue/ also has briefs from this session's commits
```

---

## Task 3 — 503 busy-rejection when Tier A saturated (~1 hr)

**Problem:** With `--parallel 1`, any 80-120s extraction prompt holds the only
Tier A slot. New requests queue silently behind it with no timeout signal.

**Where to add:**
- `service-slm/crates/slm-doorman/src/tier/local.rs` — before dispatching,
  query llama-server `GET /health` and check `"status": "ok"` vs `"loading"`.
  llama-server returns `{"status":"ok","slots_idle":1,"slots_processing":0}` when free.
  If `slots_idle == 0` AND Tier B is unavailable → return `DoormanError::TierUnavailable`
  which the router should map to 503 + `Retry-After: 30`.

**Test:** Add a unit test that mocks llama-server `/health` returning slots_idle=0.

---

## Task 4 — End-to-end integration test for Anthropic shim (~1 hr)

**Where:** `service-slm/crates/slm-doorman-server/src/http.rs` (existing test module)

**What to test:**
1. `POST /v1/messages` with `{"model":"olmo","messages":[{"role":"user","content":"hi"}]}`
   → shim translates to `ComputeRequest` → mock Tier A → SSE response `content_block_delta`
2. Tool_use request: `content: [{"type":"tool_use","id":"x","name":"fn","input":{}}]`
   → `CanonicalMessage` with `ContentBlock::ToolUse` → round-trips correctly

**Note:** The `tool_use` integration test was un-ignored in D5 Sprint 1 — verify it's
actually running by checking the test count (was 102 in slm-doorman, 51 in slm-doorman-server).

---

## Task 5 — ✅ DONE (committed in c67bb284)

---

## VM stability

- [ ] **Stage 6 promote** — see above
- [ ] **vm.swappiness** — `sysctl vm.swappiness` should be 10
- [ ] **Yo-Yo snapshot** — before next Yo-Yo start:
  `gcloud compute disks snapshot yoyo-tier-b-1 --zone=europe-west4-a --snapshot-names=yoyo-tier-b-1-$(date +%Y%m%d)`
- [ ] **Disk at 88%** — journal vacuum + service-extraction/target could free ~1.5GB:
  ```bash
  sudo journalctl --vacuum-size=500M   # saves ~800MB
  ```

---

## Execution order next session

```
Task 2 (5 min)  →  Stage 6 promote (15 min)  →  Task 3 (1 hr)  →  Task 4 (1 hr)
```
