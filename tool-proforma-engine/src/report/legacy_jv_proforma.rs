// Legacy JV (D7) — proforma + summary HTML renderers.
// V2 — corrected for Flag D7-4 NOI, 2/20 structure, issuance costs, Inc. terminology,
// ASPE 3061 capitalized construction costs, dual-column AV, XIRR comparator.

use crate::spv::irr::xirr_annual;
use crate::spv::legacy_jv_proforma::{self, LegacyJvYear};
use crate::spv::pclp1_proforma;

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
    format!("${}", fmt_int(v))
}
fn fmt_pct(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}
fn fmt_per_share(v: f64) -> String {
    format!("${:.2}", v)
}

const HEAD: &str = r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>__TITLE__</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1400px}
h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.76rem}
table.wide{width:100%;overflow-x:auto;display:block}
table.wide td,table.wide th{white-space:normal;word-break:break-word}
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:230px}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{body{margin:0;font-size:11px;max-width:none}table{break-inside:avoid;page-break-inside:avoid}h2,h3{break-after:avoid;page-break-after:avoid}p.note{break-before:avoid;page-break-before:avoid}td.lnum,th.lnum{-webkit-print-color-adjust:exact;print-color-adjust:exact;color:#bbb!important;border-right-color:#ccc!important}table.wide{table-layout:fixed;font-size:10px}table.wide td,table.wide th{padding:3px 6px}table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}}
</style></head>
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

fn head_with_title(title: &str) -> String {
    HEAD.replace("__TITLE__", title)
}

fn year_header() -> String {
    let mut s = String::from("<tr><th class=\"lbl\">Line</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row<F: Fn(&LegacyJvYear) -> f64>(label: &str, years: &[LegacyJvYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_pct<F: Fn(&LegacyJvYear) -> f64>(
    label: &str,
    years: &[LegacyJvYear],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        let v = pick(y);
        if v > 0.001 {
            s.push_str(&format!("<td>{}</td>", fmt_pct(v)));
        } else {
            s.push_str("<td>—</td>");
        }
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_x<F: Fn(&LegacyJvYear) -> f64>(label: &str, years: &[LegacyJvYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        let v = pick(y);
        if v > 0.001 {
            s.push_str(&format!("<td>{:.2}×</td>", v));
        } else {
            s.push_str("<td>—</td>");
        }
    }
    s.push_str("</tr>\n");
    s
}

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — BCSC NI 51-102.</strong> Engine-generated proforma from BRIEF v0.15.6 §5h. Apples-to-apples comparator to PCLP 1; not an investment offering. Actual results may differ materially.</p>\n".to_string()
}

// ─── IRR helpers ─────────────────────────────────────────────────────────────

fn legacy_jv_irr_cash_flows(years: &[LegacyJvYear]) -> Vec<f64> {
    // Investor perspective: Y0 = −gross equity; Y4–Y10 = net dividends; Y10 += IFRS FV terminal.
    let mut cfs: Vec<f64> = vec![0.0; 11];
    cfs[0] = -legacy_jv_proforma::LEGACY_JV_GROSS_EQUITY;
    for y in years {
        if y.year >= 1 && y.year <= 10 {
            cfs[y.year as usize] += y.dividends_to_shareholders;
        }
    }
    // Y10 terminal: investor exits at IFRS FV equity
    cfs[10] +=
        legacy_jv_proforma::LEGACY_JV_STABILIZED_AV - legacy_jv_proforma::LEGACY_JV_BANK_DEBT;
    cfs
}

fn pclp1_irr_cash_flows() -> Vec<f64> {
    // Investor perspective on PCLP 1: same $250M gross equity; receive investor fraction of
    // distributions + investor fraction of NAV at Y10.
    let pclp_years = pclp1_proforma::forecast();
    let investor_frac = pclp1_proforma::PCLP1_INVESTOR_UNITS / pclp1_proforma::PCLP1_DILUTED_UNITS;
    let mut cfs: Vec<f64> = vec![0.0; 11];
    cfs[0] = -pclp1_proforma::PCLP1_GROSS_EQUITY;
    for y in &pclp_years {
        if y.year >= 1 && y.year <= 10 {
            cfs[y.year as usize] += y.distributions * investor_frac;
        }
    }
    // Y10 terminal: investor fraction of NAV (= asset_value - closing_debt)
    if let Some(y10) = pclp_years.iter().find(|y| y.year == 10) {
        cfs[10] += y10.nav * investor_frac;
    }
    cfs
}

// ─── Public renderers ───────────────────────────────────────────────────────

pub fn render_proforma() -> String {
    use legacy_jv_proforma::*;
    let years = forecast();
    let y10 = &years[10];

    let mut s = String::new();
    s.push_str(&head_with_title("Legacy JV (D7) — Proforma V4"));
    s.push_str("<body>\n");
    s.push_str("<h1>Legacy JV (D7) — Traditional Joint Venture Proforma V4</h1>\n");
    s.push_str("<p>Engine-generated comparator proforma from BRIEF v0.15.6 §5h. Apples-to-apples to Professional Centres Canada LP (D2).<br>\n");
    s.push_str("DRAFT — 2026-06-06 — V4<br>\n");
    s.push_str("Subtitle: 'Traditional J/V Financing vs. Woodfine Direct-Hold Solutions'<br>\n");
    s.push_str("All amounts CAD — ASPE 3061 cost model (50-yr SL depreciation on fully-capitalized building component)</p>\n");

    // ── Capital Structure ──────────────────────────────────────────────────
    s.push_str("<h2>Capital Structure (BRIEF §5h)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Parameter</th><th>Value</th><th>Notes</th></tr>\n");
    let cap_rows: &[(&str, String, &str)] = &[
        (
            "Gross equity subscribed",
            fmt_full_dollar(LEGACY_JV_GROSS_EQUITY),
            "2,500,000 shares × $100",
        ),
        (
            "Less: issuance costs (1% mortgage + 1% equity)",
            format!("({})", fmt_full_dollar(LEGACY_JV_ISSUANCE_COSTS)),
            "",
        ),
        (
            "Net equity deployed",
            fmt_full_dollar(LEGACY_JV_NET_EQUITY),
            "Into project construction",
        ),
        (
            "Bank debt (construction → permanent)",
            fmt_full_dollar(LEGACY_JV_BANK_DEBT),
            "3.0× gross equity; 3.125× on net $240M deployed",
        ),
        (
            "Total construction capital",
            fmt_full_dollar(LEGACY_JV_TOTAL_CAPITAL),
            "Single-shot (net equity + bank debt)",
        ),
        (
            "Shares (traditional Inc.)",
            fmt_int(LEGACY_JV_SHARES),
            "Shareholders' agreement",
        ),
        (
            "Total portfolio sf",
            fmt_int(LEGACY_JV_TOTAL_SF),
            "BRIEF §2469",
        ),
        (
            "Bank debt / sf (construction)",
            format!("${:.2}", LEGACY_JV_COST_PER_SF),
            "= $750M / 2,298,150 sf; total capital/sf = $430.77",
        ),
        ("Development yield", fmt_pct(LEGACY_JV_DEV_YIELD), "10.5%"),
        ("Cap rate", fmt_pct(LEGACY_JV_CAP_RATE), "6.25%"),
        (
            "Stabilized NOI (net; tenant CAM pass-through)",
            fmt_full_dollar(LEGACY_JV_NOI_STABILIZED),
            "Flag D7-4: NOI is net; no opex deduction",
        ),
        (
            "Stabilized asset value (IFRS FV)",
            fmt_full_dollar(LEGACY_JV_STABILIZED_AV),
            "= NOI / cap rate = $78.75M / 6.25%",
        ),
        (
            "Permanent loan interest rate",
            fmt_pct(LEGACY_JV_INTEREST_RATE),
            "5%",
        ),
        (
            "Building component (ASPE 3061)",
            fmt_full_dollar(LEGACY_JV_BUILDING_COMPONENT),
            "$990M + $71.25M cap'd interest + $15M cap'd fees",
        ),
        (
            "Depreciation (50-yr SL)",
            format!("{:.0}-yr SL", LEGACY_JV_DEPRECIATION_YRS),
            "Building only; land excluded; $21.7M/yr",
        ),
        ("LTV covenant", fmt_pct(LEGACY_JV_LTV_COVENANT), "65% max"),
    ];
    for (l, v, n) in cap_rows {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td></tr>\n",
            l, v, n
        ));
    }
    s.push_str("</table>\n");

    // ── Fee Structure ──────────────────────────────────────────────────────
    s.push_str("<h2>Fee Structure (2/20 Management Agreement)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Fee</th><th>Amount</th><th>Notes</th></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Annual management fee (2%)</td><td>{}</td><td>2% × $250M committed equity/yr; capitalized Y1–Y3</td></tr>\n", fmt_full_dollar(LEGACY_JV_MGMT_FEE)));
    s.push_str(&format!("<tr><td class=\"lbl\">Preferred return hurdle (8%)</td><td>{}</td><td>8% × $250M gross equity</td></tr>\n", fmt_full_dollar(LEGACY_JV_HURDLE)));
    s.push_str("<tr><td class=\"lbl\">Carried interest (20%)</td><td>20% × (distributable cash − hurdle)</td><td>Annual above-hurdle carry; Y4+ only</td></tr>\n");
    s.push_str("</table>\n");

    // ── Construction Timeline ──────────────────────────────────────────────
    s.push_str("<h2>10-Year Timeline (BRIEF §2493-2496)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    let mut phase_row = String::from("<tr><td class=\"lbl\">Phase</td>");
    for y in &years {
        phase_row.push_str(&format!("<td>{}</td>", y.phase));
    }
    phase_row.push_str("</tr>\n");
    s.push_str(&phase_row);
    s.push_str(&data_row(
        "Capex (S-curve 20/50/30 on $990M)",
        &years,
        |y| y.capex,
    ));
    s.push_str(&data_row(
        "Equity contribution (24.24% of capex)",
        &years,
        |y| y.equity_contribution,
    ));
    s.push_str(&data_row("Cumulative capex", &years, |y| {
        y.cumulative_capex
    }));
    s.push_str(&data_row("Debt outstanding", &years, |y| {
        y.debt_outstanding
    }));
    s.push_str("</table>\n");

    // ── Income Statement ───────────────────────────────────────────────────
    s.push_str("<h2>10-Year Income Statement (CAD; ASPE 3061)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    s.push_str(&data_row(
        "Net Operating Income (tenant CAM pass-through)",
        &years,
        |y| y.noi,
    ));
    s.push_str(&data_row(
        "Interest on $750M @ 5% (Y1–Y3 capitalized)",
        &years,
        |y| -y.interest_expense,
    ));
    s.push_str(&data_row(
        "Depreciation (50-yr SL; ASPE 3061)",
        &years,
        |y| -y.depreciation,
    ));
    s.push_str(&data_row(
        "Management fee (2%; Y1–Y3 capitalized)",
        &years,
        |y| -y.mgmt_fee,
    ));
    s.push_str(&data_row(
        "Capitalized construction costs (ASPE 3061 offset)",
        &years,
        |y| y.capitalized_costs,
    ));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">Net income</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row("Add back depreciation (non-cash)", &years, |y| {
        y.depreciation
    }));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Distributable cash</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.distributable_cash)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row(
        "Carry to Management Co. (20% above 8% hurdle)",
        &years,
        |y| -y.carry_to_mgmt,
    ));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Net dividends to shareholders</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.dividends_to_shareholders)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row("Cumulative dividends", &years, |y| {
        y.cumulative_dividends
    }));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Y1–Y3 construction: interest and management fees accrue and are shown at gross cost, then offset by the \"Capitalized construction costs\" row (ASPE 3061) — net P&amp;L = $0. These costs consume construction capital: Y1 $12.5M, Y2 $31.25M, Y3 $42.5M capitalized into the building component ($1,086.25M total). Y4+ stabilized: NOI $78.75M (net; Flag D7-4 — tenant CAM is pass-through); debt service $37.5M; depreciation $21.7M; mgmt fee $5M; net income ~$14.5M; distributable cash ~$36.25M/yr; carry ~$3.25M/yr; net dividends to shareholders ~$33M/yr; cumulative Y4–Y10 dividends ~$231M.</p>\n");

    // ── Valuation & Ratios ─────────────────────────────────────────────────
    s.push_str("<h2>Valuation &amp; Ratios</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    s.push_str(&data_row("Asset value (ASPE book)", &years, |y| {
        y.asset_value_aspe
    }));
    s.push_str(&data_row("Shareholders' equity (ASPE book)", &years, |y| {
        y.shareholders_equity
    }));
    s.push_str(&data_row("Equity value (IFRS FV; Y4+ only)", &years, |y| {
        y.equity_value_ifrs_fv
    }));
    s.push_str(&data_row_pct("LTV (debt / ASPE book asset)", &years, |y| {
        y.ltv_book
    }));
    s.push_str(&data_row_x("DSCR (NOI / interest)", &years, |y| y.dscr));
    s.push_str(&data_row("Dividend per share (DPS)", &years, |y| y.dps));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">IFRS Fair Value at Y4+ = $78.75M NOI / 6.25% cap rate = $1,260M. Equity at FV = $1,260M − $750M = $510M (flat; income-capitalization, no depreciation). ASPE book equity declines: $218M at Y4 → $88M at Y10 (50-yr SL on $1,086.25M building component, $21.7M/yr).</p>\n");

    // ── Single-Shot Constraint ─────────────────────────────────────────────
    s.push_str("<h2>Single-Shot Constraint (BRIEF §2498-2504)</h2>\n");
    s.push_str("<p>Stabilized IFRS FV = $1,260M. At 65% LTV covenant, maximum permanent debt = $819M. Existing $750M is $69M below that ceiling — <strong>$69M of headroom</strong>, not a covenant breach. However, a Phase 2 construction facility of comparable scale would require ~$750M, roughly 11× the available headroom. Structurally single-shot: cannot compound into a Phase 2 development without a new equity raise.</p>\n");
    s.push_str("<p class=\"note\">By contrast, Professional Centres Canada LP (D2) issues debentures in 3 phases on a single fund and achieves 3.9M sf vs Legacy JV's 2.3M sf, with multi-round compounding capability built into the structure.</p>\n");

    s.push_str(&render_endpoint_summary(y10, &years));
    s.push_str(&render_comparator_table());
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn render_endpoint_summary(y10: &LegacyJvYear, years: &[LegacyJvYear]) -> String {
    use legacy_jv_proforma::*;
    let moic_at_fv = (y10.cumulative_dividends + y10.equity_value_ifrs_fv) / LEGACY_JV_GROSS_EQUITY;
    let moic_at_book =
        (y10.cumulative_dividends + y10.shareholders_equity) / LEGACY_JV_GROSS_EQUITY;

    let cfs = legacy_jv_irr_cash_flows(years);
    let irr_pct = xirr_annual(&cfs).unwrap_or(0.0);

    let mut s = String::new();
    s.push_str("<h2>Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per share</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total equity invested (Y0, gross)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_full_dollar(LEGACY_JV_GROSS_EQUITY),
        fmt_per_share(LEGACY_JV_SHARE_PRICE)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Less: issuance costs</td><td>({})</td><td>({:.2})</td></tr>\n",
        fmt_full_dollar(LEGACY_JV_ISSUANCE_COSTS),
        LEGACY_JV_ISSUANCE_COSTS / LEGACY_JV_SHARES
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Net equity deployed</td><td>{}</td><td>{}</td></tr>\n",
        fmt_full_dollar(LEGACY_JV_NET_EQUITY),
        fmt_per_share(LEGACY_JV_NET_EQUITY / LEGACY_JV_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Cumulative dividends Y4–Y10</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.cumulative_dividends),
        fmt_per_share(y10.cumulative_dividends / LEGACY_JV_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 IFRS FV asset value</td><td>{}</td><td>—</td></tr>\n",
        fmt_m(LEGACY_JV_STABILIZED_AV)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 IFRS FV equity (AV − $750M debt)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.equity_value_ifrs_fv),
        fmt_per_share(y10.equity_value_ifrs_fv / LEGACY_JV_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 ASPE book equity (book; declining)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.shareholders_equity),
        fmt_per_share(y10.shareholders_equity / LEGACY_JV_SHARES)
    ));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Total return (dividends + Y10 IFRS FV equity)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.cumulative_dividends + y10.equity_value_ifrs_fv),
                        fmt_per_share((y10.cumulative_dividends + y10.equity_value_ifrs_fv) / LEGACY_JV_SHARES)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC — IFRS FV basis (pre-tax, gross)</td><td>{:.2}×</td><td>—</td></tr>\n",
        moic_at_fv
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC — ASPE book basis</td><td>{:.2}×</td><td>—</td></tr>\n",
        moic_at_book
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">IRR (pre-tax; IFRS FV terminal)</td><td>{:.1}%</td><td>—</td></tr>\n",
        irr_pct * 100.0
    ));
    s.push_str("</table>\n");
    s.push_str(&format!("<p class=\"note\">Cash-on-cash yield ~{:.1}% on $250M gross equity (${:.1}M annual net dividends / $250M). MOIC at IFRS FV: ${:.1}M dividends + $510M FV equity = ${:.1}M total return / $250M gross equity = {:.2}×. ASPE book MOIC lower because 50-yr depreciation reduces book equity to ${:.1}M by Y10.</p>\n",
                        (y10.dividends_to_shareholders / legacy_jv_proforma::LEGACY_JV_GROSS_EQUITY) * 100.0,
                        y10.dividends_to_shareholders / 1_000_000.0,
                        y10.cumulative_dividends / 1_000_000.0,
                        (y10.cumulative_dividends + y10.equity_value_ifrs_fv) / 1_000_000.0,
                        moic_at_fv,
                        y10.shareholders_equity / 1_000_000.0));
    s
}

