// WCP Inc. (Woodfine Capital Projects Inc.) — proforma + summary HTML renderers.
//
// Consumes engine output from `src/spv/wcp_proforma.rs`. Produces:
//   - render_proforma() — full 10-year IS / Book Value / Four Valuations
//   - render_summary() — single-page investor rollup with per-share metrics
//   - render_json() — engine state dump

use crate::spv::pclp1_proforma;
use crate::spv::wcp_proforma::{self, WcpYear};

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

fn fmt_per_share(v: f64) -> String {
    if v.abs() < 1e-4 {
        "—".to_string()
    } else {
        format!("${:.2}", v)
    }
}

fn fmt_pct(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}

// ─── HTML scaffold ──────────────────────────────────────────────────────────

const HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Woodfine Capital Projects Inc. (WCP) — Proforma V2</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1400px}
h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.76rem}
table.wide{width:100%;table-layout:fixed}
table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}
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

fn header_block(version: &str) -> String {
    format!(
        "<h1>Woodfine Capital Projects Inc. (WCP) — Proforma {version}</h1>\n\
         <p>DRAFT — 2026-06-10 — {version}<br>\n\
         All amounts CAD — Prepared under IFRS — Forward-looking projections; BCSC continuous-disclosure posture</p>\n"
    )
}

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — Notice under applicable securities legislation including the British Columbia Securities Commission (BCSC) and NI 51-102.</strong> This document contains forward-looking information. All amounts shown are computed from declared inputs and the DHS-01 (Professional Centres Canada LP) forecast. Actual results may differ materially.</p>\n".to_string()
}

// ─── Section renderers ──────────────────────────────────────────────────────

fn render_inputs() -> String {
    let mut s = String::new();
    s.push_str("<h2>Capital Structure &amp; Key Inputs</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Parameter</th><th>Value</th><th>Ref</th></tr>\n");

    let rows = [
        (
            "Shares outstanding",
            fmt_int(wcp_proforma::WCP_SHARES_OUTSTANDING),
            "§1016",
        ),
        (
            "Price per share Y0",
            format!("${:.2}", wcp_proforma::WCP_PRICE_PER_SHARE_Y0),
            "§1017",
        ),
        (
            "Financing tranche Y1",
            fmt_full_dollar(wcp_proforma::WCP_FINANCING_Y1),
            "§1020",
        ),
        (
            "Financing tranche Y2",
            fmt_full_dollar(wcp_proforma::WCP_FINANCING_Y2),
            "§1020",
        ),
        (
            "CAD-USD rate",
            format!("{:.4}", wcp_proforma::WCP_CAD_USD),
            "§1018",
        ),
        (
            "CAD-EUR rate",
            format!("{:.4}", wcp_proforma::WCP_CAD_EUR),
            "§1019",
        ),
        ("Tax rate", fmt_pct(wcp_proforma::WCP_TAX_RATE), "§1021"),
        (
            "P/E multiple",
            format!("{:.2}×", wcp_proforma::WCP_PE_MULTIPLE),
            "§1022",
        ),
        (
            "Earnings yield",
            fmt_pct(wcp_proforma::WCP_DIVIDEND_YIELD),
            "§1023",
        ),
        (
            "DHS beneficial ownership (each)",
            fmt_pct(wcp_proforma::WCP_LP_BENEFICIAL_OWNERSHIP),
            "§1075",
        ),
    ];
    for (lbl, val, cell) in &rows {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td></tr>\n",
            lbl, val, cell
        ));
    }
    s.push_str("</table>\n");

    s.push_str("<h3>Six Direct-Hold Solutions (Revenue Generator)</h3>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Direct-Hold Solution</th><th>Launch</th><th>Size factor</th><th>Advisory FX</th><th>Distribution FX</th><th>NAV FX</th></tr>\n");
    for lp in wcp_proforma::WCP_LPS.iter() {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>Y{}</td><td>{:.0}×</td><td>{:.4}</td><td>{:.4}</td><td>{:.4}</td></tr>\n",
            lp.name, lp.launch_year, lp.size_factor, lp.advisory_fx, lp.dist_fx, lp.nav_fx
        ));
    }
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">DHS-01 = Professional Centres Canada LP look-through. DHS-02 through DHS-06 derived from DHS-01 with size factor × FX × launch lag. Distribution FX rates for USD-denominated direct-hold solutions are stated at the CAD-USD rate (revised in V2).</p>\n");
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

