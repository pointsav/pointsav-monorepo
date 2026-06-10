// WCP $500,000 Common Share Allocation — individual proforma (JW1).
// $500K loan to Woodfine Properties Inc. at 7% p.a., 18-month term.
// 25,000 WCP shares at $20 + 75,000 bonus (4:1 ratio) = 100,000 total (1% of 10M).
// Y3 Capital Recovery Sale: sell 25K allocated shares at WCP engine book_value_per_share.
// Consumes WCP V1 forecast for Y3+ per-share valuation.

use crate::spv::wcp_proforma::WcpYear;
use serde::Serialize;

// ─── Constants ───────────────────────────────────────────────────────────────

pub const JW1_LOAN_PRINCIPAL: f64 = 500_000.0;
pub const JW1_LOAN_RATE: f64 = 0.07;
pub const JW1_WCP_ALLOCATED_SHARES: f64 = 25_000.0;
pub const JW1_WCP_PURCHASE_PRICE: f64 = 20.0;
pub const JW1_WCP_BONUS_SHARES: f64 = 75_000.0;
pub const JW1_WCP_BONUS_NOMINAL: f64 = 0.000_332_23;
pub const JW1_WCP_TOTAL_SHARES: f64 = 100_000.0;
pub const JW1_WCP_LEVEL3_PROXY: f64 = 4.55; // IFRS 13 Level 3 proxy Y0–Y2
pub const JW1_Y3_CRS_SHARES_SOLD: f64 = 25_000.0; // Y3 Capital Recovery Sale
pub const JW1_Y3_RESIDUAL_SHARES: f64 = 75_000.0; // bonus shares retained Y3+
pub const JW1_TAX_RATE: f64 = 0.27;

// ─── Output struct ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct AllocJw1Year {
    pub year: u32,
    pub interest_income: f64,
    pub realised_gain_crs: f64, // Y3: 25K × WCP Y3 bvps
    pub wcp_fv_change: f64,     // FVTPL each year; Y0 ≈ −$45K
    pub total_investment_income: f64,
    pub income_before_tax: f64,
    pub tax: f64,
    pub net_income: f64,
    pub loan_receivable_closing: f64, // state — $500K Y0–Y1, $0 Y2+
    pub wcp_shares_held: f64,         // state — 100K Y0–Y2, 75K Y3+
    pub wcp_per_share_value: f64,
    pub wcp_holding_fv: f64,
    pub cumulative_cash_received: f64, // interest + principal + CRS
    pub residual_nav: f64,             // non-zero Y10 only
}

// ─── Per-share WCP value ─────────────────────────────────────────────────────

fn wcp_value_at_year(wcp: &[WcpYear], y: u32) -> f64 {
    // Y0–Y2: IFRS 13 Level 3 proxy ($4.55).
    // Y3+: WCP engine book_value_per_share (CRS at Y3 establishes a transaction price
    // that displaces the Level 3 proxy going forward). Floor at Level 3 proxy.
    let idx = y as usize;
    match y {
        0..=2 => JW1_WCP_LEVEL3_PROXY,
        _ => {
            if idx < wcp.len() {
                wcp[idx].book_value_per_share.max(JW1_WCP_LEVEL3_PROXY)
            } else {
                JW1_WCP_LEVEL3_PROXY
            }
        }
    }
}

// ─── Forecast ────────────────────────────────────────────────────────────────

