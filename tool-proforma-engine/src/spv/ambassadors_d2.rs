use crate::excel::pclp1::{Pclp1Data, Pclp1Year};

const AD2_UNITS: f64 = 250_000.0;
const AD2_UNIT_PRICE: f64 = 100.0;

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
