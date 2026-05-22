use crate::base::*;
use crate::model::{Assumptions, ProformaOutput, YearOutput};

/// Compute the 10-year PCLP 1 proforma given a set of sensitivity assumptions.
///
/// Rust translation of `stress()` in `pclp1-sensitivity-v6.html`.
/// All rates in `assumptions` are fractions (0.0625 = 6.25%).
pub fn compute(a: &Assumptions) -> ProformaOutput {
    let occ_f = a.occupancy / BASE_OCC;
    let dev_f = a.dev_yield / BASE_DEV_YIELD;
    let int_f = a.debt_rate / BASE_DEBT_RATE;
    let lease_f = (BASE_LEASE_UP_MONTHS as f64 / a.lease_up_months as f64).min(1.0);

    // Operating expenses per year: NOI − EBITDA (pre-computed; constant across scenarios).
    let expenses: [f64; 10] = std::array::from_fn(|i| BASE_NOI[i] - BASE_EBITDA[i]);

    let mut years = Vec::with_capacity(10);

    for i in 0..10 {
        // Lease-up multiplier applies only in stabilisation years (Y4, Y6, Y8).
        let lf = if STAB_IDX.contains(&i) { lease_f } else { 1.0 };

        // NOI: zero in construction years (Y1–Y3); stressed in all others.
        let noi = if i < 3 { 0.0 } else { BASE_NOI[i] * occ_f * dev_f * lf };

        let ebitda = noi - expenses[i];

        // Interest: negative entries (interest income in early years) are not stressed.
        let interest = if BASE_INTEREST[i] <= 0.0 {
            BASE_INTEREST[i]
        } else {
            BASE_INTEREST[i] * int_f
        };

        // Distribution waterfall: 90% of after-interest cash; 10% to debt prepayment.
        let after_int = ebitda - interest;
        let debt_repay = if after_int > 0.0 && BASE_DEBT[i] > 0.0 {
            after_int * 0.10
        } else {
            0.0
        };
        let dist_per_unit = (after_int - debt_repay).max(0.0) / DILUTED;

        // Asset value = cash + capitalised NOI (at stressed cap rate) + WIP at cost.
        let cap_noi = if noi > 0.0 && a.cap_rate > 0.0 {
            noi / a.cap_rate
        } else {
            0.0
        };
        let asset_val = BASE_CASH[i] + cap_noi + BASE_WIP[i];

        // Y1–Y3: NAV and asset value held at proforma par (income continuity not stressed).
        let nav_per_unit = if i < 3 {
            BASE_NAV_PU[i]
        } else {
            (asset_val - BASE_DEBT[i]) / DILUTED
        };
        let asset_per_unit = if i < 3 { BASE_ASSET_PU[i] } else { asset_val / DILUTED };

        // DSCR = EBITDA ÷ interest (undefined when interest ≤ 0).
        let coverage = if interest > 0.0 { Some(ebitda / interest) } else { None };
        let debt_to_av = if asset_val > 0.0 { BASE_DEBT[i] / asset_val } else { 0.0 };

        // Market value: Y1–Y7 from fixed proforma schedule; Y8–Y10 = dist ÷ market_yield.
        let mv_per_unit = if i < 7 {
            BASE_MV_PU_FIXED[i]
        } else {
            if a.market_yield > 0.0 { dist_per_unit / a.market_yield } else { 0.0 }
        };

        years.push(YearOutput {
            year: i as u32 + 1,
            noi,
            ebitda,
            interest,
            dist_per_unit,
            asset_per_unit,
            nav_per_unit,
            mv_per_unit,
            coverage,
            debt_to_av,
        });
    }

    let total_dist_per_unit = years.iter().map(|y| y.dist_per_unit).sum();

    ProformaOutput {
        assumptions: a.clone(),
        years,
        total_dist_per_unit,
    }
}
