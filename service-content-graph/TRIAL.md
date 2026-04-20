# TRIAL.md
# Trial Run — 3-File Validation
**Version:** 1 · April 19, 2026
**Goal:** Full pipeline validation on 3 files before committing to the full 2.5 GB corpus run.
**The single most important output:** actual tok/s on the chosen GPU tier.

---

## Why Trial First

The full 2.5 GB corpus run costs $200–$340 depending on GPU tier and actual throughput.
Running 3 files first costs < $5 and answers the critical calibration question: what is the
actual tok/s throughput on the chosen GPU? This determines whether the full run takes 20 hours
or 85 hours and whether the cost estimate is correct.

Do not skip the trial. Do not commit to full corpus spend without trial validation.

---

## Trial Dataset

Three source files already in service-fs on Laptop-A:
- `INVESTOR RELATONS_2024_02_28_Design Slides_Victoria Johnson_1 to end_MW4_JW4_DW copy 2.docx`
- `INVESTOR RELATONS_2024_05_14_Design Slides_Hengming Zhang_Woodfine Response copy.docx`
- `INVESTOR RELATONS_2024_06_13_Design Slides_Victoria Johnson_Woodfine Response_Supplement copy.docx`

/ledger and /assets already exist for all three files (extracted by Gemini API).
Domain: Corporate · Category: Investor Relations · CoA macro: Stakeholder & Reporting

---

## Pre-Trial Checklist

### GCP
- [ ] Cloud Run service provisioned (see GCP-NODE.md)
- [ ] GPU tier confirmed available in chosen region
- [ ] Gemma 4 26B A4B loaded and responding to test prompt
- [ ] text-embedding-005 API key active and tested
- [ ] GCS checkpoint bucket confirmed
- [ ] Budget alert configured

### MacPro (development)
- [ ] Python 3.10.11 confirmed
- [ ] venv active
- [ ] `pip list` run and reviewed — all packages from STACK.md installable
- [ ] Pipeline scripts written (ingest.py, apply_delta.py, verify_graph.py, snapshot.py)
- [ ] Scripts deployed to Laptop-A via SSH

### Laptop-A (substitute Totebox Archive)
- [ ] `python3 --version` confirmed
- [ ] LadybugDB installed and database initialized
- [ ] SQLite sync.db initialized
- [ ] SSH key pair to GCP working
- [ ] 3 trial /ledger files confirmed in service-fs
- [ ] 3 trial /assets files confirmed in service-fs
- [ ] SHA-256 checksums recorded for all 6 files

### Schema
- [ ] LadybugDB schema initialized (all node and relationship tables from SCHEMA.md)
- [ ] NaviX HNSW vector index created on Chunk.embedding
- [ ] seeds/ CSV files in place (coa.csv, archetypes.csv, domains.csv)

---

## Trial Run Steps

| Step | Action | Pass criterion |
|---|---|---|
| 1 | LadybugDB schema initialization | All tables exist, vector index created |
| 2 | service-content/state/sync.db initialized | 3 files registered as "pending" |
| 3 | Sanitise payload (service-slm) | 3 /ledger + /assets pairs sanitised, PII stripped |
| 4 | Send to GCP via SSH tunnel | GCP node receives payload without error |
| 5 | Gemma 4 processes 3 files | Graph delta returned: Document + Chunk + Entity + Metric nodes |
| 6 | Derivative layers synthesized | Domain / Glossary / Topic / Archetype / Theme nodes in delta |
| 7 | Chunks embedded via text-embedding-005 | 768-dim vectors returned for all Chunk nodes |
| **8** | **Record actual tok/s throughput** | **tok/s recorded → full corpus cost calculated** |
| 9 | Delta received by service-slm | Delta file present and parseable |
| 10 | Entity resolution pass | "Victoria Johnson" → single canonical node · "Hengming Zhang" → single canonical node |
| 11 | Delta applied to LadybugDB | apply_delta.py completes without error |
| 12 | Integrity check | verify_graph.py passes: all expected nodes present, no orphaned edges, vector index populated |
| 13 | GCP node tears down | Zero instances running · cost confirmed < $5 |
| 14 | sync.db updated | 3 files marked "ingested" in SQLite |
| 15 | YAML snapshot generated | Snapshot written to `state/snapshots/[date]/` |
| 16 | Domain nodes confirmed | At least 1 Domain node (corporate) |
| 17 | Glossary nodes confirmed | At least 3 Glossary terms detected |
| 18 | Topic nodes confirmed | At least 5 Topic nodes with Chunk references |
| 19 | SUPERSEDES relationship | June 2024 file correctly linked to May 2024 and Feb 2024 via `[:SUPERSEDES]` |
| 20 | Test query 1 — entity | "Who are the key contacts in the investor relations corpus?" → coherent response with L0 citations |
| 21 | Test query 2 — factual | "What is the minimum investment threshold for Woodfine LPs?" → $250,000 with L0 citation |
| 22 | Test query 3 — theme | "What themes are currently active?" → sensible Theme list |
| 23 | Citation grounding check | Every wiki page claim has at least one `L0-asset-id:char-offset` citation |
| 24 | DARP I4 commutation test | DuckDB SQL + Oxigraph SPARQL produce identical answer sets for 5 canonical queries |

---

## Pass / Fail Criteria

**Trial PASSES if:**
- Steps 1–19 complete without error
- Entity resolution produces single canonical nodes for both named individuals
- At least 5 Topic nodes, 3 Glossary terms, 1 active Theme detected
- SUPERSEDES relationship correctly inferred from file naming and dates
- All 3 test queries return coherent, citation-grounded responses
- No hallucinations without citation in wiki output (instructor enforces this)
- tok/s throughput recorded and > 40 tok/s on L4 (indicates correct GPU config)
- DARP I4 commutation test passes

**Trial FAILS if:**
- Any pipeline script produces an uncaught exception
- Entity resolution creates duplicate canonical nodes for same individual
- No derivative layer nodes generated (only Document/Chunk)
- Claude API responses contain unsupported claims (uncited hallucinations)
- GPU throughput < 10 tok/s (wrong hardware tier or configuration error)
- DARP I4 test produces divergent answer sets between DuckDB and Oxigraph

---

## After the Trial

### If trial passes:
1. Calculate full corpus run cost: `actual_tok/s × 2.5 GB token count / 3600 × $/hr`
2. Compare against GPU tier options (see GCP-NODE.md cost table)
3. Select optimal GPU tier for full run
4. Confirm budget with stakeholder
5. Proceed to full 2.5 GB corpus run

### If throughput is lower than expected:
- Check GPU configuration: is the correct GPU type actually running?
- Check model precision: is Gemma 4 running in float16 (correct) or float32 (slower)?
- Check batch size: tune chunk batching parameters
- Consider upgrading to A100 80GB even for trial

### If cost is too high:
- Reduce chunk size to reduce token count
- Optimize Gemma 4 prompt (fewer tokens in, same quality out)
- Consider 4-bit quantization (quality tradeoff — test on trial first)

---

## What NOT to Do

- Do not run the full 2.5 GB corpus without completing the trial
- Do not skip the tok/s measurement step — it is the most important output
- Do not proceed if DARP I4 test fails — fix the commutation issue first
- Do not manually seed Topic or Archetype nodes — they must emerge from the data
- Do not publish any wiki pages until the full corpus run is complete and reviewed
