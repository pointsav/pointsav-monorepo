// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-cli
//!
//! The single binary produced by the service-slm workspace. This is the
//! entry point invoked by systemd on a Totebox host, by the Cloud Run GPU
//! node's init script, and by operators running one-off commands.
//!
//! See [`../../README.md`] for the user-facing description and
//! [`../../CLAUDE.md`] for the invariants.

use clap::{Parser, Subcommand};

/// service-slm operator CLI.
///
/// This binary wraps every subsystem in the service-slm workspace. Phase 2
/// scaffolding stage: subcommands are defined but not yet implemented.
#[derive(Parser, Debug)]
#[command(
    name = "slm-cli",
    version,
    about = "PointSav doorman and yo-yo compute substrate — operator CLI",
    long_about = None,
)]
struct Cli {
    /// Verbose output. Repeat for more detail: `-v` = debug, `-vv` = trace.
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Emit logs as JSON (one event per line) instead of human-readable.
    #[arg(long, global = true)]
    json_logs: bool,

    /// Path to the service configuration file (TOML).
    #[arg(short, long, env = "SLM_CONFIG", global = true)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Command,
}

/// Top-level subcommands.
#[derive(Subcommand, Debug)]
enum Command {
    /// Start the service-slm server (axum HTTP API + background workers).
    Serve,

    /// Run a doorman cycle end-to-end against a given payload.
    Doorman {
        /// Path to the input document to sanitise and send.
        #[arg(short, long)]
        input: std::path::PathBuf,
    },

    /// Inspect or export the audit ledger.
    Ledger {
        /// Path to the ledger CSV file.
        #[arg(short, long, env = "SLM_LEDGER_PATH")]
        path: std::path::PathBuf,

        #[command(subcommand)]
        action: LedgerAction,
    },

    /// Manage the yo-yo compute node (spin up, tear down, status).
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },

    /// Manage `LoRA` adapters in the memory/adapters registry.
    Adapter {
        #[command(subcommand)]
        action: AdapterAction,
    },
}

/// Ledger subcommands.
#[derive(Subcommand, Debug)]
enum LedgerAction {
    /// Print recent events.
    Tail {
        /// How many events to show. Default: 50.
        #[arg(short, long, default_value_t = 50)]
        n: usize,
    },
    /// Export events in CSV for external audit tools.
    Export {
        /// Destination file.
        #[arg(short, long)]
        out: std::path::PathBuf,
    },
}

/// Node subcommands.
#[derive(Subcommand, Debug)]
enum NodeAction {
    /// Spin up the GCP yo-yo node.
    Up,
    /// Tear the node down and record the final cost.
    Down,
    /// Report current node status.
    Status,
}

/// Adapter subcommands.
#[derive(Subcommand, Debug)]
enum AdapterAction {
    /// List registered adapters.
    List,
    /// Verify the signature of an adapter artefact.
    Verify {
        /// Adapter id.
        id: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose, cli.json_logs);

    tracing::info!(
        config = ?cli.config,
        "slm-cli starting (Phase 2 scaffolding; no real behaviour yet)"
    );

    // Scaffolding: every subcommand is a friendly "not yet implemented"
    // with a pointer to the task queue. As crates ship real behaviour,
    // each arm gets a real body.
    match cli.command {
        Command::Serve => unimplemented(
            "serve",
            "slm-api + slm-inference-remote + slm-ledger integration",
        ),
        Command::Doorman { input } => unimplemented(
            "doorman",
            &format!("slm-doorman end-to-end cycle for {}", input.display()),
        ),
        Command::Ledger { path, action } => match action {
            LedgerAction::Tail { n } => {
                let events = slm_ledger::tail_events(&path, n)?;
                if events.is_empty() {
                    println!("(no events)");
                } else {
                    for e in &events {
                        println!(
                            "{} | {:<20} | {:<24} | node={} job={} status={}",
                            e.timestamp_utc.format("%Y-%m-%dT%H:%M:%SZ"),
                            e.event_type.to_string(),
                            e.module_id.to_string(),
                            e.node_id.as_deref().unwrap_or("-"),
                            e.job_id.as_deref().unwrap_or("-"),
                            e.completion_status.as_deref().unwrap_or("-"),
                        );
                    }
                }
                Ok(())
            }
            LedgerAction::Export { out } => {
                unimplemented("ledger export", &format!("export to {}", out.display()))
            }
        },
        Command::Node { action } => match action {
            NodeAction::Up => unimplemented("node up", "slm-compute Cloud Run driver"),
            NodeAction::Down => unimplemented("node down", "slm-compute teardown"),
            NodeAction::Status => unimplemented("node status", "slm-compute status query"),
        },
        Command::Adapter { action } => match action {
            AdapterAction::List => {
                unimplemented("adapter list", "slm-memory-adapters registry listing")
            }
            AdapterAction::Verify { id } => unimplemented(
                "adapter verify",
                &format!("Sigstore verification of adapter {id}"),
            ),
        },
    }
}

/// Emit the not-yet-implemented placeholder message.
///
/// Remove and replace with a real implementation as each crate ships.
fn unimplemented(subcommand: &str, next_task: &str) -> anyhow::Result<()> {
    anyhow::bail!(
        "`{subcommand}` is not yet implemented.\n\
         Next work unit: {next_task}.\n\
         See TASKS.md for the ordered queue."
    )
}

/// Install the tracing subscriber.
fn init_tracing(verbose: u8, json: bool) {
    use tracing_subscriber::{fmt, EnvFilter};

    let level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("slm_cli={level},slm={level}")));

    if json {
        fmt().with_env_filter(filter).json().init();
    } else {
        fmt().with_env_filter(filter).init();
    }
}
