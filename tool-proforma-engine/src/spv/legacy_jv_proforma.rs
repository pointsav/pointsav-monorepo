// Legacy JV (D7) — Traditional Joint Venture proforma engine module.
// BRIEF v0.15.6 §5h; Jennifer decisions D7-1 through D7-5 (resolved 2026-06-02/06).
//
// Apples-to-apples comparator to PCLP 1 (D2). Same $250M gross equity but traditional
// bank-debt JV financing (3.0× leverage net, 65% LTV covenant), single development round.
//
// Structural form: traditional Inc. with shareholders agreement (not LP/GP).
// Fee structure: 2/20 — 2% management fee on committed equity + 20% annual carry above
// 8% preferred return.
//
// ASPE 3061 cost model. All Y1–Y3 costs (interest + management fee) capitalized.
// 50-year straight-line depreciation on fully-capitalized building component.
//
// Flag D7-4: $78.75M is NET NOI (tenant CAM pass-through). Engine does NOT apply
// portfolio-level opex on top — only the $5M/yr management fee is the portfolio overhead.

use serde::Serialize;

// ─── Constants ───────────────────────────────────────────────────────────────

// Capital structure
pub const LEGACY_JV_GROSS_EQUITY: f64 = 250_000_000.0; // gross subscribed by investors
pub const LEGACY_JV_ISSUANCE_COSTS: f64 = 10_000_000.0; // 1% mortgage + 1% equity formation
pub const LEGACY_JV_NET_EQUITY: f64 = 240_000_000.0; // deployed into project
pub const LEGACY_JV_BANK_DEBT: f64 = 750_000_000.0; // construction → permanent at Y4
pub const LEGACY_JV_TOTAL_CAPITAL: f64 = 990_000_000.0; // net equity + debt
pub const LEGACY_JV_DEBT_RATIO: f64 = 3.125; // $750M / $240M net equity

// Shares (Inc., not LP units; gross shares = gross equity / share price)
pub const LEGACY_JV_SHARES: f64 = 2_500_000.0; // $250M gross / $100/share
pub const LEGACY_JV_SHARE_PRICE: f64 = 100.0;

// Portfolio
pub const LEGACY_JV_TOTAL_SF: f64 = 2_298_150.0; // BRIEF §2469
pub const LEGACY_JV_COST_PER_SF: f64 = 326.35; // BRIEF §2468

// Yield + valuation
pub const LEGACY_JV_DEV_YIELD: f64 = 0.105; // 10.5%
pub const LEGACY_JV_CAP_RATE: f64 = 0.0625; // 6.25%
                                            // Flag D7-4: $78.75M is the NET development yield — tenant CAM pass-through means
                                            // building-level opex is already netted. Engine MUST NOT deduct opex again.
pub const LEGACY_JV_NOI_STABILIZED: f64 = 78_750_000.0; // net NOI; Flag D7-4
pub const LEGACY_JV_STABILIZED_AV: f64 = 1_260_000_000.0; // $78.75M / 6.25% cap rate

// Fee structure (2/20 Inc. management agreement)
pub const LEGACY_JV_INTEREST_RATE: f64 = 0.050; // 5% permanent loan
pub const LEGACY_JV_MGMT_FEE: f64 = 5_000_000.0; // 2% × $250M committed/yr (Y4+ only)
pub const LEGACY_JV_CARRY_PCT: f64 = 0.20; // 20% annual carry above hurdle
pub const LEGACY_JV_HURDLE: f64 = 20_000_000.0; // 8% preferred return × $250M

// Depreciation (ASPE 3061 — fully-capitalized building component)
// Hard cost:                                    $990M (net capital deployed)
// Capitalized Y1–Y3 interest:
//   Y1 $150M debt × 5% =   $7.50M
//   Y2 $525M debt × 5% =  $26.25M
//   Y3 $750M debt × 5% =  $37.50M   total $71.25M
// Capitalized Y1–Y3 management fee:  $5M × 3 = $15.00M
// Total building component:                   $1,086.25M
pub const LEGACY_JV_BUILDING_COMPONENT: f64 = 1_086_250_000.0;
pub const LEGACY_JV_DEPRECIATION_YRS: f64 = 50.0;
pub const LEGACY_JV_DEPRECIATION_ANNUAL: f64 = 21_725_000.0; // $1,086.25M / 50

