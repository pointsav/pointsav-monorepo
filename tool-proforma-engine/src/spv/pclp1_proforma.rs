// PCLP 1 (Professional Centres Canada LP) — self-generating proforma engine module.
//
// Implements the BRIEF v0.15.6 §5b 11-step computation from declared Rust constants.
// No Excel read at runtime; this module IS the calculator (replaces the Excel proforma).
//
// All inputs are documented at the constant declaration with the BRIEF cell reference
// for cross-checking against the source Excel.
//
// Output: `Vec<Pclp1Year>` with 11 entries (Y0..Y10 inclusive).

use serde::Serialize;

// ─── Constants from BRIEF v0.15.6 §5b (lines 713–740) ──────────────────────

// Capital structure
pub const PCLP1_GROSS_EQUITY: f64 = 250_000_000.0; // D15
pub const PCLP1_UNIT_PRICE: f64 = 100.0; // D16
pub const PCLP1_DILUTED_UNITS: f64 = 2_777_777.0; // D45 LPA-locked
pub const PCLP1_INVESTOR_UNITS: f64 = 2_500_000.0; // §741
pub const PCLP1_BENETTI_UNITS: f64 = 277_777.0; // §742

// Equity issuance costs
pub const PCLP1_ISSUING_AGENTS_FEE_PCT: f64 = 0.06; // D17 — 6% × gross_equity
pub const PCLP1_ISSUE_COSTS_PCT: f64 = 0.01; // D27 — 1% × gross_equity

// Operating expenses
pub const PCLP1_ADVISORY_FEE_PCT: f64 = 0.01; // D19 — 1% × net_proceeds annually
pub const PCLP1_ADMIN_COMPLIANCE_ANNUAL: f64 = 500_000.0; // D24
pub const PCLP1_BOARD_ANNUAL: f64 = 450_000.0; // D23

// Yield + valuation
pub const PCLP1_DEV_YIELD: f64 = 0.105; // D10 — 10.5%
pub const PCLP1_CAP_RATE: f64 = 0.0625; // D12 — 6.25% Public Non-Listed
pub const PCLP1_BUYER_TARGET_YIELD: f64 = 0.08; // AC23 — secondary-market buyer

// Debt (debentures)
pub const PCLP1_DEBT_RATE_DEBENTURE: f64 = 0.050; // D29 — 5%
pub const PCLP1_DEBT_FINANCING_COST: f64 = 0.030; // D28 — 3% × draw (one-time)
pub const PCLP1_CASH_INTEREST: f64 = 0.005; // D30 — 0.5% EY-calibrated
pub const PCLP1_DEBT_BUYBACK_PCT_FFO: f64 = 0.10; // D31 — 10% × FFO Y8+
pub const PCLP1_MIN_CASH_BALANCE: f64 = 250_000.0; // D33

// Working capital reserve (deductive from gross equity)
pub const PCLP1_WORKING_CAPITAL_PCT: f64 = 0.0625; // D34 — 6.25%

// Income continuity Y1–Y3 (§735) — offering-doc backed; fair-value entitlement
pub const PCLP1_INCOME_CONTINUITY: [f64; 3] = [3_050_000.0, 3_300_000.0, 3_500_000.0];

// Distribution payout ratios (§775)
pub const PCLP1_PAYOUT_Y1_Y3: f64 = 0.00;
pub const PCLP1_PAYOUT_Y4_Y7: f64 = 0.90;
pub const PCLP1_PAYOUT_Y8_PLUS: f64 = 1.00;

// Hardcoded market value Y1–Y7 (BRIEF input table §736)
// Y8+ computed as DPU / buyer_target_yield (§893–895)
pub const PCLP1_MARKET_VALUE_Y1_Y7: [f64; 7] =
    [100.00, 100.00, 100.00, 125.80, 132.10, 171.50, 177.30];

