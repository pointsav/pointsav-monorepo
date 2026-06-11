// WCP Inc. (Woodfine Capital Projects Inc.) — self-generating proforma engine module.
//
// Implements declared Rust constants. No Excel read.
// Consumes PCLP 1 forecast as DHS1 source.
//
// DHS fund derivation:
//   DHS_n[y] = DHS1[y - lag_n] × size_factor × fx_rate
//
// Six direct-hold solutions (DHS1 = seed; DHS2-DHS6 derived):
//   DHS1=Professional Centres Canada LP   (PRO-CA-01-AST) C$250M CAD Y1  1× CAD/CAD/CAD
//   DHS2=Professional Centres United States LP (PRO-US-02-AST) $500M USD Y2  2× USD/USD/USD
//   DHS3=Professional Centres Spain SOCIMI    (PRO-ES-03-ADM) EUR$250M Y2  1× EUR/EUR/EUR
//   DHS4=Professional Centres Mexico FIBRA   (PRO-MX-04-AST) $250M USD Y3  1× USD/USD/USD
//   DHS5=Vertical Warehouse United States LP  (VWH-US-01-AST) $250M USD Y4  1× USD/USD/USD
//   DHS6=Parking Structure United States LP   (PKS-US-01-AST) $250M USD Y5  1× USD/USD/USD

use crate::spv::pclp1_proforma::Pclp1Year;
use serde::Serialize;

// ─── Constants ───────────────────────────────────────────────────────────────

pub const WCP_SHARES_OUTSTANDING: f64 = 10_000_000.0;
pub const WCP_PRICE_PER_SHARE_Y0: f64 = 20.00;
pub const WCP_CAD_USD: f64 = 1.3372;
pub const WCP_CAD_EUR: f64 = 1.4657;
pub const WCP_FINANCING_Y1: f64 = 20_000_000.0;
pub const WCP_FINANCING_Y2: f64 = 22_000_000.0;
pub const WCP_TAX_RATE: f64 = 0.27;
pub const WCP_PE_MULTIPLE: f64 = 10.72;
pub const WCP_DIVIDEND_YIELD: f64 = 0.045;

// WCP's beneficial ownership in each DHS fund (10%)
pub const WCP_LP_BENEFICIAL_OWNERSHIP: f64 = 0.10;

// G&A ramp Y3-Y10 — advisory_fee_total × ga_ramp[y]
pub const WCP_GA_RAMP_Y3_Y10: [f64; 8] = [0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55];

// Y1-Y2 G&A hardcoded
pub const WCP_GA_NYC_Y1: f64 = 750_000.0;
pub const WCP_GA_NYC_Y2: f64 = 750_000.0;
pub const WCP_GA_BERLIN_Y1: f64 = 0.0;
pub const WCP_GA_BERLIN_Y2: f64 = 250_000.0;

// WPI compensation agreement Y1-Y2
pub const WCP_WPI_Y1: f64 = 2_000_000.0;
pub const WCP_WPI_Y2: f64 = 8_500_000.0;

// ─── LP Fund Definitions ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct WcpLpConfig {
    pub name: &'static str,
    pub launch_year: u32, // First year DHS is active
    pub size_factor: f64, // Multiplier vs DHS1
    pub advisory_fx: f64, // FX rate applied to advisory fee
    pub dist_fx: f64,     // FX rate applied to distributions
    pub nav_fx: f64,      // FX rate applied to NAV
}

