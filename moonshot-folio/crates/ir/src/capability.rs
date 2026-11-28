// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Capability matrix: for every IR feature and output target, is it native, degraded or rejected.
//! [`cell`] matches `Feature` exhaustively, so adding a feature fails to compile until a row is
//! written for every target.

use crate::model::{Block, Body, CellValue, Document, FieldKind, ImageFormat, Inline};
use moonshot_folio_core::{FolioError, Result};

/// An output format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// PDF (layout writer).
    Pdf,
    /// `WordprocessingML` (flow writer).
    Docx,
    /// HTML (flow writer).
    Html,
    /// `SpreadsheetML`.
    Xlsx,
    /// RFC 4180 CSV.
    Csv,
}

/// How a target treats a feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Represented faithfully.
    Native,
    /// Represented with loss; the reason is reported to the caller.
    Degraded(&'static str),
    /// Cannot be represented; rendering must fail.
    Rejected(&'static str),
}

/// A feature a document can use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feature {
    /// Paragraphs, headings and styled runs.
    Text,
    /// Lists.
    List,
    /// Flow tables.
    Table,
    /// Column spans greater than one.
    ColumnSpan,
    /// Repeated header rows.
    RepeatHeader,
    /// PNG images.
    ImagePng,
    /// JPEG images.
    ImageJpeg,
    /// Page and total-page fields.
    PageFields,
    /// Table of contents field.
    Toc,
    /// Hyperlinks and anchors.
    Links,
    /// Page and line breaks.
    Breaks,
    /// A workbook body.
    Workbook,
    /// A flow body.
    FlowBody,
    /// Numeric cells.
    NumberCell,
}

/// Support for `(feature, target)`, in the order Pdf, Docx, Html, Xlsx, Csv.
fn cell(f: Feature) -> [Support; 5] {
    use Support::{Degraded as D, Native as N, Rejected as R};
    let flow_only = R("flow content has no sheet form");
    match f {
        Feature::Text | Feature::List | Feature::Links => [N, N, N, flow_only, flow_only],
        Feature::ImagePng | Feature::ImageJpeg => [
            N,
            R("DOCX images are not written yet"),
            N,
            flow_only,
            flow_only,
        ],
        Feature::Table => [N, N, N, D("table becomes a sheet region"), D("rows only")],
        Feature::ColumnSpan => [N, N, N, N, R("CSV has no merged cells")],
        Feature::RepeatHeader => [N, N, D("not repeated in HTML"), N, D("header row only")],
        Feature::PageFields => [N, N, D("no pages in HTML"), flow_only, flow_only],
        Feature::Toc => [
            N,
            D("field updated on open"),
            D("static list"),
            flow_only,
            flow_only,
        ],
        Feature::Breaks => [N, N, D("page break is a CSS hint"), flow_only, flow_only],
        Feature::Workbook => [
            R("sheets have no page form yet"),
            R("no sheet form"),
            N,
            N,
            N,
        ],
        Feature::FlowBody => [N, N, N, R("no flow form"), R("no flow form")],
        Feature::NumberCell => [N, N, N, N, N],
    }
}

/// Look up one cell of the matrix.
#[must_use]
pub fn support(f: Feature, t: Target) -> Support {
    let idx = match t {
        Target::Pdf => 0,
        Target::Docx => 1,
        Target::Html => 2,
        Target::Xlsx => 3,
        Target::Csv => 4,
    };
    cell(f)
        .get(idx)
        .copied()
        .unwrap_or(Support::Rejected("unknown target"))
}

/// The features a document uses, without duplicates, in a fixed order.
#[must_use]
pub fn features(doc: &Document) -> Vec<Feature> {
    let mut out = Vec::new();
    let mut add = |f: Feature| {
        if !out.contains(&f) {
            out.push(f);
        }
    };
    match &doc.body {
        Body::Flow(sections) => {
            add(Feature::FlowBody);
            for b in sections
                .iter()
                .flat_map(|s| s.blocks.iter().chain(&s.header).chain(&s.footer))
            {
                block_features(b, &mut add);
            }
        }
        Body::Workbook(sheets) => {
            add(Feature::Workbook);
            for c in sheets.iter().flat_map(|s| &s.rows).flat_map(|r| &r.cells) {
                if matches!(c.value, CellValue::Number(_)) {
                    add(Feature::NumberCell);
                }
                if c.span > 1 {
                    add(Feature::ColumnSpan);
                }
            }
        }
    }
    out
}

fn block_features(b: &Block, add: &mut impl FnMut(Feature)) {
    match b {
        Block::Para(p) => inline_features(&p.inlines, add),
        Block::Heading(h) => inline_features(&h.inlines, add),
        Block::List(l) => {
            add(Feature::List);
            l.items.iter().for_each(|i| inline_features(i, add));
        }
        Block::Table(t) => {
            add(Feature::Table);
            if t.repeat_header_rows > 0 {
                add(Feature::RepeatHeader);
            }
            if t.rows.iter().flat_map(|r| &r.cells).any(|c| c.span > 1) {
                add(Feature::ColumnSpan);
            }
        }
        Block::Image(i) => add(match i.format {
            ImageFormat::Png => Feature::ImagePng,
            ImageFormat::Jpeg => Feature::ImageJpeg,
        }),
        Block::Field(k) => add(field_feature(*k)),
        Block::Break(_) => add(Feature::Breaks),
    }
}

fn field_feature(k: FieldKind) -> Feature {
    match k {
        FieldKind::Page | FieldKind::NumPages => Feature::PageFields,
        FieldKind::Toc => Feature::Toc,
    }
}

fn inline_features(inlines: &[Inline], add: &mut impl FnMut(Feature)) {
    add(Feature::Text);
    for i in inlines {
        match i {
            Inline::Link { .. } | Inline::Anchor(_) => add(Feature::Links),
            Inline::FieldRef(k) => add(field_feature(*k)),
            Inline::Text(_) | Inline::Run { .. } | Inline::Highlight(_) => {}
        }
    }
}

/// Check a document against a target. Returns the degradation notes, or an error naming the
/// first rejected feature.
///
/// # Errors
/// `Unsupported` when any used feature is `Rejected` for `target`.
pub fn check_target(doc: &Document, target: Target) -> Result<Vec<&'static str>> {
    let mut notes = Vec::new();
    for f in features(doc) {
        match support(f, target) {
            Support::Native => {}
            Support::Degraded(why) => notes.push(why),
            Support::Rejected(why) => return Err(FolioError::Unsupported(why)),
        }
    }
    Ok(notes)
}
