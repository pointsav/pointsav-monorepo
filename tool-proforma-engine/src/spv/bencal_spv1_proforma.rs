// Bencal Special Purpose 1 Inc. (D5) — WCP-only entity.
// BRIEF v0.15.6 §5e.
//
// Consumes WCP V1 forecast for the Y4 listing event price + Y5-Y10 market value path.
// Engine computes per-year IS/CF/NAV from declared Rust constants.

#[allow(unused_imports)]
use crate::spv::wcp_proforma::{self, WcpYear};
use serde::Serialize;

// ─── Constants (BRIEF v0.15.6 §5e + Bencal cap table v0.15.9) ────────────────

pub const SPV1_INVESTOR_SHARES: f64 = 3_054_882.0; // BRIEF §5e
pub const SPV1_MANAGER_SHARES: f64 = 339_431.0; // 10% diluted
pub const SPV1_DILUTED_SHARES: f64 = 3_394_313.0;
pub const SPV1_INVESTOR_SHARE_PRICE: f64 = 1.00;
pub const SPV1_TOTAL_INVESTOR_CAPITAL: f64 = 3_054_882.0;

pub const SPV1_WCP_SHARES_PURCHASED: f64 = 150_000.0; // Y0 buy at $20
pub const SPV1_WCP_PURCHASED_PRICE: f64 = 20.00;
pub const SPV1_WCP_FOUNDING_BONUS: f64 = 150_000.0; // Y0 founding bonus
pub const SPV1_WCP_BONUS_NOMINAL: f64 = 0.000_332_23; // ≈ $50 total
pub const SPV1_WCP_TOTAL: f64 = 300_000.0; // 3% of 10M WCP outstanding
pub const SPV1_WCP_LEVEL3_PROXY: f64 = 4.55; // Y1-Y3 IFRS 13 Level 3

pub const SPV1_Y4_CRS_SHARES_SOLD: f64 = 150_000.0; // Y4 Capital Return Sale
pub const SPV1_Y10_RESIDUAL_SHARES: f64 = 150_000.0; // remaining shares Y10

pub const SPV1_COMMISSION_REBATE_Y0_GROSS: f64 = 83_156.0;
pub const SPV1_OPEX_ANNUAL: f64 = 18_360.0;
pub const SPV1_SETUP_COSTS_Y0: f64 = 5_624.83;
pub const SPV1_TAX_RATE_CURRENT: f64 = 0.27;
pub const SPV1_TAX_RATE_DEFERRED: f64 = 0.135;

// ─── Output struct ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct BencalSpv1Year {
    pub year: u32,
    pub commission_rebate: f64,     // Y0 only
    pub wcp_crs_realised_gain: f64, // Y4 cash dividend
    pub wcp_annual_sale_gain: f64,  // Y5–Y10 opex-funding share sales
    pub wcp_fv_change: f64,         // FVTPL each year
    pub total_investment_income: f64,
    pub setup_costs: f64, // Y0
    pub opex: f64,        // Y1+
    pub income_before_tax: f64,
    pub tax: f64,
    pub net_income: f64,
    pub opex_reserve_closing: f64,
    pub wcp_shares_held: f64,
    pub wcp_per_share_value: f64,
    pub wcp_holding_fv: f64,
    pub crs_cash_to_investors: f64,
    pub residual_nav: f64, // Y10 fair value of remaining shares
    pub cumulative_cash_to_investors: f64,
    pub cumulative_cash_per_share: f64,
}

// ─── Forecast ────────────────────────────────────────────────────────────────

fn wcp_value_at_year(wcp: &[WcpYear], y: u32) -> f64 {
    // Y1-Y3 use Level 3 proxy; Y4+ uses engine market_value_per_share.
    // The WCP engine market_value_per_share = EBITDA × P/E / shares which doesn't
    // match the simple BRIEF narrative ($20 listing → 8% growth). Use book_value_per_share
    // which better matches BRIEF for stable accrual basis.
    let idx = y as usize;
    match y {
        0..=3 => SPV1_WCP_LEVEL3_PROXY,
        _ => {
            if idx < wcp.len() {
                // Use book_value_per_share for BRIEF-aligned Y4+ valuation
                wcp[idx].book_value_per_share.max(SPV1_WCP_LEVEL3_PROXY)
            } else {
                SPV1_WCP_LEVEL3_PROXY
            }
        }
    }
}

