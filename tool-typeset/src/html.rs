//! HTML backend: renders a Doc to a complete, reviewable HTML document.
//! Plain text, git-diffable -- the reviewable step before a final PDF.

use crate::doc::{Align, Block, CellContent, ColWidth, Doc, Register, RowKind, Span};

pub fn render_html(doc: &Doc) -> String {
    let mut body = String::new();
    // Tab stops exist only in the register that measured one (see pdf.rs's
    // `tab_stop_pt`); everywhere else a tab stays what HTML makes of
    // whitespace, which is what every document written before them expects.
    let tab_stops = doc.page.register == Register::FormalStatement;
    for block in &doc.blocks {
        render_block(block, tab_stops, &mut body);
    }

    // The formal register's overrides are appended rather than branching the
    // whole stylesheet, so the two registers can't silently drift apart on
    // everything they still share.
    let css = match doc.page.register {
        Register::WorkingDocument => STYLE.to_string(),
        Register::FormalStatement => format!("{STYLE}{FORMAL_STYLE}"),
    };
    let body_class = match doc.page.register {
        Register::WorkingDocument => "working-document",
        Register::FormalStatement => "formal-statement",
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>{css}</style>
</head>
<body class="{body_class}">
{body}
</body>
</html>
"#,
        title = escape(doc.page.running_header.as_deref().unwrap_or("Document")),
        css = css,
        body_class = body_class,
        body = body,
    )
}

fn render_block(block: &Block, tab_stops: bool, out: &mut String) {
    match block {
        Block::Masthead {
            entity_name,
            statement_title,
            period_label,
        } => {
            out.push_str(&format!(
                "<div class=\"masthead\"><div class=\"entity\">{}</div><div class=\"title\">{}</div><div class=\"period\">{}</div></div>\n",
                escape(entity_name), escape(statement_title), escape(period_label)
            ));
        }
        Block::CoverTitle {
            entity_name,
            statement_title,
            period_label,
            currency_label,
        } => {
            out.push_str(&format!(
                "<div class=\"cover\"><div class=\"cover-entity\">{}</div><div class=\"cover-title\">{}</div><div class=\"cover-period\">{}</div><div class=\"cover-currency\">{}</div></div>\n",
                escape(entity_name),
                escape(statement_title),
                escape(period_label),
                escape(currency_label)
            ));
        }
        Block::Heading { level, text } => {
            let level = (*level).clamp(1, 6);
            out.push_str(&format!("<h{level}>{}</h{level}>\n", escape(text)));
        }
        Block::Para { spans, style } => {
            // The per-paragraph measurements ride as inline style rather
            // than as classes: they are real measured values off one
            // document, not a fixed vocabulary of paragraph kinds the
            // stylesheet could enumerate. Emitted only when non-default, so
            // an ordinary paragraph is still a bare `<p>`.
            let mut css = String::new();
            if style.space_before_pt != 0.0 {
                css.push_str(&format!("margin-top:{}pt;", style.space_before_pt));
            }
            if let Some(after) = style.space_after_pt {
                css.push_str(&format!("margin-bottom:{after}pt;"));
            }
            if style.left_indent_pt != 0.0 {
                css.push_str(&format!("padding-left:{}pt;", style.left_indent_pt));
            }
            if css.is_empty() {
                out.push_str("<p>");
            } else {
                out.push_str(&format!("<p style=\"{css}\">"));
            }
            render_spans(spans, tab_stops, out);
            out.push_str("</p>\n");
        }
        Block::Note(spans) => {
            out.push_str("<p class=\"note\">");
            render_spans(spans, tab_stops, out);
            out.push_str("</p>\n");
        }
        Block::Table(table) => render_table(table, out),
        Block::Rule => out.push_str("<hr>\n"),
        Block::Spacer(pt) => out.push_str(&format!("<div style=\"height:{pt}pt\"></div>\n")),
        Block::PageBreak => out.push_str("<div class=\"page-break\"></div>\n"),
        Block::Footer(text) => {
            out.push_str(&format!("<div class=\"footer\">{}</div>\n", escape(text)))
        }
        Block::Placeholder { kind, caption } => {
            out.push_str(&format!(
                "<div class=\"placeholder\">[{}: {}]</div>\n",
                escape(kind),
                escape(caption)
            ));
        }
    }
}

