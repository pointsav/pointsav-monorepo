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

// ─── Three-mode scenario types (V3 engine — sensitivity V8) ──────────────────
//
// The engine supports three professionally-distinct scenario constructions.
// Terminology confirmed against Big-4 assurance vocabulary; see audit notes in
// report/pclp1_sensitivity_v8.rs.
//
//   1. ManagedDownside    — "Management Response Scenario." The LP re-runs the
//      proforma from Y0 under stressed inputs and CONSTRAINS debenture issuance
//      + development scale so interest coverage never drops below the covenant
//      floor. Returns fall; no breach; no dispositions. (MD&A, forward-looking.)
//
//   2. SingleInputStress  — "Single-Input Sensitivity / Break-even Analysis"
//      (IFRS 13 §93(h)(ii)). One unobservable input moves; everything else is
//      held constant (ceteris paribus). Shows the exact breach threshold. No
//      management response is assumed.
//
//   3. CovenantCure { shock_year } — "Covenant Cure (Corrective Disposition)
//      Scenario." The portfolio is built out under base conditions, then a
//      combined shock lands at `shock_year`. The LP disposes of the MINIMUM
//      portfolio fraction, valued at STRESSED cap rates, required to restore the
//      coverage covenant. (MD&A; carries the orderly-transaction caveat.)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum ModelMode {
    /// LP constrains debenture issuance + development scale to hold coverage ≥ floor.
    #[default]
    ManagedDownside,
    /// Ceteris paribus: one input changes, all else held constant. Shows breach.
    SingleInputStress,
    /// Post-construction combined shock; minimum corrective disposition at stressed values.
    CovenantCure { shock_year: u32 },
}

/// Full input set for one forecast run. All fields default to the engine base
/// constants; stressed scenarios override individual fields.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ForecastParams {
    /// Stabilised occupancy multiplier. 1.0 = engine base; 0.95 = 95% occupancy.
    pub occupancy_pct: f64,
    /// Development yield on cost. Base PCLP1_DEV_YIELD = 0.105.
    pub dev_yield: f64,
    /// Debenture coupon. Base PCLP1_DEBT_RATE_DEBENTURE = 0.050.
    pub debt_rate: f64,
    /// Capitalisation rate for valuation (and stressed disposition value). Base 0.0625.
    pub cap_rate: f64,
    /// Months to stabilised occupancy. Base 12.0 (≤12 ⇒ no lease-up drag).
    pub lease_up_months: f64,
    /// Secondary-market buyer target yield (Y8+ market value). Base 0.080.
    pub market_yield: f64,
    /// Interest-coverage covenant floor. Base 1.20×.
    pub dscr_floor: f64,
    /// Scenario construction mode.
    pub mode: ModelMode,
}

impl Default for ForecastParams {
    fn default() -> Self {
        ForecastParams {
            occupancy_pct: 1.0,
            dev_yield: PCLP1_DEV_YIELD,
            debt_rate: PCLP1_DEBT_RATE_DEBENTURE,
            cap_rate: PCLP1_CAP_RATE,
            lease_up_months: 12.0,
            market_yield: PCLP1_BUYER_TARGET_YIELD,
            dscr_floor: 1.20,
            mode: ModelMode::ManagedDownside,
        }
    }
}

/// Which covenant drove a corrective disposition.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum DispositionTrigger {
    /// Interest-coverage (DSCR) covenant.
    Dscr,
    /// NAV-per-unit floor (informational only — never curable by disposition).
    NavFloor,
}

/// Result of a covenant-cure corrective-disposition solve at the shock year.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Pclp1DispositionEvent {
    pub shock_year: u32,
    /// Minimum generating-asset fraction sold, s ∈ [0, 1]. 1.0 when incurable.
    pub fraction_sold: f64,
    /// True when full-portfolio disposition cannot restore the covenant.
    pub incurable: bool,
    pub trigger: DispositionTrigger,
    /// s × (stressed market value of generating assets).
    pub sale_value_total: f64,
    /// Debt retired by the disposition (= sale_value_total; all proceeds deleverage).
    pub debt_retired: f64,
    /// Coverage after the cure (≈ floor when curable; pre-cure value when not).
    pub dscr_post_cure: f64,
    /// NAV/unit after the cure (equals the stressed NAV/unit — see note below).
    pub nav_per_unit_post_cure: f64,
    /// Generating portfolio retained, (1 − s) × 100.
    pub buildings_remaining_pct: f64,
    /// Distributable cash per unit after debt service on the retained portfolio
    /// (post-shock, post-disposition) = (EBITDA_remaining − interest_remaining) / units,
    /// floored at 0. The income a holder still sees once the shock and the cure have run.
    pub dist_per_unit_post_cure: f64,
    /// ALWAYS false. NAV = MV − Debt is invariant to the sale fraction when assets
    /// are sold at market value (the s·MV terms in assets and proceeds cancel).
    /// The binding cure mechanism is therefore always the coverage covenant.
    pub nav_curable_by_disposition: bool,
}

