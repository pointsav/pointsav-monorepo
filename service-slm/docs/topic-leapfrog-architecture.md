# TOPIC: Leapfrog 2030 Architecture & Multi-Yo-Yo Pipeline

## 1. Executive Summary
This document outlines the "Leapfrog 2030" architecture for PointSav's Datagraph generation and AI training pipeline. It utilizes a **Multi-Yo-Yo Compute Pool** managed by `service-slm` (The Doorman) to balance massive scale, zero-hallucination graph extraction, and cost-efficiency.

## 2. Core Architecture
- **Service-SLM (The Router):** Manages a dynamic pool of Yo-Yo instances (Tier B) alongside Tier A (local) and Tier C (External API). It captures all daily CLI work via `capture-edit.py` to generate engineering tuples.
- **Service-Content (The Synthesizer):** Enforces strict ontological schemas (`Archetypes.json`, `ChartOfAccounts.json`, `Domains.json`, `Themes.json`). It reads from the LadybugDB graph to draft Institutional-Grade content, polishing it via Doorman's Tier C Proxy (Claude 3.5 Sonnet).

## 3. The Multi-Yo-Yo Pool
The architecture decouples the software from the hardware, relying on two distinct Tier B hardware profiles:

### Yo-Yo #1 (The "Trainer")
- **Hardware:** 1x L4 GPU on a GCP Spot Instance (`g2-standard-4`).
- **Model:** OLMo 3 32B-Think (`Q4_K_M` GGUF).
- **Role:** Continuous background learning. It processes the SFT engineering tuples captured by the Doorman.
- **Cost Optimization:** Runs on the "Night Shift" (e.g., 11 PM to 6 AM PT) to guarantee Spot availability. It relies on the pre-configured `mistralrs-idle.timer` (`idle_shutdown_minutes`) to auto-shutdown after 30 minutes of inactivity, guaranteeing zero wasted spend.

### Yo-Yo #2 (The "Graph Extractor")
- **Hardware:** 1x H100 GPU on a Dedicated On-Demand Instance (`a3-highgpu-1g`).
- **Model:** Llama 3.3 70B (Open-weights frontier-level reasoning).
- **Role:** Batch processor. Spun up manually by the Operator to ingest massive archives (e.g., the 1,600+ files in `cluster-totebox-jennifer`). It builds the LadybugDB graph with Claude-level accuracy for ~$50, keeping all data strictly private.
- **Ontological Strictness:** The Doorman passes the `service-content` schemas to Yo-Yo #2 as strict JSON constraints (via `grammar` injection), forcing Llama 3.3 70B to perfectly map output to PointSav's taxonomy.

## 4. Comprehensive To-Do List

### Phase 1: Infrastructure Unblocking (Master Agent Scope)
- [ ] **Create GCP Project:** Physically create the `pointsav-public` GCP project.
- [ ] **Author D4 Image Pipeline:** Write the Packer/OpenTofu pipeline to build the base GCE image for the Yo-Yo fleet (Ubuntu 24.04 + CUDA + vLLM >= 0.12 + Nginx TLS + `idle_shutdown_minutes` systemd timer).
- [ ] **Bake and Publish Image:** Run the D4 pipeline to publish the image to the `pointsav-public` family.

### Phase 2: Service-SLM Configuration (Task Agent Scope)
- [ ] **Multi-Yo-Yo Support:** Refactor `service-slm/crates/slm-doorman-server/src/main.rs` to support multiple endpoints (e.g., `SLM_YOYO_TRAINER_ENDPOINT` and `SLM_YOYO_GRAPH_ENDPOINT`).
- [ ] **Tier C Auth:** Add billing-capped API keys for Claude/Gemini to `/etc/local-doorman/local-doorman.env` to enable the `audit_proxy` for service-content polishing.
- [ ] **Grammar Constraints:** Ensure `service-slm` can read the JSON/CSV seeds from `service-content` and pass them as explicit `grammar` rules to Yo-Yo #2 during graph extraction.

### Phase 3: Yo-Yo Deployment
- [ ] **Deploy Yo-Yo #1 (Trainer):** Apply the OpenTofu module configuring `g2-standard-4` as a Spot instance. Verify the night-shift schedule and idle-shutdown timer are operational.
- [ ] **Deploy Yo-Yo #2 (Extractor):** Author a secondary OpenTofu profile for the `a3-highgpu-1g` Dedicated instance. Deploy only when ready to process `cluster-totebox-jennifer`.

### Phase 4: Service-Content Orchestration
- [ ] **Seed Alignment:** Verify `Archetypes.json`, `Domains.json`, and `Themes.json` are fully locked and exported for `service-slm` consumption.
- [ ] **Batch Ingestion:** Feed the 1,600+ deployment files into Yo-Yo #2 and monitor LadybugDB graph generation.
- [ ] **Tier C Drafting Pipeline:** Configure `service-content`'s output compiler to query LadybugDB, package the subgraph into a 2,000-token payload, and proxy it to Claude 3.5 Sonnet (via Doorman Tier C) for final document generation.

## 5. Notes on service-content Adjustments
- `service-content` must shift from being a standalone parser to acting as the **Graph Query and Publishing Engine**.
- The `content-compiler.rs` needs to be updated to query `entities.lbug` directly, filtering by `Domain` and `ChartOfAccounts` IDs.
- Drafting logic must wrap the queried facts into a strict payload before dispatching an `AuditProxyRequest` to `service-slm` to trigger the Tier C generation.
