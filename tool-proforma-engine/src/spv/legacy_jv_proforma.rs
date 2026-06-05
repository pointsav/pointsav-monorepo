// Legacy JV (D7) — Traditional Joint Venture proforma engine module.
// BRIEF v0.15.6 §5h.
//
// Apples-to-apples comparator to PCLP 1 (D2). Same $250M equity but traditional bank-debt
// JV financing (3.0× leverage, 65% LTV covenant), single development round, no compounding.
//
// ASPE 3061 cost model (banks prefer no IFRS FV swings on covenanted real estate).
// 50-year straight-line depreciation on building component (land excluded).

use serde::Serialize;

// ─── Constants from BRIEF v0.15.6 §5h (lines 2456-2576) ──────────────────────

// Capital structure
pub const LEGACY_JV_EQUITY: f64 = 250_000_000.0; // BRIEF §2464
pub const LEGACY_JV_BANK_DEBT: f64 = 750_000_000.0; // BRIEF §2465
pub const LEGACY_JV_TOTAL_CAPITAL: f64 = 1_000_000_000.0; // BRIEF §2467
pub const LEGACY_JV_DEBT_RATIO: f64 = 3.0; // 3.0× D/E

// LP cap structure (mirrors PCLP 1 unit price for like-for-like comparison)
pub const LEGACY_JV_LP_UNITS: f64 = 2_500_000.0; // $250M / $100/unit
pub const LEGACY_JV_UNIT_PRICE: f64 = 100.0;

// Building portfolio
pub const LEGACY_JV_TOTAL_SF: f64 = 2_298_150.0; // BRIEF §2469
pub const LEGACY_JV_COST_PER_SF: f64 = 326.35; // BRIEF §2468

// Yield + valuation
pub const LEGACY_JV_DEV_YIELD: f64 = 0.105; // 10.5%
pub const LEGACY_JV_CAP_RATE: f64 = 0.0625; // 6.25%

// Operating
// BRIEF §2528 names "$78.75M = 10.5% × $750M debt" as the *gross* rental revenue
// at stabilization. After 20% opex, NOI = $63M (matches BRIEF §2533 net income math).
pub const LEGACY_JV_GROSS_REV_STABILIZED: f64 = 78_750_000.0;
pub const LEGACY_JV_NOI_STABILIZED: f64 = 63_000_000.0; // = $78.75M × (1 - 20%)
pub const LEGACY_JV_OPEX_PCT: f64 = 0.20;
pub const LEGACY_JV_GA_ANNUAL: f64 = 2_000_000.0; // BRIEF §2533
pub const LEGACY_JV_INTEREST_RATE: f64 = 0.050; // 5% permanent loan
pub const LEGACY_JV_DEPRECIATION_YRS: f64 = 50.0; // ASPE 3061
pub const LEGACY_JV_BUILDING_COMPONENT: f64 = 1_055_000_000.0; // Building only (land excluded)

// S-curve construction draw Y1-Y3 (BRIEF §2494)
pub const LEGACY_JV_DRAW_Y1: f64 = 200_000_000.0; // 20% of $1B
pub const LEGACY_JV_DRAW_Y2: f64 = 500_000_000.0; // 50%
pub const LEGACY_JV_DRAW_Y3: f64 = 300_000_000.0; // 30%

// LTV covenant
pub const LEGACY_JV_LTV_COVENANT: f64 = 0.65; // 65% max
pub const LEGACY_JV_STABILIZED_AV: f64 = 1_008_000_000.0; // = $63M NOI / 6.25% cap (IFRS FV)

// ─── Output struct ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct LegacyJvYear {
    pub year: u32,
    pub phase: &'static str, // "Construction" | "Stabilized"
    pub capex: f64,          // S-curve construction draws
    pub cumulative_capex: f64,
    pub debt_outstanding: f64,
    pub equity_contribution: f64,
    pub gross_rental_revenue: f64,
    pub operating_expenses: f64,
    pub noi: f64,
    pub interest_expense: f64,
    pub depreciation: f64,
    pub ga_expense: f64,
    pub net_income: f64,
    pub distributable_cash: f64,
    pub distributions_to_lps: f64,
    pub cumulative_distributions: f64,
    pub dpu: f64,
    pub asset_value_aspe: f64, // Book at cost minus accumulated depr
    pub partners_capital: f64,
    pub equity_value: f64, // Asset value - debt
    pub equity_value_per_unit: f64,
    pub ltv_book: f64,
    pub dscr: f64, // NOI / debt service
}

