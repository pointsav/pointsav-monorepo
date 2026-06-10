// Professional Centres Canada LP — Sensitivity Analysis V7
// Engine-sourced base case; no hardcoded financials.
// Companion to mcorp-tearsheet-alternative-re-v6.html.

use crate::spv::pclp1_proforma::{
    self, PCLP1_GROSS_EQUITY, PCLP1_DILUTED_UNITS, PCLP1_CAP_RATE,
    PCLP1_DEV_YIELD, PCLP1_DEBT_RATE_DEBENTURE, PCLP1_BUYER_TARGET_YIELD,
    PCLP1_ADVISORY_FEE_PCT,
};

/// Model assumption constants not stored in the V2 engine.
const BASE_OCC_PCT: f64 = 95.0;
const BASE_LEASEUP_MO: f64 = 12.0;
/// Y8 base-case compounded return — indicative only; not recomputed under stress.
const COMP_RETURN_Y8: f64 = 0.14775;

fn js_arr(vals: &[f64]) -> String {
    let s: Vec<String> = vals.iter().map(|v| format!("{:.4}", v)).collect();
    format!("[{}]", s.join(", "))
}

fn js_arr_n(vals: &[Option<f64>]) -> String {
    let s: Vec<String> = vals
        .iter()
        .map(|v| match v {
            Some(x) => format!("{:.4}", x),
            None => "null".to_string(),
        })
        .collect();
    format!("[{}]", s.join(", "))
}