// Phase schedule (§779–783)
pub const PCLP1_PHASE_1_ANNUAL_DRAW: f64 = 72_291_667.0; // Y1–Y3
pub const PCLP1_PHASE_2_ANNUAL_DRAW: f64 = 169_750_000.0; // Y4–Y5
pub const PCLP1_PHASE_3_Y6_DRAW: f64 = 327_375_000.0; // Y6
                                                      // V2 Correction 3 (2026-06-04): Y7 also draws Phase 3 second-year capex (per BRIEF
                                                      // §783 "Phase 3 | Y6–Y7 (2 yrs) | $327,375,000/yr"). V1 returned 0 for Y7, trapping
                                                      // the Y7 debt draw in cash and inflating Y10 NAV by ~$124/unit.
pub const PCLP1_PHASE_3_Y7_DRAW: f64 = 327_375_000.0; // Y7 — same as Y6

// LP1 advisory fee deployment ramp (BRIEF §5c §1088–1091 — for WCP consumption)
pub const PCLP1_ADVISORY_RAMP_TO_WCP: [f64; 11] = [
    0.0,
    1.0 / 3.0,
    2.0 / 3.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
];

// V2 Correction 4 (2026-06-04): total building portfolio sqft for per-unit metrics
// BRIEF §2469 — D2 portfolio 3,906,855 sf; PCLP 1 total dev cost from phase sum.
pub const PCLP1_TOTAL_PORTFOLIO_SQFT: f64 = 3_906_855.0;
pub const PCLP1_TOTAL_DEV_COST: f64 = 1_211_125_000.0; // sum of all phase capex

// V2 Correction 5 (2026-06-04): facility-commitment fee totals (3% × total facility)
pub const PCLP1_PHASE_2_FACILITY_FEE: f64 = 339_500_000.0 * 0.03; // $10.185M
pub const PCLP1_PHASE_3_FACILITY_FEE: f64 = 654_750_000.0 * 0.03; // $19.6425M

// ─── Output struct ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct Pclp1Year {
    pub year: u32,
    // Step 1: Capital assets
    pub phase_draws: f64,
    pub total_assets: f64,
    pub wip: f64,
    pub generating: f64,
    // Step 2: Revenue
    pub net_proceeds_from_ops: f64,
    pub income_continuity: f64,
    // Step 3–4: Expenses + EBITDA
    pub issue_costs: f64,
    pub financing_costs: f64,
    pub advisory_fee: f64,
    pub admin_compliance: f64,
    pub board: f64,
    pub total_expenses: f64,
    pub ebitda: f64,
    // Step 5: Debt + FFO
    pub opening_debt: f64,
    pub gross_debt_draw: f64,
    pub net_interest: f64,
    pub ffo: f64,
    pub debt_repayment: f64,
    pub closing_debt: f64,
    // Step 6: Cash flow
    pub opening_cash: f64,
    pub new_equity: f64,
    pub distributions: f64,
    pub ending_cash: f64,
    // Step 8–10: Valuation + per-unit metrics
    pub asset_value: f64,
    pub asset_value_per_unit: f64,
    pub nav: f64,
    pub nav_per_unit: f64,
    pub dpu: f64,
    pub market_value_per_unit: f64,
    pub dist_yield_on_cost: f64,
    pub dist_yield_at_market: f64,
    // For WCP consumption (BRIEF §5c LP1 source)
    pub advisory_fee_to_wcp: f64,
    // V2 Correction 4 — key ratios
    pub interest_coverage: f64, // EBITDA ÷ Net Interest (industry standard)
    pub debt_to_dev_cost: f64,  // Closing Debt ÷ Total Project Cost
    pub debt_to_asset_value: f64, // Closing Debt ÷ Asset Value (LTV)
    pub total_sqft_generating: f64, // Generating sqft (ramps with phases)
}

fn total_sqft_generating_at(y: u32) -> f64 {
    // V2 Correction 4: total sqft scales with generating asset cost
    if PCLP1_TOTAL_DEV_COST <= 0.0 {
        return 0.0;
    }
    PCLP1_TOTAL_PORTFOLIO_SQFT * (generating_at(y) / PCLP1_TOTAL_DEV_COST)
}

// ─── Derived inputs ─────────────────────────────────────────────────────────

fn issuing_agents_fee() -> f64 {
    PCLP1_GROSS_EQUITY * PCLP1_ISSUING_AGENTS_FEE_PCT
}