// ─── Forecast ───────────────────────────────────────────────────────────────

fn capex_at(y: u32) -> f64 {
    match y {
        1 => LEGACY_JV_DRAW_Y1,
        2 => LEGACY_JV_DRAW_Y2,
        3 => LEGACY_JV_DRAW_Y3,
        _ => 0.0,
    }
}

pub fn forecast() -> Vec<LegacyJvYear> {
    let mut years: Vec<LegacyJvYear> = Vec::with_capacity(11);
    let mut cum_capex: f64 = 0.0;
    let mut cum_distributions: f64 = 0.0;
    let mut cum_depr: f64 = 0.0;

    // Y0: pre-launch state
    years.push(LegacyJvYear {
        year: 0,
        phase: "Pre-launch",
        capex: 0.0,
        cumulative_capex: 0.0,
        debt_outstanding: 0.0,
        equity_contribution: 0.0,
        gross_rental_revenue: 0.0,
        operating_expenses: 0.0,
        noi: 0.0,
        interest_expense: 0.0,
        depreciation: 0.0,
        ga_expense: 0.0,
        net_income: 0.0,
        distributable_cash: 0.0,
        distributions_to_lps: 0.0,
        cumulative_distributions: 0.0,
        dpu: 0.0,
        asset_value_aspe: 0.0,
        partners_capital: 0.0,
        equity_value: 0.0,
        equity_value_per_unit: 0.0,
        ltv_book: 0.0,
        dscr: 0.0,
    });

    for y in 1u32..=10 {
        let capex = capex_at(y);
        cum_capex += capex;

        // Bank debt first, equity last (BRIEF §552). At end of Y3 total = $1B
        // with $750M debt + $250M equity. Approximate: each year's draw goes
        // 75% debt / 25% equity (proportional to overall mix).
        let equity_contrib = capex * (LEGACY_JV_EQUITY / LEGACY_JV_TOTAL_CAPITAL);
        let debt_drawn_this_year = capex * (LEGACY_JV_BANK_DEBT / LEGACY_JV_TOTAL_CAPITAL);

        // Construction loan; debt only fully drawn by end Y3, then refinanced
        // to permanent loan at Y4 (no further changes in V1 model).
        let debt_outstanding = if y >= 4 {
            LEGACY_JV_BANK_DEBT
        } else {
            // Cumulative debt drawn through this year
            (cum_capex * (LEGACY_JV_BANK_DEBT / LEGACY_JV_TOTAL_CAPITAL)).min(LEGACY_JV_BANK_DEBT)
        };
        let _ = debt_drawn_this_year;

        // Income statement (Y4+ stabilized; Y1-Y3 construction, no revenue)
        let phase = if y <= 3 { "Construction" } else { "Stabilized" };
        let gross_rev = if y >= 4 {
            LEGACY_JV_GROSS_REV_STABILIZED
        } else {
            0.0
        };
        let opex = gross_rev * LEGACY_JV_OPEX_PCT;
        let noi = gross_rev - opex;
        let interest = debt_outstanding * LEGACY_JV_INTEREST_RATE;
        let depr = if y >= 4 {
            LEGACY_JV_BUILDING_COMPONENT / LEGACY_JV_DEPRECIATION_YRS
        } else {
            0.0
        };
        cum_depr += depr;
        let ga = if y >= 4 { LEGACY_JV_GA_ANNUAL } else { 0.0 };
        let net_income = noi - interest - depr - ga;

        let distributable_cash = if y >= 4 { net_income + depr } else { 0.0 };
        let distributions = distributable_cash;
        cum_distributions += distributions;
        let dpu = distributions / LEGACY_JV_LP_UNITS;

        // ASPE asset value = cumulative capex - accumulated depreciation
        let asset_value_aspe = cum_capex - cum_depr;

        // Partners' capital = Opening + Contributions - Distributions + Net Income
        // Y0=0; Y1+: cumulative_equity_contrib + cumulative_net_income - cumulative_distributions
        let cum_equity =
            (cum_capex * (LEGACY_JV_EQUITY / LEGACY_JV_TOTAL_CAPITAL)).min(LEGACY_JV_EQUITY);
        let partners_capital = cum_equity + cum_distributions.min(0.0); // Simplification

        let equity_value = asset_value_aspe - debt_outstanding;
        let equity_value_per_unit = equity_value / LEGACY_JV_LP_UNITS;

        let ltv_book = if asset_value_aspe > 0.0 {
            debt_outstanding / asset_value_aspe
        } else {
            0.0
        };
        let dscr = if interest > 0.0 { noi / interest } else { 0.0 };

        years.push(LegacyJvYear {
            year: y,
            phase,
            capex,
            cumulative_capex: cum_capex,
            debt_outstanding,
            equity_contribution: equity_contrib,
            gross_rental_revenue: gross_rev,
            operating_expenses: opex,
            noi,
            interest_expense: interest,
            depreciation: depr,
            ga_expense: ga,
            net_income,
            distributable_cash,
            distributions_to_lps: distributions,
            cumulative_distributions: cum_distributions,
            dpu,
            asset_value_aspe,
            partners_capital,
            equity_value,
            equity_value_per_unit,
            ltv_book,
            dscr,
        });
    }
    years
}

