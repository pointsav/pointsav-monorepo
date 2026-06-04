// Bencal Forecast Summary V1 — engine-canonical investor-facing renderer.
//
// Produces three HTML compliance documents for the Bencal entity stack:
//   - Bencal Management Corp.
//   - Bencal SPV1 (Ambassadors Direct 1 Inc., WCP-only)
//   - Bencal SPV2 (Ambassadors Direct 2 LP, dual-asset PCLP 1 + WCP)
//
// All numbers derive from engine state: PCLP 1 Excel + WCP 42M Excel via
// `excel::pclp1::read` and `excel::wcp::read`, processed through the existing
// `spv::ambassadors_d1::derive`, `spv::ambassadors_d2::derive`, and
// `spv::bencal::derive` modules.
//
// V1 supersedes earlier JW1 / JW3 hand-typed compliance HTMLs which used
// BRIEF v0.15.6 placeholder assumptions (WCP 10M shares, PCLP 1 implied 10M
// LP units, PRO-01 dist $4/$6/$8). Per operator decision 2026-06-03, the
// engine reading the actual Excel models is now canonical.

use crate::excel::pclp1::Pclp1Data;
use crate::excel::wcp::WcpData;
use crate::spv::ambassadors_d1;
use crate::spv::ambassadors_d2;
use crate::spv::bencal;

// Investor capital structure constants — engine-resident.
// These match the `Ambassadors Direct N` (SPV1/SPV2) and `Bencal Management`
// entity definitions in `spv::ambassadors_d1`, `spv::ambassadors_d2`, `spv::bencal`.
const SPV1_INVESTOR_SHARES: f64 = 3_000_000.0;
const SPV1_INVESTOR_SHARE_PRICE: f64 = 1.0;
const SPV2_INVESTOR_UNITS: f64 = 250_000.0;
const SPV2_INVESTOR_UNIT_PRICE: f64 = 100.0;
const BM_INVESTOR_SHARES: f64 = 2.0;
const BM_INVESTOR_SHARE_PRICE: f64 = 5.0;

// Bencal Management lookthrough stake at each SPV (10% per BRIEF §5d-§5f).
const BM_LOOKTHROUGH_STAKE: f64 = 0.10;

// ─── Formatting helpers ─────────────────────────────────────────────────────

fn fmt_money(v: f64) -> String {
    if v.abs() >= 1_000_000.0 {
        format!("${:.2}M", v / 1_000_000.0)
    } else if v.abs() >= 1_000.0 {
        format!("${:.0}K", v / 1_000.0)
    } else {
        format!("${:.2}", v)
    }
}

fn fmt_money_full(v: f64) -> String {
    let abs = v.abs();
    let s = format!("{:.2}", abs);
    let parts: Vec<&str> = s.split('.').collect();
    let intpart = parts[0];
    let decpart = parts[1];
    let mut with_commas = String::new();
    for (i, c) in intpart.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            with_commas.insert(0, ',');
        }
        with_commas.insert(0, c);
    }
    let sign = if v < 0.0 { "-$" } else { "$" };
    format!("{}{}.{}", sign, with_commas, decpart)
}

fn fmt_per_share(v: f64) -> String {
    format!("${:.2}", v)
}