fn render_comparator_table() -> String {
    use legacy_jv_proforma::*;
    let years = forecast();
    let y10 = &years[10];

    let moic_d7 = (y10.cumulative_dividends + y10.equity_value_ifrs_fv) / LEGACY_JV_GROSS_EQUITY;

    let cfs_d7 = legacy_jv_irr_cash_flows(&years);
    let irr_d7 = xirr_annual(&cfs_d7).unwrap_or(0.0);

    let cfs_d2 = pclp1_irr_cash_flows();
    let irr_d2 = xirr_annual(&cfs_d2).unwrap_or(0.0);

    let mut s = String::new();
    s.push_str("<h2>D7 Legacy JV vs D2 Professional Centres Canada LP — Headline Comparison (BRIEF §2564-2575)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric (Y10)</th><th>D7 Legacy JV</th><th>D2 Professional Centres Canada LP</th><th>D2 advantage</th></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Total sf delivered</td><td>{} sf</td><td>3,906,855 sf</td><td>+41% (LP additional vs LP total)</td></tr>\n",
                        fmt_int(LEGACY_JV_TOTAL_SF)));
    s.push_str("<tr><td class=\"lbl\">Total development capital</td><td>$990M (net)</td><td>$1,211M</td><td>+22% capital</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Equity / partner contributions (gross)</td><td>$250M</td><td>$250M</td><td>same</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Debt to partner contributions</td><td>3.0× ($750M / $250M)</td><td>~4.0× ($994M peak / $250M)</td><td>D2 higher leverage</td></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Partner contributions per SF (equity cost)</td><td>${:.2}/sf</td><td>$63.99/sf</td><td>LP $44.79/sf lower</td></tr>\n",
                        LEGACY_JV_GROSS_EQUITY / LEGACY_JV_TOTAL_SF));
    s.push_str("<tr><td class=\"lbl\">Leverage structure</td><td>$750M bank debt (3.0× gross D/E)</td><td>Debentures (phased)</td><td>structural</td></tr>\n");

    // Dual-column AV
    s.push_str(&format!("<tr><td class=\"lbl\">Stabilized asset value — ASPE book (Y10)</td><td>{}</td><td>N/A (IFRS FV only)</td><td>—</td></tr>\n",
                        fmt_m(y10.asset_value_aspe)));
    s.push_str(&format!("<tr><td class=\"lbl\">Stabilized asset value — IFRS FV</td><td>{}</td><td>~$2,060M</td><td>+63%</td></tr>\n",
                        fmt_m(LEGACY_JV_STABILIZED_AV)));

    // Dual-column equity
    s.push_str(&format!("<tr><td class=\"lbl\">Shareholders' / unitholders' equity — ASPE book (Y10)</td><td>{}</td><td>N/A</td><td>—</td></tr>\n",
                        fmt_m(y10.shareholders_equity)));
    s.push_str(&format!("<tr><td class=\"lbl\">Equity value — IFRS FV (Y10)</td><td>{}</td><td>~$1,090M</td><td>+2.1×</td></tr>\n",
                        fmt_m(y10.equity_value_ifrs_fv)));

    s.push_str(&format!("<tr><td class=\"lbl\"><strong>MOIC — IFRS FV basis (pre-tax, gross)</strong></td><td><strong>{:.2}×</strong></td><td><strong>~4.0×</strong></td><td><strong>+{:.0}%</strong></td></tr>\n",
                        moic_d7, ((4.0 - moic_d7) / moic_d7) * 100.0));
    s.push_str(&format!("<tr><td class=\"lbl\"><strong>IRR (pre-tax; IFRS FV terminal)</strong></td><td><strong>{:.1}%</strong></td><td><strong>{:.1}%</strong></td><td><strong>Professional Centres Canada LP higher</strong></td></tr>\n",
                        irr_d7 * 100.0, irr_d2 * 100.0));
    s.push_str("<tr><td class=\"lbl\">Refinancing headroom at stabilization</td><td>$69M ($1,260M × 65% = $819M; debt $750M)</td><td>Phased (3 tranches)</td><td>structural</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Continuous development rounds possible?</td><td>No (single-shot; $69M ≪ $750M needed)</td><td>Yes</td><td>compounding</td></tr>\n");
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Accounting basis: D7 uses ASPE 3061 cost model — asset value declines 50-yr SL ($21.7M/yr depreciation on $1,086.25M building component). D2 reports under IFRS with income-capitalization (no depreciation). The IFRS FV column is the apples-to-apples basis for MOIC and IRR comparison. ASPE book equity shown for disclosure but is not the return basis.<br>\n");
    s.push_str("IRR computed via Newton-Raphson XIRR on annual series. D7: Y0 = −$250M, Y1–Y3 = $0 (construction), Y4–Y10 = net dividends, Y10 terminal = IFRS FV equity $510M. D2: investor fraction (90%) of aggregate LP distributions + investor fraction of NAV at Y10.</p>\n");
    s
}

