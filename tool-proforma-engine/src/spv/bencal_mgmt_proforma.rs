// Bencal Management Corp. (D6) — Manager-tier holding entity.
// BRIEF v0.15.6 §5f.
//
// Receives 10% lookthrough of SPV1 + SPV2. Two shares × $5 nominal = $10 paid-in capital.

use serde::Serialize;
use crate::spv::bencal_spv1_proforma::{self, BencalSpv1Year};
use crate::spv::bencal_spv2_proforma::{self, BencalSpv2Year};

// ─── Constants ──────────────────────────────────────────────────────────────

pub const BM_SHARES_OUTSTANDING: f64        = 2.0;
pub const BM_SHARE_PRICE: f64               = 5.00;
pub const BM_PAID_IN_CAPITAL: f64           = 10.00;       // 2 × $5 nominal

pub const BM_LOOKTHROUGH_STAKE_SPV1: f64    = 0.10;
pub const BM_LOOKTHROUGH_STAKE_SPV2: f64    = 0.10;

pub const BM_COMMISSION_REBATE_Y0_GROSS: f64 = 64_637.0;
pub const BM_OPEX_ANNUAL: f64               = 13_870.0;
pub const BM_SETUP_COSTS_Y0: f64            = 5_575.0;
pub const BM_TAX_RATE_CURRENT: f64          = 0.27;
pub const BM_TAX_RATE_DEFERRED: f64         = 0.135;

// ─── Output struct ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct BencalMgmtYear {
    pub year: u32,
    pub commission_rebate: f64,
    pub spv1_lookthrough_cash: f64,       // 10% of SPV1 net income / cash distributions
    pub spv2_lookthrough_cash: f64,       // 10% of SPV2 cash to investors
    pub spv1_lookthrough_fv: f64,         // 10% of SPV1 FVTPL change
    pub spv2_lookthrough_fv: f64,         // 10% of SPV2 FVTPL change
    pub total_investment_income: f64,
    pub setup_costs: f64,
    pub opex: f64,
    pub income_before_tax: f64,
    pub tax: f64,
    pub net_income: f64,
    pub opex_reserve_closing: f64,
    pub cumulative_cash: f64,
    pub spv1_nav_lookthrough: f64,
    pub spv2_nav_lookthrough: f64,
    pub portfolio_nav: f64,
}

pub fn forecast(spv1: &[BencalSpv1Year], spv2: &[BencalSpv2Year]) -> Vec<BencalMgmtYear> {
    let mut years: Vec<BencalMgmtYear> = Vec::with_capacity(11);
    let net_y0_rebate = BM_COMMISSION_REBATE_Y0_GROSS * (1.0 - BM_TAX_RATE_CURRENT);
    let initial_reserve = net_y0_rebate - BM_SETUP_COSTS_Y0;
    let mut reserve = 0.0;
    let mut cumulative_cash: f64 = 0.0;

    for y in 0u32..=10 {
        let commission = if y == 0 { BM_COMMISSION_REBATE_Y0_GROSS } else { 0.0 };
        let setup = if y == 0 { BM_SETUP_COSTS_Y0 } else { 0.0 };
        let opex = if y >= 1 { BM_OPEX_ANNUAL } else { 0.0 };

        let s1 = &spv1[y as usize];
        let s2 = &spv2[y as usize];

        // BM cash lookthrough = 10% of each SPV's investor cash distributions
        let spv1_cash = s1.crs_cash_to_investors * BM_LOOKTHROUGH_STAKE_SPV1;
        let spv2_cash = s2.cash_to_investors * BM_LOOKTHROUGH_STAKE_SPV2;

        // FVTPL lookthrough = 10% of each SPV's FV change
        let spv1_fv = (s1.wcp_fv_change) * BM_LOOKTHROUGH_STAKE_SPV1;
        let spv2_fv = (s2.pclp1_fv_change + s2.wcp_fv_change) * BM_LOOKTHROUGH_STAKE_SPV2;

        let total_income = commission + spv1_cash + spv2_cash + spv1_fv + spv2_fv;
        let income_before_tax = total_income - setup - opex;
        let tax = if income_before_tax > 0.0 { income_before_tax * BM_TAX_RATE_CURRENT } else { 0.0 };
        let net_income = income_before_tax - tax;

        if y == 0 {
            reserve = initial_reserve;
        } else if y <= 3 && reserve > 0.0 {
            reserve = (reserve - opex).max(0.0);
        }

        let bm_cash_this_year = spv1_cash + spv2_cash;
        cumulative_cash += bm_cash_this_year;

        let spv1_nav_lookthrough = (s1.residual_nav + s1.wcp_holding_fv) * BM_LOOKTHROUGH_STAKE_SPV1;
        // For SPV1 Y<10 use wcp_holding_fv (residual_nav only populated Y10)
        let spv1_nav_lookthrough = if y == 10 {
            s1.residual_nav * BM_LOOKTHROUGH_STAKE_SPV1
        } else {
            s1.wcp_holding_fv * BM_LOOKTHROUGH_STAKE_SPV1
        };
        let spv2_nav_lookthrough = (s2.pclp1_nav_total + s2.wcp_holding_fv) * BM_LOOKTHROUGH_STAKE_SPV2;
        let portfolio_nav = spv1_nav_lookthrough + spv2_nav_lookthrough;

        years.push(BencalMgmtYear {
            year: y,
            commission_rebate: commission,
            spv1_lookthrough_cash: spv1_cash,
            spv2_lookthrough_cash: spv2_cash,
            spv1_lookthrough_fv: spv1_fv,
            spv2_lookthrough_fv: spv2_fv,
            total_investment_income: total_income,
            setup_costs: setup,
            opex,
            income_before_tax,
            tax,
            net_income,
            opex_reserve_closing: reserve,
            cumulative_cash,
            spv1_nav_lookthrough,
            spv2_nav_lookthrough,
            portfolio_nav,
        });
    }
    years
}

