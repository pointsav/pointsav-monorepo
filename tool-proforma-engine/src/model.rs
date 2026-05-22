use serde::{Deserialize, Serialize};

/// Input assumptions for the proforma engine.
/// All rates are fractions (0.0625 = 6.25%); lease_up_months is an integer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assumptions {
    /// Capitalisation rate applied to stabilised NOI to derive asset value. E.g. 0.0625.
    pub cap_rate: f64,
    /// Portfolio occupancy as a fraction of fully-stabilised NOI. E.g. 0.95.
    pub occupancy: f64,
    /// Lease-up period in months for stabilisation years (Y4, Y6, Y8). Base = 12.
    pub lease_up_months: u32,
    /// Development yield used to calibrate the NOI projection. E.g. 0.105.
    pub dev_yield: f64,
    /// Blended debenture interest rate as a fraction. E.g. 0.05.
    pub debt_rate: f64,
    /// Secondary-market yield applied to distributions to derive market value (Y8+). E.g. 0.08.
    pub market_yield: f64,
}

/// Per-year output from the 10-year proforma model.
#[derive(Debug, Serialize)]
pub struct YearOutput {
    /// Calendar year number (1 = Y1, 10 = Y10).
    pub year: u32,
    /// Fund-level net operating income.
    pub noi: f64,
    /// EBITDA (NOI less operating expenses).
    pub ebitda: f64,
    /// Net interest expense on debentures (positive = expense).
    pub interest: f64,
    /// Distributable income per LP unit (90% of after-interest cash after debt repayment).
    pub dist_per_unit: f64,
    /// Total asset value per LP unit (cash + capitalised NOI + WIP @ cost).
    pub asset_per_unit: f64,
    /// Net asset value per LP unit ((asset value − debt) ÷ diluted units).
    pub nav_per_unit: f64,
    /// Secondary-market value per LP unit (Y1–Y7 hardcoded proforma; Y8+ = dist ÷ market_yield).
    pub mv_per_unit: f64,
    /// Interest coverage ratio (EBITDA ÷ interest). None when interest ≤ 0.
    pub coverage: Option<f64>,
    /// Outstanding debt as a fraction of total asset value.
    pub debt_to_av: f64,
}

/// Full 10-year proforma output.
#[derive(Debug, Serialize)]
pub struct ProformaOutput {
    /// The assumptions used to produce this output.
    pub assumptions: Assumptions,
    /// Annual outputs for Y1–Y10 (10 elements, index 0 = Y1).
    pub years: Vec<YearOutput>,
    /// Sum of dist_per_unit across all 10 years.
    pub total_dist_per_unit: f64,
}
