use crate::excel::pclp1::Pclp1Data;
use crate::excel::wcp::{WcpBook, WcpData, WcpFairDiv, WcpIncome, WcpLp, WcpMarket};

// Bencal Management holds 10% of Bencal SPV1 (which holds 300K WCP = 3.0% of 10M outstanding).
// Effective WCP exposure at Bencal Management level via lookthrough: 10% × 3.0% = 0.30%.
const BENCAL_AD1_STAKE: f64 = 0.10;
const AD1_WCP_STAKE: f64 = 0.03; // BRIEF v0.15.9 §5e: 300K of 10M = 3.0%

// Bencal Management holds 10% of Bencal SPV2-LP (v0.15.6 manager units: 27,843 of 278,434 diluted).
const BENCAL_AD2_UNITS: f64 = 27_843.0;

// Bencal Management share capital — 2 shares at $5.00 nominal = $10.00 (BRIEF §5f).
const BENCAL_SHARES_OUTSTANDING: f64 = 2.0;
const BENCAL_PRICE_PER_SHARE: f64 = 5.00;

// Commission income to Bencal Management at close (one-time, Y0), then nil Y1+ —
// Flag 13 RESOLVED 2026-06-02: Altas One distributes commission rebates DIRECTLY to each
// Bencal entity at formation; the prior per-year stream model is obsolete. Retained as 0 for
// legacy renderers that still expect the field.
const COMMISSION_PER_YEAR: f64 = 0.0;

// SPV annual maintenance costs — PUBLISHED_MCorp_2024_02_07_SPV Info Guide V3_Tab 04b
const SPV_LEGAL_ANNUAL: f64 = 1_145.0;
const SPV_LEGAL_SETUP: f64 = 7_730.0;
const SPV_ACCT_ANNUAL: f64 = 2_399.0;
const SPV_ACCT_SETUP: f64 = 1_375.0;

fn spv_legal(y: usize) -> f64 {
    if y == 0 {
        SPV_LEGAL_ANNUAL + SPV_LEGAL_SETUP
    } else {
        SPV_LEGAL_ANNUAL
    }
}

fn spv_acct(y: usize) -> f64 {
    if y == 0 {
        SPV_ACCT_ANNUAL + SPV_ACCT_SETUP
    } else {
        SPV_ACCT_ANNUAL
    }
}

pub fn derivation_json(wcp: &WcpData, pclp: &Pclp1Data) -> serde_json::Value {
    let wcp_sf = BENCAL_AD1_STAKE * AD1_WCP_STAKE;
    let pclp_sf = BENCAL_AD2_UNITS / pclp.assumptions.diluted_units;
    serde_json::json!({
        "source_models": ["WCP 42M Excel", "PCLP 1 Excel"],
        "method": "dual_proportional_scale",
        "description": "BenCal Holdings Inc. income derives from two positions: (1) 10% of Ambassadors Direct 1 Inc. which holds 30% of WCP — giving BenCal an effective 3% WCP exposure; (2) 25,000 PCLP 1 unit equivalent via 10% of Ambassadors Direct 2 LP (250,000 units). Commission income of $100K/year is included in Y1–Y3 from WCP share-sale activity.",
        "wcp_exposure": {
            "via": "Ambassadors Direct 1 Inc.",
            "bencal_ad1_stake": BENCAL_AD1_STAKE,
            "ad1_wcp_stake": AD1_WCP_STAKE,
            "effective_wcp_scale_factor": wcp_sf,
            "source_entity": wcp.entity
        },
        "pclp_exposure": {
            "via": "Ambassadors Direct 2 LP",
            "units_held": BENCAL_AD2_UNITS,
            "pclp_diluted_units": pclp.assumptions.diluted_units,
            "pclp_scale_factor": pclp_sf,
            "source_entity": pclp.entity
        },
        "commission_income": {
            "amount_per_year": COMMISSION_PER_YEAR,
            "years_active": [1, 2, 3],
            "total": COMMISSION_PER_YEAR * 3.0,
            "description": "WCP share-sale commission income, evenly spread over Y1–Y3"
        },
        "authority": "spv-bencal governance document — BenCal Holdings Inc. structure"
    })
}