pub fn forecast(wcp: &[WcpYear]) -> Vec<AllocJw1Year> {
    let cost_basis = JW1_WCP_ALLOCATED_SHARES * JW1_WCP_PURCHASE_PRICE
        + JW1_WCP_BONUS_SHARES * JW1_WCP_BONUS_NOMINAL;

    let mut prev_wcp_fv = cost_basis; // initial recognition at cost
    let mut cumulative_cash: f64 = 0.0;
    let mut years: Vec<AllocJw1Year> = Vec::with_capacity(11);

    for y in 0u32..=10 {
        // Interest: full year Y1; half-year Y2 (18-month term)
        let interest_income = match y {
            1 => JW1_LOAN_PRINCIPAL * JW1_LOAN_RATE,
            2 => JW1_LOAN_PRINCIPAL * JW1_LOAN_RATE * 0.5,
            _ => 0.0,
        };

        let loan_receivable_closing = match y {
            0 | 1 => JW1_LOAN_PRINCIPAL,
            _ => 0.0,
        };

        let wcp_per_share_value = wcp_value_at_year(wcp, y);

        let wcp_shares_held = if y < 3 {
            JW1_WCP_TOTAL_SHARES
        } else {
            JW1_Y3_RESIDUAL_SHARES
        };

        let wcp_holding_fv = wcp_shares_held * wcp_per_share_value;

        // Y3: Capital Recovery Sale — sell 25K allocated shares at engine bvps
        let crs_cash = if y == 3 {
            JW1_Y3_CRS_SHARES_SOLD * wcp_per_share_value
        } else {
            0.0
        };

        // FV change: full position revaluation (mirrors SPV1 pattern).
        // At Y3: (75K×bvps + 25K×bvps) − 100K×$4.55 = 100K×(bvps−$4.55)
        let wcp_fv_change = (wcp_holding_fv + crs_cash) - prev_wcp_fv;

        let total_investment_income = interest_income + crs_cash + wcp_fv_change;
        let income_before_tax = total_investment_income;
        let tax = if income_before_tax > 0.0 {
            income_before_tax * JW1_TAX_RATE
        } else {
            0.0
        };
        let net_income = income_before_tax - tax;

        // Cash events: interest (Y1–Y2), loan principal return (Y2), CRS (Y3)
        let principal_return = if y == 2 { JW1_LOAN_PRINCIPAL } else { 0.0 };
        cumulative_cash += interest_income + crs_cash + principal_return;

        let residual_nav = if y == 10 { wcp_holding_fv } else { 0.0 };

        years.push(AllocJw1Year {
            year: y,
            interest_income,
            realised_gain_crs: crs_cash,
            wcp_fv_change,
            total_investment_income,
            income_before_tax,
            tax,
            net_income,
            loan_receivable_closing,
            wcp_shares_held,
            wcp_per_share_value,
            wcp_holding_fv,
            cumulative_cash_received: cumulative_cash,
            residual_nav,
        });

        prev_wcp_fv = wcp_holding_fv; // only remaining shares after CRS
    }

    years
}

