// Bencal Management Block F renderer — side-by-side per-share + aggregate MOIC
// (BRIEF §5f, v0.15.9; Flag 3 + Block F decision RESOLVED 2026-06-02).
//
// Block F is the Y10 headline card published in CIM investor materials. The header
// note explains why per-share MOIC looks shocking — the entity's $5.00 nominal
// share-capital basis vs the much larger economic claim flowing through 10/90
// dilution at Bencal SPV1 + SPV2.

use crate::spv::bencal::BlockF;

/// Format a dollar value with a scale suffix appropriate for its magnitude.
fn fmt_currency(v: f64) -> String {
    let abs = v.abs();
    if abs >= 1_000_000.0 {
        format!("${:.2}M", v / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("${:.2}K", v / 1_000.0)
    } else {
        format!("${:.2}", v)
    }
}

fn fmt_moic(v: f64) -> String {
    let abs = v.abs();
    if abs >= 1_000_000.0 {
        format!("{:.2}M×", v / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.2}K×", v / 1_000.0)
    } else if abs >= 100.0 {
        format!("{:.0}×", v)
    } else {
        format!("{:.2}×", v)
    }
}

fn fmt_pct(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}

/// Render Block F as markdown — the canonical text source. Renderers for other
/// formats (HTML, PDF) should derive from this layout.
pub fn render(block: &BlockF) -> String {
    let mut out = String::new();
    out.push_str("# Bencal Management Corp. — Y10 Headline (Block F)\n\n");
    out.push_str("> ");
    out.push_str(BlockF::header_note());
    out.push_str("\n\n");
    out.push_str("| Metric | Aggregate (entity total) | Per-Share ($5.00 basis) |\n");
    out.push_str("|---|---|---|\n");
    out.push_str(&format!(
        "| Y10 Portfolio NAV | {} | {} |\n",
        fmt_currency(block.portfolio_nav_total),
        fmt_currency(block.portfolio_nav_per_share),
    ));
    out.push_str(&format!(
        "| Invested Capital (Y0 paid in) | {} | {} |\n",
        fmt_currency(block.total_invested_capital),
        fmt_currency(block.per_share_invested_capital),
    ));
    out.push_str(&format!(
        "| **MOIC** | **{}** | **{}** |\n",
        fmt_moic(block.moic_aggregate),
        fmt_moic(block.moic_per_share),
    ));
    out.push_str(&format!(
        "| Nominal CAGR (Y0→Y10) | {} | {} |\n",
        fmt_pct(block.cagr_y10),
        fmt_pct(block.cagr_y10),
    ));
    out.push('\n');
    out.push_str(
        "*Aggregate and per-share MOIC are mathematically equal; both columns are shown so \
         readers can read the per-share figure in context of the entity's nominal $10 paid-in \
         share capital. See §5d–§5e of the offering BRIEF for the SPV1/SPV2 dilution mechanics.*\n",
    );
    out
}

/// Render Block F as a self-contained HTML fragment (`<section>...`). For inclusion
/// in CIM materials with consistent typography.
pub fn render_html(block: &BlockF) -> String {
    let mut out = String::new();
    out.push_str("<section class=\"block-f\">\n");
    out.push_str("<h2>Bencal Management Corp. &mdash; Y10 Headline (Block F)</h2>\n");
    out.push_str("<blockquote class=\"header-note\"><p>");
    out.push_str(&html_escape(BlockF::header_note()));
    out.push_str("</p></blockquote>\n");
    out.push_str("<table class=\"block-f-moic\">\n");
    out.push_str(
        "<thead><tr><th>Metric</th><th>Aggregate (entity total)</th>\
         <th>Per-Share ($5.00 basis)</th></tr></thead>\n",
    );
    out.push_str("<tbody>\n");
    out.push_str(&format!(
        "<tr><td>Y10 Portfolio NAV</td><td>{}</td><td>{}</td></tr>\n",
        fmt_currency(block.portfolio_nav_total),
        fmt_currency(block.portfolio_nav_per_share),
    ));
    out.push_str(&format!(
        "<tr><td>Invested Capital (Y0 paid in)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_currency(block.total_invested_capital),
        fmt_currency(block.per_share_invested_capital),
    ));
    out.push_str(&format!(
        "<tr><td><strong>MOIC</strong></td><td><strong>{}</strong></td>\
         <td><strong>{}</strong></td></tr>\n",
        fmt_moic(block.moic_aggregate),
        fmt_moic(block.moic_per_share),
    ));
    out.push_str(&format!(
        "<tr><td>Nominal CAGR (Y0&rarr;Y10)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_pct(block.cagr_y10),
        fmt_pct(block.cagr_y10),
    ));
    out.push_str("</tbody>\n</table>\n");
    out.push_str(
        "<p class=\"footnote\"><em>Aggregate and per-share MOIC are mathematically equal; \
         both columns are shown so readers can read the per-share figure in context of the \
         entity's nominal $10 paid-in share capital.</em></p>\n",
    );
    out.push_str("</section>\n");
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel::wcp::{WcpBook, WcpData, WcpFairDiv, WcpIncome, WcpMarket};
    use crate::spv::bencal::{compute_block_f, BlockF};

    fn synthetic(nav_y10: f64) -> WcpData {
        let book = WcpBook {
            cumulative_fcf_wci: [0.0; 10],
            beneficial_ownership_lps: [0.0; 10],
            book_value: {
                let mut a = [0.0; 10];
                a[9] = nav_y10;
                a
            },
            book_value_per_share: [0.0; 10],
        };
        WcpData {
            title: "t".to_string(),
            entity: "Bencal Management Corp.".to_string(),
            date: "Y0".to_string(),
            shares_outstanding: 2.0,
            price_per_share: 5.00,
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
    fn markdown_contains_both_moic_columns() {
        let data = synthetic(17_500_000.0);
        let bf: BlockF = compute_block_f(&data);
        let md = render(&bf);
        assert!(md.contains("Aggregate"));
        assert!(md.contains("Per-Share"));
        assert!(md.contains("MOIC"));
        // header-note text appears
        assert!(md.contains("$5.00"));
    }

    #[test]
    fn html_contains_block_f_section() {
        let data = synthetic(17_500_000.0);
        let bf: BlockF = compute_block_f(&data);
        let html = render_html(&bf);
        assert!(html.contains("<section class=\"block-f\">"));
        assert!(html.contains("MOIC"));
        assert!(html.contains("Aggregate"));
        assert!(html.contains("Per-Share"));
    }
}