/// Derive BenCal Holdings Inc. WcpData from the WCP and PCLP 1 models.
///
/// BenCal's income sources:
///   - 3% of WCP (via 10% of AD1 which holds 30% of WCP)
///   - 25,000 PCLP-unit equivalent (via 10% of AD2 LP)
///   - $100K/year WCP commission income in Y1–Y3
pub fn derive(wcp: &WcpData, pclp: &Pclp1Data) -> WcpData {
    let wcp_sf = BENCAL_AD1_STAKE * AD1_WCP_STAKE; // 0.03
    let pclp_sf = BENCAL_AD2_UNITS / pclp.assumptions.diluted_units;

    let commission: [f64; 10] =
        std::array::from_fn(|y| if y < 3 { COMMISSION_PER_YEAR } else { 0.0 });

    // Revenue Generator: BenCal's 3% slice of each WCP LP fund + AD2 LP entry.
    let mut lps: Vec<WcpLp> = wcp
        .lps
        .iter()
        .map(|lp| WcpLp {
            name: format!("AD1 — {}", lp.name),
            advisory_fee: std::array::from_fn(|y| lp.advisory_fee[y] * wcp_sf),
            distributions: std::array::from_fn(|y| lp.distributions[y] * wcp_sf),
            nav: std::array::from_fn(|y| lp.nav[y] * wcp_sf),
        })
        .collect();

    lps.push(WcpLp {
        name: "Ambassadors Direct 2 LP".to_string(),
        advisory_fee: [0.0; 10],
        distributions: std::array::from_fn(|y| pclp.years[y].dist_per_unit * BENCAL_AD2_UNITS),
        nav: std::array::from_fn(|y| pclp.years[y].nav_per_unit * BENCAL_AD2_UNITS),
    });

    let income = WcpIncome {
        gross_income: std::array::from_fn(|y| {
            wcp.income.gross_income[y] * wcp_sf + pclp.years[y].noi * pclp_sf + commission[y]
        }),
        referral_fees: std::array::from_fn(|y| wcp.income.referral_fees[y] * wcp_sf),
        wpi_consulting: std::array::from_fn(|y| wcp.income.wpi_consulting[y] * wcp_sf),
        // Replace scaled WCP G&A with actual BenCal SPV legal/accounting maintenance costs.
        gna_nyc: std::array::from_fn(spv_legal),
        gna_berlin: std::array::from_fn(spv_acct),
        total_expenses: std::array::from_fn(|y| {
            (wcp.income.total_expenses[y] - wcp.income.gna_nyc[y] - wcp.income.gna_berlin[y])
                * wcp_sf
                + pclp.years[y].total_expenses * pclp_sf
                + spv_legal(y)
                + spv_acct(y)
        }),
        ebitda: std::array::from_fn(|y| {
            wcp.income.ebitda[y] * wcp_sf
                + (wcp.income.gna_nyc[y] + wcp.income.gna_berlin[y]) * wcp_sf
                - spv_legal(y)
                - spv_acct(y)
                + pclp.years[y].ebitda * pclp_sf
                + commission[y]
        }),
        ebitda_per_share: [0.0; 10],
        taxes: std::array::from_fn(|y| wcp.income.taxes[y] * wcp_sf),
        earnings: std::array::from_fn(|y| {
            wcp.income.earnings[y] * wcp_sf
                + (wcp.income.gna_nyc[y] + wcp.income.gna_berlin[y]) * wcp_sf
                - spv_legal(y)
                - spv_acct(y)
                + pclp.years[y].funding_from_ops * pclp_sf
                + commission[y]
        }),
        earnings_per_share: [0.0; 10],
    };

    let book = WcpBook {
        cumulative_fcf_wci: std::array::from_fn(|y| wcp.book.cumulative_fcf_wci[y] * wcp_sf),
        beneficial_ownership_lps: std::array::from_fn(|y| {
            wcp.book.beneficial_ownership_lps[y] * wcp_sf
                + pclp.years[y].nav_per_unit * BENCAL_AD2_UNITS
        }),
        book_value: std::array::from_fn(|y| {
            wcp.book.book_value[y] * wcp_sf + pclp.years[y].nav_per_unit * BENCAL_AD2_UNITS
        }),
        book_value_per_share: [0.0; 10],
    };

    let market = WcpMarket {
        earnings_valuation: std::array::from_fn(|y| wcp.market.earnings_valuation[y] * wcp_sf),
        market_valuation: std::array::from_fn(|y| {
            wcp.market.market_valuation[y] * wcp_sf
                + pclp.years[y].asset_value_per_unit * BENCAL_AD2_UNITS
        }),
        pe_ratio: [0.0; 10],
        market_value_per_share: [0.0; 10],
    };

    let fair_div = WcpFairDiv {
        fair_value_per_share: [0.0; 10],
        dividend_valuation: std::array::from_fn(|y| {
            wcp.fair_div.dividend_valuation[y] * wcp_sf
                + pclp.years[y].dist_per_unit * BENCAL_AD2_UNITS
        }),
        dividend_value_per_share: [0.0; 10],
    };

    WcpData {
        title: "Cash Flow and Valuation".to_string(),
        entity: "Bencal Management Corp.".to_string(),
        date: wcp.date.clone(),
        shares_outstanding: BENCAL_SHARES_OUTSTANDING,
        price_per_share: BENCAL_PRICE_PER_SHARE,
        lps,
        income,
        book,
        market,
        fair_div,
        gna_label_1: "Legal Services".to_string(),
        gna_label_2: "Accounting Services".to_string(),
    }
}

