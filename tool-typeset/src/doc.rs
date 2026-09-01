//! The Document model: one internal tree, rendered by two independent
//! backends (html.rs, pdf.rs) -- never HTML-to-PDF conversion. Cells carry
//! pre-formatted strings, never Money/f64 -- formatting is each caller's
//! own job, which is what keeps this crate genuinely domain-agnostic.

#[derive(Debug, Clone)]
pub struct Doc {
    pub page: PageSetup,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct PageSetup {
    pub size: PageSize,
    pub landscape: bool,
    pub margin_pt: f32,
    pub running_header: Option<String>,
    pub page_numbers: bool,
    pub suppress_header_on_first: bool,
    pub register: Register,
}

impl Default for PageSetup {
    fn default() -> Self {
        PageSetup {
            size: PageSize::Letter,
            landscape: false,
            margin_pt: 72.0, // 1in
            running_header: None,
            page_numbers: true,
            suppress_header_on_first: true,
            register: Register::WorkingDocument,
        }
    }
}

/// Which *visual register* a document is rendered in. A working document is
/// read on screen by one person and wants scanning aids, while a filed
/// formal document is a legal artifact whose whole convention is that
/// nothing decorative competes with the figures -- genuinely different
/// documents, not a theme preference.
///
/// This is one enum rather than two renderers because everything *else*
/// about the two -- flow, page breaking, repeated headers, column geometry
/// -- is identical, and a forked pdf.rs would drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Register {
    /// Working document (detail listings, working papers, event registers):
    /// one restrained accent on the masthead rule, one light section-banner
    /// fill, a hairline under every data row, near-black ink.
    #[default]
    WorkingDocument,
    /// Formal filed statement: pure black ink, zero fill, zero accent, and
    /// horizontal rules only where a real audited/filed document draws them
    /// -- under the column headers, above subtotals, and above plus doubled
    /// below grand totals. Never under an ordinary data row.
    FormalStatement,
}

#[derive(Debug, Clone, Copy)]
pub enum PageSize {
    Letter,
    /// 11x17in -- landscape-usable for a wide task list plus a plotted
    /// timeline that `Letter` is too narrow for.
    Tabloid,
    Legal,
}

impl PageSize {
    /// (width, height) in PDF points, portrait orientation.
    pub fn dims_pt(&self) -> (f32, f32) {
        match self {
            PageSize::Letter => (612.0, 792.0),
            PageSize::Tabloid => (792.0, 1224.0),
            PageSize::Legal => (612.0, 1008.0),
        }
    }
}

/// A run of paragraph text. A literal tab character inside the text is a
/// real tab: in a register that defines a tab stop it advances the text
/// origin to the next stop (see `Register`/`tab_stop_pt`) -- how a lettered
/// sub-item is genuinely encoded as `"(a)\tStatement of compliance"`, not a
/// label and a space. Registers with no tab stop treat it as ordinary
/// whitespace, exactly as before tab stops existed here.
#[derive(Debug, Clone)]
pub enum Span {
    Text(String),
    Bold(String),
}

impl Span {
    pub fn text(&self) -> &str {
        match self {
            Span::Text(s) | Span::Bold(s) => s,
        }
    }
    pub fn is_bold(&self) -> bool {
        matches!(self, Span::Bold(_))
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    Masthead {
        entity_name: String,
        statement_title: String,
        period_label: String,
    },
    /// A standalone cover title block -- centered, large, unruled, dropped
    /// down the page. Distinct from `Masthead` because a real document
    /// bundle uses two genuinely different title treatments: one centered
    /// block on the cover, and a compact left-aligned ruled one repeated
    /// above every statement. Collapsing them into one block with a flag
    /// would hide that they are different typography, not one style toggled.
    CoverTitle {
        entity_name: String,
        statement_title: String,
        period_label: String,
        currency_label: String,
    },
    Heading {
        level: u8,
        text: String,
    },
    /// A body paragraph. `style` carries only the measurements a real
    /// document sets *per paragraph*; everything a document sets once for
    /// all its body text (justification, leading, the default gap between
    /// paragraphs, the tab stop) belongs to the register, not here.
    Para {
        spans: Vec<Span>,
        style: ParaStyle,
    },
    Note(Vec<Span>),
    Table(Table),
    Rule,
    Spacer(f32),
    PageBreak,
    Footer(String),
    /// A named, captioned placeholder for content this v1 doesn't render
    /// (e.g. a chart) -- PDF draws a ruled box with the caption; HTML may
    /// carry real content in a future pass. Never silently dropped.
    Placeholder {
        kind: String,
        caption: String,
    },
    /// A Gantt-style bar/calendar-grid block: a leader panel beside a time
    /// axis, one row of marks plotted per leader row. See `Timeline` below.
    Timeline(Timeline),
}

/// Paragraph-level typography a real document *measures* rather than
/// implies. Every field's default is "what this engine did before the field
/// existed", so `Block::para(spans)` renders byte-for-byte as the old
/// unstyled paragraph did -- these exist for the specific paragraphs whose
/// source document sets them, not as a general style system.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ParaStyle {
    /// Left indent of the whole paragraph, in points -- every line
    /// including the first, measured off the source document rather than
    /// approximated with spaces.
    pub left_indent_pt: f32,
    /// Space opened above this paragraph, in points.
    pub space_before_pt: f32,
    /// Space left below this paragraph, in points. `None` takes the
    /// register's own paragraph gap; `Some(0.0)` genuinely means none, which
    /// is what a sub-item heading sets so its caption sits tight above the
    /// text it introduces.
    pub space_after_pt: Option<f32>,
}