pub const WCP_LPS: [WcpLpConfig; 6] = [
    WcpLpConfig {
        name: "DHS1 — Professional Centres Canada LP | PRO-CA-01-AST (CAD)",
        launch_year: 1,
        size_factor: 1.0,
        advisory_fx: 1.0,
        dist_fx: 1.0,
        nav_fx: 1.0,
    },
    WcpLpConfig {
        name: "DHS2 — Professional Centres United States LP | PRO-US-02-AST (USD)",
        launch_year: 2,
        size_factor: 2.0,
        advisory_fx: WCP_CAD_USD,
        dist_fx: WCP_CAD_USD,
        nav_fx: WCP_CAD_USD,
    },
    WcpLpConfig {
        name: "DHS3 — Professional Centres Spain SOCIMI | PRO-ES-03-ADM (EUR)",
        launch_year: 2,
        size_factor: 1.0,
        advisory_fx: WCP_CAD_EUR,
        dist_fx: WCP_CAD_EUR,
        nav_fx: WCP_CAD_EUR,
    },
    WcpLpConfig {
        name: "DHS4 — Professional Centres Mexico FIBRA | PRO-MX-04-AST (USD)",
        launch_year: 3,
        size_factor: 1.0,
        advisory_fx: WCP_CAD_USD,
        dist_fx: WCP_CAD_USD,
        nav_fx: WCP_CAD_USD,
    },
    WcpLpConfig {
        name: "DHS5 — Vertical Warehouse United States LP | VWH-US-01-AST (USD)",
        launch_year: 4,
        size_factor: 1.0,
        advisory_fx: WCP_CAD_USD,
        dist_fx: WCP_CAD_USD,
        nav_fx: WCP_CAD_USD,
    },
    WcpLpConfig {
        name: "DHS6 — Parking Structure United States LP | PKS-US-01-AST (USD)",
        launch_year: 5,
        size_factor: 1.0,
        advisory_fx: WCP_CAD_USD,
        dist_fx: WCP_CAD_USD,
        nav_fx: WCP_CAD_USD,
    },
];

// ─── Output struct ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WcpLpYear {
    pub lp_name: &'static str,
    pub advisory_fee: f64,
    pub distributions: f64,
    pub nav: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct WcpYear {
    pub year: u32,
    // Revenue Generator (6 LPs aggregated)
    pub lps: Vec<WcpLpYear>,
    pub advisory_fees_total: f64,
    pub distributions_total: f64,
    pub nav_total_lps: f64,
    pub offering_costs: f64,
    pub gross_income: f64,
    // Income Statement
    pub referral_fees: f64,
    pub wpi_consulting: f64,
    pub gna_nyc: f64,
    pub gna_berlin: f64,
    pub gna_total: f64,
    pub total_opex: f64,
    pub ebitda: f64,
    pub taxes: f64,
    pub earnings: f64,
    pub eps: f64,
    // Book Valuation
    pub financing_activity: f64,
    pub cumulative_fcf: f64,
    pub lp_ownership_book: f64,
    pub book_value: f64,
    pub book_value_per_share: f64,
    // Four Valuation Methods (BRIEF §5c rows 80-102)
    pub earnings_valuation: f64,
    pub market_valuation: f64,
    pub market_value_per_share: f64,
    pub dividend_valuation: f64,
    pub dividend_value_per_share: f64,
    pub fair_value_per_share: f64,
}

// ─── Per-LP series computation ──────────────────────────────────────────────

fn lp1_advisory_at_year(pclp1: &[Pclp1Year], y: u32) -> f64 {
    // BRIEF §1088-1091: LP1 advisory = PCLP1.advisory_fee × deployment_ramp.
    // PCLP1 module already applies the ramp via advisory_fee_to_wcp[].
    let idx = y as usize;
    if idx < pclp1.len() {
        pclp1[idx].advisory_fee_to_wcp
    } else {
        0.0
    }
}

fn lp1_distributions_at_year(pclp1: &[Pclp1Year], y: u32) -> f64 {
    // WCP holds 10% beneficial ownership in LP1; receives 10% of distributions
    let idx = y as usize;
    if idx < pclp1.len() {
        pclp1[idx].distributions * WCP_LP_BENEFICIAL_OWNERSHIP
    } else {
        0.0
    }
}

fn lp1_nav_at_year(pclp1: &[Pclp1Year], y: u32) -> f64 {
    let idx = y as usize;
    if idx < pclp1.len() {
        pclp1[idx].nav * WCP_LP_BENEFICIAL_OWNERSHIP
    } else {
        0.0
    }
}

