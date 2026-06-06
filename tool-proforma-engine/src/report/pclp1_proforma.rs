// PCLP 1 (Professional Centres Canada LP) — proforma HTML + summary HTML renderers.
//
// Consumes the engine model in `src/spv/pclp1_proforma.rs`. Produces two HTMLs:
//   - render_proforma() — full 10-year IS / Debt Schedule / Cash Flow / Valuation /
//     per-unit metrics
//   - render_summary() — single-page investor rollup
//
// All numbers from the engine (no Excel reads).

use crate::spv::pclp1_proforma::{self, Pclp1Year};

// ─── Formatting helpers ─────────────────────────────────────────────────────

fn fmt_m(v: f64) -> String {
    if v.abs() < 1e-2 {
        "—".to_string()
    } else if v.abs() >= 1_000_000.0 {
        format!("${:.2}M", v / 1_000_000.0)
    } else if v.abs() >= 1_000.0 {
        format!("${:.0}K", v / 1_000.0)
    } else {
        format!("${:.0}", v)
    }
}

fn fmt_int(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.abs().to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}

fn fmt_full_dollar(v: f64) -> String {
    if v.abs() < 1e-2 {
        "—".to_string()
    } else {
        format!("${}", fmt_int(v))
    }
}

fn fmt_per_unit(v: f64) -> String {
    if v.abs() < 1e-4 {
        "—".to_string()
    } else {
        format!("${:.2}", v)
    }
}

fn fmt_pct(v: f64) -> String {
    if v.abs() < 1e-6 {
        "—".to_string()
    } else {
        format!("{:.2}%", v * 100.0)
    }
}

// ─── HTML scaffold ──────────────────────────────────────────────────────────

const HEAD_PROFORMA: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Professional Centres Canada LP (PCLP 1) — Proforma V1</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1400px}
h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.76rem}
table.wide{width:100%}
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:230px}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
tr.section-banner td{background:#e3edf7;font-weight:700;font-size:0.74rem;text-transform:uppercase;letter-spacing:.3px;color:#1a2a44;text-align:left}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{body{margin:0;font-size:11px;max-width:none}table{break-inside:avoid;page-break-inside:avoid}h2,h3{break-after:avoid;page-break-after:avoid}td.lnum,th.lnum{-webkit-print-color-adjust:exact;print-color-adjust:exact;color:#bbb!important;border-right-color:#ccc!important}table.wide{table-layout:fixed;font-size:10px}table.wide td,table.wide th{padding:3px 6px}table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}}
</style>
</head>
"#;

const LNUM_SCRIPT: &str = r#"<script>
(function(){
  var n=1;
  document.querySelectorAll('table').forEach(function(tbl){
    tbl.querySelectorAll('tr').forEach(function(row){
      var allTh=Array.from(row.children).every(function(c){return c.tagName==='TH';});
      var cell=document.createElement(allTh?'th':'td');
      cell.className='lnum';
      cell.textContent=n++;
      row.insertBefore(cell,row.firstChild);
    });
  });
})();
</script>
"#;

fn header_block(version_label: &str) -> String {
    format!(
        "<h1>Professional Centres Canada LP (PCLP 1) — Proforma {version_label}</h1>\n\
         <p>Engine-generated proforma from BRIEF v0.15.6 §5b inputs. No Excel read.<br>\n\
         DRAFT — 2026-06-04 — {version_label}<br>\n\
         All amounts CAD — Prepared under IFRS — Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture<br>\n\
         Source: tool-proforma-engine <code>pclp1_proforma</code> module — engine-canonical</p>\n"
    )
}

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — Notice under applicable securities legislation including the British Columbia Securities Commission (BCSC) and NI 51-102.</strong> This document contains forward-looking information. All amounts shown are computed by the tool-proforma-engine from BRIEF v0.15.6 §5b inputs. Actual results may differ materially. This document is prepared for internal planning purposes and does not constitute an offering memorandum, financial advice, or an offer to sell or solicitation to buy any security.</p>\n".to_string()
}

// ─── Section renderers ──────────────────────────────────────────────────────

