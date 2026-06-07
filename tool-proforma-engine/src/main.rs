use clap::{Parser, Subcommand};
use proforma_engine::{
    compute,
    excel::{pclp1, titleco, wcp},
    html,
    report::{self, d1_dev_classes, d2_direct_hold, d3_wcp},
    spv::{ambassadors_d1, ambassadors_d2, audited_json, bencal},
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
    /// D1 v2 — Calibrated dev classes (10.5% dev yield, 6.25% cap rate) with floor variants per class.
    /// Does not read any Excel; uses the locked configuration in src/report/d1_dev_classes_v2.rs.
    DevClassesV2 {
        // No xlsx — config is hardcoded per plan 2026-06-03
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
    /// PCLP 1 V1 — Self-generating proforma from BRIEF v0.15.6 §5b inputs (no Excel).
    /// Emits three files: full proforma HTML, summary HTML, JSON dump.
    Pclp1V1 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// PCLP 1 V2 — Self-generating proforma with 2026-06-04 operator corrections
    /// (advisory fee on gross equity; Y7 capex bug fix; IC = EBITDA/NetInterest +
    /// Key Ratios table; facility fee at commitment). No Excel.
    Pclp1V2 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// WCP V1 — Self-generating proforma from BRIEF v0.15.6 §5c (no Excel).
    /// Consumes PCLP 1 V2 forecast as LP1 source. Emits proforma HTML, summary HTML, JSON.
    WcpV1 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Bencal SPV1/SPV2/Management V1 — Self-generating Bencal proformas.
    /// Consumes PCLP 1 V2 + WCP V1 forecasts. Emits 9 files (3 entities × proforma/summary/JSON).
    BencalAllV1 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Building Portfolio V1 — Reformatted v3 D1 dev-classes (70 buildings; 4 classes).
    /// Self-generating from locked variant config. Emits proforma + summary + JSON.
    BuildingPortfolioV1 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V1 — Self-generating apples-to-apples comparator to PCLP 1
    /// per BRIEF v0.15.6 §5h. Traditional bank-debt JV ($250M equity + $750M debt =
    /// $1B single-shot). Emits proforma + summary + JSON.
    LegacyJvV1 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V2 — Corrected engine: Flag D7-4 NOI fix ($78.75M net), 2/20 structure,
    /// issuance costs ($10M), traditional Inc. terminology, ASPE 3061 capitalized construction,
    /// XIRR comparator, dual-column ASPE/IFRS AV. Emits proforma + summary + JSON.
    LegacyJvV2 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V3 — V2 + Y1-Y3 capitalized cost transparency (interest + mgmt fee shown
    /// gross with ASPE 3061 offset row), CSS timeline bleed fix, footnote pagination fix,
    /// debt ratio aligned with Excel convention, "Woodfine Direct-Hold Solutions" rename.
    /// Emits proforma HTML + JSON only (no summary HTML).
    LegacyJvV3 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V4 — V3 + comparator table: debt-to-contributions + equity-cost-per-SF
    /// rows; "No Excel" removed; "PCLP 1" → "Professional Centres Canada LP" throughout;
    /// wide-table overflow fix on comparator and return-summary tables.
    /// Emits proforma HTML + JSON only (no summary HTML).
    LegacyJvV4 {
        /// Output directory (default: current directory)
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V5 — V4 + global white-space:nowrap removed from CSS so ALL tables
    /// clip/wrap instead of bleeding; no structural content change from V4.
    /// Emits proforma HTML + JSON only (no summary HTML).
    LegacyJvV5 {
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// Legacy JV (D7) V6 — V5 + sf comparison changed from JV-denominator (70%)
    /// to LP-denominator (41%), matching the Excel marketing materials.
    LegacyJvV6 {
        #[arg(long, default_value = ".")]
        out_dir: PathBuf,
    },
    /// PCLP1 Sensitivity Analysis V7 — engine-sourced base case; no hardcoded financials;
    /// "Professional Centres Canada LP" throughout; LP VS JV section removed; print CSS;
    /// static audit section (IFRS 13 §93(h)(ii)); companion to tearsheet v6.
    /// Emits: pclp1-sensitivity-v7.html + COMPLIANCE_MCorp_2026_06_07_Sensitivity_PCCL_V7.json
    PclpSensitivityV7 {
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
                if cli.html {
                    html::render(&md, &data.title)
                } else {
                    md
                }
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
                if cli.html {
                    html::render(&md, &data.title)
                } else {
                    md
                }
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
                if cli.html {
                    html::render(&md, &title)
                } else {
                    md
                }
            };
            write_output(&out, cli.out.as_ref());
        }
        Some(Command::DevClassesV2 {}) => {
            // Self-contained — no Excel input. Configuration is the const data in
            // src/report/d1_dev_classes_v2.rs per plan 2026-06-03.
            let html_out = report::d1_dev_classes_v2::render_html();
            write_output(&html_out, cli.out.as_ref());
        }
        Some(Command::SpvBencal {
            pclp,
            wcp: wcp_path,
            out_dir,
        }) => {
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
            let ad2_json = audited_json(&ad2, ambassadors_d2::derivation_json(&pclp_data));
            write_trio("ambassadors-d2", &ad2_md, &ad2_title, &ad2_json);

            // Ambassadors Direct 1 Inc. — d3-wcp format
            let ad1 = ambassadors_d1::derive(&wcp_data);
            let ad1_md = d3_wcp::render(&ad1);
            let ad1_json = audited_json(&ad1, ambassadors_d1::derivation_json(&wcp_data));
            write_trio("ambassadors-d1", &ad1_md, &ad1.title, &ad1_json);

            // Bencal Management Corp. — d3-wcp format (full cashflow + valuation report)
            let bc = bencal::derive(&wcp_data, &pclp_data);
            let bc_md = d3_wcp::render(&bc);
            let bc_json = audited_json(&bc, bencal::derivation_json(&wcp_data, &pclp_data));
            write_trio("bencal", &bc_md, &bc.entity, &bc_json);

            // Bencal Management Corp. — Block F (Y10 headline; side-by-side MOIC views,
            // BRIEF §5f / Flag 3 lock 2026-06-02)
            let bf = bencal::compute_block_f(&bc);
            let bf_md = report::bencal::render(&bf);
            let bf_title = format!("{} — Block F (Y10 Headline)", bc.entity);
            let bf_json = serde_json::to_string_pretty(&bf).expect("BlockF serialisation failed");
            write_trio("bencal-block-f", &bf_md, &bf_title, &bf_json);

            // V1 — engine-canonical Bencal Forecast Summaries (supersedes JW1/JW3
            // hand-typed placeholders 2026-06-03; engine reads PCLP 1 + WCP Excels).
            let mgmt_v1 = report::bencal_forecast_v1::render_management(&pclp_data, &wcp_data);
            let spv1_v1 = report::bencal_forecast_v1::render_spv1(&wcp_data);
            let spv2_v1 = report::bencal_forecast_v1::render_spv2(&pclp_data, &wcp_data);
            write_output(
                &mgmt_v1,
                Some(
                    &out_dir.join("COMPLIANCE_MCorp_2026_06_03_Forecast_Bencal_Management_V1.html"),
                ),
            );
            write_output(
                &spv1_v1,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_03_Forecast_Bencal_SPV1_V1.html")),
            );
            write_output(
                &spv2_v1,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_03_Forecast_Bencal_SPV2_V1.html")),
            );

            eprintln!("wrote 15 files to {}", out_dir.display());
        }
        Some(Command::Pclp1V1 { out_dir }) => {
            // PCLP 1 V1 — engine self-generating proforma (no Excel input).
            // Source: BRIEF v0.15.6 §5b inputs, hardcoded as Rust constants in
            // src/spv/pclp1_proforma.rs.
            let proforma_html = report::pclp1_proforma::render_proforma();
            let summary_html = report::pclp1_proforma::render_summary();
            let json_dump = report::pclp1_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Proforma_PCLP1_V1.html")),
            );
            write_output(
                &summary_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Summary_PCLP1_V1.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_PCLP1_V1.json")),
            );
            eprintln!("wrote 3 files to {}", out_dir.display());
        }
        Some(Command::Pclp1V2 { out_dir }) => {
            // PCLP 1 V2 — engine self-generating proforma with 2026-06-04 operator
            // corrections. Same module path as V1; engine is now V2 internally.
            // Outputs at V2 versioning to preserve V1 audit trail.
            let proforma_html = report::pclp1_proforma::render_proforma();
            let summary_html = report::pclp1_proforma::render_summary();
            let json_dump = report::pclp1_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Proforma_PCLP1_V2.html")),
            );
            write_output(
                &summary_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Summary_PCLP1_V2.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_PCLP1_V2.json")),
            );
            eprintln!("wrote 3 V2 files to {}", out_dir.display());
        }
        Some(Command::WcpV1 { out_dir }) => {
            // WCP V1 — engine self-generating proforma from BRIEF §5c. Consumes PCLP 1 V2.
            let proforma_html = report::wcp_proforma::render_proforma();
            let summary_html = report::wcp_proforma::render_summary();
            let json_dump = report::wcp_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Proforma_WCP_V1.html")),
            );
            write_output(
                &summary_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Summary_WCP_V1.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_WCP_V1.json")),
            );
            eprintln!("wrote 3 WCP V1 files to {}", out_dir.display());
        }
        Some(Command::BuildingPortfolioV1 { out_dir }) => {
            // Building Portfolio V1 — v3 D1 dev-classes reformatted with proforma +
            // summary + JSON outputs. Self-generating from locked Rust constants.
            let proforma_html = report::d1_dev_classes_v2::render_proforma();
            let summary_html = report::d1_dev_classes_v2::render_summary();
            let json_dump = report::d1_dev_classes_v2::render_json();
            write_output(
                &proforma_html,
                Some(
                    &out_dir.join("COMPLIANCE_MCorp_2026_06_04_Proforma_BuildingPortfolio_V1.html"),
                ),
            );
            write_output(
                &summary_html,
                Some(
                    &out_dir.join("COMPLIANCE_MCorp_2026_06_04_Summary_BuildingPortfolio_V1.html"),
                ),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_BuildingPortfolio_V1.json")),
            );
            eprintln!(
                "wrote 3 Building Portfolio V1 files to {}",
                out_dir.display()
            );
        }
        Some(Command::LegacyJvV1 { out_dir }) => {
            // Legacy JV (D7) V1 — engine self-generating comparator to PCLP 1.
            // Source: BRIEF v0.15.6 §5h. Apples-to-apples 10-year return analysis.
            // NOTE: V1 renderer removed (struct fields updated for V2). Use legacy-jv-v2.
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let summary_html = report::legacy_jv_proforma::render_summary();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Proforma_LegacyJV_V1.html")),
            );
            write_output(
                &summary_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_Summary_LegacyJV_V1.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_04_LegacyJV_V1.json")),
            );
            eprintln!("wrote 3 Legacy JV V1 files to {}", out_dir.display());
        }
        Some(Command::LegacyJvV2 { out_dir }) => {
            // Legacy JV (D7) V2 — corrected engine.
            // Flag D7-4: $78.75M is NET NOI; 2/20 structure; issuance costs; Inc. terminology;
            // ASPE 3061 capitalized construction; XIRR comparator; dual-column ASPE/IFRS AV.
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let summary_html = report::legacy_jv_proforma::render_summary();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V2.html")),
            );
            write_output(
                &summary_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Summary_LegacyJV_V2.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_LegacyJV_V2.json")),
            );
            eprintln!("wrote 3 Legacy JV V2 files to {}", out_dir.display());
        }
        Some(Command::LegacyJvV3 { out_dir }) => {
            // Legacy JV (D7) V3 — V2 + Y1-Y3 capitalized cost transparency; CSS bleed fix;
            // footnote pagination; debt ratio note aligned with Excel; renamed subtitle.
            // No summary HTML for this version (proforma + JSON only).
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V3.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_LegacyJV_V3.json")),
            );
            eprintln!("wrote 2 Legacy JV V3 files to {}", out_dir.display());
        }
        Some(Command::LegacyJvV4 { out_dir }) => {
            // Legacy JV (D7) V4 — comparator table additions + terminology cleanup.
            // No summary HTML for this version (proforma + JSON only).
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V4.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_LegacyJV_V4.json")),
            );
            eprintln!("wrote 2 Legacy JV V4 files to {}", out_dir.display());
        }
        Some(Command::LegacyJvV5 { out_dir }) => {
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V5.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_LegacyJV_V5.json")),
            );
            eprintln!("wrote 2 Legacy JV V5 files to {}", out_dir.display());
        }
        Some(Command::LegacyJvV6 { out_dir }) => {
            let proforma_html = report::legacy_jv_proforma::render_proforma();
            let json_dump = report::legacy_jv_proforma::render_json();
            write_output(
                &proforma_html,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V6.html")),
            );
            write_output(
                &json_dump,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_06_LegacyJV_V6.json")),
            );
            eprintln!("wrote 2 Legacy JV V6 files to {}", out_dir.display());
        }
        Some(Command::BencalAllV1 { out_dir }) => {
            // Bencal SPV1, SPV2, Management V2 — engine self-generating proformas.
            // Consumes PCLP 1 V2 + WCP V1. 9 HTML/JSON files + 6 PDFs = 15 total.
            use report::bencal_v1_proforma::*;
            let pairs = [
                (
                    "Bencal_SPV1",
                    render_proforma_spv1(),
                    render_summary_spv1(),
                    render_json_spv1(),
                ),
                (
                    "Bencal_SPV2",
                    render_proforma_spv2(),
                    render_summary_spv2(),
                    render_json_spv2(),
                ),
                (
                    "Bencal_Management",
                    render_proforma_mgmt(),
                    render_summary_mgmt(),
                    render_json_mgmt(),
                ),
            ];
            let mut pdf_count = 0usize;
            for (name, proforma, summary, json) in &pairs {
                let proforma_path = out_dir.join(format!(
                    "COMPLIANCE_MCorp_2026_06_05_Proforma_{}_V2.html",
                    name
                ));
                let summary_path = out_dir.join(format!(
                    "COMPLIANCE_MCorp_2026_06_05_Summary_{}_V2.html",
                    name
                ));
                let json_path =
                    out_dir.join(format!("COMPLIANCE_MCorp_2026_06_05_{}_V2.json", name));
                write_output(proforma, Some(&proforma_path));
                write_output(summary, Some(&summary_path));
                write_output(json, Some(&json_path));
                for html_path in &[&proforma_path, &summary_path] {
                    let pdf_path = html_path.with_extension("pdf");
                    let status = std::process::Command::new("weasyprint")
                        .arg(html_path.as_os_str())
                        .arg(pdf_path.as_os_str())
                        .status();
                    match status {
                        Ok(s) if s.success() => {
                            pdf_count += 1;
                        }
                        Ok(s) => eprintln!("weasyprint exited {} for {}", s, html_path.display()),
                        Err(e) => eprintln!("weasyprint not found or failed: {e}"),
                    }
                }
            }
            eprintln!(
                "wrote 9 Bencal V2 files + {} PDFs to {}",
                pdf_count,
                out_dir.display()
            );
        }
        Some(Command::PclpSensitivityV7 { out_dir }) => {
            let (html, json) = report::pclp1_sensitivity_v7::render();
            write_output(
                &html,
                Some(&out_dir.join("pclp1-sensitivity-v7.html")),
            );
            write_output(
                &json,
                Some(&out_dir.join("COMPLIANCE_MCorp_2026_06_07_Sensitivity_PCCL_V7.json")),
            );
            eprintln!("wrote 2 PCLP1 Sensitivity V7 files to {}", out_dir.display());
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