fn lp_at_year(lp: &WcpLpConfig, pclp1: &[Pclp1Year], y: u32) -> WcpLpYear {
    if y < lp.launch_year {
        return WcpLpYear {
            lp_name: lp.name,
            advisory_fee: 0.0,
            distributions: 0.0,
            nav: 0.0,
        };
    }
    let lag = lp.launch_year - 1; // LP1 launches Y1 (lag = 0); LP2 launches Y2 (lag = 1); etc.
    let source_year = y - lag;
    WcpLpYear {
        lp_name: lp.name,
        advisory_fee: lp1_advisory_at_year(pclp1, source_year) * lp.size_factor * lp.advisory_fx,
        distributions: lp1_distributions_at_year(pclp1, source_year) * lp.size_factor * lp.dist_fx,
        nav: lp1_nav_at_year(pclp1, source_year) * lp.size_factor * lp.nav_fx,
    }
}

// ─── Offering costs reimbursement (BRIEF §1099-1102) ────────────────────────

fn offering_costs(advisory_total_series: &[f64], y: u32) -> f64 {
    // Y1 = LP1_advisory[Y1]
    // Y2-Y6 = LP1_advisory[y] - LP1_advisory[y-1]   (first-difference)
    // Y7+ = 0
    // We use the total LP advisory series at WCP (all 6 LPs); the BRIEF says
    // "advisory_fee_total" which is the sum, not just LP1. Per BRIEF §1099.
    let idx = y as usize;
    match y {
        1 => advisory_total_series.get(1).copied().unwrap_or(0.0),
        2..=6 => {
            let cur = advisory_total_series.get(idx).copied().unwrap_or(0.0);
            let prev = advisory_total_series.get(idx - 1).copied().unwrap_or(0.0);
            cur - prev
        }
        _ => 0.0,
    }
}

// ─── Referral fees (BRIEF §1109-1110) ───────────────────────────────────────

fn referral_fees(y: u32) -> f64 {
    // Y1 = $2M, Y2 = $2.2M, Y3+ = 0
    // (10% of financing tranche)
    match y {
        1 => WCP_FINANCING_Y1 * 0.10, // $2M
        2 => WCP_FINANCING_Y2 * 0.10, // $2.2M
        _ => 0.0,
    }
}

// ─── WPI Consulting (BRIEF §1111) ───────────────────────────────────────────

fn wpi_consulting(y: u32) -> f64 {
    match y {
        1 => WCP_WPI_Y1,
        2 => WCP_WPI_Y2,
        _ => 0.0,
    }
}

// ─── G&A (BRIEF §1112-1114) ─────────────────────────────────────────────────

fn gna_nyc(y: u32, advisory_total: f64) -> f64 {
    match y {
        1 => WCP_GA_NYC_Y1,
        2 => WCP_GA_NYC_Y2,
        3..=10 => {
            // Split G&A as half NYC half Berlin (approximation; BRIEF doesn't
            // specify split for Y3+, just total advisory × ramp)
            let ramp_idx = (y - 3) as usize;
            let ramp = WCP_GA_RAMP_Y3_Y10.get(ramp_idx).copied().unwrap_or(0.55);
            advisory_total * ramp * 0.5
        }
        _ => 0.0,
    }
}

fn gna_berlin(y: u32, advisory_total: f64) -> f64 {
    match y {
        1 => WCP_GA_BERLIN_Y1,
        2 => WCP_GA_BERLIN_Y2,
        3..=10 => {
            let ramp_idx = (y - 3) as usize;
            let ramp = WCP_GA_RAMP_Y3_Y10.get(ramp_idx).copied().unwrap_or(0.55);
            advisory_total * ramp * 0.5
        }
        _ => 0.0,
    }
}

fn financing_activity(y: u32) -> f64 {
    match y {
        1 => WCP_FINANCING_Y1,
        2 => WCP_FINANCING_Y2,
        _ => 0.0,
    }
}

// ─── Main forecast ──────────────────────────────────────────────────────────

