# slm-cli

The operator CLI and the single binary produced by this workspace.

Every other crate is a library. This is the one that compiles to
`target/release/slm-cli` and ships as the os-totebox appliance component,
the Cloud Run node entrypoint, and the developer one-off command.

## Subcommands

Scaffolded (not yet implemented):

- `slm-cli serve` — start the HTTP API + background workers.
- `slm-cli doorman --input <path>` — run one doorman cycle.
- `slm-cli ledger tail|export` — query the audit trail.
- `slm-cli node up|down|status` — manage the yo-yo node.
- `slm-cli adapter list|verify` — manage LoRA adapters.

See `slm-cli --help` for the current surface.

## Why this is the only binary

Per SLM-STACK §5.2, service-slm is flat. One process to install, start,
stop, update. One log stream. One signed artefact. If you find yourself
wanting a second binary, open an ADR first.
