# NEXT.md — project-intelligence (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-19 (Session 25 shutdown)

---

## Active (Totebox scope)

- [ ] **down_for_secs in TierBInfo** — `health_down_secs: Option<u64>` added to TierBInfo
      + `health_down_since_secs: Arc<AtomicU64>` wired in YoYoTierClient/run_health_probe;
      committed but deploy pending (Stage 6 + slm-doorman-server rebuild required)
      [2026-06-19 totebox@project-intelligence]
- [ ] **Phase 4b reconciliation pass** — 1,281 sweep-ledger entries written before Tier B online;
      DOC_sweep quarantine gate in place; Totebox sprint when Tier B restores; gated on
      yoyo-batch being provisioned in us-central1-a (operator approval required)
      [2026-06-15 command@claude-code]
- [x] **CLAUDE.md contamination** — confirmed clean (81 lines, correct project-intelligence
      SLM/Doorman/OLMo/LoRA/DataGraph content; no project-console text)
      [2026-06-19 totebox@project-intelligence]
- [ ] **Phase 5b — adapter pull verification** — pull wired in nightly-run.sh (Phase 5b block);
      pulls from yoyo-batch:/data/weights/adapters/apprenticeship-pointsav-wip/ at start of
      Phase 1 each cycle; verify after first successful yoyo-batch cycle:
      `ls /srv/foundry/data/adapters/apprenticeship-pointsav-incremental/`
      [2026-06-19 totebox@project-intelligence]
- [x] **Phase 6-D — enrichment spot-check** — 3 extractions confirmed; `tier_used: "tier_a_fallback"`;
      OLMo-2 Tier A returning clean entities (Person/Company/Location); f1879462 verified working
      [2026-06-19 totebox@project-intelligence]
- [ ] **Remove dead config** — `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=false` confirmed
      absent from all codebase files; must be in live systemd unit only; Command scope
      (systemd override cleanup + daemon-reload); routed via outbox
      [2026-06-19 totebox@project-intelligence]
- [x] **Bug: semaphore leak on client disconnect** — fixed 2026-06-19; 120 s timeout wrapper
      (`EXTRACT_DEADLINE_SECS`) around entire routing block in `/v1/extract` handler;
      `DoormanError::RequestTimeout` returned on deadline → permit drops via RAII; bounds
      permit hold to 120 s even when hyper 0.14 keeps handler alive after client disconnect
      [2026-06-19 totebox@project-intelligence]
- [x] **Bug: DeferReason wildcard in http.rs** — fixed 2026-06-19; added `TierAFailed`,
      `ParseError`, `Timeout`, `AllTiersUnavailable` variants to `DeferReason` enum in
      slm-core; both extract + batch handler wildcards now have explicit arms;
      `DoormanError::RequestTimeout` added to error.rs + ApiError status mapping
      [2026-06-19 totebox@project-intelligence]
- [ ] **Known: queue saturates OLMo in Tier B degraded mode** — corpus queue runs 2 in-flight
      (matching OLMo --parallel 2); when Tier B down, queue uses Tier A leaving 0 slots for
      interactive /v1/extract; resolves automatically when yoyo-batch restores (queue → Tier B);
      workaround: limit queue to 1 in-flight via SLM_BATCH_CONCURRENCY=1 when Tier B down
      [2026-06-19 totebox@project-intelligence]

## Blocked — Command Session (route via outbox)

- [x] **local-slm.service `--parallel 2`** — operator approved 2026-06-19; applied to
      threads.conf drop-in; daemon-reload + restart; service active; two slots now available
      [2026-06-19 command@claude-code]
- [ ] **yoyo-batch ML libs** — trl/peft/transformers/accelerate/bitsandbytes not installed in
      training venv on GPU VM; LoRA training has never produced a real adapter; install needed
      before next training cycle; yoyo-batch TERMINATED (us-central1-a STOCKOUT); restart
      requires operator approval [2026-06-16 operator]

## Completed (Sessions 18–25)

- [x] **Stage 6 complete — 13 commits total** — 8 commits (088b8e21→4886129d) + 5 commits
      (1fe42506→12076cf1) on canonical; includes Doorman Tier A fallback (f1879462),
      LoRA r=32/alpha=64 + sigmoid_norm DPO (60e88399), batch-extract endpoint, drain-hold fix,
      repair-ledger.py, DOC_sweep quarantine gate, entity_filter.rs hardening
      [2026-06-19 command@claude-code]
- [x] **Doorman Tier A fallback (f1879462)** — `/v1/extract` now falls back to Tier A when
      Tier B circuit open; canonical but binary rebuild pending (in-flight 2026-06-19)
      [2026-06-19 command@claude-code]
- [x] **service-content rebuilt** — binary from 631574ee (prompt v3 + entity_filter.rs);
      local-content.service active; entity_count=12,080 [2026-06-19 command@claude-code]
