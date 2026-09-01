//! HTML backend: renders a Doc to a complete, reviewable HTML document.
//! Plain text, git-diffable -- the reviewable step before a final PDF.

use crate::doc::{
    Align, BarStyle, Block, CellContent, ColWidth, Doc, Mark, MilestoneShape, Register, RowKind,
    Span, Timeline,
};

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
    let mut css = match doc.page.register {
        Register::WorkingDocument => STYLE.to_string(),
        Register::FormalStatement => format!("{STYLE}{FORMAL_STYLE}"),
    };
    // HTML has no page concept to swap width/height on the way pdf.rs's
    // Layout::new does -- the print-time page-size hint is the equivalent.
    if doc.page.landscape {
        css.push_str("\n@media print { @page { size: landscape; } }\n");
    }
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
        Block::Timeline(timeline) => render_timeline(timeline, out),
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
            let align = align_class(&table.columns, col_idx);
            // A measured inline value, not an enumerated class vocabulary --
            // the same rationale ParaStyle's left_indent_pt already applies.
            // Only left-aligned cells indent (mirrors pdf.rs's
            // place_cell_text: indenting a right-aligned figure would move
            // it off the figure rail its whole column aligns to).
            let indent_style = if cell.indent > 0 && align == "left" {
                format!(" style=\"padding-left:{}pt\"", indent_pt(cell.indent))
            } else {
                String::new()
            };
            let content = match &cell.content {
                CellContent::Text(s) | CellContent::Figure(s) => escape(s),
                CellContent::Blank => String::new(),
            };
            out.push_str(&format!(
                "<td colspan=\"{}\" class=\"{}{}{}{}\"{}>{}</td>",
                cell.colspan,
                align,
                bold,
                figure,
                ruled(col_idx),
                indent_style,
                content
            ));
            col_idx += cell.colspan as usize;
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</tbody>\n</table>\n");
}

