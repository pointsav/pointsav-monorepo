// Bencal Special Purpose 2 (GP + LP) (D4) — dual-asset entity.
// BRIEF v0.15.6 §5d + Flag 15 path b.
//
// Holds 250K PCLP 1 LP units + 600K WCP founding bonus shares.
// Consumes both PCLP 1 V2 and WCP V1 forecasts.

use serde::Serialize;
use crate::spv::pclp1_proforma::Pclp1Year;
use crate::spv::wcp_proforma::WcpYear;

// ─── Constants ──────────────────────────────────────────────────────────────

pub const SPV2_INVESTOR_UNITS: f64           = 250_591.0;
pub const SPV2_MANAGER_UNITS: f64            = 27_843.0;
pub const SPV2_DILUTED_UNITS: f64            = 278_434.0;
pub const SPV2_UNIT_PRICE: f64               = 100.00;
pub const SPV2_TOTAL_INVESTOR_CAPITAL: f64   = 25_059_100.0;

pub const SPV2_PCLP1_UNITS_HELD: f64         = 250_000.0;          // BRIEF §5d
pub const SPV2_PCLP1_COST_PER_UNIT: f64      = 100.00;             // $25M cost basis

pub const SPV2_WCP_FOUNDING_BONUS: f64       = 600_000.0;          // Flag 15 path b
pub const SPV2_WCP_BONUS_NOMINAL: f64        = 0.000_332_23;       // $199.34 nominal
pub const SPV2_WCP_LEVEL3_PROXY: f64         = 4.55;
pub const SPV2_Y10_WCP_CRS_SHARES: f64       = 393_824.0;          // Sold at Y10
pub const SPV2_WCP_RESIDUAL_SHARES: f64      = 206_176.0;          // = 600K - 393,824

pub const SPV2_COMMISSION_REBATE_Y0_GROSS: f64 = 92_207.0;
pub const SPV2_OPEX_ANNUAL: f64              = 19_402.0;
pub const SPV2_SETUP_COSTS_Y0: f64           = 9_105.0;
pub const SPV2_TAX_RATE_CURRENT: f64         = 0.27;

// ─── Output struct ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct BencalSpv2Year {
    pub year: u32,
    pub commission_rebate: f64,
    pub pclp1_distributions: f64,        // PCLP 1 DPU × 250K units
    pub wcp_crs_realised_gain: f64,      // Y10 sale of 393,824 WCP shares
    pub pclp1_fv_change: f64,            // FVTPL on PCLP 1 LP units
    pub wcp_fv_change: f64,              // FVTPL on WCP shares
    pub total_investment_income: f64,
    pub setup_costs: f64,
    pub opex: f64,
    pub income_before_tax: f64,
    pub tax: f64,
    pub net_income: f64,
    pub opex_reserve_closing: f64,
    pub pclp1_nav_total: f64,            // 250K × pclp1.nav_per_unit
    pub wcp_per_share_value: f64,
    pub wcp_shares_held: f64,
    pub wcp_holding_fv: f64,
    pub cash_to_investors: f64,          // Each year
    pub cumulative_cash_to_investors: f64,
    pub cumulative_cash_per_unit: f64,
    pub y_end_position_per_unit: f64,    // (PCLP1 + WCP) NAV / investor units
}

