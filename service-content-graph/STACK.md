# STACK.md
# Complete Technical Stack
**Version:** 1 · April 19, 2026
**Principle: 100% open source. Everything forkable. "We own it."**

---

## Critical License Findings

Five license discoveries from research that change plans:

1. **MinIO Community Edition is end-of-life** — archived read-only February 13, 2026.
   AGPL-v3 since 2021, admin console stripped in early 2025. **Use SeaweedFS instead.**

2. **leidenalg is GPL-3** — cannot link into a proprietary application.
   **Use graspologic-native (Microsoft, MIT, Rust Leiden) instead.**

3. **Most "open" LLMs are not OSI-approved.** Llama 3/4, Gemma, Mistral Large, Qwen large,
   and Jina Embeddings v3/v4 all have custom community licenses that restrict use.
   **Only OLMo 3 (AI2, Apache 2.0), Phi-4 (MIT), DeepSeek-R1 weights (MIT), Mistral 7B,
   and some Qwen 2.5 variants are genuinely OSI-approved.**

4. **DuckPGQ is still "community extension, under active development"** — missing
   full Kleene-star path queries needed for enterprise KG workloads. Use LadybugDB
   (MIT, Kùzu fork) as primary; DuckPGQ as SQL-native fallback/commutation partner.

5. **Raphtory is GPL-3** — treat as a service boundary, not a directly linked library.

---

## Full Stack Table

| Layer | Tool | Version | License | Notes |
|---|---|---|---|---|
| **Graph DB — Primary** | **LadybugDB** (Kùzu fork) | v0.15.2 | **MIT** | Active monthly releases. CLA removed at v0.12.0. NaviX HNSW vector index included. Bolt-protocol. WASM. |
| **Graph DB — Backup** | RyuGraph (Kùzu fork, Predictable Labs) | v25.9.2 | **MIT** | Enterprise-flavored. Full-text search + vectors added on top of Kùzu base. |
| **Graph DB — SQL Alt** | DuckDB + DuckPGQ extension | 1.1.3+ | **MIT** | SQL/PGQ (ISO standard). DARP I4 commutation partner — runs canonical test queries against same data as LadybugDB. Missing full variable-length path queries. |
| **Graph DB — RDF** | Oxigraph | latest | **MIT/Apache** | SPARQL 1.1. Second commutation engine for DARP I4 CI test. |
| **Vector index** | LadybugDB NaviX HNSW (built-in) | — | MIT | Primary. Same file as graph. |
| **Vector at scale** | LanceDB | — | **Apache 2.0** | Zero-copy Arrow, columnar, embedded. When LadybugDB NaviX isn't enough. |
| **Vector fallback** | FAISS or hnswlib | — | MIT / Apache 2.0 | Clean licenses. |
| **Embedding model** | nomic-embed-text-v1.5 | — | **Apache 2.0** | 137M params, 768-dim Matryoshka (down to 64), 8192-token context, fully open weights + data. Default. |
| **Embedding model alt** | BAAI/bge-m3 | — | **MIT** | Multi-lingual, 8192 context. |
| **Embedding model (no torch)** | Model2Vec potion-base-8M | — | **Apache 2.0** | Fallback for Laptop-A $7 VM constraint. |
| **Embedding API** | text-embedding-005 (Google) | — | Proprietary API | $0.006/MTok. Used during GCP batch jobs. ~$3 for full 2.5 GB corpus (one-time). |
| **AVOID: Embeddings** | Jina Embeddings v3/v4 | — | CC-BY-NC | **Non-commercial — do not use** |
| **Ingest LLM (GCP)** | Gemma 4 26B A4B (self-hosted) | — | **Apache 2.0** | Genuinely OSI-approved. Pay compute only, no generation API fees. |
| **Content gen LLM** | Claude API — Sonnet 4.6 | claude-sonnet-4-6 | Anthropic | Layer 5 wiki synthesis + content generation. Prompt caching enabled. |
| **LLM backup** | OLMo 3 (AI2) | — | **Apache 2.0** | Genuinely OSI-approved. |
| **LLM backup** | Phi-4 (Microsoft) | — | **MIT** | Genuinely OSI-approved. |
| **LLM backup** | DeepSeek-R1 weights | — | **MIT** | Genuinely OSI-approved. |
| **AVOID: LLMs** | Llama 3/4, Gemma (large), Mistral Large, Qwen large | — | Custom | **NOT OSI-approved despite "open" branding** |
| **Topic modeling** | FASTopic | NeurIPS 2024 | **MIT** | Supersedes BERTopic for multi-domain short text. Uses optimal-transport dual semantic relations. Doesn't discard outliers like HDBSCAN. |
| **Topic modeling backup** | BERTopic | 0.17.4 | **MIT** | SBERT → UMAP → HDBSCAN → c-TF-IDF. Safetensors serialization (never pickle). |
| **Community detection** | **graspologic-native (Microsoft)** | latest | **MIT** | Rust Leiden implementation. **Use this instead of leidenalg.** |
| **Community detection backup** | igraph + leidenalg | 0.11 / 0.11.0 | GPL/LGPL | **GPL — segregate via service boundary if used** |
| **Temporal graph** | Raphtory | 0.12+ | **GPL-3** | Treat as service boundary, not linked lib. Graph-view windowing maps to DKA cadence tiers. |
| **Derivative orchestration** | dbt-core (on LadybugDB/DuckDB) | 1.9+ | **Apache 2.0** | SQL-expressible layers (L0→L2). Incremental materialisations with `unique_key`. |
| **Job orchestration** | Dagster | 1.13+ | **Apache 2.0** | Python-heavy layers (embeddings, Leiden, BERTopic, wiki gen). FreshnessPolicy per cadence tier. |
| **Job orchestration (lightweight)** | APScheduler | 3.11 | **MIT** | Minimal VM path where Dagster is overkill. |
| **CSV file watching** | watchdog | 6.0 | **Apache 2.0** | Debounced. Falls back to 10-second polling in containerized environments where inotify is unreliable. |
| **Yo-yo compute** | SkyPilot | 0.11+ | **Apache 2.0** | Spot instance lifecycle management. Auto-recovery. Multi-cloud. Declarative YAML. Production refs: Shopify. |
| **Structured output** | Pydantic v2 + instructor | — | **MIT** | Citation grounding mandatory on all L5 wiki content. Every claim linked to L0 source. |
| **WORM object storage** | **SeaweedFS** | — | **Apache 2.0** | **NOT MinIO** — MinIO Community archived Feb 13, 2026. |
| **Immutability / provenance** | sigstore + cosign + in-toto + SLSA | — | **Apache 2.0** | Keyless signing. Rekor transparency mirrors. SBOM attestations on every OCI release. |
| **Graph portability** | IPFS CAR + OCI Artifacts | — | Open | Bootable portability layer. CIDs identical on both sides of air-gap. |
| **Wiki CMS** | Docusaurus or BookStack or MkDocs | — | MIT / MIT / BSD-2 | All clean licenses. |
| **AVOID: Wiki CMS** | WikiJS (AGPL), Outline (BSL), MediaWiki (GPL-2) | — | Copyleft/BSL | Contamination risk |
| **Consent CMP** | Klaro! | — | **BSD-3** | Self-hostable. No TCF runtime dependency. |
| **Live state DB** | SQLite (in service-content/state/) | — | Public domain | sync.db + YAML snapshots |
| **Key management (Phase 1)** | SSH environment variables | — | — | Not Secret Manager. Will upgrade post-trial. |