/// Metadata describing how a forecast run resolved (scales, trigger year, disposition).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ForecastMeta {
    /// Phase 2 issuance scale actually drawn, s2 ∈ [0, 1].
    pub phase2_scale: f64,
    /// Phase 3 issuance scale actually drawn, s3 ∈ [0, 1].
    pub phase3_scale: f64,
    /// First year at which a management-response constraint bound (scale < 1).
    pub dscr_constraint_triggered_year: Option<u32>,
    /// Generating portfolio built relative to the full base programme, ×100.
    pub buildings_built_pct: f64,
    pub mode: ModelMode,
    /// Populated only in CovenantCure mode.
    pub disposition: Option<Pclp1DispositionEvent>,
}

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
    // V3 (three-mode engine): issuance scales actually drawn this run.
    // Both 1.0 in the base/default run — preserves backward compatibility.
    pub phase2_scale: f64,
    pub phase3_scale: f64,
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

// ─── Scaled / parameterised helpers (V3 three-mode engine) ───────────────────
//
// Each mirrors its unscaled counterpart above but accepts Phase 2 / Phase 3
// issuance scales (s2, s3). At s2 = s3 = 1.0 the scaled output equals the
// unscaled output exactly — this is what preserves backward compatibility.

fn phase_draw_scaled(y: u32, s2: f64, s3: f64) -> f64 {
    match y {
        1..=3 => PCLP1_PHASE_1_ANNUAL_DRAW,
        4..=5 => PCLP1_PHASE_2_ANNUAL_DRAW * s2,
        6 | 7 => PCLP1_PHASE_3_Y6_DRAW * s3,
        _ => 0.0,
    }
}

fn wip_scaled_at(y: u32, s2: f64, s3: f64) -> f64 {
    let mut wip = 0.0;
    if y < generates_from(1) {
        wip += PCLP1_PHASE_1_ANNUAL_DRAW * (y.min(3) as f64);
    }
    if y < generates_from(2) && y >= 4 {
        wip += PCLP1_PHASE_2_ANNUAL_DRAW * s2 * ((y - 3).min(2) as f64);
    }
    if y < generates_from(3) && y >= 6 {
        let drawn_y6 = PCLP1_PHASE_3_Y6_DRAW * s3;
        let drawn_y7 = if y >= 7 { PCLP1_PHASE_3_Y6_DRAW * s3 } else { 0.0 };
        wip += drawn_y6 + drawn_y7;
    }
    wip
}

fn generating_scaled_at(y: u32, s2: f64, s3: f64) -> f64 {
    let mut gen = 0.0;
    if y >= generates_from(1) {
        gen += PCLP1_PHASE_1_ANNUAL_DRAW * 3.0;
    }
    if y >= generates_from(2) {
        gen += PCLP1_PHASE_2_ANNUAL_DRAW * 2.0 * s2;
    }
    if y >= generates_from(3) {
        gen += PCLP1_PHASE_3_Y6_DRAW * 2.0 * s3;
    }
    gen
}

/// Average in-place occupancy during generating-year `cur_year` for a phase that
/// began generating in `gen_start`, under a linear lease-up of `months`.
/// Returns 1.0 for `months ≤ 12` (engine base ⇒ no lease-up drag), preserving
/// backward compatibility. Applied to operating NOI only — never to the
/// stabilised valuation input (avoids double-penalising NAV for a timing effect).
fn noi_ramp_factor(gen_start: u32, cur_year: u32, months: f64) -> f64 {
    if cur_year < gen_start {
        return 0.0;
    }
    if months <= 12.0 {
        return 1.0;
    }
    let stabilize_years = months / 12.0;
    let k = (cur_year - gen_start) as f64; // 0 = first generating year
    let start = (k / stabilize_years).min(1.0);
    let end = ((k + 1.0) / stabilize_years).min(1.0);
    (start + end) / 2.0
}

/// Operating NOI at year `y` with occupancy, dev-yield, and lease-up ramp applied
/// per phase. Drives `net_proceeds_from_ops` (the cash NOI line).
fn ramped_noi_at(y: u32, s2: f64, s3: f64, p: &ForecastParams) -> f64 {
    let mut noi = 0.0;
    if y >= generates_from(1) {
        let amt = PCLP1_PHASE_1_ANNUAL_DRAW * 3.0;
        noi += amt * p.dev_yield * p.occupancy_pct
            * noi_ramp_factor(generates_from(1), y, p.lease_up_months);
    }
    if y >= generates_from(2) {
        let amt = PCLP1_PHASE_2_ANNUAL_DRAW * 2.0 * s2;
        noi += amt * p.dev_yield * p.occupancy_pct
            * noi_ramp_factor(generates_from(2), y, p.lease_up_months);
    }
    if y >= generates_from(3) {
        let amt = PCLP1_PHASE_3_Y6_DRAW * 2.0 * s3;
        noi += amt * p.dev_yield * p.occupancy_pct
            * noi_ramp_factor(generates_from(3), y, p.lease_up_months);
    }
    noi
}