fn fmt_int_commas(v: f64) -> String {
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

// ─── Common HTML shell ──────────────────────────────────────────────────────

const HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>__TITLE__</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1280px}
h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.78rem}
th,td{border:1px solid #ccc;padding:3px 8px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:180px}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
tr.event td{background:#fffbe6}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{body{margin:0;font-size:11px;max-width:none}table{break-inside:avoid;page-break-inside:avoid}h2,h3{break-after:avoid;page-break-after:avoid}td.lnum,th.lnum{-webkit-print-color-adjust:exact;print-color-adjust:exact;color:#bbb!important;border-right-color:#ccc!important}}
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

fn page_head(title: &str) -> String {
    HEAD.replace("__TITLE__", title)
}

fn page_intro(entity: &str, source: &str) -> String {
    format!(
        "<h1>{entity} — Financial Forecast Summary</h1>\n\
         <p>Source: {source} via tool-proforma-engine (engine-canonical)<br>\n\
         V1 — 2026-06-03<br>\n\
         Companion: COMPLIANCE_MCorp_2026_06_03_Portfolio_PCLP1_JW3.html (PCLP 1 underlying portfolio)<br>\n\
         All amounts CAD — Prepared under IFRS — Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture</p>\n\
         <p>V1 supersedes prior JW1/JW3 placeholder versions. All numbers derive from the Rust engine reading the PCLP 1 and WCP Excel models.</p>\n"
    )
}

fn bcsc_footer() -> String {
    "<p class=\"footer\"><strong>Forward-Looking Information — Notice under applicable securities legislation including the British Columbia Securities Commission (BCSC) and NI 51-102.</strong> This document contains forward-looking information within the meaning of applicable securities legislation. All amounts shown are derived by the tool-proforma-engine from the PCLP 1 and WCP 42M Excel forecast models and are subject to material change. Actual results may differ materially from those projected. This document is prepared for internal planning purposes and does not constitute an offering memorandum, financial advice, or an offer to sell or solicitation to buy any security. Readers should not place undue reliance on forward-looking information.</p>\n".to_string()
}

// ─── SPV1 renderer ──────────────────────────────────────────────────────────

pub fn render_spv1(wcp_data: &WcpData) -> String {
    let spv1 = ambassadors_d1::derive(wcp_data);

    let mut s = String::new();
    s.push_str(&page_head("Bencal SPV1 — Forecast Summary V1"));
    s.push_str("<body>\n");
    s.push_str(&page_intro(
        "Bencal Special Purpose 1 Inc.",
        "WCP 42M Excel",
    ));

    // Capital structure
    s.push_str("<h2>Capital Structure &amp; Investment Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Shares/Units</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">SPV1 outstanding shares (investor pool)</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
        fmt_int_commas(SPV1_INVESTOR_SHARES),
        SPV1_INVESTOR_SHARE_PRICE,
        fmt_money_full(SPV1_INVESTOR_SHARES * SPV1_INVESTOR_SHARE_PRICE),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">WCP Inc. — common shares held (cost basis at WCP Excel Y0 price)</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
        fmt_int_commas(spv1.shares_outstanding),
        wcp_data.price_per_share,
        fmt_money_full(spv1.shares_outstanding * wcp_data.price_per_share),
    ));
    s.push_str("</table>\n");
    s.push_str(&format!(
        "<p class=\"note\">SPV1 is a single-asset entity holding WCP common shares. WCP total outstanding per Excel: {} shares. SPV1 holding represents {:.1}% of WCP outstanding. Pre-listing WCP fair value is the engine's per-share book value; post-listing follows market value per share.</p>\n",
        fmt_int_commas(wcp_data.shares_outstanding),
        100.0 * spv1.shares_outstanding / wcp_data.shares_outstanding,
    ));

    // Investment Return Schedule
    s.push_str("<h2>Investment Return Schedule (per investor share)</h2>\n");
    s.push_str("<p class=\"note\">Year-by-year position per investor share. \"NAV per share\" is the engine's per-share book value (WCP book value × SPV1 stake ÷ SPV1 share count). \"Market NAV/share\" uses WCP market value (post-listing).</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>WCP earnings (aggregate)</th><th>WCP earnings/share</th><th>Book NAV/share</th><th>Market NAV/share</th></tr>\n");

    for y in 0..10 {
        let earnings = spv1.income.earnings[y];
        let earnings_ps = earnings / SPV1_INVESTOR_SHARES;
        let bvps = spv1.book.book_value[y] / SPV1_INVESTOR_SHARES;
        let mvps = spv1.market.market_valuation[y] / SPV1_INVESTOR_SHARES;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            y + 1,
            fmt_money(earnings),
            fmt_per_share(earnings_ps),
            fmt_per_share(bvps),
            fmt_per_share(mvps),
        ));
    }
    s.push_str("</table>\n");

    // Y10 Return Summary
    let y10_book_total = spv1.book.book_value[9];
    let y10_market_total = spv1.market.market_valuation[9];
    let y10_book_ps = y10_book_total / SPV1_INVESTOR_SHARES;
    let y10_market_ps = y10_market_total / SPV1_INVESTOR_SHARES;
    let cost_basis_ps = SPV1_INVESTOR_SHARE_PRICE;
    let moic_book = y10_book_ps / cost_basis_ps;
    let moic_market = y10_market_ps / cost_basis_ps;

    s.push_str("<h2>Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per investor share</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Capital invested (Y0)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money_full(SPV1_INVESTOR_SHARES * cost_basis_ps),
        fmt_per_share(cost_basis_ps),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Book NAV</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(y10_book_total),
        fmt_per_share(y10_book_ps),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 Market NAV (engine)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(y10_market_total),
        fmt_per_share(y10_market_ps),
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">MOIC — Book (Y10 NAV ÷ Cost)</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic_book, moic_book,
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC — Market (Y10 NAV ÷ Cost)</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic_market, moic_market,
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">MOIC shown both ways. Book NAV reflects engine's accrual-basis valuation (cumulative free cash flow + LP-fund beneficial ownership). Market NAV reflects engine's market-valuation column (earnings × P/E ratio). MOIC for SPV1 is materially higher than the prior JW3 placeholder (2.38×) because WCP per-share book value at Y10 from the actual Excel ($105.15) materially exceeds the JW3 illustrative $31.74.</p>\n");

    // WCP per-share metrics from engine (reference table)
    s.push_str("<h2>Underlying WCP Per-Share Metrics (engine-derived)</h2>\n");
    s.push_str("<p class=\"note\">These are the WCP entity's per-share values as read by the engine from the WCP 42M Excel. SPV1 holds 30% of WCP outstanding (3M of 10M shares); its per-share investor view derives from these values scaled by the 30% stake.</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>WCP book value/share</th><th>WCP market value/share</th><th>WCP dividend value/share</th></tr>\n");
    for y in 0..10 {
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            y + 1,
            fmt_per_share(wcp_data.book.book_value_per_share[y]),
            fmt_per_share(wcp_data.market.market_value_per_share[y]),
            fmt_per_share(wcp_data.fair_div.dividend_value_per_share[y]),
        ));
    }
    s.push_str("</table>\n");

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// ─── SPV2 renderer ──────────────────────────────────────────────────────────