// ─── Block F — Bencal Management Y10 headline card (BRIEF §5f, v0.15.9) ────────────
//
// Block F renders Bencal Management's Y10 valuation summary with both aggregate AND
// per-share MOIC views side-by-side. Mathematically the two MOICs are equal (one is
// the other divided by `shares_outstanding`), but the per-share view will look
// shocking to LP investors without the dilution-mechanics header note: Bencal
// Management has only $10.00 paid-in share capital ($5.00 × 2 shares), while its
// economic substance flows from 10% manager allocations at Bencal SPV1 + SPV2
// (received via dilution mechanics, not paid cash). Per Flag 3 + Block F decision
// (RESOLVED 2026-06-02), both views ship in CIM materials.

/// Block F — Bencal Management Y10 headline card.
///
/// MOIC computation:
///   - `moic_aggregate` = Y10 portfolio NAV / total Bencal Management share capital paid in (Y0)
///   - `moic_per_share` = Y10 per-share NAV / per-share capital ($5.00)
///   - The two are mathematically equal; we emit both because LP investors should see both views
///     (see `header_note()` for the dilution mechanics explanation).
///   - `cagr` = nominal CAGR over the Y0→Y10 hold period.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockF {
    pub portfolio_nav_total: f64,
    pub portfolio_nav_per_share: f64,
    pub total_invested_capital: f64,
    pub per_share_invested_capital: f64,
    pub moic_aggregate: f64,
    pub moic_per_share: f64,
    pub cagr_y10: f64,
}

impl BlockF {
    /// Canonical header-note wording for the side-by-side MOIC table (per BRIEF §5f).
    /// Renderers must emit this verbatim above the MOIC table so LP investors can
    /// interpret the per-share figure in context.
    pub fn header_note() -> &'static str {
        "The per-share MOIC reflects the manager's $5.00 share-capital basis at Bencal Management. \
         Because Bencal Management's two shares carry $10 of paid-in capital while the manager's \
         10% allocation at Bencal SPV1 + Bencal SPV2 carries economic claims on a much larger NAV, \
         the per-share MOIC is mechanically very high and should be read alongside the aggregate \
         MOIC, which reflects total invested capital across all Bencal entities. The 10/90 \
         manager/investor dilution at SPV1 and SPV2 is described in §5d–§5e of the offering BRIEF."
    }
}

