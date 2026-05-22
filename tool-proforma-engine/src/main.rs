use clap::Parser;
use proforma_engine::{compute, Assumptions};
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tool-proforma-engine")]
#[command(about = "PCLP 1 proforma engine — compute 10-year LP projections from JSON assumptions")]
struct Cli {
    /// Path to JSON assumptions file. Reads from stdin if omitted.
    #[arg(short, long)]
    assumptions: Option<PathBuf>,

    /// Write JSON output to this file instead of stdout.
    #[arg(short, long)]
    out: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let json_input = match cli.assumptions {
        Some(path) => std::fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("error: cannot read {}: {e}", path.display());
            std::process::exit(1);
        }),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
                eprintln!("error: reading stdin: {e}");
                std::process::exit(1);
            });
            buf
        }
    };

    let assumptions: Assumptions = serde_json::from_str(&json_input).unwrap_or_else(|e| {
        eprintln!("error: invalid assumptions JSON: {e}");
        std::process::exit(1);
    });

    let output = compute(&assumptions);

    let json_output =
        serde_json::to_string_pretty(&output).expect("serialisation of ProformaOutput failed");

    match cli.out {
        Some(path) => std::fs::write(&path, &json_output).unwrap_or_else(|e| {
            eprintln!("error: cannot write {}: {e}", path.display());
            std::process::exit(1);
        }),
        None => println!("{json_output}"),
    }
}
