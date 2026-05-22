use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

use super::{get_f64, get_str};

/// Input assumptions read from the left-hand side of the PCLP 1_250M sheet.
#[derive(Debug, Clone)]
pub struct Pclp1Assumptions {
    pub dev_yield: f64,
    pub cap_rate: f64,
    pub total_equity: f64,
    pub cost_per_unit: f64,
    pub advisory_fee_pct: f64,
    pub benetti_dilution: f64,
    pub board_expense: f64,
    pub admin_costs: f64,
    pub debenture_interest_rate: f64,
    pub interest_on_cash: f64,
    pub debenture_buyback_pct: f64,
    pub min_cash_balance: f64,
    pub working_capital_reserve: f64,
    pub diluted_units: f64,
    /// Year headers Y1..Y10 (index 0 = Y1)
    pub year_labels: [String; 10],
}

/// Per-year income statement, cash flow, debenture, asset, and valuation rows.
/// Index 0 = Y1, index 9 = Y10.
#[derive(Debug, Clone)]
pub struct Pclp1Year {
    pub year: u32,
    // Income
    pub noi: f64,
    pub income_continuity: f64,
    pub issue_costs: f64,
    pub financing_costs: f64,
    pub advisory_fees: f64,
    pub admin_compliance: f64,
    pub board_of_directors: f64,
    pub total_expenses: f64,
    pub ebitda: f64,
    pub interest_net: f64,
    pub funding_from_ops: f64,
    pub interest_coverage: Option<f64>,
    pub debt_service_ratio: Option<f64>,
    // Cash flow
    pub opening_cash: f64,
    pub new_equity: f64,
    pub new_debt_gross: f64,
    pub capex: f64,
    pub debt_repayment: f64,
    pub distributions: f64,
    pub ending_cash: f64,
    // Debenture
    pub opening_debt: f64,
    pub debt_additions: f64,
    pub debt_payments: f64,
    pub ending_debt: f64,
    // Assets
    pub opening_assets: f64,
    pub total_capital_assets: f64,
    pub assets_generating_rent: f64,
    pub buildings_under_construction: f64,
    pub debt_to_dev_cost: f64,
    // Valuation
    pub asset_value_total: f64,
    pub asset_value_per_unit: f64,
    pub nav_total: f64,
    pub nav_per_unit: f64,
    pub distribution_yield: f64,
    pub total_expense_ratio: f64,
    pub distributions_to_lps: f64,
    pub dist_per_unit: f64,
    pub dist_yield_on_cost: f64,
    // Financial Forecast per-unit (right-hand summary section)
    pub ff_revenue_pu: f64,
    pub ff_dist_pu: f64,
    pub ff_dist_yield_on_cost: f64,
    pub ff_asset_value_pu: f64,
    pub ff_total_debt_pu: f64,
    pub ff_nav_pu: f64,
    pub ff_market_value_pu: f64,
    pub ff_coverage: Option<f64>,
    pub ff_debt_to_dev_cost: f64,
    pub ff_debt_to_av: f64,
    pub ff_ter: f64,
    pub ff_sqft: f64,
}

