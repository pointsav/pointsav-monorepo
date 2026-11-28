// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The document model. Plain data; invariants are enforced by [`crate::validate`].
//! Styles are referenced by role name, never by inline formatting, so a token change
//! restyles every output from one place.

use moonshot_folio_core::Emu;

/// A whole document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    /// Caller-supplied metadata (the engine never reads a clock).
    pub meta: Meta,
    /// The content.
    pub body: Body,
}

/// Document metadata. All values come from the caller, which keeps output deterministic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meta {
    /// Title.
    pub title: String,
    /// Author.
    pub author: String,
    /// Creation date, `YYYY-MM-DD`.
    pub date: String,
    /// BCP 47 language tag, required for tagged PDF.
    pub lang: String,
}

/// The two containers. Flow lowers to PDF, DOCX and HTML; Workbook to XLSX, CSV and HTML.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    /// Paged, flowing content.
    Flow(Vec<Section>),
    /// Named sheets of cells.
    Workbook(Vec<Sheet>),
}

/// A run of pages that share a page setup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    /// Page geometry.
    pub page: PageSetup,
    /// Content blocks, in order.
    pub blocks: Vec<Block>,
    /// Blocks repeated at the top of every page (empty for none).
    pub header: Vec<Block>,
    /// Blocks repeated at the bottom of every page (empty for none).
    pub footer: Vec<Block>,
    /// When true the first page of the section has no header (the footer still shows).
    pub different_first_page: bool,
    /// When true page numbering restarts at 1 in this section (`w:pgNumType`).
    pub restart_page_numbers: bool,
}

impl Section {
    /// A section with no header or footer.
    #[must_use]
    pub fn new(page: PageSetup, blocks: Vec<Block>) -> Self {
        Self {
            page,
            blocks,
            header: Vec::new(),
            footer: Vec::new(),
            different_first_page: false,
            restart_page_numbers: false,
        }
    }
}

/// Page size and a uniform margin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageSetup {
    /// Page width.
    pub width: Emu,
    /// Page height.
    pub height: Emu,
    /// Margin on all four sides.
    pub margin: Emu,
}

/// Pagination hints. They map to `w:keepNext`, `w:widowControl`, `w:pageBreakBefore`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pagination {
    /// Keep with the next block.
    pub keep_with_next: bool,
    /// Avoid widow and orphan lines.
    pub widow_orphan: bool,
    /// Start a new page before this block.
    pub page_break_before: bool,
}

/// A block-level element. `Chart` joins in F8.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    /// A paragraph.
    Para(Para),
    /// A heading, level 1 to 6.
    Heading(Heading),
    /// A list.
    List(List),
    /// A table.
    Table(Table),
    /// A raster image.
    Image(Image),
    /// A field resolved at layout time.
    Field(FieldKind),
    /// An explicit break.
    Break(BreakKind),
}

/// A paragraph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Para {
    /// Style role name.
    pub role: String,
    /// Content.
    pub inlines: Vec<Inline>,
    /// Pagination hints.
    pub pagination: Pagination,
}

/// A heading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heading {
    /// Level, 1 to 6.
    pub level: u8,
    /// Content.
    pub inlines: Vec<Inline>,
    /// Pagination hints.
    pub pagination: Pagination,
}

/// A list. Numbering definitions are referenced, not inlined.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct List {
    /// Numbering definition id.
    pub numbering: u32,
    /// True for numbered, false for bulleted.
    pub ordered: bool,
    /// Items, each a run of inlines.
    pub items: Vec<Vec<Inline>>,
}

/// Inline content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inline {
    /// Plain text.
    Text(String),
    /// Text with a style role.
    Run {
        /// Style role name.
        role: String,
        /// The text.
        text: String,
    },
    /// A hyperlink.
    Link {
        /// Target URI or `#anchor`.
        target: String,
        /// Visible text.
        text: String,
    },
    /// A named anchor.
    Anchor(String),
    /// Highlighted text.
    Highlight(String),
    /// A field reference.
    FieldRef(FieldKind),
}

/// Fields resolved by the layout pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// Current page number.
    Page,
    /// Total page count.
    NumPages,
    /// Table of contents.
    Toc,
}

/// Explicit breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakKind {
    /// Page break.
    Page,
    /// Line break.
    Line,
}

