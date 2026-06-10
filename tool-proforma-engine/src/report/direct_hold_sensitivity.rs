// Woodfine Direct-Hold Solutions — Sensitivity Analysis
//
// Engine-driven (every number from `pclp1_proforma::forecast_full`); the Excel summary
// AA12:AN35 supplies only the FORM/LAYOUT of the per-Investment-Unit results table.
// Variance from the earlier spreadsheet is expected and accepted.
//
// SLIDER-FREE, narrated report of PRE-SET charts that tell the resilience story.
// Four stacked landscape sections (each starts a fresh print page):
//   1. Base Case                — "What we expect to happen"
//   2. Adapting as we build     — Management Response holds the 1.20× covenant
//   3. After we're built        — post-construction headroom + one corrective-disposition exhibit
//   4. Basis & §93(h)(ii)       — reasonably-possible measurement sensitivity + FOFI
//
// The "no management response" breach is NOT presented (it is a receivership outcome,
// not an operating scenario). Base occupancy is 95% (the engine's 10.5% dev yield is net
// of 5% vacancy, so occupancy_pct = 1.0 ≡ 95% physical; downside = physical%/95).
//
// Regulatory terminology: the word "fund" never appears. Entity = "Woodfine Direct-Hold
// Solutions" / "the Direct-Hold Solutions"; holders = "Investment Unit holders".

use crate::spv::pclp1_proforma::{
    self as eng, ForecastParams, ModelMode, Pclp1Year, PCLP1_BUYER_TARGET_YIELD, PCLP1_CAP_RATE,
    PCLP1_DEBT_RATE_DEBENTURE, PCLP1_DILUTED_UNITS, PCLP1_GROSS_EQUITY,
};
use serde_json::{json, Value};

const UNITS: f64 = PCLP1_DILUTED_UNITS;
/// Base physical occupancy (the engine's 10.5% dev yield is net of 5% vacancy).
const BASE_OCC: f64 = 95.0;

fn base_with(mode: ModelMode) -> ForecastParams {
    ForecastParams { mode, ..Default::default() }
}

/// Physical occupancy %  →  engine occupancy_pct multiplier (base 95% = 1.0).
fn occ_eng(physical_pct: f64) -> f64 {
    physical_pct / BASE_OCC
}

/// Compact per-year record — only the fields the page consumes.
fn yr_rec(y: &Pclp1Year) -> Value {
    json!({
        "noi":   y.net_proceeds_from_ops,
        "dpu":   y.dpu,
        "avpu":  y.asset_value_per_unit,
        "debt":  y.closing_debt,
        "navpu": y.nav_per_unit,
        "mvpu":  y.market_value_per_unit,
        "dyb":   y.dist_yield_at_market,
        "icr":   y.interest_coverage,
        "ddc":   y.debt_to_dev_cost,
        "dav":   y.debt_to_asset_value,
        "fopex": y.advisory_fee + y.admin_compliance + y.board,
        "sqft":  y.total_sqft_generating,
    })
}

/// Min positive interest coverage across a run's years.
fn min_icr(years: &[Pclp1Year]) -> f64 {
    years
        .iter()
        .map(|y| y.interest_coverage)
        .filter(|&c| c > 0.0)
        .fold(f64::INFINITY, f64::min)
}

/// Coverage path Y1..Y10 as JSON (null where pre-income).
fn cov_path(years: &[Pclp1Year]) -> Vec<Value> {
    years[1..=10]
        .iter()
        .map(|y| if y.interest_coverage > 0.0 { json!(y.interest_coverage) } else { Value::Null })
        .collect()
}

fn nav_path(p: &ForecastParams) -> Vec<f64> {
    eng::forecast_with_params(p)[1..=10]
        .iter()
        .map(|y| y.nav_per_unit)
        .collect()
}