- [x] **OOV cleanup** — 531 pre-OLMo3 entities + 84 noise-name entities deleted;
      615 total removed; DataGraph healthier post-cleanup [2026-06-19 totebox@project-intelligence]
- [x] **Phase 7 Tier A test** — 12/14 tests passed (prompt v3); two remaining are semantic
      edge cases (GCP zone context + Doorman entity classification) [2026-06-19 totebox@project-intelligence]
- [x] **yoyo-batch /data/weights/adapters** — directory created; June 14 adapter rsync'd;
      1,043 pairs queued; training will succeed on next cycle when VM restarts
      [2026-06-19 totebox@project-intelligence]
- [x] **LoRA target_modules fix** — OLMo 2 names: att_proj/ff_proj/ff_out/attn_out; startup
      assertion added; real LoRA training now possible [2026-06-16 totebox@project-intelligence]
- [x] **Bug 1: SHA-on-202-ACK** — repair-ledger.py (52746a3c) ran; stale SHA entries cleared;
      ~400 files will re-enrich automatically when Tier B restores [2026-06-16 totebox@project-intelligence]
- [x] **Doorman batch-extract endpoint** — POST /v1/batch/extract; Semaphore(4) Tier A /
      Semaphore(1) Tier B; CONTENT_BATCH_SIZE env var; commit e5c0ee4f [2026-06-16 command@claude-code]
- [x] **redrive-quarantine.py** — 737 quarantined briefs → queue; queue_quarantine=0
      [2026-06-16 command@claude-code]
- [x] **NEXT.md contamination repaired** — project-gis content replaced with correct
      project-intelligence state [2026-06-19 command@claude-code]
- [x] **overnight-aec-builds.sh path fix** — ingest-osm-parking.py was called from wrong dir;
      now uses `../pointsav-monorepo/app-orchestration-gis/ingest-osm-parking.py` [2026-06-17 totebox@claude-code]
- [x] **build-aec-seismic.sh EU join fix** — Step 8 `or` condition split into two separate `if`
      guards; EU vector join was skipping all clusters [2026-06-17 totebox@claude-code]
- [x] **build-aec-flood.sh AQUEDUCT threshold fix** — lowered from 100MB to 85MB; S3 file is
      92MB, causing every clean download to fail validation [2026-06-17 totebox@claude-code]
- [x] **PKS Phase 5b** — 7,045 clusters (T1=692/T2=2,665/T3=3,188); MX=177; false-US removed
      [2026-06-16 totebox]
- [x] **park-and-ride anchor ingest** — 23,117 records [2026-06-16 totebox]
- [x] **EU/US car rental + hotel chain ingests** [2026-06-16 totebox]
- [x] **PKS archetype rebalanced** — Fable analysis + mode-group collapse [2026-06-15 totebox]
- [x] **VWH retail_contamination badge** — showArchetypeDetail() badge for 3,048/6,368 clusters
      [2026-06-13 totebox]
- [x] **AEC wetland VRT fix** — 408 GWL_FCS30 5°-tiles assembled; gdal_translate removed (9c041f65)
      [2026-06-16 totebox]
- [x] **AEC wildfire numpy fix** — pure Python GDAL API (2ea45b07) [2026-06-16 totebox]
- [x] **ashrae_zone producer script** — build-ashrae-zone.py; 6,493/6,493 populated (dce0d157)
      [2026-06-16 totebox]
- [x] **PKS opportunity_class field** — SATURATED/EXPAND/DEVELOP per BRIEF §10.12 (2ea45b07)
      [2026-06-16 totebox]
- [x] **EFEHR seismic API (sample-eshm20-api.py)** — maps.efehr.org NXDOMAIN; main build script
      uses gitlab.seismo.ethz.ch tarball (unaffected); sample script is dev-only [2026-06-16 totebox]
- [x] **NEXT.md contamination (M-17)** — project-knowledge + project-intelligence content removed;
      GIS-only content restored [2026-06-17 totebox@claude-code]
# NEXT.md — project-console

> **Scope: this archive only (pointsav-monorepo Totebox).**
> Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Out-of-scope items route to outbox, not this file.

Last updated: 2026-06-19 [Jennifer Woodfine / claude-code]

---

## Phase H1 — seL4 unikernel substrate — COMPLETE 2026-06-19

| Item | Status | Notes |
|---|---|---|
| vendor-sel4-kernel AArch64 build | COMPLETE | `build/aarch64-qemu/kernel.elf` (910K, AArch64 ELF) |
| moonshot-sel4-vmm `#![no_std]` PD runtime | COMPLETE | `lib.rs`, `syscall.rs`, `debug.rs`, `types.rs`; seL4 ABI wrappers; cfg-gated AArch64 asm |
| `console_hello.c` bare-metal PD + TOML spec | COMPLETE | `moonshot-toolkit/examples/console_hello.c`; `os-console-hello.toml` |
| moonshot-toolkit image build | COMPLETE | `build/system-image.bin` (1.1M elfloader ELF) built via separate target-dir to avoid cargo-lock contention |
| QEMU boot verification | **GATE PASSED** | `Hello from os-console seL4 PD` on serial; QEMU `-m 1G` required (DTB reports 1 GiB; 512M causes Data Abort) |
| Phase H1 commit | COMMITTED | All Phase H1 files staged and committed |

