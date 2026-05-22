use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

use super::{get_f64, get_str};

/// One rentable component (retail or office floor).
#[derive(Debug, Clone)]
pub struct TitlecoArea {
    pub label: String,
    pub sqft: f64,
    pub rate_per_sqft: f64,
    pub annual_rent: f64,
}

/// Full static development proforma extracted from TitleCo 3.
/// Provides the per-sqft cost and rent assumptions used by the D1 engine.
#[derive(Debug, Clone)]
pub struct TitlecoProforma {
    pub title: String,
    pub entity: String,
    pub date: String,
    // Rental areas (all non-zero components)
    pub areas: Vec<TitlecoArea>,
    pub total_sqft: f64,
    pub total_projected_revenue: f64,
    // Per-sqft cost rates (from construction section)
    pub construction_rate: f64,   // $/sqft gross floor area
    pub contingency_rate: f64,    // fraction of construction cost (e.g. 0.05)
    pub ti_rate: f64,             // $/sqft leasable
    // Cost totals (for reference / validation)
    pub land_cost: f64,
    pub construction_cost: f64,
    pub contingency_cost: f64,
    pub other_construction_cost: f64,
    pub ti_cost: f64,
    pub professional_fees: f64,
    pub marketing_leasing: f64,
    pub total_cost: f64,
    pub total_cost_per_sqft: f64,
    pub equity: f64,
    // Performance measures
    pub profit_on_cost: f64,
    pub profit_on_gdv: f64,
    pub development_yield: f64,
    pub net_initial_yield: f64,   // cap rate used in capitalization
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<TitlecoProforma, Box<dyn std::error::Error>> {
    let mut wb: Xlsx<_> = open_workbook(path)?;
    let range = wb.worksheet_range("Test Site_Proforma")?;

    // NOTE: the sheet's used range starts at absolute (row 1, col 1).
    // get_value() takes absolute 0-indexed coordinates.
    // The discover tool displays row numbers equal to the absolute row index.
    // Column labels in discover: [A]=absolute 1, [B]=2, [C]=3, [D]=4, [E]=5, [F]=6, [G]=7, [H]=8, [I]=9, [J]=10
    //
    // So: r(N) = N directly (no -1); all col indices shifted +1 vs. naive assignment.
    let r = |row: u32| row;

    // Rental area section column layout: [F]=sqft(6), [G]=rate(7), [H]=annual-rent(8)
    // Construction section column layout: [E]=sqft(5), [F]=rate(6), [G]=cost(7)
    // Performance/totals column: [H]=8
    let col_sqft = 6u32;   // [F]
    let col_rate = 7u32;   // [G]
    let col_h    = 8u32;   // [H]

    // Header: col [A] = absolute col 1
    let title  = get_str(&range, r(1), 1);
    let entity = get_str(&range, r(2), 1);
    let date   = get_str(&range, r(4), 1);

    // Rental area rows 11–18 (Underground through Office Floor 6)
    let area_defs: &[(u32, &str)] = &[
        (11, "Underground"),
        (12, "Retail"),
        (13, "Office — Floor 1"),
        (14, "Office — Floor 2"),
        (15, "Office — Floor 3"),
        (16, "Office — Floor 4"),
        (17, "Office — Floor 5"),
        (18, "Office — Floor 6"),
    ];
    let mut areas = Vec::new();
    for &(row, label) in area_defs {
        let sqft = get_f64(&range, r(row), col_sqft);
        if sqft == 0.0 {
            continue;
        }
        let rate = get_f64(&range, r(row), col_rate);
        let rent = get_f64(&range, r(row), col_h);
        areas.push(TitlecoArea {
            label: label.to_string(),
            sqft,
            rate_per_sqft: rate,
            annual_rent: rent,
        });
    }

    let total_sqft             = get_f64(&range, r(19), col_sqft);  // row 19 totals
    let total_projected_revenue= get_f64(&range, r(35), col_h);    // Total Projected Revenue

    // Construction section: col [E]=sqft(5), col [F]=rate(6), col [G]=cost(7)
    let construction_rate      = get_f64(&range, r(45), 6);         // [F] = rate/sqft (not col_rate which is [G])
    let contingency_rate       = get_f64(&range, r(54), 6);         // [F] = 0.05 fraction
    let ti_rate                = get_f64(&range, r(63), 6);         // [F] = TI cost/sqft

    let land_cost              = get_f64(&range, r(41), col_h);
    let construction_cost      = get_f64(&range, r(52), col_h);
    let contingency_cost       = get_f64(&range, r(55), col_h);
    let other_construction_cost= get_f64(&range, r(59), col_h);
    let ti_cost                = get_f64(&range, r(70), col_h);
    let professional_fees      = get_f64(&range, r(80), col_h);
    let marketing_leasing      = get_f64(&range, r(87), col_h);
    let total_cost             = get_f64(&range, r(89), col_h);
    let total_cost_per_sqft    = get_f64(&range, r(90), col_h);
    let equity                 = get_f64(&range, r(93), col_h);

    // Performance measures
    let profit_on_cost    = get_f64(&range, r(99),  col_h);
    let profit_on_gdv     = get_f64(&range, r(100), col_h);
    let development_yield = get_f64(&range, r(101), col_h);
    let net_initial_yield = get_f64(&range, r(102), col_h);

    Ok(TitlecoProforma {
        title,
        entity,
        date,
        areas,
        total_sqft,
        total_projected_revenue,
        construction_rate,
        contingency_rate,
        ti_rate,
        land_cost,
        construction_cost,
        contingency_cost,
        other_construction_cost,
        ti_cost,
        professional_fees,
        marketing_leasing,
        total_cost,
        total_cost_per_sqft,
        equity,
        profit_on_cost,
        profit_on_gdv,
        development_yield,
        net_initial_yield,
    })
}