// Construction draws — S-curve on $990M net capital (20/50/30; Flag D7-2)
pub const LEGACY_JV_DRAW_Y1: f64 = 198_000_000.0; // 20%
pub const LEGACY_JV_DRAW_Y2: f64 = 495_000_000.0; // 50%
pub const LEGACY_JV_DRAW_Y3: f64 = 297_000_000.0; // 30%

// Debt/equity share per draw ($750M / $990M = 75.76% debt, 24.24% equity)
const DEBT_SHARE: f64 = 750_000_000.0 / 990_000_000.0;
const EQUITY_SHARE: f64 = 240_000_000.0 / 990_000_000.0;

// LTV covenant
pub const LEGACY_JV_LTV_COVENANT: f64 = 0.65; // 65% max

// ─── Output struct ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct LegacyJvYear {
    pub year: u32,
    pub phase: &'static str, // "Pre-launch" | "Construction" | "Stabilized"
    pub capex: f64,          // S-curve draw (Y1–Y3 only)
    pub cumulative_capex: f64,
    pub debt_outstanding: f64,
    pub equity_contribution: f64, // equity drawn proportional to capex
    pub noi: f64,                 // net NOI (Y4+ only; Flag D7-4)
    pub interest_expense: f64,    // all years — Y1–Y3 capitalized (shown explicitly)
    pub depreciation: f64,        // Y4+ only (ASPE 3061)
    pub mgmt_fee: f64,            // all years — Y1–Y3 capitalized (shown explicitly)
    pub capitalized_costs: f64,   // = interest + mgmt_fee for Y1–Y3; 0 for Y4+
    pub net_income: f64,
    pub distributable_cash: f64,        // net_income + depreciation
    pub carry_to_mgmt: f64,             // 20% above 8% hurdle (Y4+)
    pub dividends_to_shareholders: f64, // distributable_cash − carry
    pub cumulative_dividends: f64,
    pub dps: f64,                  // dividends per share
    pub asset_value_aspe: f64,     // cumulative_capex − accumulated_depreciation
    pub shareholders_equity: f64,  // asset_value_aspe − debt (ASPE book)
    pub equity_value_ifrs_fv: f64, // STABILIZED_AV − debt (Y4+ only)
    pub ltv_book: f64,             // debt / asset_value_aspe
    pub dscr: f64,                 // noi / interest_expense
}