pub fn render_summary() -> String {
    use legacy_jv_proforma::*;
    let years = forecast();
    let y10 = &years[10];
    let mut s = String::new();
    s.push_str(&head_with_title("Legacy JV (D7) — Summary V4"));
    s.push_str("<body>\n");
    s.push_str("<h1>Legacy JV (D7) — Investor Summary V4</h1>\n");
    s.push_str("<p>Engine-generated comparator summary from BRIEF v0.15.6 §5h.<br>\n");
    s.push_str("DRAFT — 2026-06-06 — V4<br>\n");
    s.push_str(
        "Companion: <code>COMPLIANCE_MCorp_2026_06_06_Proforma_LegacyJV_V4.html</code></p>\n",
    );

    s.push_str("<p>Legacy JV (D7) is the traditional joint-venture comparator to Professional Centres Canada LP (D2). Same $250M gross equity but financed with $750M bank debt at $990M net capital. Traditional Inc. with shareholders agreement; 2/20 management fee structure. Construction Y1–Y3 (S-curve 20/50/30), stabilized Y4+. All construction-period costs capitalized under ASPE 3061; Y1–Y3 net income = $0. No further development rounds possible at scale — single-shot structure.</p>\n");

    s.push_str(&render_endpoint_summary(y10, &years));
    s.push_str(&render_comparator_table());
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json() -> String {
    serde_json::to_string_pretty(&legacy_jv_proforma::forecast_json())
        .expect("Legacy JV JSON serialization failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proforma_well_formed() {
        let html = render_proforma();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Legacy JV"));
        assert!(html.contains("Capital Structure"));
        assert!(html.contains("Income Statement"));
        assert!(html.contains("Single-Shot Constraint"));
        assert!(html.contains("Professional Centres Canada LP"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn proforma_v2_terminology() {
        let html = render_proforma();
        // V2 should use Inc./shareholder terminology
        assert!(
            html.contains("shareholders"),
            "Should use 'shareholders' not 'LP'"
        );
        assert!(
            html.contains("Management Co."),
            "Should reference Management Co."
        );
        assert!(
            !html.contains("distributions_to_lps"),
            "Old field name must not appear"
        );
        // V2 should reference corrected NOI
        assert!(html.contains("78.75"), "NOI should be $78.75M");
        // V2 should reference IRR
        assert!(html.contains("IRR"), "Comparator must include IRR row");
        // V2 should reference ASPE book column
        assert!(html.contains("ASPE book"), "Must show ASPE book column");
    }

    #[test]
    fn summary_well_formed() {
        let html = render_summary();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Legacy JV"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn json_valid() {
        let json_str = render_json();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["version"], "V2");
        assert!(parsed["years"].is_array());
    }
}