---

## Python Package List (fresh venv on MacPro/Laptop-A)

```
# Graph and vector
ladybugdb          # primary graph DB (or install from GitHub if not on PyPI)
duckdb             # SQL analytics + DARP I4 commutation
duckdb-extensions[vss,pgq]   # vector and graph extensions
oxigraph           # SPARQL engine for DARP I4 CI test

# Embeddings and ML
nomic              # nomic-embed-text-v1.5
sentence-transformers
scikit-learn
faiss-cpu          # fallback vector index

# Topic and community
fasttopic          # FASTopic (MIT, NeurIPS 2024)
bertopic>=0.17.4
graspologic        # MIT Leiden (NOT leidenalg)

# Extraction and structured output
pydantic>=2.0
instructor
keybert
yaml              # pyyaml

# Orchestration
dbt-core
dbt-duckdb
dagster
dagster-duckdb
apscheduler>=3.11
watchdog>=6.0
tenacity          # retry with exponential backoff
skypilot>=0.11    # yo-yo compute management

# Google / GCP
google-generativeai    # text-embedding-005 + Gemma 4 API
google-cloud-storage   # GCS checkpoints

# Anthropic
anthropic              # Claude API for L5 content generation

# SSH / networking
paramiko               # SSH tunnel to GCP

# Document processing
python-docx
pypdf

# Utilities
requests
python-dateutil
```

---

## Yo-Yo Recovery Pattern

```python
# Pattern that must be applied to all GCP batch jobs
import tenacity

@tenacity.retry(
    stop=tenacity.stop_after_attempt(5),   # hard cap — prevents runaway billing
    wait=tenacity.wait_exponential(multiplier=1, min=4, max=60),
    reraise=True
)
def run_batch_job(input_hash: str, job_version: str):
    """
    Jobs must be idempotent by (input_hash, job_version).
    GCS conditional writes prevent duplicate processing.
    Checkpoints written via atomic tmp-then-replace every N minutes.
    SIGTERM handler flushes in-memory state within 2-minute GCP grace window.
    Object storage is the ONLY source of truth — never local disk.
    """
    pass
```

---

## Claude API Best Practices

```python
# Prompt caching — graph subgraph + source chunks cached for 5 min (1 hr extended)
response = anthropic.messages.create(
    model="claude-sonnet-4-6",
    max_tokens=4096,
    system=[{
        "type": "text",
        "text": system_prompt,
        "cache_control": {"type": "ephemeral"}  # cached tokens ~10% of input cost
    }],
    messages=[{
        "role": "user",
        "content": assembled_context  # Topic node + contributing Chunks + Entities
    }]
)

# Extended thinking for Explanation pages. Disabled for Reference pages.
# Batch API (50% discount) for biennial archetype refresh yo-yo jobs.

# Citation grounding via Pydantic + instructor — MANDATORY on all wiki output
from pydantic import BaseModel
from typing import List

class SupportedClaim(BaseModel):
    claim: str
    support: List[str]  # e.g. ["L0-asset-id:456-500", "L0-asset-id-2:123-200"]

class WikiPage(BaseModel):
    title: str
    body: List[SupportedClaim]  # every claim must have support or is dropped
```
