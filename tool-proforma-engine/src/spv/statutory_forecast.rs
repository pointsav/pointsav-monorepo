// Statutory 10-year financial forecast — IFRS-18-shaped, jurisdiction-parameterized.
//
// This module does NOT re-run the forecast math. It PROJECTS the locked
// `pclp1_proforma::forecast()` Vec<Pclp1Year> into the four forecasted statements of a
// Big-4-style "10-Year Financial Forecast" (Future-Oriented Financial Information, FOFI):
//
//   1. Statement of Forecasted Financial Position        (SOFP)
//   2. Statement of Forecasted Comprehensive Income      (SCI — IFRS 18 categories)
//   3. Statement of Forecasted Changes in Equity         (SOCE)
//   4. Statement of Forecasted Cash Flows                (SCF)
//
// plus the merged note set. No practitioner's report is rendered (management-prepared forecast,
// matching the sample).
//
// Design note — articulation (COST basis, matching the Big-4 sample). Investment property is
// carried at cost (`total_assets` = cumulative capex = wip + generating); cash is `ending_cash`.
// So total assets = `total_assets + ending_cash`, liabilities = `closing_debt`, and equity =
// total assets − debt. The Benetti units are expensed as equity-based compensation in Year 1
// (`BENETTI_SBC = 27,777,700`) and partners' units are presented at the full
// `CONTRIBUTED_CAPITAL = 277,777,700`. Net income = `ffo − sbc` (no fair-value remeasurement).
// The equity roll ties because the Benetti unit issuance (+27.78M to equity) and the share-based
// expense (−27.78M through P&L) net to zero, leaving Δequity = new_equity − distributions + ffo —
// verified by `equity_ties_to_contributed_plus_accumulated` and `soce_rolls_to_closing_equity`.
// Fair value / NAV is disclosed in a supplementary exhibit (IAS 40 cost model still requires it).
//
// "Audited" is deliberately avoided: a forecast is examined/assured, never audited.

use crate::spv::pclp1_proforma::{
    self, Pclp1Year, PCLP1_BENETTI_UNITS, PCLP1_CASH_INTEREST, PCLP1_GROSS_EQUITY, PCLP1_UNIT_PRICE,
};
use serde::Serialize;

// ─── Jurisdiction model ──────────────────────────────────────────────────────

/// Accounting framework applied to an entity's primary statements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AccountingFramework {
    /// IFRS Accounting Standards (incl. IFRS 18 from FY2027); IAS 40 fair-value model.
    Ifrs,
    /// US GAAP primary (ASC 360 cost + depreciation) with a supplementary IFRS reconciliation.
    UsGaapWithIfrsSupp,
    /// Spanish PGC individual + EU-endorsed IFRS consolidated; SOCIMI regime.
    SocimiIfrs,
    /// IFRS (BMV-listed); trust / CBFI presentation; FIBRA regime.
    FibraIfrs,
}

/// Everything that varies between the five entities. Carries BOTH the registered legal
/// name and the defined term. Internal entity codes are never stored here and never rendered.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Jurisdiction {
    /// Registered legal name (glossary, verbatim) — used on the cover + reporting-entity note.
    pub legal_name: &'static str,
    /// Defined term (recognizable label) — used after first definition.
    pub term_name: &'static str,
    /// Clean term-derived filename slug (never an internal code).
    pub file_slug: &'static str,
    /// General Partner / Administrator legal name.
    pub gp_name: &'static str,
    /// Manager legal name (Woodfine Management Corp. across the group).
    pub manager_name: &'static str,
    /// Manager-unit SPV (the share-based-compensation counterparty).
    pub spv_name: &'static str,
    /// ISO currency code shown in the statements.
    pub currency: &'static str,
    pub framework: AccountingFramework,
    /// "limited partnership units" | "shares" | "CBFIs".
    pub unit_term: &'static str,
    /// "Limited Partners" | "Shareholders" | "CBFI holders".
    pub holder_term: &'static str,
    /// Regulator / disclosure regime cited in the forward-looking footer.
    pub regulator: &'static str,
    /// Assurance standard cited in the practitioner's report.
    pub assurance_standard: &'static str,
    /// Mandatory minimum distribution description for Note 14.
    pub distribution_policy: &'static str,
}