pub fn forecast_json() -> serde_json::Value {
    let years = forecast();
    let y10 = &years[10];

    let ifrs_fv_equity = LEGACY_JV_STABILIZED_AV - LEGACY_JV_BANK_DEBT;
    let moic_at_fv = (y10.cumulative_distributions + ifrs_fv_equity) / LEGACY_JV_EQUITY;
    let moic_at_book = (y10.cumulative_distributions + y10.equity_value) / LEGACY_JV_EQUITY;

    serde_json::json!({
        "entity": "Legacy JV (D7) — Traditional Joint Venture",
        "source": "tool-proforma-engine src/spv/legacy_jv_proforma module",
        "brief_section": "v0.15.6 §5h",
        "version": "V1",
        "generated_at": "2026-06-04",
        "purpose": "Apples-to-apples 10-year return comparator to PCLP 1 (D2)",
        "inputs": {
            "equity": LEGACY_JV_EQUITY,
            "bank_debt": LEGACY_JV_BANK_DEBT,
            "total_capital": LEGACY_JV_TOTAL_CAPITAL,
            "debt_to_equity_ratio": LEGACY_JV_DEBT_RATIO,
            "lp_units": LEGACY_JV_LP_UNITS,
            "unit_price": LEGACY_JV_UNIT_PRICE,
            "total_sf": LEGACY_JV_TOTAL_SF,
            "cost_per_sf": LEGACY_JV_COST_PER_SF,
            "dev_yield": LEGACY_JV_DEV_YIELD,
            "cap_rate": LEGACY_JV_CAP_RATE,
            "gross_rev_stabilized": LEGACY_JV_GROSS_REV_STABILIZED,
            "noi_stabilized": LEGACY_JV_NOI_STABILIZED,
            "opex_pct": LEGACY_JV_OPEX_PCT,
            "ga_annual": LEGACY_JV_GA_ANNUAL,
            "interest_rate": LEGACY_JV_INTEREST_RATE,
            "depreciation_yrs": LEGACY_JV_DEPRECIATION_YRS,
            "building_component": LEGACY_JV_BUILDING_COMPONENT,
            "draw_y1": LEGACY_JV_DRAW_Y1,
            "draw_y2": LEGACY_JV_DRAW_Y2,
            "draw_y3": LEGACY_JV_DRAW_Y3,
            "ltv_covenant": LEGACY_JV_LTV_COVENANT,
            "stabilized_av_ifrs": LEGACY_JV_STABILIZED_AV,
        },
        "years": years,
        "y10_endpoint": {
            "cumulative_distributions": y10.cumulative_distributions,
            "asset_value_aspe": y10.asset_value_aspe,
            "asset_value_ifrs_fv": LEGACY_JV_STABILIZED_AV,
            "equity_value_aspe_book": y10.equity_value,
            "equity_value_ifrs_fv": ifrs_fv_equity,
            "moic_at_book": moic_at_book,
            "moic_at_ifrs_fv": moic_at_fv,
            "single_shot_constraint": "Stabilized IFRS FV asset value = $63M NOI / 6.25% cap = $1,008M. 65% LTV covenant ceiling = $655M permanent debt. Existing $750M construction loan → $95M covenant gap requiring equity paydown at stabilization. No refinancing headroom for a Phase 2 development round; legacy JV structure is single-shot by design."
        }
    })
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    #[test]
    fn forecast_has_11_years() {
        let f = forecast();
        assert_eq!(f.len(), 11);
        assert_eq!(f[0].year, 0);
        assert_eq!(f[10].year, 10);
    }

    #[test]
    fn construction_phase_y1_y3() {
        let f = forecast();
        assert_eq!(f[1].phase, "Construction");
        assert_eq!(f[2].phase, "Construction");
        assert_eq!(f[3].phase, "Construction");
        assert_eq!(f[4].phase, "Stabilized");
        assert_eq!(f[10].phase, "Stabilized");
    }

    #[test]
    fn s_curve_construction_draws() {
        let f = forecast();
        assert!((f[1].capex - LEGACY_JV_DRAW_Y1).abs() < 1.0);
        assert!((f[2].capex - LEGACY_JV_DRAW_Y2).abs() < 1.0);
        assert!((f[3].capex - LEGACY_JV_DRAW_Y3).abs() < 1.0);
        assert!((f[3].cumulative_capex - LEGACY_JV_TOTAL_CAPITAL).abs() < 1.0);
    }

    #[test]
    fn y4_plus_stabilized_noi() {
        let f = forecast();
        for y in 4..=10 {
            assert!(
                (f[y].noi - 63_000_000.0).abs() < 100.0,
                "Y{} NOI = {} (expected ~$63M)",
                y,
                f[y].noi
            );
        }
    }

    #[test]
    fn y4_plus_distributable_cash_around_23m() {
        // Per BRIEF §2536: ~$23.5M/yr distributable cash
        let f = forecast();
        for y in 4..=10 {
            assert!(
                f[y].distributable_cash > 20_000_000.0 && f[y].distributable_cash < 27_000_000.0,
                "Y{} distributable cash = {} (expected ~$23.5M)",
                y,
                f[y].distributable_cash
            );
        }
    }

    #[test]
    fn y10_cumulative_distributions_around_164m() {
        // Per BRIEF §2558: ~$164M cumulative Y4-Y10 (7 years × ~$23.5M)
        let f = forecast();
        let y10 = &f[10];
        assert!(
            y10.cumulative_distributions > 150_000_000.0
                && y10.cumulative_distributions < 180_000_000.0,
            "Y10 cumulative distributions = {} (expected ~$164M)",
            y10.cumulative_distributions
        );
    }

    #[test]
    fn dscr_above_threshold_at_stabilized() {
        // DSCR = NOI / Interest = $63M / $37.5M = 1.68×. Lender covenant
        // threshold typically 1.20× minimum; 1.68× is comfortable.
        let f = forecast();
        for y in 4..=10 {
            assert!(
                f[y].dscr > 1.5,
                "Y{} DSCR = {} (expected ~1.68×)",
                y,
                f[y].dscr
            );
        }
    }

    #[test]
    fn json_well_formed() {
        let json = forecast_json();
        assert_eq!(json["version"], "V1");
        assert!(json["years"].is_array());
        assert!(json["y10_endpoint"]["moic_at_ifrs_fv"].as_f64().unwrap() > 1.5);
    }
}