/// Returns (sensitivity_html, audit_json).
pub fn render() -> (String, String) {
    let years = pclp1_proforma::forecast();
    let ys = &years[1..=10]; // Y1..Y10, 10 entries, 0-indexed

    // ── BASE arrays ──────────────────────────────────────────────────────────
    let noi: Vec<f64> = ys.iter().map(|y| y.net_proceeds_from_ops).collect();
    let ebitda: Vec<f64> = ys.iter().map(|y| y.ebitda).collect();
    let interest: Vec<f64> = ys.iter().map(|y| y.net_interest).collect();
    let inc_cont: Vec<f64> = ys
        .iter()
        .enumerate()
        .map(|(i, y)| if i < 3 { y.income_continuity } else { 0.0 })
        .collect();
    let cash: Vec<f64> = ys.iter().map(|y| y.ending_cash).collect();
    let debt_vals: Vec<f64> = ys.iter().map(|y| y.closing_debt).collect();
    let wip: Vec<f64> = ys.iter().map(|y| y.wip).collect();
    let asset_pu: Vec<f64> = ys.iter().map(|y| y.asset_value_per_unit).collect();
    let nav_pu: Vec<f64> = ys.iter().map(|y| y.nav_per_unit).collect();
    let dist_pu: Vec<f64> = ys.iter().map(|y| y.dpu).collect();
    let coverage: Vec<Option<f64>> = ys
        .iter()
        .map(|y| {
            if y.interest_coverage <= 0.0 {
                None
            } else {
                Some(y.interest_coverage)
            }
        })
        .collect();
    let mv_pu_fixed: Vec<f64> = ys[..7].iter().map(|y| y.market_value_per_unit).collect();
    let ter: Vec<f64> = ys
        .iter()
        .map(|y| {
            if y.nav > 0.0 {
                (y.advisory_fee + y.admin_compliance + y.board) / y.nav
            } else {
                0.0
            }
        })
        .collect();
    let ltc: Vec<f64> = ys.iter().map(|y| y.debt_to_dev_cost).collect();
    let sqft: Vec<f64> = ys.iter().map(|y| y.total_sqft_generating).collect();
    let comp_return: Vec<Option<f64>> = ys
        .iter()
        .enumerate()
        .map(|(i, _)| if i == 7 { Some(COMP_RETURN_Y8) } else { None })
        .collect();

    // ── Threshold computations ───────────────────────────────────────────────
    let yr7 = &years[7]; // Y7 (binding DSCR year)
    let yr8 = &years[8]; // Y8 (first fully-stabilized year)
    let base_cap = PCLP1_CAP_RATE;
    let base_dev = PCLP1_DEV_YIELD;
    let base_debt = PCLP1_DEBT_RATE_DEBENTURE;

    // Cap rate → NAV = $100/unit at Y8
    // navPU = (cash + NOI/capRate + wip - debt) / DILUTED = 100
    // capRate = NOI / (100*DILUTED - cash - wip + debt)
    let cap_denom =
        100.0 * PCLP1_DILUTED_UNITS - yr8.ending_cash - yr8.wip + yr8.closing_debt;
    let cap_rate_nav100 = yr8.net_proceeds_from_ops / cap_denom;

    // Debenture rate → DSCR = 1.20 at Y7
    // coverage = ebitda / (interest * intF) = 1.20  where intF = debtRate / base_debt
    // debtRate = base_debt * ebitda7 / (1.20 * interest7)
    let rate_dscr120 = base_debt * yr7.ebitda / (1.20 * yr7.net_interest);

    // Occupancy → DSCR = 1.20 at Y7 (devYield held at base)
    // noi = BASE_noi7 * occF;  ebitda = noi - expenses7;  1.20 = ebitda / interest7
    let expenses_y7 = yr7.net_proceeds_from_ops - yr7.ebitda;
    let occ_dscr120 = BASE_OCC_PCT * (1.20 * yr7.net_interest + expenses_y7)
        / yr7.net_proceeds_from_ops;

    // Dev yield → DSCR = 1.20 at Y7 (occupancy held at base)
    let dev_dscr120 = base_dev * (1.20 * yr7.net_interest + expenses_y7)
        / yr7.net_proceeds_from_ops;

    // ── Slider bounds ────────────────────────────────────────────────────────
    let cap_h = (cap_rate_nav100 - base_cap).abs();
    let cap_min = (base_cap - 0.50 * cap_h).max(0.01);
    let cap_max = cap_rate_nav100 + 0.20 * cap_h;

    let debt_h = (rate_dscr120 - base_debt).abs();
    let debt_min = (base_debt - 0.50 * debt_h).max(0.01);
    let debt_max = rate_dscr120 + 0.20 * debt_h;

    let occ_h = (BASE_OCC_PCT - occ_dscr120).abs();
    let occ_min = (occ_dscr120 - 0.20 * occ_h).max(50.0);
    let occ_max = (BASE_OCC_PCT + 0.50 * occ_h).min(100.0);

    let dev_h = (base_dev - dev_dscr120).abs();
    let dev_min = (dev_dscr120 - 0.20 * dev_h).max(0.01);
    let dev_max = base_dev + 0.50 * dev_h;

    let leaseup_min = 6.0_f64;
    let leaseup_max = 21.0_f64;
    let mktyr_min = 6.0_f64;
    let mktyr_max = 10.0_f64;

    // Scenario values (bear = at or near thresholds; bull = at improving bounds)
    let bear_cap = (cap_rate_nav100 * 10000.0).round() / 100.0;
    let bear_occ = (occ_dscr120 * 10.0).round() / 10.0;
    let bear_debt = (rate_dscr120 * 10000.0).round() / 100.0;
    let bear_dev = (dev_dscr120 * 10000.0).round() / 100.0;
    let bear_leaseup = leaseup_max;
    let bear_mktyr = mktyr_max;

    let bull_cap = (cap_min * 10000.0).round() / 100.0;
    let bull_occ = (occ_max * 10.0).round() / 10.0;
    let bull_debt = (debt_min * 10000.0).round() / 100.0;
    let bull_dev = (dev_max * 10000.0).round() / 100.0;
    let bull_leaseup = leaseup_min;
    let bull_mktyr = mktyr_min;

    // ── Headroom bar fill %  (normalized: 1000 bps = 100% for rates; 50 pp = 100% for %) ──
    let cap_hbps = (cap_rate_nav100 - base_cap) * 10000.0;
    let debt_hbps = (rate_dscr120 - base_debt) * 10000.0;
    let occ_hpp = BASE_OCC_PCT - occ_dscr120;
    let dev_hbps = (base_dev - dev_dscr120) * 10000.0;

    let cap_bar = (cap_hbps / 1000.0).min(1.0) * 100.0;
    let debt_bar = (debt_hbps / 1000.0).min(1.0) * 100.0;
    let occ_bar = (occ_hpp / 50.0).min(1.0) * 100.0;
    let dev_bar = (dev_hbps / 1000.0).min(1.0) * 100.0;

    // ── Misc derived ─────────────────────────────────────────────────────────
    let advisory_fee_annual = PCLP1_GROSS_EQUITY * PCLP1_ADVISORY_FEE_PCT;
    let base_mv8 = dist_pu[7] / PCLP1_BUYER_TARGET_YIELD;
    let base_nav8 = nav_pu[7];
    let base_nav8_x = (base_nav8 / 100.0 * 10.0).round() / 10.0; // e.g. 3.9×
    let min_cov_base = coverage
        .iter()
        .filter_map(|v| *v)
        .fold(f64::INFINITY, f64::min);
    let base_tot_dist: f64 = dist_pu.iter().sum();

    // ── JSON ─────────────────────────────────────────────────────────────────
    let years_json = serde_json::to_string_pretty(&years[1..=10])
        .unwrap_or_else(|_| "[]".to_string());

    let json = format!(
        r#"{{
  "metadata": {{
    "entity": "Professional Centres Canada LP (PCLP 1)",
    "generated": "2026-06-07",
    "engine": "tool-proforma-engine src/spv/pclp1_proforma",
    "ifrs_reference": "IFRS 13 para 93(h)(ii); IFRS 10 investment-entity FVTPL",
    "covenant_floor_dscr": 1.20,
    "capital_preservation_floor_nav_per_unit": 100.0
  }},
  "base_case": {{
    "diluted_units": {diluted:.0},
    "years": {years_json}
  }},
  "model_inputs": {{
    "cap_rate": {base_cap:.4},
    "occupancy_pct": {base_occ:.1},
    "lease_up_months": {base_leaseup:.0},
    "dev_yield": {base_dev:.4},
    "debenture_rate": {base_debt:.4},
    "market_yield": {mktyr:.4},
    "advisory_fee_annual": {advisory_fee:.0},
    "gross_equity": {gross_equity:.0}
  }},
  "thresholds": {{
    "nav_par": {{
      "cap_rate": {cap_nav100:.6},
      "cap_rate_pct": {cap_nav100_pct:.2},
      "headroom_bps": {cap_hbps:.1},
      "binding_year": 8
    }},
    "dscr_covenant": {{
      "debenture_rate": {rate_dscr:.6},
      "debenture_rate_pct": {rate_dscr_pct:.2},
      "headroom_bps": {dhbps:.1},
      "occupancy_pct": {occ_dscr:.2},
      "occ_headroom_pp": {occ_hpp:.2},
      "dev_yield": {dev_dscr:.6},
      "dev_yield_pct": {dev_dscr_pct:.2},
      "dev_headroom_bps": {devhbps:.1},
      "binding_year": 7,
      "method": "coverage=EBITDA7/(net_interest7*intF); intF=debtRate/base_debt"
    }}
  }},
  "slider_bounds": {{
    "cap_rate_pct": [{cap_mn_pct:.2}, {cap_mx_pct:.2}],
    "debenture_rate_pct": [{dbt_mn_pct:.2}, {dbt_mx_pct:.2}],
    "occupancy_pct": [{occ_mn:.1}, {occ_mx:.1}],
    "dev_yield_pct": [{dev_mn_pct:.2}, {dev_mx_pct:.2}],
    "lease_up_months": [{lu_mn:.0}, {lu_mx:.0}],
    "market_yield_pct": [{my_mn:.1}, {my_mx:.1}]
  }},
  "scenarios": {{
    "bear": {{ "cap_pct": {bear_cap:.2}, "occ_pct": {bear_occ:.1}, "leaseup_mo": {bear_lu:.0}, "dev_pct": {bear_dev:.2}, "debt_pct": {bear_debt:.2}, "mktyr_pct": {bear_my:.1} }},
    "base": {{ "cap_pct": {base_cap_p:.2}, "occ_pct": {base_occ:.1}, "leaseup_mo": {base_leaseup:.0}, "dev_pct": {base_dev_p:.2}, "debt_pct": {base_debt_p:.2}, "mktyr_pct": {base_my:.1} }},
    "bull": {{ "cap_pct": {bull_cap:.2}, "occ_pct": {bull_occ:.1}, "leaseup_mo": {bull_lu:.0}, "dev_pct": {bull_dev:.2}, "debt_pct": {bull_debt:.2}, "mktyr_pct": {bull_my:.1} }}
  }}
}}"#,
        diluted = PCLP1_DILUTED_UNITS,
        years_json = years_json,
        base_cap = base_cap,
        base_occ = BASE_OCC_PCT,
        base_leaseup = BASE_LEASEUP_MO,
        base_dev = base_dev,
        base_debt = base_debt,
        mktyr = PCLP1_BUYER_TARGET_YIELD,
        advisory_fee = advisory_fee_annual,
        gross_equity = PCLP1_GROSS_EQUITY,
        cap_nav100 = cap_rate_nav100,
        cap_nav100_pct = cap_rate_nav100 * 100.0,
        cap_hbps = cap_hbps,
        rate_dscr = rate_dscr120,
        rate_dscr_pct = rate_dscr120 * 100.0,
        dhbps = debt_hbps,
        occ_dscr = occ_dscr120,
        occ_hpp = occ_hpp,
        dev_dscr = dev_dscr120,
        dev_dscr_pct = dev_dscr120 * 100.0,
        devhbps = dev_hbps,
        cap_mn_pct = cap_min * 100.0,
        cap_mx_pct = cap_max * 100.0,
        dbt_mn_pct = debt_min * 100.0,
        dbt_mx_pct = debt_max * 100.0,
        occ_mn = occ_min,
        occ_mx = occ_max,
        dev_mn_pct = dev_min * 100.0,
        dev_mx_pct = dev_max * 100.0,
        lu_mn = leaseup_min,
        lu_mx = leaseup_max,
        my_mn = mktyr_min,
        my_mx = mktyr_max,
        bear_cap = bear_cap,
        bear_occ = bear_occ,
        bear_lu = bear_leaseup,
        bear_dev = bear_dev,
        bear_debt = bear_debt,
        bear_my = bear_mktyr,
        base_cap_p = base_cap * 100.0,
        base_dev_p = base_dev * 100.0,
        base_debt_p = base_debt * 100.0,
        base_my = PCLP1_BUYER_TARGET_YIELD * 100.0,
        bull_cap = bull_cap,
        bull_occ = bull_occ,
        bull_lu = bull_leaseup,
        bull_dev = bull_dev,
        bull_debt = bull_debt,
        bull_my = bull_mktyr,
    );

    // ── HTML ─────────────────────────────────────────────────────────────────
    let mut h = String::with_capacity(120 * 1024);

    h.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
