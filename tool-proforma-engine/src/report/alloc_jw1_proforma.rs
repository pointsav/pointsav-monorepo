// WCP $500,000 Common Share Allocation Proforma renderer (JW1).
// CSS, HEAD, LNUM_SCRIPT, and formatting helpers are verbatim copies from
// bencal_v1_proforma.rs to keep this module self-contained.

use crate::spv::{alloc_jw1_proforma, alloc_jw1_proforma::AllocJw1Year, pclp1_proforma, wcp_proforma};

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
    if n < 0 { format!("-{out}") } else { out }
}

fn fmt_full_dollar(v: f64) -> String {
    format!("${}", fmt_int(v))
}

fn fmt_per_share(v: f64) -> String {
    format!("${:.2}", v)
}

// ─── HTML constants ──────────────────────────────────────────────────────────

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
table.wide{width:100%}
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:230px}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.8cm 1.5cm}
@page{@bottom-center{content:"- " counter(page) " -";font-size:9px;color:#666}}
@page :first{@bottom-center{content:""}}
@media print{body{margin:0;font-size:11px;max-width:none}table.wide{table-layout:fixed;font-size:10px}table.wide td,table.wide th{padding:3px 6px}table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}}
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

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — BCSC NI 51-102.</strong> Engine-generated proforma from BRIEF v0.15.6. Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture. Actual results may differ materially.</p>\n".to_string()
}

// ─── Row helpers ─────────────────────────────────────────────────────────────

fn data_row_jw1<F: Fn(&AllocJw1Year) -> f64>(
    label: &str,
    years: &[AllocJw1Year],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn int_row_jw1<F: Fn(&AllocJw1Year) -> f64>(
    label: &str,
    years: &[AllocJw1Year],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        let v = pick(y);
        let cell = if v.abs() < 1e-2 { "—".to_string() } else { fmt_int(v) };
        s.push_str(&format!("<td>{}</td>", cell));
    }
    s.push_str("</tr>\n");
    s
}

// ─── Main renderer ───────────────────────────────────────────────────────────

