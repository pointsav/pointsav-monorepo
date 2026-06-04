// Legacy JV (D7) — proforma + summary HTML renderers.
// Apples-to-apples comparator to PCLP 1.

use crate::spv::legacy_jv_proforma::{self, LegacyJvYear};

// ─── Formatting helpers ─────────────────────────────────────────────────────

fn fmt_m(v: f64) -> String {
    if v.abs() < 1e-2 { "—".to_string() }
    else if v.abs() >= 1_000_000.0 { format!("${:.2}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("${:.0}K", v / 1_000.0) }
    else { format!("${:.0}", v) }
}

fn fmt_int(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.abs().to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 { out.push(','); }
        out.push(*b as char);
    }
    if n < 0 { format!("-{out}") } else { out }
}

fn fmt_full_dollar(v: f64) -> String { format!("${}", fmt_int(v)) }
fn fmt_pct(v: f64) -> String { format!("{:.1}%", v * 100.0) }
fn fmt_per_unit(v: f64) -> String { format!("${:.2}", v) }

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
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:200px}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{body{margin:0;font-size:11px;max-width:none}table{break-inside:avoid}}
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

fn head_with_title(title: &str) -> String { HEAD.replace("__TITLE__", title) }

fn year_header() -> String {
    let mut s = String::from("<tr><th class=\"lbl\">Line</th>");
    for y in 0..=10 { s.push_str(&format!("<th>Y{}</th>", y)); }
    s.push_str("</tr>\n");
    s
}

fn data_row<F: Fn(&LegacyJvYear) -> f64>(label: &str, years: &[LegacyJvYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years { s.push_str(&format!("<td>{}</td>", fmt_m(pick(y)))); }
    s.push_str("</tr>\n");
    s
}

fn data_row_pct<F: Fn(&LegacyJvYear) -> f64>(label: &str, years: &[LegacyJvYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        let v = pick(y);
        if v > 0.001 { s.push_str(&format!("<td>{}</td>", fmt_pct(v))); }
        else { s.push_str("<td>—</td>"); }
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_x<F: Fn(&LegacyJvYear) -> f64>(label: &str, years: &[LegacyJvYear], pick: F) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        let v = pick(y);
        if v > 0.001 { s.push_str(&format!("<td>{:.2}×</td>", v)); }
        else { s.push_str("<td>—</td>"); }
    }
    s.push_str("</tr>\n");
    s
}

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — BCSC NI 51-102.</strong> Engine-generated proforma from BRIEF v0.15.6 §5h. Apples-to-apples comparator to PCLP 1; not an investment offering. Actual results may differ materially.</p>\n".to_string()
}

// ─── Public renderers ───────────────────────────────────────────────────────