// ─── Forecast ─────────────────────────────────────────────────────────────────

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
    let mut cum_dividends: f64 = 0.0;
    let mut cum_depr: f64 = 0.0;

    // Y0: pre-launch (no capital deployed yet)
    years.push(LegacyJvYear {
        year: 0,
        phase: "Pre-launch",
        capex: 0.0,
        cumulative_capex: 0.0,
        debt_outstanding: 0.0,
        equity_contribution: 0.0,
        noi: 0.0,
        interest_expense: 0.0,
        depreciation: 0.0,
        mgmt_fee: 0.0,
        capitalized_costs: 0.0,
        net_income: 0.0,
        distributable_cash: 0.0,
        carry_to_mgmt: 0.0,
        dividends_to_shareholders: 0.0,
        cumulative_dividends: 0.0,
        dps: 0.0,
        asset_value_aspe: 0.0,
        shareholders_equity: 0.0,
        equity_value_ifrs_fv: 0.0,
        ltv_book: 0.0,
        dscr: 0.0,
    });

    for y in 1u32..=10 {
        let capex = capex_at(y);
        cum_capex += capex;

        let equity_contrib = capex * EQUITY_SHARE;

        // Debt drawn proportionally during construction; fixed at $750M from Y4
        let debt_outstanding = if y >= 4 {
            LEGACY_JV_BANK_DEBT
        } else {
            (cum_capex * DEBT_SHARE).min(LEGACY_JV_BANK_DEBT)
        };

        let phase = if y <= 3 { "Construction" } else { "Stabilized" };

        // Interest and management fee accrue in ALL years.
        // Y1–Y3: costs are capitalized into building component (ASPE 3061) — shown explicitly
        //        as gross amounts with an offsetting `capitalized_costs` row; net_income = 0.
        // Y4+:   costs hit the P&L normally.
        let noi = if y >= 4 {
            LEGACY_JV_NOI_STABILIZED
        } else {
            0.0
        };
        let interest = debt_outstanding * LEGACY_JV_INTEREST_RATE;
        let depr = if y >= 4 {
            LEGACY_JV_DEPRECIATION_ANNUAL
        } else {
            0.0
        };
        cum_depr += depr;
        let mgmt_fee = LEGACY_JV_MGMT_FEE;

        // ASPE 3061 offset: during construction the gross cost is capitalized, not expensed.
        let capitalized_costs = if y < 4 { interest + mgmt_fee } else { 0.0 };

        let net_income = noi - interest - depr - mgmt_fee + capitalized_costs;
        let distributable_cash = net_income + depr; // add back non-cash depreciation

        // 2/20 annual above-hurdle carry
        let carry = if distributable_cash > LEGACY_JV_HURDLE {
            (distributable_cash - LEGACY_JV_HURDLE) * LEGACY_JV_CARRY_PCT
        } else {
            0.0
        };
        let dividends = distributable_cash - carry;
        cum_dividends += dividends;
        let dps = dividends / LEGACY_JV_SHARES;

        // ASPE book: cumulative capex minus accumulated depreciation
        let asset_value_aspe = cum_capex - cum_depr;
        let shareholders_equity = asset_value_aspe - debt_outstanding;

        // IFRS FV equity: income-capitalized AV minus debt (Y4+ stabilized only)
        let equity_value_ifrs_fv = if y >= 4 {
            LEGACY_JV_STABILIZED_AV - LEGACY_JV_BANK_DEBT
        } else {
            0.0
        };

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
            noi,
            interest_expense: interest,
            depreciation: depr,
            mgmt_fee,
            capitalized_costs,
            net_income,
            distributable_cash,
            carry_to_mgmt: carry,
            dividends_to_shareholders: dividends,
            cumulative_dividends: cum_dividends,
            dps,
            asset_value_aspe,
            shareholders_equity,
            equity_value_ifrs_fv,
            ltv_book,
            dscr,
        });
    }
    years
}