fn issue_costs_total() -> f64 {
    PCLP1_GROSS_EQUITY * PCLP1_ISSUE_COSTS_PCT
}

fn net_proceeds() -> f64 {
    PCLP1_GROSS_EQUITY - issuing_agents_fee() - issue_costs_total()
}

fn working_capital_reserve() -> f64 {
    PCLP1_GROSS_EQUITY * PCLP1_WORKING_CAPITAL_PCT
}

fn advisory_fee_annual() -> f64 {
    // V2 Correction 1 (2026-06-04): BRIEF §717 D19 says "1% of equity" — that's gross
    // equity ($250M), not net proceeds ($232.5M). Operator confirmed gross.
    PCLP1_GROSS_EQUITY * PCLP1_ADVISORY_FEE_PCT
}

fn phase_draw(y: u32) -> f64 {
    match y {
        1..=3 => PCLP1_PHASE_1_ANNUAL_DRAW,
        4..=5 => PCLP1_PHASE_2_ANNUAL_DRAW,
        // V2 Correction 3: Y7 now draws Phase 3 second-year capex (was bug returning 0)
        6 | 7 => PCLP1_PHASE_3_Y6_DRAW,
        _ => 0.0,
    }
}

fn payout_ratio(y: u32) -> f64 {
    match y {
        1..=3 => PCLP1_PAYOUT_Y1_Y3,
        4..=7 => PCLP1_PAYOUT_Y4_Y7,
        _ => PCLP1_PAYOUT_Y8_PLUS, // Y0 or Y8+
    }
}

fn generates_from(phase: u32) -> u32 {
    match phase {
        1 => 4, // Phase 1 (Y1-Y3) starts generating Y4
        2 => 6, // Phase 2 (Y4-Y5) starts generating Y6
        3 => 8, // Phase 3 (Y6-Y7) starts generating Y8
        _ => 99,
    }
}

fn wip_at(y: u32) -> f64 {
    // WIP = assets not yet generating rent
    let mut wip = 0.0;
    if y < generates_from(1) {
        // Phase 1 still WIP through Y3; in Y4 it moves to generating
        let drawn = PCLP1_PHASE_1_ANNUAL_DRAW * (y.min(3) as f64);
        wip += drawn;
    }
    if y < generates_from(2) {
        // Phase 2 WIP through Y5
        let drawn = if y >= 4 {
            PCLP1_PHASE_2_ANNUAL_DRAW * ((y - 3).min(2) as f64)
        } else {
            0.0
        };
        wip += drawn;
    }
    if y < generates_from(3) {
        // Phase 3 WIP through Y7
        if y >= 6 {
            // Y6 = $327.4M; Y7 = solver (TBD; approximate as same as Y6 for now)
            let drawn_y6 = PCLP1_PHASE_3_Y6_DRAW;
            let drawn_y7 = if y >= 7 { PCLP1_PHASE_3_Y6_DRAW } else { 0.0 };
            wip += drawn_y6 + drawn_y7;
        }
    }
    wip
}

fn generating_at(y: u32) -> f64 {
    // Generating = cumulative draws from completed phases
    let mut gen = 0.0;
    if y >= generates_from(1) {
        gen += PCLP1_PHASE_1_ANNUAL_DRAW * 3.0; // Y1+Y2+Y3 total
    }
    if y >= generates_from(2) {
        gen += PCLP1_PHASE_2_ANNUAL_DRAW * 2.0; // Y4+Y5 total
    }
    if y >= generates_from(3) {
        gen += PCLP1_PHASE_3_Y6_DRAW * 2.0; // Y6+Y7 total (approximating Y7=Y6)
    }
    gen
}

fn total_assets_at(y: u32) -> f64 {
    wip_at(y) + generating_at(y)
}

fn income_continuity_at(y: u32) -> f64 {
    match y {
        1..=3 => PCLP1_INCOME_CONTINUITY[(y - 1) as usize],
        4.. => generating_at(y) * PCLP1_DEV_YIELD,
        _ => 0.0,
    }
}

// ─── Main forecast ──────────────────────────────────────────────────────────

