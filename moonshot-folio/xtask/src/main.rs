// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `moonshot-folio` assurance gates. Standard library only.
//!
//! Usage: `cargo xtask <gate|all> [--strict]`. `--strict` turns "not applicable yet" into a failure.

#![forbid(unsafe_code)]

mod fuzz;
mod rules;
mod trace;
mod util;
mod vendor;

use std::path::Path;
use std::process::ExitCode;
use util::{family_root, Res, Verdict};

const GATES: [&str; 7] = [
    "trace",
    "rules",
    "vendor-check",
    "fuzz",
    "determinism",
    "golden",
    "uax-tables",
];

/// Gates that need a renderer or generated tables that do not exist yet.
fn pending(name: &str) -> Verdict {
    Verdict::Na(format!(
        "{name}: no renderer or table generator yet (roadmap F1+)"
    ))
}

/// Run one gate by name.
fn run_gate(name: &str, root: &Path, args: &[String]) -> Res<Verdict> {
    match name {
        "trace" => trace::run(root),
        "rules" => rules::run(root),
        "vendor-check" => vendor::run(root),
        "fuzz" => fuzz::run(root, args),
        "determinism" | "golden" | "uax-tables" => Ok(pending(name)),
        other => Err(format!("unknown gate: {other}")),
    }
}

/// Print one verdict; true when it counts as a failure.
fn report(name: &str, verdict: &Verdict, strict: bool) -> bool {
    match verdict {
        Verdict::Pass(m) => println!("GATE {name}: PASS ({m})"),
        Verdict::Na(m) => println!("GATE {name}: N/A ({m})"),
        Verdict::Fail(list) => {
            println!("GATE {name}: FAIL");
            for p in list {
                println!("  {p}");
            }
        }
    }
    matches!(verdict, Verdict::Fail(_)) || (strict && matches!(verdict, Verdict::Na(_)))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let strict = args.iter().any(|a| a == "--strict");
    let Some(which) = args.iter().find(|a| !a.starts_with("--")) else {
        eprintln!("usage: cargo xtask <{}|all> [--strict]", GATES.join("|"));
        return ExitCode::from(2);
    };
    let root = match family_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let names: Vec<&str> = if which == "all" {
        GATES.to_vec()
    } else {
        vec![which.as_str()]
    };
    let mut failed = false;
    for name in names {
        match run_gate(name, &root, &args) {
            Ok(v) => failed |= report(name, &v, strict),
            Err(e) => {
                println!("GATE {name}: ERROR {e}");
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
