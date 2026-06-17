// 10-Year Financial Forecast — classic audited-statement renderer (jurisdiction-parameterized).
//
// Consumes the IFRS-18-shaped model in `src/spv/statutory_forecast.rs` and emits a print-first,
// self-contained document in the form of a TRUE FINANCIAL STATEMENT, matching the Big-4 sample
// (`inputs/CORPORATE_Woodfine LPs_Forecast_Financial_10 Year.pdf`):
//
//   cover (portrait) → four forecasted statements (LANDSCAPE, IFRS 18 structure) →
//   supplementary per-unit schedule (landscape) → notes (portrait).
//
// Classic conventions (research-backed; see BRIEF): serif, pure black-and-white, figures expressed
// in thousands with comma separators, parentheses for negatives, en-dash for nil, accounting rules
// (single line above subtotals, double rule below totals, NO fills), "Projected 1…10 / $" column
// heads, bold-underlined section captions. No practitioner's report is rendered (the forecast is
// management-prepared, matching the sample). All numbers are engine-computed.

use crate::spv::statutory_forecast::{
    self, Jurisdiction, LineFormat, LineStyle, Note, StatementLine,
};

// ─── Number formatting (thousands; parentheses negatives; en-dash nil) ───────

fn group(n: i64) -> String {
    let s = n.abs().to_string();
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in b.iter().enumerate() {
        if i > 0 && (b.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*c as char);
    }
    out
}

/// Dollar amounts presented in thousands (Big-4/IB convention): value/1000, rounded, grouped.
fn fmt_thousands(v: f64) -> String {
    let t = (v / 1000.0).round();
    if t.abs() < 0.5 {
        return "&ndash;".to_string();
    }
    let g = group(t as i64);
    if t < 0.0 {
        format!("({g})")
    } else {
        g
    }
}

/// Per-unit amounts stay in dollars and cents (not scaled to thousands).
fn fmt_pu(v: f64) -> String {
    if v.abs() < 1e-4 {
        "&ndash;".to_string()
    } else if v < 0.0 {
        format!("({:.2})", v.abs())
    } else {
        format!("{:.2}", v)
    }
}

fn fmt_pct(v: f64) -> String {
    if v.abs() < 1e-6 {
        "&ndash;".to_string()
    } else {
        format!("{:.1}%", v * 100.0)
    }
}

fn fmt_ratio(v: f64) -> String {
    if v.abs() < 1e-6 {
        "&ndash;".to_string()
    } else {
        format!("{:.2}&times;", v)
    }
}

fn fmt_cell(v: f64, f: LineFormat) -> String {
    match f {
        LineFormat::Dollar => fmt_thousands(v),
        LineFormat::PerUnit => fmt_pu(v),
        LineFormat::Pct => fmt_pct(v),
        LineFormat::Ratio => fmt_ratio(v),
    }
}

// ─── HTML scaffold (classic serif, black-and-white, named pages) ─────────────

