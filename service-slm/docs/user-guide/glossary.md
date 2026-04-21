# Glossary

Terms used throughout service-slm documentation.

**Adapter.** A LoRA module that sits on top of the base model and
encodes task-specific behaviour. Small (~50 MB), versioned,
Sigstore-signed. See Ring 3b.

**BLAKE3.** The cryptographic hash used for `input_hash` in the
ledger and for KV block hashing.

**Bootstrap.** Ring 1 of the three-ring memory model: the container,
weights, and secrets needed to start an inference engine.

**Cold start.** The time between `BOOT_REQUEST` and `BOOT_COMPLETE`
for a previously-torn-down node.

**CoA.** Chart of Accounts. The foundational classification vocabulary
for Woodfine-related financial documents; a shared adapter is
trained on it.

**Compute manifest.** A YAML document in `compute/manifest.yaml`
describing the GCP node configuration (image, GPU tier, region,
scale parameters).

**CLA.** Contributor License Agreement. A signed agreement that every
contributor must have on file before their pull request can be merged.
Enforced via CLA Assistant per `factory-release-engineering/` policy
for AGPLv3 repos.

**Doorman protocol.** The five-step sanitise / send / await /
receive / rehydrate procedure that every external call flows through.

**AGPL-3.0-only.** GNU Affero General Public License, version 3. The licence
covering PointSav-authored code in this repository. See
[ADR-0003](../adr/0003-agpl3-for-own-code.md).

**Ledger.** The append-only CSV (with SQLite mirror) that records
every inference event. SOC3 processing-integrity artefact.

**LMCache.** The vLLM / mistral.rs-integrated KV cache connector
that fronts Mooncake Store.

**Ledger event.** A single row in the audit ledger. Ten variants;
see [architecture/05-ledger-schema.md](../architecture/05-ledger-schema.md).

**mistral.rs.** The Rust-native LLM inference engine. Replaces vLLM
in Phase 2; see [ADR-0002](../adr/0002-mistralrs-over-vllm-phase-2.md).

**moduleId.** The string that namespaces a project across all five
service-slm layers: bootstrap, KV cache, graph, adapters, ledger.
See [YOYO-COMPUTE §6](../../specs/YOYO-COMPUTE.md).

**Mooncake Store.** The C++ distributed KV cache pool developed at
Moonshot AI. service-slm talks to it over its wire protocol as a
sidecar.

**os-totebox.** The eventual PointSav archive appliance. service-slm
is a prototype component of it.

**PointSav.** The product brand. Separate from the corporate
copyright holder (Woodfine Capital Projects Inc.).

**Ring 1 / Ring 2 / Ring 3a / Ring 3b.** The four tiers of the
three-ring memory model. See
[architecture/04-three-ring-memory.md](../architecture/04-three-ring-memory.md).

**Sigstore.** The keyless code-signing system used for release
artefacts.

**SkyPilot.** The Python multi-cloud orchestration tool used in
Phase 1 only. Replaced by direct `google-cloud-rust` use in Phase 2.

**SLSA.** Supply-chain Levels for Software Artifacts. The framework
under which we produce provenance attestations at release time.

**Totebox.** The archive appliance form factor. `Laptop-A` is the
low-RAM reference host.

**vLLM.** The Python-native LLM inference server used in Phase 1.
Replaced by mistral.rs in Phase 2.

**Woodfine.** Woodfine Capital Projects Inc., the copyright holder
and commercial steward of this project.

**Yo-yo.** The scale-to-zero-and-back pattern for GPU inference.
Node comes up on demand, serves, tears down. See
[user-guide/yoyo-substrate.md](./yoyo-substrate.md).
