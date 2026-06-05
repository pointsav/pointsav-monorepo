use crate::excel::pclp1::{Pclp1Data, Pclp1Year};
use crate::excel::wcp::WcpData;

const AD2_UNITS: f64 = 250_000.0;
const AD2_UNIT_PRICE: f64 = 100.0;

// ─── Dual-asset Bencal SPV2 — WCP founding-bonus parameters (BRIEF v0.15.9 §5d) ────
//
// Bencal SPV2 receives 600,000 WCP common shares from the Strategic Partner block
// as a founding capital contribution in exchange for completing its minimum
// CAD 13,000,000 investment in Professional Centres Canada LP. Treatment locked at
// Flag 15 path (b) (RESOLVED 2026-06-02): recorded at FMV against contributed
// surplus equity at Y0; no Y0 IS impact at Bencal SPV2.
//
// Source of shares: carved from Strategic Partner's 1,800,000-share block
// (reduced to 1,200,000); WCP total outstanding UNCHANGED at 10,000,000.

/// `Ad2Config` — Bencal SPV2's founding-bonus WCP parameters. Defaults match the
/// BRIEF v0.15.9 §5c WCP cap-table values for the Bencal deployment; per-deal
/// overrides supplied via TOML.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Ad2Config {
    pub wcp_bonus_shares_received: f64,
    pub wcp_bonus_price_per_share: f64,
    pub wcp_bonus_proxy_value_per_share: f64,
    pub wcp_bonus_trigger_lp_minimum: f64,
    /// Year (0-indexed; 0 = Y1) at which the WCP exchange listing transitions
    /// the fair-value classification from Level 3 management proxy → Level 1
    /// observable market price. `None` if listing has not yet occurred.
    pub wcp_listing_year: Option<u8>,
}

impl Default for Ad2Config {
    fn default() -> Self {
        Self {
            wcp_bonus_shares_received: 600_000.0,
            wcp_bonus_price_per_share: 0.000_332_23,
            wcp_bonus_proxy_value_per_share: 4.55,
            wcp_bonus_trigger_lp_minimum: 13_000_000.0,
            wcp_listing_year: None,
        }
    }
}

/// Y0 capital-contribution journal entry per Flag 15 path (b).
///
/// At Y0 the 600,000 WCP shares are recorded at FMV (~$2,730,000 = 600,000 × $4.55
/// Level 3 management proxy) against contributed surplus equity. No income-statement
/// impact at Bencal SPV2; tax effect (ITA s.69 deemed FMV disposal) is recognised
/// at the Strategic Partner level only.
///
/// Balanced entry: dr_investment_wcp = cr_contributed_surplus + cr_cash.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Ad2WcpCapitalContribution {
    pub dr_investment_wcp: f64,
    pub cr_contributed_surplus: f64,
    pub cr_cash: f64,
}

impl Ad2WcpCapitalContribution {
    /// Compute the Y0 JE from `Ad2Config` parameters. Returns a balanced entry.
    pub fn from_config(cfg: &Ad2Config) -> Self {
        let shares = cfg.wcp_bonus_shares_received;
        let fmv = shares * cfg.wcp_bonus_proxy_value_per_share;
        let cash = shares * cfg.wcp_bonus_price_per_share;
        Self {
            dr_investment_wcp: fmv,
            cr_contributed_surplus: fmv - cash,
            cr_cash: cash,
        }
    }

    /// Sanity check — the entry must balance to within float epsilon.
    pub fn is_balanced(&self) -> bool {
        (self.dr_investment_wcp - self.cr_contributed_surplus - self.cr_cash).abs() < 1e-6
    }
}

/// Bencal SPV2's WCP holding — fair-value series over the 10-year forecast.
///
/// Pre-listing: fair value is a function of the Level 3 management proxy.
/// Post-listing (year ≥ wcp_listing_year): fair value follows the WCP forecast
/// per-share book value scaled by Bencal SPV2's share count.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Ad2WcpHolding {
    pub cost_basis: f64,
    pub y0_journal_entry: Ad2WcpCapitalContribution,
    pub stake_of_wcp_outstanding: f64,
    pub fair_value_by_year: [f64; 10],
    pub level_by_year: [FairValueLevel; 10],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum FairValueLevel {
    /// IFRS 13 Level 3 — management proxy / no observable market price.
    Level3Proxy,
    /// IFRS 13 Level 1 — quoted price in an active market for an identical asset.
    Level1Quoted,
}

