use clap::{Parser, Subcommand};
use proforma_engine::{
    compute, Assumptions,
    excel::{pclp1, titleco, wcp},
    html,
    report::{d1_dev_classes, d2_direct_hold, d3_wcp},
};
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tool-proforma-engine")]
#[command(about = "PCLP 1 proforma engine — PCLP 1 / WCP / Development Class 10-year projections")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// (Legacy) JSON assumptions file for the base sensitivity engine.
    #[arg(short, long, global = true)]
    assumptions: Option<PathBuf>,

    /// Write output to this file instead of stdout.
    #[arg(short, long, global = true)]
    out: Option<PathBuf>,

    /// Render as a self-contained HTML page instead of markdown.
    #[arg(long, global = true)]
    html: bool,
}

#[derive(Subcommand)]
enum Command {
    /// D2 — Direct-Hold Solution: 10-year IS/CF/BS + Financial Forecast from PCLP 1 Excel.
    DirectHold {
        /// Path to PCLP 1 Excel file (.xlsx)
        xlsx: PathBuf,
    },
    /// D3 — WCP Inc.: 10-year IS/BS + Revenue Generator + Valuation Matrix from WCP Excel.
    Wcp {
        /// Path to WCP 42M Excel file (.xlsx)
        xlsx: PathBuf,
    },
    /// D1 — Development Classes: parameterised 10-year proformas from TitleCo 3 Excel.
    DevClasses {
        /// Path to TitleCo 3 Excel file (.xlsx)
        xlsx: PathBuf,
    },
}

fn write_output(content: &str, out: Option<&PathBuf>) {
    match out {
        Some(path) => std::fs::write(path, content).unwrap_or_else(|e| {
            eprintln!("error: cannot write {}: {e}", path.display());
            std::process::exit(1);
        }),
        None => print!("{content}"),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::DirectHold { xlsx }) => {
            let data = pclp1::read(&xlsx).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", xlsx);
                std::process::exit(1);
            });
            let md = d2_direct_hold::render(&data);
            let out = if cli.html { html::render(&md, &data.title) } else { md };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::Wcp { xlsx }) => {
            let data = wcp::read(&xlsx).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", xlsx);
                std::process::exit(1);
            });
            let md = d3_wcp::render(&data);
            let out = if cli.html { html::render(&md, &data.title) } else { md };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::DevClasses { xlsx }) => {
            let base = titleco::read(&xlsx).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", xlsx);
                std::process::exit(1);
            });
            let md = d1_dev_classes::render(&base);
            let title = format!("Development Classes — {}", base.entity);
            let out = if cli.html { html::render(&md, &title) } else { md };
            write_output(&out, cli.out.as_ref());
        }
        None => {
            // Legacy: JSON assumptions → sensitivity engine
            let json_input = match cli.assumptions {
                Some(ref path) => std::fs::read_to_string(path).unwrap_or_else(|e| {
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
            let json_out =
                serde_json::to_string_pretty(&output).expect("serialisation failed");
            write_output(&json_out, cli.out.as_ref());
        }
    }
}