/// A table. Shares [`Cell`] with [`Sheet`], so one data table lowers to PDF and XLSX.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    /// Number of grid columns.
    pub columns: u32,
    /// Leading rows repeated on each page (`w:tblHeader`).
    pub repeat_header_rows: u32,
    /// The rows.
    pub rows: Vec<Row>,
}

/// A table row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// Cells; column spans must sum to the table's columns.
    pub cells: Vec<Cell>,
    /// May the row split across pages (`w:cantSplit` when false).
    pub can_split: bool,
}

/// One cell, in a table or a sheet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The value.
    pub value: CellValue,
    /// Columns covered, at least 1.
    pub span: u32,
    /// Style role name (empty for the default).
    pub role: String,
}

/// Cell content. Numbers are decimal strings formatted by the caller: there is no float.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellValue {
    /// Text.
    Text(String),
    /// A decimal number: optional `-`, digits, optional `.digits`.
    Number(String),
    /// A boolean.
    Bool(bool),
    /// No content.
    Empty,
}

/// A raster image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    /// Alternative text, required for accessibility.
    pub alt_text: String,
    /// Encoding of `data`.
    pub format: ImageFormat,
    /// The encoded file bytes.
    pub data: Vec<u8>,
    /// Display width.
    pub width: Emu,
    /// Display height.
    pub height: Emu,
}

/// Supported image encodings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    /// PNG.
    Png,
    /// JPEG (DCT passthrough).
    Jpeg,
}

/// A worksheet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sheet {
    /// Sheet name, 1 to 31 characters, none of `[]:*?/\`.
    pub name: String,
    /// Number of columns.
    pub columns: u32,
    /// Leading rows frozen in view.
    pub freeze_rows: u32,
    /// Leading columns frozen in view.
    pub freeze_cols: u32,
    /// Rows of cells.
    pub rows: Vec<Row>,
    /// Column widths in hundredths of a character; 0 means the default. May be shorter than `columns`.
    pub col_widths: Vec<u32>,
    /// Print landscape and fit all columns on one page width.
    pub landscape_fit_width: bool,
    /// Workbook-scoped defined names.
    pub names: Vec<DefinedName>,
    /// Data-validation rules.
    pub validations: Vec<Validation>,
    /// Conditional formats.
    pub cond_formats: Vec<CondFormat>,
    /// Formula cells: the formula text; the cell's own value is its mandatory cached result.
    pub formulas: Vec<Formula>,
}

/// A defined name referring to a range on a sheet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinedName {
    /// The name: a letter or underscore, then letters, digits, underscores or dots.
    pub name: String,
    /// First row, 0-based.
    pub row: u32,
    /// First column, 0-based.
    pub col: u32,
    /// Rows covered, at least 1.
    pub rows: u32,
    /// Columns covered, at least 1.
    pub cols: u32,
}

/// A formula attached to the cell at `(row, col)` (0-based).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Formula {
    /// Row, 0-based.
    pub row: u32,
    /// Column, 0-based.
    pub col: u32,
    /// Formula text without the leading `=`, restricted to the whitelist checked by validate.
    pub text: String,
}

/// A rectangle of cells on a sheet (0-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    /// First row.
    pub row: u32,
    /// First column.
    pub col: u32,
    /// Rows covered, at least 1.
    pub rows: u32,
    /// Columns covered, at least 1.
    pub cols: u32,
}

/// What a validated cell may hold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationKind {
    /// One of a fixed list of strings (no comma or quote in an item; 255 characters in all).
    List(Vec<String>),
    /// A whole number between `min` and `max` inclusive.
    WholeNumber {
        /// Smallest allowed value.
        min: i64,
        /// Largest allowed value.
        max: i64,
    },
}

/// A data-validation rule over a range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Validation {
    /// Cells covered.
    pub range: Range,
    /// The rule.
    pub kind: ValidationKind,
}

/// Comparison for a conditional format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
}

/// Apply the style of `role` to cells in `range` whose value compares to `value`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CondFormat {
    /// Cells covered.
    pub range: Range,
    /// Comparison.
    pub op: CmpOp,
    /// Decimal to compare with (plain decimal syntax).
    pub value: String,
    /// Style role applied when the comparison holds.
    pub role: String,
}