pub fn forecast(pclp1: &[Pclp1Year], wcp: &[WcpYear]) -> Vec<BencalSpv2Year> {
    let mut years: Vec<BencalSpv2Year> = Vec::with_capacity(11);
    let net_y0_rebate = SPV2_COMMISSION_REBATE_Y0_GROSS * (1.0 - SPV2_TAX_RATE_CURRENT);
    let initial_reserve = net_y0_rebate - SPV2_SETUP_COSTS_Y0;
    let mut reserve = 0.0;

    let pclp1_cost_basis = SPV2_PCLP1_UNITS_HELD * SPV2_PCLP1_COST_PER_UNIT;
    let wcp_cost_basis = SPV2_WCP_FOUNDING_BONUS * SPV2_WCP_BONUS_NOMINAL;

    let mut prev_pclp1_fv = pclp1_cost_basis;
    let mut prev_wcp_fv = wcp_cost_basis;
    let mut prev_wcp_shares = SPV2_WCP_FOUNDING_BONUS;
    let mut cumulative_cash: f64 = 0.0;

    for y in 0u32..=10 {
        let commission = if y == 0 { SPV2_COMMISSION_REBATE_Y0_GROSS } else { 0.0 };
        let setup = if y == 0 { SPV2_SETUP_COSTS_Y0 } else { 0.0 };
        let opex = if y >= 1 { SPV2_OPEX_ANNUAL } else { 0.0 };

        // PCLP 1 distributions = engine DPU × 250K units (no scale; SPV2 holds 250K of 2,777,777)
        let pclp1_dpu = if (y as usize) < pclp1.len() { pclp1[y as usize].dpu } else { 0.0 };
        let pclp1_dist = pclp1_dpu * SPV2_PCLP1_UNITS_HELD;

        // PCLP 1 NAV
        let pclp1_nav_per_unit = if (y as usize) < pclp1.len() { pclp1[y as usize].nav_per_unit } else { 0.0 };
        let pclp1_nav_total = pclp1_nav_per_unit * SPV2_PCLP1_UNITS_HELD;
        let pclp1_fv_change = pclp1_nav_total - prev_pclp1_fv;

        // WCP per-share value: pre-listing proxy $4.55; post-listing book value per share
        let wcp_value_per_share = if y < 4 {
            SPV2_WCP_LEVEL3_PROXY
        } else if (y as usize) < wcp.len() {
            wcp[y as usize].book_value_per_share.max(SPV2_WCP_LEVEL3_PROXY)
        } else { SPV2_WCP_LEVEL3_PROXY };

        // WCP shares held: 600K through Y9; 206K after Y10 sale
        let wcp_shares_held = if y < 10 { SPV2_WCP_FOUNDING_BONUS } else { SPV2_WCP_RESIDUAL_SHARES };

        // Y10 CRS event
        let wcp_crs_cash = if y == 10 {
            SPV2_Y10_WCP_CRS_SHARES * wcp_value_per_share
        } else { 0.0 };

        let wcp_holding_fv = wcp_shares_held * wcp_value_per_share;
        let wcp_fv_change = (wcp_holding_fv + wcp_crs_cash) - prev_wcp_fv;

        let total_income = commission + pclp1_dist + wcp_crs_cash + pclp1_fv_change + wcp_fv_change;
        let income_before_tax = total_income - setup - opex;
        let tax = if income_before_tax > 0.0 { income_before_tax * SPV2_TAX_RATE_CURRENT } else { 0.0 };
        let net_income = income_before_tax - tax;

        if y == 0 {
            reserve = initial_reserve;
        } else if y <= 3 && reserve > 0.0 {
            reserve = (reserve - opex).max(0.0);
        }

        // Cash to investors: pass-through of PCLP 1 distributions + Y10 WCP CRS
        let cash_to_investors = pclp1_dist + wcp_crs_cash;
        cumulative_cash += cash_to_investors;
        let cum_per_unit = cumulative_cash / SPV2_INVESTOR_UNITS;

        let position_per_unit = (pclp1_nav_total + wcp_holding_fv) / SPV2_INVESTOR_UNITS;

        years.push(BencalSpv2Year {
            year: y,
            commission_rebate: commission,
            pclp1_distributions: pclp1_dist,
            wcp_crs_realised_gain: wcp_crs_cash,
            pclp1_fv_change,
            wcp_fv_change,
            total_investment_income: total_income,
            setup_costs: setup,
            opex,
            income_before_tax,
            tax,
            net_income,
            opex_reserve_closing: reserve,
            pclp1_nav_total,
            wcp_per_share_value: wcp_value_per_share,
            wcp_shares_held,
            wcp_holding_fv,
            cash_to_investors,
            cumulative_cash_to_investors: cumulative_cash,
            cumulative_cash_per_unit: cum_per_unit,
            y_end_position_per_unit: position_per_unit,
        });

        prev_pclp1_fv = pclp1_nav_total;
        prev_wcp_fv = wcp_holding_fv;
        prev_wcp_shares = wcp_shares_held;
        let _ = prev_wcp_shares;
    }
    years
}