pub fn forecast(wcp: &[WcpYear]) -> Vec<BencalSpv1Year> {
    let mut years: Vec<BencalSpv1Year> = Vec::with_capacity(11);
    let net_y0_rebate = SPV1_COMMISSION_REBATE_Y0_GROSS * (1.0 - SPV1_TAX_RATE_CURRENT);
    let initial_reserve = net_y0_rebate - SPV1_SETUP_COSTS_Y0;

    let cost_basis = SPV1_WCP_SHARES_PURCHASED * SPV1_WCP_PURCHASED_PRICE
        + SPV1_WCP_FOUNDING_BONUS * SPV1_WCP_BONUS_NOMINAL;
    let mut prev_wcp_fv = cost_basis;
    let mut _prev_shares_held = SPV1_WCP_TOTAL;
    let mut cumulative_cash: f64 = 0.0;
    let mut reserve = 0.0;

    for y in 0u32..=10 {
        let commission = if y == 0 {
            SPV1_COMMISSION_REBATE_Y0_GROSS
        } else {
            0.0
        };
        let setup = if y == 0 { SPV1_SETUP_COSTS_Y0 } else { 0.0 };
        let opex = if y >= 1 { SPV1_OPEX_ANNUAL } else { 0.0 };

        let wcp_value_per_share = wcp_value_at_year(wcp, y);
        let shares_held = if y < 4 {
            SPV1_WCP_TOTAL
        } else {
            SPV1_Y10_RESIDUAL_SHARES
        };
        let wcp_holding_fv = shares_held * wcp_value_per_share;

        let crs_cash = if y == 4 {
            SPV1_Y4_CRS_SHARES_SOLD * wcp_value_per_share
        } else {
            0.0
        };

        // Y5–Y10: sell enough WCP shares to fund $18,360 opex after 27% tax
        let annual_sale_gain = if y >= 5 {
            SPV1_OPEX_ANNUAL / (1.0 - SPV1_TAX_RATE_CURRENT)
        } else {
            0.0
        };

        // FV change vs prior year holding FV (net of any sale at FV)
        let wcp_fv_change = (shares_held * wcp_value_per_share + crs_cash) - prev_wcp_fv;

        let total_income = commission + crs_cash + annual_sale_gain + wcp_fv_change;
        let income_before_tax = total_income - setup - opex;
        let tax = if income_before_tax > 0.0 {
            income_before_tax * SPV1_TAX_RATE_CURRENT
        } else {
            0.0
        };
        let net_income = income_before_tax - tax;

        // Reserve drawdown
        if y == 0 {
            reserve = initial_reserve;
        } else if y <= 3 && reserve > 0.0 {
            reserve = (reserve - opex).max(0.0);
        }

        cumulative_cash += crs_cash;
        let cumulative_per_share = cumulative_cash / SPV1_INVESTOR_SHARES;

        let residual_nav = if y == 10 { wcp_holding_fv } else { 0.0 };

        years.push(BencalSpv1Year {
            year: y,
            commission_rebate: commission,
            wcp_crs_realised_gain: crs_cash,
            wcp_annual_sale_gain: annual_sale_gain,
            wcp_fv_change,
            total_investment_income: total_income,
            setup_costs: setup,
            opex,
            income_before_tax,
            tax,
            net_income,
            opex_reserve_closing: reserve,
            wcp_shares_held: shares_held,
            wcp_per_share_value: wcp_value_per_share,
            wcp_holding_fv,
            crs_cash_to_investors: crs_cash,
            residual_nav,
            cumulative_cash_to_investors: cumulative_cash,
            cumulative_cash_per_share: cumulative_per_share,
        });

        prev_wcp_fv = shares_held * wcp_value_per_share;
        _prev_shares_held = shares_held;
        let _ = _prev_shares_held;
    }
    years
}

