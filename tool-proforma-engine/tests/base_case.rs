use proforma_engine::{base::scenarios, compute};

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

/// Assert base-case Y8 values match the known-good values from pclp1-sensitivity-v6.html.
#[test]
fn base_case_y8() {
    let output = compute(&scenarios::base());
    let y8 = &output.years[7]; // index 7 = Y8

    assert!(
        approx_eq(y8.nav_per_unit, 380.349, 0.01),
        "Y8 NAV/unit: expected ≈380.349, got {:.4}",
        y8.nav_per_unit
    );
    assert!(
        approx_eq(y8.asset_per_unit, 733.075, 0.01),
        "Y8 asset/unit: expected ≈733.075, got {:.4}",
        y8.asset_per_unit
    );
    assert!(
        approx_eq(y8.dist_per_unit, 24.092, 0.01),
        "Y8 dist/unit: expected ≈24.092, got {:.4}",
        y8.dist_per_unit
    );
    assert!(
        approx_eq(y8.mv_per_unit, 301.15, 0.02),
        "Y8 MV/unit: expected ≈301.15, got {:.4}",
        y8.mv_per_unit
    );
    assert!(
        approx_eq(y8.coverage.unwrap(), 2.506, 0.002),
        "Y8 DSCR: expected ≈2.506, got {:.4}",
        y8.coverage.unwrap()
    );
}

/// Y1–Y3 should be at par (income continuity; no sensitivity stress applied).
#[test]
fn base_case_y1_y3_at_par() {
    let output = compute(&scenarios::base());

    for i in 0..3 {
        let y = &output.years[i];
        assert_eq!(y.noi, 0.0, "Y{} NOI should be zero", i + 1);
        assert!(
            approx_eq(y.nav_per_unit, 100.0, 0.3),
            "Y{} NAV/unit should be near par, got {:.4}",
            i + 1,
            y.nav_per_unit
        );
    }
}

/// Y7 has the tightest DSCR in the base case (1.31×).
#[test]
fn base_case_y7_min_dscr() {
    let output = compute(&scenarios::base());
    let y7 = &output.years[6]; // index 6 = Y7

    let dscr = y7.coverage.expect("Y7 should have positive interest");
    assert!(
        approx_eq(dscr, 1.313, 0.005),
        "Y7 DSCR: expected ≈1.313, got {:.4}",
        dscr
    );
}

/// Bear scenario: higher cap rate, lower occupancy, longer lease-up → lower NAV and distributions.
#[test]
fn bear_produces_lower_nav_than_base() {
    let base_out = compute(&scenarios::base());
    let bear_out = compute(&scenarios::bear());

    let base_nav = base_out.years[7].nav_per_unit;
    let bear_nav = bear_out.years[7].nav_per_unit;

    assert!(
        bear_nav < base_nav,
        "Bear Y8 NAV/unit ({:.2}) should be less than base ({:.2})",
        bear_nav,
        base_nav
    );
}

/// Bull scenario: lower cap rate, full occupancy, fast lease-up → higher NAV than base.
#[test]
fn bull_produces_higher_nav_than_base() {
    let base_out = compute(&scenarios::base());
    let bull_out = compute(&scenarios::bull());

    let base_nav = base_out.years[7].nav_per_unit;
    let bull_nav = bull_out.years[7].nav_per_unit;

    assert!(
        bull_nav > base_nav,
        "Bull Y8 NAV/unit ({:.2}) should be greater than base ({:.2})",
        bull_nav,
        base_nav
    );
}

/// Output must have exactly 10 years.
#[test]
fn output_has_ten_years() {
    let output = compute(&scenarios::base());
    assert_eq!(output.years.len(), 10);
    assert_eq!(output.years[0].year, 1);
    assert_eq!(output.years[9].year, 10);
}

/// total_dist_per_unit should equal sum of annual dist_per_unit values.
#[test]
fn total_dist_consistency() {
    let output = compute(&scenarios::base());
    let manual_sum: f64 = output.years.iter().map(|y| y.dist_per_unit).sum();
    assert!(
        approx_eq(output.total_dist_per_unit, manual_sum, 1e-9),
        "total_dist_per_unit mismatch"
    );
}
