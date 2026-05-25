use crate::excel::wcp::WcpData;

fn fmt_m(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    let m = v / 1_000_000.0;
    if m < 0.0 {
        format!("({:.2}M)", m.abs())
    } else {
        format!("{:.2}M", m)
    }
}

fn fmt_dollar(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    if v < 0.0 {
        format!("({:.2})", v.abs())
    } else {
        format!("{:.2}", v)
    }
}

fn fmt_smart(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    let abs = v.abs();
    if abs >= 1_000_000.0 {
        let m = v / 1_000_000.0;
        if m < 0.0 {
            format!("({:.2}M)", m.abs())
        } else {
            format!("{:.2}M", m)
        }
    } else if abs >= 1_000.0 {
        let k = v / 1_000.0;
        if k < 0.0 {
            format!("({:.1}K)", k.abs())
        } else {
            format!("{:.1}K", k)
        }
    } else {
        if v < 0.0 {
            format!("(${:.0})", v.abs())
        } else {
            format!("${:.0}", v)
        }
    }
}

fn fmt_pct(v: f64) -> String {
    format!("{:.2}%", v * 100.0)
}

fn yr_header() -> String {
    "| | Y1 | Y2 | Y3 | Y4 | Y5 | Y6 | Y7 | Y8 | Y9 | Y10 |\n".to_string()
}

fn separator() -> String {
    "|:---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n".to_string()
}

fn yr_row_m(label: &str, vals: &[f64; 10]) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt_m(v)).collect();
    format!("| {} | {} |\n", label, cells.join(" | "))
}

fn yr_row_d(label: &str, vals: &[f64; 10]) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt_dollar(v)).collect();
    format!("| {} | {} |\n", label, cells.join(" | "))
}

fn yr_row_smart(label: &str, vals: &[f64; 10]) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt_smart(v)).collect();
    format!("| {} | {} |\n", label, cells.join(" | "))
}

fn yr_row_p(label: &str, vals: &[f64; 10]) -> String {
    let cells: Vec<String> = vals
        .iter()
        .map(|&v| {
            if v == 0.0 {
                "—".to_string()
            } else {
                fmt_pct(v)
            }
        })
        .collect();
    format!("| {} | {} |\n", label, cells.join(" | "))
}