pub fn forecast_json(pclp1: &[Pclp1Year], wcp: &[WcpYear]) -> serde_json::Value {
    let years = forecast(pclp1, wcp);
    let y10 = &years[10];
    let total_return = y10.cumulative_cash_to_investors + y10.pclp1_nav_total + y10.wcp_holding_fv;
    let moic = total_return / SPV2_TOTAL_INVESTOR_CAPITAL;
    serde_json::json!({
        "entity": "Bencal Special Purpose 2 (GP + LP)",
        "source": "tool-proforma-engine src/spv/bencal_spv2_proforma module",
        "brief_section": "v0.15.6 §5d + Flag 15 path (b) + cap table v0.15.9 §5c",
        "version": "V1",
        "generated_at": "2026-06-04",
        "consumes": "PCLP 1 V2 + WCP V1 forecasts (dual-asset)",
        "inputs": {
            "investor_units": SPV2_INVESTOR_UNITS,
            "manager_units": SPV2_MANAGER_UNITS,
            "diluted_units": SPV2_DILUTED_UNITS,
            "unit_price": SPV2_UNIT_PRICE,
            "total_investor_capital": SPV2_TOTAL_INVESTOR_CAPITAL,
            "pclp1_units_held": SPV2_PCLP1_UNITS_HELD,
            "pclp1_cost_per_unit": SPV2_PCLP1_COST_PER_UNIT,
            "wcp_founding_bonus": SPV2_WCP_FOUNDING_BONUS,
            "wcp_bonus_nominal": SPV2_WCP_BONUS_NOMINAL,
            "wcp_level3_proxy": SPV2_WCP_LEVEL3_PROXY,
            "y10_wcp_crs_shares": SPV2_Y10_WCP_CRS_SHARES,
            "wcp_residual_shares": SPV2_WCP_RESIDUAL_SHARES,
            "commission_rebate_y0_gross": SPV2_COMMISSION_REBATE_Y0_GROSS,
            "opex_annual": SPV2_OPEX_ANNUAL,
            "setup_costs_y0": SPV2_SETUP_COSTS_Y0,
        },
        "years": years,
        "y10_endpoint": {
            "cumulative_cash": y10.cumulative_cash_to_investors,
            "pclp1_nav": y10.pclp1_nav_total,
            "wcp_residual_fv": y10.wcp_holding_fv,
            "total_return": total_return,
            "moic_aggregate": moic,
            "moic_per_investor_unit": (y10.cumulative_cash_per_unit + y10.y_end_position_per_unit) / SPV2_UNIT_PRICE,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spv::{pclp1_proforma, wcp_proforma};

    #[test]
    fn forecast_has_11_years() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv2 = forecast(&pclp1, &wcp);
        assert_eq!(spv2.len(), 11);
    }

    #[test]
    fn y4_plus_pclp1_distributions() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv2 = forecast(&pclp1, &wcp);
        // Y4+ should have PCLP 1 distributions = PCLP1.dpu × 250K
        for y in 4..=10 {
            let expected = pclp1[y].dpu * SPV2_PCLP1_UNITS_HELD;
            assert!((spv2[y].pclp1_distributions - expected).abs() < 1.0,
                    "Y{} PCLP1 dist = {} (expected {})", y, spv2[y].pclp1_distributions, expected);
        }
    }

    #[test]
    fn y10_wcp_crs_event() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv2 = forecast(&pclp1, &wcp);
        // Y10 CRS = 393,824 × WCP Y10 value
        let y10 = &spv2[10];
        assert!(y10.wcp_crs_realised_gain > 0.0);
        assert!(y10.wcp_shares_held == SPV2_WCP_RESIDUAL_SHARES);
    }

    #[test]
    fn y10_total_return_above_capital() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv2 = forecast(&pclp1, &wcp);
        let y10 = &spv2[10];
        let tr = y10.cumulative_cash_to_investors + y10.pclp1_nav_total + y10.wcp_holding_fv;
        assert!(tr > SPV2_TOTAL_INVESTOR_CAPITAL,
                "Y10 total return ({}) should exceed capital ({})", tr, SPV2_TOTAL_INVESTOR_CAPITAL);
    }

    #[test]
    fn json_dump_well_formed() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let json = forecast_json(&pclp1, &wcp);
        assert_eq!(json["version"], "V1");
        assert!(json["years"].is_array());
    }
}