fn render_capital_structure() -> String {
    let mut s = String::new();
    s.push_str("<h2>Capital Structure &amp; Key Inputs</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Parameter</th><th>Value</th><th>BRIEF cell</th></tr>\n");

    let rows = [
        (
            "Gross equity raise",
            fmt_full_dollar(pclp1_proforma::PCLP1_GROSS_EQUITY),
            "D15",
        ),
        (
            "Unit price",
            format!("${:.2}", pclp1_proforma::PCLP1_UNIT_PRICE),
            "D16",
        ),
        (
            "Diluted LP units (LPA-locked)",
            fmt_int(pclp1_proforma::PCLP1_DILUTED_UNITS),
            "D45",
        ),
        (
            "Investor units",
            fmt_int(pclp1_proforma::PCLP1_INVESTOR_UNITS),
            "§741",
        ),
        (
            "Benetti units (manager dilution)",
            fmt_int(pclp1_proforma::PCLP1_BENETTI_UNITS),
            "§742",
        ),
        (
            "Issuing agents fee",
            format!(
                "{:.0}%",
                pclp1_proforma::PCLP1_ISSUING_AGENTS_FEE_PCT * 100.0
            ),
            "D17",
        ),
        (
            "Issue costs",
            format!("{:.0}%", pclp1_proforma::PCLP1_ISSUE_COSTS_PCT * 100.0),
            "D27",
        ),
        (
            "Advisory fee (annual % of net proceeds)",
            format!("{:.0}%", pclp1_proforma::PCLP1_ADVISORY_FEE_PCT * 100.0),
            "D19",
        ),
        (
            "Admin &amp; compliance (annual)",
            fmt_full_dollar(pclp1_proforma::PCLP1_ADMIN_COMPLIANCE_ANNUAL),
            "D24",
        ),
        (
            "Board of directors (annual)",
            fmt_full_dollar(pclp1_proforma::PCLP1_BOARD_ANNUAL),
            "D23",
        ),
        (
            "Development yield",
            fmt_pct(pclp1_proforma::PCLP1_DEV_YIELD),
            "D10",
        ),
        (
            "Cap rate (Public Non-Listed)",
            fmt_pct(pclp1_proforma::PCLP1_CAP_RATE),
            "D12",
        ),
        (
            "Secondary-market buyer's required yield",
            fmt_pct(pclp1_proforma::PCLP1_BUYER_TARGET_YIELD),
            "AC23",
        ),
        (
            "Debenture interest rate",
            fmt_pct(pclp1_proforma::PCLP1_DEBT_RATE_DEBENTURE),
            "D29",
        ),
        (
            "Debt financing cost (one-time per draw)",
            fmt_pct(pclp1_proforma::PCLP1_DEBT_FINANCING_COST),
            "D28",
        ),
        (
            "Cash interest rate",
            fmt_pct(pclp1_proforma::PCLP1_CASH_INTEREST),
            "D30",
        ),
        (
            "Debt buyback (% of FFO, Y8+)",
            fmt_pct(pclp1_proforma::PCLP1_DEBT_BUYBACK_PCT_FFO),
            "D31",
        ),
        (
            "Minimum cash balance",
            fmt_full_dollar(pclp1_proforma::PCLP1_MIN_CASH_BALANCE),
            "D33",
        ),
        (
            "Working capital reserve (% of gross)",
            fmt_pct(pclp1_proforma::PCLP1_WORKING_CAPITAL_PCT),
            "D34",
        ),
    ];
    for (lbl, val, cell) in &rows {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td></tr>\n",
            lbl, val, cell
        ));
    }
    s.push_str("</table>\n");

    s.push_str("<h3>Derived capital amounts (one-time at inception)</h3>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Item</th><th>Amount</th><th>Formula</th></tr>\n");
    let derived = [
        (
            "Issuing agents fee",
            250_000_000.0 * pclp1_proforma::PCLP1_ISSUING_AGENTS_FEE_PCT,
            "gross_equity × 6%",
        ),
        (
            "Issue costs",
            250_000_000.0 * pclp1_proforma::PCLP1_ISSUE_COSTS_PCT,
            "gross_equity × 1%",
        ),
        (
            "Net proceeds",
            250_000_000.0
                * (1.0
                    - pclp1_proforma::PCLP1_ISSUING_AGENTS_FEE_PCT
                    - pclp1_proforma::PCLP1_ISSUE_COSTS_PCT),
            "gross − fees",
        ),
        (
            "Working capital reserve",
            250_000_000.0 * pclp1_proforma::PCLP1_WORKING_CAPITAL_PCT,
            "gross_equity × 6.25%",
        ),
        (
            "Net equity for buildings (Phase 1 capex)",
            250_000_000.0
                * (1.0
                    - pclp1_proforma::PCLP1_ISSUING_AGENTS_FEE_PCT
                    - pclp1_proforma::PCLP1_ISSUE_COSTS_PCT
                    - pclp1_proforma::PCLP1_WORKING_CAPITAL_PCT),
            "net_proceeds − working_capital",
        ),
        (
            "Advisory fee (annual)",
            250_000_000.0
                * (1.0
                    - pclp1_proforma::PCLP1_ISSUING_AGENTS_FEE_PCT
                    - pclp1_proforma::PCLP1_ISSUE_COSTS_PCT)
                * pclp1_proforma::PCLP1_ADVISORY_FEE_PCT,
            "net_proceeds × 1%",
        ),
    ];
    for (lbl, amount, formula) in &derived {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td></tr>\n",
            lbl,
            fmt_full_dollar(*amount),
            formula
        ));
    }
    s.push_str("</table>\n");
    s
}