fn head(j: &Jurisdiction) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>{term} — 10-Year Financial Forecast</title>
<style>
body{{font-family:"Times New Roman",Georgia,"Liberation Serif",serif;font-size:11pt;color:#000;margin:0}}
.cover{{padding:9cm 3cm}}
.cover h1{{font-size:16pt;font-weight:700;margin:0 0 .4rem}}
.cover p{{font-size:11.5pt;margin:.15rem 0}}
.stmt-head{{border-bottom:1.5pt solid #000;padding-bottom:.25rem;margin-bottom:.7rem}}
.stmt-head .entity{{font-weight:700;font-size:13.5pt}}
.stmt-head .title{{font-size:12pt}}
.stmt-head .sub{{font-size:10.5pt;font-weight:700}}
table{{border-collapse:collapse;width:100%;table-layout:fixed;font-size:9pt;font-variant-numeric:tabular-nums}}
th,td{{padding:1.5pt 5pt;text-align:right;white-space:nowrap;overflow:hidden}}
th.lbl,td.lbl{{text-align:left;white-space:normal;overflow-wrap:break-word}}
thead th{{font-weight:400;border:none;vertical-align:bottom}}
thead th.yh{{font-weight:400}}
td.detail{{padding-left:1.6rem}}
tr.caption td{{text-align:left;padding-top:.4rem}}
.cap{{font-weight:700;text-decoration:underline}}
tr.subtotal td{{border-top:.75pt solid #000;font-weight:700}}
tr.total td{{border-top:1pt solid #000;border-bottom:3pt double #000;font-weight:700}}
.rounding{{font-size:9pt;font-style:italic;margin-top:.3rem}}
.mpm-note{{font-size:9.5pt;margin-top:.3rem}}
.notes h2{{font-size:12.5pt;font-weight:700;border-bottom:1.5pt solid #000;padding-bottom:.2rem}}
.notes h3{{font-size:11pt;font-weight:700;margin:.9rem 0 .2rem}}
.notes p,.notes li{{font-size:10.5pt;text-align:justify;margin:.3rem 0;line-height:1.3}}
.notes .body{{margin-left:1.4rem}}
.footer{{font-size:9pt;margin-top:1.2rem;border-top:.5pt solid #000;padding-top:.3rem;text-align:justify}}
section.stmt{{break-before:page;page:landscape}}
section.notes{{break-before:page}}
@page{{size:letter portrait;margin:2.2cm 2cm 1.8cm 2cm;@bottom-right{{content:counter(page);font-size:9pt;font-family:serif}}}}
@page landscape{{size:letter landscape;margin:1.6cm 2cm;@bottom-right{{content:counter(page);font-size:9pt;font-family:serif}}}}
@media screen{{body{{max-width:1100px;margin:1.5rem auto;padding:0 1rem}}section.stmt,section.notes{{margin-top:2.5rem;border-top:1px dashed #bbb;padding-top:1rem}}}}
</style>
</head>
"#,
        term = j.term_name,
    )
}

fn cover(j: &Jurisdiction) -> String {
    format!(
        "<div class=\"cover\">\n<h1>{legal}</h1>\n\
         <p>10-Year Financial Forecast</p>\n\
         <p>(Expressed in thousands of Canadian dollars)</p>\n\
         <p>&nbsp;</p>\n<p>{term}</p>\n</div>\n",
        legal = j.legal_name,
        term = j.term_name,
    )
}

/// Per-statement header block (entity / statement title / forecast / scale), repeated on each
/// statement page exactly as the Big-4 sample does.
fn stmt_head(j: &Jurisdiction, title: &str) -> String {
    format!(
        "<div class=\"stmt-head\">\n\
         <div class=\"entity\">{legal}</div>\n\
         <div class=\"title\">{title}</div>\n\
         <div class=\"sub\">10-Year Financial Forecast</div>\n\
         <div class=\"sub\">(Expressed in thousands of {ccy} dollars)</div>\n\
         </div>\n",
        legal = j.legal_name,
        title = title,
        ccy = currency_word(j.currency),
    )
}

fn currency_word(code: &str) -> &str {
    match code {
        "CAD" => "Canadian",
        "USD" => "United States",
        "EUR" => "euro",
        "MXN" => "Mexican peso",
        _ => code,
    }
}

fn year_header() -> String {
    let mut s = String::from("<thead>\n<tr><th class=\"lbl\"></th>");
    for _ in 1..=10 {
        s.push_str("<th class=\"yh\">Projected</th>");
    }
    s.push_str("</tr>\n<tr><th class=\"lbl\"></th>");
    for y in 1..=10 {
        s.push_str(&format!("<th class=\"yh\">{y}</th>"));
    }
    s.push_str("</tr>\n<tr><th class=\"lbl\"></th>");
    for _ in 1..=10 {
        s.push_str("<th class=\"yh\">$</th>");
    }
    s.push_str("</tr>\n</thead>\n");
    s
}

fn render_line(line: &StatementLine) -> String {
    match line.style {
        LineStyle::SectionBanner => {
            format!(
                "<tr class=\"caption\"><td colspan=\"11\"><span class=\"cap\">{}</span></td></tr>\n",
                line.label
            )
        }
        _ => {
            let (cls, lblcls) = match line.style {
                LineStyle::Total => (" class=\"total\"", "lbl"),
                LineStyle::Subtotal => (" class=\"subtotal\"", "lbl"),
                _ => ("", "lbl detail"),
            };
            let sup = line
                .note_ref
                .map(|n| format!(" <sup>{n}</sup>"))
                .unwrap_or_default();
            let mut s = format!("<tr{cls}><td class=\"{lblcls}\">{}{sup}</td>", line.label);
            for v in &line.values {
                s.push_str(&format!("<td>{}</td>", fmt_cell(*v, line.format)));
            }
            s.push_str("</tr>\n");
            s
        }
    }
}

fn render_statement(
    j: &Jurisdiction,
    title: &str,
    lines: &[StatementLine],
    rounding: bool,
) -> String {
    let mut s = String::from("<section class=\"stmt\">\n");
    s.push_str(&stmt_head(j, title));
    s.push_str("<table>\n<colgroup><col style=\"width:26%\">");
    for _ in 1..=10 {
        s.push_str("<col style=\"width:7.4%\">");
    }
    s.push_str("</colgroup>\n");
    s.push_str(&year_header());
    s.push_str("<tbody>\n");
    for l in lines {
        s.push_str(&render_line(l));
    }
    s.push_str("</tbody>\n</table>\n");
    if rounding {
        s.push_str("<p class=\"rounding\">Amounts are expressed in thousands and may not sum precisely due to rounding.</p>\n");
    }
    s.push_str("</section>\n");
    s
}

fn render_notes(notes: &[Note]) -> String {
    let mut s = String::from(
        "<section class=\"notes\">\n<h2>Notes to the 10-Year Financial Forecast</h2>\n",
    );
    for n in notes {
        s.push_str(&format!(
            "<h3>{num}.&nbsp;&nbsp;{title}</h3>\n<div class=\"body\">{body}</div>\n",
            num = n.number,
            title = n.title,
            body = n.body_html,
        ));
    }
    s.push_str("</section>\n");
    s
}

fn footer(j: &Jurisdiction) -> String {
    format!(
        "<section class=\"notes\"><p class=\"footer\"><strong>Forward-Looking Information &mdash; \
         notice under applicable securities legislation including {reg}.</strong> This document \
         presents future-oriented financial information prepared by management. All amounts are \
         computed by the tool-proforma-engine from the locked forecast model and are not drawn \
         from any spreadsheet or prior report. Because the forecast is based on assumptions about \
         future events, actual results will vary from the forecast and the variations may be \
         material. This document is prepared for the purpose stated in Note 1 and does not \
         constitute an offering memorandum or an offer to sell or solicitation to buy any \
         security.</p></section>\n",
        reg = j.regulator,
    )
}

// ─── Public renderers ────────────────────────────────────────────────────────

pub fn render_proforma(j: Jurisdiction) -> String {
    let st = statutory_forecast::build(j);
    let mut s = String::new();
    s.push_str(&head(&j));
    s.push_str("<body>\n");
    s.push_str(&cover(&j));
    s.push_str(&render_statement(
        &j,
        "Statement of Forecasted Financial Position",
        &st.financial_position,
        true,
    ));
    s.push_str(&render_statement(
        &j,
        "Statement of Forecasted Net Income and Comprehensive Income",
        &st.comprehensive_income,
        true,
    ));
    s.push_str(&render_statement(
        &j,
        "Statement of Forecasted Changes in Equity",
        &st.changes_in_equity,
        true,
    ));
    s.push_str(&render_statement(
        &j,
        "Statement of Forecasted Cash Flows",
        &st.cash_flows,
        true,
    ));
    let mut sup = render_statement(
        &j,
        "Supplementary Schedule — Fair Value and Net Asset Value (non-statutory)",
        &st.per_unit,
        false,
    );
    sup = sup.replace(
        "</section>\n",
        "<p class=\"mpm-note\">Investment property is carried at cost in the statements above; this \
         exhibit discloses its fair value and the resulting net asset value, as required under IAS 40 \
         (cost model) and IFRS 13. The net asset value, per-unit and ratio measures are management-\
         defined performance measures presented for comparability (per-unit amounts are in dollars, \
         not thousands); they are not a substitute for an IFRS subtotal and are not part of the \
         primary financial statements.</p>\n</section>\n",
    );
    s.push_str(&sup);
    s.push_str(&render_notes(&st.notes));
    s.push_str(&footer(&j));
    s.push_str("</body></html>\n");
    s
}

pub fn render_summary(j: Jurisdiction) -> String {
    let st = statutory_forecast::build(j);
    let mut s = String::new();
    s.push_str(&head(&j));
    s.push_str("<body>\n");
    s.push_str(&cover(&j));
    s.push_str(&render_statement(
        &j,
        "Statement of Forecasted Net Income and Comprehensive Income",
        &st.comprehensive_income,
        true,
    ));
    s.push_str(&render_statement(
        &j,
        "Supplementary Schedule — Fair Value and Net Asset Value (non-statutory)",
        &st.per_unit,
        false,
    ));
    s.push_str(&footer(&j));
    s.push_str("</body></html>\n");
    s
}

pub fn render_json(j: Jurisdiction) -> String {
    let st = statutory_forecast::build(j);
    let derivation = serde_json::json!({
        "engine": "tool-proforma-engine::spv::statutory_forecast",
        "source_model": "pclp1_proforma::forecast() (locked)",
        "framework": format!("{:?}", j.framework),
        "currency": j.currency,
        "legal_name": j.legal_name,
        "term_name": j.term_name,
        "presentation": "Classic financial statement; IFRS 18 categories; IAS 40 fair-value model; figures in thousands",
        "articulation": {
            "investment_property_fv": "asset_value - ending_cash",
            "fv_gain": "(ip_fv_t - ip_fv_{t-1}) - phase_draws_t",
            "comprehensive_income": "ffo + fv_gain",
            "sofp": "assets(asset_value) = liabilities(closing_debt) + equity(nav)",
            "soce": "opening + contributions + comprehensive_income - distributions = nav"
        },
        "assurance_standard": j.assurance_standard,
        "note": "All figures engine-computed; no Excel/sample input. Management-prepared forecast; no practitioner's report rendered."
    });
    crate::spv::audited_json(&st, derivation)
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spv::statutory_forecast::CANADA;

    #[test]
    fn proforma_is_classic_and_complete() {
        let html = render_proforma(CANADA);
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("Woodfine Professional Centres Limited Partnership"));
        assert!(html.contains("Professional Centres Canada LP"));
        // Four statements present.
        assert!(html.contains("Statement of Forecasted Financial Position"));
        assert!(html.contains("Statement of Forecasted Net Income and Comprehensive Income"));
        assert!(html.contains("Statement of Forecasted Changes in Equity"));
        assert!(html.contains("Statement of Forecasted Cash Flows"));
        assert!(html.contains("Notes to the 10-Year Financial Forecast"));
        // Cost-basis statement markers (match the Big-4 sample).
        assert!(html.contains("Investment property, at cost"));
        assert!(html.contains("Revenue from operations"));
        assert!(html.contains("Equity-based compensation"));
        assert!(html.contains("Partners' units"));
        assert!(html.contains("Operating profit"));
        // Fair value / NAV moved to a supplementary non-statutory exhibit.
        assert!(html.contains("Fair Value and Net Asset Value (non-statutory)"));
        assert!(!html.contains("Income continuity entitlement"));
        // Classic format markers.
        assert!(html.contains("Projected"));
        assert!(html.contains("(Expressed in thousands of Canadian dollars)"));
        assert!(html.contains("serif"));
        // Landscape statements + named pages.
        assert!(html.contains("@page landscape"));
        assert!(html.contains("section.stmt"));
        // No dashboard fills, no practitioner's-report box, no "audited" claim.
        assert!(!html.contains("#eef2f7"));
        assert!(!html.contains("#e3edf7"));
        assert!(!html.contains("class=\"assurance\""));
        assert!(!html.contains("Independent Practitioner's Assurance Report"));
        assert!(!html.contains("audited forecast"));
        // No internal codes leak.
        assert!(!html.contains("PCLP1"));
        assert!(!html.contains("PCCL"));
        assert!(html.ends_with("</body></html>\n"));
    }

    #[test]
    fn json_carries_derivation_and_assurance() {
        let j = render_json(CANADA);
        assert!(j.contains("_derivation"));
        assert!(j.contains("IAS 40 fair-value model"));
        // Assurance block retained in the model/JSON even though not rendered.
        assert!(j.contains("assurance"));
    }
}
