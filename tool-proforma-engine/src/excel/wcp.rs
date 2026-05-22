use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

use super::{get_f64, get_str};

/// One LP fund tracked in the WCP 42M model.
#[derive(Debug, Clone)]
pub struct WcpLp {
    pub name: String,
    /// Advisory fee Y1–Y10
    pub advisory_fee: [f64; 10],
    /// Distributions Y1–Y10
    pub distributions: [f64; 10],
    /// Net Asset Value Y1–Y10
    pub nav: [f64; 10],
}

/// Corporate income statement (Y1–Y10).
#[derive(Debug, Clone)]
pub struct WcpIncome {
    pub gross_income: [f64; 10],
    pub referral_fees: [f64; 10],
    pub wpi_consulting: [f64; 10],
    pub gna_nyc: [f64; 10],
    pub gna_berlin: [f64; 10],
    pub total_expenses: [f64; 10],
    pub ebitda: [f64; 10],
    pub ebitda_per_share: [f64; 10],
    pub taxes: [f64; 10],
    pub earnings: [f64; 10],
    pub earnings_per_share: [f64; 10],
}

/// Book valuation (Y1–Y10).
#[derive(Debug, Clone)]
pub struct WcpBook {
    pub cumulative_fcf_wci: [f64; 10],
    pub beneficial_ownership_lps: [f64; 10],
    pub book_value: [f64; 10],
    pub book_value_per_share: [f64; 10],
}

/// Market valuation (Y1–Y10).
#[derive(Debug, Clone)]
pub struct WcpMarket {
    pub earnings_valuation: [f64; 10],
    pub market_valuation: [f64; 10],
    pub pe_ratio: [f64; 10],
    pub market_value_per_share: [f64; 10],
}

/// Fair and dividend valuations (Y1–Y10).
#[derive(Debug, Clone)]
pub struct WcpFairDiv {
    pub fair_value_per_share: [f64; 10],
    pub dividend_valuation: [f64; 10],
    pub dividend_value_per_share: [f64; 10],
}

/// Full parsed WCP 42M model.
#[derive(Debug)]
pub struct WcpData {
    pub title: String,
    pub entity: String,
    pub date: String,
    pub shares_outstanding: f64,
    pub price_per_share: f64,
    pub lps: Vec<WcpLp>,
    pub income: WcpIncome,
    pub book: WcpBook,
    pub market: WcpMarket,
    pub fair_div: WcpFairDiv,
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<WcpData, Box<dyn std::error::Error>> {
    let mut wb: Xlsx<_> = open_workbook(path)?;
    let range = wb.worksheet_range("WCP_42M")?;

    // NOTE: the sheet's used range starts at absolute (row 1, col 1).
    // get_value() takes absolute 0-indexed coordinates.
    // Discover displays rows as absolute indices (row N = absolute row N).
    // Column labels: [A]=absolute 1, [B]=2, [C]=3, [D]=4, [E]=5, [F]=6, [G]=7, [H]=8 ... [Q]=17
    //
    // So: r(N) = N directly (no -1); column [X] = letter_index(X) + 1.
    let r = |row: u32| row;
    // Y1–Y10 in columns [H]–[Q] = absolute 8–17
    let yc = |y: usize| (8 + y) as u32;

    // Header metadata: col [B] = absolute col 2
    let title = get_str(&range, r(1), 2);
    let entity = get_str(&range, r(2), 2);
    let date = get_str(&range, r(4), 2);
    let shares_outstanding = get_f64(&range, r(8), 5); // col [E] = absolute 5
    let price_per_share = get_f64(&range, r(9), 5);

    // LP blocks: name in col [B] (absolute 2); advisory/dist/nav on following 3 rows, data in [H]–[Q].
    // Blocks at discover rows 19, 24, 29, 34, 39, 44.
    let lp_name_rows: [u32; 6] = [19, 24, 29, 34, 39, 44];
    let mut lps = Vec::with_capacity(6);
    for &nr in &lp_name_rows {
        let name = get_str(&range, r(nr), 2);
        if name.is_empty() {
            continue;
        }
        lps.push(WcpLp {
            name,
            advisory_fee: std::array::from_fn(|y| get_f64(&range, r(nr + 1), yc(y))),
            distributions: std::array::from_fn(|y| get_f64(&range, r(nr + 2), yc(y))),
            nav: std::array::from_fn(|y| get_f64(&range, r(nr + 3), yc(y))),
        });
    }

    let income = WcpIncome {
        gross_income: std::array::from_fn(|y| get_f64(&range, r(54), yc(y))),
        referral_fees: std::array::from_fn(|y| get_f64(&range, r(57), yc(y))),
        wpi_consulting: std::array::from_fn(|y| get_f64(&range, r(58), yc(y))),
        gna_nyc: std::array::from_fn(|y| get_f64(&range, r(59), yc(y))),
        gna_berlin: std::array::from_fn(|y| get_f64(&range, r(60), yc(y))),
        total_expenses: std::array::from_fn(|y| get_f64(&range, r(61), yc(y))),
        ebitda: std::array::from_fn(|y| get_f64(&range, r(63), yc(y))),
        ebitda_per_share: std::array::from_fn(|y| get_f64(&range, r(64), yc(y))),
        taxes: std::array::from_fn(|y| get_f64(&range, r(66), yc(y))),
        earnings: std::array::from_fn(|y| get_f64(&range, r(68), yc(y))),
        earnings_per_share: std::array::from_fn(|y| get_f64(&range, r(69), yc(y))),
    };

    let book = WcpBook {
        cumulative_fcf_wci: std::array::from_fn(|y| get_f64(&range, r(74), yc(y))),
        beneficial_ownership_lps: std::array::from_fn(|y| get_f64(&range, r(75), yc(y))),
        book_value: std::array::from_fn(|y| get_f64(&range, r(76), yc(y))),
        book_value_per_share: std::array::from_fn(|y| get_f64(&range, r(77), yc(y))),
    };

    let market = WcpMarket {
        earnings_valuation: std::array::from_fn(|y| get_f64(&range, r(80), yc(y))),
        market_valuation: std::array::from_fn(|y| get_f64(&range, r(82), yc(y))),
        pe_ratio: std::array::from_fn(|y| get_f64(&range, r(83), yc(y))),
        market_value_per_share: std::array::from_fn(|y| get_f64(&range, r(84), yc(y))),
    };

    let fair_div = WcpFairDiv {
        fair_value_per_share: std::array::from_fn(|y| get_f64(&range, r(91), yc(y))),
        dividend_valuation: std::array::from_fn(|y| get_f64(&range, r(95), yc(y))),
        dividend_value_per_share: std::array::from_fn(|y| get_f64(&range, r(96), yc(y))),
    };

    Ok(WcpData {
        title,
        entity,
        date,
        shares_outstanding,
        price_per_share,
        lps,
        income,
        book,
        market,
        fair_div,
    })
}