fn data_row<F: Fn(&WcpYear) -> f64>(label: &str, years: &[WcpYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_ps<F: Fn(&WcpYear) -> f64>(label: &str, years: &[WcpYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_per_share(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn render_revenue_generator(years: &[WcpYear]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Revenue Generator — Six Direct-Hold Solutions (CAD-equivalent)</h2>\n");

    s.push_str("<h3>Advisory fees by direct-hold solution</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    for (i, lp) in wcp_proforma::WCP_LPS.iter().enumerate() {
        s.push_str(&data_row(lp.name, years, move |y| {
            y.lps.get(i).map(|l| l.advisory_fee).unwrap_or(0.0)
        }));
    }
    s.push_str(&data_row("Total advisory fees", years, |y| {
        y.advisory_fees_total
    }));
    s.push_str("</table>\n");

    s.push_str("<h3>Distributions by direct-hold solution (WCP's 10% beneficial ownership)</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    for (i, lp) in wcp_proforma::WCP_LPS.iter().enumerate() {
        s.push_str(&data_row(lp.name, years, move |y| {
            y.lps.get(i).map(|l| l.distributions).unwrap_or(0.0)
        }));
    }
    s.push_str(&data_row("Total distributions", years, |y| {
        y.distributions_total
    }));
    s.push_str("</table>\n");

    s.push_str("<h3>NAV by direct-hold solution (WCP's 10% beneficial ownership)</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    for (i, lp) in wcp_proforma::WCP_LPS.iter().enumerate() {
        s.push_str(&data_row(lp.name, years, move |y| {
            y.lps.get(i).map(|l| l.nav).unwrap_or(0.0)
        }));
    }
    s.push_str(&data_row("Total NAV (book ownership)", years, |y| {
        y.nav_total_lps
    }));
    s.push_str("</table>\n");

    s
}

fn render_income_statement(years: &[WcpYear]) -> String {
    let mut s = String::new();
    s.push_str("<h2>10-Year Income Statement (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str("<tr class=\"section-banner\"><td colspan=\"12\">Revenue</td></tr>\n");
    s.push_str(&data_row("Advisory fees (total)", years, |y| {
        y.advisory_fees_total
    }));
    s.push_str(&data_row("Distributions from DHS holdings", years, |y| {
        y.distributions_total
    }));
    s.push_str(&data_row("Offering costs reimbursement", years, |y| {
        y.offering_costs
    }));
    s.push_str(&data_row("Gross income", years, |y| y.gross_income));
    s.push_str("<tr class=\"section-banner\"><td colspan=\"12\">Operating Expenses</td></tr>\n");
    s.push_str(&data_row("Referral fees (Y1–Y2)", years, |y| {
        y.referral_fees
    }));
    s.push_str(&data_row(
        "WPI compensation agreement (Y1–Y2)",
        years,
        |y| y.wpi_consulting,
    ));
    s.push_str(&data_row("G&amp;A — New York City", years, |y| y.gna_nyc));
    s.push_str(&data_row("G&amp;A — Berlin", years, |y| y.gna_berlin));
    s.push_str(&data_row("Total operating expenses", years, |y| {
        y.total_opex
    }));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">EBITDA</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.ebitda)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row("Taxes (27%)", years, |y| -y.taxes));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Earnings</td>");
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.earnings)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row_ps("EPS", years, |y| y.eps));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">EBITDA is negative in Y1–Y2 (capital raise period; high WPI compensation agreement and referral fees). Projected to turn positive from Y3 as DHS advisory fees increase. Taxes at 27%; earnings = EBITDA × (1 − 0.27).</p>\n");
    s
}

fn render_book_valuation(years: &[WcpYear]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Book Valuation</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row("Financing activity", years, |y| {
        y.financing_activity
    }));
    s.push_str(&data_row("Cumulative free cash flow", years, |y| {
        y.cumulative_fcf
    }));
    s.push_str(&data_row(
        "Direct-hold solution ownership (10% of DHS NAV)",
        years,
        |y| y.lp_ownership_book,
    ));
    s.push_str(&data_row("Book value", years, |y| y.book_value));
    s.push_str(&data_row_ps("Book value per share", years, |y| {
        y.book_value_per_share
    }));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Book value = cumulative free cash flow (financing + earnings) + DHS beneficial ownership (10% of each DHS's NAV).</p>\n");
    s
}

fn render_four_valuations(years: &[WcpYear]) -> String {
    let mut s = String::new();
    s.push_str("<h2>Valuation — Three Methods and Fair Value Composite</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header_row());
    s.push_str(&data_row("Earnings valuation (× 10.72 P/E)", years, |y| {
        y.earnings_valuation
    }));
    s.push_str(&data_row_ps("  per share", years, |y| {
        y.market_value_per_share
    }));
    s.push_str(&data_row(
        "Earnings yield valuation (÷ 4.5%)",
        years,
        |y| y.dividend_valuation,
    ));
    s.push_str(&data_row_ps("  per share", years, |y| {
        y.dividend_value_per_share
    }));
    s.push_str(&data_row_ps("Book value per share", years, |y| {
        y.book_value_per_share
    }));
    s.push_str(&data_row_ps(
        "Fair value per share (3-method average)",
        years,
        |y| y.fair_value_per_share,
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Three valuation methods: Earnings valuation (× 10.72 P/E); Earnings yield valuation (÷ 4.5% target yield); Book value (cumulative free cash flow + DHS ownership). Fair value composite = simple average of the three per-share values.</p>\n");
    s
}

// ─── Public renderers ──────────────────────────────────────────────────────

pub fn render_proforma() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let years = wcp_proforma::forecast(&pclp1);
    let mut s = String::new();
    s.push_str(HEAD);
    s.push_str("<body>\n");
    s.push_str(&header_block("V2"));
    s.push_str(&render_inputs());
    s.push_str(&render_revenue_generator(&years));
    s.push_str(&render_income_statement(&years));
    s.push_str(&render_book_valuation(&years));
    s.push_str(&render_four_valuations(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_summary() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let years = wcp_proforma::forecast(&pclp1);
    let y10 = &years[10];
    let mut s = String::new();
    s.push_str(&HEAD.replace(
        "<title>Woodfine Capital Projects Inc. (WCP) — Proforma V2</title>",
        "<title>Woodfine Capital Projects Inc. (WCP) — Summary V2</title>",
    ));
    s.push_str("<body>\n");
    s.push_str("<h1>Woodfine Capital Projects Inc. (WCP) — Summary V2</h1>\n");
    s.push_str("<p>DRAFT — 2026-06-10 — V2<br>\n");
    s.push_str("Companion: <code>COMPLIANCE_MCorp_2026_06_04_Proforma_WCP_V2.html</code> (full 10-year proforma)<br>\n");
    s.push_str("All amounts CAD — Prepared under IFRS — Forward-looking projections; BCSC continuous-disclosure posture</p>\n");

    s.push_str("<h2>Capital Structure</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Item</th><th>Value</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Shares outstanding</td><td>{}</td></tr>\n",
        fmt_int(wcp_proforma::WCP_SHARES_OUTSTANDING)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Price per share Y0</td><td>${:.2}</td></tr>\n",
        wcp_proforma::WCP_PRICE_PER_SHARE_Y0
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y0 implied market cap</td><td>{}</td></tr>\n",
        fmt_full_dollar(
            wcp_proforma::WCP_SHARES_OUTSTANDING * wcp_proforma::WCP_PRICE_PER_SHARE_Y0
        )
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total Y1–Y2 financing</td><td>{}</td></tr>\n",
        fmt_full_dollar(wcp_proforma::WCP_FINANCING_Y1 + wcp_proforma::WCP_FINANCING_Y2)
    ));
    s.push_str("</table>\n");

    s.push_str("<h2>Y10 Endpoint Summary</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per share (10M)</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Earnings</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.earnings),
        fmt_per_share(y10.eps)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Book value</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.book_value),
        fmt_per_share(y10.book_value_per_share)
    ));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 Earnings valuation (× 10.72 P/E)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.market_valuation), fmt_per_share(y10.market_value_per_share)));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 Earnings yield value (Earnings / 4.5%)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.dividend_valuation), fmt_per_share(y10.dividend_value_per_share)));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Y10 Fair value per share (3-method average)</td><td>—</td><td>{}</td></tr>\n",
                        fmt_per_share(y10.fair_value_per_share)));
    s.push_str("</table>\n");

    s.push_str("<h2>10-Year Earnings &amp; Per-Share Progression</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>EBITDA</th><th>Earnings</th><th>EPS</th><th>Book/share</th><th>Market/share</th><th>Yield/shr</th></tr>\n");
    for y in &years {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            y.year, fmt_m(y.ebitda), fmt_m(y.earnings), fmt_per_share(y.eps),
            fmt_per_share(y.book_value_per_share),
            fmt_per_share(y.market_value_per_share),
            fmt_per_share(y.dividend_value_per_share)
        ));
    }
    s.push_str("</table>\n");

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json() -> String {
    let pclp1 = pclp1_proforma::forecast();
    serde_json::to_string_pretty(&wcp_proforma::forecast_json(&pclp1))
        .expect("WCP JSON serialization failed")
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proforma_well_formed() {
        let html = render_proforma();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Woodfine Capital Projects"));
        assert!(html.contains("Revenue Generator"));
        assert!(html.contains("10-Year Income Statement"));
        assert!(html.contains("Book Valuation"));
        assert!(html.contains("Valuation — Three Methods"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn summary_well_formed() {
        let html = render_summary();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Summary V2"));
        assert!(html.contains("Y10 Endpoint Summary"));
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