/// A CSS grid: leader panel in the first track (the real `Table`, dropped in
/// via the existing `render_table` unchanged), a plotted panel in the
/// second. Each mark is absolutely positioned by percentage from
/// `Timeline::fraction()` -- real fractions, not grid-snapped, so unequal
/// quarters/months land correctly. Row alignment between the leader
/// `<table>`'s `<tr>`s and the plot's own row divs is approximate (a CSS
/// custom property sets both to the same nominal height) -- exact to the pt
/// is the PDF backend's job; this is the reviewable copy.
fn render_timeline(timeline: &Timeline, out: &mut String) {
    let Some(leader) = &timeline.leader else {
        return;
    };
    let leader_pct = match timeline.leader_width {
        ColWidth::Pct(p) => p,
        // HTML has no exact pt-to-grid-track equivalent to pdf.rs's
        // absolute geometry; a generous default keeps the plot panel usable.
        ColWidth::Pt(_) => 30.0,
    };
    let row_h = timeline.row_height_pt.unwrap_or(20.0);

    out.push_str(&format!(
        "<div class=\"timeline\" style=\"--tl-row-h:{row_h}pt;grid-template-columns:{leader_pct}% 1fr;\">\n"
    ));

    out.push_str("<div class=\"tl-leader\">\n");
    render_table(leader, out);
    out.push_str("</div>\n<div class=\"tl-plot\">\n");

    let band_levels = timeline
        .axis
        .bands
        .iter()
        .map(|b| b.level)
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    if band_levels > 0 {
        out.push_str("<div class=\"tl-axis-header\">\n");
        for band in &timeline.axis.bands {
            let x1 = timeline.fraction(band.start).value * 100.0;
            let x2 = timeline.fraction(band.end).value * 100.0;
            out.push_str(&format!(
                "<div class=\"tl-band\" style=\"left:{x1}%;width:{}%;top:calc(var(--tl-row-h) * {})\">{}</div>\n",
                (x2 - x1).max(0.0),
                band.level,
                escape(&band.label)
            ));
        }
        out.push_str("</div>\n");
    }

    out.push_str("<div class=\"tl-rows\">\n");
    let n = leader.rows.len().min(timeline.rows.len());
    for i in 0..n {
        out.push_str("<div class=\"tl-row\">\n");
        for &g in &timeline.axis.gridlines {
            let x = timeline.fraction(g).value * 100.0;
            out.push_str(&format!(
                "<div class=\"tl-gridline\" style=\"left:{x}%\"></div>\n"
            ));
        }
        if let Some(marker) = &timeline.axis.marker {
            let x = timeline.fraction(marker.at).value * 100.0;
            out.push_str(&format!(
                "<div class=\"tl-marker\" style=\"left:{x}%\"></div>\n"
            ));
        }
        for mark in &timeline.rows[i].marks {
            render_mark(mark, timeline, out);
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n");

    if !timeline.legend.is_empty() {
        out.push_str("<div class=\"tl-legend\">\n");
        for entry in &timeline.legend {
            let class = legend_sample_class(&entry.sample);
            out.push_str(&format!(
                "<span class=\"tl-legend-item\"><span class=\"tl-legend-swatch {class}\"></span>{}</span>\n",
                escape(&entry.caption)
            ));
        }
        out.push_str("</div>\n");
    }

    out.push_str("</div>\n</div>\n");
}

fn legend_sample_class(sample: &crate::doc::LegendSample) -> &'static str {
    match sample {
        crate::doc::LegendSample::Bar(BarStyle::Task) => "tl-bar--task",
        crate::doc::LegendSample::Bar(BarStyle::Summary) => "tl-bar--summary",
        crate::doc::LegendSample::Bar(BarStyle::External) => "tl-bar--external",
        crate::doc::LegendSample::Milestone(MilestoneShape::Diamond) => "tl-ms--diamond",
        crate::doc::LegendSample::Milestone(MilestoneShape::Triangle) => "tl-ms--triangle",
        crate::doc::LegendSample::Milestone(MilestoneShape::Bar) => "tl-ms--bar",
    }
}

fn render_mark(mark: &Mark, timeline: &Timeline, out: &mut String) {
    match mark {
        Mark::Bar {
            start,
            end,
            style,
            progress,
            label,
        } => {
            let x1 = timeline.fraction(*start).value * 100.0;
            let x2 = timeline.fraction(*end).value * 100.0;
            let class = match style {
                BarStyle::Task => "tl-bar--task",
                BarStyle::Summary => "tl-bar--summary",
                BarStyle::External => "tl-bar--external",
            };
            out.push_str(&format!(
                "<div class=\"tl-bar {class}\" style=\"left:{x1}%;width:{}%\">",
                (x2 - x1).max(0.1)
            ));
            if let Some(p) = progress {
                out.push_str(&format!(
                    "<div class=\"tl-progress\" style=\"width:{}%\"></div>",
                    p.clamp(0.0, 1.0) * 100.0
                ));
            }
            out.push_str("</div>\n");
            render_mark_label(label, x1, x2, out);
        }
        Mark::Milestone { at, shape, label } => {
            let x = timeline.fraction(*at).value * 100.0;
            let class = match shape {
                MilestoneShape::Diamond => "tl-ms--diamond",
                MilestoneShape::Triangle => "tl-ms--triangle",
                MilestoneShape::Bar => "tl-ms--bar",
            };
            out.push_str(&format!(
                "<div class=\"tl-milestone {class}\" style=\"left:{x}%\"></div>\n"
            ));
            render_mark_label(label, x, x, out);
        }
    }
}

fn render_mark_label(label: &Option<crate::doc::MarkLabel>, x1: f64, x2: f64, out: &mut String) {
    let Some(lbl) = label else { return };
    let (side, pos) = match lbl.placement {
        crate::doc::LabelPlacement::Before => ("right", 100.0 - x1),
        crate::doc::LabelPlacement::After => ("left", x2),
        crate::doc::LabelPlacement::Inside => ("left", (x1 + x2) / 2.0),
    };
    let bold = if lbl.bold { " bold" } else { "" };
    out.push_str(&format!(
        "<div class=\"tl-mark-label{bold}\" style=\"{side}:{pos}%\">{}</div>\n",
        escape(&lbl.text)
    ));
}

fn align_class(columns: &[crate::doc::Column], col_idx: usize) -> &'static str {
    match columns.get(col_idx).map(|c| c.align) {
        Some(Align::Right) => "right",
        Some(Align::Center) => "center",
        _ => "left",
    }
}