pub fn render_proforma() -> String {
    use legacy_jv_proforma::*;
    let years = forecast();
    let y10 = &years[10];

    let mut s = String::new();
    s.push_str(&head_with_title("Legacy JV (D7) — Proforma V1"));
    s.push_str("<body>\n");
    s.push_str("<h1>Legacy JV (D7) — Traditional Joint Venture Proforma V1</h1>\n");
    s.push_str("<p>Engine-generated comparator proforma from BRIEF v0.15.6 §5h. Apples-to-apples to PCLP 1 (D2). No Excel.<br>\n");
    s.push_str("DRAFT — 2026-06-04 — V1<br>\n");
    s.push_str("Subtitle: 'Traditional J/V Financing vs. the Woodfine LPs'<br>\n");
    s.push_str("All amounts CAD — ASPE 3061 cost model (50-yr SL depreciation on building)</p>\n");

    s.push_str("<h2>Capital Structure (BRIEF §5h)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Parameter</th><th>Value</th><th>Notes</th></tr>\n");
    let rows = [
        ("Equity contributions (LP)", fmt_full_dollar(LEGACY_JV_EQUITY), "Same \\$250M as PCLP 1"),
        ("Bank debt (construction → permanent)", fmt_full_dollar(LEGACY_JV_BANK_DEBT), "3.0× D/E"),
        ("Total capital deployed", fmt_full_dollar(LEGACY_JV_TOTAL_CAPITAL), "Single-shot"),
        ("LP units (= equity / \\$100)", fmt_int(LEGACY_JV_LP_UNITS), ""),
        ("Total portfolio sf", fmt_int(LEGACY_JV_TOTAL_SF), "BRIEF §2469"),
        ("Construction cost / sf", format!("${:.2}", LEGACY_JV_COST_PER_SF), "BRIEF §2468"),
        ("Development yield", fmt_pct(LEGACY_JV_DEV_YIELD), "10.5%"),
        ("Cap rate", fmt_pct(LEGACY_JV_CAP_RATE), "6.25%"),
        ("Stabilized gross rent", fmt_full_dollar(LEGACY_JV_GROSS_REV_STABILIZED), "10.5% × \\$750M debt; BRIEF §2528"),
        ("Stabilized NOI", fmt_full_dollar(LEGACY_JV_NOI_STABILIZED), "Gross rent × (1 - 20% opex)"),
        ("Stabilized asset value (IFRS FV)", fmt_full_dollar(LEGACY_JV_STABILIZED_AV), "= NOI / cap rate"),
        ("Permanent loan interest rate", fmt_pct(LEGACY_JV_INTEREST_RATE), "5%"),
        ("Operating expense ratio", fmt_pct(LEGACY_JV_OPEX_PCT), "~20% of gross"),
        ("Annual G&amp;A", fmt_full_dollar(LEGACY_JV_GA_ANNUAL), ""),
        ("Depreciation (ASPE 3061)", format!("{:.0}-yr SL", LEGACY_JV_DEPRECIATION_YRS), "Building only; land excluded"),
        ("LTV covenant", fmt_pct(LEGACY_JV_LTV_COVENANT), "65%"),
    ];
    for (l, v, n) in &rows {
        s.push_str(&format!("<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td></tr>\n", l, v, n));
    }
    s.push_str("</table>\n");

    s.push_str("<h2>10-Year Timeline (BRIEF §2493-2496)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(&year_header());
    let mut phase_row = String::from("<tr><td class=\"lbl\">Phase</td>");
    for y in &years { phase_row.push_str(&format!("<td>{}</td>", y.phase)); }
    phase_row.push_str("</tr>\n");
    s.push_str(&phase_row);
    s.push_str(&data_row("Capex (S-curve 20/50/30)", &years, |y| y.capex));
    s.push_str(&data_row("Cumulative capex", &years, |y| y.cumulative_capex));
    s.push_str(&data_row("Debt outstanding", &years, |y| y.debt_outstanding));
    s.push_str("</table>\n");

    s.push_str("<h2>10-Year Income Statement (CAD; ASPE 3061)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(&year_header());
    s.push_str(&data_row("Gross rental revenue", &years, |y| y.gross_rental_revenue));
    s.push_str(&data_row("Operating expenses (20%)", &years, |y| -y.operating_expenses));
    s.push_str(&data_row("Net Operating Income (NOI)", &years, |y| y.noi));
    s.push_str(&data_row("Interest on $750M @ 5%", &years, |y| -y.interest_expense));
    s.push_str(&data_row("Depreciation (50-yr SL)", &years, |y| -y.depreciation));
    s.push_str(&data_row("G&amp;A", &years, |y| -y.ga_expense));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">Net income</td>");
    for y in &years { s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income))); }
    s.push_str("</tr>\n");
    s.push_str(&data_row("Add back depreciation", &years, |y| y.depreciation));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Distributable cash</td>");
    for y in &years { s.push_str(&format!("<td>{}</td>", fmt_m(y.distributable_cash))); }
    s.push_str("</tr>\n");
    s.push_str(&data_row("Distributions to LPs", &years, |y| y.distributions_to_lps));
    s.push_str(&data_row("Cumulative distributions", &years, |y| y.cumulative_distributions));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Y1-Y3 construction phase: no revenue, no opex, no depreciation. Y4+ stabilized: gross rent $78.75M; opex (20%) $15.75M; NOI $63M; debt service $37.5M; depreciation $21.1M; G&A $2M; net income ~$2.4M; distributable cash ~$23.5M/yr; cumulative Y4–Y10 distributions ~$164M.</p>\n");

    s.push_str("<h2>Valuation &amp; Ratios</h2>\n");
    s.push_str("<table>\n");
    s.push_str(&year_header());
    s.push_str(&data_row("Asset value (ASPE book)", &years, |y| y.asset_value_aspe));
    s.push_str(&data_row("Equity value (asset − debt, book)", &years, |y| y.equity_value));
    s.push_str(&data_row_pct("LTV (debt / book asset)", &years, |y| y.ltv_book));
    s.push_str(&data_row_x("DSCR (NOI / interest)", &years, |y| y.dscr));
    s.push_str(&data_row("DPU (per LP unit)", &years, |y| y.dpu));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">IFRS Fair Value at Y10 = $63M NOI / 6.25% cap rate = $1,008M. Equity value at FV = $1,008M − $750M debt = $258M. Y10 ASPE book asset value declines due to accumulated depreciation, but FV stays at the income-capitalized level.</p>\n");

    s.push_str("<h2>Single-Shot Constraint (BRIEF §2498-2504)</h2>\n");
    s.push_str("<p>Stabilized asset value $1,008M × 65% LTV covenant = $655M max permanent debt. Existing debt is $750M — already <strong>$95M above LTV covenant ceiling</strong> at stabilization. No refinancing headroom; lender may require a paydown rather than permit a second round. Structurally incapable of compounding into a Phase 2 development without new equity injection.</p>\n");
    s.push_str("<p class=\"note\">By contrast, PCLP 1 (D2) issues debentures in 3 phases on a single fund and achieves 3.9M sf vs Legacy JV's 2.3M sf, with multi-round compounding capability built into the structure.</p>\n");

    s.push_str(&render_endpoint_summary(y10));
    s.push_str(&render_comparator_table());
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn render_endpoint_summary(y10: &LegacyJvYear) -> String {
    use legacy_jv_proforma::*;
    let ifrs_fv_equity = LEGACY_JV_STABILIZED_AV - LEGACY_JV_BANK_DEBT;
    let moic_at_fv = (y10.cumulative_distributions + ifrs_fv_equity) / LEGACY_JV_EQUITY;

    let mut s = String::new();
    s.push_str("<h2>Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per LP unit</th></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Total LP equity invested (Y0)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_full_dollar(LEGACY_JV_EQUITY),
                        fmt_per_unit(LEGACY_JV_UNIT_PRICE)));
    s.push_str(&format!("<tr><td class=\"lbl\">Cumulative distributions Y4–Y10</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.cumulative_distributions),
                        fmt_per_unit(y10.cumulative_distributions / LEGACY_JV_LP_UNITS)));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 IFRS FV asset value</td><td>{}</td><td>—</td></tr>\n",
                        fmt_m(LEGACY_JV_STABILIZED_AV)));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 IFRS FV equity (asset − debt)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(ifrs_fv_equity),
                        fmt_per_unit(ifrs_fv_equity / LEGACY_JV_LP_UNITS)));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Total return (cumulative dist + Y10 equity)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.cumulative_distributions + ifrs_fv_equity),
                        fmt_per_unit((y10.cumulative_distributions + ifrs_fv_equity) / LEGACY_JV_LP_UNITS)));
    s.push_str(&format!("<tr><td class=\"lbl\">MOIC (pre-tax, gross)</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
                        moic_at_fv, moic_at_fv));
    s.push_str("</table>\n");
    s.push_str(&format!("<p class=\"note\">Cash-on-cash yield ~9.4% on \\$250M equity (\\${:.1}M annual distributable / \\$250M = 9.4%). Per BRIEF §2536. MOIC computed at IFRS FV equity value; ASPE book equity value declines with depreciation but real-world investor value is the FV.</p>\n",
                        y10.distributable_cash / 1_000_000.0));
    s
}