/// Full parsed PCLP 1 model.
#[derive(Debug)]
pub struct Pclp1Data {
    pub title: String,
    pub entity: String,
    pub date: String,
    pub assumptions: Pclp1Assumptions,
    pub years: Vec<Pclp1Year>, // 10 elements
    pub market_yield: f64,     // Financial Forecast section
    pub compounded_return_y8: f64,
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Pclp1Data, Box<dyn std::error::Error>> {
    let mut wb: Xlsx<_> = open_workbook(path)?;
    let range = wb.worksheet_range("PCLP 1_250M")?;

    // Rows are 0-indexed in calamine; Excel row N → index N-1.
    let r = |row: u32| row - 1;

    // NOTE: the sheet's used range begins at absolute column B (index 1).
    // get_value() takes absolute (row, col) coordinates.
    // Labels are in relative col B (absolute 2); assumption values in relative col C (absolute 3).
    // Year columns Y1–Y10 are relative D–M (absolute 4–13).
    // Financial Forecast columns Y1–Y10 are relative AE–AN (absolute 30–39).

    let title = get_str(&range, r(1), 2); // absolute C1
    let entity = get_str(&range, r(2), 2); // absolute C2
    let date = get_str(&range, r(3), 2); // absolute C3

    // Assumptions: value in absolute col D (index 3) = relative col C
    let assumptions = Pclp1Assumptions {
        dev_yield: get_f64(&range, r(10), 3),
        cap_rate: get_f64(&range, r(12), 3),
        total_equity: get_f64(&range, r(15), 3),
        cost_per_unit: get_f64(&range, r(16), 3),
        advisory_fee_pct: get_f64(&range, r(19), 3),
        benetti_dilution: get_f64(&range, r(21), 3),
        board_expense: get_f64(&range, r(23), 3),
        admin_costs: get_f64(&range, r(24), 3),
        debenture_interest_rate: get_f64(&range, r(29), 3),
        interest_on_cash: get_f64(&range, r(30), 3),
        debenture_buyback_pct: get_f64(&range, r(31), 3),
        min_cash_balance: get_f64(&range, r(33), 3),
        working_capital_reserve: get_f64(&range, r(34), 3),
        diluted_units: get_f64(&range, r(45), 3),
        // Year labels in Financial Forecast section: row 12, absolute AE–AN (30–39)
        year_labels: std::array::from_fn(|y| get_str(&range, r(12), (30 + y) as u32)),
    };

    // Main year data: Y1–Y10 in absolute cols E–N (4–13)
    let yr_col = |y: usize| (4 + y) as u32;
    // Financial Forecast section: Y1–Y10 in absolute cols AE–AN (30–39)
    let ff_col = |y: usize| (30 + y) as u32;
    // market_yield at absolute col AB (28) = relative AA, row 23
    let market_yield = get_f64(&range, r(23), 28);
    // compounded_return_y8 at ff_col(7) = col 37 (AK), row 27
    let compounded_return_y8 = get_f64(&range, r(27), ff_col(7));

    let parse_cov = |y: usize, row: u32| -> Option<f64> {
        let v = get_f64(&range, r(row), yr_col(y));
        if v == 0.0 {
            None
        } else {
            Some(v)
        }
    };
    let parse_ff_cov = |y: usize, row: u32| -> Option<f64> {
        let v = get_f64(&range, r(row), ff_col(y));
        if v == 0.0 {
            None
        } else {
            Some(v)
        }
    };

    let mut years = Vec::with_capacity(10);
    for y in 0..10 {
        let yc = yr_col(y);
        years.push(Pclp1Year {
            year: y as u32 + 1,
            noi: get_f64(&range, r(57), yc),
            income_continuity: get_f64(&range, r(58), yc),
            issue_costs: get_f64(&range, r(61), yc),
            financing_costs: get_f64(&range, r(62), yc),
            advisory_fees: get_f64(&range, r(63), yc),
            admin_compliance: get_f64(&range, r(64), yc),
            board_of_directors: get_f64(&range, r(65), yc),
            total_expenses: get_f64(&range, r(66), yc),
            ebitda: get_f64(&range, r(68), yc),
            interest_net: get_f64(&range, r(69), yc),
            funding_from_ops: get_f64(&range, r(72), yc),
            interest_coverage: parse_cov(y, 74),
            debt_service_ratio: parse_cov(y, 75),
            opening_cash: get_f64(&range, r(79), yc),
            new_equity: get_f64(&range, r(80), yc),
            new_debt_gross: get_f64(&range, r(81), yc),
            capex: get_f64(&range, r(82), yc),
            debt_repayment: get_f64(&range, r(83), yc),
            distributions: get_f64(&range, r(84), yc),
            ending_cash: get_f64(&range, r(86), yc),
            opening_debt: get_f64(&range, r(89), yc),
            debt_additions: get_f64(&range, r(90), yc),
            debt_payments: get_f64(&range, r(91), yc),
            ending_debt: get_f64(&range, r(92), yc),
            opening_assets: get_f64(&range, r(96), yc),
            total_capital_assets: get_f64(&range, r(100), yc),
            assets_generating_rent: get_f64(&range, r(102), yc),
            buildings_under_construction: get_f64(&range, r(104), yc),
            debt_to_dev_cost: get_f64(&range, r(106), yc),
            asset_value_total: get_f64(&range, r(109), yc),
            asset_value_per_unit: get_f64(&range, r(110), yc),
            nav_total: get_f64(&range, r(114), yc),
            nav_per_unit: get_f64(&range, r(115), yc),
            distribution_yield: get_f64(&range, r(116), yc),
            total_expense_ratio: get_f64(&range, r(117), yc),
            distributions_to_lps: get_f64(&range, r(120), yc),
            dist_per_unit: get_f64(&range, r(122), yc),
            dist_yield_on_cost: get_f64(&range, r(123), yc),
            // Financial Forecast per-unit
            ff_revenue_pu: get_f64(&range, r(13), ff_col(y)),
            ff_dist_pu: get_f64(&range, r(14), ff_col(y)),
            ff_dist_yield_on_cost: get_f64(&range, r(16), ff_col(y)),
            ff_asset_value_pu: get_f64(&range, r(18), ff_col(y)),
            ff_total_debt_pu: get_f64(&range, r(19), ff_col(y)),
            ff_nav_pu: get_f64(&range, r(21), ff_col(y)),
            ff_market_value_pu: get_f64(&range, r(23), ff_col(y)),
            ff_coverage: parse_ff_cov(y, 30),
            ff_debt_to_dev_cost: get_f64(&range, r(31), ff_col(y)),
            ff_debt_to_av: get_f64(&range, r(32), ff_col(y)),
            ff_ter: get_f64(&range, r(34), ff_col(y)),
            ff_sqft: get_f64(&range, r(35), ff_col(y)),
        });
    }

    Ok(Pclp1Data {
        title,
        entity,
        date,
        assumptions,
        years,
        market_yield,
        compounded_return_y8,
    })
}