/// Matches pdf.rs's `Style::indent_pt` step exactly, so the HTML review copy
/// shows the same indent the PDF will print. 5 entries (0..=4); levels
/// beyond the array's length clamp onto its last entry.
const INDENT_PT: [f32; 5] = [0.0, 16.0, 28.0, 40.0, 52.0];
fn indent_pt(level: u8) -> f32 {
    INDENT_PT[(level as usize).min(INDENT_PT.len() - 1)]
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

/* Timeline: a Gantt-style bar/calendar-grid block. Colors are part of what
   BarStyle/MilestoneShape mean, not a register accent -- shared by both
   registers, unlike everything above this comment. */
.timeline { display: grid; margin: 1em 0; align-items: start; }
.tl-leader table { margin: 0; }
.tl-leader tr { height: var(--tl-row-h, 20pt); }
.tl-plot { position: relative; }
.tl-axis-header { position: relative; height: calc(var(--tl-row-h, 20pt) * 2); font-size: 8.5pt; font-weight: bold; color: #222; }
.tl-band { position: absolute; height: var(--tl-row-h, 20pt); text-align: center; border-left: 1px solid #ccc; box-sizing: border-box; padding-top: 2px; }
.tl-rows { position: relative; }
.tl-row { position: relative; height: var(--tl-row-h, 20pt); border-bottom: 0.5px solid #eee; }
.tl-gridline { position: absolute; top: 0; bottom: 0; width: 0; border-left: 1px solid #d9d9d9; }
.tl-marker { position: absolute; top: 0; bottom: 0; width: 0; border-left: 1.5px dashed #cc1a1a; }
.tl-bar { position: absolute; top: 25%; height: 50%; border-radius: 2px; background: #1f3a5f; overflow: hidden; }
.tl-bar--task { background: #1f3a5f; }
.tl-bar--summary { background: #4d4d4d; height: 30%; top: 35%; border-radius: 0; }
.tl-bar--external { background: #a6a6a6; }
.tl-progress { position: absolute; top: 0; left: 0; bottom: 0; background: #0f1c30; }
.tl-milestone { position: absolute; top: 15%; width: 0; height: 70%; }
.tl-ms--diamond { border-left: 6px solid transparent; border-right: 6px solid transparent; background: none; border-top: 6px solid #cc4c0d; border-bottom: 6px solid #cc4c0d; box-sizing: border-box; transform: translateX(-6px); width: 12px; }
.tl-ms--triangle { border-left: 6px solid transparent; border-right: 6px solid transparent; border-bottom: 10px solid #cc4c0d; background: none; transform: translateX(-6px); width: 12px; height: 10px; }
.tl-ms--bar { width: 2px; background: #cc4c0d; transform: translateX(-1px); }
.tl-mark-label { position: absolute; top: 30%; font-size: 8pt; color: #333; white-space: nowrap; }
.tl-mark-label.bold { font-weight: bold; }
.tl-legend { margin-top: 6pt; font-size: 8pt; color: #333; }
.tl-legend-item { margin-right: 14pt; }
.tl-legend-swatch { display: inline-block; width: 14px; height: 8px; margin-right: 3px; vertical-align: middle; border-radius: 2px; }
.tl-legend-swatch.tl-bar--task { background: #1f3a5f; }
.tl-legend-swatch.tl-bar--summary { background: #4d4d4d; }
.tl-legend-swatch.tl-bar--external { background: #a6a6a6; }
.tl-legend-swatch.tl-ms--diamond, .tl-legend-swatch.tl-ms--triangle, .tl-legend-swatch.tl-ms--bar { background: #cc4c0d; width: 8px; height: 8px; border-radius: 50%; }
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

    #[test]
    fn landscape_emits_the_print_page_size_hint() {
        let portrait = render_html(&doc(Register::WorkingDocument, vec![]));
        assert!(!portrait.contains("size: landscape"));

        let landscape_doc = Doc {
            page: PageSetup {
                landscape: true,
                ..Default::default()
            },
            blocks: vec![],
        };
        let landscape = render_html(&landscape_doc);
        assert!(
            landscape.contains("@media print") && landscape.contains("size: landscape"),
            "expected a print page-size hint when landscape is set"
        );
    }

    #[test]
    fn indent_level_3_gets_its_own_measured_offset_not_a_missing_class() {
        use crate::doc::{Cell, ColWidth, Column, Row, Table};
        let mut table = Table::new(vec![Column {
            width: ColWidth::Pct(100.0),
            align: crate::doc::Align::Left,
        }]);
        table.rows = vec![Row::data(vec![Cell::text("Deep item").indent(3)])];
        let html = render_html(&doc(Register::WorkingDocument, vec![Block::Table(table)]));
        assert!(
            html.contains("padding-left:40pt"),
            "level 3 must carry its own inline offset (was previously silently dropped: \
             no indent3 class existed in either register's stylesheet), got: {html}"
        );
    }

    #[test]
    fn a_timeline_renders_its_leader_and_a_positioned_bar() {
        use crate::doc::{
            AxisBand, BarStyle, Cell, ColWidth as CW, Column, Mark, Row, Table, TimeAxis, Timeline,
            TimelineRow,
        };
        let mut leader = Table::new(vec![Column {
            width: CW::Pct(100.0),
            align: crate::doc::Align::Left,
        }]);
        leader.rows.push(Row::data(vec![Cell::text("Task A")]));
        let axis = TimeAxis {
            start: 0.0,
            end: 100.0,
            bands: vec![AxisBand {
                label: "Q1".into(),
                start: 0.0,
                end: 100.0,
                level: 0,
            }],
            gridlines: vec![50.0],
            marker: None,
        };
        let rows = vec![TimelineRow::one(Mark::Bar {
            start: 10.0,
            end: 60.0,
            style: BarStyle::Task,
            progress: None,
            label: None,
        })];
        let timeline = Timeline::with_leader(leader, CW::Pct(30.0), axis, rows).unwrap();
        let html = render_html(&doc(
            Register::WorkingDocument,
            vec![Block::Timeline(timeline)],
        ));
        assert!(html.contains("Task A"), "the leader table must render");
        assert!(html.contains("class=\"tl-bar tl-bar--task\""));
        assert!(
            html.contains("left:10%"),
            "the bar's start fraction must reach the page"
        );
    }
}