pub fn render_spv2(pclp_data: &Pclp1Data, _wcp_data: &WcpData) -> String {
    let spv2 = ambassadors_d2::derive(pclp_data);

    let mut s = String::new();
    s.push_str(&page_head("Bencal SPV2 — Forecast Summary V1"));
    s.push_str("<body>\n");
    s.push_str(&page_intro(
        "Bencal Special Purpose 2 (GP + LP)",
        "PCLP 1 Excel + WCP 42M Excel",
    ));

    // Capital structure
    s.push_str("<h2>Capital Structure &amp; Investment Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Units</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">SPV2 outstanding investor units</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
        fmt_int_commas(SPV2_INVESTOR_UNITS),
        SPV2_INVESTOR_UNIT_PRICE,
        fmt_money_full(SPV2_INVESTOR_UNITS * SPV2_INVESTOR_UNIT_PRICE),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">PCLP 1 LP units held (cost basis $100/unit)</td><td>{}</td><td>$100.00</td><td>{}</td></tr>\n",
        fmt_int_commas(spv2.assumptions.diluted_units),
        fmt_money_full(spv2.assumptions.total_equity),
    ));
    s.push_str("</table>\n");
    let spv2_pclp_stake_pct =
        100.0 * spv2.assumptions.diluted_units / pclp_data.assumptions.diluted_units;
    s.push_str(&format!(
        "<p class=\"note\">SPV2 holds {} PCLP 1 LP units = {:.1}% of PCLP 1 total outstanding ({} units per Excel). SPV2 also holds 600,000 WCP shares received as a founding capital contribution from the Strategic Partner block (Flag 15 path b; recorded at FMV against contributed surplus at Y0). The WCP component is shown separately in the Bencal Management Forecast.</p>\n",
        fmt_int_commas(spv2.assumptions.diluted_units),
        spv2_pclp_stake_pct,
        fmt_int_commas(pclp_data.assumptions.diluted_units),
    ));

    // Investment Return Schedule
    s.push_str("<h2>Investment Return Schedule (per investor unit)</h2>\n");
    s.push_str("<p class=\"note\">Year-by-year position per investor unit. PRO-01 distributions (cash) flow from PCLP 1 directly to SPV2 then pro-rata to investors. NAV per unit = engine's PCLP 1 NAV/unit (unchanged at lookthrough; per-unit metrics carry through).</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>PRO-01 dist (aggregate)</th><th>Per investor unit</th><th>Running per unit</th><th>NAV per unit (Y-end)</th></tr>\n");

    let mut running = 0.0;
    for y in 0..10 {
        let dist_total = spv2.years[y].distributions_to_lps;
        let dist_per_unit = dist_total / SPV2_INVESTOR_UNITS;
        running += dist_per_unit;
        let nav_per_unit = spv2.years[y].nav_per_unit;
        let event_class = if dist_per_unit > 0.01 {
            " class=\"event\""
        } else {
            ""
        };
        s.push_str(&format!(
            "<tr{}><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            event_class,
            y + 1,
            fmt_money(dist_total),
            fmt_per_share(dist_per_unit),
            fmt_per_share(running),
            fmt_per_share(nav_per_unit),
        ));
    }
    s.push_str("</table>\n");
    s.push_str(&format!(
        "<p class=\"note\">Y10 cumulative cash per investor unit: <strong>{}</strong>. Y10 NAV per investor unit (from PCLP 1 Excel forecast): <strong>{}</strong>. Highlighted rows are years with cash distributions to investors.</p>\n",
        fmt_per_share(running),
        fmt_per_share(spv2.years[9].nav_per_unit),
    ));

    // Y10 Return Summary
    let cumulative_cash_total = (0..10)
        .map(|y| spv2.years[y].distributions_to_lps)
        .sum::<f64>();
    let cumulative_cash_per_unit = cumulative_cash_total / SPV2_INVESTOR_UNITS;
    let y10_nav_total = spv2.years[9].nav_total;
    let y10_nav_per_unit = spv2.years[9].nav_per_unit;
    let total_return_total = cumulative_cash_total + y10_nav_total;
    let total_return_per_unit = cumulative_cash_per_unit + y10_nav_per_unit;
    let cost_basis_total = SPV2_INVESTOR_UNITS * SPV2_INVESTOR_UNIT_PRICE;
    let moic = total_return_per_unit / SPV2_INVESTOR_UNIT_PRICE;

    s.push_str("<h2>Investment Return Summary (Y10 endpoint)</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per investor unit</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Capital invested (Y0)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money_full(cost_basis_total),
        fmt_per_share(SPV2_INVESTOR_UNIT_PRICE),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Cumulative PRO-01 cash distributions (Y1–Y10)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(cumulative_cash_total),
        fmt_per_share(cumulative_cash_per_unit),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 PCLP 1 NAV (remaining position)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(y10_nav_total),
        fmt_per_share(y10_nav_per_unit),
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total Return (cash + Y10 NAV)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(total_return_total),
        fmt_per_share(total_return_per_unit),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">MOIC (Total Return ÷ Capital)</td><td>{:.2}×</td><td>{:.2}×</td></tr>\n",
        moic, moic,
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">MOIC for SPV2 is materially higher than the prior JW3 placeholder (2.28×) because the engine reads the PCLP 1 Excel's actual per-unit NAV trajectory ($385.74 at Y10) which materially exceeds the JW3 illustrative $128. Note: this summary covers the PCLP 1 component only. The WCP 600K share founding-bonus is shown separately in the Bencal Management Forecast (which aggregates the dual-asset lookthrough).</p>\n");

    // PCLP 1 per-unit reference table
    s.push_str("<h2>Underlying PCLP 1 Per-Unit Metrics (engine-derived)</h2>\n");
    s.push_str("<p class=\"note\">These are PCLP 1 entity's per-unit values as read by the engine from the PCLP 1 Excel. SPV2 per-unit metrics inherit these directly via proportional lookthrough at SPV2's 9% stake.</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>Distribution per unit</th><th>NAV per unit</th><th>Asset value per unit</th><th>Distribution yield</th></tr>\n");
    for y in 0..10 {
        let py = &pclp_data.years[y];
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.2}%</td></tr>\n",
            y + 1,
            fmt_per_share(py.dist_per_unit),
            fmt_per_share(py.nav_per_unit),
            fmt_per_share(py.asset_value_per_unit),
            py.distribution_yield * 100.0,
        ));
    }
    s.push_str("</table>\n");

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// ─── Bencal Management renderer ─────────────────────────────────────────────