pub fn render_proforma_jw1() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let years = alloc_jw1_proforma::forecast(&wcp);
    let cost_basis = alloc_jw1_proforma::JW1_WCP_ALLOCATED_SHARES
        * alloc_jw1_proforma::JW1_WCP_PURCHASE_PRICE
        + alloc_jw1_proforma::JW1_WCP_BONUS_SHARES * alloc_jw1_proforma::JW1_WCP_BONUS_NOMINAL;

    let title =
        "Woodfine Capital Projects Inc. — $500,000 Common Share Allocation | Proforma V1 (JW1)";
    let mut s = String::new();
    s.push_str(&head_with_title(title));
    s.push_str("<body>\n");
    s.push_str(&format!("<h1>{}</h1>\n", title));
    s.push_str("<p>Engine-generated proforma — 2026-06-09 — V1<br>\n");
    s.push_str("All amounts CAD — Prepared under IFRS 9 FVTPL / IFRS 13 Level 3</p>\n");

    // ─── Section 1: Capital Structure ────────────────────────────────────
    s.push_str("<h2>Capital Structure &amp; Allocation Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Units / Shares</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Allocated WCP common shares</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        fmt_int(alloc_jw1_proforma::JW1_WCP_ALLOCATED_SHARES),
        fmt_per_share(alloc_jw1_proforma::JW1_WCP_PURCHASE_PRICE),
        fmt_full_dollar(
            alloc_jw1_proforma::JW1_WCP_ALLOCATED_SHARES
                * alloc_jw1_proforma::JW1_WCP_PURCHASE_PRICE
        )
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Bonus WCP common shares (4:1 ratio)</td><td>{}</td><td>${:.8}</td><td>${:.2}</td></tr>\n",
        fmt_int(alloc_jw1_proforma::JW1_WCP_BONUS_SHARES),
        alloc_jw1_proforma::JW1_WCP_BONUS_NOMINAL,
        alloc_jw1_proforma::JW1_WCP_BONUS_SHARES * alloc_jw1_proforma::JW1_WCP_BONUS_NOMINAL
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total WCP common shares held (1.0% of 10M outstanding)</td><td>{}</td><td>—</td><td>—</td></tr>\n",
        fmt_int(alloc_jw1_proforma::JW1_WCP_TOTAL_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Loan to Woodfine Properties Inc. (7% p.a., 18-month term)</td><td>—</td><td>—</td><td>{}</td></tr>\n",
        fmt_full_dollar(alloc_jw1_proforma::JW1_LOAN_PRINCIPAL)
    ));
    s.push_str("</table>\n");
    s.push_str(&format!(
        "<p class=\"note\">Total equity cost basis: {} ({} allocated shares at {} + {} bonus shares at nominal $0.000332 par). \
        Loan is a separate cash deployment earning 7% p.a.; principal returned in Y2 after 18 months. \
        Y3 Capital Recovery Sale: 25,000 allocated shares sold at WCP engine book value per share.</p>\n",
        fmt_full_dollar(cost_basis),
        fmt_int(alloc_jw1_proforma::JW1_WCP_ALLOCATED_SHARES),
        fmt_per_share(alloc_jw1_proforma::JW1_WCP_PURCHASE_PRICE),
        fmt_int(alloc_jw1_proforma::JW1_WCP_BONUS_SHARES)
    ));

    // ─── Section 2: Income Statement ─────────────────────────────────────
    s.push_str("<h2>10-Year Income Statement &amp; Loan Position (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    s.push_str(&data_row_jw1(
        "Interest income — Woodfine Properties Inc. loan",
        &years,
        |y| y.interest_income,
    ));
    s.push_str(&data_row_jw1(
        "Realised gain — WCP share sale (Y3 Capital Recovery Sale)",
        &years,
        |y| y.realised_gain_crs,
    ));
    s.push_str(&data_row_jw1(
        "Net change in FV — WCP shares (FVTPL)",
        &years,
        |y| y.wcp_fv_change,
    ));
    s.push_str("<tr class=\"subtotal\">");
    s.push_str("<td class=\"lbl\">Total investment income</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.total_investment_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row_jw1("Income (loss) before tax", &years, |y| y.income_before_tax));
    s.push_str(&data_row_jw1("Tax (27%)", &years, |y| -y.tax));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Net income (loss)</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row_jw1(
        "Loan receivable — closing balance",
        &years,
        |y| y.loan_receivable_closing,
    ));
    s.push_str(&int_row_jw1(
        "WCP shares held — closing",
        &years,
        |y| y.wcp_shares_held,
    ));
    s.push_str("</table>\n");
    s.push_str(
        "<p class=\"note\">Y0 unrealised FV change of approximately \u{2212}$45,025 reflects \
        remeasurement of 100,000 WCP common shares from their cost of $500,024.92 to the IFRS 13 \
        Level 3 management proxy of $4.55 per share ($455,000), consistent with the portfolio-wide \
        WCP valuation input applied across Woodfine Capital Projects entities. This is a measurement \
        convention, not a value impairment; the economic cost of the allocation is $500,024.92. \
        Level 3 proxy is held flat through Y2; the Y3 Capital Recovery Sale transaction establishes \
        a transaction price that transitions valuation to the WCP engine book value path.</p>\n",
    );

    // ─── Sections 3 + 4 ──────────────────────────────────────────────────
    s.push_str(&render_jw1_summary_section(&years, cost_basis));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// ─── Return Summary (Section 3) ─────────────────────────────────────────────