fn render_spans(spans: &[Span], tab_stops: bool, out: &mut String) {
    for span in spans {
        let text = escape(span.text());
        // A tab advances to the next default tab stop. HTML has no tab
        // stops, but since a stop is reached by widening what precedes it,
        // giving the text before the tab that stop as a minimum width is
        // the exact same result for the short labels this is for -- and it
        // keeps the tab out of the text itself, where it would collapse.
        let text = if tab_stops && text.contains('\t') {
            let parts: Vec<&str> = text.split('\t').collect();
            let last = parts.len() - 1;
            parts
                .iter()
                .enumerate()
                .map(|(i, part)| {
                    if i == last {
                        part.to_string()
                    } else {
                        format!("<span class=\"tabstop\">{part}</span>")
                    }
                })
                .collect()
        } else {
            text
        };
        if span.is_bold() {
            out.push_str(&format!("<strong>{text}</strong>"));
        } else {
            out.push_str(&text);
        }
    }
}

fn render_table(table: &crate::doc::Table, out: &mut String) {
    out.push_str("<table>\n");
    if let Some(label) = &table.continuation_label {
        out.push_str(&format!(
            "<caption>{} (continued)</caption>\n",
            escape(label)
        ));
    }
    out.push_str("<colgroup>\n");
    for col in &table.columns {
        let width = match col.width {
            ColWidth::Pct(p) => format!("{p}%"),
            ColWidth::Pt(pt) => format!("{pt}pt"),
        };
        out.push_str(&format!("<col style=\"width:{width}\">\n"));
    }
    out.push_str("</colgroup>\n");

    // Mirrors the PDF backend's `rules_from_col`: only cells at or right of
    // this index carry the separator borders, so a rule never runs under the
    // caption text.
    let ruled = |col_idx: usize| match table.rules_from_col {
        Some(first) if col_idx >= first => " ruled",
        // No restriction asked for: emit nothing and let the base
        // stylesheet's own per-row borders cover every cell, exactly as
        // before this option existed.
        _ => "",
    };

    if table.header.is_some() || table.subheader.is_some() {
        out.push_str("<thead>");
        for (line, class) in [
            (&table.header, "header-row"),
            (&table.subheader, "subheader-row"),
        ] {
            let Some(cells) = line else { continue };
            out.push_str(&format!("<tr class=\"{class}\">"));
            let mut col_idx = 0usize;
            for cell in cells {
                out.push_str(&format!(
                    "<th colspan=\"{}\" class=\"{}{}\">{}</th>",
                    cell.colspan,
                    align_class(&table.columns, col_idx),
                    ruled(col_idx),
                    escape(cell.as_str())
                ));
                col_idx += cell.colspan as usize;
            }
            out.push_str("</tr>");
        }
        out.push_str("</thead>\n");
    }

    out.push_str("<tbody>\n");
    for row in &table.rows {
        let row_class = match row.kind {
            RowKind::Data => "data",
            RowKind::Subtotal => "subtotal",
            RowKind::Total => "total",
            RowKind::SectionBanner => "section-banner",
        };
        out.push_str(&format!("<tr class=\"{row_class}\">"));
        let mut col_idx = 0usize;
        for cell in &row.cells {
            let bold = if cell.bold { " bold" } else { "" };
            let figure = if cell.is_figure() { " figure" } else { "" };
            let indent = match cell.indent {
                0 => String::new(),
                n => format!(" indent{}", n.min(2)),
            };
            let content = match &cell.content {
                CellContent::Text(s) | CellContent::Figure(s) => escape(s),
                CellContent::Blank => String::new(),
            };
            out.push_str(&format!(
                "<td colspan=\"{}\" class=\"{}{}{}{}{}\">{}</td>",
                cell.colspan,
                align_class(&table.columns, col_idx),
                bold,
                figure,
                indent,
                ruled(col_idx),
                content
            ));
            col_idx += cell.colspan as usize;
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</tbody>\n</table>\n");
}

fn align_class(columns: &[crate::doc::Column], col_idx: usize) -> &'static str {
    match columns.get(col_idx).map(|c| c.align) {
        Some(Align::Right) => "right",
        Some(Align::Center) => "center",
        _ => "left",
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// Mirrors the PDF backend's type scale, rule hierarchy, one accent color,
// and single section-banner fill (never zebra-striping).
const STYLE: &str = r#"
body { font-family: Helvetica, Arial, sans-serif; font-size: 9pt; color: #111; max-width: 800px; margin: 2em auto; }
.masthead { text-align: center; margin-bottom: 0; padding-bottom: 0.75em; border-bottom: 1.7px solid #1f3a5f; }
.masthead .entity { font-weight: bold; font-size: 14pt; color: #111; }
.masthead .title { font-size: 11pt; margin-top: 0.3em; color: #222; }
.masthead .period { font-style: italic; margin-top: 0.3em; color: #555; }
h1, h2, h3 { font-weight: bold; margin: 1.2em 0 0.4em; }
h1 { font-size: 12.5pt; color: #111; border-bottom: 1px solid #999; padding-bottom: 2px; }
h2 { font-size: 10.5pt; color: #222; }
h3 { font-size: 9.5pt; color: #222; }
table { width: 100%; border-collapse: collapse; margin: 1em 0; table-layout: fixed; }
caption { caption-side: top; text-align: left; font-style: italic; font-size: 8pt; color: #666; padding-bottom: 0.3em; }
th, td { padding: 3px 6px; vertical-align: top; }
th { border-bottom: 1px solid #999; text-align: left; font-size: 8.5pt; color: #222; }
td { font-size: 9pt; }
td.right, th.right { text-align: right; }
td.center, th.center { text-align: center; }
tr.data td { border-bottom: 0.5px solid #e3e3e3; }
tr.subtotal td { border-top: 1px solid #aaa; }
tr.total td { border-top: 1.3px solid #888; border-bottom: 3px double #888; font-weight: bold; }
tr.section-banner td { font-weight: bold; padding-top: 0.75em; background-color: #f2f4f7; }
td.bold, th.bold { font-weight: bold; }
.note { font-size: 8.5pt; font-style: italic; color: #555; }
.footer { font-size: 7.5pt; color: #999; text-align: center; margin-top: 2em; }
.placeholder { border: 1px dashed #666; padding: 1em; text-align: center; color: #666; font-style: italic; margin: 1em 0; }
.page-break { page-break-after: always; }
hr { border: none; border-top: 1px solid #999; }
"#;

// The formal filed-document register, as overrides on top of the shared
// stylesheet above: pure black ink, zero fill, zero accent, rules only over
// the figure columns. Kept as an appended override block rather than a
// second full stylesheet so the two registers cannot drift on what they
// still share.
const FORMAL_STYLE: &str = r#"
body.formal-statement { font-size: 10pt; line-height: 1.35; color: #000; }
.formal-statement .cover { text-align: center; margin: 180pt 0 0; color: #000; }
.formal-statement .cover-entity { font-size: 16pt; font-weight: bold; margin-bottom: 10pt; }
.formal-statement .cover-title { font-size: 14pt; font-weight: bold; margin-bottom: 2pt; }
.formal-statement .cover-period { font-size: 14pt; font-weight: bold; margin-bottom: 30pt; }
.formal-statement .cover-currency { font-size: 10pt; font-style: italic; }
.formal-statement .masthead { text-align: left; border-bottom: 1.44pt solid #000; padding-bottom: 4pt; }
.formal-statement .masthead .entity { font-size: 11pt; font-weight: bold; color: #000; }
.formal-statement .masthead .title { font-size: 11pt; font-weight: bold; color: #000; margin-top: 0; }
.formal-statement .masthead .period { font-size: 10pt; font-style: italic; color: #000; margin-top: 0; }
.formal-statement table { margin: 20pt 0 10pt; }
.formal-statement th, .formal-statement td { padding: 2pt 4pt; font-size: 10pt; color: #000; border: none; }
.formal-statement th { vertical-align: bottom; font-weight: bold; }
.formal-statement tr.header-row th { padding-bottom: 1pt; }
.formal-statement tr.subheader-row th { padding-top: 0; padding-bottom: 3pt; }
.formal-statement tr.subheader-row th.ruled { border-bottom: 1pt solid #000; }
.formal-statement tr.data td { border-bottom: none; }
.formal-statement tr.subtotal td.ruled { border-top: 0.5pt solid #000; }
.formal-statement tr.total td { font-weight: bold; border: none; padding-bottom: 4pt; }
.formal-statement tr.total td.ruled { border-top: 0.5pt solid #000; border-bottom: 3pt double #000; }
.formal-statement tr.section-banner td { background: none; font-weight: bold; padding-top: 12pt; }
.formal-statement td.indent1 { padding-left: 16pt; }
.formal-statement td.indent2 { padding-left: 28pt; }
.formal-statement td.figure { white-space: nowrap; font-variant-numeric: tabular-nums; }
.formal-statement .note { color: #000; }
.formal-statement p { text-align: justify; line-height: 12pt; margin: 0 0 10pt; }
.formal-statement .tabstop { display: inline-block; min-width: 36pt; }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::{Block, PageSetup, ParaStyle, Span};

    fn doc(register: Register, blocks: Vec<Block>) -> Doc {
        Doc {
            page: PageSetup {
                register,
                ..Default::default()
            },
            blocks,
        }
    }

    fn para(text: &str) -> Block {
        Block::para(vec![Span::Text(text.to_string())])
    }

    #[test]
    fn the_formal_register_carries_the_same_measurements_the_pdf_backend_does() {
        // Two backends, one model: the HTML review copy has to show the
        // reviewer the typography the PDF will print, or reviewing it is
        // reviewing a different document.
        let html = render_html(&doc(Register::FormalStatement, vec![para("Body.")]));
        assert!(
            html.contains("text-align: justify"),
            "the formal register justifies its body"
        );
        assert!(
            html.contains("line-height: 12pt"),
            "... at the measured exact 12pt line"
        );
        assert!(
            html.contains("margin: 0 0 10pt"),
            "... with the measured 10pt paragraph gap"
        );
    }

    #[test]
    fn the_working_register_keeps_its_ragged_edge() {
        let html = render_html(&doc(Register::WorkingDocument, vec![para("Body.")]));
        assert!(
            !html.contains("justify"),
            "the working document is never justified"
        );
        assert!(
            html.contains("<p>Body.</p>"),
            "and an ordinary paragraph stays a bare <p>"
        );
    }

    #[test]
    fn a_tab_becomes_a_real_tab_stop_width_rather_than_collapsing() {
        // HTML collapses a raw tab to nothing, so a sub-item caption would
        // sit one space past its label instead of on the 36pt stop.
        let block = Block::Para {
            spans: vec![Span::Bold("(a)\tStatement of compliance".to_string())],
            style: ParaStyle::default(),
        };
        let html = render_html(&doc(Register::FormalStatement, vec![block.clone()]));
        assert!(
            html.contains(
                "<strong><span class=\"tabstop\">(a)</span>Statement of compliance</strong>"
            ),
            "expected a tab-stop span, got: {html}"
        );
        // The register that defines no tab stop leaves the text alone.
        let working = render_html(&doc(Register::WorkingDocument, vec![block]));
        assert!(!working.contains("tabstop"));
    }

    #[test]
    fn measured_paragraph_values_ride_as_inline_style() {
        let html = render_html(&doc(
            Register::FormalStatement,
            vec![Block::Para {
                spans: vec![Span::Text("Defined term.".to_string())],
                style: ParaStyle {
                    left_indent_pt: 14.2,
                    space_before_pt: 2.0,
                    space_after_pt: Some(0.0),
                },
            }],
        ));
        assert!(
            html.contains("padding-left:14.2pt"),
            "the left indent is a real measurement"
        );
        assert!(html.contains("margin-top:2pt"));
        assert!(
            html.contains("margin-bottom:0pt"),
            "a measured zero gap must beat the register's"
        );
    }
}
