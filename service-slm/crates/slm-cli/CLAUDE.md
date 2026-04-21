# CLAUDE.md — slm-cli

You are working on the only binary in the workspace. Every subcommand
wraps a library crate; the CLI itself should contain very little
business logic.

## What this crate owns

- Argument parsing (`clap`).
- Tracing subscriber initialisation.
- Top-level error handling and exit-code mapping.
- Subcommand dispatch.

## Invariants specific to this crate

1. **`anyhow::Error` is acceptable here** because the consumer is a
   human reading the terminal. Every other crate uses typed errors.
2. **No subcommand larger than ~30 lines.** If a subcommand needs more
   than that, extract a function into the relevant library crate.
3. **`--help` is a deliverable.** Every subcommand has a clear
   description and examples.
4. **Subcommands map 1:1 to library crates** wherever possible. This
   keeps the CLI a thin facade.

## Next work units

See `TASKS.md`. Each unimplemented subcommand in `main.rs` is a
pointer to the next work unit in the crate it will call.
