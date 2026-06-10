// Legacy JV vs Direct-Hold — two-sided structural tear sheet (V2).
//
// Companion to "Woodfine Direct-Hold Solutions — Sensitivity Analysis" (internally V2): same layout,
// palette and voice, but a TWO-SIDED structural comparison told across SIX sections with focused,
// single-message charts (one chart = one message) instead of one overloaded figure.
//
// Engine-built — every chart series and every figure quoted in the prose comes from
// legacy_jv_proforma::forecast() (D7) and pclp1_proforma::forecast() (D2). MOIC/IRR via
// spv::irr::xirr_annual over the per-year cash-flow vectors. No JS math. The Years 11–20 section is a
// forward illustration projected in this module from the engine Y10 end state (pclp1_proforma is
// hard-coded Y1–10 and consumed by many reports, so it is NOT modified).
//
//   green = Legacy JV (D7)        blue = Direct-Hold (D2)        red = alerts only
//   bars for levels (square footage, cash, MOIC/IRR) · lines for trajectories (leverage, LTV, value, coverage)
//
// Naming rule: full "Woodfine Direct-Hold Solutions" / "Direct-Hold Solutions" on first/formal reference
// per section; "Direct-Hold" shorthand thereafter; chart legends, cards, table rows and scoreboard always
// use the short "Direct-Hold".

use crate::spv::irr::xirr_annual;
use crate::spv::legacy_jv_proforma::{
    self, LegacyJvYear, LEGACY_JV_BANK_DEBT, LEGACY_JV_GROSS_EQUITY, LEGACY_JV_SHARES,
    LEGACY_JV_STABILIZED_AV, LEGACY_JV_TOTAL_SF,
};
use crate::spv::pclp1_proforma::{
    self, Pclp1Year, PCLP1_ADMIN_COMPLIANCE_ANNUAL, PCLP1_ADVISORY_FEE_PCT, PCLP1_BOARD_ANNUAL,
    PCLP1_CAP_RATE, PCLP1_DEBT_RATE_DEBENTURE, PCLP1_DEV_YIELD, PCLP1_DILUTED_UNITS,
    PCLP1_GROSS_EQUITY, PCLP1_INVESTOR_UNITS, PCLP1_TOTAL_PORTFOLIO_SQFT,
};
use serde_json::{json, Value};

const LBL_JV: &str = "Legacy JV";
const LBL_LP: &str = "Direct-Hold";

fn jv_at(v: &[LegacyJvYear], y: u32) -> &LegacyJvYear {
    v.iter()
        .find(|r| r.year == y)
        .expect("legacy JV year present")
}
fn lp_at(v: &[Pclp1Year], y: u32) -> &Pclp1Year {
    v.iter().find(|r| r.year == y).expect("PCLP1 year present")
}

// ─── Cash-flow builders (replicated from report::legacy_jv_proforma for MOIC/IRR) ─────────────

fn legacy_cfs(years: &[LegacyJvYear]) -> Vec<f64> {
    let mut cfs = vec![0.0; 11];
    cfs[0] = -LEGACY_JV_GROSS_EQUITY;
    for y in years {
        if (1..=10).contains(&y.year) {
            cfs[y.year as usize] += y.dividends_to_shareholders;
        }
    }
    cfs[10] += LEGACY_JV_STABILIZED_AV - LEGACY_JV_BANK_DEBT; // Y10 IFRS-FV equity exit
    cfs
}

fn pclp1_cfs(years: &[Pclp1Year]) -> Vec<f64> {
    let inv = PCLP1_INVESTOR_UNITS / PCLP1_DILUTED_UNITS;
    let mut cfs = vec![0.0; 11];
    cfs[0] = -PCLP1_GROSS_EQUITY;
    for y in years {
        if (1..=10).contains(&y.year) {
            cfs[y.year as usize] += y.distributions * inv;
        }
    }
    if let Some(y10) = years.iter().find(|y| y.year == 10) {
        cfs[10] += y10.nav * inv; // investor fraction of Y10 NAV
    }
    cfs
}

// ─── Years 11–20 forward continuation (operator profile: debenture-recycling, compounding) ─────
//
// Seeded from the engine Y10 row; NOT a re-run of pclp1_proforma. Each year the structure recycles a
// share of FFO as development equity and issues MATCHED debentures at a target loan-to-cost, funding a
// measured continued build. Because development yields 10.5% against a 5% coupon, coverage stays well
// above the 1.20× covenant throughout. All constants are tunable.

const EXT_LAST_YEAR: u32 = 20;
const EXT_RECYCLE_FFO_PCT: f64 = 0.50; // share of FFO recycled into new development (rest distributed)
const EXT_NEW_DEV_LTC: f64 = 0.60; // new development funded 60% debentures / 40% recycled equity
const EXT_COST_PER_SF: f64 = 1_211_125_000.0 / 3_906_855.0; // ≈ $310/sf (D2 total dev cost ÷ portfolio sf)

struct ExtYear {
    year: u32,
    sqft: f64,
    debt: f64,
    noi: f64,
    coverage: f64,
    nav_per_unit: f64,
    distributions: f64,
    new_sf: f64,
}

fn ext_opex(noi: f64) -> f64 {
    // Recurring operating expense: advisory (1% × net proceeds) + admin + board.
    noi * PCLP1_ADVISORY_FEE_PCT + PCLP1_ADMIN_COMPLIANCE_ANNUAL + PCLP1_BOARD_ANNUAL
}

fn extend_y11_y20(y10: &Pclp1Year) -> Vec<ExtYear> {
    let mut debt = y10.closing_debt;
    let mut noi = y10.net_proceeds_from_ops; // generating NOI
    let mut sqft = y10.total_sqft_generating;
    let mut out = Vec::with_capacity(10);
    for y in 11..=EXT_LAST_YEAR {
        let ebitda = noi - ext_opex(noi);
        let net_int = debt * PCLP1_DEBT_RATE_DEBENTURE;
        let ffo = ebitda - net_int;
        let coverage = if net_int > 1.0 { ebitda / net_int } else { 0.0 };

        let recycled = (ffo * EXT_RECYCLE_FFO_PCT).max(0.0); // development equity from retained cash
        let dev_cost = recycled / (1.0 - EXT_NEW_DEV_LTC);
        let new_debt = dev_cost * EXT_NEW_DEV_LTC; // matched debentures recycled into the next build
        let new_sf = dev_cost / EXT_COST_PER_SF;
        let new_noi = dev_cost * PCLP1_DEV_YIELD; // generates next year (1-yr lease-up)
        let distributions = ffo * (1.0 - EXT_RECYCLE_FFO_PCT);

        let debt_next = debt + new_debt;
        let sqft_next = sqft + new_sf;
        // Fair value: capitalised generating NOI − debt + this year's development at cost (WIP).
        let nav = noi / PCLP1_CAP_RATE - debt_next + dev_cost;

        out.push(ExtYear {
            year: y,
            sqft: sqft_next,
            debt: debt_next,
            noi,
            coverage,
            nav_per_unit: nav / PCLP1_DILUTED_UNITS,
            distributions,
            new_sf,
        });
        debt = debt_next;
        noi += new_noi;
        sqft = sqft_next;
    }
    out
}

// ─── Engine data assembly ─────────────────────────────────────────────────────

struct Series {
    labels: Vec<String>,
    de_jv: Vec<f64>,
    de_lp: Vec<f64>,
    sqft_jv: Vec<f64>,
    sqft_lp: Vec<f64>,
    ltv_jv: Vec<f64>, // IFRS fair-value LTV (flat ~59.5% Y4+; 0 in construction → null)
    ltv_lp: Vec<f64>,
    dscr_jv: Vec<f64>, // 0 in construction → null
    icr_lp: Vec<f64>,  // 0 in construction → null
    nav_jv: Vec<f64>,  // IFRS-FV equity per share ($100 during construction → ~$204)
    nav_lp: Vec<f64>,  // nav_per_unit ($100 → ~$392)
    div_jv: Vec<f64>,  // aggregate dividends to shareholders ($/yr)
    dist_lp: Vec<f64>, // aggregate distributions ($/yr)
}