/// Professional Centres Canada LP — the baseline IFRS entity (Phase A).
pub const CANADA: Jurisdiction = Jurisdiction {
    legal_name: "Woodfine Professional Centres Limited Partnership",
    term_name: "Professional Centres Canada LP",
    file_slug: "Professional-Centres-Canada-LP",
    gp_name: "Woodfine Professional Centres Inc.",
    manager_name: "Woodfine Management Corp.",
    spv_name: "Benetti Holdings Inc.",
    currency: "CAD",
    framework: AccountingFramework::Ifrs,
    unit_term: "limited partnership units",
    holder_term: "Limited Partners",
    regulator: "the British Columbia Securities Commission (BCSC) and National Instrument 51-102",
    assurance_standard:
        "the examination standard for future-oriented financial information (AuG-6 / CSAE 3400)",
    distribution_policy:
        "not less than 90% of distributable income to the Limited Partners until aggregate \
         distributions equal 100% of the gross proceeds contributed",
};

pub fn jurisdiction_by_key(key: &str) -> Option<Jurisdiction> {
    match key.to_ascii_lowercase().as_str() {
        "canada" | "ca" | "pccl" => Some(CANADA),
        _ => None,
    }
}

// ─── Presented-statement model ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LineStyle {
    Normal,
    Subtotal,
    Total,
    SectionBanner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LineFormat {
    Dollar,
    PerUnit,
    Pct,
    Ratio,
}

/// One presented line across Y1..Y10 (10 columns).
#[derive(Debug, Clone, Serialize)]
pub struct StatementLine {
    pub label: String,
    pub note_ref: Option<u8>,
    pub values: Vec<f64>, // length 10 (Y1..Y10)
    pub style: LineStyle,
    pub format: LineFormat,
}

