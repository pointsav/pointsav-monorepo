use clap::{Parser, Subcommand};
use proforma_engine::{
    compute,
    excel::{pclp1, titleco, wcp},
    html,
    report::{d1_dev_classes, d2_direct_hold, d3_wcp},
    spv::{ambassadors_d1, ambassadors_d2, bencal},
    Assumptions,
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

    /// Emit the raw parsed data as pretty-printed JSON instead of a report.
    #[arg(long, global = true)]
    json: bool,
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
    /// SPV — BenCal / Ambassadors Direct 1 & 2: derive three reports from WCP + PCLP 1 Excel.
    SpvBencal {
        /// Path to PCLP 1 Excel file (.xlsx)
        #[arg(long)]
        pclp: PathBuf,
        /// Path to WCP 42M Excel file (.xlsx)
        #[arg(long)]
        wcp: PathBuf,
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
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
            let out = if cli.json {
                serde_json::to_string_pretty(&data).expect("serialisation failed")
            } else {
                let md = d2_direct_hold::render(&data);
                if cli.html { html::render(&md, &data.title) } else { md }
            };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::Wcp { xlsx }) => {
            let data = wcp::read(&xlsx).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", xlsx);
                std::process::exit(1);
            });
            let out = if cli.json {
                serde_json::to_string_pretty(&data).expect("serialisation failed")
            } else {
                let md = d3_wcp::render(&data);
                if cli.html { html::render(&md, &data.title) } else { md }
            };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::DevClasses { xlsx }) => {
            let base = titleco::read(&xlsx).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", xlsx);
                std::process::exit(1);
            });
            let out = if cli.json {
                serde_json::to_string_pretty(&base).expect("serialisation failed")
            } else {
                let md = d1_dev_classes::render(&base);
                let title = format!("Development Classes — {}", base.entity);
                if cli.html { html::render(&md, &title) } else { md }
            };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::SpvBencal { pclp, wcp: wcp_path, out_dir }) => {
            let pclp_data = pclp1::read(&pclp).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", pclp);
                std::process::exit(1);
            });
            let wcp_data = wcp::read(&wcp_path).unwrap_or_else(|e| {
                eprintln!("error reading {:?}: {e}", wcp_path);
                std::process::exit(1);
            });

            let write_trio = |stem: &str, md: &str, title: &str, json_str: &str| {
                let base = out_dir.join(stem);
                write_output(md, Some(&base.with_extension("md")));
                write_output(&html::render(md, title), Some(&base.with_extension("html")));
                write_output(json_str, Some(&base.with_extension("json")));
            };

            // Ambassadors Direct 2 LP — d2-direct-hold format
            let ad2 = ambassadors_d2::derive(&pclp_data);
            let ad2_md = d2_direct_hold::render(&ad2);
            let ad2_title = format!("Direct-Hold Solution — {}", ad2.entity);
            let ad2_json = serde_json::to_string_pretty(&ad2).expect("serialisation failed");
            write_trio("ambassadors-d2", &ad2_md, &ad2_title, &ad2_json);

            // Ambassadors Direct 1 Inc. — d3-wcp format
            let ad1 = ambassadors_d1::derive(&wcp_data);
            let ad1_md = d3_wcp::render(&ad1);
            let ad1_json = serde_json::to_string_pretty(&ad1).expect("serialisation failed");
            write_trio("ambassadors-d1", &ad1_md, &ad1.title, &ad1_json);

            // BenCal Holdings Inc. — d3-wcp format
            let bc = bencal::derive(&wcp_data, &pclp_data);
            let bc_md = d3_wcp::render(&bc);
            let bc_json = serde_json::to_string_pretty(&bc).expect("serialisation failed");
            write_trio("bencal", &bc_md, &bc.entity, &bc_json);

            eprintln!("wrote 9 files to {}", out_dir.display());
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
            let json_out = serde_json::to_string_pretty(&output).expect("serialisation failed");
            write_output(&json_out, cli.out.as_ref());
        }
    }
}