pub fn forecast_json() -> serde_json::Value {
    let years = forecast();
    let y10 = &years[10];

    let moic_at_fv = (y10.cumulative_dividends + y10.equity_value_ifrs_fv) / LEGACY_JV_GROSS_EQUITY;
    let moic_at_book =
        (y10.cumulative_dividends + y10.shareholders_equity) / LEGACY_JV_GROSS_EQUITY;

    serde_json::json!({
        "entity": "Legacy JV (D7) — Traditional Joint Venture (Inc.)",
        "source": "tool-proforma-engine src/spv/legacy_jv_proforma module",
        "brief_section": "v0.15.6 §5h",
        "version": "V2",
        "generated_at": "2026-06-06",
        "purpose": "Apples-to-apples 10-year return comparator to PCLP 1 (D2)",
        "inputs": {
            "gross_equity": LEGACY_JV_GROSS_EQUITY,
            "issuance_costs": LEGACY_JV_ISSUANCE_COSTS,
            "net_equity": LEGACY_JV_NET_EQUITY,
            "bank_debt": LEGACY_JV_BANK_DEBT,
            "total_capital": LEGACY_JV_TOTAL_CAPITAL,
            "shares": LEGACY_JV_SHARES,
            "share_price": LEGACY_JV_SHARE_PRICE,
            "total_sf": LEGACY_JV_TOTAL_SF,
            "cost_per_sf": LEGACY_JV_COST_PER_SF,
            "dev_yield": LEGACY_JV_DEV_YIELD,
            "cap_rate": LEGACY_JV_CAP_RATE,
            "noi_stabilized": LEGACY_JV_NOI_STABILIZED,
            "stabilized_av_ifrs": LEGACY_JV_STABILIZED_AV,
            "interest_rate": LEGACY_JV_INTEREST_RATE,
            "mgmt_fee_annual": LEGACY_JV_MGMT_FEE,
            "carry_pct": LEGACY_JV_CARRY_PCT,
            "preferred_return_hurdle": LEGACY_JV_HURDLE,
            "depreciation_yrs": LEGACY_JV_DEPRECIATION_YRS,
            "building_component": LEGACY_JV_BUILDING_COMPONENT,
            "draw_y1": LEGACY_JV_DRAW_Y1,
            "draw_y2": LEGACY_JV_DRAW_Y2,
            "draw_y3": LEGACY_JV_DRAW_Y3,
            "ltv_covenant": LEGACY_JV_LTV_COVENANT,
        },
        "years": years,
        "y10_endpoint": {
            "cumulative_dividends": y10.cumulative_dividends,
            "asset_value_aspe_book": y10.asset_value_aspe,
            "asset_value_ifrs_fv": LEGACY_JV_STABILIZED_AV,
            "equity_value_aspe_book": y10.shareholders_equity,
            "equity_value_ifrs_fv": y10.equity_value_ifrs_fv,
            "moic_at_book": moic_at_book,
            "moic_at_ifrs_fv": moic_at_fv,
            "single_shot_constraint": "Stabilized IFRS FV AV = $78.75M NOI / 6.25% = $1,260M. Max permanent debt at 65% LTV = $819M. Existing $750M < $819M ceiling — $69M headroom. Structurally still single-shot: $69M is insufficient for a Phase 2 construction loan (~$750M required). No compounding without new equity injection."
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
    fn s_curve_draws_total_990m() {
        let f = forecast();
        assert!((f[1].capex - LEGACY_JV_DRAW_Y1).abs() < 1.0);
        assert!((f[2].capex - LEGACY_JV_DRAW_Y2).abs() < 1.0);
        assert!((f[3].capex - LEGACY_JV_DRAW_Y3).abs() < 1.0);
        assert!(
            (f[3].cumulative_capex - LEGACY_JV_TOTAL_CAPITAL).abs() < 1.0,
            "Y3 cumulative capex = {} (expected $990M)",
            f[3].cumulative_capex
        );
    }

    #[test]
    fn construction_years_zero_pnl() {
        // Y1–Y3: interest and mgmt_fee accrue at gross but are offset by capitalized_costs.
        // Net P&L must still be zero; distributable cash and dividends also zero.
        let f = forecast();
        for y in 1..=3 {
            assert_eq!(
                f[y].net_income, 0.0,
                "Y{y} net_income should be 0 (ASPE 3061 offset)"
            );
            assert_eq!(
                f[y].distributable_cash, 0.0,
                "Y{y} distributable_cash should be 0"
            );
            assert_eq!(
                f[y].dividends_to_shareholders, 0.0,
                "Y{y} dividends should be 0"
            );
            // Gross costs are now shown; capitalized_costs must equal interest + mgmt_fee
            assert!(
                f[y].interest_expense > 0.0,
                "Y{y} interest_expense should be positive (shown at gross)"
            );
            assert!(
                (f[y].capitalized_costs - (f[y].interest_expense + f[y].mgmt_fee)).abs() < 1.0,
                "Y{y} capitalized_costs ({}) must equal interest ({}) + mgmt_fee ({})",
                f[y].capitalized_costs,
                f[y].interest_expense,
                f[y].mgmt_fee
            );
        }
    }

    #[test]
    fn construction_capitalized_costs_correct() {
        let f = forecast();
        // Y1: debt = $150M; interest = $7.5M; mgmt = $5M; capitalized = $12.5M
        assert!(
            (f[1].interest_expense - 7_500_000.0).abs() < 100.0,
            "Y1 interest = {}",
            f[1].interest_expense
        );
        assert!(
            (f[1].capitalized_costs - 12_500_000.0).abs() < 100.0,
            "Y1 cap_costs = {}",
            f[1].capitalized_costs
        );
        // Y2: debt = $525M; interest = $26.25M; capitalized = $31.25M
        assert!(
            (f[2].interest_expense - 26_250_000.0).abs() < 100.0,
            "Y2 interest = {}",
            f[2].interest_expense
        );
        assert!(
            (f[2].capitalized_costs - 31_250_000.0).abs() < 100.0,
            "Y2 cap_costs = {}",
            f[2].capitalized_costs
        );
        // Y3: debt = $750M; interest = $37.5M; capitalized = $42.5M
        assert!(
            (f[3].interest_expense - 37_500_000.0).abs() < 100.0,
            "Y3 interest = {}",
            f[3].interest_expense
        );
        assert!(
            (f[3].capitalized_costs - 42_500_000.0).abs() < 100.0,
            "Y3 cap_costs = {}",
            f[3].capitalized_costs
        );
        // Y4+: capitalized_costs = 0
        for y in 4..=10 {
            assert_eq!(
                f[y].capitalized_costs, 0.0,
                "Y{y} capitalized_costs should be 0"
            );
        }
    }

    #[test]
    fn y4_plus_stabilized_noi() {
        // Flag D7-4: NOI = $78.75M net (no opex deduction)
        let f = forecast();
        for y in 4..=10 {
            assert!(
                (f[y].noi - 78_750_000.0).abs() < 100.0,
                "Y{y} NOI = {} (expected $78.75M per Flag D7-4)",
                f[y].noi
            );
        }
    }

    #[test]
    fn y4_plus_dscr_around_2x() {
        // DSCR = $78.75M NOI / $37.5M interest = 2.10×
        let f = forecast();
        for y in 4..=10 {
            assert!(
                (f[y].dscr - 2.10).abs() < 0.01,
                "Y{y} DSCR = {:.3} (expected ~2.10×)",
                f[y].dscr
            );
        }
    }

    #[test]
    fn y4_plus_dividends_around_33m() {
        // Distributable = $36.25M; carry = $3.25M; net dividends ≈ $33M
        let f = forecast();
        for y in 4..=10 {
            assert!(
                f[y].dividends_to_shareholders > 30_000_000.0
                    && f[y].dividends_to_shareholders < 36_000_000.0,
                "Y{y} dividends = {} (expected ~$33M)",
                f[y].dividends_to_shareholders
            );
        }
    }

    #[test]
    fn y10_cumulative_dividends_around_231m() {
        // 7 years × ~$33M = ~$231M
        let f = forecast();
        let y10 = &f[10];
        assert!(
            y10.cumulative_dividends > 210_000_000.0 && y10.cumulative_dividends < 250_000_000.0,
            "Y10 cumulative dividends = {} (expected ~$231M)",
            y10.cumulative_dividends
        );
    }

    #[test]
    fn ifrs_fv_equity_at_stabilization() {
        // IFRS FV equity = $1,260M AV - $750M debt = $510M from Y4+
        let f = forecast();
        for y in 4..=10 {
            assert!(
                (f[y].equity_value_ifrs_fv - 510_000_000.0).abs() < 1.0,
                "Y{y} IFRS FV equity = {} (expected $510M)",
                f[y].equity_value_ifrs_fv
            );
        }
    }

    #[test]
    fn moic_at_fv_around_296x() {
        let json = forecast_json();
        let moic = json["y10_endpoint"]["moic_at_ifrs_fv"].as_f64().unwrap();
        assert!(
            moic > 2.5 && moic < 3.5,
            "MOIC at IFRS FV = {moic:.3} (expected ~2.96×)"
        );
    }

    #[test]
    fn json_version_v2() {
        let json = forecast_json();
        assert_eq!(json["version"], "V2");
        assert!(json["years"].is_array());
    }
}
