use crate::excel::pclp1::Pclp1Data;

fn fmt_m(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    let m = v / 1_000_000.0;
    if m < 0.0 {
        format!("({:.2}M)", m.abs())
    } else {
        format!("{:.2}M", m)
    }
}

fn fmt_k(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    let k = v / 1_000.0;
    if k < 0.0 {
        format!("({:.1}K)", k.abs())
    } else {
        format!("{:.1}K", k)
    }
}

fn fmt_pct(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    format!("{:.2}%", v * 100.0)
}

fn fmt_dollar(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    if v < 0.0 {
        format!("({:.2})", v.abs())
    } else {
        format!("{:.2}", v)
    }
}

fn fmt_ratio(v: Option<f64>) -> String {
    match v {
        None => "—".to_string(),
        Some(r) => format!("{:.2}x", r),
    }
}

fn yr_row(label: &str, vals: &[f64], fmt: fn(f64) -> String) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt(v)).collect();
    format!("| {:<42} | {} |\n", label, cells.join(" | "))
}

fn yr_row_opt(label: &str, vals: &[Option<f64>], fmt: fn(Option<f64>) -> String) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt(v)).collect();
    format!("| {:<42} | {} |\n", label, cells.join(" | "))
}

fn header_row(years: &[String; 10]) -> String {
    format!("| {:<42} | {} |\n", "", years.to_vec().join(" | "))
}

fn separator(n_years: usize) -> String {
    let col = "|:---".repeat(n_years + 1);
    format!("{}|\n", col)
}