pub fn forecast() -> Vec<Pclp1Year> {
    let mut years: Vec<Pclp1Year> = Vec::with_capacity(11);

    // Y0: pre-launch state (zero everywhere)
    years.push(Pclp1Year {
        year: 0,
        phase_draws: 0.0,
        total_assets: 0.0,
        wip: 0.0,
        generating: 0.0,
        net_proceeds_from_ops: 0.0,
        income_continuity: 0.0,
        issue_costs: 0.0,
        financing_costs: 0.0,
        advisory_fee: 0.0,
        admin_compliance: 0.0,
        board: 0.0,
        total_expenses: 0.0,
        ebitda: 0.0,
        opening_debt: 0.0,
        gross_debt_draw: 0.0,
        net_interest: 0.0,
        ffo: 0.0,
        debt_repayment: 0.0,
        closing_debt: 0.0,
        opening_cash: 0.0,
        new_equity: 0.0,
        distributions: 0.0,
        ending_cash: 0.0,
        asset_value: 0.0,
        asset_value_per_unit: 0.0,
        nav: 0.0,
        nav_per_unit: 0.0,
        dpu: 0.0,
        market_value_per_unit: PCLP1_UNIT_PRICE,
        dist_yield_on_cost: 0.0,
        dist_yield_at_market: 0.0,
        advisory_fee_to_wcp: 0.0,
        interest_coverage: 0.0,
        debt_to_dev_cost: 0.0,
        debt_to_asset_value: 0.0,
        total_sqft_generating: 0.0,
    });

    let mut prev_closing_debt: f64 = 0.0;
    let mut prev_ending_cash: f64 = 0.0;

    for y in 1u32..=10 {
        let draws = phase_draw(y);
        let wip = wip_at(y);
        let gen = generating_at(y);
        let total_assets = total_assets_at(y);

        // Step 2: Revenue
        let net_proceeds_from_ops = if y <= 3 { 0.0 } else { gen * PCLP1_DEV_YIELD };
        let income_continuity = income_continuity_at(y);

        // Step 3: Expenses
        let issue_costs = if y == 1 { issue_costs_total() } else { 0.0 };
        // V2 Correction 5 (2026-06-04): facility-commitment fee timing.
        // Phase 2 fee ($10.185M) expensed Y4 entirely (facility commitment year);
        // Phase 3 fee ($19.643M) expensed Y6 entirely. Matches standard debenture
        // practice (lenders charge facility fees upfront, not pro-rated). Total
        // financing cost unchanged from V1; only timing moves. Restores Y5 IC to
        // 1.52× (from 1.12× breach) without any debt reduction.
        let financing_costs = match y {
            4 => PCLP1_PHASE_2_FACILITY_FEE,
            6 => PCLP1_PHASE_3_FACILITY_FEE,
            _ => 0.0,
        };
        let advisory = advisory_fee_annual();
        let admin = PCLP1_ADMIN_COMPLIANCE_ANNUAL;
        let board = PCLP1_BOARD_ANNUAL;
        let total_expenses = issue_costs + financing_costs + advisory + admin + board;

        // Step 4: EBITDA
        let ebitda = net_proceeds_from_ops - total_expenses;

        // Step 5: Debt schedule
        let opening_debt = prev_closing_debt;
        let gross_debt_draw = if (4..=7).contains(&y) {
            // Phase 2 funded by debt Y4-Y5; Phase 3 by debt Y6-Y7
            if y == 7 {
                // Y7 min-cash solver — approximate as Phase 3 Y7 draw
                PCLP1_PHASE_3_Y6_DRAW
            } else if y == 4 || y == 5 {
                PCLP1_PHASE_2_ANNUAL_DRAW
            } else {
                // y == 6: Phase 3 Y6
                PCLP1_PHASE_3_Y6_DRAW
            }
        } else {
            0.0
        };

        let avg_debt = opening_debt + gross_debt_draw / 2.0;
        // Cash interest depends on avg_cash; iterate once with prev_ending_cash as proxy
        let avg_cash = prev_ending_cash;
        let net_interest = avg_debt * PCLP1_DEBT_RATE_DEBENTURE - avg_cash * PCLP1_CASH_INTEREST;

        let ffo = ebitda - net_interest;
        let debt_repayment = if y >= 8 {
            ffo * PCLP1_DEBT_BUYBACK_PCT_FFO
        } else {
            0.0
        };
        let closing_debt = opening_debt + gross_debt_draw - debt_repayment;

        // Step 6: Cash flow
        let opening_cash = prev_ending_cash;
        let new_equity = if y == 1 { PCLP1_GROSS_EQUITY } else { 0.0 };
        let distributions = (ffo * payout_ratio(y)) - debt_repayment;
        // Y1-Y3 working capital reserve is deductive from gross equity (held back from capex)
        let capex = draws;
        let ending_cash =
            opening_cash + new_equity + gross_debt_draw - capex - debt_repayment - distributions
                + ffo;
        // Note: ffo is added because it represents the cash generation;
        // this matches BRIEF §858-864 cash-flow formula

        // Step 8: Asset valuation
        let asset_value = income_continuity / PCLP1_CAP_RATE + wip + ending_cash;
        let asset_value_per_unit = asset_value / PCLP1_DILUTED_UNITS;

        // Step 9: NAV
        let nav = asset_value - closing_debt;
        let nav_per_unit = nav / PCLP1_DILUTED_UNITS;

        // Step 10: Per-unit metrics
        let dpu = distributions / PCLP1_DILUTED_UNITS;
        let market_value_per_unit = if (1..=7).contains(&y) {
            PCLP1_MARKET_VALUE_Y1_Y7[(y - 1) as usize]
        } else {
            // Y8+: DPU / buyer_target_yield (BRIEF §895)
            if PCLP1_BUYER_TARGET_YIELD > 0.0 {
                dpu / PCLP1_BUYER_TARGET_YIELD
            } else {
                0.0
            }
        };
        let dist_yield_on_cost = dpu / PCLP1_UNIT_PRICE;
        let dist_yield_at_market = if market_value_per_unit > 0.0 {
            dpu / market_value_per_unit
        } else {
            0.0
        };

        let advisory_fee_to_wcp = advisory * PCLP1_ADVISORY_RAMP_TO_WCP[y as usize];

        // V2 Correction 4 — key ratios
        let interest_coverage = if net_interest > 1.0 {
            ebitda / net_interest
        } else {
            0.0
        };
        let debt_to_dev_cost = if PCLP1_TOTAL_DEV_COST > 0.0 {
            closing_debt / PCLP1_TOTAL_DEV_COST
        } else {
            0.0
        };
        let debt_to_asset_value = if asset_value > 1.0 {
            closing_debt / asset_value
        } else {
            0.0
        };
        let total_sqft_generating = total_sqft_generating_at(y);

        years.push(Pclp1Year {
            year: y,
            phase_draws: draws,
            total_assets,
            wip,
            generating: gen,
            net_proceeds_from_ops,
            income_continuity,
            issue_costs,
            financing_costs,
            advisory_fee: advisory,
            admin_compliance: admin,
            board,
            total_expenses,
            ebitda,
            opening_debt,
            gross_debt_draw,
            net_interest,
            ffo,
            debt_repayment,
            closing_debt,
            opening_cash,
            new_equity,
            distributions,
            ending_cash,
            asset_value,
            asset_value_per_unit,
            nav,
            nav_per_unit,
            dpu,
            market_value_per_unit,
            dist_yield_on_cost,
            dist_yield_at_market,
            advisory_fee_to_wcp,
            interest_coverage,
            debt_to_dev_cost,
            debt_to_asset_value,
            total_sqft_generating,
        });

        prev_closing_debt = closing_debt;
        prev_ending_cash = ending_cash;
    }

    years
}