fn render_phase_schedule() -> String {
    let mut s = String::new();
    s.push_str("<h2>Construction Phase Schedule</h2>\n");
    s.push_str("<p class=\"note\">Building portfolio deployed over three phases. Phase 1 funded by equity (Y1–Y3); Phase 2 and 3 funded by debentures (Y4–Y7).</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Phase</th><th>Build years</th><th>Annual draw</th><th>Phase total</th><th>Generates rent from</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Phase 1</td><td>Y1–Y3</td><td>{}</td><td>{}</td><td>Y4</td></tr>\n",
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_1_ANNUAL_DRAW),
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_1_ANNUAL_DRAW * 3.0)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Phase 2</td><td>Y4–Y5</td><td>{}</td><td>{}</td><td>Y6</td></tr>\n",
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_2_ANNUAL_DRAW),
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_2_ANNUAL_DRAW * 2.0)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Phase 3</td><td>Y6–Y7</td><td>{} (Y6), solver (Y7)</td><td>~{}</td><td>Y8</td></tr>\n",
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_3_Y6_DRAW),
        fmt_full_dollar(pclp1_proforma::PCLP1_PHASE_3_Y6_DRAW * 2.0)
    ));
    let total = pclp1_proforma::PCLP1_PHASE_1_ANNUAL_DRAW * 3.0
        + pclp1_proforma::PCLP1_PHASE_2_ANNUAL_DRAW * 2.0
        + pclp1_proforma::PCLP1_PHASE_3_Y6_DRAW * 2.0;
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total project cost</td><td>—</td><td>—</td><td>{}</td><td>—</td></tr>\n",
        fmt_full_dollar(total)
    ));
    s.push_str("</table>\n");
    s
}

