# slm-doorman

The doorman protocol. This is the crate every external call to service-slm
ultimately flows through.

The protocol is five steps:

1. **Sanitise** — strip sensitive fields from the outbound payload.
2. **Send** — ship the sanitised payload to the compute target
   (`slm-inference-local` or `slm-inference-remote`).
3. **Await** — block for a response with timeout and retry.
4. **Receive** — validate the incoming response against the expected
   schema (using `validator` + `schemars`, per SLM-STACK §3.7).
5. **Rehydrate** — re-attach sensitive fields locally before the response
   crosses back into PointSav's trust boundary.

Full specification: [SERVICES.md §Two Outbound Roles](../../specs/)
(original service-slm spec, referenced from YOYO-COMPUTE §12).

## What lives here

- The `DoormanRequest` and `DoormanResponse` types.
- The sanitisation rules registry.
- Retry and backoff policies.
- The rehydration logic and its inverse checks.

## What does not live here

- The transport: that's `slm-inference-{local,remote}`.
- The audit log: that's `slm-ledger`.
- The HTTP surface: that's `slm-api`.