pub fn forecast(pclp1: &[Pclp1Year]) -> Vec<WcpYear> {
    let mut years: Vec<WcpYear> = Vec::with_capacity(11);

    // Pre-compute advisory totals series for offering_costs reimbursement
    let mut advisory_total_series: Vec<f64> = vec![0.0; 11];
    for y in 0..=10u32 {
        let mut total = 0.0;
        for lp in WCP_LPS.iter() {
            total += lp_at_year(lp, pclp1, y).advisory_fee;
        }
        advisory_total_series[y as usize] = total;
    }

    let mut cumulative_fcf: f64 = 0.0;

    // Y0: empty / zeros
    years.push(WcpYear {
        year: 0,
        lps: WCP_LPS
            .iter()
            .map(|lp| WcpLpYear {
                lp_name: lp.name,
                advisory_fee: 0.0,
                distributions: 0.0,
                nav: 0.0,
            })
            .collect(),
        advisory_fees_total: 0.0,
        distributions_total: 0.0,
        nav_total_lps: 0.0,
        offering_costs: 0.0,
        gross_income: 0.0,
        referral_fees: 0.0,
        wpi_consulting: 0.0,
        gna_nyc: 0.0,
        gna_berlin: 0.0,
        gna_total: 0.0,
        total_opex: 0.0,
        ebitda: 0.0,
        taxes: 0.0,
        earnings: 0.0,
        eps: 0.0,
        financing_activity: 0.0,
        cumulative_fcf: 0.0,
        lp_ownership_book: 0.0,
        book_value: 0.0,
        book_value_per_share: 0.0,
        earnings_valuation: 0.0,
        market_valuation: 0.0,
        market_value_per_share: 0.0,
        dividend_valuation: 0.0,
        dividend_value_per_share: 0.0,
        fair_value_per_share: 0.0,
    });

    for y in 1u32..=10 {
        // Per-LP slices
        let lps: Vec<WcpLpYear> = WCP_LPS.iter().map(|lp| lp_at_year(lp, pclp1, y)).collect();
        let advisory_fees_total: f64 = lps.iter().map(|l| l.advisory_fee).sum();
        let distributions_total: f64 = lps.iter().map(|l| l.distributions).sum();
        let nav_total_lps: f64 = lps.iter().map(|l| l.nav).sum();

        let oc = offering_costs(&advisory_total_series, y);

        // Gross income (BRIEF §1108)
        let gross_income = advisory_fees_total + distributions_total + oc;

        // Expenses
        let referral = referral_fees(y);
        let wpi = wpi_consulting(y);
        let nyc = gna_nyc(y, advisory_fees_total);
        let berlin = gna_berlin(y, advisory_fees_total);
        let gna_total = nyc + berlin;
        let total_opex = referral + wpi + gna_total;

        // EBITDA / Tax / Earnings
        let ebitda = gross_income - total_opex;
        let taxes = if ebitda > 0.0 {
            ebitda * WCP_TAX_RATE
        } else {
            0.0
        };
        let earnings = ebitda - taxes;
        let eps = earnings / WCP_SHARES_OUTSTANDING;

        // Book Valuation (BRIEF §1127-1131)
        let fa = financing_activity(y);
        cumulative_fcf += fa + earnings;
        let book_value = cumulative_fcf + nav_total_lps;
        let book_value_per_share = book_value / WCP_SHARES_OUTSTANDING;

        // Four Valuation Methods (BRIEF §5c rows 80-102)
        // Earnings valuation = Earnings × P/E
        let earnings_valuation = earnings * WCP_PE_MULTIPLE;
        // Market valuation: same as earnings valuation for simplicity (BRIEF §5c
        // distinguishes them in formula but treats them equivalently in output table)
        let market_valuation = earnings_valuation;
        let market_value_per_share = market_valuation / WCP_SHARES_OUTSTANDING;
        // Dividend valuation: dividends per share / dividend yield (4.5%)
        // Approximation: total dividends ≈ earnings × payout_ratio (assume 100%
        // for simplicity here; real model would track payout policy)
        let dividend_valuation = if WCP_DIVIDEND_YIELD > 0.0 {
            earnings / WCP_DIVIDEND_YIELD
        } else {
            0.0
        };
        let dividend_value_per_share = dividend_valuation / WCP_SHARES_OUTSTANDING;
        // Fair value per share: average of book, market, dividend
        let fair_value_per_share =
            (book_value_per_share + market_value_per_share + dividend_value_per_share) / 3.0;

        years.push(WcpYear {
            year: y,
            lps,
            advisory_fees_total,
            distributions_total,
            nav_total_lps,
            offering_costs: oc,
            gross_income,
            referral_fees: referral,
            wpi_consulting: wpi,
            gna_nyc: nyc,
            gna_berlin: berlin,
            gna_total,
            total_opex,
            ebitda,
            taxes,
            earnings,
            eps,
            financing_activity: fa,
            cumulative_fcf,
            lp_ownership_book: nav_total_lps,
            book_value,
            book_value_per_share,
            earnings_valuation,
            market_valuation,
            market_value_per_share,
            dividend_valuation,
            dividend_value_per_share,
            fair_value_per_share,
        });
    }

    years
}