impl Block {
    /// An ordinary body paragraph, with no per-paragraph measurements.
    pub fn para(spans: Vec<Span>) -> Block {
        Block::Para {
            spans,
            style: ParaStyle::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColWidth {
    Pct(f32),
    Pt(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub width: ColWidth,
    pub align: Align,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    Data,
    Subtotal,
    Total,
    SectionBanner,
}

#[derive(Debug, Clone)]
pub enum CellContent {
    Text(String),
    /// A right-aligned numeric figure, pre-formatted by the caller (e.g.
    /// "128.50", "(36.00)", "-"). Kept distinct from Text so a future
    /// backend could apply figure-specific styling without the caller
    /// needing to know about it.
    Figure(String),
    Blank,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub content: CellContent,
    pub bold: bool,
    pub colspan: u8,
    /// Nesting depth of this cell's label, 0 = flush. Real measurement, not
    /// leading spaces: each backend turns the level into its own real
    /// indent (padding-left in HTML, a text-origin offset in PDF). Leading
    /// spaces would leak into text extraction and copy-paste, collapse in
    /// HTML, and give a width that depends on the space glyph rather than
    /// on the design's own indent step.
    pub indent: u8,
}

impl Cell {
    pub fn text(s: impl Into<String>) -> Self {
        Cell {
            content: CellContent::Text(s.into()),
            bold: false,
            colspan: 1,
            indent: 0,
        }
    }
    pub fn figure(s: impl Into<String>) -> Self {
        Cell {
            content: CellContent::Figure(s.into()),
            bold: false,
            colspan: 1,
            indent: 0,
        }
    }
    pub fn blank() -> Self {
        Cell {
            content: CellContent::Blank,
            bold: false,
            colspan: 1,
            indent: 0,
        }
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn colspan(mut self, n: u8) -> Self {
        self.colspan = n;
        self
    }
    pub fn indent(mut self, level: u8) -> Self {
        self.indent = level;
        self
    }
    pub fn as_str(&self) -> &str {
        match &self.content {
            CellContent::Text(s) | CellContent::Figure(s) => s,
            CellContent::Blank => "",
        }
    }
    pub fn is_figure(&self) -> bool {
        matches!(self.content, CellContent::Figure(_))
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub kind: RowKind,
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn data(cells: Vec<Cell>) -> Self {
        Row {
            kind: RowKind::Data,
            cells,
        }
    }
    pub fn total(cells: Vec<Cell>) -> Self {
        Row {
            kind: RowKind::Total,
            cells,
        }
    }
    pub fn subtotal(cells: Vec<Cell>) -> Self {
        Row {
            kind: RowKind::Subtotal,
            cells,
        }
    }
    pub fn section_banner(label: impl Into<String>, colspan: u8) -> Self {
        Row {
            kind: RowKind::SectionBanner,
            cells: vec![Cell::text(label).colspan(colspan)],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub columns: Vec<Column>,
    pub header: Option<Vec<Cell>>,
    /// A second header line, rendered under `header` and above the header
    /// rule. Formal filed statements often need it: a currency symbol
    /// appearing exactly once per statement, on its own line beneath the
    /// period heading, with a narrow note-reference column labelled on that
    /// same line rather than beside the period.
    pub subheader: Option<Vec<Cell>>,
    pub rows: Vec<Row>,
    pub repeat_header: bool,
    pub keep_together: bool,
    /// Index of the first column that horizontal separator rules span;
    /// `None` spans the full content width. Real statements typically rule
    /// only the figure columns -- a rule running under the caption text
    /// would read as an underline on the words rather than as an
    /// arithmetic operator on the figures.
    pub rules_from_col: Option<usize>,
    /// When set, a page break landing mid-table re-emits
    /// "{label} (continued)" above the repeated header, rather than just a
    /// bare repeated header.
    pub continuation_label: Option<String>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Table {
            columns,
            header: None,
            subheader: None,
            rows: Vec::new(),
            repeat_header: true,
            keep_together: false,
            rules_from_col: None,
            continuation_label: None,
        }
    }
    pub fn continued_as(mut self, label: impl Into<String>) -> Self {
        self.continuation_label = Some(label.into());
        self
    }
}

/// A Gantt-style bar/calendar-grid block. The leader panel reuses the real
/// `Table` type directly -- both backends render it through their existing
/// table code paths, so the leader can never diverge from an ordinary
/// table. `rows.len()` must equal the leader's own body row count;
/// `with_leader` enforces this at construction rather than trusting the
/// caller to keep two `Vec`s in lockstep -- the same "never silently
/// dropped" posture `Placeholder` applies to content, here applied to row
/// alignment.
#[derive(Debug, Clone)]
pub struct Timeline {
    pub axis: TimeAxis,
    pub leader: Option<Table>,
    pub leader_width: ColWidth,
    pub rows: Vec<TimelineRow>,
    pub legend: Vec<LegendEntry>,
    pub repeat_axis: bool,
    pub continuation_label: Option<String>,
    /// `None` takes the leader's own row height (the register's body
    /// leading).
    pub row_height_pt: Option<f32>,
}

/// The plotted panel's horizontal scale. Units are caller-supplied `f64`,
/// never real dates -- matching "cells carry pre-formatted strings, never
/// Money/f64" applied to time instead of currency; a caller converts its
/// own calendar type to a day-offset (or any other linear unit) before
/// handing it here.
#[derive(Debug, Clone)]
pub struct TimeAxis {
    pub start: f64,
    pub end: f64,
    /// Hierarchical labelled header bands -- e.g. a year row (`level: 0`)
    /// above a quarter row (`level: 1`).
    pub bands: Vec<AxisBand>,
    /// Plain vertical gridlines, unlabelled -- real positions, not a stride.
    pub gridlines: Vec<f64>,
    /// An optional emphasised vertical line with a label -- e.g. a status
    /// date on a progress report.
    pub marker: Option<AxisMarker>,
}

#[derive(Debug, Clone)]
pub struct AxisBand {
    pub label: String,
    pub start: f64,
    pub end: f64,
    pub level: u8,
}

#[derive(Debug, Clone)]
pub struct AxisMarker {
    pub at: f64,
    pub label: Option<String>,
}

/// The marks plotted for one leader row. Empty when a row has nothing to
/// plot -- it still occupies its row slot, keeping every leader row aligned
/// with its own line in the plotted panel.
#[derive(Debug, Clone, Default)]
pub struct TimelineRow {
    pub marks: Vec<Mark>,
}

impl TimelineRow {
    pub fn empty() -> Self {
        TimelineRow { marks: Vec::new() }
    }
    pub fn one(mark: Mark) -> Self {
        TimelineRow { marks: vec![mark] }
    }
}

#[derive(Debug, Clone)]
pub enum Mark {
    Bar {
        start: f64,
        end: f64,
        style: BarStyle,
        /// `Some(0.0)` is a measured zero; `None` is no progress data at
        /// all -- distinct states, not the same "nothing to show".
        progress: Option<f32>,
        label: Option<MarkLabel>,
    },
    Milestone {
        at: f64,
        shape: MilestoneShape,
        label: Option<MarkLabel>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarStyle {
    Task,
    Summary,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneShape {
    Diamond,
    Triangle,
    Bar,
}

#[derive(Debug, Clone)]
pub struct MarkLabel {
    pub text: String,
    pub placement: LabelPlacement,
    pub bold: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelPlacement {
    Before,
    After,
    Inside,
}

#[derive(Debug, Clone)]
pub struct LegendEntry {
    pub sample: LegendSample,
    pub caption: String,
}

#[derive(Debug, Clone)]
pub enum LegendSample {
    Bar(BarStyle),
    Milestone(MilestoneShape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimelineError {
    RowCountMismatch {
        leader_rows: usize,
        plotted_rows: usize,
    },
    EmptyAxis,
    InvertedBar {
        row: usize,
        mark: usize,
    },
}

impl Timeline {
    /// Refuses a leader/row count mismatch, an inverted axis, or an
    /// inverted bar at construction rather than trusting the caller to get
    /// it right -- a caller mistake becomes a construction-time refusal,
    /// never a silent visual bug.
    pub fn with_leader(
        leader: Table,
        leader_width: ColWidth,
        axis: TimeAxis,
        rows: Vec<TimelineRow>,
    ) -> Result<Timeline, TimelineError> {
        let body = leader.rows.len();
        if body != rows.len() {
            return Err(TimelineError::RowCountMismatch {
                leader_rows: body,
                plotted_rows: rows.len(),
            });
        }
        if axis.end <= axis.start {
            return Err(TimelineError::EmptyAxis);
        }
        for (row_idx, row) in rows.iter().enumerate() {
            for (mark_idx, mark) in row.marks.iter().enumerate() {
                if let Mark::Bar { start, end, .. } = mark {
                    if end < start {
                        return Err(TimelineError::InvertedBar {
                            row: row_idx,
                            mark: mark_idx,
                        });
                    }
                }
            }
        }
        Ok(Timeline {
            axis,
            leader: Some(leader),
            leader_width,
            rows,
            legend: Vec::new(),
            repeat_axis: true,
            continuation_label: None,
            row_height_pt: None,
        })
    }

    pub fn with_legend(mut self, legend: Vec<LegendEntry>) -> Self {
        self.legend = legend;
        self
    }

    /// Unit -> fraction of the plotted panel, clamped to `[0, 1]`. The one
    /// piece of geometry both backends share, so it lives on the type
    /// rather than being written twice.
    pub fn fraction(&self, unit: f64) -> Clamped {
        let span = self.axis.end - self.axis.start;
        let raw = (unit - self.axis.start) / span;
        if raw < 0.0 {
            Clamped {
                value: 0.0,
                clipped: Clipped::Before,
            }
        } else if raw > 1.0 {
            Clamped {
                value: 1.0,
                clipped: Clipped::After,
            }
        } else {
            Clamped {
                value: raw,
                clipped: Clipped::No,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clamped {
    pub value: f64,
    pub clipped: Clipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clipped {
    No,
    Before,
    After,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leader(row_count: usize) -> Table {
        let mut t = Table::new(vec![Column {
            width: ColWidth::Pct(100.0),
            align: Align::Left,
        }]);
        for i in 0..row_count {
            t.rows.push(Row::data(vec![Cell::text(format!("row {i}"))]));
        }
        t
    }

    fn axis() -> TimeAxis {
        TimeAxis {
            start: 0.0,
            end: 100.0,
            bands: Vec::new(),
            gridlines: Vec::new(),
            marker: None,
        }
    }

    #[test]
    fn with_leader_refuses_a_row_count_mismatch() {
        let rows = vec![TimelineRow::empty(), TimelineRow::empty()];
        let err = Timeline::with_leader(leader(3), ColWidth::Pct(30.0), axis(), rows).unwrap_err();
        assert_eq!(
            err,
            TimelineError::RowCountMismatch {
                leader_rows: 3,
                plotted_rows: 2
            }
        );
    }

    #[test]
    fn with_leader_refuses_an_empty_or_inverted_axis() {
        let mut bad_axis = axis();
        bad_axis.end = bad_axis.start;
        let err =
            Timeline::with_leader(leader(0), ColWidth::Pct(30.0), bad_axis, vec![]).unwrap_err();
        assert_eq!(err, TimelineError::EmptyAxis);
    }

    #[test]
    fn with_leader_refuses_an_inverted_bar() {
        let rows = vec![TimelineRow::one(Mark::Bar {
            start: 50.0,
            end: 10.0,
            style: BarStyle::Task,
            progress: None,
            label: None,
        })];
        let err = Timeline::with_leader(leader(1), ColWidth::Pct(30.0), axis(), rows).unwrap_err();
        assert_eq!(err, TimelineError::InvertedBar { row: 0, mark: 0 });
    }

    #[test]
    fn with_leader_accepts_a_matched_row_count() {
        let rows = vec![TimelineRow::empty(), TimelineRow::empty()];
        let tl = Timeline::with_leader(leader(2), ColWidth::Pct(30.0), axis(), rows).unwrap();
        assert_eq!(tl.rows.len(), 2);
        assert!(tl.repeat_axis, "defaults to repeating on every page");
    }

    #[test]
    fn fraction_clamps_outside_the_axis_span() {
        let a = axis();
        let tl = Timeline::with_leader(leader(0), ColWidth::Pct(30.0), a, vec![]).unwrap();
        assert_eq!(
            tl.fraction(-10.0),
            Clamped {
                value: 0.0,
                clipped: Clipped::Before
            }
        );
        assert_eq!(
            tl.fraction(150.0),
            Clamped {
                value: 1.0,
                clipped: Clipped::After
            }
        );
        let mid = tl.fraction(25.0);
        assert_eq!(mid.clipped, Clipped::No);
        assert!((mid.value - 0.25).abs() < 1e-9);
    }
}