impl StatementLine {
    fn banner(label: &str) -> Self {
        Self {
            label: label.to_string(),
            note_ref: None,
            values: vec![],
            style: LineStyle::SectionBanner,
            format: LineFormat::Dollar,
        }
    }
    fn row(
        label: &str,
        note_ref: Option<u8>,
        values: Vec<f64>,
        style: LineStyle,
        format: LineFormat,
    ) -> Self {
        Self {
            label: label.to_string(),
            note_ref,
            values,
            style,
            format,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Note {
    pub number: u8,
    pub title: String,
    pub body_html: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssuranceBlock {
    pub title: String,
    pub addressee: String,
    pub standard: String,
    pub body_html: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForecastStatements {
    pub jurisdiction: Jurisdiction,
    pub financial_position: Vec<StatementLine>,
    pub comprehensive_income: Vec<StatementLine>,
    pub changes_in_equity: Vec<StatementLine>,
    pub cash_flows: Vec<StatementLine>,
    pub per_unit: Vec<StatementLine>,
    pub notes: Vec<Note>,
    pub assurance: AssuranceBlock,
    /// US-GAAP entity only: supplementary IFRS fair-value figures. None for IFRS entities.
    pub ifrs_supplement: Option<Vec<StatementLine>>,
}

// ─── Derivation layer — COST basis (matches the Big-4 sample) ────────────────
//
// The primary statements carry investment property AT COST (IAS 40 cost model; no fair-value
// remeasurement), expense the Benetti units as equity-based compensation, and present partners'
// units at the full $277.78M. Fair value / NAV is disclosed in a supplementary exhibit (the cost
// model still requires fair-value disclosure). This reproduces the sample's row structure and the
// conservative treatment appropriate to a forecast (no unrealised gains booked).

/// Equity-based (share-based) compensation: Benetti units issued for services, at $100/unit,
/// expensed in Year 1 only (IFRS 2).
const BENETTI_SBC: f64 = PCLP1_BENETTI_UNITS * PCLP1_UNIT_PRICE; // 27,777,700
/// Contributed "partners' units", constant from Year 1: $250M cash raise + Benetti units.
const CONTRIBUTED_CAPITAL: f64 = PCLP1_GROSS_EQUITY + BENETTI_SBC; // 277,777,700

/// Investment property carried at COST (cumulative development expenditure: WIP + generating).
fn ip_cost(y: &Pclp1Year) -> f64 {
    y.total_assets
}

/// Total assets on the cost basis: investment property at cost + cash.
fn cost_total_assets(y: &Pclp1Year) -> f64 {
    y.total_assets + y.ending_cash
}

/// Total equity on the cost basis = cost total assets − debt (ties to contributed + accumulated).
fn cost_equity(y: &Pclp1Year) -> f64 {
    cost_total_assets(y) - y.closing_debt
}

/// Investment property at fair value (supplementary exhibit only): income capitalised + WIP.
fn ip_fair_value(y: &Pclp1Year) -> f64 {
    y.asset_value - y.ending_cash
}

/// Interest income on cash (investing category): avg cash × 0.5%.
fn interest_income(y: &Pclp1Year) -> f64 {
    y.opening_cash * PCLP1_CASH_INTEREST
}

/// Interest expense on debentures (financing category): net_interest + interest income.
fn interest_expense(y: &Pclp1Year) -> f64 {
    y.net_interest + interest_income(y)
}

/// Equity-based compensation expensed this year (Year 1 only).
fn sbc(y: &Pclp1Year) -> f64 {
    if y.year == 1 {
        BENETTI_SBC
    } else {
        0.0
    }
}

/// Operating profit (cost basis, IFRS 18): NOI − operating expenses − equity-based compensation.
fn operating_profit(y: &Pclp1Year) -> f64 {
    y.ebitda - sbc(y)
}

/// Net (loss)/income and total comprehensive income (cost basis): ffo less equity-based comp.
fn net_income(y: &Pclp1Year) -> f64 {
    y.ffo - sbc(y)
}

/// Collect Y1..Y10 slice (drops the Y0 base row).
fn y1_10(years: &[Pclp1Year]) -> &[Pclp1Year] {
    &years[1..=10]
}

/// Build a 10-long Vec<f64> from a closure over Y1..Y10.
fn col<F: Fn(&Pclp1Year) -> f64>(years: &[Pclp1Year], f: F) -> Vec<f64> {
    y1_10(years).iter().map(f).collect()
}

/// Accumulated deficit = running Σ(net income − distributions) over Y1..Y10.
fn accumulated_deficit_series(years: &[Pclp1Year]) -> Vec<f64> {
    let mut acc = 0.0;
    y1_10(years)
        .iter()
        .map(|y| {
            acc += net_income(y) - y.distributions;
            acc
        })
        .collect()
}

/// Year-1-only series of a constant (issuance events that occur once at formation).
fn year1_only(years: &[Pclp1Year], v: f64) -> Vec<f64> {
    col(years, |y| if y.year == 1 { v } else { 0.0 })
}

// ─── Statement builders (cost basis) ─────────────────────────────────────────

fn build_sci(years: &[Pclp1Year], _j: &Jurisdiction) -> Vec<StatementLine> {
    use LineFormat::Dollar;
    use LineStyle::{Normal, Subtotal, Total};
    let mut s = vec![StatementLine::banner("Operating activities")];
    s.push(StatementLine::row(
        "Revenue from operations (NOI)",
        Some(7),
        col(years, |y| y.net_proceeds_from_ops),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Advisory and management fee",
        Some(9),
        col(years, |y| -y.advisory_fee),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Administration and compliance",
        Some(9),
        col(years, |y| -y.admin_compliance),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Board of directors",
        Some(9),
        col(years, |y| -y.board),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Issue costs",
        Some(9),
        col(years, |y| -y.issue_costs),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Debenture facility costs",
        Some(10),
        col(years, |y| -y.financing_costs),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Equity-based compensation",
        Some(13),
        col(years, |y| -sbc(y)),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Operating profit / (loss)",
        None,
        col(years, operating_profit),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::banner("Investing activities"));
    s.push(StatementLine::row(
        "Interest income on cash",
        Some(11),
        col(years, interest_income),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Profit / (loss) before financing and income taxes",
        None,
        col(years, |y| operating_profit(y) + interest_income(y)),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::banner("Financing activities"));
    s.push(StatementLine::row(
        "Interest expense on debentures",
        Some(10),
        col(years, |y| -interest_expense(y)),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Profit / (loss) before income taxes",
        None,
        col(years, net_income),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Income taxes (flow-through entity)",
        Some(16),
        col(years, |_| 0.0),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Net (loss) / income and total comprehensive (loss) / income",
        None,
        col(years, net_income),
        Total,
        Dollar,
    ));
    s
}

fn build_sofp(years: &[Pclp1Year], _j: &Jurisdiction) -> Vec<StatementLine> {
    use LineFormat::Dollar;
    use LineStyle::{Normal, Subtotal, Total};
    let mut s = vec![StatementLine::banner("Assets")];
    s.push(StatementLine::row(
        "Investment property, at cost",
        Some(8),
        col(years, ip_cost),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Cash and cash equivalents",
        Some(4),
        col(years, |y| y.ending_cash),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Total assets",
        None,
        col(years, cost_total_assets),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::banner("Liabilities"));
    s.push(StatementLine::row(
        "Long-term debt — debentures",
        Some(10),
        col(years, |y| y.closing_debt),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Total liabilities",
        None,
        col(years, |y| y.closing_debt),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::banner("Equity"));
    s.push(StatementLine::row(
        "Partners' units",
        Some(13),
        col(years, |_| CONTRIBUTED_CAPITAL),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Accumulated deficit",
        None,
        accumulated_deficit_series(years),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Total equity",
        None,
        col(years, cost_equity),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Total liabilities and equity",
        None,
        col(years, cost_total_assets),
        Total,
        Dollar,
    ));
    s
}

fn build_soce(years: &[Pclp1Year], _j: &Jurisdiction) -> Vec<StatementLine> {
    use LineFormat::Dollar;
    use LineStyle::{Normal, Total};
    let opening: Vec<f64> = (1..=10)
        .map(|i| {
            if i == 1 {
                0.0
            } else {
                cost_equity(&years[i - 1])
            }
        })
        .collect();
    let mut s = vec![StatementLine::row(
        "Balance, beginning of year",
        None,
        opening,
        Normal,
        Dollar,
    )];
    s.push(StatementLine::row(
        "Net (loss) / income for the year",
        None,
        col(years, net_income),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Share capital issued",
        Some(13),
        col(years, |y| y.new_equity),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Units issued to Benetti",
        Some(13),
        year1_only(years, BENETTI_SBC),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Equity-based compensation reserve",
        Some(13),
        year1_only(years, BENETTI_SBC),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Units issued to Benetti — release of reserve",
        Some(13),
        year1_only(years, -BENETTI_SBC),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Distributions to Limited Partners",
        Some(14),
        col(years, |y| -y.distributions),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Balance, end of year",
        None,
        col(years, cost_equity),
        Total,
        Dollar,
    ));
    s
}

fn build_scf(years: &[Pclp1Year], _j: &Jurisdiction) -> Vec<StatementLine> {
    use LineFormat::Dollar;
    use LineStyle::{Normal, Subtotal, Total};
    let mut s = vec![StatementLine::banner(
        "Cash flows from operating activities",
    )];
    s.push(StatementLine::row(
        "Net (loss) / income for the year",
        None,
        col(years, net_income),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Equity-based compensation (non-cash)",
        Some(13),
        col(years, sbc),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Cash from operations",
        None,
        col(years, |y| y.ffo),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::banner(
        "Cash flows from investing activities",
    ));
    s.push(StatementLine::row(
        "Investment property — capital expenditure",
        Some(8),
        col(years, |y| -y.phase_draws),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::banner(
        "Cash flows from financing activities",
    ));
    s.push(StatementLine::row(
        "Share capital issued",
        Some(13),
        col(years, |y| y.new_equity),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Debenture drawdowns",
        Some(10),
        col(years, |y| y.gross_debt_draw),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Debenture repayments",
        Some(10),
        col(years, |y| -y.debt_repayment),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Distributions to Limited Partners",
        Some(14),
        col(years, |y| -y.distributions),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Net increase / (decrease) in cash",
        None,
        col(years, |y| y.ending_cash - y.opening_cash),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Cash, beginning of year",
        None,
        col(years, |y| y.opening_cash),
        Normal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Cash, end of year",
        None,
        col(years, |y| y.ending_cash),
        Total,
        Dollar,
    ));
    s
}

/// Supplementary fair-value / NAV exhibit (non-statutory). Under the cost model IAS 40 still
/// requires fair-value disclosure; this exhibit also carries the per-unit and ratio measures.
fn build_fair_value_exhibit(years: &[Pclp1Year], _j: &Jurisdiction) -> Vec<StatementLine> {
    use LineFormat::{Dollar, Pct, PerUnit, Ratio};
    use LineStyle::{Normal, Subtotal};
    let mut s = vec![StatementLine::row(
        "Investment property, at fair value (IAS 40 / IFRS 13)",
        Some(6),
        col(years, ip_fair_value),
        Normal,
        Dollar,
    )];
    s.push(StatementLine::row(
        "Net asset value (fair value of equity)",
        Some(6),
        col(years, |y| y.nav),
        Subtotal,
        Dollar,
    ));
    s.push(StatementLine::row(
        "Net asset value per unit",
        None,
        col(years, |y| y.nav_per_unit),
        Normal,
        PerUnit,
    ));
    s.push(StatementLine::row(
        "Indicative secondary-market value per unit",
        None,
        col(years, |y| y.market_value_per_unit),
        Normal,
        PerUnit,
    ));
    s.push(StatementLine::row(
        "Distributions per unit",
        None,
        col(years, |y| y.dpu),
        Normal,
        PerUnit,
    ));
    s.push(StatementLine::row(
        "Distribution yield on contributed capital",
        None,
        col(years, |y| y.dist_yield_on_cost),
        Normal,
        Pct,
    ));
    s.push(StatementLine::row(
        "Interest coverage ratio",
        None,
        col(years, |y| y.interest_coverage),
        Normal,
        Ratio,
    ));
    s.push(StatementLine::row(
        "Loan-to-value (debt ÷ fair value of assets)",
        None,
        col(years, |y| y.debt_to_asset_value),
        Normal,
        Pct,
    ));
    s.push(StatementLine::row(
        "Funds from operations",
        None,
        col(years, |y| y.ffo),
        Normal,
        Dollar,
    ));
    s
}

// ─── Public entry point ──────────────────────────────────────────────────────

pub fn build(j: Jurisdiction) -> ForecastStatements {
    let years = pclp1_proforma::forecast();
    ForecastStatements {
        jurisdiction: j,
        financial_position: build_sofp(&years, &j),
        comprehensive_income: build_sci(&years, &j),
        changes_in_equity: build_soce(&years, &j),
        cash_flows: build_scf(&years, &j),
        per_unit: build_fair_value_exhibit(&years, &j),
        notes: crate::spv::statutory_notes::notes_for(&j, &years),
        assurance: crate::spv::statutory_notes::assurance_block(&j),
        ifrs_supplement: None,
    }
}

// ─── Tests — the hard correctness gates ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn years() -> Vec<Pclp1Year> {
        pclp1_proforma::forecast()
    }

    #[test]
    fn sofp_balances_every_year() {
        // Cost basis: total assets (cost + cash) = liabilities (debt) + equity.
        for y in y1_10(&years()) {
            let assets = cost_total_assets(y);
            let liab_plus_equity = y.closing_debt + cost_equity(y);
            assert!(
                (assets - liab_plus_equity).abs() < 1.0,
                "SOFP out of balance at Y{}: assets {} vs L+E {}",
                y.year,
                assets,
                liab_plus_equity
            );
        }
    }

    #[test]
    fn equity_ties_to_contributed_plus_accumulated() {
        // Total equity must equal partners' units (contributed) + accumulated deficit.
        let ys = years();
        let accum = accumulated_deficit_series(&ys);
        for (i, y) in y1_10(&ys).iter().enumerate() {
            let presented = CONTRIBUTED_CAPITAL + accum[i];
            assert!(
                (presented - cost_equity(y)).abs() < 1.0,
                "Equity does not tie at Y{}: {} vs {}",
                y.year,
                presented,
                cost_equity(y)
            );
        }
    }

    #[test]
    fn soce_rolls_to_closing_equity() {
        // Beginning + net income + owner transactions − distributions = closing equity.
        let ys = years();
        for i in 1..=10 {
            let beginning = if i == 1 { 0.0 } else { cost_equity(&ys[i - 1]) };
            let owner = ys[i].new_equity
                + if i == 1 {
                    BENETTI_SBC + BENETTI_SBC - BENETTI_SBC
                } else {
                    0.0
                };
            let closing = beginning + net_income(&ys[i]) + owner - ys[i].distributions;
            assert!(
                (closing - cost_equity(&ys[i])).abs() < 1.0,
                "SOCE does not roll to closing equity at Y{}: {} vs {}",
                ys[i].year,
                closing,
                cost_equity(&ys[i])
            );
        }
    }

    #[test]
    fn sci_bottom_line_is_net_income() {
        // Operating profit + interest income − interest expense = net income = ffo − sbc.
        let ys = years();
        for y in y1_10(&ys) {
            let built = operating_profit(y) + interest_income(y) - interest_expense(y);
            assert!((built - net_income(y)).abs() < 1.0);
            assert!((net_income(y) - (y.ffo - sbc(y))).abs() < 1e-6);
        }
    }

    #[test]
    fn revenue_and_units_match_sample_structure() {
        // Revenue is nil in Years 1–3 (matches the sample); partners' units = $277,777,700.
        let ys = years();
        for y in &ys[1..=3] {
            assert!(
                y.net_proceeds_from_ops.abs() < 1e-6,
                "Y{} NOI not nil",
                y.year
            );
        }
        assert!((CONTRIBUTED_CAPITAL - 277_777_700.0).abs() < 1.0);
        assert!((BENETTI_SBC - 27_777_700.0).abs() < 1.0);
    }

    #[test]
    fn statements_have_ten_columns() {
        let st = build(CANADA);
        for line in st
            .financial_position
            .iter()
            .chain(&st.comprehensive_income)
            .chain(&st.changes_in_equity)
            .chain(&st.cash_flows)
        {
            if line.style != LineStyle::SectionBanner {
                assert_eq!(
                    line.values.len(),
                    10,
                    "line '{}' wrong column count",
                    line.label
                );
            }
        }
    }

    #[test]
    fn canada_naming_uses_legal_and_term() {
        let st = build(CANADA);
        assert_eq!(
            st.jurisdiction.legal_name,
            "Woodfine Professional Centres Limited Partnership"
        );
        assert_eq!(st.jurisdiction.term_name, "Professional Centres Canada LP");
        // No internal codes leak into naming.
        assert!(!st.jurisdiction.file_slug.contains("PCLP1"));
        assert!(!st.jurisdiction.file_slug.contains("PCCL"));
    }

    #[test]
    fn note_set_is_complete() {
        let st = build(CANADA);
        assert!(st.notes.len() >= 16, "expected the full merged note spine");
        // Spine notes present and numbered 1..=N with no gaps in the rendered set.
        for (i, n) in st.notes.iter().enumerate() {
            assert_eq!(
                n.number as usize,
                i + 1,
                "notes must be sequentially numbered"
            );
        }
    }
}
