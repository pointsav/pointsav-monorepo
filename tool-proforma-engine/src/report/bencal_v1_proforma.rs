// Bencal entity proforma renderers — SPV1, SPV2, Management.
// Each entity gets render_proforma_X(), render_summary_X(), render_json_X().

use crate::spv::bencal_mgmt_proforma::BencalMgmtYear;
use crate::spv::bencal_spv1_proforma::BencalSpv1Year;
use crate::spv::bencal_spv2_proforma::BencalSpv2Year;
use crate::spv::{
    bencal_mgmt_proforma, bencal_spv1_proforma, bencal_spv2_proforma, pclp1_proforma, wcp_proforma,
};

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
@media print{body{margin:0;font-size:11px;max-width:none}}
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

// ─── SPV1 renderers ─────────────────────────────────────────────────────────

pub fn render_proforma_spv1() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let years = bencal_spv1_proforma::forecast(&wcp);

    let mut s = String::new();
    s.push_str(&head_with_title("Bencal Special Purpose 1 — Proforma V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Special Purpose 1 Inc. — Proforma V2</h1>\n");
    s.push_str("<p>Engine-generated proforma from BRIEF v0.15.6 §5e + Bencal cap table v0.15.9. Consumes WCP V1 forecast.<br>\n");
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str("All amounts CAD — Prepared under IFRS</p>\n");

    s.push_str("<h2>Capital Structure &amp; Investment Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Units/Shares</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!("<tr><td class=\"lbl\">Investor common shares</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_INVESTOR_SHARES),
                        bencal_spv1_proforma::SPV1_INVESTOR_SHARE_PRICE,
                        fmt_full_dollar(bencal_spv1_proforma::SPV1_TOTAL_INVESTOR_CAPITAL)));
    s.push_str(&format!("<tr><td class=\"lbl\">Manager shares (10% diluted)</td><td>{}</td><td>—</td><td>—</td></tr>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_MANAGER_SHARES)));
    s.push_str(&format!("<tr><td class=\"lbl\">WCP purchased shares (Y0)</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_WCP_SHARES_PURCHASED),
                        bencal_spv1_proforma::SPV1_WCP_PURCHASED_PRICE,
                        fmt_full_dollar(bencal_spv1_proforma::SPV1_WCP_SHARES_PURCHASED * bencal_spv1_proforma::SPV1_WCP_PURCHASED_PRICE)));
    s.push_str(&format!("<tr><td class=\"lbl\">WCP founding-bonus shares</td><td>{}</td><td>${:.8}</td><td>${:.2}</td></tr>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_WCP_FOUNDING_BONUS),
                        bencal_spv1_proforma::SPV1_WCP_BONUS_NOMINAL,
                        bencal_spv1_proforma::SPV1_WCP_FOUNDING_BONUS * bencal_spv1_proforma::SPV1_WCP_BONUS_NOMINAL));
    s.push_str(&format!("<tr><td class=\"lbl\">Total WCP held (3% of 10M outstanding)</td><td>{}</td><td>—</td><td>—</td></tr>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_WCP_TOTAL)));
    s.push_str("</table>\n");
    s.push_str(&format!("<p class=\"note\">Investor class ({} shares) includes a $54,882 operating reserve funded by Altas One commission rebate at closing (Flag 13 Option C). The reserve is issued as investor-class shares at $1.00 par.</p>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_INVESTOR_SHARES)));

    s.push_str("<h2>10-Year Income Statement &amp; Operating Reserve Drawdown (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    s.push_str(&data_row("Commission rebate (Y0)", &years, |y| {
        y.commission_rebate
    }));
    s.push_str(&data_row(
        "Realised gain — WCP share sales",
        &years,
        |y| y.wcp_crs_realised_gain + y.wcp_annual_sale_gain,
    ));
    s.push_str(&data_row(
        "Net change in FV — WCP shares (FVTPL)",
        &years,
        |y| y.wcp_fv_change,
    ));
    s.push_str(&data_row("Total investment income", &years, |y| {
        y.total_investment_income
    }));
    s.push_str(&data_row("Setup costs (Y0)", &years, |y| -y.setup_costs));
    s.push_str(&data_row("Annual operating expenses", &years, |y| -y.opex));
    s.push_str(&data_row("Income (loss) before tax", &years, |y| {
        y.income_before_tax
    }));
    s.push_str(&data_row("Tax (27%)", &years, |y| -y.tax));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Net income</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row(
        "Operating expense reserve (closing)",
        &years,
        |y| y.opex_reserve_closing,
    ));
    s.push_str("</table>\n");

    s.push_str(&render_spv1_summary_section(&years));

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn render_spv1_summary_section(years: &[BencalSpv1Year]) -> String {
    let y10 = &years[10];
    let total_return = y10.cumulative_cash_to_investors + y10.residual_nav;
    let moic = total_return / bencal_spv1_proforma::SPV1_TOTAL_INVESTOR_CAPITAL;
    let per_share_return = total_return / bencal_spv1_proforma::SPV1_DILUTED_SHARES;
    let mut s = String::new();
    s.push_str("<h2 style=\"page-break-before:always;break-before:page\">Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per investor share</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total investor capital (Y0)</td><td>{}</td><td>${:.2}</td></tr>\n",
        fmt_full_dollar(bencal_spv1_proforma::SPV1_TOTAL_INVESTOR_CAPITAL),
        bencal_spv1_proforma::SPV1_INVESTOR_SHARE_PRICE
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y4 Capital Return Sale dividend</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(years[4].crs_cash_to_investors),
        fmt_per_share(years[4].cumulative_cash_per_share)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Residual fair value</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.residual_nav),
        fmt_per_share(y10.residual_nav / bencal_spv1_proforma::SPV1_DILUTED_SHARES)
    ));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Total return Y0→Y10</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(total_return), fmt_per_share(per_share_return)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic,
        per_share_return / bencal_spv1_proforma::SPV1_INVESTOR_SHARE_PRICE
    ));
    s.push_str("</table>\n");
    s.push_str(&render_spv1_annual_returns(years));
    s
}

