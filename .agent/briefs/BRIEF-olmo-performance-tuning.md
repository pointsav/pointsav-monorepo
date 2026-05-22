---
topic: OLMo 7B Q4_K_M performance tuning
created: 2026-05-21
status: in-progress
---

# OLMo 7B Q4_K_M Performance Tuning

## Baseline (2026-05-21, threads=3)
- Generation: 1.71 tok/s
- Prompt-eval: 3.56 tok/s
- Bottleneck: **memory bandwidth**, not compute

llama-server: v1 (dcad77c), Intel Xeon @ 2.20 GHz (8 vCPUs, AVX2), CPU-only

## Tuning applied
- `--threads 3` → `--threads 6`
- Added `--threads-batch 8`
- `CPUQuota=350%` → `CPUQuota=600%`
- Applied to `/etc/systemd/system/local-slm.service` (system file; NOT in git — tracked in `~/Foundry/infrastructure/local-slm/`)

## Tuned result (2026-05-21, threads=6+batch=8)
- Generation: 1.95 tok/s (+14%)
- Prompt-eval: 2.60 tok/s (slight regression due to test running after large cached prompt)
- Wall-clock for "list 3 primes" (13 prompt + 16 generated = 29 tokens): 13.2 s

## Why thread count doesn't help much
The 7B Q4_K_M weights are ~4.2 GiB. Each token requires a full pass over these weights.
Memory bandwidth on this VM (~20–25 GB/s) is saturated at even 2–3 threads.
Going 3→6 threads adds ~14% because the extra threads can slightly overlap DRAM prefetch
but the bandwidth wall is hit quickly.

## Higher-impact options (not yet done)
| Option | Expected gain | Cost |
|---|---|---|
| IQ4_XS quantization (~3.5 GiB) | ~1.2x | Model re-download; slight quality loss |
| Q4_0 (faster but lower quality) | ~1.3x | Same size, simpler kernel |
| Newer llama-server binary (b5000+) | ~1.2x | Build from source or download |
| GPU offloading (any GPU) | ~10–50x | Requires VM with GPU; Yo-Yo path |
| mistralrs-server (SLM-STACK D43 target) | ~1.3–1.5x | New binary, FlashInfer kernel |

## Note on task 0 / queue replay
When local-slm restarts, if Doorman has a queued/in-flight request it immediately
re-dispatches on startup. The first test request was blocked ~9 min behind a 757-token
generation from task 0 (likely replayed from the pre-restart stuck slot). This is
correct Doorman queue behavior, but means test latency includes queue drain time.
To get clean timing: restart local-slm, wait for slot idle, then test.

## Next steps
1. Try `--flash-attn on` (currently auto; may or may not activate on Q4_K_M)
2. Consider IQ4_XS download if quality acceptable
3. Track mistralrs-server as the D43 target (SLM-STACK spec)
4. GPU is the only path to usable real-time speed on this class of model