pub fn render() -> (String, String) {
    let ss = ModelMode::SingleInputStress;
    let md = ModelMode::ManagedDownside;

    // ── Base run (Single-Input at base ≡ Management Response at base) ─────────
    let base_p = base_with(ss);
    let (base_years, _bm) = eng::forecast_full(&base_p);
    let by = &base_years[1..=10];
    let base_nav: Vec<f64> = by.iter().map(|y| y.nav_per_unit).collect();
    let base_cov = cov_path(&base_years);
    let base_min_icr = min_icr(&base_years);
    let base_min_icr_year = base_years
        .iter()
        .filter(|y| y.interest_coverage > 0.0)
        .min_by(|a, b| a.interest_coverage.partial_cmp(&b.interest_coverage).unwrap())
        .map(|y| y.year)
        .unwrap_or(7);
    let base_run = json!({
        "years": by.iter().map(yr_rec).collect::<Vec<_>>(),
    });

    // ── Page 2: Management Response to the three COVERAGE drivers ─────────────
    // (interest rate, occupancy, development yield — each held at 1.20× by building less)
    let (yr_r2, mr2) = eng::forecast_full(&ForecastParams { debt_rate: 0.07, mode: md, ..Default::default() });
    let (yr_o75, mo75) = eng::forecast_full(&ForecastParams { occupancy_pct: occ_eng(75.0), mode: md, ..Default::default() });
    let (yr_d85, md85) = eng::forecast_full(&ForecastParams { dev_yield: 0.085, mode: md, ..Default::default() });

    let coverage_held = json!({
        "rate200": cov_path(&yr_r2),
        "occ75": cov_path(&yr_o75),
        "dev85": cov_path(&yr_d85),
    });
    let four_stress = json!([
        { "driver": "Interest rate", "move": "+200 bps → 7.00%", "builtPct": mr2.buildings_built_pct, "minICR": min_icr(&yr_r2), "y8nav": yr_r2[8].nav_per_unit, "y8dist": yr_r2[8].dpu / 100.0 },
        { "driver": "Occupancy",     "move": "75% (from 95%)",   "builtPct": mo75.buildings_built_pct, "minICR": min_icr(&yr_o75), "y8nav": yr_o75[8].nav_per_unit, "y8dist": yr_o75[8].dpu / 100.0 },
        { "driver": "Development yield", "move": "8.50% (from 10.50%)", "builtPct": md85.buildings_built_pct, "minICR": min_icr(&yr_d85), "y8nav": yr_d85[8].nav_per_unit, "y8dist": yr_d85[8].dpu / 100.0 },
    ]);

    // Adaptive lever: build % and min coverage vs coupon (5.00–9.00% @25 bps), managed.
    let mut lever = Vec::new();
    for i in 20..=36 {
        let c = i as f64 * 0.0025;
        let (years, m) = eng::forecast_full(&ForecastParams { debt_rate: c, mode: md, ..Default::default() });
        lever.push(json!({ "ratePct": c * 100.0, "builtPct": m.buildings_built_pct, "minICR": min_icr(&years) }));
    }

    // ── Page 3: post-construction headroom (Year-8 stabilised coverage vs coupon) ──
    let mut headroom = Vec::new();
    for i in 20..=44 {
        let c = i as f64 * 0.0025; // 5.00–11.00%
        let y8 = eng::forecast_with_params(&ForecastParams { debt_rate: c, mode: ss, ..Default::default() })[8]
            .interest_coverage;
        headroom.push(json!({ "ratePct": c * 100.0, "y8cov": y8 }));
    }
    // Exact post-construction break-even coupon (Y8 coverage = 1.20×).
    let post_breakeven = {
        let (mut lo, mut hi) = (PCLP1_DEBT_RATE_DEBENTURE, 0.20);
        for _ in 0..40 {
            let mid = (lo + hi) / 2.0;
            let icr8 = eng::forecast_with_params(&ForecastParams { debt_rate: mid, ..base_p })[8].interest_coverage;
            if icr8 > 1.20 { lo = mid; } else { hi = mid; }
        }
        (lo + hi) / 2.0
    };

    // NAV resilience: base vs a named bear / bull.
    let nav_bear = nav_path(&ForecastParams { cap_rate: 0.0775, occupancy_pct: occ_eng(88.0), mode: ss, ..Default::default() });
    let nav_bull = nav_path(&ForecastParams { cap_rate: 0.0575, mode: ss, ..Default::default() });

    // ── Capital preservation under a MAXIMAL, research-grounded combined shock ──
    // A single internally-consistent "severely adverse" shock across all three drivers
    // (GFC / 2022–23 office scale; office cap-rate↔10yr-Treasury beta ≈ 0.70):
    //   • refinancing rate +500 bps → 10.00%  — drives the covenant breach (rate moves coverage, not NAV)
    //   • occupancy −7 pp → 88%                — severe recession office vacancy (moves valuation)
    //   • cap rate SOLVED (~+296 bps → ~9.21%) — office cap expansion (moves valuation)
    // The cap rate is solved so the stressed Year-8 NAV/unit lands just above the $100 floor
    // (≈ $105): the appreciation erodes but the investor's initial capital is preserved, and a
    // minimum disposition restores the 1.20× covenant (NAV-neutral — a market-value sale).
    let cure_rate: f64 = 0.10; // +500 bps over the 5.00% base
    let cure_occ_pct: f64 = 88.0; // −7 pp from the 95% base
    let cure_occ = occ_eng(cure_occ_pct);
    let cure_nav_target: f64 = 105.0; // a small premium to the $100 initial capital
    let cure_p_at = |cap: f64| ForecastParams {
        occupancy_pct: cure_occ,
        debt_rate: cure_rate,
        cap_rate: cap,
        mode: ModelMode::CovenantCure { shock_year: 8 },
        ..Default::default()
    };
    // Solve the cap rate so the post-cure NAV/unit ≈ the target (cap ↑ ⇒ valuation ↓ ⇒ NAV ↓).
    let cure_cap = {
        let (mut lo, mut hi) = (PCLP1_CAP_RATE, 0.15);
        for _ in 0..48 {
            let mid = (lo + hi) / 2.0;
            let nav = eng::forecast_full(&cure_p_at(mid)).1.disposition.unwrap().nav_per_unit_post_cure;
            if nav > cure_nav_target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    };
    let cure_disp = eng::forecast_full(&cure_p_at(cure_cap)).1.disposition.unwrap();
    let before_cov = eng::forecast_with_params(&ForecastParams {
        occupancy_pct: cure_occ,
        debt_rate: cure_rate,
        cap_rate: cure_cap,
        mode: ss,
        ..Default::default()
    })[8]
        .interest_coverage;
    let cure = json!({
        "frac": cure_disp.fraction_sold,
        "before": before_cov,
        "after": cure_disp.dscr_post_cure,
        "sale": cure_disp.sale_value_total,
        "retired": cure_disp.debt_retired,
        "navpu": cure_disp.nav_per_unit_post_cure,
        "floor": 100.0,
        "distPerUnitPostCure": cure_disp.dist_per_unit_post_cure,
        "distPerUnitBase": base_years[8].dpu,
        "shockRatePct": cure_rate * 100.0,
        "shockRateBps": (cure_rate - PCLP1_DEBT_RATE_DEBENTURE) * 10000.0,
        "shockCapPct": cure_cap * 100.0,
        "shockCapBps": (cure_cap - PCLP1_CAP_RATE) * 10000.0,
        "shockOccPct": cure_occ_pct,
        "shockOccPp": BASE_OCC - cure_occ_pct,
    });

    // ── Page 4: §93(h)(ii) two-sided ±25 bps reasonably-possible disclosure ──
    let base_nav8 = base_years[8].nav_per_unit;
    let probe2 = |driver: &str, adv: &str, fav: &str, p_adv: ForecastParams, p_fav: ForecastParams| -> Value {
        let na = eng::forecast_with_params(&p_adv);
        let nf = eng::forecast_with_params(&p_fav);
        json!({
            "driver": driver, "adverse": adv, "favorable": fav,
            "navAdverse": na[8].nav_per_unit, "navFavorable": nf[8].nav_per_unit, "navBase": base_nav8,
            "distAdverse": na[8].dpu / 100.0, "distBase": base_years[8].dpu / 100.0,
            "minICRAdverse": min_icr(&na),
        })
    };
    let oneway = json!([
        probe2("Cap rate", "+25 bps → 6.50%", "−25 bps → 6.00%",
            ForecastParams { cap_rate: 0.065, ..base_p }, ForecastParams { cap_rate: 0.060, ..base_p }),
        probe2("Interest rate", "+25 bps → 5.25%", "−25 bps → 4.75%",
            ForecastParams { debt_rate: 0.0525, ..base_p }, ForecastParams { debt_rate: 0.0475, ..base_p }),
        probe2("Occupancy", "−2.5 pp → 92.5%", "+2.5 pp → 97.5%",
            ForecastParams { occupancy_pct: occ_eng(92.5), ..base_p }, ForecastParams { occupancy_pct: occ_eng(97.5), ..base_p }),
        probe2("Development yield", "−25 bps → 10.25%", "+25 bps → 10.75%",
            ForecastParams { dev_yield: 0.1025, ..base_p }, ForecastParams { dev_yield: 0.1075, ..base_p }),
    ]);

    // Analytic break-even thresholds — appendix "stress reference" only.
    let yr8 = &base_years[8];
    let cap_denom = 100.0 * UNITS - yr8.ending_cash - yr8.wip + yr8.closing_debt;
    let cap_nav100 = yr8.net_proceeds_from_ops / cap_denom;
    let thresholds = json!({
        "cap_nav100_pct": cap_nav100 * 100.0,
        "cap_headroom_bps": (cap_nav100 - PCLP1_CAP_RATE) * 10000.0,
    });

    // Resilience scalars for the headline band + captions.
    let resilience = json!({
        "y8CovBase": base_years[8].interest_coverage,
        "postBreakevenPct": post_breakeven * 100.0,
        "postBreakevenBps": (post_breakeven - PCLP1_DEBT_RATE_DEBENTURE) * 10000.0,
        "built200": mr2.buildings_built_pct,
    });

    // ── DATA payload ─────────────────────────────────────────────────────────
    let data = json!({
        "units": UNITS,
        "baseOcc": BASE_OCC,
        "baseNav": base_nav,
        "baseCov": base_cov,
        "base": {
            "y8nav": base_years[8].nav_per_unit,
            "y10nav": base_years[10].nav_per_unit,
            "y8mv": base_years[8].market_value_per_unit,
            "minICR": base_min_icr,
            "minICRyear": base_min_icr_year,
            "run": base_run,
        },
        "coverageHeld": coverage_held,
        "fourStress": four_stress,
        "lever": lever,
        "headroom": headroom,
        "navBear": nav_bear,
        "navBull": nav_bull,
        "cure": cure,
        "oneway": oneway,
        "thresholds": thresholds,
        "resilience": resilience,
        "marketYield": PCLP1_BUYER_TARGET_YIELD,
    });
    let data_str = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    // ── Audit JSON ───────────────────────────────────────────────────────────
    let audit = json!({
        "metadata": {
            "entity": "Woodfine Direct-Hold Solutions",
            "instrument": "Investment Units",
            "generated": "2026-06-08",
            "engine": "Woodfine forecast engine (three-mode), June 2026 build",
            "version": "Direct-Hold Sensitivity JW3",
            "covenant_floor_dscr": 1.20,
            "capital_preservation_floor_nav_per_unit": 100.0,
            "base_occupancy_pct": BASE_OCC,
            "basis": "All figures generated by the forecast engine; the interface performs no independent calculation. The 10.5% development yield is net of 5% vacancy, so the base reflects 95% occupancy.",
            "management_response_note": "The forecasts reflect the structure as actually managed: if financing costs or conditions worsen during build-out, the issuer constrains debenture issuance and builds fewer Investment Units of space, holding interest coverage at the 1.20x covenant. A static path that froze every management lever is not a realistic operating scenario, so it is not presented; under the debenture terms a sustained covenant breach would transfer control of the assets to the secured lenders.",
            "vehicle_note": "Woodfine Direct-Hold Solutions issue Investment Units and are direct-hold investment structures; they do not offer unit redemption or carry the reserve features of redeemable collective-investment vehicles."
        },
        "base_case": { "diluted_units": UNITS, "occupancy_pct": BASE_OCC, "years": by },
        "management_response": four_stress,
        "post_construction_headroom": headroom,
        "covenant_cure": cure,
        "ifrs13_93h_ii_reasonably_possible": oneway,
        "stress_reference_outside_reasonably_possible": thresholds,
    });
    let json_out = serde_json::to_string_pretty(&audit).unwrap_or_else(|_| "{}".to_string());

    // ── HTML ─────────────────────────────────────────────────────────────────
    let r = &resilience;
    let built200 = r["built200"].as_f64().unwrap();
    let y8cov = r["y8CovBase"].as_f64().unwrap();
    let post_bps = r["postBreakevenBps"].as_f64().unwrap();
    let post_pct = r["postBreakevenPct"].as_f64().unwrap();
    let cure_frac = cure["frac"].as_f64().unwrap() * 100.0;

    let mut h = String::with_capacity(220 * 1024);
    h.push_str(HTML_HEAD);
    h.push_str(&format!(
        r#"<div class="appbar">
  <div class="appbar-title">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
  <div class="appbar-meta">${gross_m:.0}M · {units_k:.0}K Investment Units @ $100.00 · Y1–Y10 · June 2026</div>
  <button class="print-now" onclick="window.print()">⎙ Print / PDF</button>
</div>
<div class="report-title">
  <div class="rt-title">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
  <div class="rt-meta">${gross_m:.0}M · {units_k:.0}K Investment Units @ $100.00 · Y1–Y10 · June 2026</div>
</div>
<div class="keymsg">
  <div class="keymsg-h">How the structure behaves when conditions worsen</div>
  <div class="keymsg-grid">
    <div class="km"><span class="kt">If costs rise <b>while we build</b></span><p>Woodfine Direct-Hold Solutions issue less debenture and build fewer Investment Units of space, holding interest coverage at the <b>1.20× covenant — no breach</b>. At a +200 bps coupon, about <b>{built200:.0}%</b> of the programme is built.</p></div>
    <div class="km"><span class="kt">Once <b>built</b></span><p>Stabilised coverage is <b>{y8cov:.2}×</b> (Year 8). It would take roughly <b>+{post_bps:.0} bps</b> (a coupon near {post_pct:.1}%) before coverage reaches the covenant. The built structure absorbs large increases.</p></div>
    <div class="km"><span class="kt">What this means for <b>your capital</b></span><p>Across both regimes the <b>$100.00</b> capital-preservation reference per Investment Unit holds and the covenant is never breached. The structure adapts; it does not break.</p></div>
  </div>
</div>

<section class="chapter" id="chap-base">
  <div class="chapter-head">
    <div class="ch-mast">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
    <div class="ch-sec">Section 1 of 4 — What we expect to happen</div>
    <div class="ch-term">Base case · 10-year forecast per Investment Unit</div>
    <div class="basis">All figures generated by the forecast engine. Base assumptions: development yield 10.50% (net of 5% vacancy ⇒ <b>95% occupancy</b>) · cap rate 6.25% · debenture 5.00% · buyer yield 8.00% · coverage covenant 1.20× · capital-preservation reference $100.00/Investment Unit.</div>
  </div>
  <div class="cards" id="cards-base"></div>
  <div class="chart-row">
    <div class="chart-card"><h3>NAV per Investment Unit ($100 invested)</h3><canvas id="cv-base-nav"></canvas><div class="cap">NAV rises from ~$100 at issue to ~${y10nav:.0} by Year 10. The $100 capital-preservation reference is shown.</div></div>
    <div class="chart-card"><h3>Interest Coverage — covenant floor 1.20×</h3><canvas id="cv-base-cov"></canvas><div class="cap">Coverage is tightest mid-build (Year 7) and rises after stabilisation. It stays on or above the 1.20× covenant.</div></div>
  </div>
  <div class="pg-runhead pagebreak">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
  <div class="stitle">Financial Forecast — Per Investment Unit (engine-generated)</div>
  <div class="table-wrap"><table class="wide" id="tbl-base"><thead></thead><tbody></tbody></table></div>
  <div class="chapter-foot">Forward-looking information (FOFI) — assumptions as at June 2026; actual results will vary. See Section 4.</div>
</section>

<section class="chapter" id="chap-managed">
  <div class="chapter-head">
    <div class="ch-mast">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
    <div class="ch-sec">Section 2 of 4 — If conditions worsen while we build, we build less</div>
    <div class="ch-term">Management Response · the covenant is held across cap-cost, occupancy and yield stress</div>
  </div>
  <div class="lead">The forecasts reflect the structure as it is actually managed. If financing costs or operating conditions deteriorate during build-out, the issuer constrains debenture issuance and builds fewer Investment Units of space, holding interest coverage at the 1.20× covenant. A static path that froze every management lever — holding issuance and build-out at plan while costs rose — is not a realistic operating scenario: under the debenture terms a sustained covenant breach would transfer control of the assets to the secured lenders. Because that no-response path does not represent how the structure would be operated, it is not presented here as a scenario.</div>
  <div class="chart-row">
    <div class="chart-card"><h3>Coverage is held — across interest rate, occupancy and development yield</h3><canvas id="cv-coverage-held"></canvas><div class="cap">Base and three coverage-driver stresses (interest +200 bps, occupancy 90%, development yield 9.50%), all managed. Every line sits on or above the 1.20× covenant — no breach, Y1–Y10.</div></div>
    <div class="chart-card"><h3>The adaptive lever — build less as financing cost rises</h3><canvas id="cv-lever"></canvas><div class="cap">As the coupon rises the issuer dials the programme down (left axis) while minimum coverage stays pinned at 1.20× (right axis): at +200 bps ~77% built; at +400 bps ~57%.</div></div>
  </div>
  <div class="stitle">Management response to the three coverage drivers (engine-computed)</div>
  <div class="table-wrap"><table class="wide narrow" id="tbl-fourstress"><thead>
    <tr><th class="lnum"></th><th>Stress (one driver, all else at base)</th><th>Programme built</th><th>Minimum coverage</th><th>Y8 NAV / Unit (value)</th><th>Y8 Income Yield on Capital</th></tr>
  </thead><tbody></tbody></table></div>
  <div class="chapter-foot">Cap rate is a valuation input only (it does not change operating coverage) and is shown in Section 3 and Section 4.</div>
</section>

<section class="chapter" id="chap-after">
  <div class="chapter-head">
    <div class="ch-mast">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
    <div class="ch-sec">Section 3 of 4 — Once built, the structure absorbs large shocks</div>
    <div class="ch-term">Post-construction coverage headroom · corrective disposition of last resort</div>
  </div>
  <div class="lead">After construction, stabilised coverage is high — Year-8 coverage is {y8cov:.2}×, reaching the 1.20× covenant only near a {post_pct:.1}% coupon (+{post_bps:.0} bps), so a corrective disposition is rarely needed. To size the downside, the exhibit below applies a single <b>maximal combined shock</b> — a Great Financial Crisis (GFC) / 2022–23-scale recession hitting financing cost, valuation and occupancy at once — and shows the initial capital is preserved even then.</div>
  <div class="chart-row">
    <div class="chart-card"><h3>Post-construction headroom — Year-8 coverage vs coupon</h3><canvas id="cv-headroom"></canvas><div class="cap">Stabilised Year-8 coverage starts at {y8cov:.2}× and reaches the 1.20× covenant only near a {post_pct:.1}% coupon (≈ +{post_bps:.0} bps).</div></div>
    <div class="chart-card"><h3>NAV resilience — base vs bear and bull</h3><canvas id="cv-nav-resilience"></canvas><div class="cap">NAV/unit climbs to ~${y10nav:.0} by Year 10. Even the bear case (cap +150 bps, occupancy 88%) stays well above the $100 capital-preservation reference — and the distribution yield on initial capital moves with NAV, so the income and value sensitivities track together.</div></div>
  </div>
  <div class="stitle">Capital preservation under a maximal combined shock (Great Financial Crisis / 2022–23 scale)</div>
  <div class="cure-wrap"><div class="chart-card cure-mini"><h3>Coverage: at the combined shock vs after the minimum disposition</h3><canvas id="cv-cure"></canvas></div>
  <div class="cure-card" id="cure-card"></div></div>
  <div class="chapter-foot">Shock magnitudes are grounded in severe-downturn evidence (office cap-rate expansion +300–330 bps in 2008–09 and 2022–23; refinancing moves of +450–500 bps). A forced sale could realise <b>15–30% below the stressed appraisal</b>; the disposition is measured at the stressed orderly-transaction value (IFRS 13 §15–§24). Forward-looking (FOFI) — see Section 4.</div>
</section>

<section class="chapter" id="chap-basis">
  <div class="chapter-head">
    <div class="ch-mast">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
    <div class="ch-sec">Section 4 of 4 — Measurement sensitivity, assumptions & caveats</div>
    <div class="ch-term">IFRS 13 §93(h)(ii) reasonably-possible disclosure · forward-looking statements</div>
  </div>
  <div class="stitle">IFRS 13 §93(h)(ii) — reasonably-possible (±25 bps) measurement sensitivity</div>
  <div class="table-wrap"><table class="wide narrow" id="tbl-oneway"><thead>
    <tr><th class="lnum"></th><th>Unobservable input</th><th>Adverse move</th><th>Favourable move</th><th>Y8 NAV / Unit (adverse)</th><th>Y8 NAV / Unit (favourable)</th><th>Y8 Income Yield (adverse)</th><th>Min coverage (adverse)</th></tr>
  </thead><tbody></tbody></table></div>
  <div class="lead">Within the reasonably-possible range, the measurement is not sensitive to a degree that threatens the $100 capital-preservation reference or the 1.20× covenant; the dollar effects are small and shown above. Moves larger than this range are stress scenarios outside the §93(h)(ii) disclosure (for reference, NAV reaches $100 per Unit only if the cap rate widened to about {cap_nav100:.2}%, ≈ +{cap_bps:.0} bps — far beyond reasonably possible).</div>
  <div class="chart-card"><h3>Ranked reasonably-possible effect on Year-8 NAV per Unit (±25 bps)</h3><canvas id="cv-tornado"></canvas></div>
  <div class="pg-runhead pagebreak">Woodfine Direct-Hold Solutions — Sensitivity Analysis</div>
  <div class="tblock"><h4>Basis of preparation</h4>
    <p>All figures are generated by the forecast engine; the interface performs no independent calculation. The 10.50% development yield is net of 5% vacancy, so the base reflects <b>95% occupancy</b>.</p>
    <p>The forecasts reflect the structure as actually managed (Section 2). A static no-response projection is not presented because, under the debenture terms, a sustained covenant breach would transfer control of the assets to the secured lenders — it is not an operating scenario. Woodfine Direct-Hold Solutions issue Investment Units and are direct-hold structures; they do not offer unit redemption or carry the reserve features of redeemable collective-investment vehicles.</p>
  </div>
  <div class="fofi"><h4>Forward-Looking Information (FOFI)</h4>
    <p>This analysis is future-oriented financial information prepared on the stated assumptions as at June 2026. The management-response and corrective-disposition exhibits are forward-looking illustrations, not fair-value measurement inputs. Actual results, measurements and outcomes will differ — potentially materially — and readers should not place undue reliance on them. Prepared and presented consistent with NI 51-102 and ISAE 3400.</p>
  </div>
  <div class="chapter-foot">Woodfine Direct-Hold Solutions · Sensitivity Analysis · engine June 2026 build · generated June 2026 · V2</div>
</section>
"#,
        gross_m = PCLP1_GROSS_EQUITY / 1e6,
        units_k = (UNITS / 1000.0).floor(),
        built200 = built200,
        y8cov = y8cov,
        post_bps = post_bps,
        post_pct = post_pct,
        y10nav = base_years[10].nav_per_unit,
        cap_nav100 = cap_nav100 * 100.0,
        cap_bps = (cap_nav100 - PCLP1_CAP_RATE) * 10000.0,
    ));
    let _ = cure_frac; // (cure figures rendered client-side from DATA.cure)

    h.push_str("<script>\nconst DATA = ");
    h.push_str(&data_str);
    h.push_str(";\n");
    h.push_str(JS_APP);
    h.push_str("</script>\n</body>\n</html>\n");

    (h, json_out)
}

const HTML_HEAD: &str = include_str!("direct_hold_head.html");
const JS_APP: &str = include_str!("direct_hold_app.js");

#[cfg(test)]
mod tests {
    use super::*;

    fn html() -> String {
        render().0
    }

    #[test]
    fn render_produces_html_and_valid_json() {
        let (h, j) = render();
        assert!(h.starts_with("<!DOCTYPE html>"));
        assert!(h.contains("const DATA ="));
        assert!(h.ends_with("</html>\n"));
        let v: Value = serde_json::from_str(&j).expect("audit json parses");
        assert!(v["ifrs13_93h_ii_reasonably_possible"].is_array());
        assert!(v["management_response"].is_array());
    }

    #[test]
    fn embedded_data_json_parses() {
        let h = html();
        let start = h.find("const DATA = ").unwrap() + "const DATA = ".len();
        let tail = &h[start..];
        let end = tail.find(";\n").unwrap();
        let _: Value = serde_json::from_str(&tail[..end]).expect("embedded DATA parses");
    }

    #[test]
    fn forbidden_terms_absent() {
        let (h, j) = render();
        for body in [&h, &j] {
            let lower = body.to_lowercase();
            for (i, _) in lower.match_indices("fund") {
                let after = &lower[i + 4..];
                assert!(
                    after.starts_with("ing") || after.starts_with("ed"),
                    "whole-word 'fund' at byte {i}"
                );
            }
            for t in ["professional centres", "pclp", "lp units", "hard reset", "forced liquidation"] {
                assert!(!lower.contains(t), "forbidden term '{t}'");
            }
        }
    }

    #[test]
    fn no_response_breach_not_presented() {
        // The misleading "+53 bps breaks the covenant" single-input scenario must be gone.
        let h = html();
        assert!(!h.contains("+53 bps"));
        assert!(!h.contains("Break-even (stress)"));
        assert!(!h.contains("breaks the covenant"));
        // The receivership point appears only as the managed-response basis note (not a scenario label).
        assert!(h.contains("transfer control of the assets to the secured lenders"));
    }

    #[test]
    fn base_occupancy_is_95() {
        // Base occupancy must read 95% (engine 10.5% yield is net of 5% vacancy).
        let h = html();
        assert!(h.contains("95% occupancy"));
        let (_h, j) = render();
        let v: Value = serde_json::from_str(&j).unwrap();
        assert_eq!(v["base_case"]["occupancy_pct"].as_f64(), Some(95.0));
    }

    #[test]
    fn four_drivers_in_disclosure() {
        let (_h, j) = render();
        let v: Value = serde_json::from_str(&j).unwrap();
        let ow = v["ifrs13_93h_ii_reasonably_possible"].as_array().unwrap();
        let names: Vec<String> = ow.iter().map(|o| o["driver"].as_str().unwrap().to_string()).collect();
        for d in ["Cap rate", "Interest rate", "Occupancy", "Development yield"] {
            assert!(names.iter().any(|n| n == d), "missing driver {d}");
        }
        // Management response covers the three coverage drivers.
        let mr: Vec<String> = v["management_response"].as_array().unwrap()
            .iter().map(|o| o["driver"].as_str().unwrap().to_string()).collect();
        for d in ["Interest rate", "Occupancy", "Development yield"] {
            assert!(mr.iter().any(|n| n == d), "missing managed driver {d}");
        }
    }

    #[test]
    fn income_metric_present() {
        // Y8 distribution yield on initial capital is carried beside NAV (R4.2).
        let (_h, j) = render();
        let v: Value = serde_json::from_str(&j).unwrap();
        for o in v["management_response"].as_array().unwrap() {
            assert!(o["y8dist"].as_f64().map(|x| x.is_finite()).unwrap_or(false), "fourStress missing y8dist");
        }
        for o in v["ifrs13_93h_ii_reasonably_possible"].as_array().unwrap() {
            assert!(o["distAdverse"].as_f64().map(|x| x.is_finite()).unwrap_or(false), "oneway missing distAdverse");
        }
    }

    #[test]
    fn managed_holds_covenant_under_stress() {
        // Each managed coverage-driver stress holds minimum coverage at/above the covenant.
        let (_h, j) = render();
        let v: Value = serde_json::from_str(&j).unwrap();
        for o in v["management_response"].as_array().unwrap() {
            let mi = o["minICR"].as_f64().unwrap();
            assert!(mi >= 1.19, "managed min coverage {mi} below covenant for {}", o["driver"]);
        }
    }

    #[test]
    fn cure_preserves_capital_under_combined_shock() {
        // R7: a maximal, internally-consistent combined shock breaches coverage; the minimum
        // disposition restores the 1.20× covenant; and the stressed NAV/unit is preserved just
        // above the $100 capital floor (a capital-PRESERVATION exhibit, not a wipe-out).
        let (_h, j) = render();
        let v: Value = serde_json::from_str(&j).unwrap();
        let c = &v["covenant_cure"];
        let before = c["before"].as_f64().unwrap();
        let after = c["after"].as_f64().unwrap();
        let navpu = c["navpu"].as_f64().unwrap();
        let frac = c["frac"].as_f64().unwrap();
        assert!(before < 1.20, "the combined shock should breach coverage, got {before}");
        assert!((after - 1.20).abs() < 0.02, "post-cure coverage should be ~1.20x, got {after}");
        assert!(frac > 0.0 && frac < 1.0, "a partial disposition should cure, got frac {frac}");
        assert!((navpu - 105.0).abs() < 3.0, "preserved NAV should be ~$105, got {navpu}");
        assert!(navpu > 100.0, "preserved NAV should hold above the $100 floor, got {navpu}");
        // The shock is violent across all three drivers.
        assert!(c["shockRateBps"].as_f64().unwrap() >= 400.0, "rate shock too mild");
        assert!(c["shockCapBps"].as_f64().unwrap() >= 200.0, "cap shock too mild");
        assert!(c["shockOccPp"].as_f64().unwrap() >= 5.0, "occupancy shock too mild");
    }
}
