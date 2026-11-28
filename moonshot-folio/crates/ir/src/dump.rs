// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Canonical text form of an IR document. One line per node, text in `{:?}` escapes, so the
//! output is byte-stable and suits golden files and diffs.

use crate::model::{
    Block, Body, Cell, CellValue, Document, FieldKind, Inline, Pagination, Row, Section, Sheet,
};
use core::fmt::Write;

/// Render a document as canonical text.
#[must_use]
pub fn dump(doc: &Document) -> String {
    let mut o = String::new();
    let m = &doc.meta;
    line(
        &mut o,
        0,
        format_args!(
            "document title={:?} author={:?} date={} lang={}",
            m.title, m.author, m.date, m.lang
        ),
    );
    match &doc.body {
        Body::Flow(sections) => sections.iter().for_each(|s| section(&mut o, s)),
        Body::Workbook(sheets) => sheets.iter().for_each(|s| sheet(&mut o, s)),
    }
    o
}

fn line(o: &mut String, depth: usize, args: core::fmt::Arguments<'_>) {
    for _ in 0..depth {
        o.push_str("  ");
    }
    o.write_fmt(args).ok();
    o.push('\n');
}

fn section(o: &mut String, s: &Section) {
    let p = &s.page;
    line(
        o,
        1,
        format_args!(
            "section width={} height={} margin={}",
            p.width.raw(),
            p.height.raw(),
            p.margin.raw()
        ),
    );
    if s.different_first_page {
        line(o, 2, format_args!("different_first_page"));
    }
    if s.restart_page_numbers {
        line(o, 2, format_args!("restart_page_numbers"));
    }
    for (label, v) in [("header", &s.header), ("footer", &s.footer)] {
        if !v.is_empty() {
            line(o, 2, format_args!("{label}"));
            for b in v {
                block(o, b);
            }
        }
    }
    for b in &s.blocks {
        block(o, b);
    }
}

fn block(o: &mut String, b: &Block) {
    match b {
        Block::Para(p) => line(
            o,
            2,
            format_args!(
                "para role={:?}{} [{}]",
                p.role,
                pag(p.pagination),
                inlines(&p.inlines)
            ),
        ),
        Block::Heading(h) => line(
            o,
            2,
            format_args!(
                "heading level={}{} [{}]",
                h.level,
                pag(h.pagination),
                inlines(&h.inlines)
            ),
        ),
        Block::List(l) => {
            line(
                o,
                2,
                format_args!("list numbering={} ordered={}", l.numbering, l.ordered),
            );
            for i in &l.items {
                line(o, 3, format_args!("item [{}]", inlines(i)));
            }
        }
        Block::Table(t) => {
            line(
                o,
                2,
                format_args!(
                    "table columns={} repeat_header_rows={}",
                    t.columns, t.repeat_header_rows
                ),
            );
            t.rows.iter().for_each(|r| row(o, 3, r));
        }
        Block::Image(i) => line(
            o,
            2,
            format_args!(
                "image format={:?} bytes={} width={} height={} alt={:?}",
                i.format,
                i.data.len(),
                i.width.raw(),
                i.height.raw(),
                i.alt_text
            ),
        ),
        Block::Field(k) => line(o, 2, format_args!("field {}", field(*k))),
        Block::Break(k) => line(o, 2, format_args!("break {k:?}")),
    }
}

fn sheet(o: &mut String, s: &Sheet) {
    line(
        o,
        1,
        format_args!(
            "sheet name={:?} columns={} freeze_rows={} freeze_cols={} widths={:?} landscape_fit_width={}",
            s.name, s.columns, s.freeze_rows, s.freeze_cols, s.col_widths, s.landscape_fit_width
        ),
    );
    for n in &s.names {
        line(
            o,
            2,
            format_args!(
                "name {} row={} col={} rows={} cols={}",
                n.name, n.row, n.col, n.rows, n.cols
            ),
        );
    }
    for v in &s.validations {
        line(o, 2, format_args!("validation {:?} {:?}", v.range, v.kind));
    }
    for c in &s.cond_formats {
        line(
            o,
            2,
            format_args!(
                "condformat {:?} {:?} {} role={:?}",
                c.range, c.op, c.value, c.role
            ),
        );
    }
    for f in &s.formulas {
        line(
            o,
            2,
            format_args!("formula row={} col={} {:?}", f.row, f.col, f.text),
        );
    }
    for r in &s.rows {
        row(o, 2, r);
    }
}

fn row(o: &mut String, depth: usize, r: &Row) {
    let cells: Vec<String> = r.cells.iter().map(cell).collect();
    line(
        o,
        depth,
        format_args!("row split={} | {}", r.can_split, cells.join(" | ")),
    );
}

fn cell(c: &Cell) -> String {
    let v = match &c.value {
        CellValue::Text(t) => format!("text {t:?}"),
        CellValue::Number(n) => format!("num {n}"),
        CellValue::Bool(b) => format!("bool {b}"),
        CellValue::Empty => "empty".to_string(),
    };
    format!("{v} span={} role={:?}", c.span, c.role)
}

fn pag(p: Pagination) -> String {
    let mut s = String::new();
    if p.keep_with_next {
        s.push_str(" keep_with_next");
    }
    if p.widow_orphan {
        s.push_str(" widow_orphan");
    }
    if p.page_break_before {
        s.push_str(" page_break_before");
    }
    s
}

fn field(k: FieldKind) -> &'static str {
    match k {
        FieldKind::Page => "page",
        FieldKind::NumPages => "numpages",
        FieldKind::Toc => "toc",
    }
}

fn inlines(v: &[Inline]) -> String {
    let parts: Vec<String> = v
        .iter()
        .map(|i| match i {
            Inline::Text(t) => format!("t{t:?}"),
            Inline::Run { role, text } => format!("run({role:?}){text:?}"),
            Inline::Link { target, text } => format!("link({target:?}){text:?}"),
            Inline::Anchor(a) => format!("anchor{a:?}"),
            Inline::Highlight(t) => format!("hl{t:?}"),
            Inline::FieldRef(k) => format!("field:{}", field(*k)),
        })
        .collect();
    parts.join(" ")
}
