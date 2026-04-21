# The doorman protocol

The doorman protocol is the five-step procedure every external call to
service-slm follows. It exists to keep PointSav-trusted data inside the
trust boundary even when compute happens outside it.

## The five steps

1. **Sanitise.** The outbound payload is inspected against a
   `SanitisationPolicy`. Fields marked sensitive are stripped and
   replaced with stable references. A round-trip contract holds: the
   original payload can be reconstructed from the sanitised payload
   plus a local re-attachment table. Anything that cannot be proven to
   round-trip refuses the call.
2. **Send.** The sanitised payload is shipped to the compute target —
   either `slm-inference-local` (for Totebox-capable hosts) or
   `slm-inference-remote` (for the GCP yo-yo node). The transport
   records a `JOB_START` ledger row.
3. **Await.** The caller blocks, with timeout and retry, on the
   compute target's response. Retry policies are exponential backoff
   bounded at five attempts by default.
4. **Receive.** The response is validated against a schema generated
   from the expected return type's JSON Schema. Validation uses the
   `validator` crate per SLM-STACK §3.7. Invalid responses are
   rejected; they never propagate into domain code.
5. **Rehydrate.** Sensitive fields stripped in step 1 are re-attached
   locally using the re-attachment table. A `JOB_COMPLETE` ledger row
   is written.

## Why this shape

This is the canonical trust-boundary pattern when computation has to
happen outside the trusted zone. It is used by end-to-end encrypted
messaging systems for the same reason we use it here: the external
system is assumed to be potentially compromised, so it never receives
anything it is not authorised to see.

The rehydration step is what distinguishes us from "just use a proxy."
A proxy sits between the caller and the compute target but does not
change what the compute target sees; the doorman changes both ends of
the conversation so the target never receives sensitive content.

## Invariants

- **Sanitisation is the only gate** between trusted data and external
  compute. A bug here is a data-leak bug. `slm-doorman` carries
  property tests asserting `rehydrate ∘ sanitise = id`.
- **Every cycle writes at least two ledger rows:** `JOB_START` and
  `JOB_COMPLETE` (or an error row).
- **No silent fallback from sanitisation failure.** A refusal is a
  typed error, never a degraded response.

## Where it lives

`crates/slm-doorman/` owns the protocol. The sanitisation policy is a
trait; `slm-core` owns the shared error types. See
[`CLAUDE.md`](../../crates/slm-doorman/CLAUDE.md) for implementation
invariants.