pub fn forecast_json(wcp: &[WcpYear]) -> serde_json::Value {
    let years = forecast(wcp);
    let cost_basis = JW1_WCP_ALLOCATED_SHARES * JW1_WCP_PURCHASE_PRICE
        + JW1_WCP_BONUS_SHARES * JW1_WCP_BONUS_NOMINAL;
    let y3 = &years[3];
    let y10 = &years[10];
    let equity_return = y3.realised_gain_crs + y10.residual_nav;
    let moic = equity_return / cost_basis;
    serde_json::json!({
        "entity": "Woodfine Capital Projects Inc. — $500,000 Common Share Allocation (JW1)",
        "source": "tool-proforma-engine src/spv/alloc_jw1_proforma",
        "version": "V1",
        "generated_at": "2026-06-09",
        "consumes": "WCP V1 forecast (book_value_per_share path)",
        "inputs": {
            "loan_principal": JW1_LOAN_PRINCIPAL,
            "loan_rate": JW1_LOAN_RATE,
            "wcp_allocated_shares": JW1_WCP_ALLOCATED_SHARES,
            "wcp_purchase_price": JW1_WCP_PURCHASE_PRICE,
            "wcp_bonus_shares": JW1_WCP_BONUS_SHARES,
            "wcp_bonus_nominal": JW1_WCP_BONUS_NOMINAL,
            "wcp_total_shares": JW1_WCP_TOTAL_SHARES,
            "equity_cost_basis": cost_basis,
            "wcp_level3_proxy_y0_y2": JW1_WCP_LEVEL3_PROXY,
            "y3_crs_shares_sold": JW1_Y3_CRS_SHARES_SOLD,
            "y3_residual_shares": JW1_Y3_RESIDUAL_SHARES,
            "tax_rate": JW1_TAX_RATE,
        },
        "years": years,
        "y10_endpoint": {
            "y3_crs_proceeds": y3.realised_gain_crs,
            "y3_wcp_bvps": y3.wcp_per_share_value,
            "y10_residual_nav": y10.residual_nav,
            "y10_wcp_bvps": y10.wcp_per_share_value,
            "cumulative_cash_received": y10.cumulative_cash_received,
            "equity_return": equity_return,
            "moic_equity": moic,
        },
        "per_dollar_invested": {
            "denominator": cost_basis,
            "denominator_note": "total equity cost basis; loan principal excluded as cash-neutral round-trip",
            "annual_cash_receipts": years.iter().map(|yr| {
                let principal_ret = if yr.year == 2 { JW1_LOAN_PRINCIPAL } else { 0.0 };
                let cash = yr.interest_income + yr.realised_gain_crs + principal_ret;
                cash / cost_basis
            }).collect::<Vec<_>>(),
            "y10_residual_nav_per_dollar": y10.residual_nav / cost_basis,
            "moic_equity_per_dollar": moic,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spv::{pclp1_proforma, wcp_proforma};

    fn get_wcp() -> Vec<WcpYear> {
        let pclp1 = pclp1_proforma::forecast();
        wcp_proforma::forecast(&pclp1)
    }

    #[test]
    fn forecast_has_11_years() {
        assert_eq!(forecast(&get_wcp()).len(), 11);
    }

    #[test]
    fn y0_fv_change_is_negative() {
        let years = forecast(&get_wcp());
        // Level 3 $4.55 × 100K = $455,000 vs cost ≈$500,024.92 → loss ≈$45,025
        assert!(years[0].wcp_fv_change < 0.0);
        assert!(
            (years[0].wcp_fv_change + 45_025.0).abs() < 1.0,
            "expected ≈ -$45,025, got {}",
            years[0].wcp_fv_change
        );
    }

    #[test]
    fn y1_interest_correct() {
        let years = forecast(&get_wcp());
        assert!(
            (years[1].interest_income - 35_000.0).abs() < 0.01,
            "expected $35,000, got {}",
            years[1].interest_income
        );
    }

    #[test]
    fn y2_cumulative_cash_includes_principal() {
        let years = forecast(&get_wcp());
        // $35K (Y1) + $17.5K (Y2) + $500K (principal) = $552,500
        assert!(
            (years[2].cumulative_cash_received - 552_500.0).abs() < 1.0,
            "expected $552,500, got {}",
            years[2].cumulative_cash_received
        );
    }

    #[test]
    fn y3_crs_positive() {
        let years = forecast(&get_wcp());
        assert!(
            years[3].realised_gain_crs > 0.0,
            "Y3 CRS proceeds should be positive, got {}",
            years[3].realised_gain_crs
        );
    }

    #[test]
    fn y3_loan_balance_zero() {
        let years = forecast(&get_wcp());
        assert_eq!(years[3].loan_receivable_closing, 0.0);
    }

    #[test]
    fn y10_residual_nav_positive() {
        let years = forecast(&get_wcp());
        assert!(
            years[10].residual_nav > 0.0,
            "Y10 residual NAV should be positive, got {}",
            years[10].residual_nav
        );
    }

    #[test]
    fn json_parses_and_moic_positive() {
        let wcp = get_wcp();
        let json = forecast_json(&wcp);
        let moic = json["y10_endpoint"]["moic_equity"].as_f64().unwrap();
        assert!(moic > 0.0, "MOIC should be positive, got {moic}");
    }
}