/// Stabilised income for valuation at year `y`: scaled generating × dev-yield ×
/// occupancy, WITHOUT the lease-up ramp (valuation uses stabilised, not timing-
/// discounted, income). Y1–Y3 use the contractual income-continuity entitlements.
fn stabilized_income_at(y: u32, s2: f64, s3: f64, p: &ForecastParams) -> f64 {
    match y {
        1..=3 => PCLP1_INCOME_CONTINUITY[(y - 1) as usize],
        4.. => generating_scaled_at(y, s2, s3) * p.dev_yield * p.occupancy_pct,
        _ => 0.0,
    }
}

/// Fixed operating expenses (do not scale with portfolio): advisory + admin + board.
fn fixed_operating_expenses() -> f64 {
    advisory_fee_annual() + PCLP1_ADMIN_COMPLIANCE_ANNUAL + PCLP1_BOARD_ANNUAL
}

/// Full generating base at stabilisation (Y8, all phases, unscaled) — the
/// denominator for `buildings_built_pct`.
fn full_generating_base() -> f64 {
    PCLP1_PHASE_1_ANNUAL_DRAW * 3.0 + PCLP1_PHASE_2_ANNUAL_DRAW * 2.0 + PCLP1_PHASE_3_Y6_DRAW * 2.0
}

// ─── Main forecast ──────────────────────────────────────────────────────────

/// Backward-compatible entry point: base parameters, ManagedDownside mode.
/// At default params both issuance scales clamp to 1.0 and the lease-up ramp is
/// inert (12-month base), so this reproduces the V2 hardcoded forecast exactly.
pub fn forecast() -> Vec<Pclp1Year> {
    forecast_full(&ForecastParams::default()).0
}

/// Run the forecast under arbitrary parameters; return the year series only.
pub fn forecast_with_params(p: &ForecastParams) -> Vec<Pclp1Year> {
    forecast_full(p).0
}

/// Run the forecast and return both the 11-year series and resolution metadata.
/// Dispatches on `p.mode`.
pub fn forecast_full(p: &ForecastParams) -> (Vec<Pclp1Year>, ForecastMeta) {
    match p.mode {
        ModelMode::CovenantCure { shock_year } => forecast_covenant_cure(p, shock_year),
        ModelMode::ManagedDownside | ModelMode::SingleInputStress => {
            forecast_managed_or_static(p)
        }
    }
}

/// Builds an empty Y0 pre-launch row (zero everywhere; market value at par).
fn y0_row() -> Pclp1Year {
    Pclp1Year {
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
        phase2_scale: 1.0,
        phase3_scale: 1.0,
    }
}

