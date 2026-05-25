use crate::excel::pclp1::Pclp1Data;
use crate::excel::wcp::{WcpBook, WcpData, WcpFairDiv, WcpIncome, WcpLp, WcpMarket};

// BenCal holds 10% of Ambassadors Direct 1, which holds 30% of WCP.
const BENCAL_AD1_STAKE: f64 = 0.10;
const AD1_WCP_STAKE: f64 = 0.30;

// BenCal holds 10% of Ambassadors Direct 2 LP (250,000 units × 10% = 25,000 units).
const BENCAL_AD2_UNITS: f64 = 25_000.0;

// WCP share-sale commission income: $100K/year spread over Y1–Y3.
const COMMISSION_PER_YEAR: f64 = 100_000.0;

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
        entity: "BenCal Holdings Inc.".to_string(),
        date: wcp.date.clone(),
        shares_outstanding: 2.0, // Ben and Caleb
        price_per_share: 0.0,    // not applicable — private holding company
        lps,
        income,
        book,
        market,
        fair_div,
        gna_label_1: "Legal Services".to_string(),
        gna_label_2: "Accounting Services".to_string(),
    }
}
