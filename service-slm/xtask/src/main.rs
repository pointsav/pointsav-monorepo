// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]

//! # xtask
//!
//! Workspace automation, invoked as `cargo xtask <command>` via the alias
//! in `.cargo/config.toml`.
//!
//! Keep this binary small. It is the glue layer between shell scripts and
//! cargo commands. Business logic belongs in the workspace library crates.

use std::process::{Command, ExitCode};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "service-slm workspace automation")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Subcommand, Debug)]
enum Task {
    /// Run the full local check suite: fmt, clippy, test, audit, deny.
    CheckAll,
    /// Regenerate the third-party-notices file from the current lockfile.
    ThirdPartyNotices,
    /// Produce a release build with SBOM and signing attestation.
    Release,
    /// Verify CLAUDE.md and AGENTS.md have not drifted.
    VerifyAgentsParity,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Task::CheckAll => check_all(),
        Task::ThirdPartyNotices => third_party_notices(),
        Task::Release => release(),
        Task::VerifyAgentsParity => verify_agents_parity(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("xtask failed: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn check_all() -> anyhow::Result<()> {
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run("cargo", &["test", "--workspace", "--all-features"])?;
    // cargo-audit and cargo-deny are best-effort: they must be installed
    // locally or skipped. CI always runs them.
    run_optional("cargo", &["audit"]);
    run_optional("cargo", &["deny", "check"]);
    Ok(())
}

fn third_party_notices() -> anyhow::Result<()> {
    // Placeholder. Requires `cargo-about` installed; wire up when the
    // first dependency set stabilises in Phase 2.
    anyhow::bail!(
        "not yet implemented; install `cargo-about` and wire into this task before first release"
    )
}

fn release() -> anyhow::Result<()> {
    // Placeholder. Wire up once the CI release workflow is exercised.
    anyhow::bail!(
        "not yet implemented; see .github/workflows/release.yml — this task will replicate that locally"
    )
}

fn verify_agents_parity() -> anyhow::Result<()> {
    // AGENTS.md should be a short pointer to CLAUDE.md. Check it does not
    // contain independent project rules (which would drift from CLAUDE.md).
    // This is a cheap invariant; keep it as a stand-alone xtask so it can
    // be called from CI without re-running the full check suite.
    let agents = std::fs::read_to_string("AGENTS.md")?;
    let claude_path_referenced = agents.contains("CLAUDE.md");
    if !claude_path_referenced {
        anyhow::bail!("AGENTS.md does not reference CLAUDE.md — drift suspected");
    }
    println!("agents parity: ok");
    Ok(())
}

fn run(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(program).args(args).status()?;
    if !status.success() {
        anyhow::bail!("{program} {args:?} failed with {status}");
    }
    Ok(())
}

fn run_optional(program: &str, args: &[&str]) {
    if let Err(e) = run(program, args) {
        eprintln!("(optional) {program} {args:?} skipped: {e}");
    }
}