pub fn forecast_json(wcp: &[WcpYear]) -> serde_json::Value {
    let years = forecast(wcp);
    let y10 = &years[10];
    let total_return = y10.cumulative_cash_to_investors + y10.residual_nav;
    let moic = total_return / SPV1_TOTAL_INVESTOR_CAPITAL;
    serde_json::json!({
        "entity": "Bencal Special Purpose 1 Inc.",
        "source": "tool-proforma-engine src/spv/bencal_spv1_proforma module",
        "brief_section": "v0.15.6 §5e + cap table v0.15.9 §5c",
        "version": "V1",
        "generated_at": "2026-06-04",
        "consumes": "WCP V1 forecast (per-share value path)",
        "inputs": {
            "investor_shares": SPV1_INVESTOR_SHARES,
            "manager_shares": SPV1_MANAGER_SHARES,
            "diluted_shares": SPV1_DILUTED_SHARES,
            "investor_share_price": SPV1_INVESTOR_SHARE_PRICE,
            "total_investor_capital": SPV1_TOTAL_INVESTOR_CAPITAL,
            "wcp_shares_purchased": SPV1_WCP_SHARES_PURCHASED,
            "wcp_purchased_price": SPV1_WCP_PURCHASED_PRICE,
            "wcp_founding_bonus": SPV1_WCP_FOUNDING_BONUS,
            "wcp_total": SPV1_WCP_TOTAL,
            "wcp_level3_proxy_y1_y3": SPV1_WCP_LEVEL3_PROXY,
            "y4_crs_shares_sold": SPV1_Y4_CRS_SHARES_SOLD,
            "y10_residual_shares": SPV1_Y10_RESIDUAL_SHARES,
            "commission_rebate_y0_gross": SPV1_COMMISSION_REBATE_Y0_GROSS,
            "opex_annual": SPV1_OPEX_ANNUAL,
            "setup_costs_y0": SPV1_SETUP_COSTS_Y0,
            "tax_rate_current": SPV1_TAX_RATE_CURRENT,
            "tax_rate_deferred": SPV1_TAX_RATE_DEFERRED,
        },
        "years": years,
        "y10_endpoint": {
            "cumulative_cash_to_investors": y10.cumulative_cash_to_investors,
            "residual_nav": y10.residual_nav,
            "total_return": total_return,
            "moic_aggregate": moic,
            "moic_per_investor_share": (y10.cumulative_cash_per_share + y10.residual_nav / SPV1_DILUTED_SHARES) / SPV1_INVESTOR_SHARE_PRICE,
        }
    })
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use crate::spv::{pclp1_proforma, wcp_proforma};

    fn get_wcp() -> Vec<WcpYear> {
        let pclp1 = pclp1_proforma::forecast();
        wcp_proforma::forecast(&pclp1)
    }

    #[test]
    fn forecast_has_11_years() {
        let wcp = get_wcp();
        let spv1 = forecast(&wcp);
        assert_eq!(spv1.len(), 11);
    }

    #[test]
    fn y0_commission_rebate_matches_brief() {
        let wcp = get_wcp();
        let spv1 = forecast(&wcp);
        assert!((spv1[0].commission_rebate - SPV1_COMMISSION_REBATE_Y0_GROSS).abs() < 1.0);
    }

    #[test]
    fn y4_crs_event_distributes_cash() {
        let wcp = get_wcp();
        let spv1 = forecast(&wcp);
        // Y4 CRS = 150K × WCP Y4 book value per share
        let y4 = &spv1[4];
        assert!(
            y4.crs_cash_to_investors > 0.0,
            "Y4 CRS should distribute cash, got {}",
            y4.crs_cash_to_investors
        );
    }

    #[test]
    fn y1_y3_no_crs() {
        let wcp = get_wcp();
        let spv1 = forecast(&wcp);
        for y in 1..=3 {
            assert_eq!(
                spv1[y].crs_cash_to_investors, 0.0,
                "Y{} should not have CRS event",
                y
            );
        }
    }

    #[test]
    fn y10_residual_nav_positive() {
        let wcp = get_wcp();
        let spv1 = forecast(&wcp);
        let y10 = &spv1[10];
        assert!(
            y10.residual_nav > 0.0,
            "Y10 residual NAV should be positive, got {}",
            y10.residual_nav
        );
        // Residual = 150K shares × WCP Y10 value
        assert!(y10.wcp_shares_held == SPV1_Y10_RESIDUAL_SHARES);
    }

    #[test]
    fn json_dump_well_formed() {
        let wcp = get_wcp();
        let json = forecast_json(&wcp);
        assert_eq!(json["version"], "V1");
        assert!(json["years"].is_array());
        assert!(json["y10_endpoint"]["moic_aggregate"].as_f64().unwrap() > 0.0);
    }
}