fn render_comparator_table() -> String {
    use legacy_jv_proforma::*;
    let mut s = String::new();
    s.push_str("<h2>D7 Legacy JV vs D2 PCLP 1 — Headline Comparison (BRIEF §2564-2575)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric (Y10)</th><th>D7 Legacy JV</th><th>D2 PCLP 1 (V2)</th><th>D2 advantage</th></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Total sf delivered</td><td>{} sf</td><td>3,906,855 sf</td><td>+70%</td></tr>\n",
                        fmt_int(LEGACY_JV_TOTAL_SF)));
    s.push_str("<tr><td class=\"lbl\">Total development capital</td><td>$1,000M</td><td>$1,211M</td><td>—</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Equity in</td><td>$250M</td><td>$250M</td><td>same</td></tr>\n");
    s.push_str(&format!("<tr><td class=\"lbl\">Stabilized asset value</td><td>{}</td><td>~$2,035M</td><td>+2×</td></tr>\n",
                        fmt_m(LEGACY_JV_STABILIZED_AV)));
    s.push_str("<tr><td class=\"lbl\">Equity value (asset − debt, IFRS FV)</td><td>$258M</td><td>~$1,090M</td><td>+4×</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\"><strong>MOIC (pre-tax, gross)</strong></td><td><strong>~1.69×</strong></td><td><strong>~5.0×</strong></td><td><strong>+196%</strong></td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Sf per $1 initial equity (10-yr)</td><td>9.19 sf/$</td><td>15.63 sf/$</td><td>+70%</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Refinancing headroom at stabilization</td><td>(\\$95M) covenant gap</td><td>Phased (3 tranches)</td><td>structural</td></tr>\n");
    s.push_str("<tr><td class=\"lbl\">Continuous development rounds possible?</td><td>No (single-shot)</td><td>Yes</td><td>compounding</td></tr>\n");
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">PCLP 1's debenture-financed phased structure achieves materially higher leverage of the same $250M equity by extending construction over 3 phases, with each phase generating NOI that supports the next. Legacy JV's traditional bank-debt model is capped by the LTV covenant at stabilization and cannot compound without raising new equity.</p>\n");
    s
}