/// Derive Bencal SPV2's WCP holding from `Ad2Config` + the WCP forecast.
///
/// The Level 3 management proxy stays constant pre-listing; on the listing year and
/// thereafter, the holding's fair value follows the WCP forecast's per-share book
/// value scaled by Bencal SPV2's share count (lookthrough basis).
pub fn derive_wcp_holding(cfg: &Ad2Config, wcp: &WcpData) -> Ad2WcpHolding {
    let shares = cfg.wcp_bonus_shares_received;
    let proxy_fv = shares * cfg.wcp_bonus_proxy_value_per_share;
    let cost = shares * cfg.wcp_bonus_price_per_share;

    let stake = if wcp.shares_outstanding > 0.0 {
        shares / wcp.shares_outstanding
    } else {
        0.0
    };

    let mut fair_value_by_year = [0.0; 10];
    let mut level_by_year = [FairValueLevel::Level3Proxy; 10];
    for y in 0..10 {
        let listed = match cfg.wcp_listing_year {
            Some(ly) => (y as u8) >= ly,
            None => false,
        };
        if listed {
            // Level 1 — lookthrough from WCP per-share value.
            // WCP book_value_per_share may be unpopulated; fall back to book_value × stake.
            let per_share = wcp.book.book_value_per_share[y];
            fair_value_by_year[y] = if per_share > 0.0 {
                per_share * shares
            } else {
                wcp.book.book_value[y] * stake
            };
            level_by_year[y] = FairValueLevel::Level1Quoted;
        } else {
            // Level 3 — management proxy held constant.
            fair_value_by_year[y] = proxy_fv;
            level_by_year[y] = FairValueLevel::Level3Proxy;
        }
    }

    Ad2WcpHolding {
        cost_basis: cost,
        y0_journal_entry: Ad2WcpCapitalContribution::from_config(cfg),
        stake_of_wcp_outstanding: stake,
        fair_value_by_year,
        level_by_year,
    }
}

pub fn derivation_json(pclp: &Pclp1Data) -> serde_json::Value {
    let sf = AD2_UNITS / pclp.assumptions.diluted_units;
    serde_json::json!({
        "source_model": "PCLP 1 Excel",
        "source_entity": pclp.entity,
        "method": "proportional_scale",
        "description": "Ambassadors Direct 2 LP holds 250,000 Professional Centres Canada LP units at $100/unit ($25M total investment). All dollar totals are scaled by units_held / pclp_diluted_units. Per-unit metrics are carried through unchanged.",
        "units_held": AD2_UNITS,
        "pclp_diluted_units": pclp.assumptions.diluted_units,
        "scale_factor": sf,
        "unit_price": AD2_UNIT_PRICE,
        "total_investment": AD2_UNITS * AD2_UNIT_PRICE,
        "authority": "spv-bencal governance document — Ambassadors Direct 2 LP subscription agreement"
    })
}

/// Derive Ambassadors Direct 2 LP Pclp1Data from the parent PCLP 1 model.
/// AD2 LP holds 250,000 Professional Centres Canada LP units at $100/unit = $25M.
/// All dollar totals scale by 250,000 / diluted_units; per-unit metrics are unchanged.
pub fn derive(pclp: &Pclp1Data) -> Pclp1Data {
    let sf = AD2_UNITS / pclp.assumptions.diluted_units;

    let mut assumptions = pclp.assumptions.clone();
    assumptions.diluted_units = AD2_UNITS;
    assumptions.total_equity = AD2_UNITS * AD2_UNIT_PRICE;

    Pclp1Data {
        title: pclp.title.clone(),
        entity: "Ambassadors Direct 2 Limited Partnership".to_string(),
        date: pclp.date.clone(),
        assumptions,
        years: pclp.years.iter().map(|y| scale_year(y, sf)).collect(),
        market_yield: pclp.market_yield,
        compounded_return_y8: pclp.compounded_return_y8,
    }
}