pub fn render(data: &Pclp1Data) -> String {
    let mut out = String::new();

    let yrs = &data.assumptions.year_labels;
    let y = &data.years;

    // ── Page 1: Income Statement ──────────────────────────────────────────────
    out.push_str(&format!(
        "# {}\n\n**{}** — {}\n\n",
        data.title, data.entity, data.date
    ));
    out.push_str("---\n\n");
    out.push_str("## Income Statement\n\n");

    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Net Operating Income",
        &y.iter().map(|y| y.noi).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Income Continuity",
        &y.iter().map(|y| y.income_continuity).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Issue Costs",
        &y.iter().map(|y| y.issue_costs).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Financing Costs",
        &y.iter().map(|y| y.financing_costs).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Advisory Fees",
        &y.iter().map(|y| y.advisory_fees).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Admin & Compliance",
        &y.iter().map(|y| y.admin_compliance).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Board of Directors",
        &y.iter().map(|y| y.board_of_directors).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Total Expenses",
        &y.iter().map(|y| y.total_expenses).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "EBITDA",
        &y.iter().map(|y| y.ebitda).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Interest (Net)",
        &y.iter().map(|y| y.interest_net).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Funding from Operations",
        &y.iter().map(|y| y.funding_from_ops).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row_opt(
        "Interest Coverage",
        &y.iter().map(|y| y.interest_coverage).collect::<Vec<_>>(),
        fmt_ratio,
    ));
    out.push_str(&yr_row_opt(
        "Debt Service Ratio",
        &y.iter().map(|y| y.debt_service_ratio).collect::<Vec<_>>(),
        fmt_ratio,
    ));

    // ── Cash Flow Statement ───────────────────────────────────────────────────
    out.push_str("\n---\n\n## Cash Flow Statement\n\n");
    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Opening Cash",
        &y.iter().map(|y| y.opening_cash).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "New Equity",
        &y.iter().map(|y| y.new_equity).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "New Debt (Gross)",
        &y.iter().map(|y| y.new_debt_gross).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Capital Expenditures",
        &y.iter().map(|y| y.capex).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Debt Repayment",
        &y.iter().map(|y| y.debt_repayment).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Distributions",
        &y.iter().map(|y| y.distributions).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Ending Cash",
        &y.iter().map(|y| y.ending_cash).collect::<Vec<_>>(),
        fmt_m,
    ));

    // ── Debenture / Debt Schedule ─────────────────────────────────────────────
    out.push_str("\n---\n\n## Debenture Schedule\n\n");
    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Opening Debt",
        &y.iter().map(|y| y.opening_debt).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Debt Additions",
        &y.iter().map(|y| y.debt_additions).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Debt Payments",
        &y.iter().map(|y| y.debt_payments).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Ending Debt",
        &y.iter().map(|y| y.ending_debt).collect::<Vec<_>>(),
        fmt_m,
    ));

    // ── Asset Schedule ────────────────────────────────────────────────────────
    out.push_str("\n---\n\n## Asset Schedule\n\n");
    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Opening Assets",
        &y.iter().map(|y| y.opening_assets).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Total Capital Assets",
        &y.iter().map(|y| y.total_capital_assets).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Assets Generating Rent",
        &y.iter()
            .map(|y| y.assets_generating_rent)
            .collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Buildings Under Construction",
        &y.iter()
            .map(|y| y.buildings_under_construction)
            .collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Debt-to-Dev Cost",
        &y.iter().map(|y| y.debt_to_dev_cost).collect::<Vec<_>>(),
        fmt_pct,
    ));

    // ── Valuation ─────────────────────────────────────────────────────────────
    out.push_str("\n---\n\n## Valuation\n\n");
    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Asset Value (Total)",
        &y.iter().map(|y| y.asset_value_total).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Asset Value (per Unit)",
        &y.iter().map(|y| y.asset_value_per_unit).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "NAV (Total)",
        &y.iter().map(|y| y.nav_total).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "NAV (per Unit)",
        &y.iter().map(|y| y.nav_per_unit).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Distribution Yield",
        &y.iter().map(|y| y.distribution_yield).collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Total Expense Ratio",
        &y.iter().map(|y| y.total_expense_ratio).collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Distributions to LPs",
        &y.iter().map(|y| y.distributions_to_lps).collect::<Vec<_>>(),
        fmt_m,
    ));
    out.push_str(&yr_row(
        "Distribution (per Unit)",
        &y.iter().map(|y| y.dist_per_unit).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Distribution Yield on Cost",
        &y.iter().map(|y| y.dist_yield_on_cost).collect::<Vec<_>>(),
        fmt_pct,
    ));

    // ── Page 2: Financial Forecast (verbatim AA12:AN35 layout) ───────────────
    out.push_str("\n\n---\n\n## Financial Forecast — Per Unit\n\n");

    out.push_str(&header_row(yrs));
    out.push_str(&separator(10));
    out.push_str(&yr_row(
        "Revenue per Unit",
        &y.iter().map(|y| y.ff_revenue_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Distribution per Unit",
        &y.iter().map(|y| y.ff_dist_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Dist. Yield on Cost",
        &y.iter()
            .map(|y| y.ff_dist_yield_on_cost)
            .collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Asset Value per Unit",
        &y.iter().map(|y| y.ff_asset_value_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Total Debt per Unit",
        &y.iter().map(|y| y.ff_total_debt_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "NAV per Unit",
        &y.iter().map(|y| y.ff_nav_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row(
        "Market Value per Unit",
        &y.iter().map(|y| y.ff_market_value_pu).collect::<Vec<_>>(),
        fmt_dollar,
    ));
    out.push_str(&yr_row_opt(
        "Interest Coverage",
        &y.iter().map(|y| y.ff_coverage).collect::<Vec<_>>(),
        fmt_ratio,
    ));
    out.push_str(&yr_row(
        "Debt to Dev Cost",
        &y.iter().map(|y| y.ff_debt_to_dev_cost).collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Debt to Asset Value",
        &y.iter().map(|y| y.ff_debt_to_av).collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Total Expense Ratio",
        &y.iter().map(|y| y.ff_ter).collect::<Vec<_>>(),
        fmt_pct,
    ));
    out.push_str(&yr_row(
        "Sqft (stabilised)",
        &y.iter().map(|y| y.ff_sqft).collect::<Vec<_>>(),
        fmt_k,
    ));

    out.push_str(&format!(
        "\n**Market Yield:** {:.2}%    **Compounded Return (Y8):** {:.2}%\n",
        data.market_yield * 100.0,
        data.compounded_return_y8 * 100.0,
    ));

    out
}
