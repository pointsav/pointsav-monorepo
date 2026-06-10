// Professional Centres Canada LP — Sensitivity Analysis V8 (three-mode dynamic engine)
//
// Every scenario is computed in Rust by the V3 forecast engine and embedded as data;
// the page performs NO financial recomputation — it only selects among pre-computed runs.
//
// Three professionally-distinct constructions (terminology confirmed against Big-4
// assurance vocabulary):
//   • Management Response Scenario   — ManagedDownside: constrained issuance, no breach
//   • Single-Input Sensitivity       — SingleInputStress: ceteris paribus, IFRS 13 §93(h)(ii)
//   • Covenant Cure (Corrective Disposition) — CovenantCure: minimum disposition at stressed value
//
// "Hard Reset" / "forced liquidation" appear nowhere in any output, by design.

use crate::spv::pclp1_proforma::{
    self as eng, ForecastParams, ModelMode, PCLP1_CAP_RATE, PCLP1_DEBT_RATE_DEBENTURE,
    PCLP1_DEV_YIELD, PCLP1_DILUTED_UNITS, PCLP1_GROSS_EQUITY,
};
use serde_json::{json, Value};

struct Scn {
    key: &'static str,
    label: &'static str,
    p: ForecastParams,
}

fn base_with(mode: ModelMode) -> ForecastParams {
    ForecastParams {
        mode,
        ..Default::default()
    }
}

/// Build the embeddable JSON object for one scenario from a live engine run.
fn scn_json(s: &Scn) -> Value {
    let (years, meta) = eng::forecast_full(&s.p);
    json!({
        "key": s.key,
        "label": s.label,
        "params": {
            "occupancy_pct": s.p.occupancy_pct,
            "dev_yield": s.p.dev_yield,
            "debt_rate": s.p.debt_rate,
            "cap_rate": s.p.cap_rate,
            "lease_up_months": s.p.lease_up_months,
            "market_yield": s.p.market_yield,
        },
        "meta": meta,
        "years": &years[1..=10],
    })
}