/// ManagedDownside + SingleInputStress modes. The only difference: ManagedDownside
/// constrains Phase 2/3 issuance scales to hold interest coverage ≥ floor;
/// SingleInputStress fixes both scales at 1.0 and lets the breach surface.
fn forecast_managed_or_static(p: &ForecastParams) -> (Vec<Pclp1Year>, ForecastMeta) {
    let adaptive = p.mode == ModelMode::ManagedDownside;
    let mut years: Vec<Pclp1Year> = Vec::with_capacity(11);
    years.push(y0_row());

    let mut phase2_scale: f64 = 1.0;
    let mut phase3_scale: f64 = 1.0;
    let mut triggered_year: Option<u32> = None;

    let mut prev_closing_debt: f64 = 0.0;
    let mut prev_ending_cash: f64 = 0.0;

    for y in 1u32..=10 {
        // ── Management-response issuance decisions (taken before drawing) ──
        // Phase 2 sized at Y4 to hold projected Y5 coverage ≥ floor.
        // avg_debt_Y5 = 1.5·s2·P2_ANNUAL = 0.75·s2·P2_TOTAL ⇒ the 0.75 coefficient.
        // The projection uses the LEASE-UP-ADJUSTED income the LP will actually earn
        // (the covenant is tested on actual, not stabilised, NOI). A slower lease-up
        // depresses decision-year coverage ⇒ debt issued more slowly ⇒ fewer buildings.
        if y == 4 && adaptive {
            let p1_gen = PCLP1_PHASE_1_ANNUAL_DRAW * 3.0;
            let ramp_p1_y5 = noi_ramp_factor(generates_from(1), 5, p.lease_up_months);
            let proj_noi_y5 = p1_gen * p.dev_yield * p.occupancy_pct * ramp_p1_y5;
            let proj_ebitda_y5 = proj_noi_y5 - fixed_operating_expenses();
            let p2_total = PCLP1_PHASE_2_ANNUAL_DRAW * 2.0;
            let denom = p.dscr_floor * 0.75 * p2_total * p.debt_rate;
            let max_s2 = if denom > 0.0 { proj_ebitda_y5 / denom } else { 1.0 };
            phase2_scale = max_s2.clamp(0.0, 1.0);
            if phase2_scale < 1.0 {
                triggered_year = triggered_year.or(Some(5));
            }
        }
        // Phase 3 sized at Y6 to hold projected Y7 coverage ≥ floor, net of the Phase 2
        // debt already committed. Lease-up-adjusted income, per the Phase 2 note above.
        if y == 6 && adaptive {
            let p1_gen = PCLP1_PHASE_1_ANNUAL_DRAW * 3.0;
            let p2_gen_scaled = PCLP1_PHASE_2_ANNUAL_DRAW * 2.0 * phase2_scale;
            let ramp_p1_y7 = noi_ramp_factor(generates_from(1), 7, p.lease_up_months);
            let ramp_p2_y7 = noi_ramp_factor(generates_from(2), 7, p.lease_up_months);
            let proj_noi_y7 =
                (p1_gen * ramp_p1_y7 + p2_gen_scaled * ramp_p2_y7) * p.dev_yield * p.occupancy_pct;
            let proj_ebitda_y7 = proj_noi_y7 - fixed_operating_expenses();
            let p2_total = PCLP1_PHASE_2_ANNUAL_DRAW * 2.0;
            let p3_total = PCLP1_PHASE_3_Y6_DRAW * 2.0;
            let lhs = proj_ebitda_y7 / p.dscr_floor - phase2_scale * p2_total * p.debt_rate;
            let rhs = 0.75 * p3_total * p.debt_rate;
            let max_s3 = if rhs > 0.0 && lhs > 0.0 { (lhs / rhs).min(1.0) } else { 0.0 };
            phase3_scale = max_s3.clamp(0.0, 1.0);
            if phase3_scale < 1.0 {
                triggered_year = triggered_year.or(Some(7));
            }
        }

        let s2 = phase2_scale;
        let s3 = phase3_scale;

        let draws = phase_draw_scaled(y, s2, s3);
        let wip = wip_scaled_at(y, s2, s3);
        let gen = generating_scaled_at(y, s2, s3);
        let total_assets = wip + gen;

        // Step 2: Revenue (occupancy + dev-yield + lease-up applied via ramped_noi_at)
        let net_proceeds_from_ops = if y <= 3 { 0.0 } else { ramped_noi_at(y, s2, s3, p) };
        let income_continuity = stabilized_income_at(y, s2, s3, p);

        // Step 3: Expenses. Facility-commitment fees scale with the committed
        // facility (s2 / s3); at base scale they equal the V2 amounts.
        let issue_costs = if y == 1 { issue_costs_total() } else { 0.0 };
        let financing_costs = match y {
            4 => PCLP1_PHASE_2_FACILITY_FEE * s2,
            6 => PCLP1_PHASE_3_FACILITY_FEE * s3,
            _ => 0.0,
        };
        let advisory = advisory_fee_annual();
        let admin = PCLP1_ADMIN_COMPLIANCE_ANNUAL;
        let board = PCLP1_BOARD_ANNUAL;
        let total_expenses = issue_costs + financing_costs + advisory + admin + board;

        // Step 4: EBITDA
        let ebitda = net_proceeds_from_ops - total_expenses;

        // Step 5: Debt schedule (draws scale with issuance decisions)
        let opening_debt = prev_closing_debt;
        let gross_debt_draw = match y {
            4 | 5 => PCLP1_PHASE_2_ANNUAL_DRAW * s2,
            6 | 7 => PCLP1_PHASE_3_Y6_DRAW * s3,
            _ => 0.0,
        };

        let avg_debt = opening_debt + gross_debt_draw / 2.0;
        let avg_cash = prev_ending_cash;
        let net_interest = avg_debt * p.debt_rate - avg_cash * PCLP1_CASH_INTEREST;

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
        let capex = draws;
        let ending_cash =
            opening_cash + new_equity + gross_debt_draw - capex - debt_repayment - distributions
                + ffo;

        // Step 8: Asset valuation (cap rate is a stressed input)
        let asset_value = income_continuity / p.cap_rate + wip + ending_cash;
        let asset_value_per_unit = asset_value / PCLP1_DILUTED_UNITS;

        // Step 9: NAV
        let nav = asset_value - closing_debt;
        let nav_per_unit = nav / PCLP1_DILUTED_UNITS;

        // Step 10: Per-unit metrics
        let dpu = distributions / PCLP1_DILUTED_UNITS;
        let market_value_per_unit = if (1..=7).contains(&y) {
            PCLP1_MARKET_VALUE_Y1_Y7[(y - 1) as usize]
        } else if p.market_yield > 0.0 {
            dpu / p.market_yield
        } else {
            0.0
        };
        let dist_yield_on_cost = dpu / PCLP1_UNIT_PRICE;
        let dist_yield_at_market = if market_value_per_unit > 0.0 {
            dpu / market_value_per_unit
        } else {
            0.0
        };

        let advisory_fee_to_wcp = advisory * PCLP1_ADVISORY_RAMP_TO_WCP[y as usize];

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
        let total_sqft_generating = if PCLP1_TOTAL_DEV_COST > 0.0 {
            PCLP1_TOTAL_PORTFOLIO_SQFT * (gen / PCLP1_TOTAL_DEV_COST)
        } else {
            0.0
        };

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
            phase2_scale: s2,
            phase3_scale: s3,
        });

        prev_closing_debt = closing_debt;
        prev_ending_cash = ending_cash;
    }

    let buildings_built_pct = {
        let base = full_generating_base();
        if base > 0.0 {
            (generating_scaled_at(10, phase2_scale, phase3_scale) / base) * 100.0
        } else {
            0.0
        }
    };

    let meta = ForecastMeta {
        phase2_scale,
        phase3_scale,
        dscr_constraint_triggered_year: triggered_year,
        buildings_built_pct,
        mode: p.mode,
        disposition: None,
    };

    (years, meta)
}