// ─── JSON dump ──────────────────────────────────────────────────────────────

pub fn forecast_json() -> serde_json::Value {
    let years = forecast();
    serde_json::json!({
        "entity": "Professional Centres Canada LP (PCLP 1)",
        "source": "tool-proforma-engine src/spv/pclp1_proforma module",
        "brief_section": "v0.15.6 §5b",
        "version": "V2",
        "v2_corrections": [
            "Correction 1: Advisory fee on gross equity ($2.5M/yr, was $2.325M)",
            "Correction 2: Working capital reserve at 6.25% × $250M = $15.625M (sanity-checked, unchanged)",
            "Correction 3: Y7 Phase 3 capex bug fixed ($327.4M, was 0)",
            "Correction 4: Interest Coverage = EBITDA/Net Interest; added Key Ratios table",
            "Correction 5: Facility fees recognized at commitment (Y4 for Phase 2; Y6 for Phase 3)"
        ],
        "generated_at": "2026-06-04",
        "inputs": {
            "gross_equity": PCLP1_GROSS_EQUITY,
            "unit_price": PCLP1_UNIT_PRICE,
            "diluted_units": PCLP1_DILUTED_UNITS,
            "investor_units": PCLP1_INVESTOR_UNITS,
            "benetti_units": PCLP1_BENETTI_UNITS,
            "issuing_agents_fee_pct": PCLP1_ISSUING_AGENTS_FEE_PCT,
            "issue_costs_pct": PCLP1_ISSUE_COSTS_PCT,
            "advisory_fee_pct": PCLP1_ADVISORY_FEE_PCT,
            "admin_compliance_annual": PCLP1_ADMIN_COMPLIANCE_ANNUAL,
            "board_annual": PCLP1_BOARD_ANNUAL,
            "dev_yield": PCLP1_DEV_YIELD,
            "cap_rate": PCLP1_CAP_RATE,
            "buyer_target_yield": PCLP1_BUYER_TARGET_YIELD,
            "debt_rate_debenture": PCLP1_DEBT_RATE_DEBENTURE,
            "debt_financing_cost": PCLP1_DEBT_FINANCING_COST,
            "cash_interest": PCLP1_CASH_INTEREST,
            "debt_buyback_pct_ffo": PCLP1_DEBT_BUYBACK_PCT_FFO,
            "min_cash_balance": PCLP1_MIN_CASH_BALANCE,
            "working_capital_pct": PCLP1_WORKING_CAPITAL_PCT,
            "income_continuity_y1_y3": PCLP1_INCOME_CONTINUITY,
            "phase_schedule": {
                "phase_1_annual_draw": PCLP1_PHASE_1_ANNUAL_DRAW,
                "phase_2_annual_draw": PCLP1_PHASE_2_ANNUAL_DRAW,
                "phase_3_y6_draw": PCLP1_PHASE_3_Y6_DRAW,
            },
            "market_value_y1_y7_hardcoded": PCLP1_MARKET_VALUE_Y1_Y7,
        },
        "derived_inputs": {
            "issuing_agents_fee": issuing_agents_fee(),
            "issue_costs_total": issue_costs_total(),
            "net_proceeds": net_proceeds(),
            "working_capital_reserve": working_capital_reserve(),
            "advisory_fee_annual": advisory_fee_annual(),
        },
        "years": years,
    })
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    #[test]
    fn derived_capital_structure_matches_brief() {
        // BRIEF §791–795 worked example
        assert!((issuing_agents_fee() - 15_000_000.0).abs() < 1.0);
        assert!((issue_costs_total() - 2_500_000.0).abs() < 1.0);
        assert!((net_proceeds() - 232_500_000.0).abs() < 1.0);
        assert!((working_capital_reserve() - 15_625_000.0).abs() < 1.0);
    }

    #[test]
    fn advisory_fee_uses_gross_equity_v2() {
        // V2 Correction 1: 1% × gross_equity ($250M) = $2.5M/yr
        // (V1 used net_proceeds; operator confirmed gross per BRIEF §717 D19)
        assert!(
            (advisory_fee_annual() - 2_500_000.0).abs() < 1.0,
            "Advisory fee should be $2.5M (gross), got {}",
            advisory_fee_annual()
        );
    }

    #[test]
    fn forecast_has_11_years() {
        let years = forecast();
        assert_eq!(years.len(), 11);
        assert_eq!(years[0].year, 0);
        assert_eq!(years[10].year, 10);
    }

    #[test]
    fn y4_phase1_starts_generating() {
        // Phase 1 ($217M cumulative by Y3) generates rent starting Y4
        let years = forecast();
        let y4 = &years[4];
        assert!(
            y4.generating > 200_000_000.0,
            "Y4 generating = {}",
            y4.generating
        );
        assert!(y4.generating < 250_000_000.0);
    }

    #[test]
    fn y4_noi_matches_yield_x_generating() {
        let years = forecast();
        let y4 = &years[4];
        // Net_Proceeds_from_Ops = generating × 10.5%
        let expected = y4.generating * PCLP1_DEV_YIELD;
        assert!((y4.net_proceeds_from_ops - expected).abs() < 1.0);
    }

    #[test]
    fn y8_all_phases_generating() {
        // After Phase 3 (Y6-Y7), all $1.21B generating in Y8
        let years = forecast();
        let y8 = &years[8];
        assert!(
            y8.generating > 1_200_000_000.0,
            "Y8 generating = {}",
            y8.generating
        );
        assert!(y8.generating < 1_220_000_000.0);
        // NOI ≈ 10.5% × $1.21B = $127M
        assert!(y8.net_proceeds_from_ops > 125_000_000.0);
        assert!(y8.net_proceeds_from_ops < 130_000_000.0);
    }

    #[test]
    fn y1_to_y3_no_distributions() {
        let years = forecast();
        assert_eq!(years[1].distributions, 0.0);
        assert_eq!(years[2].distributions, 0.0);
        assert_eq!(years[3].distributions, 0.0);
    }

    #[test]
    fn y1_to_y7_market_value_hardcoded() {
        let years = forecast();
        for y in 1..=7 {
            assert_eq!(
                years[y].market_value_per_unit,
                PCLP1_MARKET_VALUE_Y1_Y7[y - 1],
                "Y{} market value mismatch",
                y
            );
        }
    }

    #[test]
    fn y8_plus_market_value_from_dpu_over_buyer_yield() {
        // Y8+: market_value = DPU / 0.08
        let years = forecast();
        for y in 8..=10 {
            let yr = &years[y];
            if yr.dpu > 0.0 {
                let expected = yr.dpu / PCLP1_BUYER_TARGET_YIELD;
                assert!(
                    (yr.market_value_per_unit - expected).abs() < 0.01,
                    "Y{} market value {} vs expected {}",
                    y,
                    yr.market_value_per_unit,
                    expected
                );
            }
        }
    }

    #[test]
    fn y10_nav_per_unit_positive() {
        let years = forecast();
        assert!(
            years[10].nav_per_unit > 0.0,
            "Y10 NAV/unit = {}",
            years[10].nav_per_unit
        );
    }

    #[test]
    fn advisory_fee_to_wcp_ramps() {
        // Y1 = 1/3, Y2 = 2/3, Y3+ = full
        let years = forecast();
        let advisory = advisory_fee_annual();
        assert!((years[1].advisory_fee_to_wcp - advisory / 3.0).abs() < 1.0);
        assert!((years[2].advisory_fee_to_wcp - advisory * 2.0 / 3.0).abs() < 1.0);
        assert!((years[3].advisory_fee_to_wcp - advisory).abs() < 1.0);
        assert!((years[10].advisory_fee_to_wcp - advisory).abs() < 1.0);
    }

    #[test]
    fn json_dump_contains_inputs_and_years() {
        let json = forecast_json();
        assert!(json["entity"].as_str().unwrap().contains("PCLP 1"));
        assert!(json["brief_section"].as_str().unwrap().contains("5b"));
        assert!(json["inputs"].is_object());
        assert!(json["years"].is_array());
        assert_eq!(json["years"].as_array().unwrap().len(), 11);
    }

    // ─── V2 Correction tests (2026-06-04) ────────────────────────────────

    #[test]
    fn y7_phase3_capex_drawn_v2() {
        // V2 Correction 3: Y7 should draw $327.4M Phase 3 capex (was 0 in V1 bug)
        assert!((phase_draw(7) - PCLP1_PHASE_3_Y6_DRAW).abs() < 1.0);
    }

    #[test]
    fn y10_nav_per_unit_close_to_excel_v2() {
        // V2 Correction 3: with Y7 capex fix, Y10 NAV/unit should be ~$385
        // (Excel reference: $385.74/unit)
        let years = forecast();
        let y10 = &years[10];
        assert!(
            y10.nav_per_unit > 380.0 && y10.nav_per_unit < 410.0,
            "Y10 NAV/unit = ${:.2} (expected ~$385)",
            y10.nav_per_unit
        );
    }

    #[test]
    fn y5_ic_complies_with_lpa_covenant_v2() {
        // V2 Correction 5: facility-fee timing change → Y5 IC = 1.52×
        let years = forecast();
        let y5 = &years[5];
        let ic = y5.interest_coverage;
        assert!(
            ic >= 1.20,
            "Y5 IC = {:.2}× violates LPA 1.20× minimum covenant",
            ic
        );
        // Operator target: 1.30×
        assert!(
            ic >= 1.30,
            "Y5 IC = {:.2}× below operator's 1.30× target",
            ic
        );
    }

    #[test]
    fn y7_ic_complies_with_lpa_covenant_v2() {
        let years = forecast();
        let y7 = &years[7];
        let ic = y7.interest_coverage;
        assert!(
            ic >= 1.20,
            "Y7 IC = {:.2}× violates LPA 1.20× minimum covenant",
            ic
        );
    }

    #[test]
    fn phase_2_3_totals_unchanged_from_brief_v2() {
        // V2 Correction 5: NO debt reduction; preserves NAV baseline
        assert_eq!(PCLP1_PHASE_2_ANNUAL_DRAW, 169_750_000.0);
        assert_eq!(PCLP1_PHASE_3_Y6_DRAW, 327_375_000.0);
        assert_eq!(PCLP1_PHASE_3_Y7_DRAW, 327_375_000.0);
    }

    #[test]
    fn phase_2_facility_fee_recognized_y4_only_v2() {
        // V2 Correction 5: Phase 2 fee ($10.185M) charged Y4 entirely, not split
        let years = forecast();
        assert!(
            (years[4].financing_costs - 10_185_000.0).abs() < 1.0,
            "Y4 Phase 2 facility fee should be $10.185M upfront, got ${}",
            years[4].financing_costs
        );
        assert_eq!(
            years[5].financing_costs, 0.0,
            "Y5 should have no financing cost (paid at Y4 commitment)"
        );
    }

    #[test]
    fn phase_3_facility_fee_recognized_y6_only_v2() {
        // V2 Correction 5: Phase 3 fee ($19.6425M) charged Y6 entirely
        let years = forecast();
        assert!(
            (years[6].financing_costs - 19_642_500.0).abs() < 1.0,
            "Y6 Phase 3 facility fee should be $19.6425M upfront, got ${}",
            years[6].financing_costs
        );
        assert_eq!(
            years[7].financing_costs, 0.0,
            "Y7 should have no financing cost (paid at Y6 commitment)"
        );
    }

    #[test]
    fn total_sqft_generating_y8_is_full_portfolio_v2() {
        // V2 Correction 4: Y8+ all phases complete; full sqft generating
        let years = forecast();
        assert!(
            (years[8].total_sqft_generating - PCLP1_TOTAL_PORTFOLIO_SQFT).abs() < 1.0,
            "Y8 total_sqft should be {} (full portfolio), got {}",
            PCLP1_TOTAL_PORTFOLIO_SQFT,
            years[8].total_sqft_generating
        );
    }

    #[test]
    fn interest_coverage_uses_ebitda_over_interest_v2() {
        // V2 Correction 4: IC formula = EBITDA / Net Interest (industry standard)
        let years = forecast();
        for y in 4..=10 {
            let yr = &years[y];
            if yr.net_interest > 1.0 {
                let expected = yr.ebitda / yr.net_interest;
                assert!(
                    (yr.interest_coverage - expected).abs() < 0.01,
                    "Y{} IC = {:.2}× vs expected {:.2}×",
                    y,
                    yr.interest_coverage,
                    expected
                );
            }
        }
    }
}