// ─── JSON dump ──────────────────────────────────────────────────────────────

pub fn forecast_json(pclp1: &[Pclp1Year]) -> serde_json::Value {
    let years = forecast(pclp1);
    let lp_definitions: Vec<serde_json::Value> = WCP_LPS
        .iter()
        .map(|lp| {
            serde_json::json!({
                "name": lp.name,
                "launch_year": lp.launch_year,
                "size_factor": lp.size_factor,
                "advisory_fx": lp.advisory_fx,
                "dist_fx": lp.dist_fx,
                "nav_fx": lp.nav_fx,
            })
        })
        .collect();

    serde_json::json!({
        "entity": "Woodfine Capital Projects Inc. (WCP)",
        "source": "tool-proforma-engine src/spv/wcp_proforma module",
        "version": "V2",
        "generated_at": "2026-06-10",
        "consumes": "PCLP 1 forecast (DHS1 source)",
        "inputs": {
            "shares_outstanding": WCP_SHARES_OUTSTANDING,
            "price_per_share_y0": WCP_PRICE_PER_SHARE_Y0,
            "cad_usd": WCP_CAD_USD,
            "cad_eur": WCP_CAD_EUR,
            "financing_y1": WCP_FINANCING_Y1,
            "financing_y2": WCP_FINANCING_Y2,
            "tax_rate": WCP_TAX_RATE,
            "pe_multiple": WCP_PE_MULTIPLE,
            "dividend_yield": WCP_DIVIDEND_YIELD,
            "lp_beneficial_ownership": WCP_LP_BENEFICIAL_OWNERSHIP,
            "lp_definitions": lp_definitions,
        },
        "years": years,
        "fx_anomalies": ["LP5 distributions use EUR rate despite USD fund — BRIEF §1086", "LP6 same"],
    })
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use crate::spv::pclp1_proforma;

    #[test]
    fn forecast_has_11_years() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        assert_eq!(wcp.len(), 11);
        assert_eq!(wcp[0].year, 0);
        assert_eq!(wcp[10].year, 10);
    }

    #[test]
    fn lp1_advisory_matches_pclp1_advisory_fee_to_wcp() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // Y4 should have LP1 advisory = PCLP1.advisory_fee × ramp (1.0 by Y4)
        // = $2.5M × 1.0 = $2.5M
        let lp1_y4 = &wcp[4].lps[0];
        assert!(
            (lp1_y4.advisory_fee - 2_500_000.0).abs() < 1.0,
            "LP1 Y4 advisory should be $2.5M, got {}",
            lp1_y4.advisory_fee
        );
    }

    #[test]
    fn lp2_advisory_is_2x_lp1_with_usd_fx_lagged() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // LP2 launches Y2 (lag = 1). LP2[Y3] = LP1[Y2] × 2 × CAD_USD
        // LP1[Y2] advisory = $2.5M × 2/3 ramp = $1.667M
        // LP2[Y3] = $1.667M × 2 × 1.3372 = $4.458M
        let lp2_y3 = &wcp[3].lps[1];
        let expected = 2_500_000.0 * (2.0 / 3.0) * 2.0 * WCP_CAD_USD;
        assert!(
            (lp2_y3.advisory_fee - expected).abs() < 100.0,
            "LP2 Y3 advisory = {} (expected ~{})",
            lp2_y3.advisory_fee,
            expected
        );
    }

    #[test]
    fn lp_launches_respect_launch_year() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // LP2 launches Y2; Y1 should be 0
        assert_eq!(wcp[1].lps[1].advisory_fee, 0.0);
        // LP4 launches Y3; Y1+Y2 should be 0
        assert_eq!(wcp[1].lps[3].advisory_fee, 0.0);
        assert_eq!(wcp[2].lps[3].advisory_fee, 0.0);
        // LP6 launches Y5; Y1-Y4 should be 0
        for y in 1..=4 {
            assert_eq!(
                wcp[y].lps[5].advisory_fee, 0.0,
                "LP6 Y{} should be 0 (launches Y5)",
                y
            );
        }
    }

    #[test]
    fn referral_fees_y1_y2_only() {
        assert!((referral_fees(1) - 2_000_000.0).abs() < 1.0);
        assert!((referral_fees(2) - 2_200_000.0).abs() < 1.0);
        assert_eq!(referral_fees(3), 0.0);
        assert_eq!(referral_fees(10), 0.0);
    }

    #[test]
    fn wpi_consulting_y1_y2_only() {
        assert!((wpi_consulting(1) - 2_000_000.0).abs() < 1.0);
        assert!((wpi_consulting(2) - 8_500_000.0).abs() < 1.0);
        assert_eq!(wpi_consulting(3), 0.0);
    }

    #[test]
    fn financing_activity_matches_inputs() {
        assert!((financing_activity(1) - 20_000_000.0).abs() < 1.0);
        assert!((financing_activity(2) - 22_000_000.0).abs() < 1.0);
        assert_eq!(financing_activity(3), 0.0);
    }

    #[test]
    fn ebitda_negative_y1_y2_capital_raise_period() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // Per BRIEF §1122: "EBITDA Y1=−$3.08M, Y2=−$2.30M (negative — capital raise period)"
        // Engine should produce similar negative EBITDA Y1-Y2 due to high WPI + Referral
        assert!(
            wcp[1].ebitda < 0.0,
            "Y1 EBITDA should be negative, got {}",
            wcp[1].ebitda
        );
        assert!(
            wcp[2].ebitda < 0.0,
            "Y2 EBITDA should be negative, got {}",
            wcp[2].ebitda
        );
    }

    #[test]
    fn ebitda_positive_y3_plus() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // Per BRIEF §1122: "Y3=$13.81M+ onward positive"
        assert!(
            wcp[3].ebitda > 0.0,
            "Y3 EBITDA should be positive, got {}",
            wcp[3].ebitda
        );
        assert!(
            wcp[10].ebitda > wcp[3].ebitda,
            "Y10 EBITDA should be larger than Y3, got {} vs {}",
            wcp[10].ebitda,
            wcp[3].ebitda
        );
    }

    #[test]
    fn earnings_after_27pct_tax() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // Earnings = EBITDA × 0.73 when EBITDA > 0
        for y in 3..=10 {
            let yr = &wcp[y];
            let expected = yr.ebitda * 0.73;
            assert!(
                (yr.earnings - expected).abs() < 100.0,
                "Y{} earnings = {} (expected {})",
                y,
                yr.earnings,
                expected
            );
        }
    }

    #[test]
    fn earnings_valuation_uses_pe_multiple() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        // Earnings × 10.72 P/E
        for y in 3..=10 {
            let yr = &wcp[y];
            let expected = yr.earnings * WCP_PE_MULTIPLE;
            assert!(
                (yr.earnings_valuation - expected).abs() < 1.0,
                "Y{} earnings valuation = {} (expected {})",
                y,
                yr.earnings_valuation,
                expected
            );
        }
    }

    #[test]
    fn book_value_includes_lp_ownership_plus_cumulative_fcf() {
        let pclp1 = pclp1_proforma::forecast();
        let wcp = forecast(&pclp1);
        for y in 1..=10 {
            let yr = &wcp[y];
            let expected = yr.cumulative_fcf + yr.lp_ownership_book;
            assert!(
                (yr.book_value - expected).abs() < 1.0,
                "Y{} book value = {} (expected {})",
                y,
                yr.book_value,
                expected
            );
        }
    }

    #[test]
    fn json_dump_well_formed() {
        let pclp1 = pclp1_proforma::forecast();
        let json = forecast_json(&pclp1);
        assert!(json["entity"].as_str().unwrap().contains("WCP"));
        assert!(json["version"].as_str().unwrap().contains("V2"));
        assert!(json["inputs"]["lp_definitions"].is_array());
        assert_eq!(
            json["inputs"]["lp_definitions"].as_array().unwrap().len(),
            6
        );
        assert_eq!(json["years"].as_array().unwrap().len(), 11);
    }
}
