# ⚙️ SERVICE-SLM: LINGUISTIC AIR-LOCK
**Vendor:** PointSav Digital Systems™
**Standard:** SYS-ADR-07 (Bifurcated Ingestion)
**Tier:** 5 (Service Logic)

## I. ARCHITECTURAL MANDATE
This component acts as the Sovereign Dispatcher. It is a highly controlled, headless bridging environment that sanitizes unstructured human data before it enters the self-healing knowledge graph.

## II. THE COGNITIVE FORGE
The core of this service is the **Cognitive Forge** (`cognitive-forge.rs`), a localized Rust binary that interfaces with a sub-billion parameter Small Language Model. It consumes raw text from the `transient-queues`, extracts facts, and outputs clean Markdown.