fn series() -> Series {
    let jv = legacy_jv_proforma::forecast();
    let lp = pclp1_proforma::forecast();
    let yy: Vec<u32> = (1..=10).collect();
    let g = |f: &dyn Fn(u32) -> f64| yy.iter().map(|&y| f(y)).collect::<Vec<f64>>();
    Series {
        labels: yy.iter().map(|y| format!("Y{y}")).collect(),
        de_jv: g(&|y| jv_at(&jv, y).debt_outstanding / LEGACY_JV_GROSS_EQUITY),
        de_lp: g(&|y| lp_at(&lp, y).closing_debt / PCLP1_GROSS_EQUITY),
        sqft_jv: g(&|y| if y >= 4 { LEGACY_JV_TOTAL_SF } else { 0.0 }),
        sqft_lp: g(&|y| lp_at(&lp, y).total_sqft_generating),
        // IFRS fair-value LTV: fixed $750M bank debt ÷ $1,260M stabilised value = ~59.5% flat (Y4+).
        // (Both structures now on IFRS fair value; the JV's ASPE book LTV — which rose artificially as
        // the depreciating book asset shrank against fixed debt — is no longer used.)
        ltv_jv: g(&|y| {
            if y >= 4 {
                LEGACY_JV_BANK_DEBT / LEGACY_JV_STABILIZED_AV
            } else {
                0.0
            }
        }),
        ltv_lp: g(&|y| lp_at(&lp, y).debt_to_asset_value),
        dscr_jv: g(&|y| jv_at(&jv, y).dscr),
        icr_lp: g(&|y| lp_at(&lp, y).interest_coverage),
        nav_jv: g(&|y| {
            let e = jv_at(&jv, y).equity_value_ifrs_fv;
            if e > 0.0 {
                e / LEGACY_JV_SHARES
            } else {
                100.0 // $100 subscription value held during construction
            }
        }),
        nav_lp: g(&|y| lp_at(&lp, y).nav_per_unit),
        div_jv: g(&|y| jv_at(&jv, y).dividends_to_shareholders),
        dist_lp: g(&|y| lp_at(&lp, y).distributions),
    }
}

struct Figures {
    jv_sf_online_year: u32,
    lp_sf_online_year: u32,
    jv_peak_de: f64,
    lp_peak_de: f64,
    lp_peak_ltv: f64,
    lp_stab_ltv: f64,
    jv_ltv: f64,
    jv_div_start_year: u32,
    jv_cum_div_y10: f64,
    sf_jv_total: f64,
    sf_lp_total: f64,
    sf_delta: f64,
    sf_pct_lp_denom: f64,
    moic_jv: f64,
    moic_lp: f64,
    irr_jv: f64,
    irr_lp: f64,
    lp_nav_y10: f64,
    jv_nav_y10: f64,
    // Years 11–20 continuation endpoints
    y20_sqft: f64,
    y20_coverage: f64,
    y20_nav: f64,
    y20_debt: f64,
    ext_cov_min: f64,
    y20_sf_growth_pct: f64,
}

fn figures(s: &Series) -> Figures {
    let jv = legacy_jv_proforma::forecast();
    let lp = pclp1_proforma::forecast();
    let max = |v: &[f64]| v.iter().cloned().fold(0.0_f64, f64::max);
    let inv = PCLP1_INVESTOR_UNITS / PCLP1_DILUTED_UNITS;

    let jv_online = (1..=10).find(|&y| s.sqft_jv[(y - 1) as usize] >= LEGACY_JV_TOTAL_SF - 1.0);
    let lp_online =
        (1..=10).find(|&y| s.sqft_lp[(y - 1) as usize] >= PCLP1_TOTAL_PORTFOLIO_SQFT - 1.0);
    let jv_div_start = jv
        .iter()
        .find(|r| r.dividends_to_shareholders > 1.0)
        .map(|r| r.year)
        .unwrap_or(4);

    let jv_y10 = jv_at(&jv, 10);
    let lp_y10 = lp_at(&lp, 10);
    let moic_jv =
        (jv_y10.cumulative_dividends + jv_y10.equity_value_ifrs_fv) / LEGACY_JV_GROSS_EQUITY;
    let lp_dist_sum: f64 = lp
        .iter()
        .filter(|y| (1..=10).contains(&y.year))
        .map(|y| y.distributions * inv)
        .sum();
    let moic_lp = (lp_dist_sum + lp_y10.nav * inv) / PCLP1_GROSS_EQUITY;

    let ext = extend_y11_y20(lp_y10);
    let y20 = ext.last().expect("Y20 row");

    Figures {
        jv_sf_online_year: jv_online.unwrap_or(4),
        lp_sf_online_year: lp_online.unwrap_or(8),
        jv_peak_de: max(&s.de_jv),
        lp_peak_de: max(&s.de_lp),
        lp_peak_ltv: lp.iter().map(|r| r.debt_to_asset_value).fold(0.0, f64::max),
        lp_stab_ltv: lp_y10.debt_to_asset_value,
        jv_ltv: LEGACY_JV_BANK_DEBT / LEGACY_JV_STABILIZED_AV,
        jv_div_start_year: jv_div_start,
        jv_cum_div_y10: jv_y10.cumulative_dividends,
        sf_jv_total: LEGACY_JV_TOTAL_SF,
        sf_lp_total: PCLP1_TOTAL_PORTFOLIO_SQFT,
        sf_delta: PCLP1_TOTAL_PORTFOLIO_SQFT - LEGACY_JV_TOTAL_SF,
        sf_pct_lp_denom: (PCLP1_TOTAL_PORTFOLIO_SQFT - LEGACY_JV_TOTAL_SF)
            / PCLP1_TOTAL_PORTFOLIO_SQFT
            * 100.0,
        moic_jv,
        moic_lp,
        irr_jv: xirr_annual(&legacy_cfs(&jv)).unwrap_or(0.0),
        irr_lp: xirr_annual(&pclp1_cfs(&lp)).unwrap_or(0.0),
        lp_nav_y10: lp_y10.nav_per_unit,
        jv_nav_y10: jv_y10.equity_value_ifrs_fv / LEGACY_JV_SHARES,
        y20_sqft: y20.sqft,
        y20_coverage: y20.coverage,
        y20_nav: y20.nav_per_unit,
        y20_debt: y20.debt,
        ext_cov_min: ext.iter().map(|e| e.coverage).fold(f64::INFINITY, f64::min),
        y20_sf_growth_pct: (y20.sqft - PCLP1_TOTAL_PORTFOLIO_SQFT) / PCLP1_TOTAL_PORTFOLIO_SQFT
            * 100.0,
    }
}

fn null_if_zero(v: &[f64]) -> Vec<Value> {
    v.iter()
        .map(|&x| if x > 0.0 { json!(x) } else { Value::Null })
        .collect()
}

fn chart_data_json(s: &Series) -> serde_json::Value {
    let lp = pclp1_proforma::forecast();
    let ext = extend_y11_y20(lp_at(&lp, 10));

    // Continuous Y1..Y20 arrays for the "next decade" section.
    let ext_labels: Vec<String> = (1..=20).map(|y| format!("Y{y}")).collect();
    let mut ext_sqft_lp: Vec<Value> = s.sqft_lp.iter().map(|&v| json!(v)).collect();
    ext_sqft_lp.extend(ext.iter().map(|e| json!(e.sqft)));
    let mut ext_sqft_jv: Vec<Value> = s.sqft_jv.iter().map(|&v| json!(v)).collect();
    ext_sqft_jv.extend((0..10).map(|_| Value::Null)); // JV single-shot — ends at Y10
    let mut ext_icr_lp: Vec<Value> = null_if_zero(&s.icr_lp);
    ext_icr_lp.extend(ext.iter().map(|e| json!(e.coverage)));
    let mut ext_nav_lp: Vec<Value> = s.nav_lp.iter().map(|&v| json!(v)).collect();
    ext_nav_lp.extend(ext.iter().map(|e| json!(e.nav_per_unit)));

    let f = figures(s);
    json!({
        "labels": s.labels,
        "sqftJV": s.sqft_jv, "sqftLP": s.sqft_lp,
        "deJV": s.de_jv, "deLP": s.de_lp,
        "ltvJV": null_if_zero(&s.ltv_jv), "ltvLP": s.ltv_lp,
        "dscrJV": null_if_zero(&s.dscr_jv), "icrLP": null_if_zero(&s.icr_lp),
        "navJV": s.nav_jv, "navLP": s.nav_lp,
        "divJV": s.div_jv, "distLP": s.dist_lp,
        "endpoints": {
            "moicJV": f.moic_jv, "moicLP": f.moic_lp,
            "irrJV": f.irr_jv, "irrLP": f.irr_lp
        },
        "ext": {
            "labels": ext_labels,
            "sqftLP": ext_sqft_lp, "sqftJV": ext_sqft_jv,
            "icrLP": ext_icr_lp, "navLP": ext_nav_lp,
            "extendedFrom": 10
        },
        "labelExact": { "jv": LBL_JV, "lp": LBL_LP }
    })
}

// ─── prose formatters ──────────────────────────────────────────────────────────

