---
title: lbug 0.16.1 build blocker — service-content deploy
created: 2026-05-13
status: blocked
---

# lbug 0.16.1 static prebuilt missing companion libs

## Problem

`service-content` code fixes are committed (b8a70ee) but cannot be deployed because
lbug-0.16.1's prebuilt `liblbug.a` is missing ALL companion static libraries (fastpfor,
parquet, thrift, snappy, etc.). Three build approaches tried:

| Attempt | Command | Result |
|---|---|---|
| Static prebuilt | `cargo build --release -p service-content` | FAIL — `undefined symbol: __fastpack14` |
| Build from source | `LBUG_BUILD_FROM_SOURCE=1 cargo build --release -p service-content` | FAIL — cmake ran, linker still errors (unknown root cause) |
| Shared library | `LBUG_SHARED=1 cargo build --release -p service-content` | BUILD SUCCEEDED — but service consumes 2 GB RSS on startup (LadybugDB mmap init) → OOM-killed by systemd MemoryMax=2G |

## What exists on disk

- **Committed code fix**: `service-content/src/main.rs` (commit b8a70ee) — both bugs fixed
- **Old working binary**: `/usr/local/bin/service-content` — REPLACED by broken LBUG_SHARED build; 
  old binary was 17.7 MB (May 8, built with static lbug that worked); now replaced with 2.9 MB shared binary
- **cmake build output**: `/srv/foundry/cargo-target/release/build/lbug-9afbb0d821db0455/out/build/`
  — complete cmake build with `liblbug.a` (95 MB static) + all third_party `.a` files + `liblbug.so`
- **`liblbug.so.0.16.1`**: installed at `/usr/local/lib/liblbug.so.0` (for the shared binary)
- **`libfastpfor.a`**: placed at `.cache/lbug-prebuilt/lib/` AND `lbug-src/build/release/third_party/fastpfor/`
  (both locations attempted, neither resolved the full link error)
- **`local-content.service`**: currently STOPPED (inactive)

## Nightly run

NOT triggered — awaiting service-content deploy. Timer active for 2026-05-14T00:00 UTC (auto).

## Next steps (resume)

### Option A — Fix LBUG_BUILD_FROM_SOURCE link search

The `LBUG_BUILD_FROM_SOURCE=1` build runs cmake (fast no-op since output exists) and should emit
`cargo:rustc-link-search` for all third_party dirs. When it fails, check the exact linker error:

```bash
LBUG_BUILD_FROM_SOURCE=1 cargo build --release -p service-content 2>&1 | grep "undefined symbol" | head -20
```

Then verify all third_party `.a` files exist in the cmake output:

```bash
for lib in utf8proc antlr4_cypher antlr4_runtime re2 fastpfor parquet thrift snappy zstd miniz mbedtls lz4 roaring_bitmap yyjson; do
    f=$(find /srv/foundry/cargo-target/release/build/lbug-9afbb0d821db0455/out/build/third_party -name "lib${lib}.a" 2>/dev/null | head -1)
    echo "$lib: ${f:-MISSING}"
done
```

### Option B — Raise MemoryMax and use shared library

The LBUG_SHARED=1 binary at `/usr/local/bin/service-content` works. LadybugDB may legitimately
need >2 GB for mmap init. Raise `MemoryMax=4G` in the systemd unit and restart:

```bash
sudo systemctl edit local-content.service
# Add: [Service]
# MemoryMax=4G
sudo systemctl daemon-reload
sudo systemctl start local-content.service
journalctl -u local-content.service -f
```

Monitor if RSS stabilises after graph store init or continues climbing.

### Option C — Pin to lbug 0.16.0

Change `service-content/Cargo.toml`: `lbug = "0.16"` → `lbug = "=0.16.0"`.
lbug-0.16.0 builds from source; no prebuilt; all companion libs linked correctly.
**Takes another ~45 min cmake build.**

## After deploy (whichever option works)

```bash
sudo systemctl start local-content.service
journalctl -u local-content.service -f   # confirm [TAXONOMY] Loaded + [SYSTEM] Surveillance Engaged
git status  # confirm clean
sudo systemctl start nightly-run.service
journalctl -u nightly-run.service -f
```