pub fn render_summary() -> String {
    use legacy_jv_proforma::*;
    let years = forecast();
    let y10 = &years[10];
    let mut s = String::new();
    s.push_str(&head_with_title("Legacy JV (D7) — Summary V1"));
    s.push_str("<body>\n");
    s.push_str("<h1>Legacy JV (D7) — Investor Summary V1</h1>\n");
    s.push_str("<p>Engine-generated comparator summary from BRIEF v0.15.6 §5h.<br>\n");
    s.push_str("DRAFT — 2026-06-04 — V1<br>\n");
    s.push_str("Companion: <code>COMPLIANCE_MCorp_2026_06_04_Proforma_LegacyJV_V1.html</code></p>\n");

    s.push_str("<p>Legacy JV (D7) is the traditional joint-venture comparator to PCLP 1 (D2). Same $250M LP equity but financed with $750M bank debt (3× leverage) at $1B total capital, single development round, 2,298,150 sf at $326/sf. Construction Y1-Y3 (S-curve 20/50/30), stabilized Y4+. ASPE 3061 cost model. No further development rounds possible due to LTV covenant constraint.</p>\n");

    s.push_str(&render_endpoint_summary(y10));
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
        assert!(html.contains("D2 PCLP 1"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn summary_well_formed() {
        let html = render_summary();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Summary V1"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn json_valid() {
        let json_str = render_json();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["version"], "V1");
        assert!(parsed["years"].is_array());
    }
}