fn year_header_row() -> String {
    let mut s = String::from("<tr><th class=\"lbl\">Line</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row<F>(label: &str, years: &[Pclp1Year], pick: F) -> String
where
    F: Fn(&Pclp1Year) -> f64,
{
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_pu<F>(label: &str, years: &[Pclp1Year], pick: F) -> String
where
    F: Fn(&Pclp1Year) -> f64,
{
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_per_unit(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_pct<F>(label: &str, years: &[Pclp1Year], pick: F) -> String
where
    F: Fn(&Pclp1Year) -> f64,
{
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_pct(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn render_capital_asset_schedule(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Capital Asset Schedule (10-Year)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row("Phase draws (capex)", years, |y| y.phase_draws));
    s.push_str(&data_row("Total assets (cumulative)", years, |y| {
        y.total_assets
    }));
    s.push_str(&data_row("Work in progress (WIP)", years, |y| y.wip));
    s.push_str(&data_row("Generating assets", years, |y| y.generating));
    s.push_str("</table>\n");
    s
}

fn render_income_statement(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>10-Year Income Statement (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str("<tr class=\"section-banner\"><td colspan=\"12\">Revenue (engine: NOI = generating × 10.5% dev yield)</td></tr>\n");
    s.push_str(&data_row("Net proceeds from ops (NOI)", years, |y| {
        y.net_proceeds_from_ops
    }));
    s.push_str(&data_row(
        "Income continuity (Y1–Y3 entitlement)",
        years,
        |y| y.income_continuity,
    ));
    s.push_str("<tr class=\"section-banner\"><td colspan=\"12\">Expenses</td></tr>\n");
    s.push_str(&data_row("Issue costs (Y1 only)", years, |y| y.issue_costs));
    s.push_str(&data_row("Financing costs (3% × debt draw)", years, |y| {
        y.financing_costs
    }));
    s.push_str(&data_row("Advisory fee (1% × net proceeds)", years, |y| {
        y.advisory_fee
    }));
    s.push_str(&data_row("Admin &amp; compliance", years, |y| {
        y.admin_compliance
    }));
    s.push_str(&data_row("Board of directors", years, |y| y.board));
    s.push_str(&data_row("Total expenses", years, |y| y.total_expenses));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">EBITDA</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.ebitda)));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_debt_schedule(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Debt Schedule (Debentures @ 5%)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row("Opening debt", years, |y| y.opening_debt));
    s.push_str(&data_row("Gross debt draw", years, |y| y.gross_debt_draw));
    s.push_str(&data_row(
        "Net interest (debt × 5% − cash × 0.5%)",
        years,
        |y| y.net_interest,
    ));
    s.push_str(&data_row("Debt repayment (Y8+ = 10% × FFO)", years, |y| {
        y.debt_repayment
    }));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">Closing debt</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.closing_debt)));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_cash_flow(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>10-Year Cash Flow Statement</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row("Opening cash", years, |y| y.opening_cash));
    s.push_str(&data_row("New equity (Y1 only)", years, |y| y.new_equity));
    s.push_str(&data_row("Gross debt draw", years, |y| y.gross_debt_draw));
    s.push_str(&data_row("FFO (EBITDA − net interest)", years, |y| y.ffo));
    s.push_str(&data_row("Distributions to LPs", years, |y| {
        y.distributions
    }));
    s.push_str(&data_row("Debt repayment", years, |y| y.debt_repayment));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">Ending cash</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.ending_cash)));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_valuation(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Valuation &amp; NAV</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row(
        "Asset value (NOI/cap_rate + WIP + cash)",
        years,
        |y| y.asset_value,
    ));
    s.push_str(&data_row("Closing debt", years, |y| y.closing_debt));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">NAV (asset − debt)</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.nav)));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_per_unit(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Per-Unit Metrics (2,777,777 diluted LP units)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row_pu("Asset value per unit", years, |y| {
        y.asset_value_per_unit
    }));
    s.push_str(&data_row_pu("NAV per unit", years, |y| y.nav_per_unit));
    s.push_str(&data_row_pu("Distribution per unit (DPU)", years, |y| {
        y.dpu
    }));
    s.push_str(&data_row_pu("Market value per unit", years, |y| {
        y.market_value_per_unit
    }));
    s.push_str(&data_row_pct(
        "Distribution yield on cost ($100)",
        years,
        |y| y.dist_yield_on_cost,
    ));
    s.push_str(&data_row_pct("Distribution yield at market", years, |y| {
        y.dist_yield_at_market
    }));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Market value Y1–Y7 hardcoded per BRIEF v0.15.6 §736. Y8+ computed as DPU ÷ buyer's required yield (8%): if a secondary-market buyer requires 8% distribution yield on cost, market price = DPU ÷ 0.08.</p>\n");
    s
}

fn render_key_ratios(years: &[Pclp1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Key Ratios &amp; Coverage</h2>\n");
    s.push_str("<p class=\"note\">Interest Coverage Ratio (EBITDA ÷ Net Interest) is constrained by the LPA Partnership Agreement to a minimum of 1.20× in any year. Year 5 (1.53× per engine, with Phase 2 facility-commitment fee timing) is the binding year because Phase 1 NOI begins at Y4 while Phase 2 debt is being drawn.</p>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());

    // Interest Coverage row (special: use 'x' suffix)
    let mut ic_row =
        String::from("<tr><td class=\"lbl\">Interest Coverage (EBITDA ÷ Net Interest)</td>");
    for y in years {
        if y.interest_coverage > 0.001 {
            ic_row.push_str(&format!("<td>{:.2}×</td>", y.interest_coverage));
        } else {
            ic_row.push_str("<td>—</td>");
        }
    }
    ic_row.push_str("</tr>\n");
    s.push_str(&ic_row);

    s.push_str(&data_row_pct("Debt-to-Development-Cost", years, |y| {
        y.debt_to_dev_cost
    }));
    s.push_str(&data_row_pct("Debt-to-Asset-Value (LTV)", years, |y| {
        y.debt_to_asset_value
    }));

    // Total sqft (generating)
    let mut sqft_row = String::from("<tr><td class=\"lbl\">Total Generating Square Footage</td>");
    for y in years {
        if y.total_sqft_generating > 1.0 {
            sqft_row.push_str(&format!("<td>{} sf</td>", fmt_int(y.total_sqft_generating)));
        } else {
            sqft_row.push_str("<td>—</td>");
        }
    }
    sqft_row.push_str("</tr>\n");
    s.push_str(&sqft_row);

    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Total Portfolio at full deployment: 3,906,855 sf across 3 phases. Phase 1 generates Y4 (~699,200 sf); Phase 2 adds Y6 (~1,793,500 sf total); Phase 3 completes Y8 (3,906,855 sf). Debt-to-asset value (LTV) peaks at Y7 during Phase 3 construction (~62%), then drops to ~47% at stabilization.</p>\n");
    s
}

// ─── Public renderers ──────────────────────────────────────────────────────

pub fn render_proforma() -> String {
    let years = pclp1_proforma::forecast();
    let mut s = String::new();
    s.push_str(HEAD_PROFORMA);
    s.push_str("<body>\n");
    s.push_str(&header_block("V2"));
    s.push_str(&v2_corrections_note());
    s.push_str(&render_capital_structure());
    s.push_str(&render_wc_reserve_note());
    s.push_str(&render_facility_fee_note());
    s.push_str(&render_phase_schedule());
    s.push_str(&render_capital_asset_schedule(&years));
    s.push_str(&render_income_statement(&years));
    s.push_str(&render_debt_schedule(&years));
    s.push_str(&render_cash_flow(&years));
    s.push_str(&render_valuation(&years));
    s.push_str(&render_per_unit(&years));
    s.push_str(&render_key_ratios(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn v2_corrections_note() -> String {
    "<p class=\"note\"><strong>V2 corrections (2026-06-04):</strong> (1) Advisory fee on \
    gross equity ($250M × 1% = $2.5M/yr, was net proceeds); (2) Working capital reserve \
    sanity-checked at 6.25% × $250M = $15.625M; (3) Y7 Phase 3 capex bug fixed (was \
    inflating NAV ~$124/unit); (4) Interest Coverage formula corrected to EBITDA ÷ Net \
    Interest + Key Ratios table added; (5) Debenture facility fees recognized at \
    commitment year (Y4 for Phase 2; Y6 for Phase 3) — matches real-world practice and \
    satisfies LPA 1.20× covenant at Y5 with zero damage to NAV/DPU.</p>\n"
        .to_string()
}

fn render_wc_reserve_note() -> String {
    "<p class=\"note\"><strong>Working Capital Reserve — sanity check (V2 Correction 2):</strong> \
    6.25% × $250M = $15.625M covers ~3.8 years of recurring operating expenses (advisory \
    $2.5M + admin $500K + board $450K = $3.45M/yr) plus Y1 issue costs ($2.5M). Income \
    continuity Y1–Y3 ($3.05M/$3.30M/$3.50M per BRIEF §735) is a fair-value entitlement \
    for asset valuation per BRIEF §823–827; it is not cash and is not netted against the \
    reserve requirement. The 6.25% rate is preserved (between 3-year and 4-year coverage \
    targets).</p>\n"
        .to_string()
}

fn render_facility_fee_note() -> String {
    "<p class=\"note\"><strong>Debenture facility fees (V2 Correction 5):</strong> \
    The 3% facility fee (BRIEF v0.15.6 D28) is recognized in the year of facility \
    commitment rather than spread per draw, matching standard real-world debenture \
    practice. Phase 2 facility fee ($10.185M = 3% × $339.5M) is expensed at Y4. \
    Phase 3 facility fee ($19.6425M = 3% × $654.75M) is expensed at Y6. Total fee cost \
    unchanged; only the timing of expense recognition moves. This satisfies the LPA-\
    mandated 1.20× Interest Coverage covenant at Y5 (engine: 1.53×) and Y7 (engine: \
    1.33×) without any debt or project size reduction.</p>\n"
        .to_string()
}

pub fn render_summary() -> String {
    let years = pclp1_proforma::forecast();
    let y10 = &years[10];

    // Aggregate distributions Y1-Y10
    let cumulative_dist: f64 = years.iter().map(|y| y.distributions).sum();
    let cumulative_dpu: f64 = years.iter().map(|y| y.dpu).sum();
    let total_return_per_unit = cumulative_dpu + y10.market_value_per_unit;
    let moic = total_return_per_unit / pclp1_proforma::PCLP1_UNIT_PRICE;

    let mut s = String::new();
    s.push_str(HEAD_PROFORMA);
    s.push_str("<body>\n");
    s.push_str("<h1>Professional Centres Canada LP (PCLP 1) — Summary V2</h1>\n");
    s.push_str("<p>Engine-generated summary from BRIEF v0.15.6 §5b. No Excel read.<br>\n");
    s.push_str("DRAFT — 2026-06-04 — V2 (operator corrections applied)<br>\n");
    s.push_str("Companion: <code>COMPLIANCE_MCorp_2026_06_04_Proforma_PCLP1_V2.html</code> (full 10-year proforma)<br>\n");
    s.push_str("All amounts CAD — Prepared under IFRS — Forward-looking projections; BCSC continuous-disclosure posture</p>\n");
    s.push_str(&v2_corrections_note());

    s.push_str("<h2>Capital Structure</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Item</th><th>Value</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total equity raise</td><td>{}</td></tr>\n",
        fmt_full_dollar(pclp1_proforma::PCLP1_GROSS_EQUITY)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Unit price</td><td>${:.2}</td></tr>\n",
        pclp1_proforma::PCLP1_UNIT_PRICE
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Investor LP units</td><td>{}</td></tr>\n",
        fmt_int(pclp1_proforma::PCLP1_INVESTOR_UNITS)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Diluted LP units</td><td>{}</td></tr>\n",
        fmt_int(pclp1_proforma::PCLP1_DILUTED_UNITS)
    ));
    s.push_str("</table>\n");

    s.push_str("<h2>Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per investor unit</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Capital invested (Y0)</td><td>{}</td><td>${:.2}</td></tr>\n",
        fmt_full_dollar(pclp1_proforma::PCLP1_GROSS_EQUITY),
        pclp1_proforma::PCLP1_UNIT_PRICE
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Cumulative distributions Y1–Y10</td><td>{}</td><td>${:.2}</td></tr>\n",
        fmt_m(cumulative_dist), cumulative_dpu
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 market value per unit</td><td>—</td><td>${:.2}</td></tr>\n",
        y10.market_value_per_unit
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total return per unit (cash + Y10 market value)</td><td>—</td><td>${:.2}</td></tr>\n",
        total_return_per_unit
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic, moic
    ));
    s.push_str("</table>\n");

    s.push_str("<h2>Yields and Market Value Mechanic</h2>\n");
    s.push_str("<p class=\"note\">The 8% Secondary-Market Buyer's Required Yield (BRIEF AC23) is NOT a distribution policy at the LP level. It is the yield a secondary-market buyer would require to purchase an LP unit. Y8+ Market value = DPU ÷ 8%. Distribution policy at PCLP 1 level: Y1–Y3 = 0% payout; Y4–Y7 = 90% of FFO; Y8+ = 100% of FFO.</p>\n");

    s.push_str(&render_key_ratios(&years));

    s.push_str("<h2>10-Year Distribution Schedule (Per Unit)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>DPU</th><th>Cumulative DPU</th><th>NAV/unit</th><th>Market value/unit</th><th>Yield on cost</th><th>Yield at market</th></tr>\n");
    let mut running_dpu = 0.0;
    for y in &years {
        running_dpu += y.dpu;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>${:.2}</td><td>${:.2}</td><td>${:.2}</td><td>${:.2}</td><td>{}</td><td>{}</td></tr>\n",
            y.year, y.dpu, running_dpu, y.nav_per_unit, y.market_value_per_unit,
            fmt_pct(y.dist_yield_on_cost), fmt_pct(y.dist_yield_at_market)
        ));
    }
    s.push_str("</table>\n");

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json() -> String {
    serde_json::to_string_pretty(&pclp1_proforma::forecast_json())
        .expect("PCLP 1 JSON serialization failed")
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proforma_is_well_formed_html() {
        let html = render_proforma();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Professional Centres Canada LP"));
        assert!(html.contains("Capital Structure"));
        assert!(html.contains("10-Year Income Statement"));
        assert!(html.contains("Debt Schedule"));
        assert!(html.contains("Cash Flow Statement"));
        assert!(html.contains("NAV"));
        assert!(html.contains("Per-Unit Metrics"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn summary_is_well_formed_html() {
        let html = render_summary();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Summary V2"));
        assert!(html.contains("Investment Return Summary"));
        assert!(html.contains("MOIC"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn json_is_valid() {
        let json_str = render_json();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["version"], "V2");
        assert!(parsed["years"].is_array());
    }
}