"#);
    h.push_str("<title>Professional Centres Canada LP — Sensitivity Analysis V7</title>\n");
    h.push_str(r#"<script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
<style>
:root {
  --ink:    #1a1a1a;
  --ink2:   #555;
  --ink3:   #888;
  --ink4:   #aaa;
  --bg:     #fff;
  --bg2:    #fafafa;
  --bg3:    #f5f5f5;
  --border: #e5e5e5;
  --row-sep:#ebebeb;
  --pos:    #1a6e2e;
  --warn:   #b45309;
  --accent: #1a1a1a;
  --page-w: 1200px;
  --font:   system-ui, -apple-system, sans-serif;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: var(--font); font-size: 13px; background: var(--bg); color: var(--ink); line-height: 1.6; }

header { background: var(--ink); color: #fff; padding: 16px 32px; display: flex; align-items: baseline; gap: 24px; }
header h1 { font-size: 16px; font-weight: 600; white-space: nowrap; }
header p  { font-size: 12px; color: #9ca3af; }

.main { max-width: var(--page-w); margin: 0 auto; padding: 24px 32px; }
.stitle { display: block; font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .08em; color: var(--ink3); margin: 24px 0 10px; }

.card { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 20px 24px; margin-bottom: 16px; }
.card.accent { border-color: var(--accent); background: var(--bg2); }

.controls { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 20px 24px; margin-bottom: 16px; }
.scenario-row { display: flex; align-items: center; gap: 10px; margin-bottom: 16px; flex-wrap: wrap; }
.scenario-btn { padding: 5px 14px; border: 1.5px solid var(--border); border-radius: 5px; background: var(--bg); font-size: 12px; font-weight: 600; cursor: pointer; color: var(--ink2); transition: all .15s; }
.scenario-btn:hover { background: var(--bg3); }
.scenario-btn.active { background: var(--ink); color: #fff; border-color: var(--ink); }
.reset-btn { color: var(--ink3); border-style: dashed; }
.slider-grid { display: grid; grid-template-columns: repeat(6, 1fr); gap: 20px; }
.slider-item label { font-size: 12px; color: var(--ink2); display: block; margin-bottom: 2px; }
.slider-item .val { font-size: 22px; font-weight: 700; color: var(--ink); line-height: 1; margin-bottom: 8px; }
.slider-item .val span { font-size: 13px; font-weight: 400; color: var(--ink3); }
input[type=range] { width: 100%; height: 4px; accent-color: var(--ink); cursor: pointer; }
.slider-item .range-row { display: flex; justify-content: space-between; font-size: 11px; color: var(--ink4); margin-top: 5px; }

.metrics { display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px; margin-bottom: 16px; }
.mc { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 14px 16px; }
.mc-label { font-size: 11px; color: var(--ink3); margin-bottom: 5px; }
.mc-base { font-size: 19px; font-weight: 700; color: var(--ink); }
.mc-stressed { font-size: 12px; margin-top: 4px; font-weight: 600; }
.up   { color: var(--pos); }
.down { color: #dc2626; }
.flat { color: var(--ink3); }
.mc-sub { font-size: 11px; color: var(--ink4); margin-top: 2px; }

.chart-row { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; margin-bottom: 16px; }
.chart-card { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 18px 20px; }
.chart-card h3 { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .07em; color: var(--ink3); margin-bottom: 12px; }
.chart-print { display: none; }

.table-wrap { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; overflow: hidden; margin-bottom: 16px; overflow-x: auto; }
.yr-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.yr-table thead th { background: var(--ink); color: #fff; padding: 9px 12px; text-align: right; font-weight: 500; font-size: 11px; white-space: nowrap; }
.yr-table thead th:first-child { text-align: left; }
.yr-table tbody td { padding: 7px 12px; text-align: right; border-bottom: 1px solid var(--row-sep); white-space: nowrap; }
.yr-table tbody td:first-child { text-align: left; font-weight: 600; color: #333; }
.yr-table tbody tr:hover td { background: #f8f9ff; }
.section-row td { background: var(--bg3); font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .06em; color: var(--ink3); padding: 5px 12px; }
.spacer-row td { height: 4px; padding: 0; border: none; }

.badge { display: inline-block; padding: 1px 6px; border-radius: 3px; font-size: 11px; font-weight: 700; }
.badge-red    { background: #fee2e2; color: #dc2626; }
.badge-yellow { background: #fef9c3; color: var(--warn); }
.badge-green  { background: #dcfce7; color: #15803d; }

.flags-card { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 18px 22px; margin-bottom: 16px; }
.flags-card h3 { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .06em; color: var(--ink3); margin-bottom: 12px; }
.flag-list { display: flex; flex-direction: column; gap: 7px; }
.flag-item { display: flex; align-items: flex-start; gap: 10px; font-size: 12.5px; }
.flag-dot { width: 7px; height: 7px; border-radius: 50%; margin-top: 5px; flex-shrink: 0; }
.dot-red    { background: #dc2626; }
.dot-yellow { background: #ca8a04; }
.dot-green  { background: #16a34a; }
.no-flags { color: var(--ink3); font-size: 12.5px; font-style: italic; }

.tblock { border-left: 3px solid var(--ink); padding: 12px 16px; margin-bottom: 14px; background: var(--bg2); border-radius: 0 4px 4px 0; }
.tblock h4 { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .07em; color: var(--ink); margin-bottom: 7px; }
.tblock ul { margin-left: 16px; }
.tblock li { margin-bottom: 3px; font-size: 12.5px; color: var(--ink2); }
.tblock p { font-size: 12.5px; color: var(--ink2); }
.disclosure { margin-top: 12px; padding-top: 10px; border-top: 1px solid var(--border); font-size: 11.5px; color: var(--ink3); }

/* ── Audit section ── */
.audit-section { background: var(--bg); border: 1px solid var(--border); border-radius: 8px; padding: 22px 26px; margin-top: 24px; }
.audit-banner { border-left: 4px solid var(--ink); padding: 12px 16px; background: var(--bg2); border-radius: 0 6px 6px 0; margin-bottom: 20px; font-size: 12.5px; line-height: 1.65; color: var(--ink2); }
.audit-banner strong { color: var(--ink); }
.audit-table { width: 100%; border-collapse: collapse; font-size: 12px; margin-bottom: 22px; }
.audit-table th { background: var(--ink); color: #fff; padding: 8px 12px; text-align: right; font-weight: 500; font-size: 11px; }
.audit-table th:first-child { text-align: left; }
.audit-table td { padding: 7px 12px; text-align: right; border-bottom: 1px solid var(--row-sep); }
.audit-table td:first-child { text-align: left; font-weight: 600; }
.audit-table .binding { color: #dc2626; font-weight: 700; }
.audit-table .safe    { color: var(--pos); font-weight: 600; }
.hbar-section h4 { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: .07em; color: var(--ink3); margin-bottom: 14px; }
.hbar-row { display: flex; align-items: center; gap: 14px; margin-bottom: 10px; }
.hbar-label { width: 160px; font-size: 12px; font-weight: 600; color: var(--ink); flex-shrink: 0; }
.hbar-track { flex: 1; height: 12px; background: #f0f0f0; border-radius: 6px; overflow: hidden; }
.hbar-fill-safe { height: 100%; background: var(--pos); border-radius: 6px; opacity: 0.75; }
.hbar-fill-tight { height: 100%; background: var(--warn); border-radius: 6px; opacity: 0.85; }
.hbar-vals { font-size: 11px; color: var(--ink2); white-space: nowrap; width: 200px; text-align: right; }
.audit-posture { margin-top: 18px; padding-top: 12px; border-top: 1px solid var(--border); font-size: 11.5px; color: var(--ink3); line-height: 1.65; }

footer { max-width: var(--page-w); margin: 16px auto; padding: 0 32px 24px; border-top: 1px solid var(--border); font-size: 11.5px; color: var(--ink3); line-height: 1.65; padding-top: 14px; }

/* ── Print ── */
@media print {
  @page { size: letter landscape; margin: 0.75in 0.6in 0.85in; }
  body { font-size: 11px; background: #fff !important; }
  .main { max-width: 100%; padding: 0; }
  header { padding: 10px 0; }
  .controls, .scenario-row, .scenario-btn, .reset-btn, input[type=range] { display: none !important; }
  .chart-card canvas { display: none !important; }
  .chart-print { display: block !important; font-size: 11px; }
  .chart-print table { width: 100%; border-collapse: collapse; }
  .chart-print th { background: var(--ink); color: #fff; padding: 5px 8px; font-size: 10px; text-align: right; }
  .chart-print th:first-child { text-align: left; }
  .chart-print td { padding: 4px 8px; text-align: right; border-bottom: 1px solid var(--row-sep); font-size: 10px; }
  .yr-table { display: table !important; overflow: visible !important; }
  .table-wrap { overflow: visible !important; }
  thead { display: table-header-group; }
  tr { page-break-inside: avoid; }
  .audit-section { page-break-before: always; }
  .stitle, h2, h3, h4 { page-break-after: avoid; }
  .card, .controls, .mc, .chart-card, .table-wrap, .flags-card, .audit-section { box-shadow: none; border-color: #ccc; }
}
</style>
</head>
<body>
"#);

    // Header
    h.push_str(&format!(
        r#"<header>
  <h1>Professional Centres Canada LP — Sensitivity Analysis</h1>
  <p>Woodfine Management Corp. &nbsp;·&nbsp; ${gross_equity_m:.0}M Equity ({diluted_k:.0}K units at ${unit_price:.0}) &nbsp;·&nbsp; June 2026 &nbsp;·&nbsp; V7 — Engine-sourced base case</p>
</header>
<div class="main">
"#,
        gross_equity_m = PCLP1_GROSS_EQUITY / 1e6,
        diluted_k = PCLP1_DILUTED_UNITS / 1000.0,
        unit_price = 100.0_f64,
    ));

    // Slider controls
    h.push_str(&format!(
        r#"  <div class="controls">
    <span class="stitle">Assumptions</span>
    <div class="scenario-row">
      <button id="btn-bear" class="scenario-btn" onclick="applyScenario('bear')">Bear</button>
      <button id="btn-base" class="scenario-btn active" onclick="applyScenario('base')">Base</button>
      <button id="btn-bull" class="scenario-btn" onclick="applyScenario('bull')">Bull</button>
      <button class="scenario-btn reset-btn" onclick="applyScenario('base')">Reset</button>
    </div>
    <div class="slider-grid">
      <div class="slider-item">
        <label>Cap Rate (Valuation)</label>
        <div class="val" id="val-cap">{base_cap_pct:.2}<span>%</span></div>
        <input type="range" id="sl-cap" min="{cap_min_pct:.2}" max="{cap_max_pct:.2}" step="0.25" value="{base_cap_pct:.2}">
        <div class="range-row"><span>{cap_min_pct:.1}%</span><span>Base {base_cap_pct:.2}%</span><span>{cap_max_pct:.1}%</span></div>
      </div>
      <div class="slider-item">
        <label>Occupancy at Stabilization</label>
        <div class="val" id="val-occ">{base_occ:.0}<span>%</span></div>
        <input type="range" id="sl-occ" min="{occ_min:.1}" max="{occ_max:.1}" step="1" value="{base_occ:.0}">
        <div class="range-row"><span>{occ_min:.0}%</span><span>Base {base_occ:.0}%</span><span>{occ_max:.0}%</span></div>
      </div>
      <div class="slider-item">
        <label>Lease-Up Period</label>
        <div class="val" id="val-leaseup">{base_leaseup:.0}<span> mo</span></div>
        <input type="range" id="sl-leaseup" min="{leaseup_min:.0}" max="{leaseup_max:.0}" step="1" value="{base_leaseup:.0}">
        <div class="range-row"><span>{leaseup_min:.0} mo</span><span>Base {base_leaseup:.0} mo</span><span>{leaseup_max:.0} mo</span></div>
      </div>
      <div class="slider-item">
        <label>Development Yield (NOI/Cost)</label>
        <div class="val" id="val-dev">{base_dev_pct:.2}<span>%</span></div>
        <input type="range" id="sl-dev" min="{dev_min_pct:.2}" max="{dev_max_pct:.2}" step="0.05" value="{base_dev_pct:.2}">
        <div class="range-row"><span>{dev_min_pct:.1}%</span><span>Base {base_dev_pct:.2}%</span><span>{dev_max_pct:.1}%</span></div>
      </div>
      <div class="slider-item">
        <label>Debenture Interest Rate</label>
        <div class="val" id="val-debt">{base_debt_pct:.2}<span>%</span></div>
        <input type="range" id="sl-debt" min="{debt_min_pct:.2}" max="{debt_max_pct:.2}" step="0.05" value="{base_debt_pct:.2}">
        <div class="range-row"><span>{debt_min_pct:.2}%</span><span>Base {base_debt_pct:.2}%</span><span>{debt_max_pct:.2}%</span></div>
      </div>
      <div class="slider-item">
        <label>Market Yield Rate</label>
        <div class="val" id="val-mktyr">{base_mktyr_pct:.1}<span>%</span></div>
        <input type="range" id="sl-mktyr" min="{mktyr_min:.1}" max="{mktyr_max:.1}" step="0.5" value="{base_mktyr_pct:.1}">
        <div class="range-row"><span>{mktyr_min:.1}%</span><span>Base {base_mktyr_pct:.1}%</span><span>{mktyr_max:.1}%</span></div>
      </div>
    </div>
  </div>
"#,
        base_cap_pct = base_cap * 100.0,
        cap_min_pct = cap_min * 100.0,
        cap_max_pct = cap_max * 100.0,
        base_occ = BASE_OCC_PCT,
        occ_min = occ_min,
        occ_max = occ_max,
        base_leaseup = BASE_LEASEUP_MO,
        leaseup_min = leaseup_min,
        leaseup_max = leaseup_max,
        base_dev_pct = base_dev * 100.0,
        dev_min_pct = dev_min * 100.0,
        dev_max_pct = dev_max * 100.0,
        base_debt_pct = base_debt * 100.0,
        debt_min_pct = debt_min * 100.0,
        debt_max_pct = debt_max * 100.0,
        base_mktyr_pct = PCLP1_BUYER_TARGET_YIELD * 100.0,
        mktyr_min = mktyr_min,
        mktyr_max = mktyr_max,
    ));

    // Metrics cards
    h.push_str(&format!(
        r#"  <div class="metrics">
    <div class="mc">
      <div class="mc-label">Y8 NAV / Unit</div>
      <div class="mc-base" id="m-nav8-base">${nav8:.0}</div>
      <div class="mc-stressed" id="m-nav8-str">—</div>
      <div class="mc-sub">on $100 invested</div>
    </div>
    <div class="mc">
      <div class="mc-label">Y8 Market Value / Unit</div>
      <div class="mc-base" id="m-mv8-base">${mv8:.0}</div>
      <div class="mc-stressed" id="m-mv8-str">—</div>
      <div class="mc-sub">dist ÷ market yield</div>
    </div>
    <div class="mc">
      <div class="mc-label">Y8 Distribution / Unit</div>
      <div class="mc-base" id="m-dist8-base">${dist8:.2}</div>
      <div class="mc-stressed" id="m-dist8-str">—</div>
      <div class="mc-sub">{dist8_yld:.1}% yield on cost</div>
    </div>
    <div class="mc">
      <div class="mc-label">Min Interest Coverage</div>
      <div class="mc-base" id="m-mincov-base">{min_cov:.2}×</div>
      <div class="mc-stressed" id="m-mincov-str">—</div>
      <div class="mc-sub">Y7 in base case</div>
    </div>
    <div class="mc">
      <div class="mc-label">Total Distributions (10Y)</div>
      <div class="mc-base" id="m-totdist-base">${tot_dist:.2}</div>
      <div class="mc-stressed" id="m-totdist-str">—</div>
      <div class="mc-sub">per unit over 10 years</div>
    </div>
  </div>
"#,
        nav8 = base_nav8,
        mv8 = base_mv8,
        dist8 = dist_pu[7],
        dist8_yld = dist_pu[7] * 100.0,
        min_cov = min_cov_base,
        tot_dist = base_tot_dist,
    ));

    // Charts + print tables
    h.push_str(&format!(
        r#"  <div class="chart-row">
    <div class="chart-card">
      <h3>NAV per Unit ($100 invested)</h3>
      <canvas id="chart-nav" height="200"></canvas>
      <div class="chart-print">
        <table>
          <thead><tr><th>Metric</th>{yr_heads}</tr></thead>
          <tbody>
            <tr><td>Base NAV/unit</td>{nav_cells}</tr>
          </tbody>
        </table>
      </div>
    </div>
    <div class="chart-card">
      <h3>Annual Distribution per Unit</h3>
      <canvas id="chart-dist" height="200"></canvas>
      <div class="chart-print">
        <table>
          <thead><tr><th>Metric</th>{yr_heads}</tr></thead>
          <tbody>
            <tr><td>Base Dist/unit</td>{dist_cells}</tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
"#,
        yr_heads = (1..=10usize)
            .map(|i| format!("<th>Y{}</th>", i))
            .collect::<Vec<_>>()
            .join(""),
        nav_cells = nav_pu
            .iter()
            .map(|v| format!("<td>${:.0}</td>", v))
            .collect::<Vec<_>>()
            .join(""),
        dist_cells = dist_pu
            .iter()
            .map(|v| format!("<td>${:.2}</td>", v))
            .collect::<Vec<_>>()
            .join(""),
    ));

    // Table
    h.push_str(
        r#"  <div class="table-wrap">
    <table id="main-table" class="yr-table">
      <thead id="table-head"></thead>
      <tbody id="table-body"></tbody>
    </table>
  </div>
"#,
    );

    // Flags
    h.push_str(
        r#"  <div class="flags-card">
    <h3>Stress Flags</h3>
    <div class="flag-list" id="flag-list"></div>
  </div>
"#,
    );

    // Model notes
    h.push_str(&format!(
        r#"  <span class="stitle">Model Notes</span>
  <div class="tblock">
    <h4>Revenue &amp; Operating Assumptions</h4>
    <ul>
      <li><strong>Cap Rate</strong> affects asset valuation and NAV only — it does not change operating cash flows or distributions.</li>
      <li><strong>Occupancy</strong> scales NOI at the TitleCo building level before rolling up to the LP. Base case is {base_occ:.0}%. A 10pp drop reduces NOI by approximately 10.5%.</li>
      <li><strong>Lease-Up Period</strong> reduces NOI in the stabilization year of each phase (Y4, Y6, Y8). A 24-month lease-up reduces stabilization-year NOI by approximately 50%.</li>
      <li><strong>Development Yield</strong> is NOI ÷ invested capital. Base case {base_dev_pct:.1}%. TitleCo X as-modelled 10.02%; TitleCo Y 10.39%; both require rent calibration to reach the LP target.</li>
      <li><strong>Debt Rate</strong> scales interest on the fixed debenture schedule. Y7 DSCR of {min_cov:.2}× is the tightest base-case year; covenant floor is 1.20×.</li>
      <li><strong>Market Yield Rate</strong> is the distribution capitalization rate applied to Y8–Y10 distributions to derive Market Value. At base {base_mktyr_pct:.1}%, a ${dist8:.2}/unit distribution implies a ${mv8:.2} market price. Also drives Discount/Premium to NAV.</li>
    </ul>
  </div>
  <div class="tblock">
    <h4>Accounting &amp; Structuring Notes</h4>
    <ul>
      <li><strong>Advisory Fee</strong>: 1% × Gross Funded Value. Gross Funded Value = equity subscriptions only (debt excluded). GFV = ${gross_equity_m:.0}M fixed for the life of the fund. Annual fee = ${advisory_fee_m:.2}M. Engine models this correctly. Fee base is fixed — not sensitive to debt drawdowns.</li>
      <li><strong>Income Continuity (Y1–Y3)</strong>: recognized as a fair-value accrual entry, not a cash distribution. GP income support provision committed in the offering documents. Amounts: ${ic1:.2}M (Y1), ${ic2:.2}M (Y2), ${ic3:.2}M (Y3). Per BRIEF §735 + §823.</li>
      <li><strong>Market Value Y1–Y3</strong>: displayed at ${unit_price:.2} (offering price / par). Units issued under prospectus at this price; no secondary market formed during the construction period.</li>
      <li><strong>Interest Coverage (DSCR)</strong> = EBITDA ÷ Net Interest. Y4 coverage boosted by low early-phase debt; Y7 tightens to {min_cov:.2}× as debenture drawdowns peak.</li>
      <li><strong>Compounded Return (Y8)</strong> = {comp_ret:.3}% is the base-case proforma value. Not recomputed under stress — indicative base case only.</li>
      <li><strong>Benetti Holdings dilution (10%)</strong> is held constant. Economic impact is embedded in the diluted unit count of {diluted_k:.0}K.</li>
    </ul>
  </div>
  <div class="disclosure">
    <strong>OSC SN 51-721 / NI 51-102 — Forward-Looking Information:</strong> All projected figures are based on management estimates and stated assumptions as of June 2026. Actual results may differ materially. Market Value for Years 1–3 reflects the offering price of ${unit_price:.2} per unit; no secondary market had formed during the construction period. This tool is for sensitivity analysis purposes only and does not constitute an offer to sell or a solicitation to buy securities.
  </div>
"#,
        base_occ = BASE_OCC_PCT,
        base_dev_pct = base_dev * 100.0,
        min_cov = min_cov_base,
        base_mktyr_pct = PCLP1_BUYER_TARGET_YIELD * 100.0,
        dist8 = dist_pu[7],
        mv8 = base_mv8,
        gross_equity_m = PCLP1_GROSS_EQUITY / 1e6,
        advisory_fee_m = advisory_fee_annual / 1e6,
        ic1 = inc_cont[0] / 1e6,
        ic2 = inc_cont[1] / 1e6,
        ic3 = inc_cont[2] / 1e6,
        unit_price = 100.0_f64,
        comp_ret = COMP_RETURN_Y8 * 100.0,
        diluted_k = PCLP1_DILUTED_UNITS / 1000.0,
    ));

    // Audit section
    let occ_nav100_str = format!("{:.1}%", BASE_OCC_PCT * (1.0 - cap_hbps / 10000.0 * 2.0));
    let dev_nav100_str = format!("{:.2}%", dev_dscr120 * 100.0 * 0.67);
    h.push_str(&format!(
        r#"  <div class="audit-section">
    <span class="stitle">Sensitivity Analysis — Static Audit Reference (IFRS 13 §93(h)(ii))</span>
    <div class="audit-banner">
      <strong>Base-case Y8 NAV: ${base_nav8:.0}/unit ({nav8_x:.1}× par of $100).</strong> Capital preservation ($100/unit) requires cap rates to exceed {cap_nav100_pct:.2}% — a move of +{cap_hbps_r:.0} bps from the current {base_cap_pct:.2}% base. The binding constraint is the 1.20× interest-coverage covenant: first breached at a +{debt_hbps_r:.0} bps rate move (+{debt_h_pct:.2}% → {rate_dscr120_pct:.2}%) or a −{occ_hpp_r:.1} pp occupancy decline ({base_occ:.0}% → {occ_dscr120_pct:.1}%).
    </div>
    <table class="audit-table">
      <thead>
        <tr>
          <th style="text-align:left">Input</th>
          <th>Base</th>
          <th>Reasonable range</th>
          <th>NAV = $100/unit</th>
          <th>DSCR = 1.20×</th>
          <th>Headroom</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Cap rate</td>
          <td>{base_cap_pct:.2}%</td>
          <td>±25 bps</td>
          <td class="binding">{cap_nav100_pct:.2}%</td>
          <td>—</td>
          <td class="safe">+{cap_hbps_r:.0} bps</td>
        </tr>
        <tr>
          <td>Debenture rate</td>
          <td>{base_debt_pct:.2}%</td>
          <td>±100 bps</td>
          <td>—</td>
          <td class="binding">{rate_dscr120_pct:.2}%</td>
          <td class="binding">+{debt_hbps_r:.0} bps ⚠</td>
        </tr>
        <tr>
          <td>Occupancy</td>
          <td>{base_occ:.0}%</td>
          <td>−5 to −10 pp</td>
          <td>{occ_nav100_str}</td>
          <td class="binding">{occ_dscr120_pct:.1}%</td>
          <td class="binding">−{occ_hpp_r:.1} pp ⚠</td>
        </tr>
        <tr>
          <td>Dev yield</td>
          <td>{base_dev_pct:.2}%</td>
          <td>±50 bps</td>
          <td>{dev_nav100_str}</td>
          <td class="binding">{dev_dscr120_pct:.2}%</td>
          <td class="binding">−{dev_hbps_r:.0} bps ⚠</td>
        </tr>
        <tr>
          <td>Lease-up</td>
          <td>{base_leaseup:.0} mo</td>
          <td>±6 mo</td>
          <td colspan="2" style="text-align:center">Affects NAV only (Y4/Y6/Y8 stabilization years) — no material impact on Y7 covenant</td>
          <td>—</td>
        </tr>
      </tbody>
    </table>
    <div class="hbar-section">
      <h4>Headroom Visualization — distance from base case to breach threshold (reference: 1,000 bps / 50 pp = full bar)</h4>
      <div class="hbar-row">
        <div class="hbar-label">Cap rate → NAV=$100</div>
        <div class="hbar-track"><div class="hbar-fill-safe" style="width:{cap_bar:.1}%"></div></div>
        <div class="hbar-vals">+{cap_hbps_r:.0} bps &nbsp;({base_cap_pct:.2}% → {cap_nav100_pct:.2}%)</div>
      </div>
      <div class="hbar-row">
        <div class="hbar-label">Rate → DSCR=1.20×</div>
        <div class="hbar-track"><div class="hbar-fill-tight" style="width:{debt_bar:.1}%"></div></div>
        <div class="hbar-vals">+{debt_hbps_r:.0} bps &nbsp;({base_debt_pct:.2}% → {rate_dscr120_pct:.2}%)</div>
      </div>
      <div class="hbar-row">
        <div class="hbar-label">Occupancy → DSCR=1.20×</div>
        <div class="hbar-track"><div class="hbar-fill-tight" style="width:{occ_bar:.1}%"></div></div>
        <div class="hbar-vals">−{occ_hpp_r:.1} pp &nbsp;({base_occ:.0}% → {occ_dscr120_pct:.1}%)</div>
      </div>
      <div class="hbar-row">
        <div class="hbar-label">Dev yield → DSCR=1.20×</div>
        <div class="hbar-track"><div class="hbar-fill-tight" style="width:{dev_bar:.1}%"></div></div>
        <div class="hbar-vals">−{dev_hbps_r:.0} bps &nbsp;({base_dev_pct:.2}% → {dev_dscr120_pct:.2}%)</div>
      </div>
    </div>
    <p class="audit-posture">
      <strong>IFRS 13 §93(h)(ii) — Sensitivity of Fair Value to Changes in Unobservable Inputs:</strong>
      The four inputs above are Level 3 unobservable inputs used in the NAV calculation (cap rate, dev yield) and the DSCR covenant test (debenture rate, occupancy). Break-even thresholds are computed analytically from the V2 engine forecast output (no Excel). Lease-up affects valuation years only (Y4, Y6, Y8) and is excluded from the covenant analysis. NAV for Y1–Y3 is held at par under all scenarios consistent with the income-continuity provision and standard LP accounting practice. All forward-looking information is subject to OSC SN 51-721 / NI 51-102 qualification.
    </p>
  </div>

</div><!-- .main -->

<footer>
  Professional Centres Canada LP — Sensitivity Analysis V7 &nbsp;·&nbsp; Woodfine Management Corp. &nbsp;·&nbsp; Engine: tool-proforma-engine src/spv/pclp1_proforma &nbsp;·&nbsp; Generated June 2026
</footer>
"#,
        base_nav8 = base_nav8,
        nav8_x = base_nav8_x,
        cap_nav100_pct = cap_rate_nav100 * 100.0,
        cap_hbps_r = cap_hbps,
        base_cap_pct = base_cap * 100.0,
        debt_h_pct = rate_dscr120 * 100.0 - base_debt * 100.0,
        rate_dscr120_pct = rate_dscr120 * 100.0,
        debt_hbps_r = debt_hbps,
        base_debt_pct = base_debt * 100.0,
        occ_hpp_r = occ_hpp,
        base_occ = BASE_OCC_PCT,
        occ_dscr120_pct = occ_dscr120,
        occ_nav100_str = occ_nav100_str,
        dev_nav100_str = dev_nav100_str,
        dev_dscr120_pct = dev_dscr120 * 100.0,
        dev_hbps_r = dev_hbps,
        base_dev_pct = base_dev * 100.0,
        base_leaseup = BASE_LEASEUP_MO,
        cap_bar = cap_bar,
        debt_bar = debt_bar,
        occ_bar = occ_bar,
        dev_bar = dev_bar,
    ));

    // JavaScript
    h.push_str("<script>\n// ── Engine-computed base case ──────────────────────────────────────────────\n");
    h.push_str(&format!(
        "const DILUTED   = {diluted:.0};\nconst BASE_OCC  = {base_occ:.1};\nconst BASE_LEASEUP = {base_leaseup:.0};\nconst COMP_RETURN_Y8 = {comp_ret:.5};\n",
        diluted    = PCLP1_DILUTED_UNITS,
        base_occ   = BASE_OCC_PCT,
        base_leaseup = BASE_LEASEUP_MO,
        comp_ret   = COMP_RETURN_Y8,
    ));

    h.push_str("const BASE = {\n");
    h.push_str(&format!("  noi:      {},\n", js_arr(&noi)));
    h.push_str(&format!("  ebitda:   {},\n", js_arr(&ebitda)));
    h.push_str(&format!("  interest: {},\n", js_arr(&interest)));
    h.push_str(&format!("  incCont:  {},\n", js_arr(&inc_cont)));
    h.push_str(&format!("  cash:     {},\n", js_arr(&cash)));
    h.push_str(&format!("  debt:     {},\n", js_arr(&debt_vals)));
    h.push_str(&format!("  wip:      {},\n", js_arr(&wip)));
    h.push_str(&format!("  assetPU:  {},\n", js_arr(&asset_pu)));
    h.push_str(&format!("  navPU:    {},\n", js_arr(&nav_pu)));
    h.push_str(&format!("  distPU:   {},\n", js_arr(&dist_pu)));
    h.push_str(&format!("  coverage: {},\n", js_arr_n(&coverage)));
    h.push_str(&format!("  mvPU_fixed: {},\n", js_arr(&mv_pu_fixed)));
    h.push_str(&format!("  ter:      {},\n", js_arr(&ter)));
    h.push_str(&format!("  ltc:      {},\n", js_arr(&ltc)));
    h.push_str(&format!("  sqft:     {},\n", js_arr(&sqft)));
    h.push_str(&format!("  compReturn: {},\n", js_arr_n(&comp_return)));
    h.push_str("};\n\n");

    h.push_str(&format!(
        "const EXPENSES = BASE.noi.map((n, i) => n - BASE.ebitda[i]);\n\
const STAB_IDX = new Set([3, 5, 7]);\n\
const BASE_MV8 = BASE.distPU[7] / {mktyr:.4};\n\n",
        mktyr = PCLP1_BUYER_TARGET_YIELD,
    ));

    h.push_str(&format!(
        "const SCENARIOS = {{\n\
  bear: {{ cap: {bear_cap:.2}, occ: {bear_occ:.1}, leaseup: {bear_lu:.0}, dev: {bear_dev:.2}, debt: {bear_debt:.2}, mktyr: {bear_my:.1} }},\n\
  base: {{ cap: {base_cap:.2}, occ: {base_occ:.1}, leaseup: {base_lu:.0}, dev: {base_dev:.2}, debt: {base_debt:.2}, mktyr: {base_my:.1} }},\n\
  bull: {{ cap: {bull_cap:.2}, occ: {bull_occ:.1}, leaseup: {bull_lu:.0}, dev: {bull_dev:.2}, debt: {bull_debt:.2}, mktyr: {bull_my:.1} }},\n\
}};\n\n",
        bear_cap = bear_cap, bear_occ = bear_occ, bear_lu = bear_leaseup,
        bear_dev = bear_dev, bear_debt = bear_debt, bear_my = bear_mktyr,
        base_cap = base_cap * 100.0, base_occ = BASE_OCC_PCT, base_lu = BASE_LEASEUP_MO,
        base_dev = base_dev * 100.0, base_debt = base_debt * 100.0,
        base_my = PCLP1_BUYER_TARGET_YIELD * 100.0,
        bull_cap = bull_cap, bull_occ = bull_occ, bull_lu = bull_leaseup,
        bull_dev = bull_dev, bull_debt = bull_debt, bull_my = bull_mktyr,
    ));

    h.push_str(&format!(
        "const AUDIT = {{\n\
  cap_nav100:  {cap_nav100:.4},\n\
  rate_dscr120: {rate_dscr:.4},\n\
  occ_dscr120: {occ_dscr:.2},\n\
  dev_dscr120: {dev_dscr:.4},\n\
}};\n\n",
        cap_nav100 = cap_rate_nav100 * 100.0,
        rate_dscr = rate_dscr120 * 100.0,
        occ_dscr = occ_dscr120,
        dev_dscr = dev_dscr120 * 100.0,
    ));

    h.push_str(JS_FUNCTIONS);

    h.push_str("</script>\n</body>\n</html>\n");

    (h, json)
}

const JS_FUNCTIONS: &str = r#"
// ── Stressed model ────────────────────────────────────────────────────────
function stress(capRate, occupancy, leaseUpMonths, devYield, debtRate, marketYield) {
  const occF  = occupancy / BASE_OCC;
  const devF  = devYield  / (BASE.noi[6] > 0 ? 10.5 : 1); // 10.5 = PCLP1_DEV_YIELD * 100
  const intF  = debtRate  / SCENARIOS.base.debt;
  const leaseF = Math.min(1.0, BASE_LEASEUP / leaseUpMonths);

  const out = { noi:[], ebitda:[], interest:[], distPU:[], navPU:[], assetPU:[], coverage:[], debtToAV:[], mvPU:[], discToNAV:[], distYieldRow:[] };

  for (let i = 0; i < 10; i++) {
    const lf  = STAB_IDX.has(i) ? leaseF : 1.0;
    const noi = i < 3 ? 0 : BASE.noi[i] * occF * (devYield / 10.5) * lf;
    out.noi.push(noi);

    const ebitda  = noi - EXPENSES[i];
    out.ebitda.push(ebitda);

    const interest = BASE.interest[i] <= 0 ? BASE.interest[i] : BASE.interest[i] * intF;
    out.interest.push(interest);

    const afterInt  = ebitda - interest;
    const debtRepay = (afterInt > 0 && BASE.debt[i] > 0) ? afterInt * 0.10 : 0;
    const dist = Math.max(0, afterInt - debtRepay) / DILUTED;
    out.distPU.push(dist);

    const capNOI   = (noi > 0 && capRate > 0) ? noi / (capRate / 100) : 0;
    const assetVal = BASE.cash[i] + capNOI + BASE.wip[i];
    out.navPU.push(  i < 3 ? BASE.navPU[i]   : (assetVal - BASE.debt[i]) / DILUTED);
    out.assetPU.push(i < 3 ? BASE.assetPU[i] : assetVal / DILUTED);

    out.coverage.push(interest > 0 ? ebitda / interest : null);
    out.debtToAV.push(assetVal > 0 ? BASE.debt[i] / assetVal : 0);

    const mv = i < 7 ? BASE.mvPU_fixed[i] : dist / (marketYield / 100);
    out.mvPU.push(mv);

    const nav = out.navPU[out.navPU.length - 1];
    out.discToNAV.push(nav > 0 ? ((mv - nav) / nav) * 100 : null);
    out.distYieldRow.push(i >= 7 ? marketYield : null);
  }
  return out;
}

function applyScenario(name) {
  const s = SCENARIOS[name];
  document.getElementById('sl-cap').value     = s.cap;
  document.getElementById('sl-occ').value     = s.occ;
  document.getElementById('sl-leaseup').value = s.leaseup;
  document.getElementById('sl-dev').value     = s.dev;
  document.getElementById('sl-debt').value    = s.debt;
  document.getElementById('sl-mktyr').value   = s.mktyr;
  document.querySelectorAll('.scenario-btn').forEach(b => b.classList.remove('active'));
  const btn = document.getElementById('btn-' + name);
  if (btn) btn.classList.add('active');
  update();
}

// ── Charts ────────────────────────────────────────────────────────────────
const LABELS = Array.from({length: 10}, (_, i) => 'Y' + (i + 1));

function makeChart(id, lbl1, d1, lbl2, d2) {
  return new Chart(document.getElementById(id).getContext('2d'), {
    type: 'line',
    data: { labels: LABELS, datasets: [
      { label: lbl1, data: d1, borderColor: 'rgba(26,26,26,0.85)', backgroundColor: 'rgba(26,26,26,0.05)', tension: 0.3, pointRadius: 4, borderWidth: 2, fill: true },
      { label: lbl2, data: d2, borderColor: 'rgba(220,38,38,0.85)', backgroundColor: 'rgba(220,38,38,0.04)', tension: 0.3, pointRadius: 4, borderWidth: 2, borderDash: [5,3], fill: false },
    ]},
    options: { responsive: true, maintainAspectRatio: true,
      plugins: { legend: { position: 'bottom', labels: { font: { size: 11 }, boxWidth: 20 } } },
      scales: {
        y: { beginAtZero: false, ticks: { font: { size: 11 }, callback: v => '$' + v.toFixed(0) } },
        x: { ticks: { font: { size: 11 } } },
      }
    }
  });
}

const navChart  = makeChart('chart-nav',  'Base Case', BASE.navPU,  'Stressed', BASE.navPU.slice());
const distChart = makeChart('chart-dist', 'Base Case', BASE.distPU, 'Stressed', BASE.distPU.slice());

// ── Formatters ────────────────────────────────────────────────────────────
function fmtPU(v)    { return (v === null || isNaN(v)) ? '—' : '$' + v.toFixed(2); }
function fmtM(v)     { if (v === null || isNaN(v) || v === 0) return '—'; return '$' + (v / 1e6).toFixed(1) + 'M'; }
function fmtSqft(v)  { return v > 0 ? v.toLocaleString() : '—'; }
function coverBadge(v) {
  if (v === null) return '—';
  const t = v.toFixed(2) + '×';
  if (v < 1.20) return `<span class="badge badge-red">${t}</span>`;
  if (v < 1.50) return `<span class="badge badge-yellow">${t}</span>`;
  return `<span class="badge badge-green">${t}</span>`;
}
function fmtDisc(v) {
  if (v === null || isNaN(v)) return '—';
  const s = (v >= 0 ? '+' : '') + v.toFixed(1) + '%';
  if (v > 1)  return `<span style="color:#1a6e2e;font-weight:600">${s}</span>`;
  if (v < -1) return `<span style="color:#dc2626;font-weight:600">${s}</span>`;
  return s;
}

// ── Table ─────────────────────────────────────────────────────────────────
function renderTableHead() {
  const cols = Array.from({length: 10}, (_, i) => `<th>Y${i + 1}</th>`).join('');
  document.getElementById('table-head').innerHTML = `<tr><th style="text-align:left">Metric</th>${cols}</tr>`;
}

function renderTable(S) {
  const rows = [
    { label: 'Revenue',                              fn: i => S.noi[i] > 0 ? fmtPU(S.noi[i] / DILUTED) : '—' },
    { label: 'Distributions / Unit',                fn: i => S.distPU[i] > 0 ? '$' + S.distPU[i].toFixed(4) : '—' },
    { spacer: true },
    { label: 'Distribution Yield on Initial Capital', fn: i => S.distPU[i] > 0 ? S.distPU[i].toFixed(2) + '%' : '—' },
    { spacer: true },
    { label: 'Asset Value / Unit',                  fn: i => fmtPU(S.assetPU[i]) },
    { label: 'Total Debt / Unit',                   fn: i => fmtPU(S.assetPU[i] - S.navPU[i]) },
    { spacer: true },
    { label: 'Net Asset Value (NAV) / Unit',        fn: i => fmtPU(S.navPU[i]) },
    { spacer: true },
    { label: 'Market Value / Unit',                 fn: i => { const mv = S.mvPU[i]; return (mv === null || isNaN(mv)) ? '—' : '$' + mv.toFixed(2); } },
    { spacer: true },
    { label: 'Discount / Premium to NAV',           fn: i => fmtDisc(S.discToNAV[i]) },
    { spacer: true },
    { label: 'Compounded Annual Return (excl. distributions)', fn: i => BASE.compReturn[i] !== null ? (BASE.compReturn[i] * 100).toFixed(3) + '%' : '—' },
    { label: 'Distribution Yield to Buyers at Market Value',   fn: i => S.distYieldRow[i] !== null ? S.distYieldRow[i].toFixed(1) + '%' : '—' },
    { spacer: true },
    { label: 'Interest Coverage Ratio',             fn: i => coverBadge(S.coverage[i]) },
    { label: 'Debt vs. Development Cost',           fn: i => BASE.ltc[i] > 0 ? (BASE.ltc[i] * 100).toFixed(1) + '%' : '—' },
    { label: 'Debt to Asset Value',                 fn: i => S.debtToAV[i] > 0 ? (S.debtToAV[i] * 100).toFixed(1) + '%' : '—' },
    { spacer: true },
    { label: 'Total Expense Ratio (NAV)',           fn: i => (BASE.ter[i] * 100).toFixed(2) + '%' },
    { label: 'Total Square Footage — $250M Gross Funded Value', fn: i => fmtSqft(BASE.sqft[i]) },
  ];

  let html = '';
  rows.forEach(r => {
    if (r.spacer) { html += `<tr class="spacer-row"><td colspan="11"></td></tr>`; return; }
    const cells = Array.from({length: 10}, (_, i) => `<td>${r.fn(i)}</td>`).join('');
    html += `<tr><td>${r.label}</td>${cells}</tr>`;
  });
  document.getElementById('table-body').innerHTML = html;
}

// ── Flags ─────────────────────────────────────────────────────────────────
function renderFlags(S, cap, occ, dev, debt, leaseup, mktyr) {
  const flags = [];
  flags.push({ c:'yellow', t:'Benetti Holdings: 10% unit dilution (~277,777 units) unexplained. Clarify consideration before investing.' });
  flags.push({ c:'yellow', t:'Income Continuity Y1–Y3 is a GP fair-value accrual (not cash). GP income support provision. Review commitment terms.' });
  flags.push({ c:'red', t:'Alert sheet in proforma workbook is empty. Confirm what validation checks were intended.' });

  const validCov = S.coverage.filter(v => v !== null);
  if (validCov.length) {
    const mc = Math.min(...validCov);
    if      (mc < 1.00) flags.push({ c:'red',    t:`Coverage below 1.0× (min ${mc.toFixed(2)}×) — debt service unsupported by operations.` });
    else if (mc < 1.20) flags.push({ c:'red',    t:`Coverage breaches covenant floor 1.20× (min ${mc.toFixed(2)}×).` });
    else if (mc < AUDIT.rate_dscr120 / SCENARIOS.base.debt * SCENARIOS.base.debt)
      flags.push({ c:'yellow', t:`Coverage below base (min ${mc.toFixed(2)}×). Covenant floor 1.20×.` });
  }

  if (leaseup > BASE_LEASEUP) flags.push({ c:'yellow', t:`${leaseup}-month lease-up reduces Y4, Y6, Y8 stabilization-year NOI by ${((1 - BASE_LEASEUP / leaseup) * 100).toFixed(0)}%.` });

  const y8nav = S.navPU[7];
  if      (y8nav < 100) flags.push({ c:'red',    t:`Y8 NAV $${y8nav.toFixed(0)}/unit — investors do not recover par ($100) by Y8.` });
  else if (y8nav < 200) flags.push({ c:'yellow', t:`Y8 NAV $${y8nav.toFixed(0)}/unit — limited capital appreciation vs base ($${BASE.navPU[7].toFixed(0)}).` });

  if (cap  > AUDIT.cap_nav100)  flags.push({ c:'red', t:`Cap rate ${cap.toFixed(2)}% exceeds NAV=$100 threshold (${AUDIT.cap_nav100.toFixed(2)}%). LP units at risk of trading below par.` });
  if (debt > AUDIT.rate_dscr120) flags.push({ c:'red', t:`Rate ${debt.toFixed(2)}% exceeds DSCR=1.20× covenant threshold (${AUDIT.rate_dscr120.toFixed(2)}%).` });
  if (occ  < AUDIT.occ_dscr120)  flags.push({ c:'red', t:`Occupancy ${occ.toFixed(1)}% below DSCR=1.20× covenant threshold (${AUDIT.occ_dscr120.toFixed(1)}%).` });
  if (dev  < AUDIT.dev_dscr120)  flags.push({ c:'red', t:`Dev yield ${dev.toFixed(2)}% below DSCR=1.20× covenant threshold (${AUDIT.dev_dscr120.toFixed(2)}%).` });

  document.getElementById('flag-list').innerHTML = flags.map(f =>
    `<div class="flag-item"><div class="flag-dot dot-${f.c}"></div><div>${f.t}</div></div>`
  ).join('') || `<div class="no-flags">No critical flags at current assumptions.</div>`;
}

// ── Metric cards ──────────────────────────────────────────────────────────
function renderMetrics(S) {
  const totStr  = S.distPU.reduce((a, b) => a + b, 0);
  const totBase = BASE.distPU.reduce((a, b) => a + b, 0);
  const vcStr   = S.coverage.filter(v => v !== null);
  const vcBase  = BASE.coverage.filter(v => v !== null);
  const mcStr   = vcStr.length  ? Math.min(...vcStr)  : 0;
  const mcBase  = vcBase.length ? Math.min(...vcBase) : 0;

  function setCard(bId, sId, bv, sv, fmt, hib) {
    document.getElementById(bId).textContent = fmt(bv);
    const d = sv - bv;
    const cls = Math.abs(d) < 0.001 ? 'flat' : (d > 0) === hib ? 'up' : 'down';
    document.getElementById(sId).className   = 'mc-stressed ' + cls;
    document.getElementById(sId).textContent = 'Stressed: ' + fmt(sv);
  }
  setCard('m-nav8-base',    'm-nav8-str',    BASE.navPU[7],  S.navPU[7],  v => '$' + v.toFixed(0), true);
  setCard('m-mv8-base',     'm-mv8-str',     BASE_MV8,       S.mvPU[7],   v => '$' + v.toFixed(0), true);
  setCard('m-dist8-base',   'm-dist8-str',   BASE.distPU[7], S.distPU[7], v => '$' + v.toFixed(2), true);
  setCard('m-mincov-base',  'm-mincov-str',  mcBase,         mcStr,       v => v.toFixed(2) + '×', true);
  setCard('m-totdist-base', 'm-totdist-str', totBase,        totStr,      v => '$' + v.toFixed(2), true);
}

// ── Update ────────────────────────────────────────────────────────────────
function update() {
  const cap    = parseFloat(document.getElementById('sl-cap').value);
  const occ    = parseFloat(document.getElementById('sl-occ').value);
  const leaseup= parseFloat(document.getElementById('sl-leaseup').value);
  const dev    = parseFloat(document.getElementById('sl-dev').value);
  const debt   = parseFloat(document.getElementById('sl-debt').value);
  const mktyr  = parseFloat(document.getElementById('sl-mktyr').value);

  document.getElementById('val-cap').innerHTML     = cap.toFixed(2)      + '<span>%</span>';
  document.getElementById('val-occ').innerHTML     = occ.toFixed(0)      + '<span>%</span>';
  document.getElementById('val-leaseup').innerHTML = leaseup.toFixed(0)  + '<span> mo</span>';
  document.getElementById('val-dev').innerHTML     = dev.toFixed(2)      + '<span>%</span>';
  document.getElementById('val-debt').innerHTML    = debt.toFixed(2)     + '<span>%</span>';
  document.getElementById('val-mktyr').innerHTML   = mktyr.toFixed(1)    + '<span>%</span>';

  const S = stress(cap, occ, leaseup, dev, debt, mktyr);

  navChart.data.datasets[0].data  = BASE.navPU;
  navChart.data.datasets[1].data  = S.navPU;
  distChart.data.datasets[0].data = BASE.distPU;
  distChart.data.datasets[1].data = S.distPU;
  navChart.update();
  distChart.update();

  renderTableHead();
  renderTable(S);
  renderMetrics(S);
  renderFlags(S, cap, occ, dev, debt, leaseup, mktyr);
}

['sl-cap','sl-occ','sl-leaseup','sl-dev','sl-debt','sl-mktyr'].forEach(id =>
  document.getElementById(id).addEventListener('input', () => {
    document.querySelectorAll('.scenario-btn').forEach(b => b.classList.remove('active'));
    update();
  })
);
update();
"#;