fn render_jw1_summary_section(years: &[AllocJw1Year], cost_basis: f64) -> String {
    let y3 = &years[3];
    let y10 = &years[10];
    let total_interest = years[1].interest_income + years[2].interest_income;
    let equity_return = y3.realised_gain_crs + y10.residual_nav;
    let total_return =
        total_interest + alloc_jw1_proforma::JW1_LOAN_PRINCIPAL + equity_return;
    let moic = equity_return / cost_basis;

    let mut s = String::new();
    s.push_str(
        "<h2 style=\"page-break-before:always;break-before:page\">Allocation Return Summary (Y10 endpoint)</h2>\n",
    );
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per WCP share</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total equity cost basis (Y0)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_full_dollar(cost_basis),
        fmt_per_share(cost_basis / alloc_jw1_proforma::JW1_WCP_TOTAL_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Interest income received (Y1\u{2013}Y2)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_full_dollar(total_interest),
        fmt_per_share(total_interest / alloc_jw1_proforma::JW1_WCP_TOTAL_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Loan principal returned (Y2)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_full_dollar(alloc_jw1_proforma::JW1_LOAN_PRINCIPAL),
        fmt_per_share(
            alloc_jw1_proforma::JW1_LOAN_PRINCIPAL / alloc_jw1_proforma::JW1_WCP_TOTAL_SHARES
        )
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y3 Capital Recovery Sale (25,000 allocated shares)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y3.realised_gain_crs),
        fmt_per_share(y3.wcp_per_share_value) // sale price per WCP share at Y3
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Residual fair value (75,000 WCP bonus shares)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.residual_nav),
        fmt_per_share(y10.wcp_per_share_value) // NAV per held share at Y10
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total return Y0\u{2192}Y10</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(total_return),
        fmt_per_share(total_return / alloc_jw1_proforma::JW1_WCP_TOTAL_SHARES)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC (equity return / equity cost basis)</td><td>{:.2}\u{00d7}</td><td>\u{2014}</td></tr>\n",
        moic
    ));
    s.push_str("</table>\n");
    s.push_str(
        "<p class=\"note\">MOIC denominator is the equity cost basis ($500,024.92); loan principal \
        return is a cash-neutral round-trip and is excluded from the equity return calculation. \
        Y3 and Y10 \u{201c}Per WCP share\u{201d} values show the per-share price at each event \
        (sale price and NAV respectively). All other per-share values use 100,000 total shares \
        as denominator.</p>\n",
    );
    s.push_str(&render_jw1_annual_returns(years, cost_basis));
    s
}

// ─── Annual Returns (Section 4) ─────────────────────────────────────────────

fn render_jw1_annual_returns(years: &[AllocJw1Year], cost_basis: f64) -> String {
    let mut s = String::new();
    s.push_str("<h3>Annual Returns \u{2014} per $1.00 Invested (cost basis: $500,024.92)</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");

    // Cash receipts per $1.00 invested: interest (Y1/Y2) + loan principal (Y2) + CRS (Y3)
    s.push_str("<tr><td class=\"lbl\">Cash receipts per $1.00 invested</td>");
    for yr in years {
        let principal_ret = if yr.year == 2 { alloc_jw1_proforma::JW1_LOAN_PRINCIPAL } else { 0.0 };
        let cash = yr.interest_income + yr.realised_gain_crs + principal_ret;
        if cash.abs() < 1e-2 {
            s.push_str("<td>\u{2014}</td>");
        } else {
            s.push_str(&format!("<td>${:.4}</td>", cash / cost_basis));
        }
    }
    s.push_str("</tr>\n");

    // WCP NAV per share (engine output — unchanged)
    s.push_str("<tr><td class=\"lbl\">WCP NAV per share</td>");
    for yr in years {
        s.push_str(&format!("<td>{}</td>", fmt_per_share(yr.wcp_per_share_value)));
    }
    s.push_str("</tr>\n");

    s.push_str("</table>\n");
    s.push_str(
        "<p class=\"note\">Cash receipts show all cash returned to the investor in that year \
        (interest, loan principal, CRS proceeds) divided by the total cost basis of $500,024.92. \
        A value of $1.0350 in Y2 means the investor received $1.035 back for every dollar \
        originally deployed that year (principal return plus half-year interest). \
        WCP NAV per share is the WCP engine book value; \
        the Y3 figure is the Capital Recovery Sale price per share.</p>\n",
    );
    s
}

// ─── JSON renderer ───────────────────────────────────────────────────────────

pub fn render_json_jw1() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    serde_json::to_string_pretty(&alloc_jw1_proforma::forecast_json(&wcp))
        .expect("JW1 JSON serialization failed")
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_jw1_well_formed() {
        assert!(render_proforma_jw1().starts_with("<!DOCTYPE"));
    }

    #[test]
    fn json_jw1_parses() {
        let j = render_json_jw1();
        let _: serde_json::Value = serde_json::from_str(&j).unwrap();
    }
}