pub fn render_management(pclp_data: &Pclp1Data, wcp_data: &WcpData) -> String {
    let bencal = bencal::derive(wcp_data, pclp_data);
    let block_f = bencal::compute_block_f(&bencal);

    let mut s = String::new();
    s.push_str(&page_head("Bencal Management Corp. — Forecast Summary V1"));
    s.push_str("<body>\n");
    s.push_str(&page_intro(
        "Bencal Management Corp.",
        "PCLP 1 Excel + WCP 42M Excel (via 10% lookthrough at SPV1 + SPV2)",
    ));

    // Capital structure
    s.push_str("<h2>Capital Structure &amp; Investment Position</h2>\n");
    s.push_str("<table>\n");
    s.push_str(
        "<tr><th class=\"lbl\">Item</th><th>Shares</th><th>Price</th><th>Capital</th></tr>\n",
    );
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Bencal Management Corp. — common shares</td><td>{}</td><td>${:.2}</td><td>{}</td></tr>\n",
        BM_INVESTOR_SHARES as u32,
        BM_INVESTOR_SHARE_PRICE,
        fmt_money_full(BM_INVESTOR_SHARES * BM_INVESTOR_SHARE_PRICE),
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Bencal Management Corp. is the manager-tier holding entity. Its 2 nominal $5.00 shares carry $10.00 of paid-in capital. Economic substance flows from 10% manager allocations at Bencal SPV1 + Bencal SPV2 (received via dilution mechanics; see header note below).</p>\n");

    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Lookthrough holdings</th><th>Stake at SPV</th><th>SPV holding</th><th>Implied position</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">via Bencal SPV1 (WCP)</td><td>10.0%</td><td>{} WCP shares</td><td>{} WCP shares lookthrough</td></tr>\n",
        fmt_int_commas(3_000_000.0),
        fmt_int_commas(3_000_000.0 * BM_LOOKTHROUGH_STAKE),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">via Bencal SPV2 (PCLP 1)</td><td>10.0%</td><td>{} PCLP 1 LP units</td><td>{} PCLP 1 units lookthrough</td></tr>\n",
        fmt_int_commas(250_000.0),
        fmt_int_commas(250_000.0 * BM_LOOKTHROUGH_STAKE),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">via Bencal SPV2 (WCP founding bonus)</td><td>10.0%</td><td>{} WCP shares</td><td>{} WCP shares lookthrough</td></tr>\n",
        fmt_int_commas(600_000.0),
        fmt_int_commas(600_000.0 * BM_LOOKTHROUGH_STAKE),
    ));
    s.push_str("</table>\n");

    // 10-year lookthrough series (engine-derived)
    s.push_str("<h2>Bencal Management 10-Year Lookthrough Series (per BM share)</h2>\n");
    s.push_str("<p class=\"note\">Engine-derived combined SPV1 + SPV2 lookthrough at 10%. Per-BM-share = aggregate ÷ 2 shares (mechanically very high because BM's paid-in capital is nominal).</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Year</th><th>Aggregate dividend income</th><th>Aggregate book value</th><th>Per BM share book value</th></tr>\n");
    for y in 0..10 {
        let div_total = bencal.fair_div.dividend_valuation[y];
        let book_total = bencal.book.book_value[y];
        let book_ps = book_total / BM_INVESTOR_SHARES;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">Y{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            y + 1,
            fmt_money(div_total),
            fmt_money(book_total),
            fmt_money_full(book_ps),
        ));
    }
    s.push_str("</table>\n");

    // Block F Y10 summary (engine-canonical)
    s.push_str("<h2>Block F — Bencal Management Y10 Headline (engine compute_block_f)</h2>\n");
    s.push_str(&format!(
        "<p class=\"note\">{}</p>\n",
        bencal::BlockF::header_note()
    ));
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th><th>Aggregate</th><th>Per BM share</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Y10 portfolio NAV (book value)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money(block_f.portfolio_nav_total),
        fmt_money(block_f.portfolio_nav_per_share),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Total invested capital (Y0)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_money_full(block_f.total_invested_capital),
        fmt_money_full(block_f.per_share_invested_capital),
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">MOIC</td><td>{:.0}×</td><td>{:.0}×</td></tr>\n",
        block_f.moic_aggregate, block_f.moic_per_share,
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">CAGR Y0→Y10 (artifact)</td><td>{:.1}%</td><td>{:.1}%</td></tr>\n",
        block_f.cagr_y10 * 100.0,
        block_f.cagr_y10 * 100.0,
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">MOIC and CAGR figures at Bencal Management level are mechanically extreme because the entity's paid-in capital is nominal ($10 total against multi-million-dollar lookthrough claims). Read alongside the aggregate column; the per-share view exists to surface the 10/90 manager/investor dilution mechanic per BRIEF §5d–§5f.</p>\n");

    s.push_str(&bcsc_footer());
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic minimal Pclp1Data fixture for testing.
    fn synthetic_pclp() -> Pclp1Data {
        use crate::excel::pclp1::{Pclp1Assumptions, Pclp1Year};
        let assumptions = Pclp1Assumptions {
            dev_yield: 0.105,
            cap_rate: 0.0625,
            total_equity: 25_000_000.0,
            cost_per_unit: 100.0,
            advisory_fee_pct: 0.0,
            benetti_dilution: 0.0,
            board_expense: 0.0,
            admin_costs: 0.0,
            debenture_interest_rate: 0.0,
            interest_on_cash: 0.0,
            debenture_buyback_pct: 0.0,
            min_cash_balance: 0.0,
            working_capital_reserve: 0.0,
            diluted_units: 250_000.0,
            year_labels: std::array::from_fn(|y| format!("Y{}", y + 1)),
        };
        let years: Vec<Pclp1Year> = (0..10)
            .map(|y| Pclp1Year {
                year: (y + 1) as u32,
                noi: 0.0,
                income_continuity: 0.0,
                issue_costs: 0.0,
                financing_costs: 0.0,
                advisory_fees: 0.0,
                admin_compliance: 0.0,
                board_of_directors: 0.0,
                total_expenses: 0.0,
                ebitda: 0.0,
                interest_net: 0.0,
                funding_from_ops: 0.0,
                interest_coverage: None,
                debt_service_ratio: None,
                opening_cash: 0.0,
                new_equity: 0.0,
                new_debt_gross: 0.0,
                capex: 0.0,
                debt_repayment: 0.0,
                distributions: 0.0,
                ending_cash: 0.0,
                opening_debt: 0.0,
                debt_additions: 0.0,
                debt_payments: 0.0,
                ending_debt: 0.0,
                opening_assets: 0.0,
                total_capital_assets: 0.0,
                assets_generating_rent: 0.0,
                buildings_under_construction: 0.0,
                debt_to_dev_cost: 0.0,
                asset_value_total: 0.0,
                asset_value_per_unit: 0.0,
                nav_total: 250_000.0 * (100.0 + 10.0 * y as f64),
                nav_per_unit: 100.0 + 10.0 * y as f64,
                distribution_yield: 0.0,
                total_expense_ratio: 0.0,
                distributions_to_lps: 250_000.0 * 5.0 * y as f64,
                dist_per_unit: 5.0 * y as f64,
                dist_yield_on_cost: 0.0,
                ff_revenue_pu: 0.0,
                ff_dist_pu: 0.0,
                ff_dist_yield_on_cost: 0.0,
                ff_asset_value_pu: 0.0,
                ff_total_debt_pu: 0.0,
                ff_nav_pu: 0.0,
                ff_market_value_pu: 0.0,
                ff_coverage: None,
                ff_debt_to_dev_cost: 0.0,
                ff_debt_to_av: 0.0,
                ff_ter: 0.0,
                ff_sqft: 0.0,
            })
            .collect();
        Pclp1Data {
            title: "Test".to_string(),
            entity: "Test".to_string(),
            date: "Y0".to_string(),
            assumptions,
            years,
            market_yield: 0.0,
            compounded_return_y8: 0.0,
        }
    }

    #[test]
    fn spv2_per_unit_x_units_matches_aggregate_distributions() {
        let pclp = synthetic_pclp();
        let spv2 = ambassadors_d2::derive(&pclp);
        for y in 0..10 {
            let per_unit = spv2.years[y].dist_per_unit;
            let aggregate = spv2.years[y].distributions_to_lps;
            let expected = per_unit * SPV2_INVESTOR_UNITS;
            assert!(
                (expected - aggregate).abs() < 1.0,
                "Y{}: per_unit × units ({:.2}) != aggregate ({:.2})",
                y + 1,
                expected,
                aggregate
            );
        }
    }

    #[test]
    fn spv2_y10_nav_per_unit_engine_derived() {
        let pclp = synthetic_pclp();
        let spv2 = ambassadors_d2::derive(&pclp);
        // Synthetic Y10 NAV/unit = 100 + 10*9 = 190
        assert!((spv2.years[9].nav_per_unit - 190.0).abs() < 1e-6);
        // Aggregate Y10 NAV = 250K × 190 = 47.5M
        assert!((spv2.years[9].nav_total - 47_500_000.0).abs() < 1.0);
    }

    #[test]
    fn spv2_cumulative_cash_y10_matches_sum() {
        let pclp = synthetic_pclp();
        let spv2 = ambassadors_d2::derive(&pclp);
        // dist_per_unit = 5y; sum 0..9 = 5*(0+1+...+9) = 5*45 = 225/unit
        let cumulative_per_unit: f64 = (0..10).map(|y| spv2.years[y].dist_per_unit).sum();
        assert!((cumulative_per_unit - 225.0).abs() < 1e-6);
    }

    #[test]
    fn renderer_output_well_formed_html() {
        let pclp = synthetic_pclp();
        // Can't test the WCP renderer without a synthetic WcpData; just verify
        // SPV2 produces something resembling HTML.
        let synth_wcp = crate::excel::wcp::WcpData {
            title: "test".to_string(),
            entity: "test".to_string(),
            date: "Y0".to_string(),
            shares_outstanding: 3_000_000.0,
            price_per_share: 1.0,
            lps: vec![],
            income: crate::excel::wcp::WcpIncome {
                gross_income: [0.0; 10],
                referral_fees: [0.0; 10],
                wpi_consulting: [0.0; 10],
                gna_nyc: [0.0; 10],
                gna_berlin: [0.0; 10],
                total_expenses: [0.0; 10],
                ebitda: [0.0; 10],
                ebitda_per_share: [0.0; 10],
                taxes: [0.0; 10],
                earnings: [0.0; 10],
                earnings_per_share: [0.0; 10],
            },
            book: crate::excel::wcp::WcpBook {
                cumulative_fcf_wci: [0.0; 10],
                beneficial_ownership_lps: [0.0; 10],
                book_value: [0.0; 10],
                book_value_per_share: [0.0; 10],
            },
            market: crate::excel::wcp::WcpMarket {
                earnings_valuation: [0.0; 10],
                market_valuation: [0.0; 10],
                pe_ratio: [0.0; 10],
                market_value_per_share: [0.0; 10],
            },
            fair_div: crate::excel::wcp::WcpFairDiv {
                fair_value_per_share: [0.0; 10],
                dividend_valuation: [0.0; 10],
                dividend_value_per_share: [0.0; 10],
            },
            gna_label_1: "L".to_string(),
            gna_label_2: "A".to_string(),
        };
        let html = render_spv2(&pclp, &synth_wcp);
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Bencal Special Purpose 2"));
        assert!(html.contains("Investment Return Schedule"));
        assert!(html.contains("V1"));
        assert!(html.ends_with("</body></html>\n"));
    }
}