fn scale_year(y: &Pclp1Year, sf: f64) -> Pclp1Year {
    Pclp1Year {
        year: y.year,
        // Income totals
        noi: y.noi * sf,
        income_continuity: y.income_continuity * sf,
        issue_costs: y.issue_costs * sf,
        financing_costs: y.financing_costs * sf,
        advisory_fees: y.advisory_fees * sf,
        admin_compliance: y.admin_compliance * sf,
        board_of_directors: y.board_of_directors * sf,
        total_expenses: y.total_expenses * sf,
        ebitda: y.ebitda * sf,
        interest_net: y.interest_net * sf,
        funding_from_ops: y.funding_from_ops * sf,
        interest_coverage: y.interest_coverage, // ratio — unchanged
        debt_service_ratio: y.debt_service_ratio, // ratio — unchanged
        // Cash flow totals
        opening_cash: y.opening_cash * sf,
        new_equity: y.new_equity * sf,
        new_debt_gross: y.new_debt_gross * sf,
        capex: y.capex * sf,
        debt_repayment: y.debt_repayment * sf,
        distributions: y.distributions * sf,
        ending_cash: y.ending_cash * sf,
        // Debenture totals
        opening_debt: y.opening_debt * sf,
        debt_additions: y.debt_additions * sf,
        debt_payments: y.debt_payments * sf,
        ending_debt: y.ending_debt * sf,
        // Asset totals
        opening_assets: y.opening_assets * sf,
        total_capital_assets: y.total_capital_assets * sf,
        assets_generating_rent: y.assets_generating_rent * sf,
        buildings_under_construction: y.buildings_under_construction * sf,
        debt_to_dev_cost: y.debt_to_dev_cost, // ratio — unchanged
        // Valuation totals; per-unit unchanged
        asset_value_total: y.asset_value_total * sf,
        asset_value_per_unit: y.asset_value_per_unit,
        nav_total: y.nav_total * sf,
        nav_per_unit: y.nav_per_unit,
        distribution_yield: y.distribution_yield,
        total_expense_ratio: y.total_expense_ratio,
        distributions_to_lps: y.distributions_to_lps * sf,
        dist_per_unit: y.dist_per_unit,
        dist_yield_on_cost: y.dist_yield_on_cost,
        // Financial Forecast — all per-unit, all unchanged
        ff_revenue_pu: y.ff_revenue_pu,
        ff_dist_pu: y.ff_dist_pu,
        ff_dist_yield_on_cost: y.ff_dist_yield_on_cost,
        ff_asset_value_pu: y.ff_asset_value_pu,
        ff_total_debt_pu: y.ff_total_debt_pu,
        ff_nav_pu: y.ff_nav_pu,
        ff_market_value_pu: y.ff_market_value_pu,
        ff_coverage: y.ff_coverage,
        ff_debt_to_dev_cost: y.ff_debt_to_dev_cost,
        ff_debt_to_av: y.ff_debt_to_av,
        ff_ter: y.ff_ter,
        ff_sqft: y.ff_sqft,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel::wcp::{WcpBook, WcpFairDiv, WcpIncome, WcpMarket};

    fn synthetic_wcp(book_value_y10: f64) -> WcpData {
        let mut bv = [0.0; 10];
        bv[9] = book_value_y10;
        WcpData {
            title: "t".to_string(),
            entity: "WCP".to_string(),
            date: "Y0".to_string(),
            shares_outstanding: 10_000_000.0,
            price_per_share: 20.00,
            lps: vec![],
            income: WcpIncome {
                gross_income: [0.0; 10],
                referral_fees: [0.0; 10],
                wpi_consulting: [0.0; 10],
                gna_nyc: [0.0; 10],
                gna_berlin: [0.0; 10],
                total_expenses: [0.0; 10],
                ebitda: [0.0; 10],
                ebitda_per_share: [0.0; 10],
                taxes: [0.0; 10],
                earnings: [0.0; 10],
                earnings_per_share: [0.0; 10],
            },
            book: WcpBook {
                cumulative_fcf_wci: [0.0; 10],
                beneficial_ownership_lps: [0.0; 10],
                book_value: bv,
                book_value_per_share: [0.0; 10],
            },
            market: WcpMarket {
                earnings_valuation: [0.0; 10],
                market_valuation: [0.0; 10],
                pe_ratio: [0.0; 10],
                market_value_per_share: [0.0; 10],
            },
            fair_div: WcpFairDiv {
                fair_value_per_share: [0.0; 10],
                dividend_valuation: [0.0; 10],
                dividend_value_per_share: [0.0; 10],
            },
            gna_label_1: "L".to_string(),
            gna_label_2: "A".to_string(),
        }
    }

    #[test]
    fn ad2_config_defaults_match_brief() {
        let cfg = Ad2Config::default();
        assert_eq!(cfg.wcp_bonus_shares_received, 600_000.0);
        assert!((cfg.wcp_bonus_price_per_share - 0.000_332_23).abs() < 1e-10);
        assert!((cfg.wcp_bonus_proxy_value_per_share - 4.55).abs() < 1e-6);
        assert_eq!(cfg.wcp_bonus_trigger_lp_minimum, 13_000_000.0);
        assert_eq!(cfg.wcp_listing_year, None);
    }

    #[test]
    fn y0_capital_contribution_balances() {
        let cfg = Ad2Config::default();
        let je = Ad2WcpCapitalContribution::from_config(&cfg);
        assert!(je.is_balanced());
        // FMV: 600,000 × $4.55 = $2,730,000
        assert!((je.dr_investment_wcp - 2_730_000.0).abs() < 1e-3);
        // Cash: 600,000 × $0.00033223 = $199.338
        assert!((je.cr_cash - 199.338).abs() < 1e-2);
        // Contributed surplus: $2,730,000 − $199.338 ≈ $2,729,800.66
        assert!((je.cr_contributed_surplus - 2_729_800.662).abs() < 1e-1);
    }

    #[test]
    fn wcp_holding_pre_listing_constant_proxy() {
        let cfg = Ad2Config::default(); // wcp_listing_year = None
        let wcp = synthetic_wcp(1_000_000_000.0);
        let h = derive_wcp_holding(&cfg, &wcp);
        // All years should sit at the Level 3 proxy value (constant).
        for y in 0..10 {
            assert!(
                (h.fair_value_by_year[y] - 2_730_000.0).abs() < 1e-3,
                "y={y} fair value should be Level 3 proxy"
            );
            assert_eq!(h.level_by_year[y], FairValueLevel::Level3Proxy);
        }
        // Stake: 600,000 / 10,000,000 = 6%
        assert!((h.stake_of_wcp_outstanding - 0.06).abs() < 1e-9);
    }

    #[test]
    fn wcp_holding_post_listing_uses_lookthrough() {
        let mut cfg = Ad2Config::default();
        cfg.wcp_listing_year = Some(3); // Listing at Y4 (0-indexed = 3)
        let mut wcp = synthetic_wcp(0.0);
        // Set per-share book value at Y4+ to $10.00
        for y in 3..10 {
            wcp.book.book_value_per_share[y] = 10.00;
        }
        let h = derive_wcp_holding(&cfg, &wcp);
        // Y0-Y3 (indices 0-2) should be Level 3 proxy
        for y in 0..3 {
            assert_eq!(h.level_by_year[y], FairValueLevel::Level3Proxy);
            assert!((h.fair_value_by_year[y] - 2_730_000.0).abs() < 1e-3);
        }
        // Y4+ (indices 3-9) should be Level 1 lookthrough = $10.00 × 600,000 = $6,000,000
        for y in 3..10 {
            assert_eq!(h.level_by_year[y], FairValueLevel::Level1Quoted);
            assert!(
                (h.fair_value_by_year[y] - 6_000_000.0).abs() < 1e-3,
                "y={y} fair value should be Level 1 lookthrough"
            );
        }
    }

    #[test]
    fn cost_basis_under_one_dollar_per_thousand_shares() {
        let cfg = Ad2Config::default();
        let wcp = synthetic_wcp(0.0);
        let h = derive_wcp_holding(&cfg, &wcp);
        // 600,000 × $0.00033223 ≈ $199.338
        assert!((h.cost_basis - 199.338).abs() < 1e-2);
    }
}