pub fn render_summary_spv1() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let years = bencal_spv1_proforma::forecast(&wcp);

    let mut s = String::new();
    s.push_str(&head_with_title("Bencal SPV1 — Summary V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Special Purpose 1 — Investor Summary V2</h1>\n");
    s.push_str(
        "<p>Engine-generated summary from BRIEF v0.15.6 §5e + Bencal cap table v0.15.9.<br>\n",
    );
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str(
        "Companion: <code>COMPLIANCE_MCorp_2026_06_05_Proforma_Bencal_SPV1_V2.html</code></p>\n",
    );

    s.push_str("<h2>Ownership Structure</h2>\n");
    s.push_str(&format!("<p>Bencal SPV1 holds 300,000 WCP common shares (3.0% of 10M outstanding) — 150K purchased Y0 at $20 + 150K founding-bonus shares from Strategic Partner. Y4 WCP listing event triggers Capital Return Sale: sells 150K shares at engine WCP Y4 book value per share, distributes proceeds pro-rata to {} investors. Y10 retains 150K Residual shares at engine WCP Y10 value.</p>\n",
                        fmt_int(bencal_spv1_proforma::SPV1_INVESTOR_SHARES)));

    s.push_str(&render_spv1_summary_section(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json_spv1() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    serde_json::to_string_pretty(&bencal_spv1_proforma::forecast_json(&wcp))
        .expect("SPV1 JSON serialization failed")
}

// ─── SPV2 renderers ─────────────────────────────────────────────────────────

pub fn render_proforma_spv2() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let years = bencal_spv2_proforma::forecast(&pclp1, &wcp);

    let mut s = String::new();
    s.push_str(&head_with_title("Bencal SPV2 — Proforma V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Special Purpose 2 (GP + LP) — Proforma V2</h1>\n");
    s.push_str("<p>Engine-generated proforma from BRIEF v0.15.6 §5d + Flag 15 path b. Consumes PCLP 1 V2 + WCP V1 forecasts.<br>\n");
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str("All amounts CAD — Prepared under IFRS</p>\n");

    s.push_str("<h2>Capital Structure &amp; Investment Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Units/Shares</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Investor LP units</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
        fmt_int(bencal_spv2_proforma::SPV2_INVESTOR_UNITS),
        bencal_spv2_proforma::SPV2_UNIT_PRICE,
        fmt_full_dollar(bencal_spv2_proforma::SPV2_TOTAL_INVESTOR_CAPITAL)
    ));
    s.push_str(&format!("<tr><td class=\"lbl\">Manager LP units (10% diluted)</td><td>{}</td><td>—</td><td>—</td></tr>\n",
                        fmt_int(bencal_spv2_proforma::SPV2_MANAGER_UNITS)));
    s.push_str(&format!("<tr><td class=\"lbl\">PCLP 1 LP units held</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
                        fmt_int(bencal_spv2_proforma::SPV2_PCLP1_UNITS_HELD),
                        bencal_spv2_proforma::SPV2_PCLP1_COST_PER_UNIT,
                        fmt_full_dollar(bencal_spv2_proforma::SPV2_PCLP1_UNITS_HELD * bencal_spv2_proforma::SPV2_PCLP1_COST_PER_UNIT)));
    s.push_str(&format!("<tr><td class=\"lbl\">WCP founding-bonus (6% of 10M; Flag 15 path b)</td><td>{}</td><td>${:.8}</td><td>${:.2}</td></tr>\n",
                        fmt_int(bencal_spv2_proforma::SPV2_WCP_FOUNDING_BONUS),
                        bencal_spv2_proforma::SPV2_WCP_BONUS_NOMINAL,
                        bencal_spv2_proforma::SPV2_WCP_FOUNDING_BONUS * bencal_spv2_proforma::SPV2_WCP_BONUS_NOMINAL));
    s.push_str("</table>\n");

    s.push_str("<h2>10-Year Income Statement &amp; Operating Reserve Drawdown (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    s.push_str(&data_row_s2("Commission rebate (Y0)", &years, |y| {
        y.commission_rebate
    }));
    s.push_str(&data_row_s2("PCLP 1 distributions received", &years, |y| {
        y.pclp1_distributions
    }));
    s.push_str(&data_row_s2(
        "Realised gain — WCP CRS (Y10)",
        &years,
        |y| y.wcp_crs_realised_gain,
    ));
    s.push_str(&data_row_s2(
        "Net change in FV — PCLP 1 units (FVTPL)",
        &years,
        |y| y.pclp1_fv_change,
    ));
    s.push_str(&data_row_s2(
        "Net change in FV — WCP shares (FVTPL)",
        &years,
        |y| y.wcp_fv_change,
    ));
    s.push_str(&data_row_s2("Total investment income", &years, |y| {
        y.total_investment_income
    }));
    s.push_str(&data_row_s2("Setup costs (Y0)", &years, |y| -y.setup_costs));
    s.push_str(&data_row_s2("Annual operating expenses", &years, |y| {
        -y.opex
    }));
    s.push_str(&data_row_s2("Income (loss) before tax", &years, |y| {
        y.income_before_tax
    }));
    s.push_str(&data_row_s2("Tax (27%)", &years, |y| -y.tax));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Net income</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row_s2(
        "Operating expense reserve (closing)",
        &years,
        |y| y.opex_reserve_closing,
    ));
    s.push_str("</table>\n");

    s.push_str(&render_spv2_summary_section(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn render_spv2_summary_section(years: &[BencalSpv2Year]) -> String {
    let y10 = &years[10];
    let total_return = y10.cumulative_cash_to_investors + y10.pclp1_nav_total + y10.wcp_holding_fv;
    let moic = total_return / bencal_spv2_proforma::SPV2_TOTAL_INVESTOR_CAPITAL;
    let per_unit_return = total_return / bencal_spv2_proforma::SPV2_INVESTOR_UNITS;
    let mut s = String::new();
    s.push_str("<h2 style=\"page-break-before:always;break-before:page\">Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per investor unit</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total investor capital (Y0)</td><td>{}</td><td>${:.2}</td></tr>\n",
        fmt_full_dollar(bencal_spv2_proforma::SPV2_TOTAL_INVESTOR_CAPITAL),
        bencal_spv2_proforma::SPV2_UNIT_PRICE
    ));
    s.push_str(&format!("<tr><td class=\"lbl\">Cumulative PCLP 1 distributions Y4–Y10</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.cumulative_cash_to_investors - y10.wcp_crs_realised_gain),
                        fmt_per_share((y10.cumulative_cash_to_investors - y10.wcp_crs_realised_gain) / bencal_spv2_proforma::SPV2_INVESTOR_UNITS)));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 WCP Capital Return Sale dividend</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.wcp_crs_realised_gain),
                        fmt_per_share(y10.wcp_crs_realised_gain / bencal_spv2_proforma::SPV2_INVESTOR_UNITS)));
    s.push_str(&format!("<tr><td class=\"lbl\">Y10 PCLP 1 NAV (remaining position)</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.pclp1_nav_total),
                        fmt_per_share(y10.pclp1_nav_total / bencal_spv2_proforma::SPV2_INVESTOR_UNITS)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 WCP Residual fair value</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.wcp_holding_fv),
        fmt_per_share(y10.wcp_holding_fv / bencal_spv2_proforma::SPV2_INVESTOR_UNITS)
    ));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Total return Y0→Y10</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(total_return), fmt_per_share(per_unit_return)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic,
        per_unit_return / bencal_spv2_proforma::SPV2_UNIT_PRICE
    ));
    s.push_str("</table>\n");
    s.push_str(&render_spv2_annual_returns(years));
    s
}

