use crate::excel::wcp::{WcpBook, WcpData, WcpFairDiv, WcpIncome, WcpLp, WcpMarket};

const AD1_SHARES: f64 = 3_000_000.0;
const AD1_SHARE_PRICE: f64 = 1.0;

/// Derive Ambassadors Direct 1 Inc. WcpData from the parent WCP model.
/// AD1 holds 3,000,000 WCP common shares (30% of 10M outstanding).
/// All totals scale by 0.30; per-share metrics are unchanged.
pub fn derive(wcp: &WcpData) -> WcpData {
    let sf = AD1_SHARES / wcp.shares_outstanding;

    WcpData {
        title: wcp.title.clone(),
        entity: "Ambassadors Direct 1 Inc.".to_string(),
        date: wcp.date.clone(),
        shares_outstanding: AD1_SHARES,
        price_per_share: AD1_SHARE_PRICE,
        lps: wcp.lps.iter().map(|lp| scale_lp(lp, sf)).collect(),
        income: scale_income(&wcp.income, sf),
        book: scale_book(&wcp.book, sf),
        market: scale_market(&wcp.market, sf),
        fair_div: scale_fair_div(&wcp.fair_div, sf),
    }
}

fn scale_lp(lp: &WcpLp, sf: f64) -> WcpLp {
    WcpLp {
        name: lp.name.clone(),
        advisory_fee: std::array::from_fn(|y| lp.advisory_fee[y] * sf),
        distributions: std::array::from_fn(|y| lp.distributions[y] * sf),
        nav: std::array::from_fn(|y| lp.nav[y] * sf),
    }
}

fn scale_income(inc: &WcpIncome, sf: f64) -> WcpIncome {
    WcpIncome {
        gross_income: std::array::from_fn(|y| inc.gross_income[y] * sf),
        referral_fees: std::array::from_fn(|y| inc.referral_fees[y] * sf),
        wpi_consulting: std::array::from_fn(|y| inc.wpi_consulting[y] * sf),
        gna_nyc: std::array::from_fn(|y| inc.gna_nyc[y] * sf),
        gna_berlin: std::array::from_fn(|y| inc.gna_berlin[y] * sf),
        total_expenses: std::array::from_fn(|y| inc.total_expenses[y] * sf),
        ebitda: std::array::from_fn(|y| inc.ebitda[y] * sf),
        ebitda_per_share: inc.ebitda_per_share,
        taxes: std::array::from_fn(|y| inc.taxes[y] * sf),
        earnings: std::array::from_fn(|y| inc.earnings[y] * sf),
        earnings_per_share: inc.earnings_per_share,
    }
}

fn scale_book(book: &WcpBook, sf: f64) -> WcpBook {
    WcpBook {
        cumulative_fcf_wci: std::array::from_fn(|y| book.cumulative_fcf_wci[y] * sf),
        beneficial_ownership_lps: std::array::from_fn(|y| book.beneficial_ownership_lps[y] * sf),
        book_value: std::array::from_fn(|y| book.book_value[y] * sf),
        book_value_per_share: book.book_value_per_share,
    }
}

fn scale_market(mkt: &WcpMarket, sf: f64) -> WcpMarket {
    WcpMarket {
        earnings_valuation: std::array::from_fn(|y| mkt.earnings_valuation[y] * sf),
        market_valuation: std::array::from_fn(|y| mkt.market_valuation[y] * sf),
        pe_ratio: mkt.pe_ratio,
        market_value_per_share: mkt.market_value_per_share,
    }
}

fn scale_fair_div(fd: &WcpFairDiv, sf: f64) -> WcpFairDiv {
    WcpFairDiv {
        fair_value_per_share: fd.fair_value_per_share,
        dividend_valuation: std::array::from_fn(|y| fd.dividend_valuation[y] * sf),
        dividend_value_per_share: fd.dividend_value_per_share,
    }
}