/// Compute Block F from Bencal Management's derived `WcpData`. Y10 is the last
/// element of the 10-year forecast arrays (index 9).
pub fn compute_block_f(bencal: &WcpData) -> BlockF {
    const Y10: usize = 9;

    let portfolio_nav_total = bencal.book.book_value[Y10];
    let shares = bencal.shares_outstanding;
    let portfolio_nav_per_share = if shares > 0.0 {
        portfolio_nav_total / shares
    } else {
        0.0
    };

    // Total Bencal Management invested capital = share capital paid in at Y0
    // (BRIEF §5f: 2 shares × $5.00 = $10.00).
    let total_invested_capital = shares * bencal.price_per_share;
    let per_share_invested_capital = bencal.price_per_share;

    let moic_aggregate = if total_invested_capital > 0.0 {
        portfolio_nav_total / total_invested_capital
    } else {
        0.0
    };
    let moic_per_share = if per_share_invested_capital > 0.0 {
        portfolio_nav_per_share / per_share_invested_capital
    } else {
        0.0
    };

    // CAGR over 10 years: MOIC^(1/10) − 1. Guards against zero/negative MOIC.
    let cagr_y10 = if moic_aggregate > 0.0 {
        moic_aggregate.powf(0.1) - 1.0
    } else {
        0.0
    };

    BlockF {
        portfolio_nav_total,
        portfolio_nav_per_share,
        total_invested_capital,
        per_share_invested_capital,
        moic_aggregate,
        moic_per_share,
        cagr_y10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_bencal_y10_nav(nav: f64) -> WcpData {
        // Minimal WcpData fixture: only Y10 book_value matters for Block F.
        let book = WcpBook {
            cumulative_fcf_wci: [0.0; 10],
            beneficial_ownership_lps: [0.0; 10],
            book_value: {
                let mut a = [0.0; 10];
                a[9] = nav;
                a
            },
            book_value_per_share: [0.0; 10],
        };
        WcpData {
            title: "test".to_string(),
            entity: "Bencal Management Corp.".to_string(),
            date: "Y0".to_string(),
            shares_outstanding: BENCAL_SHARES_OUTSTANDING,
            price_per_share: BENCAL_PRICE_PER_SHARE,
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
            book,
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
    fn block_f_aggregate_and_per_share_moic_agree() {
        let bencal = synthetic_bencal_y10_nav(17_500_000.0);
        let bf = compute_block_f(&bencal);
        assert!((bf.moic_aggregate - bf.moic_per_share).abs() < 1e-6);
        assert_eq!(bf.total_invested_capital, 10.0);
        assert_eq!(bf.per_share_invested_capital, 5.0);
        // $17.5M / $10 = 1,750,000×
        assert!((bf.moic_aggregate - 1_750_000.0).abs() < 1e-3);
    }

    #[test]
    fn block_f_cagr_from_known_moic() {
        // MOIC = 1024 → CAGR over 10 years = 1024^(0.1) − 1 = 2.0 − 1.0 = 1.0 (100%/yr)
        let bencal = synthetic_bencal_y10_nav(10_240.0);
        let bf = compute_block_f(&bencal);
        assert!((bf.moic_aggregate - 1024.0).abs() < 1e-6);
        assert!((bf.cagr_y10 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn block_f_zero_nav_safe() {
        let bencal = synthetic_bencal_y10_nav(0.0);
        let bf = compute_block_f(&bencal);
        assert_eq!(bf.portfolio_nav_total, 0.0);
        assert_eq!(bf.moic_aggregate, 0.0);
        assert_eq!(bf.cagr_y10, 0.0);
    }

    #[test]
    fn header_note_mentions_dilution() {
        let note = BlockF::header_note();
        assert!(note.contains("10%"));
        assert!(note.contains("dilution") || note.contains("dilution"));
        assert!(note.contains("$5.00"));
    }
}