fn fmt_int(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.abs().to_string();
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in b.iter().enumerate() {
        if i > 0 && (b.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*c as char);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}
fn fmt_x(v: f64) -> String {
    format!("{v:.1}×")
}
fn fmt_x2(v: f64) -> String {
    format!("{v:.2}×")
}
fn fmt_pct0(v: f64) -> String {
    format!("{:.0}%", v * 100.0)
}
fn fmt_pct1(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}
fn fmt_m(v: f64) -> String {
    format!("{:.2}M", v / 1_000_000.0)
}

// ─── HTML head (Woodfine palette + JW3 print idiom) ────────────────────────────

const HEAD: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Alternative Real Estate — Legacy Joint Ventures vs. Direct-Hold Solutions</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
<style>
:root{
  --ink:#111827; --ink2:#374151; --ink3:#6B7280; --ink4:#9CA3AF;
  --paper:#ffffff; --canvas:#F7F9FA; --panel:#EEF2F6; --border:#E6E7E8; --rule:#E6E7E8;
  --wf-blue:#164679; --wf-green:#54924E; --wf-gold:#F57F17; --wf-orange:#F15F22; --wf-red:#ED1B2F;
  --accent:#54924E; --accent-tint:#EEF6EC; --blue-tint:#E8EFF7; --pos:#54924E;
  --page-w:1180px; --radius:8px;
  --font:ui-sans-serif,system-ui,-apple-system,"Segoe UI",Roboto,sans-serif;
  --mono:ui-monospace,"SF Mono",Menlo,monospace;
}
*{box-sizing:border-box;margin:0;padding:0;}
body{font-family:var(--font);font-size:13px;background:var(--canvas);color:var(--ink);line-height:1.55;}

.appbar{background:var(--wf-blue);color:#fff;padding:12px 28px;display:flex;align-items:center;gap:20px;position:sticky;top:0;z-index:30;border-bottom:3px solid var(--wf-gold);}
.appbar-title{font-size:15px;font-weight:600;white-space:nowrap;}
.appbar-meta{font-size:11.5px;color:#cbd5e1;}
.print-now{margin-left:auto;padding:7px 16px;border:1.5px solid #fff;border-radius:6px;background:transparent;color:#fff;font-size:12px;font-weight:600;cursor:pointer;font-family:var(--font);}
.print-now:hover{background:#fff;color:var(--wf-blue);}
.report-title,.pg-runhead{display:none;}

.keymsg{max-width:var(--page-w);margin:18px auto 4px;padding:0 28px;}
.keymsg-h{font-size:13px;font-weight:700;color:var(--ink);margin-bottom:10px;}
.keymsg-grid{display:grid;grid-template-columns:repeat(3,1fr);gap:14px;}
.km{background:var(--paper);border:1px solid var(--border);border-left:3px solid var(--wf-gold);border-radius:0 var(--radius) var(--radius) 0;padding:13px 16px;}
.km.jv{border-left-color:var(--wf-green);} .km.lp{border-left-color:var(--wf-blue);}
.km .kt{display:block;font-size:12px;color:var(--ink);margin-bottom:6px;}
.km p{font-size:11.5px;color:var(--ink2);line-height:1.55;}
.km b{color:var(--ink);}

.chapter{display:block;max-width:var(--page-w);margin:18px auto;padding:0 28px;}
.chapter-head{background:var(--paper);border:1px solid var(--border);border-radius:var(--radius);padding:16px 22px;margin-bottom:14px;}
.ch-mast{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.09em;color:var(--wf-green);}
.ch-sec{font-size:18px;font-weight:700;color:var(--ink);margin-top:3px;}
.ch-term{font-size:12px;color:var(--ink2);margin-top:1px;}
.basis{font-size:11px;color:var(--ink3);margin-top:9px;line-height:1.5;border-top:1px solid var(--rule);padding-top:8px;}
.basis b{color:var(--ink2);}
.lead{font-size:12.5px;color:var(--ink2);line-height:1.6;margin-bottom:14px;}

.cards{display:grid;grid-template-columns:repeat(5,1fr);gap:12px;margin-bottom:14px;}
.cards.six{grid-template-columns:repeat(6,1fr);}
.cards.four{grid-template-columns:repeat(4,1fr);}
.card{background:var(--paper);border:1px solid var(--border);border-radius:var(--radius);padding:13px 15px;}
.card.lp{border-left:3px solid var(--wf-blue);}
.card.jv{border-left:3px solid var(--wf-green);}
.card .lab{font-size:11px;color:var(--ink3);margin-bottom:5px;}
.card .val{font-size:20px;font-weight:700;color:var(--ink);font-variant-numeric:tabular-nums;line-height:1;}
.card .sub{font-size:10.5px;color:var(--ink4);margin-top:4px;}

.chart-row{display:grid;grid-template-columns:1fr 1fr;gap:14px;margin-bottom:14px;}
.chart-card{background:var(--paper);border:1px solid var(--border);border-radius:var(--radius);padding:16px 18px;margin-bottom:14px;}
.chart-card h3{font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--ink3);margin-bottom:10px;}
.chart-card canvas{max-width:100%;}
.cap{font-size:11px;color:var(--ink3);line-height:1.5;margin-top:9px;}

.stitle{display:block;font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;color:var(--wf-green);margin:18px 0 9px;}
.table-wrap{background:var(--paper);border:1px solid var(--border);border-radius:var(--radius);overflow-x:auto;margin-bottom:14px;}
table.wide{width:100%;border-collapse:collapse;font-size:12px;table-layout:fixed;}
table.wide th{background:var(--ink);color:#fff;padding:7px 9px;text-align:right;font-weight:500;font-size:10.5px;white-space:nowrap;}
table.wide td{padding:5px 9px;text-align:right;border-bottom:1px solid var(--rule);white-space:nowrap;font-variant-numeric:tabular-nums;font-family:var(--mono);overflow:hidden;text-overflow:ellipsis;}
table.wide th.lnum,table.wide td.lnum{width:26px;min-width:26px;text-align:right;font-family:var(--mono);font-size:9px;font-weight:normal;color:var(--ink4);background:var(--paper);border-right:1.5px solid var(--border);padding:5px 6px 5px 3px;white-space:nowrap;}
table.wide th:nth-child(2){text-align:left;}
table.wide td:nth-child(2){text-align:left;font-weight:600;color:#333;font-family:var(--font);}
table.wide tr.section-head td.section-head-cell{text-align:left;background:var(--accent-tint);color:var(--wf-green);font-weight:700;font-size:10px;text-transform:uppercase;letter-spacing:.05em;padding:5px 9px;font-family:var(--font);border-bottom:1px solid var(--border);}
table.wide td.lpv{color:var(--wf-blue);font-weight:700;}
table.wide td.jvv{color:var(--wf-green);font-weight:700;}

.tblock{border-left:3px solid var(--wf-green);padding:12px 16px;margin-bottom:14px;background:var(--canvas);border-radius:0 4px 4px 0;}
.tblock.lp{border-left-color:var(--wf-blue);}
.tblock h4{font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--ink);margin-bottom:7px;}
.tblock p{font-size:12px;color:var(--ink2);line-height:1.6;margin-bottom:7px;}
.tblock p:last-child{margin-bottom:0;}
.tblock b{color:var(--ink);}
.fofi{border:1px solid #cdd6e0;background:#f4f7fb;border-radius:var(--radius);padding:16px 20px;margin-bottom:14px;}
.fofi h4{font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--ink2);margin-bottom:8px;}
.fofi p{font-size:11.5px;color:var(--ink2);line-height:1.6;margin-bottom:7px;}
.fofi p:last-child{margin-bottom:0;}
.chapter-foot{font-size:10.5px;color:var(--ink4);border-top:1px solid var(--rule);padding-top:9px;margin-top:4px;}

@media print{
  @page{size:letter landscape;margin:0.4in;}
  body{background:#fff !important;font-size:9px;}
  .appbar,.print-now,button{display:none !important;}
  .report-title{display:block;border-bottom:2.5px solid var(--wf-gold);padding-bottom:6px;margin-bottom:10px;}
  .report-title .rt-title{font-size:15px;font-weight:700;color:var(--wf-blue);}
  .report-title .rt-meta{font-size:10px;color:var(--ink3);margin-top:2px;}
  .pg-runhead{display:block;font-size:9px;font-weight:700;color:var(--wf-blue);border-bottom:1.5px solid var(--wf-gold);padding-bottom:4px;margin-bottom:8px;}
  .keymsg{max-width:100%;margin:0 0 8px;padding:0;page-break-after:avoid;}
  .keymsg-grid{gap:8px;} .km{padding:8px 11px;} .km p{font-size:9px;}
  .chapter{max-width:100%;margin:0 0 10px;padding:0;}
  .pagebreak{page-break-before:always;}
  .chapter-head{padding:8px 13px;margin-bottom:8px;}
  .ch-sec{font-size:13px;} .ch-mast{font-size:9px;} .ch-term{font-size:9.5px;}
  .basis,.lead{font-size:9px;margin-bottom:7px;line-height:1.45;}
  .cards{gap:7px;margin-bottom:8px;}
  .card{padding:7px 9px;} .card .val{font-size:15px;} .card .lab{font-size:9px;} .card .sub{font-size:8.5px;}
  .chart-row{gap:9px;margin-bottom:8px;}
  .chart-card{padding:8px 11px;margin-bottom:8px;}
  .chart-card h3{margin-bottom:6px;} .cap{font-size:8.5px;margin-top:6px;line-height:1.4;}
  .stitle{margin:9px 0 5px;}
  .table-wrap{overflow:visible !important;margin-bottom:8px;}
  table.wide td,table.wide th{font-size:8.5px;padding:2.5px 5px;}
  .tblock,.fofi{padding:8px 13px;margin-bottom:8px;}
  .tblock p,.fofi p{font-size:9px;margin-bottom:5px;line-height:1.45;}
  .chapter-foot{font-size:8.5px;padding-top:6px;}
  .chapter-head,.card,.chart-card,.tblock,.fofi,.km,.chart-row{box-shadow:none !important;border-color:#ccc !important;break-inside:avoid;page-break-inside:avoid;}
  tr{page-break-inside:avoid;}
  thead{display:table-header-group;}
  .stitle,h3,h4,.ch-sec,.chapter-head{page-break-after:avoid;}
  .chart-card canvas{max-width:100% !important;}
}
</style>
</head>
"##;

pub fn render() -> (String, String) {
    let s = series();
    let f = figures(&s);
    let data_str = serde_json::to_string(&chart_data_json(&s)).unwrap_or_else(|_| "{}".to_string());

    let sfpct = fmt_pct0(f.sf_pct_lp_denom / 100.0);
    let sfd = fmt_int(f.sf_delta);
    let jvsf = fmt_int(f.sf_jv_total);
    let lpsf = fmt_int(f.sf_lp_total);
    let jvde = fmt_x(f.jv_peak_de);
    let lpde = fmt_x(f.lp_peak_de);
    let ltvpk = fmt_pct0(f.lp_peak_ltv);
    let ltvst = fmt_pct0(f.lp_stab_ltv);
    let jvltv = fmt_pct1(f.jv_ltv);
    let jvy = f.jv_sf_online_year;
    let lpy = f.lp_sf_online_year;
    let jvds = f.jv_div_start_year;
    let jvdiv = format!("{:.0}", f.jv_cum_div_y10 / 1_000_000.0);
    let moicjv = fmt_x2(f.moic_jv);
    let moiclp = fmt_x2(f.moic_lp);
    let irrjv = fmt_pct1(f.irr_jv);
    let irrlp = fmt_pct1(f.irr_lp);
    let navlp = format!("{:.0}", f.lp_nav_y10);
    let navjv = format!("{:.0}", f.jv_nav_y10);
    let y20sf = fmt_m(f.y20_sqft);
    let y20cov = fmt_x(f.y20_coverage);
    let y20nav = format!("{:.0}", f.y20_nav);
    let y20debt = fmt_m(f.y20_debt);
    let extgrow = fmt_pct0(f.y20_sf_growth_pct / 100.0);
    let extcovmin = fmt_x(f.ext_cov_min);

    let mut h = String::new();
    h.push_str(HEAD);
    h.push_str("<body>\n");

    h.push_str(r#"<div class="appbar">
  <div class="appbar-title">Alternative Real Estate — Legacy Joint Ventures vs. Direct-Hold Solutions</div>
  <div class="appbar-meta">$250M identical equity · Y1–Y10 (+ Y11–Y20 illustration) · Woodfine Capital Projects · June 2026</div>
  <button class="print-now" onclick="window.print()">⎙ Print / PDF</button>
</div>
<div class="report-title">
  <div class="rt-title">Alternative Real Estate — Legacy Joint Ventures vs. Direct-Hold Solutions</div>
  <div class="rt-meta">$250M identical equity · Y1–Y10 (+ Y11–Y20 illustration) · Woodfine Capital Projects · June 2026</div>
</div>
"#);

    // ── Key messages (3-up) ──────────────────────────────────────────────────
    h.push_str(&format!(r#"<div class="keymsg">
  <div class="keymsg-h">The same $250M, two structures — what each one does with it, and for whom</div>
  <div class="keymsg-grid">
    <div class="km lp"><span class="kt">What the <b>Direct-Hold Solutions</b> structure builds</span><p>Retaining developer profit as common equity, it funds a phased build to about <b>{sfpct} more</b> square footage (~<b>{sfd}</b> additional sq ft) and a higher terminal multiple (<b>{moiclp}</b> vs {moicjv}) — at higher peak leverage and a later income date.</p></div>
    <div class="km jv"><span class="kt">What the <b>Legacy JV</b> delivers</span><p>Full <b>{jvsf} sq ft</b> online by <b>Year {jvy}</b>, a lower flat <b>{jvde}</b> leverage, and cash to investors from <b>Year {jvds}</b> — about <b>${jvdiv}M</b> cumulatively by Year 10. Earlier income, lower leverage, simpler structure.</p></div>
    <div class="km"><span class="kt">The one fork that explains it</span><p>The JV pays developer profit and operating cash <b>out</b>; the Direct-Hold structure <b>retains</b> it. That single extract-vs-retain choice drives every difference in scale, leverage, cash timing and terminal return that follows.</p></div>
  </div>
</div>
"#));

    // ── Section 1 of 6 — headline: scale + leverage ──────────────────────────
    h.push_str(&format!(r#"<section class="chapter" id="chap-1">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 1 of 6 — Same $250M, two machines</div>
    <div class="ch-term">The one fork — cash extracted vs. equity retained — read through scale and leverage</div>
    <div class="basis">All series generated by the forecast engine (D7 Legacy JV · D2 Direct-Hold Solutions). Debt to equity is measured on the same $250M of contributed capital for both. Y1–Y10.</div>
  </div>
  <div class="cards">
    <div class="card"><div class="lab">Equity (both)</div><div class="val">$250M</div><div class="sub">identical input</div></div>
    <div class="card jv"><div class="lab">JV square feet</div><div class="val">{jvsf}</div><div class="sub">full from Y{jvy}</div></div>
    <div class="card lp"><div class="lab">Direct-Hold square feet</div><div class="val">{lpsf}</div><div class="sub">full from Y{lpy}</div></div>
    <div class="card jv"><div class="lab">JV cash to investors</div><div class="val">from Y{jvds}</div><div class="sub">~${jvdiv}M by Y10</div></div>
    <div class="card lp"><div class="lab">Direct-Hold additional SF</div><div class="val">+{sfpct}</div><div class="sub">+{sfd} sq ft</div></div>
  </div>
  <div class="chart-row">
    <div class="chart-card"><h3>Portfolio scale — square feet controlled</h3><canvas id="cv-scale"></canvas><div class="cap">Square feet online by year. The JV brings its full footage on at stabilisation (Y{jvy}); the Direct-Hold structure phases up and ends about {sfpct} larger (+{sfd} sq ft).</div></div>
    <div class="chart-card"><h3>Leverage path — debt to equity on the same $250M</h3><canvas id="cv-leverage"></canvas><div class="cap">The JV holds a flat, lower {jvde}; the Direct-Hold structure ramps to a {lpde} peak then eases as stabilised value is recognised and debt is bought back.</div></div>
  </div>
  <div class="stitle">Year-by-year — leverage and square footage (engine-generated)</div>
  <div class="table-wrap"><table class="wide" id="tbl-series"><thead></thead><tbody></tbody></table></div>
  <div class="chapter-foot">Forward-looking information (FOFI) — assumptions as at June 2026; actual results will vary. See Section 6.</div>
</section>
"#));

    // ── Section 2 of 6 — Direct-Hold pulls ahead: value + LTV ────────────────
    h.push_str(&format!(r#"<section class="chapter" id="chap-2">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 2 of 6 — Where the Direct-Hold structure pulls ahead</div>
    <div class="ch-term">Compounding scale · retained-equity reserve · loan-to-value that stabilises as leverage rises</div>
  </div>
  <div class="lead">Because profit is retained rather than paid out, the Direct-Hold Solutions structure funds a phased build that ends with about {sfpct} more square footage and a <em>falling</em> loan-to-value even as debt-to-equity rises — capital efficiency, not a financing assumption.</div>
  <div class="chart-row">
    <div class="chart-card"><h3>Value per unit of capital ($100 invested)</h3><canvas id="cv-value"></canvas><div class="cap">Both shown on an IFRS fair-value basis per unit, indexed to the $100 capital reference. Direct-Hold NAV/unit compounds to ~${navlp} with step jumps as phases complete; the JV's IFRS fair-value equity holds at ~${navjv}/share.</div></div>
    <div class="chart-card"><h3>Loan-to-value — debt ÷ asset value (IFRS fair value)</h3><canvas id="cv-ltv"></canvas><div class="cap">Both on an IFRS fair-value basis. The Legacy JV holds a flat ~{jvltv} ($750M bank debt ÷ $1,260M stabilised value); the Direct-Hold LTV peaks near {ltvpk} during the leverage cycle then declines to about {ltvst} as debt is bought back — both at a 6.25% cap rate.</div></div>
  </div>
  <div class="tblock lp"><h4>Compensation taken in equity becomes a structural reserve</h4>
    <p>Woodfine takes its development compensation in common equity rather than cash. Because developer profit is not extracted, it is retained as equity on the balance sheet, building a structural reserve that supports the structure through the leverage cycle — the direct driver of the additional square footage delivered on the same initial capital.</p>
  </div>
  <div class="tblock lp"><h4>Leverage scales, but loan-to-value moderates</h4>
    <p>Debt to equity scales to a peak of about <b>{lpde}</b>, yet debt-to-asset value peaks near <b>{ltvpk}</b> and then declines to approximately <b>{ltvst}</b> as debt is bought back — while the Legacy JV's loan-to-value holds flat at ~{jvltv}. Rising leverage on the contributed-capital base coincides with a moderating loan-to-value as stabilised asset value is recognised. The higher terminal outcome comes with higher peak leverage and a later income date — see Section 3.</p>
  </div>
  <div class="chapter-foot">Direct-Hold figures: D2 Direct-Hold Solutions, engine-generated. Forward-looking — see Section 6.</div>
</section>
"#));

    // ── Section 3 of 6 — Legacy JV pulls ahead: cash + coverage ──────────────
    h.push_str(&format!(r#"<section class="chapter" id="chap-3">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 3 of 6 — Where the Legacy JV pulls ahead</div>
    <div class="ch-term">Earlier full income · lower flat leverage · cash in hand · institutional debt discipline</div>
  </div>
  <div class="lead">The comparison is two-sided. The single-shot Legacy JV carries structural advantages of its own — earlier income, a lower and steadier leverage and coverage profile, and real cash to investors years sooner.</div>
  <div class="chart-row">
    <div class="chart-card"><h3>Cash to investors per year</h3><canvas id="cv-cash"></canvas><div class="cap">The JV distributes cash from Year {jvds} (~${jvdiv}M cumulatively by Year 10); the Direct-Hold structure retains profit as equity and distributes little until the portfolio is fully generating. Front-loaded vs. back-loaded.</div></div>
    <div class="chart-card"><h3>Debt-service coverage — vs the 1.20× covenant</h3><canvas id="cv-coverage"></canvas><div class="cap">Coverage begins at stabilisation (Year {jvy}); the shaded years are construction, with no operating income. The JV holds a flat, comfortable coverage; the Direct-Hold structure dips to a mid-build trough then rises. Lower, steadier leverage gives the JV a simpler risk profile.</div></div>
  </div>
  <div class="tblock"><h4>Full square footage online earlier</h4>
    <p>The Legacy JV draws a committed bank facility during construction and brings its full <b>{jvsf} square feet</b> online by <b>Year {jvy}</b>; the Direct-Hold structure does not reach its full <b>{lpsf} square feet</b> until <b>Year {lpy}</b>. The JV reaches full stabilised income several years sooner.</p>
  </div>
  <div class="tblock"><h4>Lower, flat leverage and cash from Year {jvds}</h4>
    <p>JV leverage holds flat at about <b>{jvde}</b> debt to equity against a Direct-Hold peak near <b>{lpde}</b>, and under its 2/20 arrangement it distributes cash to shareholders from <b>Year {jvds}</b> — about <b>${jvdiv}M</b> cumulatively by Year 10. The Direct-Hold structure retains developer profit as equity; favourable for compounding, but the JV delivers realised cash to investors earlier.</p>
  </div>
  <div class="tblock"><h4>Institutional debt discipline and single-shot simplicity</h4>
    <p>The JV's construction-to-permanent bank facility is governed by institutional lending standards and covenants, and the structure is single-shot: leverage does not compound and there is no phased re-issuance to manage. For an investor who values predictability, lender discipline and a non-compounding leverage path, that simplicity is a structural advantage.</p>
  </div>
  <div class="chapter-foot">Legacy JV figures: D7, engine-generated. Forward-looking — see Section 6.</div>
</section>
"#));

    // ── Section 4 of 6 — resolution: MOIC + IRR scoreboard ───────────────────
    h.push_str(&format!(r#"<section class="chapter" id="chap-4">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 4 of 6 — How it resolves: value and return on the same dollars</div>
    <div class="ch-term">Terminal multiple and rate of return · which is &ldquo;ahead&rdquo; depends on the investor</div>
  </div>
  <div class="lead">By the close of the cycle the Direct-Hold structure produces a higher multiple and rate of return, but the Legacy JV has already returned most of its value in cash. Which structure is &ldquo;ahead&rdquo; depends on whether an investor values realised cash and simplicity or terminal compounded value.</div>
  <div class="cards six">
    <div class="card jv"><div class="lab">JV MOIC (Y10)</div><div class="val">{moicjv}</div><div class="sub">IFRS FV basis</div></div>
    <div class="card lp"><div class="lab">Direct-Hold MOIC</div><div class="val">{moiclp}</div><div class="sub">IFRS FV basis</div></div>
    <div class="card jv"><div class="lab">JV IRR (pre-tax)</div><div class="val">{irrjv}</div><div class="sub">IFRS FV terminal</div></div>
    <div class="card lp"><div class="lab">Direct-Hold IRR</div><div class="val">{irrlp}</div><div class="sub">investor fraction</div></div>
    <div class="card jv"><div class="lab">JV cash by Y10</div><div class="val">~${jvdiv}M</div><div class="sub">from Year {jvds}</div></div>
    <div class="card lp"><div class="lab">Direct-Hold NAV/unit</div><div class="val">~${navlp}</div><div class="sub">on $100 invested</div></div>
  </div>
  <div class="chart-row">
    <div class="chart-card"><h3>Multiple on invested capital (Y10)</h3><canvas id="cv-moic"></canvas><div class="cap">Total value returned per dollar of equity, IFRS fair-value basis. Direct-Hold {moiclp} vs Legacy JV {moicjv}.</div></div>
    <div class="chart-card"><h3>Internal rate of return (pre-tax)</h3><canvas id="cv-irr"></canvas><div class="cap">Annualised return on the investor cash-flow series, IFRS fair-value terminal. Direct-Hold {irrlp} vs Legacy JV {irrjv}.</div></div>
  </div>
  <div class="tblock"><h4>Higher terminal value is not strictly &ldquo;better&rdquo;</h4>
    <p>The Direct-Hold structure's higher multiple and rate of return reflect retained, compounded value realised late; the Legacy JV converted more of its result into cash the investor already holds, earlier, at lower leverage. Both structures are now stated on the same IFRS fair-value basis, so the multiple and return figures are directly comparable.</p>
  </div>
  <div class="chapter-foot">MOIC and IRR are engine-computed (xirr over per-year investor cash flows). Forward-looking — see Section 6.</div>
</section>
"#));

    // ── Section 5 of 6 — the next decade (Years 11–20 illustration) ──────────
    h.push_str(&format!(r#"<section class="chapter" id="chap-5">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 5 of 6 — The next decade: compounding past the first ten years</div>
    <div class="ch-term">Illustrative Y11–Y20 continuation · development funded by recycling cash and debentures · coverage held comfortably above the 1.20× covenant</div>
    <div class="basis">Y11–Y20 is a forward-looking illustration projected from the Y10 end state; it does not modify or re-run the D2 proforma. The structure may continue to develop by recycling retained cash and issuing matched debentures at a target loan-to-cost. Pace and funding mix are stated, tunable assumptions. See Section 6.</div>
  </div>
  <div class="lead">Even at a deliberately measured pace — funding new development out of retained cash and recycled debentures — the Direct-Hold structure may keep compounding through a second decade with interest coverage that strengthens rather than weakens, holding far above the 1.20× covenant the whole way.</div>
  <div class="cards four">
    <div class="card lp"><div class="lab">Square feet (Y20, illustrative)</div><div class="val">~{y20sf}</div><div class="sub">+{extgrow} vs Y10</div></div>
    <div class="card lp"><div class="lab">Interest coverage (Y20)</div><div class="val">~{y20cov}</div><div class="sub">covenant 1.20× · min ~{extcovmin}</div></div>
    <div class="card lp"><div class="lab">NAV / unit (Y20)</div><div class="val">~${y20nav}</div><div class="sub">continues compounding</div></div>
    <div class="card lp"><div class="lab">Debentures (Y20)</div><div class="val">~{y20debt}</div><div class="sub">recycled into new build</div></div>
  </div>
  <div class="chart-row">
    <div class="chart-card"><h3>Portfolio scale — square feet, Years 1–20</h3><canvas id="cv-ext-scale"></canvas><div class="cap">The Legacy JV is single-shot and ends at Year 10; the Direct-Hold structure may continue to develop at a measured pace through Year 20 (shaded). Square footage grows to ~{y20sf} (+{extgrow} over Year 10).</div></div>
    <div class="chart-card"><h3>Interest coverage — Years 1–20, vs the 1.20× covenant</h3><canvas id="cv-ext-coverage"></canvas><div class="cap">Coverage dips to its mid-build trough in the first decade, then strengthens as the portfolio stabilises and continues to build — staying far above the 1.20× covenant across the illustrated Y11–Y20 continuation (minimum ~{extcovmin}).</div></div>
  </div>
  <div class="tblock lp"><h4>Growth funded by recycling, not by stretching leverage</h4>
    <p>In the illustration, new development each year is paced to a share of retained funds-from-operations, with matched debentures issued at a target loan-to-cost — so the build proceeds at a measured pace and leverage is recycled rather than stretched. This is the compounding mechanism extended forward, on stated assumptions.</p>
  </div>
  <div class="tblock lp"><h4>Coverage strengthens as the portfolio compounds</h4>
    <p>Because completed space yields about 10.5% on cost against a 5.0% debenture coupon, each increment of development is accretive to coverage. Projected interest coverage holds in the ~{extcovmin}–{y20cov} range across Y11–Y20 — at no point approaching the 1.20× debenture covenant.</p>
  </div>
  <div class="chapter-foot">Y11–Y20 is an illustrative continuation generated from the Y10 end state; it is not a commitment to develop or to issue debentures, and does not re-run the D2 proforma. Forward-looking — see Section 6.</div>
</section>
"#));

    // ── Section 6 of 6 — basis + FOFI ────────────────────────────────────────
    h.push_str(r#"<section class="chapter" id="chap-6">
  <div class="chapter-head">
    <div class="ch-mast">Alternative Real Estate — Legacy JV vs. Direct-Hold</div>
    <div class="ch-sec">Section 6 of 6 — Basis of preparation &amp; caveats</div>
    <div class="ch-term">Engine provenance · IFRS fair-value measurement basis · forward-looking statements</div>
  </div>
  <div class="tblock"><h4>Basis of preparation</h4>
    <p>All Y1–Y10 figures are engine-generated; this document performs no independent calculation. Legacy JV series from the D7 model (<code>legacy_jv_proforma</code>); Direct-Hold series from the D2 Direct-Hold Solutions model (<code>pro-01_proforma</code>). Debt to equity and all per-capital figures are measured against the same $250,000,000 of contributed capital, so every comparison is on identical initial equity.</p>
    <p><b>Measurement basis.</b> Both structures are presented on an IFRS fair-value (mark-to-model) basis at a 6.25% cap rate, so value, NAV, loan-to-value, MOIC and IRR figures are stated on the same basis and are directly comparable. Woodfine Direct-Hold Solutions issue Investment Units and are direct-hold structures / an issuer; they are not redeemable collective-investment vehicles.</p>
    <p><b>Years 11–20 continuation.</b> The Y11–Y20 figures (Section 5) are an illustrative continuation of the structure's compounding capability past the initial ten-year proforma horizon, projected in this document from the Y10 end state on stated assumptions (development paced to retained funds-from-operations, matched debentures at a target loan-to-cost). They do not modify or re-run the D2 proforma.</p>
  </div>
  <div class="fofi"><h4>Forward-Looking Information (FOFI)</h4>
    <p>This tear sheet is future-oriented financial information prepared on the stated assumptions as at June 2026. References to planned capital recycling, intended portfolio growth, target leverage and expected returns are forward-looking; MOIC, IRR and the Years 11–20 projection are management estimates. Actual results, measurements and outcomes will differ — potentially materially — and readers should not place undue reliance on them.</p>
    <p>The Years 11–20 projection is an illustration of compounding capability, not a commitment, plan or undertaking to develop additional property, to issue further debentures, or to achieve any stated coverage, square footage or value. For information only; not an offer to sell or a solicitation to buy securities; securities may be offered only pursuant to an applicable prospectus exemption under National Instrument 45-106. Prepared and presented consistent with NI 51-102 and OSC SN 51-721.</p>
  </div>
  <div class="chapter-foot">Alternative Real Estate · Legacy JV vs. Direct-Hold · engine June 2026 build · generated June 2026 · V2</div>
</section>
"#);

    h.push_str("<script>\nconst D = ");
    h.push_str(&data_str);
    h.push_str(";\n");
    h.push_str(SCRIPT);
    h.push_str("\n</script>\n</body>\n</html>\n");

    (h, audit_json(&s, &f))
}

const SCRIPT: &str = r##"/* Alternative Real Estate — Legacy JV vs Direct-Hold (client).
   Ten focused charts + one table, drawn once from engine pre-computed data. One chart = one message. */
const charts = {};
const INK='#111827', GREY='#9CA3AF', JV='#54924E', LP='#164679', RED='#ED1B2F', GOLD='#F57F17';
const dash='—';
function fM(v){ if(v==null) return dash; return (v/1e6).toFixed(1)+'M'; }
function fMoney(v){ if(v==null) return dash; const a=Math.abs(v); if(a>=1e6) return '$'+(v/1e6).toFixed(1)+'M'; if(a>=1e3) return '$'+(v/1e3).toFixed(0)+'K'; return '$'+Math.round(v); }
function fInt(v){ return (v==null)?dash:Math.round(v).toLocaleString('en-US'); }
function fX(v){ return (v!=null&&v>0)?v.toFixed(2)+'×':dash; }
function fPct(v){ return (v*100).toFixed(0)+'%'; }

function mk(id,cfg){
  const c=document.getElementById(id); if(!c) return;
  if(charts[id]) charts[id].destroy();
  cfg.options=cfg.options||{};
  cfg.options.animation=false; cfg.options.responsive=true;
  if(cfg.options.aspectRatio==null) cfg.options.aspectRatio=2.3;
  charts[id]=new Chart(c,cfg);
}
function legend(on){ return { display:on, position:'bottom', labels:{ boxWidth:18, font:{size:10} } }; }
function legendNoRef(){ return { display:true, position:'bottom', labels:{ boxWidth:18, font:{size:10}, filter:i=>i.text!=='reference' } }; }
function grouped(jv,lp){ return [
  { label:D.labelExact.jv, data:jv, backgroundColor:JV, borderColor:JV, categoryPercentage:0.72, barPercentage:0.92 },
  { label:D.labelExact.lp, data:lp, backgroundColor:LP, borderColor:LP, categoryPercentage:0.72, barPercentage:0.92 },
]; }
function lines(jv,lp,opt){ opt=opt||{}; return [
  { label:D.labelExact.jv, data:jv, borderColor:JV, backgroundColor:JV, borderWidth:2.5, pointRadius:0, pointHoverRadius:4, tension:opt.t==null?.25:opt.t, fill:false, spanGaps:true },
  { label:D.labelExact.lp, data:lp, borderColor:LP, backgroundColor:LP, borderWidth:2.5, pointRadius:0, pointHoverRadius:4, tension:opt.t==null?.25:opt.t, fill:false, spanGaps:true },
]; }
function refLine(value,color,labels){ return { label:'reference', data:(labels||D.labels).map(()=>value), borderColor:color, borderDash:[4,4], borderWidth:1, pointRadius:0, fill:false }; }
// Faint shaded band over an index range [i0,i1] with an optional centred label.
function bandPlugin(i0,i1,label){ return { id:'band_'+i0+'_'+i1, beforeDraw(chart){
  const a=chart.chartArea, x=chart.scales.x; if(!a||!x) return;
  const half=(x.getPixelForValue(1)-x.getPixelForValue(0))/2;
  const xa=x.getPixelForValue(i0)-half, xb=x.getPixelForValue(i1)+half;
  const ctx=chart.ctx; ctx.save();
  ctx.fillStyle='rgba(156,163,175,0.12)'; ctx.fillRect(xa,a.top,xb-xa,a.bottom-a.top);
  if(label){ ctx.fillStyle='#6B7280'; ctx.font='9px ui-sans-serif,system-ui,sans-serif'; ctx.textAlign='center'; ctx.fillText(label,(xa+xb)/2,a.top+10); }
  ctx.restore();
} }; }

/* Section 1 — scale (bar) + leverage (line) */
function chartScale(){ mk('cv-scale',{ type:'bar', data:{ labels:D.labels, datasets:grouped(D.sqftJV,D.sqftLP) },
  options:{ aspectRatio:2.2, plugins:{ legend:legend(true),
    tooltip:{ callbacks:{ label:c=>c.dataset.label+': '+fInt(c.parsed.y)+' sq ft' } } },
    scales:{ y:{ min:0, max:4000000, title:{display:true,text:'Total square feet',font:{size:11,weight:'700'},color:INK}, ticks:{ stepSize:1000000, font:{size:10}, callback:fM } },
             x:{ ticks:{ font:{size:10} } } } } }); }
function chartLeverage(){ mk('cv-leverage',{ type:'line', data:{ labels:D.labels, datasets:lines(D.deJV,D.deLP) },
  options:{ aspectRatio:2.2, interaction:{mode:'index',intersect:false}, plugins:{ legend:legend(true),
    tooltip:{ callbacks:{ label:c=>c.dataset.label+': '+fX(c.parsed.y) } } },
    scales:{ y:{ min:0, max:4.5, title:{display:true,text:'Debt to equity (× $250M)',font:{size:11,weight:'700'},color:INK}, ticks:{ stepSize:0.5, font:{size:10}, callback:v=>v.toFixed(1)+'×' } },
             x:{ title:{display:true,text:'Year',font:{size:10}}, ticks:{ font:{size:10} } } } } }); }

/* Section 2 — value (line + $100 ref) + LTV (line + 65% ref) */
function chartValue(){ mk('cv-value',{ type:'line', data:{ labels:D.labels, datasets:[ refLine(100,RED), ...lines(D.navJV,D.navLP) ] },
  options:{ aspectRatio:2.2, interaction:{mode:'index',intersect:false}, plugins:{ legend:legendNoRef(),
    tooltip:{ callbacks:{ label:c=>c.dataset.label+': $'+c.parsed.y.toFixed(0) } } },
    scales:{ y:{ min:0, suggestedMax:480, title:{display:true,text:'Value per unit ($)',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:v=>'$'+v } },
             x:{ ticks:{ font:{size:10} } } } } }); }
function chartLtv(){ mk('cv-ltv',{ type:'line', data:{ labels:D.labels, datasets:[ refLine(0.65,GOLD), ...lines(D.ltvJV,D.ltvLP) ] },
  options:{ aspectRatio:2.2, plugins:{ legend:legendNoRef() },
    scales:{ y:{ min:0, max:1.0, title:{display:true,text:'Debt ÷ asset value',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:fPct } },
             x:{ ticks:{ font:{size:10} } } } } }); }

/* Section 3 — cash (bar) + coverage (line + 1.20× ref, clean start at Y4) */
function chartCash(){ mk('cv-cash',{ type:'bar', data:{ labels:D.labels, datasets:grouped(D.divJV,D.distLP) },
  options:{ aspectRatio:2.2, plugins:{ legend:legend(true),
    tooltip:{ callbacks:{ label:c=>c.dataset.label+': '+fMoney(c.parsed.y) } } },
    scales:{ y:{ min:0, title:{display:true,text:'Cash to investors ($/yr)',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:fMoney } },
             x:{ ticks:{ font:{size:10} } } } } }); }
function covSet(data,color){ return { label:(color===JV?D.labelExact.jv:D.labelExact.lp), data, borderColor:color, backgroundColor:color, borderWidth:2.5,
  pointRadius:data.map(v=>v==null?0:3.5), pointHoverRadius:5, pointBackgroundColor:color, tension:0.3, fill:false, spanGaps:false }; }
function chartCoverage(){ mk('cv-coverage',{ type:'line',
  data:{ labels:D.labels, datasets:[ refLine(1.20,RED), covSet(D.dscrJV,JV), covSet(D.icrLP,LP) ] },
  plugins:[ bandPlugin(0,2,'construction (no income)') ],
  options:{ aspectRatio:2.2, plugins:{ legend:legendNoRef() },
    scales:{ y:{ min:0, max:3.0, title:{display:true,text:'Coverage (×)',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:v=>v.toFixed(1)+'×' } },
             x:{ ticks:{ font:{size:10} } } } } }); }

/* Section 4 — MOIC + IRR scoreboard (horizontal bars, two structures) */
function scoreboard(id,jvVal,lpVal,axisTitle,fmt,maxv){
  mk(id,{ type:'bar', data:{ labels:['Legacy JV','Direct-Hold'], datasets:[
    { data:[jvVal,lpVal], backgroundColor:[JV,LP], barPercentage:0.7, categoryPercentage:0.8 } ]},
  options:{ aspectRatio:3.0, indexAxis:'y', plugins:{ legend:{display:false},
    tooltip:{ callbacks:{ label:c=>fmt(c.parsed.x) } } },
    scales:{ x:{ min:0, suggestedMax:maxv, title:{display:true,text:axisTitle,font:{size:10}}, ticks:{ font:{size:10}, callback:fmt } },
             y:{ ticks:{ font:{size:10} } } } } });
}
function chartMoic(){ scoreboard('cv-moic', D.endpoints.moicJV, D.endpoints.moicLP, 'Multiple on invested capital (Y10)', v=>v.toFixed(2)+'×', 4.5); }
function chartIrr(){ scoreboard('cv-irr', D.endpoints.irrJV*100, D.endpoints.irrLP*100, 'IRR (pre-tax, %)', v=>v.toFixed(1)+'%', 18); }

/* Section 5 — Years 1–20 continuation (scale + coverage), Y11–20 shaded */
function chartExtScale(){ const E=D.ext; mk('cv-ext-scale',{ type:'line',
  data:{ labels:E.labels, datasets:[
    { label:D.labelExact.jv, data:E.sqftJV, borderColor:JV, backgroundColor:JV, borderWidth:2.5, pointRadius:0, tension:.2, fill:false, spanGaps:false },
    { label:D.labelExact.lp, data:E.sqftLP, borderColor:LP, backgroundColor:LP, borderWidth:2.5, pointRadius:0, tension:.2, fill:false, spanGaps:true },
  ]},
  plugins:[ bandPlugin(10,19,'illustrative continuation (Y11–Y20)') ],
  options:{ aspectRatio:2.2, plugins:{ legend:legend(true),
    tooltip:{ callbacks:{ label:c=>c.dataset.label+': '+fInt(c.parsed.y)+' sq ft' } } },
    scales:{ y:{ min:0, title:{display:true,text:'Total square feet',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:fM } },
             x:{ ticks:{ font:{size:9}, maxRotation:0, autoSkip:true } } } } }); }
function chartExtCoverage(){ const E=D.ext; mk('cv-ext-coverage',{ type:'line',
  data:{ labels:E.labels, datasets:[ refLine(1.20,RED,E.labels),
    { label:D.labelExact.lp, data:E.icrLP, borderColor:LP, backgroundColor:LP, borderWidth:2.5, pointRadius:0, tension:.3, fill:false, spanGaps:false } ]},
  plugins:[ bandPlugin(10,19,'illustrative continuation (Y11–Y20)') ],
  options:{ aspectRatio:2.2, plugins:{ legend:legendNoRef() },
    scales:{ y:{ min:0, suggestedMax:3.5, title:{display:true,text:'Coverage (×)',font:{size:11,weight:'700'},color:INK}, ticks:{ font:{size:10}, callback:v=>v.toFixed(1)+'×' } },
             x:{ ticks:{ font:{size:9}, maxRotation:0, autoSkip:true } } } } }); }

/* Year table with continuous left-gutter line numbers */
let lineNo=1;
function gnum(){ return '<td class="lnum">'+(lineNo++)+'</td>'; }
function ghead(){ return '<th class="lnum"></th>'; }
function gblank(){ return '<td class="lnum"></td>'; }
function renderTable(){
  const t=document.getElementById('tbl-series');
  t.querySelector('thead').innerHTML='<tr>'+ghead()+'<th>Metric</th>'+D.labels.map(l=>'<th>'+l+'</th>').join('')+'</tr>';
  const xrow=(label,arr,cls)=>'<tr>'+gnum()+'<td>'+label+'</td>'+arr.map(v=>'<td class="'+cls+'">'+(v>0?v.toFixed(2)+'×':dash)+'</td>').join('')+'</tr>';
  const srow=(label,arr,cls)=>'<tr>'+gnum()+'<td>'+label+'</td>'+arr.map(v=>'<td class="'+cls+'">'+(v>0?fInt(v):dash)+'</td>').join('')+'</tr>';
  const head=(t)=>'<tr class="section-head">'+gblank()+'<td class="section-head-cell" colspan="11">'+t+'</td></tr>';
  t.querySelector('tbody').innerHTML =
    head('Debt to equity (on $250M contributed capital)') +
    xrow('Legacy JV', D.deJV, 'jvv') + xrow('Direct-Hold', D.deLP, 'lpv') +
    head('Total square feet') +
    srow('Legacy JV', D.sqftJV, 'jvv') + srow('Direct-Hold', D.sqftLP, 'lpv');
}

function renderAll(){
  lineNo=1; renderTable();
  chartScale(); chartLeverage(); chartValue(); chartLtv();
  chartCash(); chartCoverage(); chartMoic(); chartIrr();
  chartExtScale(); chartExtCoverage();
}
renderAll();
window.addEventListener('beforeprint',()=>{ Object.values(charts).forEach(c=>c.resize()); });
"##;

fn audit_json(s: &Series, f: &Figures) -> String {
    let lp = pclp1_proforma::forecast();
    let ext = extend_y11_y20(lp_at(&lp, 10));
    let ext_rows: Vec<Value> = ext
        .iter()
        .map(|e| {
            json!({
                "year": e.year, "sqft": e.sqft, "debt": e.debt, "noi": e.noi,
                "coverage": e.coverage, "nav_per_unit": e.nav_per_unit,
                "distributions": e.distributions, "new_sf": e.new_sf
            })
        })
        .collect();
    let v = json!({
        "artifact": "mcorp-tearsheet-alternative-re-v2",
        "title": "Alternative Real Estate — Legacy Joint Ventures vs. Direct-Hold Solutions",
        "source": "tool-proforma-engine — report::tearsheet_alt_re_v2 (D7 legacy_jv_proforma + D2 pclp1_proforma)",
        "generated_at": "2026-06-08",
        "version": "V2",
        "companion_of": "Woodfine Direct-Hold Solutions — Sensitivity Analysis (V2)",
        "note": "Engine-built, multi-chart two-sided comparison. Debt to equity on the same $250M contributed base for both; LTV/NAV/MOIC/IRR on IFRS fair value; MOIC/IRR via xirr over per-year investor cash flows.",
        "labels": s.labels,
        "series": {
            "deJV": s.de_jv, "deLP": s.de_lp, "sqftJV": s.sqft_jv, "sqftLP": s.sqft_lp,
            "ltvJV": s.ltv_jv, "ltvLP": s.ltv_lp, "dscrJV": s.dscr_jv, "icrLP": s.icr_lp,
            "navJV": s.nav_jv, "navLP": s.nav_lp, "divJV": s.div_jv, "distLP": s.dist_lp
        },
        "figures": {
            "jv_sf_total": f.sf_jv_total, "lp_sf_total": f.sf_lp_total,
            "additional_sf": f.sf_delta, "additional_sf_pct_lp_denominator": f.sf_pct_lp_denom,
            "jv_sf_online_year": f.jv_sf_online_year, "lp_sf_online_year": f.lp_sf_online_year,
            "jv_peak_debt_to_equity": f.jv_peak_de, "lp_peak_debt_to_equity": f.lp_peak_de,
            "jv_ifrs_ltv": f.jv_ltv, "lp_peak_ltv": f.lp_peak_ltv, "lp_stabilised_ltv": f.lp_stab_ltv,
            "jv_dividend_start_year": f.jv_div_start_year, "jv_cumulative_dividends_y10": f.jv_cum_div_y10,
            "moic_jv": f.moic_jv, "moic_lp": f.moic_lp, "irr_jv": f.irr_jv, "irr_lp": f.irr_lp,
            "jv_ifrs_fv_equity_per_share_y10": f.jv_nav_y10, "lp_nav_per_unit_y10": f.lp_nav_y10
        },
        "extension_y11_y20": {
            "profile": "debenture-recycling, compounding (operator-selected)",
            "assumptions": {
                "recycle_ffo_pct": EXT_RECYCLE_FFO_PCT, "new_dev_ltc": EXT_NEW_DEV_LTC,
                "cost_per_sf": EXT_COST_PER_SF, "dev_yield": PCLP1_DEV_YIELD,
                "cap_rate": PCLP1_CAP_RATE, "debt_rate": PCLP1_DEBT_RATE_DEBENTURE
            },
            "y20": {"sqft": f.y20_sqft, "coverage": f.y20_coverage, "nav_per_unit": f.y20_nav, "debt": f.y20_debt},
            "coverage_min": f.ext_cov_min,
            "rows": ext_rows,
            "disclosure": "Illustrative continuation; forward-looking; not a commitment. Projected from the Y10 end state; does not re-run the D2 proforma. NI 51-102 / OSC SN 51-721."
        },
        "inputs": {
            "legacy_jv_gross_equity": LEGACY_JV_GROSS_EQUITY, "legacy_jv_total_sf": LEGACY_JV_TOTAL_SF,
            "legacy_jv_shares": LEGACY_JV_SHARES, "legacy_jv_stabilized_av": LEGACY_JV_STABILIZED_AV,
            "pclp1_gross_equity": PCLP1_GROSS_EQUITY, "pclp1_total_portfolio_sqft": PCLP1_TOTAL_PORTFOLIO_SQFT,
            "pclp1_investor_fraction": PCLP1_INVESTOR_UNITS / PCLP1_DILUTED_UNITS
        },
        "palette": {"wf-green": "#54924E", "wf-blue": "#164679", "wf-gold": "#F57F17", "wf-red": "#ED1B2F"},
        "disclosure": "OSC SN 51-721 / NI 51-102 forward-looking; figures are management estimates as of 2026-06-08."
    });
    serde_json::to_string_pretty(&v).expect("tearsheet v2 audit JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_series_length_10() {
        let s = series();
        for v in [
            &s.de_jv, &s.de_lp, &s.sqft_jv, &s.sqft_lp, &s.ltv_jv, &s.ltv_lp, &s.dscr_jv,
            &s.icr_lp, &s.nav_jv, &s.nav_lp, &s.div_jv, &s.dist_lp,
        ] {
            assert_eq!(v.len(), 10);
        }
        assert_eq!(s.labels.len(), 10);
    }

    #[test]
    fn debt_to_equity_shapes() {
        let s = series();
        assert!((s.de_jv[0] - 0.60).abs() < 0.02);
        for i in 2..10 {
            assert!((s.de_jv[i] - 3.0).abs() < 0.02, "JV D/E Y{}", i + 1);
        }
        let peak = s.de_lp.iter().cloned().fold(0.0_f64, f64::max);
        assert!(peak > 3.8 && peak < 4.1, "LP peak D/E {peak}");
    }

    #[test]
    fn jv_ltv_is_ifrs_flat() {
        let s = series();
        assert_eq!(s.ltv_jv[0], 0.0); // construction → null
        for i in 3..10 {
            assert!(
                (s.ltv_jv[i] - 0.5952).abs() < 0.002,
                "JV LTV Y{} {}",
                i + 1,
                s.ltv_jv[i]
            );
        }
    }

    #[test]
    fn sqft_engine_truthful() {
        let s = series();
        assert_eq!(s.sqft_jv[0], 0.0);
        for i in 3..10 {
            assert!((s.sqft_jv[i] - LEGACY_JV_TOTAL_SF).abs() < 1.0);
        }
        assert!((s.sqft_lp[9] - PCLP1_TOTAL_PORTFOLIO_SQFT).abs() < 1.0);
    }

    #[test]
    fn coverage_construction_years_zero() {
        let s = series();
        assert_eq!(s.icr_lp[0], 0.0);
        assert!(s.icr_lp[7] > 1.0);
        assert_eq!(s.dscr_jv[0], 0.0); // JV construction coverage 0 → null in chart
        assert!((s.dscr_jv[5] - 2.10).abs() < 0.1);
    }

    #[test]
    fn cash_front_vs_back_loaded() {
        let s = series();
        assert!(s.div_jv[4] > 1_000_000.0, "JV Y5 dividend");
        assert!(s.dist_lp[0] < 1.0, "LP Y1 distribution ~0");
    }

    #[test]
    fn endpoints_moic_irr_sane() {
        let s = series();
        let f = figures(&s);
        assert!(f.moic_jv > 2.5 && f.moic_jv < 3.4, "JV MOIC {}", f.moic_jv);
        assert!(f.moic_lp > 3.3 && f.moic_lp < 5.2, "LP MOIC {}", f.moic_lp);
        assert!(f.irr_jv > 0.08 && f.irr_jv < 0.20, "JV IRR {}", f.irr_jv);
        assert!(f.irr_lp > 0.10 && f.irr_lp < 0.24, "LP IRR {}", f.irr_lp);
        assert!(
            f.moic_lp > f.moic_jv && f.irr_lp > f.irr_jv,
            "DH should lead on return"
        );
    }

    #[test]
    fn extension_grows_and_holds_coverage() {
        let lp = pclp1_proforma::forecast();
        let ext = extend_y11_y20(lp_at(&lp, 10));
        assert_eq!(ext.len(), 10);
        // square footage grows past Y10
        assert!(
            ext[9].sqft > PCLP1_TOTAL_PORTFOLIO_SQFT,
            "Y20 sqft {}",
            ext[9].sqft
        );
        // coverage stays comfortably above the 1.20× covenant every year
        for e in &ext {
            assert!(e.coverage >= 1.8, "Y{} coverage {}", e.year, e.coverage);
        }
        // NAV keeps compounding
        assert!(
            ext[9].nav_per_unit > lp_at(&lp, 10).nav_per_unit,
            "Y20 NAV {}",
            ext[9].nav_per_unit
        );
    }

    #[test]
    fn additional_sf_about_41pct() {
        let f = figures(&series());
        assert!((f.sf_delta - 1_608_705.0).abs() < 2.0);
        assert!(f.sf_pct_lp_denom > 40.0 && f.sf_pct_lp_denom < 42.0);
    }

    #[test]
    fn html_has_six_sections_and_charts() {
        let (html, _j) = render();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.ends_with("</html>\n"));
        for n in 1..=6 {
            assert!(
                html.contains(&format!("Section {n} of 6")),
                "missing section {n}"
            );
        }
        for id in [
            "cv-scale",
            "cv-leverage",
            "cv-value",
            "cv-ltv",
            "cv-cash",
            "cv-coverage",
            "cv-moic",
            "cv-irr",
            "cv-ext-scale",
            "cv-ext-coverage",
        ] {
            assert!(html.contains(id), "missing chart {id}");
        }
        assert!(html.contains("generated June 2026 · V2"));
        assert!(html.contains("size:letter landscape"));
        assert!(html.contains("Where the Direct-Hold structure pulls ahead"));
        assert!(html.contains("Where the Legacy JV pulls ahead"));
        assert!(html.contains("The next decade"));
        // editorial cleanups
        assert!(!html.contains("No Excel source"));
        assert!(!html.contains("Professional Centres Canada LP"));
        assert!(!html.contains("Traditional / JV"));
        assert!(!html.contains("Woodfine LP"));
        assert!(!html.contains("Direct-Hold (LP)"));
        assert!(!html.contains("accounting-basis"));
        assert!(html.contains("pro-01_proforma"));
    }

    #[test]
    fn embedded_data_parses() {
        let (html, _j) = render();
        let start = html.find("const D = ").expect("marker") + "const D = ".len();
        let tail = &html[start..];
        let end = tail.find(";\n").expect("terminator");
        let _: serde_json::Value = serde_json::from_str(&tail[..end]).expect("D parses");
    }

    #[test]
    fn audit_json_round_trips() {
        let (_h, json) = render();
        let v: serde_json::Value = serde_json::from_str(&json).expect("audit parses");
        assert_eq!(v["version"], "V2");
        assert!(v["series"]["navLP"].as_array().unwrap().len() == 10);
        assert!(v["extension_y11_y20"]["rows"].as_array().unwrap().len() == 10);
        assert!(
            v["figures"]["moic_lp"].as_f64().unwrap() > v["figures"]["moic_jv"].as_f64().unwrap()
        );
    }
}
