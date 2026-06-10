---
schema: foundry-cluster-manifest-v1
cluster: project-intelligence
cluster_name: project-intelligence
cluster_branch: cluster/project-intelligence
created: 2026-05-05
state: active
slm_endpoint: http://localhost:9080
module_id: intelligence
doctrine_version: 0.1.x
doctrine_claims_codified: [37, 43, 44]
doctrine_claims_proposed: []

operator: pointsav (Mathew)
working_pattern: infrastructure-first

tetrad:
  vendor: pointsav-monorepo (service-slm, slm-doorman-server)
  customer: leg-pending
  deployment: vault-privategit-source-1 (Doorman :9080, service-slm :9081)
  wiki: leg-pending

mission: |
  Doorman infrastructure and local-inference substrate. Owns
  slm-doorman-server (Tier A/B/C routing, apprenticeship, audit ledger)
  and service-slm (OLMo 7B inference server). Provides the foundry MCP
  server (slm-mcp-server) consumed by all other archives.
---
