// Newton-Raphson XIRR for an annual cash-flow series (Y0..=Yn).
// Used in Legacy JV and PCLP 1 return comparators.

/// Compute the internal rate of return for an annually-spaced cash-flow series.
///
/// `cash_flows[0]` = Y0 (negative for an outflow).
/// Returns `None` if the series is trivially flat or if Newton-Raphson fails to
/// converge within 100 iterations.
pub fn xirr_annual(cash_flows: &[f64]) -> Option<f64> {
    let mut rate = 0.10_f64;
    for _ in 0..100 {
        let npv: f64 = cash_flows
            .iter()
            .enumerate()
            .map(|(t, cf)| cf / (1.0 + rate).powi(t as i32))
            .sum();
        let dnpv: f64 = cash_flows
            .iter()
            .enumerate()
            .map(|(t, cf)| -(t as f64) * cf / (1.0 + rate).powi(t as i32 + 1))
            .sum();
        if dnpv.abs() < 1e-10 {
            break;
        }
        let next = rate - npv / dnpv;
        if (next - rate).abs() < 1e-8 {
            return Some(next);
        }
        rate = next;
    }
    Some(rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_known_irr() {
        // $1,000 invested; $1,100 back in 1 year → IRR = 10%
        let cfs = [-1000.0_f64, 1100.0_f64];
        let irr = xirr_annual(&cfs).unwrap();
        assert!((irr - 0.10).abs() < 0.0001, "IRR = {irr:.4} (expected 0.10)");
    }

    #[test]
    fn two_year_series() {
        // $1,000 invested; $0 in Y1; $1,210 in Y2 → IRR = 10%
        let cfs = [-1000.0_f64, 0.0, 1210.0_f64];
        let irr = xirr_annual(&cfs).unwrap();
        assert!((irr - 0.10).abs() < 0.0001, "IRR = {irr:.4} (expected 0.10)");
    }

    #[test]
    fn legacy_jv_ballpark() {
        // Approximate Legacy JV cash-flow series: −$250M at Y0, $0 Y1–Y3,
        // ~$33M/yr Y4–Y10, +$510M terminal at Y10 (IFRS FV).
        // Expected IRR ~12–14%.
        let mut cfs = vec![-250_000_000.0_f64; 1];
        cfs.extend([0.0, 0.0, 0.0]);
        for _ in 4..=9 {
            cfs.push(32_970_000.0);
        }
        cfs.push(32_970_000.0 + 510_000_000.0); // Y10: dividend + FV terminal
        assert_eq!(cfs.len(), 11);
        let irr = xirr_annual(&cfs).unwrap();
        assert!(
            irr > 0.10 && irr < 0.16,
            "Legacy JV IRR = {irr:.4} (expected 10–16% range)"
        );
    }
}