pub fn render(data: &WcpData) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "# {}\n\n**{}** — {}\n\n",
        data.title, data.entity, data.date
    ));
    out.push_str(&format!(
        "**Shares Outstanding:** {:.0}    **Price per Share:** ${:.2}\n\n",
        data.shares_outstanding, data.price_per_share
    ));
    out.push_str("---\n\n");

    // ── Income Statement ──────────────────────────────────────────────────────
    out.push_str("## Income Statement — 10-Year Forecast (CAD)\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    out.push_str(&yr_row_m("Gross Income", &data.income.gross_income));
    out.push_str(&yr_row_smart(&data.gna_label_1, &data.income.gna_nyc));
    out.push_str(&yr_row_smart(&data.gna_label_2, &data.income.gna_berlin));
    out.push_str(&yr_row_m(
        "Total Operating Expenses",
        &data.income.total_expenses,
    ));
    out.push_str(&yr_row_m("**EBITDA**", &data.income.ebitda));
    out.push_str(&yr_row_d("EBITDA per Share", &data.income.ebitda_per_share));
    out.push_str(&yr_row_m("Taxes (27%)", &data.income.taxes));
    out.push_str(&yr_row_m("**Earnings**", &data.income.earnings));
    out.push_str(&yr_row_d(
        "Earnings per Share",
        &data.income.earnings_per_share,
    ));

    // ── Balance Sheet ─────────────────────────────────────────────────────────
    out.push_str("\n---\n\n## Balance Sheet — 10-Year Forecast (CAD)\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    out.push_str(&yr_row_m(
        "Cumulative Free Cash Flow (WCI)",
        &data.book.cumulative_fcf_wci,
    ));
    out.push_str(&yr_row_m(
        "10% Ownership in Direct-Hold Solution",
        &data.book.beneficial_ownership_lps,
    ));
    out.push_str(&yr_row_m("**Book Value**", &data.book.book_value));
    out.push_str(&yr_row_d(
        "Book Value per Share",
        &data.book.book_value_per_share,
    ));

    // ── Page 2: Revenue Generator ─────────────────────────────────────────────
    out.push_str("\n\n---\n\n## Revenue Generator — Direct-Hold Solutions Cash Flow Timeline\n\n");
    out.push_str("Shows when each direct-hold solution enters cash flow (first non-zero distribution year).\n\n");

    const LP_SHORT_NAMES: [&str; 6] = [
        "Professional Centres Canada LP",
        "Professional Centres United States LP",
        "Professional Centres Spain SOCIMI",
        "Professional Centres Mexico FIBRA",
        "Vertical Warehouse LP",
        "Parking Structure LP",
    ];

    out.push_str(&yr_header());
    out.push_str(&separator());
    for (i, lp) in data.lps.iter().enumerate() {
        let label = LP_SHORT_NAMES.get(i).copied().unwrap_or(lp.name.as_str());
        out.push_str(&yr_row_m(
            &format!("{label} — Advisory Fee"),
            &lp.advisory_fee,
        ));
        out.push_str(&yr_row_m(
            &format!("{label} — Distributions"),
            &lp.distributions,
        ));
        out.push_str(&yr_row_m(&format!("{label} — NAV"), &lp.nav));
    }

    // ── Page 3: Valuation Matrix ──────────────────────────────────────────────
    out.push_str("\n\n---\n\n## Valuation Matrix\n\n");

    out.push_str("### Market Valuation\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    out.push_str(&yr_row_m(
        "Earnings Valuation",
        &data.market.earnings_valuation,
    ));
    out.push_str(&yr_row_m(
        "**Market Valuation**",
        &data.market.market_valuation,
    ));
    out.push_str(&yr_row_d(
        "Market Value per Share",
        &data.market.market_value_per_share,
    ));
    {
        let pe: Vec<String> = data
            .market
            .pe_ratio
            .iter()
            .map(|&v| {
                if v == 0.0 {
                    "—".to_string()
                } else {
                    format!("{:.2}x", v)
                }
            })
            .collect();
        out.push_str(&format!("| P/E Ratio | {} |\n", pe.join(" | ")));
    }

    out.push_str("\n### Fair Valuation\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    out.push_str(&yr_row_d(
        "Fair Value per Share",
        &data.fair_div.fair_value_per_share,
    ));

    out.push_str("\n### Dividend Valuation\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    out.push_str(&yr_row_m(
        "Dividend Valuation",
        &data.fair_div.dividend_valuation,
    ));
    out.push_str(&yr_row_d(
        "Dividend Value per Share",
        &data.fair_div.dividend_value_per_share,
    ));

    out.push_str("\n### Ratio Summary\n\n");
    out.push_str(&yr_header());
    out.push_str(&separator());
    // Market vs Book: derive from market_valuation / book_value
    {
        let mv_bv: [f64; 10] = std::array::from_fn(|i| {
            if data.book.book_value[i] != 0.0 {
                data.market.market_valuation[i] / data.book.book_value[i]
            } else {
                0.0
            }
        });
        out.push_str(&yr_row_p("Market vs. Book Value", &mv_bv));
    }
    {
        let mv_fv: [f64; 10] = std::array::from_fn(|i| {
            let fv = data.fair_div.fair_value_per_share[i] * data.shares_outstanding;
            if fv != 0.0 {
                data.market.market_valuation[i] / fv
            } else {
                0.0
            }
        });
        out.push_str(&yr_row_p("Market vs. Fair Valuation", &mv_fv));
    }
    {
        let mv_div: [f64; 10] = std::array::from_fn(|i| {
            if data.fair_div.dividend_valuation[i] != 0.0 {
                data.market.market_valuation[i] / data.fair_div.dividend_valuation[i]
            } else {
                0.0
            }
        });
        out.push_str(&yr_row_p("Market vs. Dividend Valuation", &mv_div));
    }

    out
}