/// Covenant Cure (Corrective Disposition) Scenario.
///
/// The portfolio is built out under BASE conditions (the LP did not foresee the
/// shock), then a combined shock — carried in `p` — lands at `shock_year`. The LP
/// disposes of the minimum generating-asset fraction, valued at the STRESSED cap
/// rate, required to restore the interest-coverage covenant.
///
/// Curability turns on the financing-versus-yield regime, NOT on a fixed
/// rate/cap threshold:
///   selling_helps  ⇔  EBITDA_stressed > cap_rate × debt_at_shock.
/// Economically: disposition deleverages, retiring debt that costs `debt_rate`
/// while shedding income that yields `cap_rate`. It improves coverage only when
/// the retained-asset yield no longer covers the covenant-scaled debt service —
/// i.e. in HIGH financing-cost regimes. When debt is cheap relative to asset
/// yield, the assets are "too good to sell": disposition lowers coverage and the
/// breach is incurable by disposition (the binding response is a Management
/// Response Scenario instead).
fn forecast_covenant_cure(p: &ForecastParams, shock_year: u32) -> (Vec<Pclp1Year>, ForecastMeta) {
    // 1. Base build (unstressed, ManagedDownside) — the pre-shock portfolio.
    let base = ForecastParams::default();
    let (base_years, _) = forecast_managed_or_static(&base);

    let sy = shock_year.clamp(1, 10) as usize;

    // 2. Pre-shock state at the shock year.
    let gen_at_shock = base_years[sy].generating; // only completed phases generate
    let debt_at_shock = base_years[sy].closing_debt;
    let fixed_exp = fixed_operating_expenses();

    // 3. Stressed coverage on the existing portfolio (instantaneous shock on
    //    stabilised assets — no lease-up ramp; cash-interest credit ignored,
    //    conservative).
    let noi_gen = gen_at_shock * p.dev_yield * p.occupancy_pct;
    let ebitda_stressed = noi_gen - fixed_exp;
    let interest_stressed = debt_at_shock * p.debt_rate;
    let dscr_stressed = if interest_stressed > 0.0 {
        ebitda_stressed / interest_stressed
    } else {
        f64::INFINITY
    };
    let mv_gen = if p.cap_rate > 0.0 { noi_gen / p.cap_rate } else { 0.0 };
    let k = p.dscr_floor;

    // 4. Solve the minimum corrective disposition.
    let selling_helps = ebitda_stressed > p.cap_rate * debt_at_shock;
    let (fraction_sold, incurable) = if dscr_stressed >= k {
        // Covenant already met under the shock — no disposition required.
        (0.0_f64, false)
    } else if !selling_helps {
        // Cheap-debt / income-collapse regime: disposition cannot restore the
        // covenant (selling sheds more coverage than debt-service relief).
        (0.0_f64, true)
    } else {
        let numerator = noi_gen - fixed_exp - k * p.debt_rate * debt_at_shock;
        let denominator = noi_gen - k * p.debt_rate * mv_gen;
        if denominator == 0.0 {
            (1.0_f64, true)
        } else {
            let s = numerator / denominator;
            if !s.is_finite() || s >= 1.0 {
                (1.0_f64, true) // full-portfolio sale still insufficient
            } else {
                (s.max(0.0), false)
            }
        }
    };

    // Applied disposition: zero when incurable (the breach stands and is shown).
    let applied_frac = if incurable { 0.0 } else { fraction_sold };
    let sale_value_total = applied_frac * mv_gen;
    let debt_retired = sale_value_total;
    let gen_remaining = (1.0 - applied_frac) * gen_at_shock;
    let debt_remaining = (debt_at_shock - debt_retired).max(0.0);

    // Post-cure coverage and NAV (stressed).
    let noi_remaining = gen_remaining * p.dev_yield * p.occupancy_pct;
    let ebitda_remaining = noi_remaining - fixed_exp;
    let interest_remaining = debt_remaining * p.debt_rate;
    let dscr_post_cure = if interest_remaining > 0.0 {
        ebitda_remaining / interest_remaining
    } else if ebitda_remaining > 0.0 {
        f64::INFINITY
    } else {
        0.0
    };
    let dscr_post_cure = if dscr_post_cure.is_finite() {
        dscr_post_cure
    } else {
        99.99 // debt fully retired with positive income — cap for display/JSON
    };

    // NAV is invariant to sale fraction at market value: selling s of MV both
    // removes s·MV of assets and retires s·MV of debt. Report it honestly.
    let nav_post = noi_remaining / p.cap_rate - debt_remaining;
    let nav_per_unit_post = nav_post / PCLP1_DILUTED_UNITS;

    // Income still available to holders after debt service on the retained portfolio.
    let dist_per_unit_post_cure = (ebitda_remaining - interest_remaining).max(0.0) / PCLP1_DILUTED_UNITS;

    let disposition = Pclp1DispositionEvent {
        shock_year,
        fraction_sold: if incurable { 1.0 } else { fraction_sold },
        incurable,
        trigger: DispositionTrigger::Dscr,
        sale_value_total,
        debt_retired,
        dscr_post_cure,
        nav_per_unit_post_cure: nav_per_unit_post,
        buildings_remaining_pct: (1.0 - applied_frac) * 100.0,
        dist_per_unit_post_cure,
        nav_curable_by_disposition: false,
    };

    // 5. Year series: base build up to the shock; stressed reduced portfolio after.
    let mut years: Vec<Pclp1Year> = Vec::with_capacity(11);
    for (idx, by) in base_years.iter().enumerate() {
        if idx < sy {
            years.push(by.clone());
            continue;
        }
        // Post-shock projection: portfolio frozen at the (reduced) generating base,
        // no further phase draws, stressed inputs in force.
        let y = idx as u32;
        let gen = gen_remaining;
        let net_proceeds_from_ops = gen * p.dev_yield * p.occupancy_pct;
        let advisory = advisory_fee_annual();
        let total_expenses = advisory + PCLP1_ADMIN_COMPLIANCE_ANNUAL + PCLP1_BOARD_ANNUAL;
        let ebitda = net_proceeds_from_ops - total_expenses;
        let net_interest = debt_remaining * p.debt_rate;
        let ffo = ebitda - net_interest;
        let debt_repayment = if y >= 8 { ffo * PCLP1_DEBT_BUYBACK_PCT_FFO } else { 0.0 };
        let closing_debt = (debt_remaining - debt_repayment).max(0.0);
        let distributions = (ffo * payout_ratio(y)) - debt_repayment;
        let income_continuity = net_proceeds_from_ops; // stabilised (no ramp post-shock)
        let asset_value = income_continuity / p.cap_rate;
        let nav = asset_value - closing_debt;
        let dpu = distributions / PCLP1_DILUTED_UNITS;
        let market_value_per_unit = if p.market_yield > 0.0 { dpu / p.market_yield } else { 0.0 };
        let interest_coverage = if net_interest > 1.0 { ebitda / net_interest } else { 0.0 };
        let total_sqft_generating = if PCLP1_TOTAL_DEV_COST > 0.0 {
            PCLP1_TOTAL_PORTFOLIO_SQFT * (gen / PCLP1_TOTAL_DEV_COST)
        } else {
            0.0
        };

        years.push(Pclp1Year {
            year: y,
            phase_draws: 0.0,
            total_assets: gen,
            wip: 0.0,
            generating: gen,
            net_proceeds_from_ops,
            income_continuity,
            issue_costs: 0.0,
            financing_costs: 0.0,
            advisory_fee: advisory,
            admin_compliance: PCLP1_ADMIN_COMPLIANCE_ANNUAL,
            board: PCLP1_BOARD_ANNUAL,
            total_expenses,
            ebitda,
            opening_debt: debt_remaining,
            gross_debt_draw: 0.0,
            net_interest,
            ffo,
            debt_repayment,
            closing_debt,
            opening_cash: 0.0,
            new_equity: 0.0,
            distributions,
            ending_cash: 0.0,
            asset_value,
            asset_value_per_unit: asset_value / PCLP1_DILUTED_UNITS,
            nav,
            nav_per_unit: nav / PCLP1_DILUTED_UNITS,
            dpu,
            market_value_per_unit,
            dist_yield_on_cost: dpu / PCLP1_UNIT_PRICE,
            dist_yield_at_market: if market_value_per_unit > 0.0 {
                dpu / market_value_per_unit
            } else {
                0.0
            },
            advisory_fee_to_wcp: advisory,
            interest_coverage,
            debt_to_dev_cost: if PCLP1_TOTAL_DEV_COST > 0.0 {
                closing_debt / PCLP1_TOTAL_DEV_COST
            } else {
                0.0
            },
            debt_to_asset_value: if asset_value > 1.0 {
                closing_debt / asset_value
            } else {
                0.0
            },
            total_sqft_generating,
            phase2_scale: 1.0,
            phase3_scale: 1.0,
        });
    }

    let buildings_built_pct = {
        let full = full_generating_base();
        if full > 0.0 {
            (gen_at_shock / full) * 100.0
        } else {
            0.0
        }
    };

    let meta = ForecastMeta {
        phase2_scale: 1.0,
        phase3_scale: 1.0,
        dscr_constraint_triggered_year: Some(shock_year),
        buildings_built_pct,
        mode: p.mode,
        disposition: Some(disposition),
    };

    (years, meta)
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
        // (V3: phase_draw superseded by phase_draw_scaled; base scale = 1.0)
        assert!((phase_draw_scaled(7, 1.0, 1.0) - PCLP1_PHASE_3_Y6_DRAW).abs() < 1.0);
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

    // ─── V3 three-mode engine tests ──────────────────────────────────────

    fn static_base() -> ForecastParams {
        ForecastParams {
            mode: ModelMode::SingleInputStress,
            ..Default::default()
        }
    }

    #[test]
    fn base_case_scales_are_unity() {
        // Backward compatibility: at default params neither issuance constraint
        // binds, so both scales clamp to 1.0 and no disposition is produced.
        let (_y, meta) = forecast_full(&ForecastParams::default());
        assert!((meta.phase2_scale - 1.0).abs() < 1e-9, "s2={}", meta.phase2_scale);
        assert!((meta.phase3_scale - 1.0).abs() < 1e-9, "s3={}", meta.phase3_scale);
        assert_eq!(meta.dscr_constraint_triggered_year, None);
        assert!(meta.disposition.is_none());
    }

    #[test]
    fn default_and_managed_base_match() {
        // forecast() must equal ManagedDownside at base params, year by year.
        let a = forecast();
        let b = forecast_with_params(&ForecastParams::default());
        assert_eq!(a.len(), b.len());
        for (ya, yb) in a.iter().zip(b.iter()) {
            assert!((ya.nav_per_unit - yb.nav_per_unit).abs() < 1e-6);
            assert!((ya.closing_debt - yb.closing_debt).abs() < 1e-6);
            assert!((ya.interest_coverage - yb.interest_coverage).abs() < 1e-9);
        }
    }

    #[test]
    fn static_base_equals_managed_base() {
        // No constraint binds at base ⇒ SingleInputStress ≡ ManagedDownside.
        let m = forecast_with_params(&ForecastParams::default());
        let s = forecast_with_params(&static_base());
        for (ym, ys) in m.iter().zip(s.iter()) {
            assert!(
                (ym.nav_per_unit - ys.nav_per_unit).abs() < 1e-6,
                "Y{} nav mismatch",
                ym.year
            );
        }
    }

    #[test]
    fn high_rate_managed_reduces_phase3() {
        // At a 7% coupon the LP cannot fully fund Phase 3 at the covenant floor.
        let p = ForecastParams {
            debt_rate: 0.07,
            mode: ModelMode::ManagedDownside,
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        assert!(meta.phase2_scale < 1.0, "s2={}", meta.phase2_scale);
        assert!(meta.phase3_scale < 1.0, "s3={}", meta.phase3_scale);
        assert!(meta.dscr_constraint_triggered_year.is_some());
    }

    #[test]
    fn managed_meta_triggered_year_set_under_mild_stress() {
        let p = ForecastParams {
            debt_rate: 0.065,
            mode: ModelMode::ManagedDownside,
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        assert_eq!(meta.dscr_constraint_triggered_year, Some(5));
    }

    #[test]
    fn extreme_rate_collapses_both_phase_scales() {
        // Phase 2 starved ⇒ Phase 3 (funded after Phase 2) near zero.
        let p = ForecastParams {
            debt_rate: 0.20,
            mode: ModelMode::ManagedDownside,
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        assert!(meta.phase2_scale < 0.40, "s2={}", meta.phase2_scale);
        assert!(meta.phase3_scale < 0.10, "s3={}", meta.phase3_scale);
    }

    #[test]
    fn high_rate_static_breaches_coverage() {
        // SingleInputStress: no management response ⇒ the breach surfaces.
        let p = ForecastParams {
            debt_rate: 0.09,
            mode: ModelMode::SingleInputStress,
            ..Default::default()
        };
        let years = forecast_with_params(&p);
        let breached = years
            .iter()
            .any(|y| y.year >= 5 && y.interest_coverage > 0.0 && y.interest_coverage < 1.20);
        assert!(breached, "expected a coverage breach at 9% static");
    }

    #[test]
    fn occupancy_scales_operating_noi() {
        let p = ForecastParams {
            occupancy_pct: 0.90,
            ..static_base()
        };
        let years = forecast_with_params(&p);
        let base = forecast_with_params(&static_base());
        // Y8: all phases stabilised, no lease-up ⇒ NOI scales linearly with occupancy.
        let r = years[8].net_proceeds_from_ops / base[8].net_proceeds_from_ops;
        assert!((r - 0.90).abs() < 1e-6, "ratio={}", r);
    }

    #[test]
    fn leaseup_24mo_reduces_first_generating_year_noi() {
        // 24-month lease-up ⇒ Phase 1's first generating year (Y4) averages 25%
        // in-place occupancy (linear-ramp midpoint) vs 100% at the 12-month base.
        let p = ForecastParams {
            lease_up_months: 24.0,
            ..static_base()
        };
        let years = forecast_with_params(&p);
        let base = forecast_with_params(&static_base());
        let r = years[4].net_proceeds_from_ops / base[4].net_proceeds_from_ops;
        assert!((r - 0.25).abs() < 1e-6, "Y4 lease-up ratio={}", r);
        // Stabilised valuation income is unaffected by the timing discount.
        assert!((years[4].income_continuity - base[4].income_continuity).abs() < 1.0);
    }

    #[test]
    fn managed_longer_leaseup_reduces_issuance() {
        // Jennifer's ask #2: a slower lease-up ⇒ debt issued more slowly ⇒ fewer
        // buildings. A 24-month lease-up must constrain Phase 2/3 below the 12-mo base.
        let p = ForecastParams {
            lease_up_months: 24.0,
            mode: ModelMode::ManagedDownside,
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        assert!(meta.phase2_scale < 1.0, "s2={}", meta.phase2_scale);
        assert!(meta.phase3_scale < 1.0, "s3={}", meta.phase3_scale);
        assert!(meta.buildings_built_pct < 100.0);
        // 12-month base remains fully built (backward compatibility of the ramp).
        let (_yb, mb) = forecast_full(&ForecastParams::default());
        assert!((mb.phase2_scale - 1.0).abs() < 1e-9);
        assert!((mb.phase3_scale - 1.0).abs() < 1e-9);
    }

    #[test]
    fn covenant_cure_no_sale_when_coverage_holds() {
        // A single occupancy stress does not breach ⇒ no corrective disposition.
        let p = ForecastParams {
            occupancy_pct: 0.90,
            mode: ModelMode::CovenantCure { shock_year: 8 },
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        let d = meta.disposition.expect("disposition event present");
        assert!(!d.incurable);
        assert!(
            d.fraction_sold.abs() < 1e-9,
            "expected no sale, got {}",
            d.fraction_sold
        );
    }

    #[test]
    fn covenant_cure_combined_stress_produces_disposition() {
        // Combined post-construction shock at Y8 (high coupon + cap expansion +
        // occupancy decline) breaches coverage; high financing cost makes the
        // breach curable by minimum disposition.
        let p = ForecastParams {
            occupancy_pct: 0.80,
            debt_rate: 0.085,
            cap_rate: 0.095,
            mode: ModelMode::CovenantCure { shock_year: 8 },
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        let d = meta.disposition.expect("disposition");
        assert!(!d.incurable, "expected curable, got incurable");
        assert!(
            d.fraction_sold > 0.0 && d.fraction_sold < 1.0,
            "fraction_sold={}",
            d.fraction_sold
        );
        assert!(
            (d.dscr_post_cure - 1.20).abs() < 0.02,
            "post-cure coverage={} (target 1.20×)",
            d.dscr_post_cure
        );
        assert!(d.sale_value_total > 0.0);
        assert!((d.debt_retired - d.sale_value_total).abs() < 1.0);
    }

    #[test]
    fn covenant_cure_incurable_cheap_debt_income_collapse() {
        // Cheap debt (5%) + income collapse (50% occ, 7% yield): the retained
        // assets out-yield the debt, so disposition lowers coverage — incurable.
        let p = ForecastParams {
            occupancy_pct: 0.50,
            dev_yield: 0.07,
            mode: ModelMode::CovenantCure { shock_year: 8 },
            ..Default::default()
        };
        let (_y, meta) = forecast_full(&p);
        let d = meta.disposition.expect("disposition");
        assert!(
            d.incurable,
            "cheap-debt income collapse must be incurable by disposition"
        );
    }

    #[test]
    fn nav_curable_by_disposition_always_false() {
        // NAV = MV − Debt is invariant to the sale fraction at market value.
        for sy in [6u32, 7, 8] {
            for &(occ, rate, cap) in &[
                (0.82, 0.085, 0.095),
                (0.50, 0.05, 0.0625),
                (0.90, 0.06, 0.070),
            ] {
                let p = ForecastParams {
                    occupancy_pct: occ,
                    debt_rate: rate,
                    cap_rate: cap,
                    mode: ModelMode::CovenantCure { shock_year: sy },
                    ..Default::default()
                };
                let (_y, meta) = forecast_full(&p);
                let d = meta.disposition.expect("disposition");
                assert!(!d.nav_curable_by_disposition);
            }
        }
    }

    #[test]
    fn covenant_cure_outputs_are_finite() {
        // JSON must never carry NaN/Infinity.
        let p = ForecastParams {
            occupancy_pct: 0.50,
            dev_yield: 0.07,
            mode: ModelMode::CovenantCure { shock_year: 8 },
            ..Default::default()
        };
        let (years, meta) = forecast_full(&p);
        let d = meta.disposition.unwrap();
        assert!(d.dscr_post_cure.is_finite());
        assert!(d.fraction_sold.is_finite());
        assert!(d.sale_value_total.is_finite());
        assert!(d.nav_per_unit_post_cure.is_finite());
        for y in &years {
            assert!(y.nav_per_unit.is_finite() && y.interest_coverage.is_finite());
        }
    }
}