/// Returns (sensitivity_html, audit_json).
pub fn render() -> (String, String) {
    // ── Scenario matrix ──────────────────────────────────────────────────────
    // Mode 1 — Management Response (ManagedDownside): the LP adapts issuance.
    let managed: Vec<Scn> = vec![
        Scn { key: "base", label: "Base", p: base_with(ModelMode::ManagedDownside) },
        Scn { key: "rate_60", label: "Coupon 6.0%", p: ForecastParams { debt_rate: 0.060, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "rate_70", label: "Coupon 7.0%", p: ForecastParams { debt_rate: 0.070, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "rate_80", label: "Coupon 8.0%", p: ForecastParams { debt_rate: 0.080, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "occ_90", label: "Occupancy 90%", p: ForecastParams { occupancy_pct: 0.90, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "occ_85", label: "Occupancy 85%", p: ForecastParams { occupancy_pct: 0.85, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "lease_18", label: "Lease-up 18 mo", p: ForecastParams { lease_up_months: 18.0, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "lease_24", label: "Lease-up 24 mo", p: ForecastParams { lease_up_months: 24.0, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "bear", label: "Bear (combined)", p: ForecastParams { occupancy_pct: 0.88, dev_yield: 0.098, debt_rate: 0.070, cap_rate: 0.075, lease_up_months: 18.0, ..base_with(ModelMode::ManagedDownside) } },
        Scn { key: "bull", label: "Bull", p: ForecastParams { debt_rate: 0.045, cap_rate: 0.0575, ..base_with(ModelMode::ManagedDownside) } },
    ];

    // Mode 2 — Single-Input Sensitivity (SingleInputStress): ceteris paribus.
    let stat: Vec<Scn> = vec![
        Scn { key: "base", label: "Base", p: base_with(ModelMode::SingleInputStress) },
        Scn { key: "rate_70", label: "Coupon 7.0%", p: ForecastParams { debt_rate: 0.070, ..base_with(ModelMode::SingleInputStress) } },
        Scn { key: "rate_90", label: "Coupon 9.0%", p: ForecastParams { debt_rate: 0.090, ..base_with(ModelMode::SingleInputStress) } },
        Scn { key: "occ_85", label: "Occupancy 85%", p: ForecastParams { occupancy_pct: 0.85, ..base_with(ModelMode::SingleInputStress) } },
        Scn { key: "dev_95", label: "Dev yield 9.5%", p: ForecastParams { dev_yield: 0.095, ..base_with(ModelMode::SingleInputStress) } },
    ];

    // Mode 3 — Covenant Cure (CovenantCure): combined shocks AFTER the portfolio is
    // built (Y8, all phases stabilised). Mid-construction shocks (Y6/Y7) are not
    // disposition cases — the binding response there is the Management Response mode
    // (halt issuance), since debt funding work-in-progress cannot be cured by selling
    // completed buildings. A clean curable→incurable spread:
    let cure: Vec<Scn> = vec![
        Scn { key: "cc_moderate", label: "Coupon+cap+occ", p: ForecastParams { occupancy_pct: 0.80, debt_rate: 0.085, cap_rate: 0.095, mode: ModelMode::CovenantCure { shock_year: 8 }, ..Default::default() } },
        Scn { key: "cc_coupon_led", label: "Coupon-led 9.0%", p: ForecastParams { occupancy_pct: 0.82, debt_rate: 0.090, cap_rate: 0.090, mode: ModelMode::CovenantCure { shock_year: 8 }, ..Default::default() } },
        Scn { key: "cc_deep", label: "Deep combined", p: ForecastParams { occupancy_pct: 0.78, debt_rate: 0.088, cap_rate: 0.092, mode: ModelMode::CovenantCure { shock_year: 8 }, ..Default::default() } },
        Scn { key: "cc_cap_blowout", label: "Cap blowout (incurable)", p: ForecastParams { occupancy_pct: 0.78, debt_rate: 0.090, cap_rate: 0.105, mode: ModelMode::CovenantCure { shock_year: 8 }, ..Default::default() } },
        Scn { key: "cc_income_collapse", label: "Income collapse (incurable)", p: ForecastParams { occupancy_pct: 0.50, dev_yield: 0.07, mode: ModelMode::CovenantCure { shock_year: 8 }, ..Default::default() } },
    ];

    let managed_json: Vec<Value> = managed.iter().map(scn_json).collect();
    let stat_json: Vec<Value> = stat.iter().map(scn_json).collect();
    let cure_json: Vec<Value> = cure.iter().map(scn_json).collect();

    // Base ManagedDownside series — pre-shock reference overlay for Covenant Cure charts.
    let (base_years, _) = eng::forecast_full(&ForecastParams::default());
    let base_nav: Vec<f64> = base_years[1..=10].iter().map(|y| y.nav_per_unit).collect();
    let base_cov: Vec<f64> = base_years[1..=10].iter().map(|y| y.interest_coverage).collect();

    // ── Type-2 break-even thresholds (the IFRS 13 §93(h)(ii) disclosure) ──────
    let yr7 = &base_years[7];
    let yr8 = &base_years[8];
    let base_cap = PCLP1_CAP_RATE;
    let base_dev = PCLP1_DEV_YIELD;
    let base_debt = PCLP1_DEBT_RATE_DEBENTURE;

    let cap_denom = 100.0 * PCLP1_DILUTED_UNITS - yr8.ending_cash - yr8.wip + yr8.closing_debt;
    let cap_rate_nav100 = yr8.net_proceeds_from_ops / cap_denom;
    let rate_dscr120 = base_debt * yr7.ebitda / (1.20 * yr7.net_interest);
    let expenses_y7 = yr7.net_proceeds_from_ops - yr7.ebitda;
    // Occupancy break-even as a multiple of modelled stabilised NOI (base = 100%).
    let occ_dscr120 = (1.20 * yr7.net_interest + expenses_y7) / yr7.net_proceeds_from_ops * 100.0;
    let dev_dscr120 =
        base_dev * (1.20 * yr7.net_interest + expenses_y7) / yr7.net_proceeds_from_ops;

    let base_nav8 = base_years[8].nav_per_unit;
    let base_min_cov = base_cov
        .iter()
        .filter(|&&c| c > 0.0)
        .fold(f64::INFINITY, |a, &b| a.min(b));

    // ── DATA object embedded for the page ────────────────────────────────────
    let data = json!({
        "managed": managed_json,
        "static": stat_json,
        "cure": cure_json,
        "baseNav": base_nav,
        "baseCov": base_cov.iter().map(|&c| if c > 0.0 { json!(c) } else { Value::Null }).collect::<Vec<Value>>(),
    });
    let data_str = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    // ── Audit JSON (second return value) ─────────────────────────────────────
    let audit = json!({
        "metadata": {
            "entity": "Professional Centres Canada LP (PCLP 1)",
            "generated": "2026-06-07",
            "engine": "tool-proforma-engine src/spv/pclp1_proforma (V3 three-mode)",
            "version": "V8",
            "covenant_floor_dscr": 1.20,
            "capital_preservation_floor_nav_per_unit": 100.0,
            "scenario_constructions": {
                "management_response": "ManagedDownside — constrained issuance; MD&A forward-looking",
                "single_input_sensitivity": "SingleInputStress — ceteris paribus; IFRS 13 para 93(h)(ii)",
                "covenant_cure": "CovenantCure — minimum corrective disposition at stressed value; MD&A; orderly-transaction caveat applies"
            },
            "curability_rule": "Disposition cures the coverage covenant only where retained-asset yield no longer covers covenant-scaled debt service (EBITDA > cap_rate x debt). Cheap-debt income-collapse breaches are incurable by disposition.",
            "nav_floor_note": "NAV = market value minus debt is invariant to the sale fraction at market value; the NAV floor is never restorable by disposition. The binding cure mechanism is always the coverage covenant."
        },
        "base_case": {
            "diluted_units": PCLP1_DILUTED_UNITS,
            "y8_nav_per_unit": base_nav8,
            "min_interest_coverage": base_min_cov,
            "years": &base_years[1..=10]
        },
        "single_input_thresholds_ifrs13_93h_ii": {
            "cap_rate_nav_par": { "value_pct": cap_rate_nav100 * 100.0, "headroom_bps": (cap_rate_nav100 - base_cap) * 10000.0, "binding_year": 8 },
            "debenture_rate_dscr120": { "value_pct": rate_dscr120 * 100.0, "headroom_bps": (rate_dscr120 - base_debt) * 10000.0, "binding_year": 7 },
            "occupancy_dscr120_pct_of_stabilised": { "value_pct": occ_dscr120, "headroom_pp": 100.0 - occ_dscr120, "binding_year": 7 },
            "dev_yield_dscr120": { "value_pct": dev_dscr120 * 100.0, "headroom_bps": (base_dev - dev_dscr120) * 10000.0, "binding_year": 7 },
            "reasonably_possible_range": "±25 bps (cap/dev/rate); movements beyond this are stress scenarios, not part of the para 93(h)(ii) disclosure"
        },
        "scenarios": {
            "management_response": managed.iter().map(scn_json).collect::<Vec<Value>>(),
            "single_input_sensitivity": stat.iter().map(scn_json).collect::<Vec<Value>>(),
            "covenant_cure": cure.iter().map(scn_json).collect::<Vec<Value>>()
        }
    });
    let json_out = serde_json::to_string_pretty(&audit).unwrap_or_else(|_| "{}".to_string());

    // ── HTML ─────────────────────────────────────────────────────────────────
    let mut h = String::with_capacity(160 * 1024);
    h.push_str(HTML_HEAD);

    h.push_str(&format!(
        r#"<header>
  <h1>Professional Centres Canada LP — Sensitivity Analysis</h1>
  <p>Woodfine Management Corp. &nbsp;·&nbsp; ${gross_m:.0}M Equity ({diluted_k:.0}K units at $100) &nbsp;·&nbsp; June 2026 &nbsp;·&nbsp; V8 — Three-mode dynamic engine</p>
</header>
<div class="main">
"#,
        gross_m = PCLP1_GROSS_EQUITY / 1e6,
        diluted_k = PCLP1_DILUTED_UNITS / 1000.0,
    ));

    // Mode selector + scenario row + active-mode description
    h.push_str(
        r#"  <div class="controls">
    <span class="stitle">Scenario Construction</span>
    <div class="mode-selector">
      <button class="mode-btn active" data-mode="managed" onclick="selectMode('managed')">Management Response</button>
      <button class="mode-btn" data-mode="static" onclick="selectMode('static')">Single-Input Stress</button>
      <button class="mode-btn" data-mode="cure" onclick="selectMode('cure')">Covenant Cure</button>
    </div>
    <p id="mode-desc" class="mode-desc"></p>
    <div class="scenario-row" id="scn-row"></div>
  </div>

  <div id="mode-card"></div>

  <div class="chart-row">
    <div class="chart-card">
      <h3>NAV per Unit ($100 invested)</h3>
      <canvas id="chart-nav" height="200"></canvas>
    </div>
    <div class="chart-card">
      <h3>Interest Coverage (DSCR) — covenant floor 1.20×</h3>
      <canvas id="chart-cov" height="200"></canvas>
    </div>
  </div>

  <div class="table-wrap">
    <table class="yr-table">
      <thead id="table-head"></thead>
      <tbody id="table-body"></tbody>
    </table>
  </div>
"#,
    );

    // Model notes
    h.push_str(
        r#"  <span class="stitle">How the Three Constructions Differ</span>
  <div class="tblock">
    <h4>1 · Management Response Scenario</h4>
    <p>The LP re-runs the proforma from inception under the stressed inputs and constrains
    debenture issuance and development scale so interest coverage never falls below the 1.20×
    covenant. Fewer buildings are financed; returns decline; no covenant is breached and no
    assets are sold. This is a forward-looking illustration of management's intended response —
    actual responses may differ. Reported in the MD&amp;A, not as an IFRS 13 measurement input.</p>
  </div>
  <div class="tblock">
    <h4>2 · Single-Input Sensitivity / Break-even Analysis</h4>
    <p>One unobservable Level 3 input is moved with all else held constant (ceteris paribus).
    This is the IFRS 13 §93(h)(ii) disclosure: it isolates the measurement sensitivity of fair
    value to each input and locates the exact breach threshold. No management response is
    assumed. Movements within ±25 bps are the reasonably-possible range; larger moves are stress
    scenarios beyond that range and are labelled as such.</p>
  </div>
  <div class="tblock">
    <h4>3 · Covenant Cure (Corrective Disposition) Scenario</h4>
    <p>The portfolio is built out under base conditions; a combined shock then lands after
    construction. The LP disposes of the <em>minimum</em> generating-asset fraction, valued at the
    <em>stressed</em> cap rate, required to restore the 1.20× coverage covenant. Disposition cures
    the covenant only where the retained-asset yield no longer covers covenant-scaled debt service
    (high financing-cost regimes); a cheap-debt income-collapse breach is incurable by disposition
    and is flagged as such. The NAV-per-unit floor is never restorable by disposition — selling at
    market value leaves NAV unchanged — so post-cure NAV is reported at its stressed level. Sale at
    a stressed cap rate may not constitute an orderly transaction (IFRS 13 §15–§24); presented for
    covenant-remediation illustration only, not as a fair-value measurement.</p>
  </div>
"#,
    );

    // Audit section — Single-Input break-even table (Type 2 = the §93(h)(ii) disclosure)
    h.push_str(&format!(
        r#"  <div class="audit-section">
    <span class="stitle">Single-Input Sensitivity — Break-even Reference (IFRS 13 §93(h)(ii))</span>
    <div class="audit-banner">
      <strong>Base-case Y8 NAV: ${base_nav8:.0}/unit.</strong> Capital preservation ($100/unit) requires the cap rate to exceed {cap_p:.2}% — a +{cap_bps:.0} bps move from the {base_cap_p:.2}% base. The binding covenant is the 1.20× interest-coverage test, first breached at a +{rate_bps:.0} bps coupon move (to {rate_p:.2}%), a decline to {occ_p:.1}% of modelled stabilised occupancy, or a dev-yield fall of {dev_bps:.0} bps (to {dev_p:.2}%). This table is the §93(h)(ii) measurement-sensitivity disclosure; the Management Response and Covenant Cure constructions are MD&amp;A forward-looking analyses, not measurement inputs.
    </div>
    <table class="audit-table">
      <thead>
        <tr><th style="text-align:left">Input</th><th>Base</th><th>Reasonably possible</th><th>NAV = $100/unit</th><th>DSCR = 1.20×</th><th>Move to breach</th></tr>
      </thead>
      <tbody>
        <tr><td>Cap rate</td><td>{base_cap_p:.2}%</td><td>±25 bps</td><td class="binding">{cap_p:.2}%</td><td>—</td><td class="safe">+{cap_bps:.0} bps (stress)</td></tr>
        <tr><td>Debenture rate</td><td>{base_debt_p:.2}%</td><td>±25 bps</td><td>—</td><td class="binding">{rate_p:.2}%</td><td class="binding">+{rate_bps:.0} bps (stress) ⚠</td></tr>
        <tr><td>Occupancy</td><td>100%</td><td>−5 to −10 pp</td><td>—</td><td class="binding">{occ_p:.1}%</td><td class="binding">−{occ_pp:.1} pp (stress) ⚠</td></tr>
        <tr><td>Dev yield</td><td>{base_dev_p:.2}%</td><td>±25 bps</td><td>—</td><td class="binding">{dev_p:.2}%</td><td class="binding">−{dev_bps:.0} bps (stress) ⚠</td></tr>
      </tbody>
    </table>
    <p class="audit-posture">
      <strong>IFRS 13 §93(h)(ii):</strong> the four inputs above are Level 3 unobservable inputs in the NAV calculation (cap rate, dev yield) and the coverage covenant test (debenture rate, occupancy). Break-even thresholds are computed analytically from the engine forecast (no Excel). Movements beyond the ±25 bps reasonably-possible range are stress scenarios and do not form part of the §93(h)(ii) disclosure. The Covenant Cure construction relies on disposition at stressed cap rates, which may not constitute an orderly transaction between market participants (IFRS 13 §15–§24) and is presented for covenant-remediation illustration only. All forward-looking information is subject to OSC SN 51-721 / NI 51-102 qualification.
    </p>
  </div>

</div><!-- .main -->

<footer>
  Professional Centres Canada LP — Sensitivity Analysis V8 &nbsp;·&nbsp; Woodfine Management Corp. &nbsp;·&nbsp; Engine: tool-proforma-engine src/spv/pclp1_proforma (V3 three-mode) &nbsp;·&nbsp; Generated June 2026
</footer>
"#,
        base_nav8 = base_nav8,
        cap_p = cap_rate_nav100 * 100.0,
        cap_bps = (cap_rate_nav100 - base_cap) * 10000.0,
        base_cap_p = base_cap * 100.0,
        rate_p = rate_dscr120 * 100.0,
        rate_bps = (rate_dscr120 - base_debt) * 10000.0,
        base_debt_p = base_debt * 100.0,
        occ_p = occ_dscr120,
        occ_pp = 100.0 - occ_dscr120,
        base_dev_p = base_dev * 100.0,
        dev_p = dev_dscr120 * 100.0,
        dev_bps = (base_dev - dev_dscr120) * 10000.0,
    ));

    // Data + app JS
    h.push_str("<script>\nconst DATA = ");
    h.push_str(&data_str);
    h.push_str(";\n");
    h.push_str(JS_APP);
    h.push_str("</script>\n</body>\n</html>\n");

    (h, json_out)
}

const HTML_HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Professional Centres Canada LP — Sensitivity Analysis V8</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
<style>
:root {
  --ink:#1a1a1a; --ink2:#555; --ink3:#888; --ink4:#aaa;
  --bg:#fff; --bg2:#fafafa; --bg3:#f5f5f5; --border:#e5e5e5; --row-sep:#ebebeb;
  --pos:#1a6e2e; --warn:#b45309; --accent:#1a1a1a; --page-w:1200px;
  --font:system-ui,-apple-system,sans-serif;
}
* { box-sizing:border-box; margin:0; padding:0; }
body { font-family:var(--font); font-size:13px; background:var(--bg); color:var(--ink); line-height:1.6; }
header { background:var(--ink); color:#fff; padding:16px 32px; display:flex; align-items:baseline; gap:24px; }
header h1 { font-size:16px; font-weight:600; white-space:nowrap; }
header p { font-size:12px; color:#9ca3af; }
.main { max-width:var(--page-w); margin:0 auto; padding:24px 32px; }
.stitle { display:block; font-size:10px; font-weight:700; text-transform:uppercase; letter-spacing:.08em; color:var(--ink3); margin:24px 0 10px; }
.controls { background:var(--bg); border:1px solid var(--border); border-radius:8px; padding:20px 24px; margin-bottom:16px; }
.mode-selector { display:flex; gap:0; border:1.5px solid var(--ink); border-radius:6px; overflow:hidden; width:fit-content; margin-bottom:12px; }
.mode-btn { padding:8px 20px; border:none; background:var(--bg); font-size:12.5px; font-weight:600; cursor:pointer; color:var(--ink2); transition:all .15s; border-right:1px solid var(--border); }
.mode-btn:last-child { border-right:none; }
.mode-btn:hover { background:var(--bg3); }
.mode-btn.active { background:var(--ink); color:#fff; }
.mode-desc { font-size:12.5px; color:var(--ink2); margin-bottom:14px; max-width:880px; }
.scenario-row { display:flex; align-items:center; gap:8px; flex-wrap:wrap; }
.scenario-btn { padding:5px 14px; border:1.5px solid var(--border); border-radius:5px; background:var(--bg); font-size:12px; font-weight:600; cursor:pointer; color:var(--ink2); transition:all .15s; }
.scenario-btn:hover { background:var(--bg3); }
.scenario-btn.active { background:var(--ink); color:#fff; border-color:var(--ink); }
.adapt-card, .cure-card { background:var(--bg2); border:1px solid var(--border); border-radius:8px; padding:20px 24px; margin-bottom:16px; }
.adapt-card.breach { border-color:#dc2626; background:#fff7f7; }
.cure-card { border-left:4px solid var(--ink); }
.adapt-card h3, .cure-card h3 { font-size:14px; font-weight:700; color:var(--ink); margin-bottom:6px; }
.adapt-card h3 .tag, .cure-card h3 .tag { font-size:10px; font-weight:600; color:var(--ink3); border:1px solid var(--border); border-radius:3px; padding:1px 6px; margin-left:6px; vertical-align:middle; }
.param-line { font-size:11.5px; color:var(--ink3); margin-bottom:14px; }
.adapt-grid, .cure-grid { display:grid; grid-template-columns:repeat(6,1fr); gap:16px; margin-bottom:14px; }
.adapt-grid { grid-template-columns:repeat(5,1fr); }
.kv { display:flex; flex-direction:column; gap:3px; }
.kv .big { font-size:24px; font-weight:700; color:var(--ink); line-height:1; }
.kv .big.down { color:#dc2626; }
.kv label { font-size:11px; color:var(--ink3); }
.fwd, .cure-card p { font-size:12.5px; color:var(--ink2); line-height:1.65; }
.caveat { margin-top:12px; padding-top:10px; border-top:1px solid var(--border); font-size:11.5px; color:var(--ink3); line-height:1.6; }
.caveat strong { color:var(--ink2); }
.incurable-banner { background:#fee2e2; border:1px solid #fca5a5; color:#b91c1c; border-radius:6px; padding:10px 14px; font-size:12px; font-weight:600; margin:6px 0 12px; line-height:1.55; }
.chart-row { display:grid; grid-template-columns:1fr 1fr; gap:14px; margin-bottom:16px; }
.chart-card { background:var(--bg); border:1px solid var(--border); border-radius:8px; padding:18px 20px; }
.chart-card h3 { font-size:10px; font-weight:700; text-transform:uppercase; letter-spacing:.07em; color:var(--ink3); margin-bottom:12px; }
.table-wrap { background:var(--bg); border:1px solid var(--border); border-radius:8px; overflow-x:auto; margin-bottom:16px; }
.yr-table { width:100%; border-collapse:collapse; font-size:12px; }
.yr-table thead th { background:var(--ink); color:#fff; padding:9px 12px; text-align:right; font-weight:500; font-size:11px; white-space:nowrap; }
.yr-table thead th:first-child { text-align:left; }
.yr-table tbody td { padding:7px 12px; text-align:right; border-bottom:1px solid var(--row-sep); white-space:nowrap; }
.yr-table tbody td:first-child { text-align:left; font-weight:600; color:#333; }
.yr-table tbody td.binding { color:#dc2626; font-weight:700; }
.yr-table tbody tr:hover td { background:#f8f9ff; }
.tblock { border-left:3px solid var(--ink); padding:12px 16px; margin-bottom:14px; background:var(--bg2); border-radius:0 4px 4px 0; }
.tblock h4 { font-size:10px; font-weight:700; text-transform:uppercase; letter-spacing:.07em; color:var(--ink); margin-bottom:7px; }
.tblock p { font-size:12.5px; color:var(--ink2); line-height:1.65; }
.tblock em { font-style:italic; color:var(--ink); }
.audit-section { background:var(--bg); border:1px solid var(--border); border-radius:8px; padding:22px 26px; margin-top:24px; }
.audit-banner { border-left:4px solid var(--ink); padding:12px 16px; background:var(--bg2); border-radius:0 6px 6px 0; margin-bottom:20px; font-size:12.5px; line-height:1.65; color:var(--ink2); }
.audit-banner strong { color:var(--ink); }
.audit-table { width:100%; border-collapse:collapse; font-size:12px; margin-bottom:18px; }
.audit-table th { background:var(--ink); color:#fff; padding:8px 12px; text-align:right; font-weight:500; font-size:11px; }
.audit-table th:first-child { text-align:left; }
.audit-table td { padding:7px 12px; text-align:right; border-bottom:1px solid var(--row-sep); }
.audit-table td:first-child { text-align:left; font-weight:600; }
.audit-table .binding { color:#dc2626; font-weight:700; }
.audit-table .safe { color:var(--pos); font-weight:600; }
.audit-posture { margin-top:8px; padding-top:12px; border-top:1px solid var(--border); font-size:11.5px; color:var(--ink3); line-height:1.65; }
footer { max-width:var(--page-w); margin:16px auto; padding:0 32px 24px; border-top:1px solid var(--border); font-size:11.5px; color:var(--ink3); line-height:1.65; padding-top:14px; }
@media print {
  @page { size:letter landscape; margin:0.75in 0.6in 0.85in; }
  body { font-size:11px; background:#fff !important; }
  .main { max-width:100%; padding:0; }
  header { padding:10px 0; }
  .mode-selector, .scenario-row, .chart-card canvas { display:none !important; }
  .chart-card::after { content:"(interactive chart — see data table below)"; font-size:10px; color:var(--ink3); }
  .yr-table { display:table !important; }
  .table-wrap { overflow:visible !important; }
  thead { display:table-header-group; }
  tr { page-break-inside:avoid; }
  .audit-section { page-break-before:always; }
  .stitle, h3, h4 { page-break-after:avoid; }
  .adapt-card, .cure-card, .chart-card, .table-wrap, .audit-section, .tblock { box-shadow:none; border-color:#ccc; }
}
</style>
</head>
<body>
"#;

const JS_APP: &str = r#"
const FLOOR = 1.20, PAR = 100.0;
let curMode = 'managed', idx = 0, navChart = null, covChart = null;

const MODE_DESC = {
  managed: "The LP re-runs the proforma from inception under stressed inputs and constrains debenture issuance to hold the coverage covenant. Returns fall; no breach; no sales. (MD&A — forward-looking.)",
  static:  "One unobservable input is moved with all else held constant (IFRS 13 §93(h)(ii)). Shows the unmitigated effect and the exact breach threshold — no management response.",
  cure:    "The portfolio is built under base conditions, then a combined shock lands after construction. The LP sells the minimum portfolio fraction at stressed values to restore the coverage covenant."
};

function usd0(v){ return '$' + Math.round(v).toLocaleString('en-US'); }
function usd2(v){ return '$' + v.toFixed(2); }
function fmtM(v){ return '$' + (v/1e6).toFixed(1) + 'M'; }
function minCov(years){ let m = Infinity; for (const y of years){ if (y.interest_coverage > 0 && y.interest_coverage < m) m = y.interest_coverage; } return m === Infinity ? 0 : m; }
function y8(years){ return years.find(y => y.year === 8) || years[years.length - 1]; }
function modeList(){ return DATA[curMode]; }
function paramLine(p){
  return 'Occupancy ' + (p.occupancy_pct*100).toFixed(0) + '% · Dev yield ' + (p.dev_yield*100).toFixed(2) + '% · Coupon ' + (p.debt_rate*100).toFixed(2) + '% · Cap ' + (p.cap_rate*100).toFixed(2) + '% · Lease-up ' + p.lease_up_months.toFixed(0) + ' mo';
}

function selectMode(m){
  curMode = m; idx = 0;
  document.querySelectorAll('.mode-btn').forEach(b => b.classList.toggle('active', b.dataset.mode === m));
  document.getElementById('mode-desc').textContent = MODE_DESC[m];
  buildScnButtons();
  render();
}
function buildScnButtons(){
  const row = document.getElementById('scn-row'); row.innerHTML = '';
  modeList().forEach((s, i) => {
    const b = document.createElement('button');
    b.className = 'scenario-btn' + (i === idx ? ' active' : '');
    b.textContent = s.label;
    b.onclick = () => { idx = i; buildScnButtons(); render(); };
    row.appendChild(b);
  });
}
function render(){
  const s = modeList()[idx];
  document.getElementById('mode-card').innerHTML = cardHtml(s);
  renderTable(s);
  renderCharts(s);
}

function cardHtml(s){
  if (curMode === 'managed') return managedCard(s);
  if (curMode === 'static')  return staticCard(s);
  return cureCard(s);
}
function managedCard(s){
  const m = s.meta, mc = minCov(s.years);
  const trig = m.dscr_constraint_triggered_year ? (' The issuance constraint first binds at Y' + m.dscr_constraint_triggered_year + '.') : '';
  return '<div class="adapt-card">'
    + '<h3>Management Response Scenario</h3>'
    + '<div class="param-line">' + paramLine(s.params) + '</div>'
    + '<div class="adapt-grid">'
    +   kv((m.phase2_scale*100).toFixed(0) + '%', 'Phase 2 issued')
    +   kv((m.phase3_scale*100).toFixed(0) + '%', 'Phase 3 issued')
    +   kv(m.buildings_built_pct.toFixed(0) + '%', 'Portfolio built')
    +   kv(mc.toFixed(2) + '×', 'Min coverage')
    +   kv(usd0(y8(s.years).nav_per_unit), 'Y8 NAV/unit')
    + '</div>'
    + '<p class="fwd">The LP constrains debenture issuance and development scale to hold interest coverage at or above the ' + FLOOR.toFixed(2) + '× covenant.' + trig + ' Returns decline with the stress, but no covenant breach occurs and no assets are sold. This illustrates management’s intended response; actual responses may differ.</p>'
    + '</div>';
}
function staticCard(s){
  const mc = minCov(s.years), breached = mc > 0 && mc < FLOOR;
  return '<div class="adapt-card' + (breached ? ' breach' : '') + '">'
    + '<h3>Single-Input Sensitivity / Break-even Analysis<span class="tag">IFRS 13 §93(h)(ii)</span></h3>'
    + '<div class="param-line">' + paramLine(s.params) + '</div>'
    + '<div class="adapt-grid">'
    +   kv('<span class="' + (breached ? 'down' : '') + '">' + mc.toFixed(2) + '×</span>', 'Min coverage (covenant ' + FLOOR.toFixed(2) + '×)')
    +   kv(usd0(y8(s.years).nav_per_unit), 'Y8 NAV/unit')
    + '</div>'
    + '<p class="fwd">One unobservable input is varied; all else held constant (ceteris paribus). No management response is assumed — the construction shows the unmitigated effect and the exact breach threshold.</p>'
    + (breached ? '<div class="incurable-banner">Interest coverage falls to ' + mc.toFixed(2) + '× — below the ' + FLOOR.toFixed(2) + '× covenant. This is a stress scenario beyond the reasonably-possible (±25 bps) range.</div>' : '')
    + '</div>';
}
function cureCard(s){
  const d = s.meta.disposition;
  if (!d) return '';
  if (d.fraction_sold === 0 && !d.incurable){
    return '<div class="cure-card"><h3>Covenant Cure (Corrective Disposition) Scenario</h3>'
      + '<div class="param-line">Shock at Year ' + d.shock_year + ' · ' + paramLine(s.params) + '</div>'
      + '<p class="fwd">Under this shock the existing portfolio holds interest coverage at or above the ' + FLOOR.toFixed(2) + '× covenant. No corrective disposition is required.</p></div>';
  }
  const incBanner = d.incurable
    ? '<div class="incurable-banner">Portfolio disposition cannot restore the ' + FLOOR.toFixed(2) + '× covenant at these stress levels. In this regime the retained assets out-yield the debt, so selling lowers coverage rather than raising it — the binding response is a Management Response Scenario, not a disposition.</div>'
    : '';
  const stats = d.incurable ? '' :
    '<div class="cure-grid">'
    + kv('<span class="down">' + (d.fraction_sold*100).toFixed(1) + '%</span>', 'Minimum disposition')
    + kv(fmtM(d.sale_value_total), 'At stressed value')
    + kv(fmtM(d.debt_retired), 'Debt retired')
    + kv(d.dscr_post_cure.toFixed(2) + '×', 'Coverage post-cure')
    + kv(d.buildings_remaining_pct.toFixed(1) + '%', 'Portfolio remaining')
    + kv(usd0(d.nav_per_unit_post_cure), 'NAV/unit post-cure')
    + '</div>';
  return '<div class="cure-card">'
    + '<h3>Covenant Cure (Corrective Disposition) Scenario</h3>'
    + '<div class="param-line">Shock at Year ' + d.shock_year + ' · ' + paramLine(s.params) + '</div>'
    + '<p>The portfolio is built out under base conditions; the combined shock then lands at Year ' + d.shock_year + '. The LP disposes of the minimum generating-asset fraction, valued at the stressed cap rate, required to restore the ' + FLOOR.toFixed(2) + '× coverage covenant.</p>'
    + stats + incBanner
    + '<p class="caveat"><strong>NAV floor is not restorable by disposition.</strong> NAV per unit equals stressed market value less debt and is invariant to the sale fraction when assets are sold at market value; it is reported at its stressed level (' + usd0(d.nav_per_unit_post_cure) + '), not at par. The binding cure mechanism is the coverage covenant. Sale at a stressed cap rate may not constitute an orderly transaction between market participants (IFRS 13 §15–§24); presented for covenant-remediation illustration only, not as a fair-value measurement. Forward-looking — actual outcomes may differ.</p>'
    + '</div>';
}
function kv(big, label){ return '<div class="kv"><span class="big">' + big + '</span><label>' + label + '</label></div>'; }

function renderTable(s){
  const ys = s.years;
  document.getElementById('table-head').innerHTML = '<tr><th>Metric</th>' + ys.map(y => '<th>Y' + y.year + '</th>').join('') + '</tr>';
  function row(label, fn, cls){ return '<tr><td>' + label + '</td>' + ys.map(y => '<td' + (cls ? (' class="' + cls(y) + '"') : '') + '>' + fn(y) + '</td>').join('') + '</tr>'; }
  document.getElementById('table-body').innerHTML = [
    row('Generating ($M)', y => (y.generating/1e6).toFixed(0)),
    row('NOI ($M)', y => (y.net_proceeds_from_ops/1e6).toFixed(1)),
    row('EBITDA ($M)', y => (y.ebitda/1e6).toFixed(1)),
    row('Net interest ($M)', y => (y.net_interest/1e6).toFixed(1)),
    row('Interest coverage', y => y.interest_coverage > 0 ? y.interest_coverage.toFixed(2) + '×' : '—', y => (y.interest_coverage > 0 && y.interest_coverage < FLOOR) ? 'binding' : ''),
    row('Closing debt ($M)', y => (y.closing_debt/1e6).toFixed(0)),
    row('NAV / unit', y => usd0(y.nav_per_unit)),
    row('Dist / unit', y => usd2(y.dpu)),
  ].join('');
}

function renderCharts(s){
  const ys = s.years, labels = ys.map(y => 'Y' + y.year);
  const nav = ys.map(y => y.nav_per_unit);
  const cov = ys.map(y => y.interest_coverage > 0 ? y.interest_coverage : null);
  const isCure = curMode === 'cure';
  const navDS = [{ label: 'NAV / unit', data: nav, borderColor: '#1a1a1a', tension: 0.25, fill: false, pointRadius: 2 }];
  const covDS = [{ label: 'Interest coverage', data: cov, borderColor: '#b45309', tension: 0.25, fill: false, pointRadius: 2, spanGaps: true }];
  if (isCure){
    navDS.push({ label: 'Base (pre-shock)', data: DATA.baseNav, borderColor: '#bbb', borderDash: [5,4], tension: 0.25, fill: false, pointRadius: 0 });
    covDS.push({ label: 'Base (pre-shock)', data: DATA.baseCov, borderColor: '#ccc', borderDash: [5,4], tension: 0.25, fill: false, pointRadius: 0, spanGaps: true });
  }
  covDS.push({ label: 'Covenant 1.20×', data: labels.map(() => FLOOR), borderColor: '#dc2626', borderDash: [3,3], pointRadius: 0, fill: false });
  navChart = remake(navChart, 'chart-nav', labels, navDS);
  covChart = remake(covChart, 'chart-cov', labels, covDS);
}
function remake(ch, id, labels, datasets){
  if (ch) ch.destroy();
  return new Chart(document.getElementById(id), {
    type: 'line',
    data: { labels, datasets },
    options: { responsive: true, animation: false,
      plugins: { legend: { display: true, labels: { boxWidth: 12, font: { size: 10 } } } },
      scales: { y: { ticks: { font: { size: 10 } } }, x: { ticks: { font: { size: 10 } } } } }
  });
}

selectMode('managed');
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_produces_html_and_valid_json() {
        let (html, json) = render();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("const DATA ="));
        assert!(html.ends_with("</html>\n"));
        let v: Value = serde_json::from_str(&json).expect("audit json parses");
        assert!(v["scenarios"]["management_response"].is_array());
        assert!(v["scenarios"]["single_input_sensitivity"].is_array());
        assert!(v["scenarios"]["covenant_cure"].is_array());
    }

    #[test]
    fn forbidden_terms_absent() {
        let (html, json) = render();
        for term in ["Hard Reset", "hard reset", "forced liquidation", "fire sale"] {
            assert!(!html.contains(term), "forbidden term in html: {term}");
            assert!(!json.contains(term), "forbidden term in json: {term}");
        }
    }

    #[test]
    fn three_modes_present_with_scenarios() {
        let (html, _) = render();
        assert!(html.contains("Management Response"));
        assert!(html.contains("Single-Input Stress"));
        assert!(html.contains("Covenant Cure"));
    }

    #[test]
    fn curable_cure_scenarios_restore_floor() {
        // Every curable disposition must land coverage at the 1.20× floor.
        let (_html, json) = render();
        let v: Value = serde_json::from_str(&json).unwrap();
        let cure = v["scenarios"]["covenant_cure"].as_array().unwrap();
        let mut curable = 0;
        for s in cure {
            let d = &s["meta"]["disposition"];
            if d["incurable"].as_bool() == Some(false)
                && d["fraction_sold"].as_f64().unwrap() > 0.0
            {
                curable += 1;
                let dscr = d["dscr_post_cure"].as_f64().unwrap();
                assert!((dscr - 1.20).abs() < 0.02, "post-cure dscr={dscr}");
            }
            // Invariant across every scenario.
            assert_eq!(d["nav_curable_by_disposition"].as_bool(), Some(false));
        }
        assert!(curable >= 2, "expected ≥2 curable cure scenarios, got {curable}");
    }

    #[test]
    fn embedded_data_json_parses() {
        // The page's `const DATA = {...}` payload must be valid JSON.
        let (html, _) = render();
        let start = html.find("const DATA = ").unwrap() + "const DATA = ".len();
        let tail = &html[start..];
        let end = tail.find(";\n").unwrap();
        let _: Value = serde_json::from_str(&tail[..end]).expect("embedded DATA parses");
    }
}
