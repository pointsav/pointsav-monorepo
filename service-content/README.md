# service-content
### *Asset & Knowledge Synthesis Engine*

**Status: Active Engineering** | **Taxonomy: Tier-5-Service**

This component is a stateless processing engine designed to synthesize institutional knowledge. It orchestrates the flow of data between the decentralized Totebox Archives (State) and external linguistic compilers (LLM APIs) to generate BCSC-compliant corporate documentation.

## 🏛️ Architectural Mandate: Stateless Compute
`service-content` holds no persistent memory and stores no proprietary data. It relies entirely on the File-Over-Database architecture. 

When triggered, the engine executes a deterministic loop:
1. **Ingest Rules:** Reads the `protocols/` directory to establish the legal and structural boundaries (e.g., Anti-Puffery, Nomenclature Lock).
2. **Ingest Context:** Pulls raw data (`RESEARCH`), plot structures (`THEMES`), and tone guidelines (`VOICE`) from the target `content-wiki-*` repository located inside a secure `os-totebox`.
3. **Linguistic Synthesis:** Transmits a strictly packaged payload to an external AI API (e.g., Gemini) for text generation. *Note: The engine can bypass this step and utilize static string concatenation if AI processing is not authorized for a specific protocol.*
4. **Local Verification:** Executes a local regex blacklist check against the generated output to ensure strict compliance.
5. **Output Routing:** Saves the final Markdown document back into the `content-wiki-*` repository or a designated `outbox/` for human review.

## 🛡️ Security Posture
The engine operates inside an Isolated Protection Domain (PD). It utilizes the PointSav Capability-Based Manager to request read/write access to specific Totebox Archives. The external API connection is structurally restricted to a one-way outbound request, preventing the LLM from executing commands on the local hardware.
