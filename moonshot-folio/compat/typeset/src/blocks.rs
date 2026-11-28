// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Block, table and span conversion.

use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir as ir;
use tool_typeset::{Block, Cell, CellContent, RowKind, Span, Table};

fn role_para(role: &str, text: &str) -> ir::Block {
    ir::Block::Para(ir::Para {
        role: role.into(),
        inlines: vec![ir::Inline::Text(text.into())],
        pagination: ir::Pagination::default(),
    })
}

fn spans(s: &[Span]) -> Vec<ir::Inline> {
    s.iter()
        .map(|x| {
            if x.is_bold() {
                ir::Inline::Run {
                    role: "bold".into(),
                    text: x.text().into(),
                }
            } else {
                ir::Inline::Text(x.text().into())
            }
        })
        .collect()
}

/// Convert one block, appending to `out`.
pub(crate) fn convert_block(
    b: &Block,
    out: &mut Vec<ir::Block>,
    notes: &mut Vec<String>,
) -> Result<()> {
    match b {
        Block::Masthead {
            entity_name,
            statement_title,
            period_label,
        } => {
            out.push(role_para("masthead-entity", entity_name));
            out.push(role_para("masthead-title", statement_title));
            out.push(role_para("masthead-period", period_label));
        }
        Block::CoverTitle {
            entity_name,
            statement_title,
            period_label,
            currency_label,
        } => {
            out.push(role_para("cover-entity", entity_name));
            out.push(role_para("cover-title", statement_title));
            out.push(role_para("cover-period", period_label));
            out.push(role_para("cover-currency", currency_label));
        }
        Block::Heading { level, text } => out.push(ir::Block::Heading(ir::Heading {
            level: (*level).clamp(1, 6),
            inlines: vec![ir::Inline::Text(text.clone())],
            pagination: ir::Pagination::default(),
        })),
        Block::Para { spans: s, .. } => {
            notes.push("paragraph indent and spacing not carried".into());
            out.push(ir::Block::Para(ir::Para {
                role: "body".into(),
                inlines: spans(s),
                pagination: ir::Pagination::default(),
            }));
        }
        Block::Note(s) => out.push(ir::Block::Para(ir::Para {
            role: "note".into(),
            inlines: spans(s),
            pagination: ir::Pagination::default(),
        })),
        Block::Table(t) => out.push(ir::Block::Table(table(t, notes)?)),
        Block::Rule => out.push(role_para("rule", "")),
        Block::Spacer(pt) => {
            notes.push(format!(
                "spacer of {pt} pt carried as an empty `spacer` paragraph"
            ));
            out.push(role_para("spacer", ""));
        }
        Block::PageBreak => out.push(ir::Block::Break(ir::BreakKind::Page)),
        Block::Footer(t) => out.push(role_para("footer", t)),
        Block::Placeholder { kind, caption } => {
            out.push(role_para("placeholder", &format!("{kind}: {caption}")));
        }
        Block::Timeline(_) => {
            notes.push("timeline not converted (charts arrive in F8); placeholder emitted".into());
            out.push(role_para("placeholder", "timeline"));
        }
    }
    Ok(())
}

fn cell(c: &Cell, role: &str) -> ir::Cell {
    let (value, base) = match &c.content {
        CellContent::Text(t) => (ir::CellValue::Text(t.clone()), role),
        CellContent::Figure(t) => (ir::CellValue::Text(t.clone()), "figure"),
        CellContent::Blank => (ir::CellValue::Empty, role),
    };
    let mut r = base.to_string();
    if c.bold {
        r.push_str(" bold");
    }
    if c.indent > 0 {
        r.push_str(" indent");
        r.push_str(&c.indent.to_string());
    }
    ir::Cell {
        value,
        span: u32::from(c.colspan.max(1)),
        role: r.trim().into(),
    }
}

fn row(cells: &[Cell], role: &str, columns: u32, can_split: bool) -> Result<ir::Row> {
    let mut out: Vec<ir::Cell> = cells.iter().map(|c| cell(c, role)).collect();
    let sum = out.iter().fold(0_u32, |a, c| a.saturating_add(c.span));
    if sum > columns {
        return Err(FolioError::Malformed(
            "table row is wider than its column list",
        ));
    }
    for _ in sum..columns {
        out.push(ir::Cell {
            value: ir::CellValue::Empty,
            span: 1,
            role: String::new(),
        });
    }
    Ok(ir::Row {
        cells: out,
        can_split,
    })
}

fn row_role(k: RowKind) -> &'static str {
    match k {
        RowKind::Data => "data",
        RowKind::Subtotal => "subtotal",
        RowKind::Total => "total",
        RowKind::SectionBanner => "banner",
    }
}

fn table(t: &Table, notes: &mut Vec<String>) -> Result<ir::Table> {
    let columns = u32::try_from(t.columns.len()).map_err(|_| FolioError::Limit("columns"))?;
    let mut rows = Vec::new();
    let mut header = 0_u32;
    for h in [&t.header, &t.subheader].into_iter().flatten() {
        rows.push(row(h, "header", columns, false)?);
        header = header.saturating_add(1);
    }
    for r in &t.rows {
        rows.push(row(&r.cells, row_role(r.kind), columns, !t.keep_together)?);
    }
    notes.push("column widths and alignment not carried".into());
    Ok(ir::Table {
        columns,
        repeat_header_rows: if t.repeat_header { header } else { 0 },
        rows,
    })
}
