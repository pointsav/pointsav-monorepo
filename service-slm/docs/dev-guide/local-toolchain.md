# Local toolchain

Details on the tools the repository uses and how to configure them for
maximum iteration speed.

## Rust toolchain

The channel is pinned in [`rust-toolchain.toml`](../../rust-toolchain.toml).
`rustup` reads this file automatically on any `cargo` invocation in
this directory, so you never need to pick a toolchain manually.

Updating the pinned toolchain is a governance event — it requires a PR
and CI pass on the new version across all targets. See the commit
history of `rust-toolchain.toml` for the rationale on each bump.

## sccache (optional, recommended)

`sccache` caches Rust compilation artefacts across projects and
branches. On a typical developer machine it cuts clean-build times by
3–5×.

```bash
cargo install sccache
# In your shell rc:
export RUSTC_WRAPPER=sccache
```

Then uncomment the `rustc-wrapper = "sccache"` line in
[`.cargo/config.toml`](../../.cargo/config.toml) if you want the
repository to opt in explicitly.

## cargo-audit

Scans the dependency tree for CVEs. Run via `./scripts/check-all.sh` or
directly:

```bash
cargo install cargo-audit
cargo audit
```

CI runs this on every PR and nightly.

## cargo-deny

Enforces the licence allow-list from [`deny.toml`](../../deny.toml).
Also catches banned crates, duplicate versions, and unknown sources.

```bash
cargo install cargo-deny
cargo deny check
```

CI runs four separate checks: `advisories`, `bans`, `licenses`,
`sources`. Run them locally with `cargo deny check <name>`.

## cargo-sbom and cargo-about

Used in the release workflow to produce the SBOM and third-party
notices. You do not need them locally unless you are debugging the
release pipeline.

```bash
cargo install cargo-sbom cargo-about
cargo sbom > sbom.json
cargo about generate about.hbs  # requires about.hbs template
```

## Cross-compilation

For Phase 3 appliance targeting:

```bash
rustup target add aarch64-unknown-linux-gnu
# Also install the cross linker on Linux:
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --bin slm-cli
```

macOS hosts need `cross` or `zig cc` for Linux targets; we have not
exercised this in Phase 2 and it will be wired up in Phase 3.