pub fn render_summary_spv2() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let years = bencal_spv2_proforma::forecast(&pclp1, &wcp);
    let mut s = String::new();
    s.push_str(&head_with_title("Bencal SPV2 — Summary V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Special Purpose 2 — Investor Summary V2</h1>\n");
    s.push_str("<p>Engine-generated summary from BRIEF v0.15.6 §5d + Flag 15 path b.<br>\n");
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str(
        "Companion: <code>COMPLIANCE_MCorp_2026_06_05_Proforma_Bencal_SPV2_V2.html</code></p>\n",
    );
    s.push_str(&format!("<p>Bencal SPV2 is dual-asset: 250,000 PCLP 1 LP units (10% of PCLP 1 fund; $25M cost basis) + 600,000 WCP shares (6% of 10M; founding-bonus from Strategic Partner per Flag 15 path b). PCLP 1 distributions flow pro-rata to {} investors Y4–Y10. Y10 WCP Capital Return Sale (393,824 shares sold) returns additional cash; 206,176 WCP shares retained.</p>\n",
                        fmt_int(bencal_spv2_proforma::SPV2_INVESTOR_UNITS)));
    s.push_str(&render_spv2_summary_section(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json_spv2() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    serde_json::to_string_pretty(&bencal_spv2_proforma::forecast_json(&pclp1, &wcp))
        .expect("SPV2 JSON serialization failed")
}

// ─── Management renderers ───────────────────────────────────────────────────

pub fn render_proforma_mgmt() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let spv1 = bencal_spv1_proforma::forecast(&wcp);
    let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
    let years = bencal_mgmt_proforma::forecast(&spv1, &spv2);

    let mut s = String::new();
    s.push_str(&head_with_title("Bencal Management — Proforma V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Management Corp. — Proforma V2</h1>\n");
    s.push_str("<p>Engine-generated proforma from BRIEF v0.15.6 §5f. Holds 10% manager-tier interests at Bencal SPV1 and SPV2, measured at FVTPL per IFRS 10.27.<br>\n");
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str("All amounts CAD — IFRS 10.27 investment entity (FVTPL holdings)</p>\n");

    s.push_str("<h2>Capital Structure</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Shares</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!("<tr><td class=\"lbl\">Bencal Management Corp. — common shares</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
                        bencal_mgmt_proforma::BM_SHARES_OUTSTANDING as i64,
                        bencal_mgmt_proforma::BM_SHARE_PRICE,
                        fmt_full_dollar(bencal_mgmt_proforma::BM_PAID_IN_CAPITAL)));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Holds 10% manager-tier interests at Bencal SPV1 and SPV2 via dilution mechanics. Economic substance flows from these interests, not from the $10 paid-in share capital. Per-share MOIC will appear mechanically extreme; read alongside the aggregate column.</p>\n");

    s.push_str("<h2>10-Year Income Statement &amp; Operating Reserve Drawdown (CAD)</h2>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str(&year_header());
    let spv1_cash_label = format!(
        "SPV1 capital distribution received ({} shares)",
        fmt_int(bencal_spv1_proforma::SPV1_MANAGER_SHARES)
    );
    let spv2_cash_label = format!(
        "SPV2 distributions received ({} units)",
        fmt_int(bencal_spv2_proforma::SPV2_MANAGER_UNITS)
    );
    s.push_str(&data_row_bm("Commission rebate (Y0)", &years, |y| {
        y.commission_rebate
    }));
    s.push_str(&data_row_bm(&spv1_cash_label, &years, |y| {
        y.spv1_lookthrough_cash
    }));
    s.push_str(&data_row_bm(&spv2_cash_label, &years, |y| {
        y.spv2_lookthrough_cash
    }));
    s.push_str(&data_row_bm(
        "Net change in FV — SPV1 shares (FVTPL)",
        &years,
        |y| y.spv1_lookthrough_fv,
    ));
    s.push_str(&data_row_bm(
        "Net change in FV — SPV2 units (FVTPL)",
        &years,
        |y| y.spv2_lookthrough_fv,
    ));
    s.push_str(&data_row_bm("Total investment income", &years, |y| {
        y.total_investment_income
    }));
    s.push_str(&data_row_bm("Setup costs (Y0)", &years, |y| -y.setup_costs));
    s.push_str(&data_row_bm("Annual operating expenses", &years, |y| {
        -y.opex
    }));
    s.push_str(&data_row_bm("Income (loss) before tax", &years, |y| {
        y.income_before_tax
    }));
    s.push_str(&data_row_bm("Tax (27%)", &years, |y| -y.tax));
    s.push_str("<tr class=\"total\">");
    s.push_str("<td class=\"lbl\">Net income</td>");
    for y in &years {
        s.push_str(&format!("<td>{}</td>", fmt_m(y.net_income)));
    }
    s.push_str("</tr>\n");
    s.push_str(&data_row_bm(
        "Operating expense reserve (closing)",
        &years,
        |y| y.opex_reserve_closing,
    ));
    s.push_str("</table>\n");

    s.push_str(&render_mgmt_summary_section(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

fn render_mgmt_summary_section(years: &[BencalMgmtYear]) -> String {
    let y10 = &years[10];
    let total_return = y10.cumulative_cash + y10.portfolio_nav;
    let moic = total_return / bencal_mgmt_proforma::BM_PAID_IN_CAPITAL;
    let mut s = String::new();
    s.push_str("<h2 style=\"page-break-before:always;break-before:page\">Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per Bencal Management share</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total paid-in capital (Y0)</td><td>{}</td><td>${:.2}</td></tr>\n",
        fmt_full_dollar(bencal_mgmt_proforma::BM_PAID_IN_CAPITAL),
        bencal_mgmt_proforma::BM_SHARE_PRICE
    ));
    s.push_str(&format!("<tr><td class=\"lbl\">Total distributions received Y0–Y10</td><td>{}</td><td>{}</td></tr>\n",
                        fmt_m(y10.cumulative_cash),
                        fmt_full_dollar(y10.cumulative_cash / bencal_mgmt_proforma::BM_SHARES_OUTSTANDING)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 SPV1 shares — fair value</td><td>{}</td><td>—</td></tr>\n",
        fmt_m(y10.spv1_nav_lookthrough)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 SPV2 units — fair value</td><td>{}</td><td>—</td></tr>\n",
        fmt_m(y10.spv2_nav_lookthrough)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 portfolio NAV (combined)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_m(y10.portfolio_nav),
        fmt_full_dollar(y10.portfolio_nav / bencal_mgmt_proforma::BM_SHARES_OUTSTANDING)
    ));
    s.push_str(&format!("<tr class=\"total\"><td class=\"lbl\">Total return Y0→Y10</td><td>{}</td><td>—</td></tr>\n",
                        fmt_m(total_return)));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC (aggregate)</td><td>{}×</td><td>—</td></tr>\n",
        fmt_int(moic)
    ));
    s.push_str("</table>\n");
    s.push_str(&render_mgmt_annual_returns(years));
    s.push_str("<p class=\"note\">Per BRIEF §5f Block F: per-share MOIC is mechanically very high because Bencal Management's paid-in capital is nominal ($10 total against multi-million-dollar manager-tier interests). Read alongside aggregate. 10/90 manager/investor dilution at SPV1 + SPV2 per §5d–§5e.</p>\n");
    s
}

pub fn render_summary_mgmt() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let spv1 = bencal_spv1_proforma::forecast(&wcp);
    let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
    let years = bencal_mgmt_proforma::forecast(&spv1, &spv2);
    let mut s = String::new();
    s.push_str(&head_with_title("Bencal Management — Summary V2"));
    s.push_str("<body>\n");
    s.push_str("<h1>Bencal Management Corp. — Investor Summary V2</h1>\n");
    s.push_str("<p>Engine-generated summary from BRIEF v0.15.6 §5f. Holds 10% manager-tier interests at Bencal SPV1 and SPV2, measured at FVTPL per IFRS 10.27.<br>\n");
    s.push_str("DRAFT — 2026-06-05 — V2<br>\n");
    s.push_str("Companion: <code>COMPLIANCE_MCorp_2026_06_05_Proforma_Bencal_Management_V2.html</code></p>\n");
    s.push_str("<p>Bencal Management Corp. is the manager-tier holding entity. Two nominal $5 shares carry $10 of paid-in capital; economic substance flows from 10% manager-tier interests at Bencal SPV1 and SPV2 (dilution mechanics — not paid cash). Total implied WCP exposure: 90,000 shares (0.9% of 10M outstanding); plus 25,000 PCLP 1 LP units (manager-tier allocation).</p>\n");
    s.push_str(&render_mgmt_summary_section(&years));
    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

pub fn render_json_mgmt() -> String {
    let pclp1 = pclp1_proforma::forecast();
    let wcp = wcp_proforma::forecast(&pclp1);
    let spv1 = bencal_spv1_proforma::forecast(&wcp);
    let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
    serde_json::to_string_pretty(&bencal_mgmt_proforma::forecast_json(&spv1, &spv2))
        .expect("Mgmt JSON serialization failed")
}

// ─── Data row helpers (per-entity year type) ────────────────────────────────

fn data_row<F: Fn(&BencalSpv1Year) -> f64>(
    label: &str,
    years: &[BencalSpv1Year],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_s2<F: Fn(&BencalSpv2Year) -> f64>(
    label: &str,
    years: &[BencalSpv2Year],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn data_row_bm<F: Fn(&BencalMgmtYear) -> f64>(
    label: &str,
    years: &[BencalMgmtYear],
    pick: F,
) -> String {
    let mut s = format!("<tr><td class=\"lbl\">{}</td>", label);
    for y in years {
        s.push_str(&format!("<td>{}</td>", fmt_m(pick(y))));
    }
    s.push_str("</tr>\n");
    s
}

fn render_spv1_annual_returns(years: &[BencalSpv1Year]) -> String {
    let mut s = String::new();
    s.push_str("<h3>Annual Returns — per investor share</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");
    s.push_str("<tr><td class=\"lbl\">Cash distribution per investor share</td>");
    for yr in years {
        if yr.crs_cash_to_investors > 0.0 {
            s.push_str(&format!(
                "<td>{}</td>",
                fmt_per_share(
                    yr.crs_cash_to_investors / bencal_spv1_proforma::SPV1_INVESTOR_SHARES
                )
            ));
        } else {
            s.push_str("<td>—</td>");
        }
    }
    s.push_str("</tr>\n");
    s.push_str("<tr><td class=\"lbl\">NAV per investor share</td>");
    for yr in years {
        let nav = yr.wcp_holding_fv / bencal_spv1_proforma::SPV1_INVESTOR_SHARES;
        s.push_str(&format!("<td>{}</td>", fmt_per_share(nav)));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_spv2_annual_returns(years: &[BencalSpv2Year]) -> String {
    let mut s = String::new();
    s.push_str("<h3>Annual Returns — per LP unit</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");
    s.push_str("<tr><td class=\"lbl\">Cash distribution per LP unit</td>");
    for yr in years {
        if yr.cash_to_investors.abs() < 1e-2 {
            s.push_str("<td>—</td>");
        } else {
            s.push_str(&format!(
                "<td>{}</td>",
                fmt_per_share(yr.cash_to_investors / bencal_spv2_proforma::SPV2_INVESTOR_UNITS)
            ));
        }
    }
    s.push_str("</tr>\n");
    s.push_str("<tr><td class=\"lbl\">NAV per LP unit</td>");
    for yr in years {
        s.push_str(&format!(
            "<td>{}</td>",
            fmt_per_share(yr.y_end_position_per_unit)
        ));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

fn render_mgmt_annual_returns(years: &[BencalMgmtYear]) -> String {
    let mut s = String::new();
    s.push_str("<h3>Annual Returns — Bencal Management Corp.</h3>\n");
    s.push_str("<table class=\"wide\">\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th>");
    for y in 0..=10 {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");
    // Cash distributions per Bencal Management share
    s.push_str("<tr><td class=\"lbl\">Cash distributions per Bencal Management share</td>");
    let mut prev_cum: f64 = 0.0;
    for yr in years {
        let period_cash = yr.cumulative_cash - prev_cum;
        if period_cash.abs() < 1e-2 {
            s.push_str("<td>—</td>");
        } else {
            s.push_str(&format!(
                "<td>{}</td>",
                fmt_full_dollar(period_cash / bencal_mgmt_proforma::BM_SHARES_OUTSTANDING)
            ));
        }
        prev_cum = yr.cumulative_cash;
    }
    s.push_str("</tr>\n");
    // Portfolio NAV per Bencal Management share
    s.push_str("<tr><td class=\"lbl\">Portfolio NAV per Bencal Management share</td>");
    for yr in years {
        s.push_str(&format!(
            "<td>{}</td>",
            fmt_full_dollar(yr.portfolio_nav / bencal_mgmt_proforma::BM_SHARES_OUTSTANDING)
        ));
    }
    s.push_str("</tr>\n");
    s.push_str("</table>\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_six_renderers_well_formed() {
        assert!(render_proforma_spv1().starts_with("<!DOCTYPE"));
        assert!(render_summary_spv1().starts_with("<!DOCTYPE"));
        assert!(render_proforma_spv2().starts_with("<!DOCTYPE"));
        assert!(render_summary_spv2().starts_with("<!DOCTYPE"));
        assert!(render_proforma_mgmt().starts_with("<!DOCTYPE"));
        assert!(render_summary_mgmt().starts_with("<!DOCTYPE"));
    }

    #[test]
    fn json_dumps_parse() {
        let p1 = render_json_spv1();
        let p2 = render_json_spv2();
        let p3 = render_json_mgmt();
        let _: serde_json::Value = serde_json::from_str(&p1).unwrap();
        let _: serde_json::Value = serde_json::from_str(&p2).unwrap();
        let _: serde_json::Value = serde_json::from_str(&p3).unwrap();
    }
}