[2026-06-19 totebox@claude-code]

## Phase H2 — next (Step 4 stretch: VirtIO serial PD + ratatui, ~200 lines)

- [ ] Implement VirtIO serial PD in moonshot-sel4-vmm (~200 lines Rust)
- [ ] Integrate ratatui rendering via VirtIO console backend
- [ ] Update moonshot-toolkit TOML spec: `examples/os-console-virtio.toml` (2-PD: console PD + virtio-serial PD)
- [ ] Gate: ratatui TUI renders in QEMU virtual terminal

---

## Phase 9 — Operations — COMPLETE 2026-06-14

| Item | Commit | What shipped |
|---|---|---|
| 1 — Graceful SIGTERM | `3e20be12` | `AtomicBool` + ctrlc handler; `request_shutdown()`; terminal restored on `systemctl stop` |
| 2 — fail2ban port 2222 | `5efb513d` | `infrastructure/fail2ban/jail.local` + filter; 5-retry, 1h ban |
| 3 — Prometheus metrics | `3e20be12` | `os_console_up` / `os_console_uptime_seconds` / `os_console_info` on loopback :9299; `metrics_port` config field |
| 4 — Multi-tab ContentCartridge | `a27860b3` | `TabSnapshot` + `Vec<tabs>`; Ctrl-T open, Ctrl-W close, Ctrl-Tab cycle; max 4 tabs; tab bar on >1 tabs |

---

## Stage 6 pending (Command scope — route via outbox)

All Phase 8+9+10+T0 commits + 2026-06-19 need `bin/promote.sh` from Command Session:

| SHA | Subject |
|---|---|
| `6f21f580` | feat(release): Phase B — CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | ops(session): Phase B complete |
| `d58960b4` | ops(brief): mark Phase B complete |
| `5c36ce66` | ops(monorepo): remove .agent/ from git index |
| `5efb513d` | ops(fail2ban): port 2222 brute-force protection |
| `3e20be12` | feat(sigterm+metrics): SIGTERM + Prometheus |
| `a27860b3` | feat(tabs): multi-tab ContentCartridge |
| `2c21e142` | ops(phase9): mark complete — NEXT.md + BRIEF |
| `469b7147` | test(tabs): 9 unit tests for tab management |
| `bc95acfa`..`fc4d0978` | Phase 10 commits (F2 People, reconnect watchdog, session persistence) |
| `5dab352e`..`91eb2148` | T0 pairing + tunnel fixes |
| `c9084667` | feat(content): pdfium-render optional — pdf feature flag |
| `3816794d` | docs(briefs): BRIEF-macos-binary-mac-pro |

## darwin-x86_64 binary pending (waiting on Jennifer)

- [ ] Jennifer builds on Mac Pro: `cargo build --release --bin os-console`
- [ ] Jennifer scps binary to `mathew@34.53.65.203:/tmp/darwin-x86_64-0.2.4`
- [ ] Deploy: scp to foundry-prod + chmod (instructions in BRIEF-macos-binary-mac-pro.md)
- [ ] Then: `curl -fsSL https://software.pointsav.com/releases/os-console/install.sh | bash` on Mac Pro

---

## Operator-gated items

- [ ] GCE firewall: open port 2222 inbound
- [ ] Deploy `local-console.service` systemd unit + enable
- [ ] `pairing-server` systemd unit on GCE VM
- [ ] Peter SSH key: `proofctl user add peter --tenant woodfine --role editor`
- [ ] Tag `v0.1.0` on pointsav-monorepo (triggers GitHub Actions release build)
- [ ] Branch rename `cluster/project-proofreader → cluster/project-console` on GitHub

---

## Phase 10 — next coding sprint (in-scope when ready)

| Item | What |
|---|---|
| F2 People cartridge | `app-console-people` lib + `PeopleCartridge`; read-only from `service-people :9091` |
| Chassis reconnect watchdog | retry MBA connection on drop; backoff; indicator in status bar |
| `/audit` log viewer | tail `service-input` ledger; search; export |
| Tab labels from state | improve `tab_label()` to pull actual query/title text live |

---

## Standing deferred

- F7 BIM cartridge — gated on `app-console-bim` activation
- F10 mesh cartridge — gated on `app-console-mesh` activation; Phase 1 scope when ready: poll `service-vm-fleet :9203` GET /v1/nodes → read-only table (node ID | hostname | ip | status | last_heartbeat | preferred role); no writes
- F11 → :9202 endpoint — currently polls :9201; will connect to `service-ppn-pairing :9202` when project-infrastructure deploys it (PPN Phase 1)
- Phase 12 (AI marginalia) — gated on SYS-ADR-07/10/19 review