pub fn forecast_json(spv1: &[BencalSpv1Year], spv2: &[BencalSpv2Year]) -> serde_json::Value {
    let years = forecast(spv1, spv2);
    let y10 = &years[10];
    let total_return = y10.cumulative_cash + y10.portfolio_nav;
    let moic = total_return / BM_PAID_IN_CAPITAL;
    serde_json::json!({
        "entity": "Bencal Management Corp.",
        "source": "tool-proforma-engine src/spv/bencal_mgmt_proforma module",
        "brief_section": "v0.15.6 §5f",
        "version": "V1",
        "generated_at": "2026-06-04",
        "consumes": "Bencal SPV1 V1 + Bencal SPV2 V1 forecasts (10% lookthrough each)",
        "inputs": {
            "shares_outstanding": BM_SHARES_OUTSTANDING,
            "share_price": BM_SHARE_PRICE,
            "paid_in_capital": BM_PAID_IN_CAPITAL,
            "lookthrough_stake_spv1": BM_LOOKTHROUGH_STAKE_SPV1,
            "lookthrough_stake_spv2": BM_LOOKTHROUGH_STAKE_SPV2,
            "commission_rebate_y0_gross": BM_COMMISSION_REBATE_Y0_GROSS,
            "opex_annual": BM_OPEX_ANNUAL,
            "setup_costs_y0": BM_SETUP_COSTS_Y0,
            "tax_rate_current": BM_TAX_RATE_CURRENT,
        },
        "years": years,
        "y10_endpoint": {
            "cumulative_cash_lookthrough": y10.cumulative_cash,
            "portfolio_nav_lookthrough": y10.portfolio_nav,
            "total_return": total_return,
            "moic_aggregate": moic,
            "moic_per_share": moic,
            "note": "Per-share MOIC is mechanically very high because BM share capital is nominal ($10 total against multi-million-dollar lookthrough claims). Read aggregate column."
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
        let spv1 = bencal_spv1_proforma::forecast(&wcp);
        let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
        let bm = forecast(&spv1, &spv2);
        assert_eq!(bm.len(), 11);
    }

    #[test]
    fn lookthrough_is_10pct_of_spv_cash() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv1 = bencal_spv1_proforma::forecast(&wcp);
        let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
        let bm = forecast(&spv1, &spv2);
        for y in 0..=10 {
            assert!((bm[y].spv1_lookthrough_cash - spv1[y].crs_cash_to_investors * 0.10).abs() < 1.0);
            assert!((bm[y].spv2_lookthrough_cash - spv2[y].cash_to_investors * 0.10).abs() < 1.0);
        }
    }

    #[test]
    fn y10_portfolio_nav_combines_spv1_spv2() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv1 = bencal_spv1_proforma::forecast(&wcp);
        let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
        let bm = forecast(&spv1, &spv2);
        let y10 = &bm[10];
        assert!(y10.portfolio_nav > 0.0);
        // Should be 10% of (SPV1 Y10 NAV + SPV2 Y10 NAV combined)
        let expected = (spv1[10].residual_nav + spv2[10].pclp1_nav_total + spv2[10].wcp_holding_fv) * 0.10;
        assert!((y10.portfolio_nav - expected).abs() < 1.0,
                "BM Y10 NAV {} should be ~{}", y10.portfolio_nav, expected);
    }

    #[test]
    fn json_dump_well_formed() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = wcp_proforma::forecast(&pclp1);
        let spv1 = bencal_spv1_proforma::forecast(&wcp);
        let spv2 = bencal_spv2_proforma::forecast(&pclp1, &wcp);
        let json = forecast_json(&spv1, &spv2);
        assert_eq!(json["version"], "V1");
        assert!(json["years"].is_array());
    }
}
