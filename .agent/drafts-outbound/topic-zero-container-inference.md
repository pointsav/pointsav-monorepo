---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-slm
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-zero-container-inference.md
audience: vendor-public
bcsc_class: forward-looking
language_protocol: PROSE-TOPIC
authored: 2026-04-28
authored_by: task-project-slm (session 3620a18e52bc5329)
authored_with: opus-4-7
references:
  - conventions/zero-container-runtime.md
  - infrastructure/slm-yoyo/tofu/
  - infrastructure/local-slm/
  - service-slm/ARCHITECTURE.md §2 Ring 1 Bootstrap
notes_for_editor: |
  Skeleton — substance lands as Yo-Yo MIN deploys (currently gated
  per operator + B1-B4/W3-W4 from PS.1 readiness review). The
  TOPIC frames the SMB-economics and operational rationale for
  zero-container inference as a deployment posture, not a
  philosophical preference.

  BCSC class is forward-looking because Yo-Yo MIN deploy hasn't
  shipped yet (eight blockers/warnings still open per PS.1
  review 2026-04-27). Once Yo-Yo MIN is operationally verified,
  BCSC class flips to current-fact at refinement time.

  This TOPIC has cross-cluster relevance — project-data, project-
  knowledge, project-orgcharts also use OpenTofu + GCE for their
  deployment legs. Project-language gateway may merge or split
  with related TOPICs at refinement.
---

# TOPIC — Zero-Container Inference

(draft-pending — substance follows when Yo-Yo MIN deploys)

## Why no containers

(draft-pending — substance follows in milestone N+1)

OCI image format implies a container registry; the registry is
the durable artefact, not the binary. For a one-shot inference
VM that boots, runs for 30 minutes, stops — the container layer
adds operator surface (registry credentials, image build chain,
CVE management for the base image) without solving any problem
the VM doesn't solve more directly.

## What is used instead

(draft-pending — substance follows in milestone N+1)

Native binary in a `pointsav-public` GCE image family. systemd
unit ExecStart. OpenTofu provisioning. Idle-shutdown timer.
GCS-cached weights. Secret Manager for API keys. nginx for TLS
termination. CUDA driver baked into the image at build time.

## SMB economics

(draft-pending — substance follows in milestone N+1)

A100 80GB preemptible at ~$0.50-0.70/hr × 30-min daily window =
~$7-8/month. The economics close because the GPU is on for
exactly the moments inference happens, not for the operator
convenience of "always-warm". Idle-shutdown is the load-bearing
primitive.

## Cold-start: the only honest concern

(draft-pending — substance follows in milestone N+1)

A GCE GPU instance from stopped state takes 60-120 seconds to
ready. For latency-critical workloads, opt into warm-VM mode by
extending `idle_shutdown_minutes`. For everything else, the cold-
start is the cost of zero-idle-cost.

## Operational artefacts

(draft-pending — substance follows in milestone N+1)

OpenTofu module, the GCE image (CUDA + vLLM + nginx + idle-
shutdown timer + systemd unit), Secret Manager (bearer token,
provider API keys), Cloud Logging (audit log location in the
customer's own project), Cloud Billing budget + Pub/Sub kill-
switch (defense-in-depth against runaway spend).

## What this rules out

(draft-pending — substance follows in milestone N+1; cross-reference
conventions/zero-container-runtime.md)

Cloud Run. Kubernetes. SkyPilot. OCI image registries.
cargo-chef Docker layer caching. Container build chains.

## References

(draft-pending — citation IDs resolve at project-language refinement)
