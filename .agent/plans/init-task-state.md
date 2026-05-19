# Plan: Initializing TASK State & Signaling Promotion Readiness

## **Objective**
Acknowledge master instructions, signal readiness for Stage 6 promotion of verified commits, and propose the final classification mapping for root YAML files in `content-wiki-documentation`.

## **Key Files & Context**
- **Inbox:** `.agent/inbox.md` (6 commits pending signal)
- **Outbox:** `.agent/outbox.md` (Target for readiness signal)
- **Repo State:** `content-wiki-documentation/` root (YAML files for classification)
- **Logs:** `content-wiki-documentation/.agent/rules/cleanup-log.md`

## **Implementation Steps**

### **1. Signal Readiness for Stage 6 Promotion**
- Create a new entry in `.agent/outbox.md` specifically responding to the master sweep.
- Explicitly confirm that all 6 SHAs (Jennifer/Peter's corporate updates and Sprints G–K in the monorepo) are verified and ready for `bin/promote.sh`.

### **2. Classify Root YAML Files**
Propose the following mapping for the remaining root YAML files in `content-wiki-documentation` (to be executed in a follow-up session or by an editorial sub-agent):
- **To `services/`:** `service-content-01.yaml`, `service-egress-01.yaml`, `service-email-01.yaml`, `service-people-01.yaml`.
- **To `governance/`:** `sys-adr-06.yaml` through `sys-adr-19.yaml`.
- **To `architecture/`:** `3d-asset-tokens.yaml`, `os-workplace-01.yaml`, `system-slm.yaml`, `system-udp.yaml`.
- **To `reference/`:** `leapfrog-facts.yaml`.

### **3. Update `NEXT.md`**
- Mark the Stage 6 signal as complete.
- Add the YAML classification mapping to the "Currently open" section of `content-wiki-documentation/NEXT.md`.

## **Verification & Testing**
- **Outbox Verification:** Ensure the outbox message matches the schema and explicitly references the SHAs from the inbox.
- **Manifest Check:** Verify the `completed_topics_this_milestone` count reflects the recent BIM and Wikipedia-parity publications.
