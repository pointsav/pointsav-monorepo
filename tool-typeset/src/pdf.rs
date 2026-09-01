//! PDF backend: renders a Doc directly to PDF bytes. Never HTML-to-PDF
//! conversion. v1 scope, deliberately bounded: standard-14 fonts only (no
//! embedding), flow layout with page-break-on-overflow, repeated table
//! headers, a running header/footer. Explicitly deferred: TTF embedding,
//! charts (rendered as a captioned placeholder box), multi-column layout,
//! links/outlines, PDF/A. "The PDF writer renders the Document model --
//! never HTML -- and its primitives are frozen at standard-14 fonts, text,
//! lines, and rectangles. If a report needs more, change the report, not
//! the renderer."
//!
//! Two visual registers render from this one file, selected by
//! `PageSetup::register` and table-driven through `Style` below: a working
//! document look (one restrained accent, one section-banner fill) and a
//! formal filed-document look (pure black, zero fill) -- the convention a
//! formal audited/filed document follows, where nothing decorative competes
//! with the figures. They share every structural code path -- flow, page
//! breaking, column geometry, repeated headers -- and differ only in the
//! numbers in their `Style`.

use crate::afm::text_width_pt;
use crate::doc::{
    Align, BarStyle, Block, CellContent, ColWidth, Doc, LabelPlacement, Mark, MilestoneShape,
    ParaStyle, Register, RowKind, Span, Table, Timeline,
};
use crate::pdf_writer::{escape_pdf_string, PdfWriter};

// ---- Shared across both registers ----
const HEADING_STYLE: [(f32, f32); 3] = [(12.5, 0.067), (10.5, 0.133), (9.5, 0.133)];
const NOTE_SIZE: f32 = 8.5;
const NOTE_GRAY: f32 = 0.333;
const CONTINUATION_SIZE: f32 = 8.0;
const CONTINUATION_GRAY: f32 = 0.400;
const HEADER_FOOTER_SIZE: f32 = 7.5;
const HEADER_FOOTER_GRAY: f32 = 0.600;
const RULE_H1: (f32, f32) = (1.0, 0.600);
const RULE_FOOTER: (f32, f32) = (0.5, 0.867);
const CELL_PAD_X: f32 = 4.0;

/// How far one justified word gap may be stretched, as a multiple of the
/// font's own space width. Word processors stretch a line however far it
/// takes, which is fine for ordinary prose -- greedy wrapping leaves less
/// than one word of slack -- but a line cut short by something unbreakable
/// (a long defined term, a bracketed marker) would then print as a handful
/// of words strung right across the measure, which reads worse than the
/// ragged edge justification was meant to remove. Past this, the line is
/// left unjustified.
const MAX_WORD_STRETCH: f32 = 3.0;

/// The one accent color in the working-document register, used only for the
/// masthead underline -- never on text, never anywhere else. The formal
/// register has no accent at all.
const ACCENT_RGB: (f32, f32, f32) = (0.122, 0.227, 0.373);
/// Section-banner fill -- one opaque light gray, not alternating/zebra.
const BANNER_FILL: (f32, f32, f32) = (0.949, 0.957, 0.969);

// ---- Timeline mark colors (register-independent -- a Gantt bar's color is
// part of what BarStyle means, not a per-document accent choice). ----
const TIMELINE_TASK_RGB: (f32, f32, f32) = ACCENT_RGB;
const TIMELINE_SUMMARY_RGB: (f32, f32, f32) = (0.30, 0.30, 0.30);
const TIMELINE_EXTERNAL_RGB: (f32, f32, f32) = (0.65, 0.65, 0.65);
const TIMELINE_MILESTONE_RGB: (f32, f32, f32) = (0.80, 0.30, 0.05);
const TIMELINE_PROGRESS_RGB: (f32, f32, f32) = (0.06, 0.11, 0.19);
const TIMELINE_MARKER_RGB: (f32, f32, f32) = (0.80, 0.10, 0.10);
const TIMELINE_GRIDLINE_GRAY: f32 = 0.85;

/// Every value that differs between the two visual registers, in one place.
/// Anything absent from this struct is deliberately shared: a difference
/// that isn't a real design difference is drift.
struct Style {
    // Masthead (the compact per-statement title block).
    masthead_entity: (f32, f32),
    masthead_title: (f32, f32),
    masthead_title_face: Face,
    masthead_period: (f32, f32),
    masthead_centered: bool,
    masthead_gap_above_rule: f32,
    masthead_rule_w: f32,
    /// `Some` paints the masthead rule in the accent color; `None` uses the
    /// masthead entity line's own ink, i.e. plain black in the formal
    /// register.
    masthead_rule_rgb: Option<(f32, f32, f32)>,
    masthead_gap_below_rule: f32,

    /// How far the cover title block is dropped from the top of the page.
    cover_top_drop: f32,

    body_size: f32,
    body_gray: f32,
    leading_factor: f32,

    /// Full justification of body paragraphs: every wrapped line except the
    /// last has its word gaps stretched so its right edge lands exactly on
    /// the margin -- the convention a formal filed document's notes follow,
    /// deliberately NOT the working document's, whose ragged right edge is
    /// the readable choice for text a single reader scans on screen.
    justify_body: bool,
    /// Exact leading for paragraph text, in points. A FIXED value, not a
    /// multiple of the font size, when the register measures one -- so a
    /// paragraph and a note can share one baseline rhythm rather than each
    /// deriving its own. `None` keeps `size * leading_factor`.
    body_line_pt: Option<f32>,
    /// Gap after every paragraph, in points, when the register measures one
    /// (with `space_before` zero). `None` keeps the proportional gap this
    /// engine used before the value was measured.
    para_space_after_pt: Option<f32>,
    /// The document's default tab stop, in points. `None` disables tab
    /// handling entirely: a tab is then ordinary whitespace, which is what
    /// every register that has never had a tab stop expects.
    tab_stop_pt: Option<f32>,

    table_header_size: f32,
    table_header_gray: f32,
    /// When true, a header cell prints bold only if the caller marked it
    /// bold. Working-document column headings are uniformly bold by design;
    /// a formal document's heading block may mix weights on purpose -- e.g.
    /// a period heading and currency symbol bold, a note-reference label
    /// beside them not.
    header_face_from_cell: bool,

    note_size: f32,
    note_gray: f32,
    /// Face for a note whose spans carry no bold of their own. Formal
    /// documents commonly set a "See accompanying notes..." line in italic.
    note_face: Face,
    footer_gray: f32,

    /// Distance from the last header baseline down to the header rule, and
    /// from that rule down to the first body baseline.
    header_rule_drop: f32,
    header_rule_gap: f32,

    /// `None` = no rule under ordinary data rows. Formal documents typically
    /// draw none: a ruled grid is a working-paper aid, not statement
    /// typography.
    rule_row: Option<(f32, f32)>,
    rule_subtotal: (f32, f32),
    rule_total_above: (f32, f32),
    rule_total_below: (f32, f32),
    rule_header: (f32, f32),
    double_rule_gap: f32,

    /// `None` = section captions are bold text on white, no fill.
    banner_fill: Option<(f32, f32, f32)>,
    /// Extra space opened above a section caption, as a multiple of body
    /// leading.
    banner_lead_factor: f32,

    /// Text-origin offset per `Cell::indent` level. 5 entries (0..=4) --
    /// levels beyond the array's length clamp onto its last entry, so this
    /// must be as deep as any real document's indent hierarchy goes.
    indent_pt: [f32; 5],
}

/// The working-document register: detail listings, working papers, event
/// registers. Values frozen at exactly what this engine rendered before the
/// formal register existed, so those reports are byte-identical.
const WORKING_DOCUMENT: Style = Style {
    masthead_entity: (14.0, 0.067),
    masthead_title: (11.0, 0.133),
    masthead_title_face: Face::Regular,
    masthead_period: (9.0, 0.333),
    masthead_centered: true,
    masthead_gap_above_rule: 10.0,
    masthead_rule_w: 1.2,
    masthead_rule_rgb: Some(ACCENT_RGB),
    masthead_gap_below_rule: 14.0,

    cover_top_drop: 0.0,

    body_size: 9.0,
    body_gray: 0.067,
    leading_factor: 1.30,
    justify_body: false,
    body_line_pt: None,
    para_space_after_pt: None,
    tab_stop_pt: None,
    table_header_size: 8.5,
    table_header_gray: 0.133,
    header_face_from_cell: false,

    note_size: 8.5,
    note_gray: 0.333,
    note_face: Face::Regular,
    footer_gray: 0.600,

    header_rule_drop: 0.0,
    header_rule_gap: 8.5 * 1.30,

    // The shared 4-step ladder: hairline / light / standard / emphasis.
    rule_row: Some((0.4, 0.890)),
    rule_subtotal: (0.6, 0.667),
    rule_total_above: (1.0, 0.533),
    rule_total_below: (0.75, 0.533),
    rule_header: (1.0, 0.600),
    double_rule_gap: 1.8,

    banner_fill: Some(BANNER_FILL),
    banner_lead_factor: 0.4,

    indent_pt: [0.0, 16.0, 28.0, 40.0, 52.0],
};

/// The formal filed-document register: pure black `#000`, zero fill, zero
/// accent, 0.5pt hairlines, a 3pt double grand-total rule, 16/28pt indents.
const FORMAL_STATEMENT: Style = Style {
    masthead_entity: (11.0, 0.0),
    masthead_title: (11.0, 0.0),
    masthead_title_face: Face::Bold,
    masthead_period: (10.0, 0.0),
    masthead_centered: false,
    // Measured from where the masthead's last line leaves the cursor, one
    // full leading below its baseline -- so a negative value pulls the rule
    // back up. Positive in the working register, which wants an airier
    // block.
    masthead_gap_above_rule: -7.5,
    masthead_rule_w: 1.44,
    masthead_rule_rgb: None,
    masthead_gap_below_rule: 26.0,

    // A formal cover typically sets its title block about a third of the
    // way down an otherwise empty page.
    cover_top_drop: 130.0,

    body_size: 10.0,
    body_gray: 0.0,
    leading_factor: 1.35,
    justify_body: true,
    body_line_pt: Some(12.0),
    para_space_after_pt: Some(10.0),
    tab_stop_pt: Some(36.0),
    table_header_size: 10.0,
    table_header_gray: 0.0,
    header_face_from_cell: true,

    note_size: 9.0,
    note_gray: 0.0,
    note_face: Face::Oblique,
    footer_gray: 0.0,

    header_rule_drop: 4.0,
    header_rule_gap: 10.0,

    rule_row: None,
    rule_subtotal: (0.5, 0.0),
    rule_total_above: (0.5, 0.0),
    rule_total_below: (1.0, 0.0),
    rule_header: (1.0, 0.0),
    // 1pt line + 1pt gap + 1pt line == a 3pt double rule.
    double_rule_gap: 2.0,

    banner_fill: None,
    banner_lead_factor: 0.9,

    indent_pt: [0.0, 16.0, 28.0, 40.0, 52.0],
};

impl Style {
    fn for_register(register: Register) -> &'static Style {
        match register {
            Register::WorkingDocument => &WORKING_DOCUMENT,
            Register::FormalStatement => &FORMAL_STATEMENT,
        }
    }

    fn indent_of(&self, level: u8) -> f32 {
        self.indent_pt[(level as usize).min(self.indent_pt.len() - 1)]
    }
}

/// One header cell already resolved to its box, weight and wrapped lines --
/// planned before anything is drawn so the whole header block's height is
/// known in time to reserve it against a page break.
struct HeaderCell {
    x: f32,
    w: f32,
    align: Align,
    face: Face,
    lines: Vec<String>,
}

/// One unit of paragraph text: a word carrying its own face, or a real tab.
#[derive(Clone, Copy)]
enum Tok<'a> {
    Word(&'a str, bool),
    Tab,
}

/// One wrapped line, planned before anything is drawn -- justification needs
/// to know the line's natural width and how many gaps it has to share the
/// slack between, and whether it is the paragraph's last (which is never
/// justified), none of which is knowable while the line is still growing.
struct Line<'a> {
    toks: Vec<Tok<'a>>,
    /// Left offset of this line from the margin -- the paragraph's own left
    /// indent, carried per line because that is where justification needs
    /// it when working out how much slack the line has left.
    indent: f32,
    /// Natural width of the line's text, from the line's own start.
    width: f32,
    /// Inter-word gaps available to absorb the slack.
    gaps: u32,
    has_tab: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Face {
    Regular,
    Bold,
    Oblique,
}

impl Face {
    fn font_op(self) -> &'static str {
        match self {
            Face::Regular => "/F1",
            Face::Bold => "/F2",
            Face::Oblique => "/F3",
        }
    }
    fn is_bold(self) -> bool {
        self == Face::Bold
    }
}

struct Layout {
    page_w: f32,
    page_h: f32,
    margin: f32,
    content_width: f32,
    y: f32,
    pages: Vec<String>,
    current: String,
    style: &'static Style,
}

impl Layout {
    fn new(doc: &Doc) -> Self {
        let (mut page_w, mut page_h) = doc.page.size.dims_pt();
        if doc.page.landscape {
            std::mem::swap(&mut page_w, &mut page_h);
        }
        let margin = doc.page.margin_pt;
        Layout {
            page_w,
            page_h,
            margin,
            content_width: page_w - 2.0 * margin,
            y: page_h - margin,
            pages: Vec::new(),
            current: String::new(),
            style: Style::for_register(doc.page.register),
        }
    }

    fn body_leading(&self) -> f32 {
        self.style.body_size * self.style.leading_factor
    }

    fn right_edge(&self) -> f32 {
        self.margin + self.content_width
    }

    fn new_page(&mut self) {
        self.pages.push(std::mem::take(&mut self.current));
        self.y = self.page_h - self.margin;
    }

    fn ensure_room(&mut self, needed: f32) {
        if self.y - needed < self.margin {
            self.new_page();
        }
    }

    fn text(&mut self, x: f32, size: f32, face: Face, gray: f32, s: &str) {
        self.text_spaced(x, size, face, gray, s, 0.0);
    }

    /// Draws one run with `word_spacing` extra points added after every
    /// space it contains -- the PDF `Tw` operator, which is how justified
    /// text is set without emitting one `Tj` per word (that would bloat the
    /// stream and break copy-paste, since the spaces would only exist as
    /// gaps between separately-positioned runs).
    ///
    /// `Tw` adds its value after every literal 0x20 byte of the string,
    /// which is exact here precisely because this engine is WinAnsi-only:
    /// 0x20 is always the space glyph, never a lead byte of something else.
    /// It is a text-state parameter and therefore part of the graphics
    /// state, so the `q`/`Q` this run is already wrapped in restores it to
    /// the document default of 0 -- word spacing can never leak into the
    /// next run.
    fn text_spaced(
        &mut self,
        x: f32,
        size: f32,
        face: Face,
        gray: f32,
        s: &str,
        word_spacing: f32,
    ) {
        let x = x.max(self.margin); // never let a bad width calc push text off-page
                                    // Emitted only when it does something, so every register that never
                                    // justifies produces exactly the byte stream it always has.
        let tw = if word_spacing != 0.0 {
            format!("{word_spacing:.3} Tw ")
        } else {
            String::new()
        };
        self.current.push_str(&format!(
            "q {gray:.3} g BT {tw}{} {size} Tf 1 0 0 1 {x:.2} {:.2} Tm ({}) Tj ET Q\n",
            face.font_op(),
            self.y,
            escape_pdf_string(s)
        ));
    }

    fn text_right_aligned(&mut self, right_edge: f32, size: f32, face: Face, gray: f32, s: &str) {
        let w = text_width_pt(s, size, face.is_bold());
        self.text(right_edge - w, size, face, gray, s);
    }

    fn text_centered(&mut self, center_x: f32, size: f32, face: Face, gray: f32, s: &str) {
        let w = text_width_pt(s, size, face.is_bold());
        self.text((center_x - w / 2.0).max(self.margin), size, face, gray, s);
    }

    fn line_gray(&mut self, x1: f32, x2: f32, y: f32, width: f32, gray: f32) {
        self.current.push_str(&format!(
            "q {gray:.3} G {width} w {x1:.2} {y:.2} m {x2:.2} {y:.2} l S Q\n"
        ));
    }

    fn line_rgb(&mut self, x1: f32, x2: f32, y: f32, width: f32, rgb: (f32, f32, f32)) {
        self.current.push_str(&format!(
            "q {:.3} {:.3} {:.3} RG {width} w {x1:.2} {y:.2} m {x2:.2} {y:.2} l S Q\n",
            rgb.0, rgb.1, rgb.2
        ));
    }

    fn double_rule(&mut self, x1: f32, x2: f32, y_top: f32, width: f32, gray: f32) {
        let gap = self.style.double_rule_gap;
        self.line_gray(x1, x2, y_top, width, gray);
        self.line_gray(x1, x2, y_top - gap, width, gray);
    }

    fn fill_rect(&mut self, x: f32, y_bottom: f32, w: f32, h: f32, rgb: (f32, f32, f32)) {
        self.current.push_str(&format!(
            "q {:.3} {:.3} {:.3} rg {x:.2} {y_bottom:.2} {w:.2} {h:.2} re f Q\n",
            rgb.0, rgb.1, rgb.2
        ));
    }

    fn advance(&mut self, pt: f32) {
        self.y -= pt;
    }

    /// Filled closed polygon: `q r g b rg  x0 y0 m  x1 y1 l  ...  h f Q`.
    /// Needed for the milestone diamond/triangle and a Summary bar's
    /// turned-down end caps -- the one shape neither `fill_rect` nor
    /// `line_gray` can draw. Stays within this backend's frozen primitive
    /// set (standard-14 text + simple vector ops).
    fn fill_polygon(&mut self, points: &[(f32, f32)], rgb: (f32, f32, f32)) {
        if points.len() < 3 {
            return;
        }
        let mut ops = format!("q {:.3} {:.3} {:.3} rg ", rgb.0, rgb.1, rgb.2);
        ops.push_str(&format!("{:.2} {:.2} m ", points[0].0, points[0].1));
        for &(x, y) in &points[1..] {
            ops.push_str(&format!("{x:.2} {y:.2} l "));
        }
        ops.push_str("h f Q\n");
        self.current.push_str(&ops);
    }

    fn wrap(text: &str, max_width: f32, size: f32, bold: bool) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if text_width_pt(&candidate, size, bold) > max_width && !current.is_empty() {
                lines.push(current);
                current = word.to_string();
            } else {
                current = candidate;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        if lines.is_empty() {
            lines.push(String::new());
        }
        lines
    }

    /// Truncates `s` to fit `max_width`, appending an ellipsis if anything
    /// was cut -- used for table cells, which flow left-to-right in a
    /// fixed column rather than wrapping (unlike paragraphs).
    fn truncate_to_width(s: &str, max_width: f32, size: f32, bold: bool) -> String {
        if text_width_pt(s, size, bold) <= max_width {
            return s.to_string();
        }
        let ellipsis = "\u{2026}";
        let ellipsis_w = text_width_pt(ellipsis, size, bold);
        let mut result = String::new();
        for c in s.chars() {
            let candidate: String = result.chars().chain(std::iter::once(c)).collect();
            if text_width_pt(&candidate, size, bold) + ellipsis_w > max_width {
                break;
            }
            result = candidate;
        }
        format!("{result}{ellipsis}")
    }

    /// Exact leading for paragraph text at `size` -- the register's own
    /// fixed line height where it measured one, otherwise the proportional
    /// one derived from the font size.
    fn para_leading(&self, size: f32) -> f32 {
        self.style
            .body_line_pt
            .unwrap_or(size * self.style.leading_factor)
    }

    /// Gap below a paragraph: the paragraph's own measured `space_after`
    /// where it sets one, else the register's, else the proportional gap
    /// this engine used before any of it was measured.
    fn para_space_after(&self, style: &ParaStyle, size: f32) -> f32 {
        style
            .space_after_pt
            .or(self.style.para_space_after_pt)
            .unwrap_or(size * self.style.leading_factor * 0.4)
    }

    /// The next tab stop strictly to the right of `x_from_margin`, in points
    /// from the left margin. Always advances at least to the following stop,
    /// even from exactly on one -- what a tab means in a word processor.
    fn next_tab_stop(&self, x_from_margin: f32) -> f32 {
        match self.style.tab_stop_pt {
            Some(stop) if stop > 0.0 => ((x_from_margin / stop).floor() + 1.0) * stop,
            _ => x_from_margin,
        }
    }

    /// Splits spans into the words and tabs a paragraph is laid out from,
    /// carrying each token's own face. Tabs survive as tokens only where
    /// the register defines a stop for them; everywhere else they collapse
    /// into surrounding whitespace exactly as they always have.
    fn tokenize<'a>(spans: &'a [Span], tabs: bool) -> Vec<Tok<'a>> {
        let mut out = Vec::new();
        for span in spans {
            if tabs {
                for (i, segment) in span.text().split('\t').enumerate() {
                    if i > 0 {
                        out.push(Tok::Tab);
                    }
                    out.extend(
                        segment
                            .split_whitespace()
                            .map(|w| Tok::Word(w, span.is_bold())),
                    );
                }
            } else {
                out.extend(
                    span.text()
                        .split_whitespace()
                        .map(|w| Tok::Word(w, span.is_bold())),
                );
            }
        }
        out
    }

    /// Lays spans out as wrapped paragraph lines honoring each span's own
    /// face: a `Span::Bold` lead-in renders bold while the running text
    /// keeps `base_face` -- the same per-span treatment the HTML backend
    /// has always given these spans (`<strong>` on the bold span only,
    /// never a whole-paragraph face change). Word-greedy wrapping on the
    /// same whitespace-collapsing rules as `wrap`; for a uniform-face
    /// paragraph with no hanging indent the line breaks are identical to
    /// `wrap`'s.
    ///
    /// In a justifying register every line but the last is then stretched
    /// at its word gaps onto the right margin (see `justify_body`).
    fn render_paragraph(
        &mut self,
        spans: &[Span],
        size: f32,
        gray: f32,
        base_face: Face,
        left_indent: f32,
    ) {
        let space_w = text_width_pt(" ", size, false);
        let toks = Self::tokenize(spans, self.style.tab_stop_pt.is_some());

        // Wrapping works in width-from-line-start, not absolute x, so that
        // an unindented paragraph's break decisions are bit-for-bit the
        // arithmetic this engine has always done.
        let mut lines: Vec<Line> = Vec::new();
        let mut current: Vec<Tok> = Vec::new();
        let mut width = 0.0f32;
        let mut gaps = 0u32;
        let mut has_tab = false;
        let indent = left_indent;
        for tok in toks {
            match tok {
                Tok::Tab => {
                    // A tab positions; it never wraps and never ends a line.
                    width = self.next_tab_stop(indent + width) - indent;
                    has_tab = true;
                    current.push(tok);
                }
                Tok::Word(word, bold) => {
                    let word_w = text_width_pt(word, size, bold);
                    let follows_word = matches!(current.last(), Some(Tok::Word(..)));
                    let sep_w = if follows_word { space_w } else { 0.0 };
                    if !current.is_empty() && width + sep_w + word_w > self.content_width - indent {
                        lines.push(Line {
                            toks: std::mem::take(&mut current),
                            indent,
                            width,
                            gaps,
                            has_tab,
                        });
                        width = word_w;
                        gaps = 0;
                        has_tab = false;
                    } else {
                        width += sep_w + word_w;
                        if follows_word {
                            gaps += 1;
                        }
                    }
                    current.push(tok);
                }
            }
        }
        if !current.is_empty() || lines.is_empty() {
            lines.push(Line {
                toks: current,
                indent,
                width,
                gaps,
                has_tab,
            });
        }

        let leading = self.para_leading(size);
        let last = lines.len() - 1;
        for (i, line) in lines.iter().enumerate() {
            self.ensure_room(leading);
            // The last line of a paragraph is never justified -- that ragged
            // final line IS justified setting, not a shortfall of it -- and
            // neither is a line with no internal gap to stretch (one word),
            // nor one positioned by a tab, whose whole point is that its
            // parts sit at fixed stops.
            let slack = self.content_width - line.indent - line.width;
            let stretch = if self.style.justify_body
                && i != last
                && !line.has_tab
                && line.gaps > 0
                && slack > 0.0
            {
                let per_gap = slack / line.gaps as f32;
                if per_gap <= MAX_WORD_STRETCH * space_w {
                    per_gap
                } else {
                    0.0
                }
            } else {
                0.0
            };

            let mut x = self.margin + line.indent;
            let mut t = 0usize;
            while t < line.toks.len() {
                let Tok::Word(_, bold) = line.toks[t] else {
                    x = self.margin + self.next_tab_stop(x - self.margin);
                    t += 1;
                    continue;
                };
                let mut run = String::new();
                let mut words = 0u32;
                while let Some(&Tok::Word(word, b)) = line.toks.get(t) {
                    if b != bold {
                        break;
                    }
                    if words > 0 {
                        run.push(' ');
                    }
                    run.push_str(word);
                    words += 1;
                    t += 1;
                }
                let face = if bold { Face::Bold } else { base_face };
                self.text_spaced(x, size, face, gray, &run, stretch);
                // Every space inside the run was widened by `Tw` too, so the
                // cursor has to account for them or a mixed-face line's
                // second run would land back on the unjustified position.
                x += text_width_pt(&run, size, bold) + stretch * (words - 1) as f32;
                if matches!(line.toks.get(t), Some(Tok::Word(..))) {
                    x += space_w + stretch;
                }
            }
            self.advance(leading);
        }
    }

    fn column_geometry(&self, table: &Table) -> Vec<(f32, f32)> {
        self.column_geometry_in(&table.columns, self.margin, self.content_width)
    }

    /// `column_geometry` generalized to an arbitrary base position/width --
    /// what `Timeline`'s leader panel needs, since its columns are
    /// percentages of the *leader's* width, not the full page content
    /// width an ordinary table's columns are.
    fn column_geometry_in(
        &self,
        columns: &[crate::doc::Column],
        base_x: f32,
        base_width: f32,
    ) -> Vec<(f32, f32)> {
        // Returns (left_x, width) per column.
        let mut out = Vec::with_capacity(columns.len());
        let mut x = base_x;
        for col in columns {
            let w = match col.width {
                ColWidth::Pct(p) => base_width * (p / 100.0),
                ColWidth::Pt(pt) => pt,
            };
            out.push((x, w));
            x += w;
        }
        out
    }

    /// Places one cell's text within its column box. `indent` shifts the
    /// text origin for left-aligned labels only -- indenting a right-aligned
    /// figure would move it off the figure rail its whole column aligns to.
    #[allow(clippy::too_many_arguments)]
    fn place_cell_text(
        &mut self,
        s: &str,
        x: f32,
        w: f32,
        align: Align,
        size: f32,
        face: Face,
        gray: f32,
        indent: f32,
    ) {
        let indent = if align == Align::Left { indent } else { 0.0 };
        let inner_w = (w - 2.0 * CELL_PAD_X - indent).max(1.0);
        let s = Self::truncate_to_width(s, inner_w, size, face.is_bold());
        match align {
            Align::Left => self.text(x + CELL_PAD_X + indent, size, face, gray, &s),
            Align::Right => self.text_right_aligned(x + w - CELL_PAD_X, size, face, gray, &s),
            Align::Center => self.text_centered(x + w / 2.0, size, face, gray, &s),
        }
    }

    /// Splits one header cell into the lines it needs inside its column.
    /// Column headings genuinely wrap in real reports -- a long line-item
    /// name can sit over a column barely wide enough for its first word --
    /// so unlike body cells they must never be truncated to an ellipsis,
    /// which would destroy the reader's only label for a figure column.
    fn wrap_header_cell(s: &str, w: f32, size: f32, face: Face) -> Vec<String> {
        Self::wrap(s, (w - 2.0 * CELL_PAD_X).max(1.0), size, face.is_bold())
    }

    /// Lays out one header line into (left_x, width, align, lines) per cell,
    /// without drawing -- so the caller can reserve the whole header block's
    /// height before committing to a page.
    fn plan_header_line(
        cells: &[crate::doc::Cell],
        columns: &[crate::doc::Column],
        geometry: &[(f32, f32)],
        size: f32,
        face_from_cell: bool,
    ) -> Vec<HeaderCell> {
        let mut out = Vec::new();
        let mut col_idx = 0usize;
        for cell in cells {
            if let Some(&(x, _)) = geometry.get(col_idx) {
                let w: f32 = (0..cell.colspan.max(1) as usize)
                    .filter_map(|i| geometry.get(col_idx + i))
                    .map(|&(_, w)| w)
                    .sum();
                let align = columns.get(col_idx).map(|c| c.align).unwrap_or(Align::Left);
                let face = if !face_from_cell || cell.bold {
                    Face::Bold
                } else {
                    Face::Regular
                };
                out.push(HeaderCell {
                    x,
                    w,
                    align,
                    face,
                    lines: Self::wrap_header_cell(cell.as_str(), w, size, face),
                });
            }
            col_idx += cell.colspan as usize;
        }
        out
    }

    /// Draws a planned header line, bottom-aligning every cell against the
    /// deepest one: a wrapped heading hangs *up* from the rule so that every
    /// column's last line shares one baseline sitting on it -- the
    /// convention a real formal document uses, and the reason a top-down
    /// block would look broken beside a one-line neighbour.
    fn draw_header_line(&mut self, plan: &[HeaderCell], size: f32, gray: f32) {
        let leading = size * self.style.leading_factor;
        let depth = plan.iter().map(|c| c.lines.len()).max().unwrap_or(1);
        let top = self.y;
        for cell in plan {
            for (j, line) in cell.lines.iter().enumerate() {
                self.y = top - (depth - cell.lines.len() + j) as f32 * leading;
                self.place_cell_text(line, cell.x, cell.w, cell.align, size, cell.face, gray, 0.0);
            }
        }
        self.y = top - (depth - 1) as f32 * leading;
    }

    fn render_table_header(&mut self, table: &Table, geometry: &[(f32, f32)], rule_x1: f32) {
        let size = self.style.table_header_size;
        let leading = size * self.style.leading_factor;
        let gray = self.style.table_header_gray;

        let head = table
            .header
            .as_ref()
            .map(|h| {
                Self::plan_header_line(
                    h,
                    &table.columns,
                    geometry,
                    size,
                    self.style.header_face_from_cell,
                )
            })
            .unwrap_or_default();
        let sub = table.subheader.as_ref().map(|c| {
            Self::plan_header_line(
                c,
                &table.columns,
                geometry,
                size,
                self.style.header_face_from_cell,
            )
        });

        let depth = |p: &[HeaderCell]| p.iter().map(|c| c.lines.len()).max().unwrap_or(1) as f32;
        // One reservation for the whole block -- header lines, the optional
        // subheader, the rule, and the gap under it -- so a page break never
        // lands between a heading and the rule that belongs to it.
        let mut needed = depth(&head) * leading;
        if let Some(sub) = &sub {
            needed += leading + depth(sub) * leading;
        }
        needed += self.style.header_rule_drop + self.style.header_rule_gap;
        self.ensure_room(needed);

        self.draw_header_line(&head, size, gray);
        if let Some(sub) = &sub {
            self.advance(leading);
            self.draw_header_line(sub, size, gray);
        }
        let rule_y = self.y - self.style.header_rule_drop;
        let (rw, rg) = self.style.rule_header;
        self.line_gray(rule_x1, self.right_edge(), rule_y, rw, rg);
        self.y = rule_y - self.style.header_rule_gap;
    }

    /// Estimates a table's total rendered height -- used only for
    /// `keep_together`, to decide whether the WHOLE table should jump to a
    /// fresh page rather than breaking mid-table. An estimate, not exact
    /// (doesn't account for cell truncation/wrapping), which is fine for
    /// its one purpose.
    fn estimate_table_height(&self, table: &Table) -> f32 {
        let leading = self.body_leading();
        let header_leading = self.style.table_header_size * self.style.leading_factor;
        let header_h = if table.header.is_some() {
            header_leading * 2.0
        } else {
            0.0
        };
        header_h + leading * table.rows.len() as f32
    }

    fn render_table(&mut self, table: &Table) {
        let geometry = self.column_geometry(table);
        let leading = self.body_leading();
        // Rules run across the money columns only when the caller says so;
        // otherwise the full content width, as working-document tables want.
        let rule_x1 = table
            .rules_from_col
            .and_then(|i| geometry.get(i))
            .map(|&(x, _)| x)
            .unwrap_or(self.margin);
        let rule_x2 = self.right_edge();

        if table.keep_together {
            self.ensure_room(self.estimate_table_height(table));
        }

        if table.header.is_some() || table.subheader.is_some() {
            self.render_table_header(table, &geometry, rule_x1);
        }

        for (row_idx, row) in table.rows.iter().enumerate() {
            let y_before = self.y;
            self.ensure_room(leading);
            let page_broke = self.y > y_before; // ensure_room reset y to the top of a new page
            if page_broke {
                if let Some(label) = &table.continuation_label {
                    self.text(
                        self.margin,
                        CONTINUATION_SIZE,
                        Face::Oblique,
                        CONTINUATION_GRAY,
                        &format!("{label} (continued)"),
                    );
                    self.advance(CONTINUATION_SIZE * self.style.leading_factor);
                }
                if table.repeat_header && (table.header.is_some() || table.subheader.is_some()) {
                    self.render_table_header(table, &geometry, rule_x1);
                    self.ensure_room(leading);
                }
            }
            // A section caption opens space against the section above it.
            // The first row of a table has no section above it -- only the
            // header rule, which already sets its own gap -- so leading
            // there would just push the whole statement down the page.
            if row.kind == RowKind::SectionBanner {
                if row_idx > 0 {
                    self.advance(leading * self.style.banner_lead_factor);
                }
                if let Some(fill) = self.style.banner_fill {
                    self.fill_rect(
                        self.margin,
                        self.y - leading + leading * 0.22,
                        self.content_width,
                        leading,
                        fill,
                    );
                }
            }
            let size = self.style.body_size;
            let (gray, base_face) = match row.kind {
                RowKind::Total => (0.0, Face::Bold), // grand totals always print in full black
                RowKind::SectionBanner => (self.style.body_gray, Face::Bold),
                _ => (self.style.body_gray, Face::Regular),
            };
            let mut col_idx = 0usize;
            for cell in &row.cells {
                if let Some(&(x, w)) = geometry.get(col_idx) {
                    let align = table
                        .columns
                        .get(col_idx)
                        .map(|c| c.align)
                        .unwrap_or(Align::Left);
                    let text = match &cell.content {
                        CellContent::Text(s) | CellContent::Figure(s) => s.as_str(),
                        CellContent::Blank => "",
                    };
                    let face = if cell.bold || base_face == Face::Bold {
                        Face::Bold
                    } else {
                        Face::Regular
                    };
                    if !text.is_empty() {
                        let indent = self.style.indent_of(cell.indent);
                        self.place_cell_text(text, x, w, align, size, face, gray, indent);
                    }
                }
                col_idx += cell.colspan as usize;
            }
            // Separator rules must clear the glyphs themselves -- text sits
            // roughly [-descent, +cap_height] around the baseline (self.y),
            // so "above the row" rules belong up near the *next* row's gap
            // (~0.75 leading above baseline) and "below the row" rules
            // belong down near the *following* row's gap (~0.2-0.35 leading
            // below baseline), never close to baseline itself or they read
            // as a strikethrough across the text.
            match row.kind {
                RowKind::Subtotal => {
                    let (w, g) = self.style.rule_subtotal;
                    self.line_gray(rule_x1, rule_x2, self.y + leading * 0.75, w, g);
                }
                RowKind::Total => {
                    let (w, g) = self.style.rule_total_above;
                    self.line_gray(rule_x1, rule_x2, self.y + leading * 0.75, w, g);
                    let (w, g) = self.style.rule_total_below;
                    self.double_rule(rule_x1, rule_x2, self.y - leading * 0.22, w, g);
                }
                _ => {
                    if let Some((w, g)) = self.style.rule_row {
                        self.line_gray(rule_x1, rule_x2, self.y - leading * 0.2, w, g);
                    }
                }
            }
            self.advance(leading);
        }
    }

    /// Renders a `Block::Timeline`: a leader panel (a real `Table`, drawn
    /// through the same cell/geometry primitives `render_table` uses, but
    /// not `render_table` itself -- the leader's header rule and row loop
    /// both need to stop at the leader's own width rather than the full
    /// page, which `render_table` doesn't parameterize) beside a plotted
    /// panel of marks, one row of marks per leader row.
    fn render_timeline(&mut self, timeline: &Timeline) {
        let Some(leader) = timeline.leader.as_ref() else {
            return;
        };
        let leader_w = match timeline.leader_width {
            ColWidth::Pct(p) => self.content_width * (p / 100.0),
            ColWidth::Pt(pt) => pt,
        };
        let axis_x = self.margin + leader_w;
        let axis_w = (self.content_width - leader_w).max(1.0);
        let row_h = timeline.row_height_pt.unwrap_or(self.body_leading());
        let leader_geometry = self.column_geometry_in(&leader.columns, self.margin, leader_w);

        self.render_timeline_header(timeline, leader, &leader_geometry, axis_x, axis_w);

        let n = leader.rows.len().min(timeline.rows.len());
        for i in 0..n {
            let row = &leader.rows[i];
            let plotted = &timeline.rows[i];

            let y_before = self.y;
            self.ensure_room(row_h);
            let page_broke = self.y > y_before; // ensure_room reset y to the top of a new page
            if page_broke {
                if let Some(label) = &timeline.continuation_label {
                    self.text(
                        self.margin,
                        CONTINUATION_SIZE,
                        Face::Oblique,
                        CONTINUATION_GRAY,
                        &format!("{label} (continued)"),
                    );
                    self.advance(CONTINUATION_SIZE * self.style.leading_factor);
                }
                if timeline.repeat_axis {
                    self.render_timeline_header(timeline, leader, &leader_geometry, axis_x, axis_w);
                }
            }

            if row.kind == RowKind::SectionBanner {
                if let Some(fill) = self.style.banner_fill {
                    self.fill_rect(
                        self.margin,
                        self.y - row_h + row_h * 0.22,
                        self.content_width,
                        row_h,
                        fill,
                    );
                }
            }

            let size = self.style.body_size;
            let (gray, base_face) = match row.kind {
                RowKind::Total => (0.0, Face::Bold),
                RowKind::SectionBanner => (self.style.body_gray, Face::Bold),
                _ => (self.style.body_gray, Face::Regular),
            };
            let mut col_idx = 0usize;
            for cell in &row.cells {
                if let Some(&(x, w)) = leader_geometry.get(col_idx) {
                    let align = leader
                        .columns
                        .get(col_idx)
                        .map(|c| c.align)
                        .unwrap_or(Align::Left);
                    let text = match &cell.content {
                        CellContent::Text(s) | CellContent::Figure(s) => s.as_str(),
                        CellContent::Blank => "",
                    };
                    let face = if cell.bold || base_face == Face::Bold {
                        Face::Bold
                    } else {
                        Face::Regular
                    };
                    if !text.is_empty() {
                        let indent = self.style.indent_of(cell.indent);
                        self.place_cell_text(text, x, w, align, size, face, gray, indent);
                    }
                }
                col_idx += cell.colspan as usize;
            }

            // Gridlines and the status marker are drawn per row rather than
            // once as one long vertical -- this row's own draw call is
            // already page-break-safe, so a tick drawn inside it is too,
            // with no need to know the table's total height in advance.
            // Consecutive rows' ticks read as one continuous line.
            let slot_bottom = self.y - row_h + row_h * 0.22;
            self.render_timeline_row_axis(timeline, axis_x, axis_w, slot_bottom, row_h);

            for mark in &plotted.marks {
                self.render_mark(timeline, mark, axis_x, axis_w, self.y, row_h);
            }

            self.advance(row_h);
        }

        if !timeline.legend.is_empty() {
            self.render_timeline_legend(&timeline.legend, axis_x, axis_w);
        }
    }

    /// Draws the leader's column headings plus the axis's labelled bands,
    /// then one header rule spanning the full leader+axis width.
    fn render_timeline_header(
        &mut self,
        timeline: &Timeline,
        leader: &Table,
        leader_geometry: &[(f32, f32)],
        axis_x: f32,
        axis_w: f32,
    ) {
        let size = self.style.table_header_size;
        let leading = size * self.style.leading_factor;

        let head = leader.header.as_ref().map(|h| {
            Self::plan_header_line(
                h,
                &leader.columns,
                leader_geometry,
                size,
                self.style.header_face_from_cell,
            )
        });
        let head_h = head
            .as_ref()
            .map(|p| p.iter().map(|c| c.lines.len()).max().unwrap_or(1) as f32 * leading)
            .unwrap_or(leading);

        let band_levels = timeline
            .axis
            .bands
            .iter()
            .map(|b| b.level)
            .max()
            .map(|m| m as f32 + 1.0)
            .unwrap_or(0.0);
        let axis_h = band_levels * leading;
        let block_h = head_h.max(axis_h);

        self.ensure_room(block_h + self.style.header_rule_drop + self.style.header_rule_gap);

        let top = self.y;
        if let Some(plan) = &head {
            self.draw_header_line(plan, size, self.style.table_header_gray);
        }
        for band in &timeline.axis.bands {
            let x1 = axis_x + timeline.fraction(band.start).value as f32 * axis_w;
            let x2 = axis_x + timeline.fraction(band.end).value as f32 * axis_w;
            self.y = top - band.level as f32 * leading;
            self.text_centered(
                (x1 + x2) / 2.0,
                size,
                Face::Bold,
                self.style.table_header_gray,
                &band.label,
            );
        }

        self.y = top - block_h;
        let rule_y = self.y - self.style.header_rule_drop;
        let (rw, rg) = self.style.rule_header;
        self.line_gray(self.margin, axis_x + axis_w, rule_y, rw, rg);
        self.y = rule_y - self.style.header_rule_gap;
    }

    /// The gridline ticks and the status marker's tick for one row's slot --
    /// factored out of `render_timeline` because both are "a thin vertical
    /// spanning the row slot", differing only in which color/weight.
    fn render_timeline_row_axis(
        &mut self,
        timeline: &Timeline,
        axis_x: f32,
        axis_w: f32,
        slot_bottom: f32,
        row_h: f32,
    ) {
        for &g in &timeline.axis.gridlines {
            let gx = axis_x + timeline.fraction(g).value as f32 * axis_w;
            self.current.push_str(&format!(
                "q {TIMELINE_GRIDLINE_GRAY:.3} G 0.4 w {gx:.2} {slot_bottom:.2} m {gx:.2} {:.2} l S Q\n",
                slot_bottom + row_h
            ));
        }
        if let Some(marker) = &timeline.axis.marker {
            let mx = axis_x + timeline.fraction(marker.at).value as f32 * axis_w;
            self.current.push_str(&format!(
                "q {:.3} {:.3} {:.3} RG [2 2] 0 d 0.75 w {mx:.2} {slot_bottom:.2} m {mx:.2} {:.2} l S Q\n",
                TIMELINE_MARKER_RGB.0,
                TIMELINE_MARKER_RGB.1,
                TIMELINE_MARKER_RGB.2,
                slot_bottom + row_h
            ));
        }
    }

    /// Draws one row's plotted marks within `[axis_x, axis_x+axis_w]`,
    /// vertically centered on the row slot ending at baseline `y` with
    /// height `row_h` -- the same slot convention `render_table`'s
    /// section-banner fill uses (`y - row_h + row_h*0.22` .. `+row_h*0.22`).
    fn render_mark(
        &mut self,
        timeline: &Timeline,
        mark: &Mark,
        axis_x: f32,
        axis_w: f32,
        y: f32,
        row_h: f32,
    ) {
        let px = |u: f64| -> f32 { axis_x + timeline.fraction(u).value as f32 * axis_w };
        let slot_bottom = y - row_h + row_h * 0.22;
        match mark {
            Mark::Bar {
                start,
                end,
                style,
                progress,
                label,
            } => {
                let x1 = px(*start);
                let x2 = px(*end).max(x1 + 1.0);
                let bar_h = row_h * 0.5;
                let bar_bottom = slot_bottom + (row_h - bar_h) / 2.0;
                let rgb = match style {
                    BarStyle::Task => TIMELINE_TASK_RGB,
                    BarStyle::Summary => TIMELINE_SUMMARY_RGB,
                    BarStyle::External => TIMELINE_EXTERNAL_RGB,
                };
                if *style == BarStyle::Summary {
                    // A summary bar draws as a thin bracket, not a solid
                    // fill: a spine plus turned-down end caps via
                    // `fill_polygon` -- the one shape `fill_rect` can't draw.
                    let cap_w = 3.0f32.min((x2 - x1) / 4.0).max(0.5);
                    let spine_h = bar_h * 0.3;
                    self.fill_rect(x1, bar_bottom + bar_h - spine_h, x2 - x1, spine_h, rgb);
                    self.fill_polygon(
                        &[
                            (x1, bar_bottom + bar_h),
                            (x1, bar_bottom),
                            (x1 + cap_w, bar_bottom),
                        ],
                        rgb,
                    );
                    self.fill_polygon(
                        &[
                            (x2, bar_bottom + bar_h),
                            (x2, bar_bottom),
                            (x2 - cap_w, bar_bottom),
                        ],
                        rgb,
                    );
                } else {
                    self.fill_rect(x1, bar_bottom, x2 - x1, bar_h, rgb);
                    if let Some(p) = progress {
                        let pw = (x2 - x1) * p.clamp(0.0, 1.0);
                        if pw > 0.0 {
                            self.fill_rect(x1, bar_bottom, pw, bar_h, TIMELINE_PROGRESS_RGB);
                        }
                    }
                }
                self.render_mark_label(label, x1, x2, slot_bottom, row_h);
            }
            Mark::Milestone { at, shape, label } => {
                let cx = px(*at);
                let cy = slot_bottom + row_h * 0.5;
                let r = (row_h * 0.35).min(6.0);
                match shape {
                    MilestoneShape::Diamond => self.fill_polygon(
                        &[(cx, cy + r), (cx + r, cy), (cx, cy - r), (cx - r, cy)],
                        TIMELINE_MILESTONE_RGB,
                    ),
                    MilestoneShape::Triangle => self.fill_polygon(
                        &[(cx, cy + r), (cx + r, cy - r), (cx - r, cy - r)],
                        TIMELINE_MILESTONE_RGB,
                    ),
                    MilestoneShape::Bar => {
                        self.fill_rect(cx - 1.0, cy - r, 2.0, r * 2.0, TIMELINE_MILESTONE_RGB)
                    }
                }
                self.render_mark_label(label, cx - r, cx + r, slot_bottom, row_h);
            }
        }
    }

    fn render_mark_label(
        &mut self,
        label: &Option<crate::doc::MarkLabel>,
        x1: f32,
        x2: f32,
        slot_bottom: f32,
        row_h: f32,
    ) {
        let Some(lbl) = label else { return };
        let size = self.style.note_size;
        let gray = self.style.body_gray;
        let face = if lbl.bold { Face::Bold } else { Face::Regular };
        let ty = slot_bottom + row_h * 0.36;
        let saved_y = self.y;
        self.y = ty;
        match lbl.placement {
            LabelPlacement::Before => {
                self.text_right_aligned(x1 - 3.0, size, face, gray, &lbl.text)
            }
            LabelPlacement::After => self.text(x2 + 3.0, size, face, gray, &lbl.text),
            LabelPlacement::Inside => {
                self.text_centered((x1 + x2) / 2.0, size, face, gray, &lbl.text)
            }
        }
        self.y = saved_y;
    }

    /// One legend line under the plotted panel: a small sample swatch per
    /// entry (a short bar or a milestone glyph) followed by its caption.
    fn render_timeline_legend(
        &mut self,
        legend: &[crate::doc::LegendEntry],
        axis_x: f32,
        axis_w: f32,
    ) {
        let size = self.style.note_size;
        let leading = size * self.style.leading_factor;
        self.ensure_room(leading + 4.0);
        self.advance(4.0);
        let mut x = axis_x;
        let saved_y = self.y;
        for entry in legend {
            let swatch_w = 18.0;
            match &entry.sample {
                crate::doc::LegendSample::Bar(style) => {
                    let rgb = match style {
                        BarStyle::Task => TIMELINE_TASK_RGB,
                        BarStyle::Summary => TIMELINE_SUMMARY_RGB,
                        BarStyle::External => TIMELINE_EXTERNAL_RGB,
                    };
                    self.fill_rect(x, self.y - size * 0.35, swatch_w, size * 0.7, rgb);
                }
                crate::doc::LegendSample::Milestone(shape) => {
                    let cx = x + swatch_w / 2.0;
                    let cy = self.y;
                    let r = size * 0.4;
                    match shape {
                        MilestoneShape::Diamond => self.fill_polygon(
                            &[(cx, cy + r), (cx + r, cy), (cx, cy - r), (cx - r, cy)],
                            TIMELINE_MILESTONE_RGB,
                        ),
                        MilestoneShape::Triangle => self.fill_polygon(
                            &[(cx, cy + r), (cx + r, cy - r), (cx - r, cy - r)],
                            TIMELINE_MILESTONE_RGB,
                        ),
                        MilestoneShape::Bar => {
                            self.fill_rect(cx - 0.75, cy - r, 1.5, r * 2.0, TIMELINE_MILESTONE_RGB)
                        }
                    }
                }
            }
            x += swatch_w + 4.0;
            self.text(x, size, Face::Regular, self.style.body_gray, &entry.caption);
            x += text_width_pt(&entry.caption, size, false) + 16.0;
            if x > axis_x + axis_w {
                break; // legend overflow: v1 doesn't wrap to a second line
            }
        }
        self.y = saved_y;
        self.advance(leading);
    }
}

pub fn render_pdf(doc: &Doc) -> Vec<u8> {
    let mut layout = Layout::new(doc);

    for block in &doc.blocks {
        match block {
            Block::Masthead {
                entity_name,
                statement_title,
                period_label,
            } => {
                let s = layout.style;
                let (e_size, e_gray) = s.masthead_entity;
                let (t_size, t_gray) = s.masthead_title;
                let (p_size, p_gray) = s.masthead_period;
                let lf = s.leading_factor;
                layout.ensure_room((e_size + t_size + p_size) * lf + 24.0);
                let lines = [
                    (entity_name, e_size, e_gray, Face::Bold),
                    (statement_title, t_size, t_gray, s.masthead_title_face),
                    (period_label, p_size, p_gray, Face::Oblique),
                ];
                let cx = layout.margin + layout.content_width / 2.0;
                for (text, size, gray, face) in lines {
                    for line in Layout::wrap(text, layout.content_width, size, face.is_bold()) {
                        if s.masthead_centered {
                            layout.text_centered(cx, size, face, gray, &line);
                        } else {
                            layout.text(layout.margin, size, face, gray, &line);
                        }
                        layout.advance(size * lf);
                    }
                }
                layout.advance(s.masthead_gap_above_rule);
                match s.masthead_rule_rgb {
                    Some(rgb) => layout.line_rgb(
                        layout.margin,
                        layout.right_edge(),
                        layout.y,
                        s.masthead_rule_w,
                        rgb,
                    ),
                    None => layout.line_gray(
                        layout.margin,
                        layout.right_edge(),
                        layout.y,
                        s.masthead_rule_w,
                        0.0,
                    ),
                }
                layout.advance(s.masthead_gap_below_rule);
            }
            Block::CoverTitle {
                entity_name,
                statement_title,
                period_label,
                currency_label,
            } => {
                // The cover of a formal document bundle: centered, bold,
                // unruled, and set in one larger size step than the pages
                // that follow.
                let lf = layout.style.leading_factor;
                layout.advance(layout.style.cover_top_drop);
                let cx = layout.margin + layout.content_width / 2.0;
                let blocks: [(&str, f32, Face, f32); 3] = [
                    (entity_name, 16.0, Face::Bold, 10.0),
                    (statement_title, 14.0, Face::Bold, 2.0),
                    (period_label, 14.0, Face::Bold, 30.0),
                ];
                for (text, size, face, gap_after) in blocks {
                    for line in Layout::wrap(text, layout.content_width, size, face.is_bold()) {
                        layout.ensure_room(size * lf);
                        layout.text_centered(cx, size, face, 0.0, &line);
                        layout.advance(size * lf);
                    }
                    layout.advance(gap_after);
                }
                for line in Layout::wrap(currency_label, layout.content_width, 10.0, false) {
                    layout.ensure_room(10.0 * lf);
                    layout.text_centered(cx, 10.0, Face::Oblique, 0.0, &line);
                    layout.advance(10.0 * lf);
                }
            }
            Block::Heading { level, text } => {
                let (size, gray) = HEADING_STYLE[(*level).clamp(1, 3) as usize - 1];
                let leading = size * layout.style.leading_factor;
                // Orphan control: reserve the heading plus a few body lines
                // so a heading never lands alone at the page foot.
                layout.ensure_room(leading * 1.5 + 3.0 * layout.body_leading());
                layout.text(layout.margin, size, Face::Bold, gray, text);
                layout.advance(leading * 0.3);
                if *level == 1 {
                    layout.line_gray(
                        layout.margin,
                        layout.margin + layout.content_width,
                        layout.y,
                        RULE_H1.0,
                        RULE_H1.1,
                    );
                    layout.advance(3.0);
                }
                layout.advance(leading * 0.6);
            }
            Block::Para { spans, style } => {
                layout.advance(style.space_before_pt);
                let (size, gray) = (layout.style.body_size, layout.style.body_gray);
                layout.render_paragraph(spans, size, gray, Face::Regular, style.left_indent_pt);
                layout.advance(layout.para_space_after(style, size));
            }
            Block::Note(spans) => {
                let (n_size, n_gray, n_face) = (
                    layout.style.note_size,
                    layout.style.note_gray,
                    layout.style.note_face,
                );
                layout.render_paragraph(spans, n_size, n_gray, n_face, 0.0);
                layout.advance(layout.para_space_after(&ParaStyle::default(), n_size));
            }
            Block::Table(table) => layout.render_table(table),
            Block::Rule => {
                layout.ensure_room(6.0);
                layout.line_gray(
                    layout.margin,
                    layout.margin + layout.content_width,
                    layout.y,
                    layout.style.rule_header.0,
                    layout.style.rule_header.1,
                );
                layout.advance(6.0);
            }
            Block::Spacer(pt) => layout.advance(*pt),
            Block::PageBreak => layout.new_page(),
            Block::Footer(text) => {
                layout.ensure_room(layout.style.note_size * layout.style.leading_factor + 8.0);
                layout.line_gray(
                    layout.margin,
                    layout.margin + layout.content_width,
                    layout.y,
                    RULE_FOOTER.0,
                    RULE_FOOTER.1,
                );
                layout.advance(8.0);
                let (n_size, n_gray, n_face) = (
                    layout.style.note_size,
                    layout.style.note_gray,
                    layout.style.note_face,
                );
                layout.render_paragraph(&[Span::Text(text.clone())], n_size, n_gray, n_face, 0.0);
            }
            Block::Placeholder { kind, caption } => {
                let leading = layout.body_leading();
                let pad = 10.0;
                let text_width = layout.content_width - 2.0 * pad;
                let full_caption = format!("[{kind}: {caption}]");
                let lines = Layout::wrap(&full_caption, text_width, NOTE_SIZE, false);
                let box_h = leading * lines.len() as f32 + pad * 2.0;
                layout.ensure_room(box_h);
                let y_top = layout.y;
                layout.current.push_str(&format!(
                    "q 0.400 G [3 2] 0 d 0.5 w {:.2} {:.2} {:.2} {:.2} re S Q\n",
                    layout.margin,
                    y_top - box_h,
                    layout.content_width,
                    box_h
                ));
                layout.advance(pad);
                let cx = layout.margin + layout.content_width / 2.0;
                for line in &lines {
                    layout.text_centered(cx, NOTE_SIZE, Face::Oblique, NOTE_GRAY, line);
                    layout.advance(leading);
                }
                layout.advance(pad - leading + lines.len() as f32 * 0.0);
                layout.y = y_top - box_h - leading * 0.5;
            }
            Block::Timeline(timeline) => layout.render_timeline(timeline),
        }
    }
    layout.pages.push(std::mem::take(&mut layout.current));

    // Running header/footer + page numbers, applied per finished page.
    let page_count = layout.pages.len();
    let mut finished_pages = Vec::with_capacity(page_count);
    for (idx, body) in layout.pages.iter().enumerate() {
        let mut page = String::new();
        let show_header =
            doc.page.running_header.is_some() && !(idx == 0 && doc.page.suppress_header_on_first);
        if let (true, Some(header)) = (show_header, &doc.page.running_header) {
            let w = text_width_pt(header, HEADER_FOOTER_SIZE, false);
            page.push_str(&format!(
                "q {HEADER_FOOTER_GRAY:.3} g BT /F1 {HEADER_FOOTER_SIZE} Tf 1 0 0 1 {:.2} {:.2} Tm ({}) Tj ET Q\n",
                (layout.margin + layout.content_width - w).max(layout.margin),
                layout.page_h - layout.margin / 2.0,
                escape_pdf_string(header)
            ));
        }
        if doc.page.page_numbers {
            let label = format!("- {} -", idx + 1);
            let w = text_width_pt(&label, HEADER_FOOTER_SIZE, false);
            let gray = layout.style.footer_gray;
            page.push_str(&format!(
                "q {gray:.3} g BT /F1 {HEADER_FOOTER_SIZE} Tf 1 0 0 1 {:.2} {:.2} Tm ({}) Tj ET Q\n",
                layout.margin + layout.content_width / 2.0 - w / 2.0,
                layout.margin / 2.0,
                escape_pdf_string(&label)
            ));
        }
        page.push_str(body);
        finished_pages.push(page);
    }

    build_pdf(&finished_pages, layout.page_w, layout.page_h)
}

fn build_pdf(pages: &[String], page_w: f32, page_h: f32) -> Vec<u8> {
    let mut w = PdfWriter::new();

    let font_regular = w.add_object(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_vec(),
    );
    let font_bold = w.add_object(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>"
            .to_vec(),
    );
    let font_oblique = w.add_object(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique /Encoding /WinAnsiEncoding >>".to_vec(),
    );
    let font_bold_oblique = w.add_object(
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-BoldOblique /Encoding /WinAnsiEncoding >>".to_vec(),
    );

    let pages_obj = w.reserve();
    let mut kids = Vec::new();

    for page_content in pages {
        let content_obj = w.add_object({
            let mut body = format!("<< /Length {} >>\nstream\n", page_content.len()).into_bytes();
            body.extend_from_slice(page_content.as_bytes());
            body.extend_from_slice(b"\nendstream");
            body
        });
        let page_obj = w.add_object(
            format!(
                "<< /Type /Page /Parent {pages_obj} 0 R /MediaBox [0 0 {page_w} {page_h}] \
                 /Resources << /Font << /F1 {font_regular} 0 R /F2 {font_bold} 0 R \
                 /F3 {font_oblique} 0 R /F4 {font_bold_oblique} 0 R >> >> \
                 /Contents {content_obj} 0 R >>"
            )
            .into_bytes(),
        );
        kids.push(page_obj);
    }

    let kids_refs: String = kids.iter().map(|k| format!("{k} 0 R ")).collect();
    w.set_object(
        pages_obj,
        format!(
            "<< /Type /Pages /Kids [ {kids_refs}] /Count {} >>",
            kids.len()
        )
        .into_bytes(),
    );

    let catalog = w.add_object(format!("<< /Type /Catalog /Pages {pages_obj} 0 R >>").into_bytes());

    w.finish(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::{Cell, Column, PageSetup, Row, Table};

    fn stream(doc: &Doc) -> String {
        String::from_utf8_lossy(&render_pdf(doc)).into_owned()
    }

    /// Every `... x y Tm (text) Tj` in the content stream, as (x, text).
    /// Enough to assert *where* a glyph run landed, which is the whole point
    /// of an indent or a wrapped column heading.
    fn placements(pdf: &str) -> Vec<(f32, String)> {
        let mut out = Vec::new();
        for line in pdf.lines() {
            let (Some(tm), Some(open)) = (line.find(" Tm ("), line.rfind(") Tj")) else {
                continue;
            };
            let x = line[..tm]
                .rsplit(' ')
                .nth(1)
                .and_then(|s| s.parse::<f32>().ok());
            if let Some(x) = x {
                out.push((x, line[tm + 5..open].to_string()));
            }
        }
        out
    }

    fn sample(register: Register, rules_from_col: Option<usize>) -> Doc {
        let mut table = Table::new(vec![
            Column {
                width: ColWidth::Pct(60.0),
                align: Align::Left,
            },
            Column {
                width: ColWidth::Pct(40.0),
                align: Align::Right,
            },
        ]);
        table.rules_from_col = rules_from_col;
        table.header = Some(vec![
            Cell::text(""),
            Cell::text("At December 31, 2021").bold(),
        ]);
        table.subheader = Some(vec![Cell::text(""), Cell::text("$").bold()]);
        table.rows = vec![
            Row::section_banner("ASSETS", 2),
            Row::data(vec![Cell::text("Cash").indent(1), Cell::figure("\u{2013}")]),
            Row::total(vec![
                Cell::text("Total assets").bold(),
                Cell::figure("1,585").bold(),
            ]),
        ];
        Doc {
            page: PageSetup {
                register,
                ..Default::default()
            },
            blocks: vec![Block::Table(table)],
        }
    }

    #[test]
    fn formal_register_paints_no_fill_and_no_accent_color() {
        let formal = stream(&sample(Register::FormalStatement, Some(1)));
        assert!(
            !formal.contains(" re f"),
            "a filed statement has no section-banner fill"
        );
        assert!(
            !formal.contains(" RG"),
            "a filed statement has no non-black stroke color"
        );
        assert!(
            !formal.contains(" rg"),
            "a filed statement has no non-black fill color"
        );
    }

    #[test]
    fn working_register_keeps_its_banner_fill_and_accent() {
        // The working-document look must survive the formal register's
        // arrival -- this is the regression guard for that.
        let working = stream(&sample(Register::WorkingDocument, None));
        assert!(
            working.contains(" re f"),
            "the working register still fills section banners"
        );
        let masthead = Doc {
            page: PageSetup::default(),
            blocks: vec![Block::Masthead {
                entity_name: "E".into(),
                statement_title: "T".into(),
                period_label: "P".into(),
            }],
        };
        assert!(
            stream(&masthead).contains(" RG"),
            "the working masthead keeps its one accent rule"
        );
    }

    #[test]
    fn bold_lead_span_stays_bold_while_running_text_keeps_its_face() {
        // Parity with the HTML backend, which has always rendered a
        // `Span::Bold` as `<strong>` on that span only -- the PDF backend
        // must not promote the whole paragraph to bold because of a lead-in.
        let doc = Doc {
            page: PageSetup::default(),
            blocks: vec![Block::para(vec![
                Span::Bold("LEADTERM".to_string()),
                Span::Text(" means the rest of this paragraph stays regular".to_string()),
            ])],
        };
        let pdf = stream(&doc);
        let lead = pdf
            .lines()
            .find(|l| l.contains("(LEADTERM) Tj"))
            .expect("bold lead run");
        assert!(lead.contains("/F2"), "the lead span renders bold: {lead}");
        let rest = pdf
            .lines()
            .find(|l| l.contains("(means the rest"))
            .expect("regular continuation run");
        assert!(
            rest.contains("/F1"),
            "the running text keeps the regular face: {rest}"
        );
    }

    #[test]
    fn formal_register_rules_only_ordinary_rows_at_totals() {
        // Filed statements draw no hairline under every data row; the
        // working register does. Count stroke ops as a proxy.
        let formal = stream(&sample(Register::FormalStatement, Some(1)));
        let working = stream(&sample(Register::WorkingDocument, None));
        let strokes = |s: &str| s.matches(" l S Q").count();
        assert!(
            strokes(&formal) < strokes(&working),
            "formal ({}) should draw fewer rules than working ({})",
            strokes(&formal),
            strokes(&working)
        );
    }

    #[test]
    fn rules_start_at_the_named_column_not_the_margin() {
        let spanning = stream(&sample(Register::FormalStatement, None));
        let money_only = stream(&sample(Register::FormalStatement, Some(1)));
        let first_rule_x = |s: &str| {
            s.lines()
                .find(|l| l.contains(" l S Q"))
                .and_then(|l| l.split_whitespace().nth(5))
                .and_then(|v| v.parse::<f32>().ok())
                .expect("a rule")
        };
        assert!(
            (first_rule_x(&spanning) - 72.0).abs() < 0.51,
            "unrestricted rules start at the margin"
        );
        // 60% of a 468pt content width, plus the 72pt margin.
        assert!((first_rule_x(&money_only) - (72.0 + 0.6 * 468.0)).abs() < 0.51);
    }

    #[test]
    fn indent_level_is_a_real_offset_not_leading_spaces() {
        let pdf = stream(&sample(Register::FormalStatement, Some(1)));
        let places = placements(&pdf);
        let cash = places.iter().find(|(_, t)| t == "Cash").expect("Cash");
        let total = places
            .iter()
            .find(|(_, t)| t == "Total assets")
            .expect("Total assets");
        assert!(
            !cash.1.starts_with(' '),
            "the indent must not be spaces in the text"
        );
        assert!(
            (cash.0 - total.0 - FORMAL_STATEMENT.indent_pt[1]).abs() < 0.01,
            "indent level 1 should offset by exactly {}pt, got {}",
            FORMAL_STATEMENT.indent_pt[1],
            cash.0 - total.0
        );
    }

    #[test]
    fn indent_levels_3_and_4_no_longer_collapse_onto_level_2() {
        // Regression test: the array used to be 3 entries, so level 3+
        // clamped onto index 2's 28pt offset in the PDF output, not just
        // the HTML review copy.
        let mut table = Table::new(vec![Column {
            width: ColWidth::Pct(100.0),
            align: Align::Left,
        }]);
        table.rows = vec![
            Row::data(vec![Cell::text("L2").indent(2)]),
            Row::data(vec![Cell::text("L3").indent(3)]),
            Row::data(vec![Cell::text("L4").indent(4)]),
        ];
        let doc = Doc {
            page: PageSetup {
                register: Register::FormalStatement,
                ..Default::default()
            },
            blocks: vec![Block::Table(table)],
        };
        let pdf = stream(&doc);
        let places = placements(&pdf);
        let l2 = places.iter().find(|(_, t)| t == "L2").expect("L2").0;
        let l3 = places.iter().find(|(_, t)| t == "L3").expect("L3").0;
        let l4 = places.iter().find(|(_, t)| t == "L4").expect("L4").0;
        assert_eq!(FORMAL_STATEMENT.indent_pt.len(), 5);
        assert!(
            (l3 - l2 - (FORMAL_STATEMENT.indent_pt[3] - FORMAL_STATEMENT.indent_pt[2])).abs()
                < 0.01,
            "level 3 must sit at its own offset, not collapse onto level 2: l2={l2} l3={l3}"
        );
        assert!(
            (l4 - l3 - (FORMAL_STATEMENT.indent_pt[4] - FORMAL_STATEMENT.indent_pt[3])).abs()
                < 0.01,
            "level 4 must sit at its own offset, not collapse onto level 2 or 3: l3={l3} l4={l4}"
        );
    }

    #[test]
    fn landscape_swaps_page_dimensions() {
        let portrait = Doc {
            page: PageSetup::default(),
            blocks: vec![],
        };
        let landscape = Doc {
            page: PageSetup {
                landscape: true,
                ..Default::default()
            },
            blocks: vec![],
        };
        let p = Layout::new(&portrait);
        let l = Layout::new(&landscape);
        assert_eq!(
            (p.page_w, p.page_h),
            (612.0, 792.0),
            "portrait Letter, unchanged"
        );
        assert_eq!(
            (l.page_w, l.page_h),
            (792.0, 612.0),
            "landscape swaps width/height rather than doing nothing"
        );
        assert!(
            stream(&landscape).contains("/MediaBox [0 0 792 612]"),
            "the swapped dims must reach the actual PDF MediaBox, not just Layout's fields"
        );
    }

    #[test]
    fn page_size_tabloid_and_legal_are_landscape_usable_for_a_wide_report() {
        assert_eq!(crate::doc::PageSize::Tabloid.dims_pt(), (792.0, 1224.0));
        assert_eq!(crate::doc::PageSize::Legal.dims_pt(), (612.0, 1008.0));
    }

    #[test]
    fn long_column_headings_wrap_instead_of_truncating() {
        // 17% of a 468pt content width == ~80pt, the real roll-forward
        // column width this heading has to survive.
        let mut table = Table::new(vec![
            Column {
                width: ColWidth::Pct(83.0),
                align: Align::Left,
            },
            Column {
                width: ColWidth::Pct(17.0),
                align: Align::Right,
            },
        ]);
        table.header = Some(vec![
            Cell::text(""),
            Cell::text("Share-based payment reserve").bold(),
        ]);
        let doc = Doc {
            page: PageSetup {
                register: Register::FormalStatement,
                ..Default::default()
            },
            blocks: vec![Block::Table(table)],
        };
        let pdf = stream(&doc);
        assert!(
            !pdf.contains('\u{2026}') && !pdf.contains("\\205"),
            "a heading must never be ellipsized away"
        );
        let texts: Vec<String> = placements(&pdf).into_iter().map(|(_, t)| t).collect();
        assert!(
            texts.iter().any(|t| t == "Share-based"),
            "expected a wrapped first line, got {texts:?}"
        );
        assert!(
            texts.iter().any(|t| t.contains("reserve")),
            "expected the tail to survive, got {texts:?}"
        );
    }

    #[test]
    fn wrapped_heading_lines_bottom_align_onto_the_rule() {
        // The last line of a 3-line heading must share its baseline with a
        // neighbouring 1-line heading, or the columns read as misaligned.
        let mut table = Table::new(vec![
            Column {
                width: ColWidth::Pct(83.0),
                align: Align::Right,
            },
            Column {
                width: ColWidth::Pct(17.0),
                align: Align::Right,
            },
        ]);
        table.header = Some(vec![
            Cell::text("Deficit").bold(),
            Cell::text("Share-based payment reserve").bold(),
        ]);
        let doc = Doc {
            page: PageSetup {
                register: Register::FormalStatement,
                ..Default::default()
            },
            blocks: vec![Block::Table(table)],
        };
        let pdf = stream(&doc);
        let baseline = |needle: &str| -> f32 {
            pdf.lines()
                .find(|l| l.contains(&format!("({needle}) Tj")))
                .and_then(|l| l.split(" Tm ").next())
                .and_then(|l| l.rsplit(' ').next())
                .and_then(|v| v.parse::<f32>().ok())
                .expect("baseline")
        };
        assert!((baseline("Deficit") - baseline("reserve")).abs() < 0.01);
    }

    #[test]
    fn currency_subheader_prints_the_dollar_sign_exactly_once() {
        let pdf = stream(&sample(Register::FormalStatement, Some(1)));
        assert_eq!(
            pdf.matches("($) Tj").count(),
            1,
            "one $ per statement, never per row"
        );
    }

    #[test]
    fn en_dash_zero_placeholder_reaches_the_stream_as_winansi() {
        let pdf = stream(&sample(Register::FormalStatement, Some(1)));
        assert!(
            pdf.contains("(\\226) Tj"),
            "U+2013 should encode to WinAnsi byte 0x96"
        );
    }

    // ---- Justified paragraph setting -------------------------------------

    /// One drawn glyph run, with everything justification is asserted on:
    /// where it starts, on which baseline, and how far `Tw` was told to
    /// widen its spaces.
    struct Run {
        x: f32,
        y: f32,
        tw: f32,
        size: f32,
        bold: bool,
        text: String,
    }

    impl Run {
        /// Where this run ends, counting the stretch `Tw` adds at each of
        /// its own spaces -- the number the right margin is checked against.
        fn end_x(&self) -> f32 {
            self.x
                + text_width_pt(&self.text, self.size, self.bold)
                + self.tw * self.text.matches(' ').count() as f32
        }
    }

    fn runs(pdf: &str) -> Vec<Run> {
        let mut out = Vec::new();
        for line in pdf.lines() {
            let (Some(tm), Some(close)) = (line.find(" Tm ("), line.rfind(") Tj")) else {
                continue;
            };
            let head = &line[..tm];
            let mut coords = head.rsplit(' ');
            let y = coords
                .next()
                .and_then(|v| v.parse::<f32>().ok())
                .unwrap_or(0.0);
            let x = coords
                .next()
                .and_then(|v| v.parse::<f32>().ok())
                .unwrap_or(0.0);
            let size = head
                .split(" Tf")
                .next()
                .and_then(|s| s.rsplit(' ').next())
                .and_then(|v| v.parse::<f32>().ok())
                .unwrap_or(0.0);
            let tw = match head.find(" Tw") {
                Some(i) => head[..i]
                    .rsplit(' ')
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0),
                None => 0.0,
            };
            out.push(Run {
                x,
                y,
                tw,
                size,
                bold: head.contains("/F2"),
                text: line[tm + 5..close].to_string(),
            });
        }
        out
    }

    /// The runs of one paragraph grouped into lines, top of the page down.
    /// Page furniture (the page number, set 7.5pt at the foot) is dropped so
    /// a paragraph assertion is only ever about the paragraph.
    fn lines_of(pdf: &str) -> Vec<Vec<Run>> {
        let mut baselines: Vec<f32> = Vec::new();
        let mut grouped: Vec<Vec<Run>> = Vec::new();
        for run in runs(pdf).into_iter().filter(|r| r.size >= 8.0) {
            match baselines.iter().position(|&y| (y - run.y).abs() < 0.01) {
                Some(i) => grouped[i].push(run),
                None => {
                    baselines.push(run.y);
                    grouped.push(vec![run]);
                }
            }
        }
        for line in grouped.iter_mut() {
            line.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        }
        grouped
    }

    fn formal(blocks: Vec<Block>) -> Doc {
        Doc {
            page: PageSetup {
                register: Register::FormalStatement,
                ..Default::default()
            },
            blocks,
        }
    }

    // Synthetic prose, chosen only for its length and word-wrap behavior --
    // not sourced from any real document. Long enough to force several
    // justified lines, which is the only property these tests need.
    const PROSE: &str = "This sample passage exists only to give the layout engine several lines of ordinary prose to wrap and justify. It has no meaning beyond that: the words are chosen for their length and syllable count, not for any real-world claim, and none of the sentences describe an actual company, person, or event.";

    /// The right margin of a Letter page at the engine's 72pt margins.
    const RIGHT_EDGE: f32 = 612.0 - 72.0;

    #[test]
    fn every_justified_line_but_the_last_ends_exactly_on_the_right_margin() {
        let pdf = stream(&formal(vec![Block::para(vec![Span::Text(
            PROSE.to_string(),
        )])]));
        let lines = lines_of(&pdf);
        assert!(
            lines.len() >= 3,
            "expected a paragraph of several lines, got {}",
            lines.len()
        );
        for (i, line) in lines.iter().enumerate().take(lines.len() - 1) {
            let end = line.last().unwrap().end_x();
            assert!(
                (end - RIGHT_EDGE).abs() < 0.02,
                "line {i} should be flush to {RIGHT_EDGE}, ended at {end}"
            );
        }
    }

    #[test]
    fn the_last_line_of_a_paragraph_stays_ragged() {
        // The whole point of justified setting: the final line is never
        // stretched, so it must carry no word spacing at all.
        let pdf = stream(&formal(vec![Block::para(vec![Span::Text(
            PROSE.to_string(),
        )])]));
        let lines = lines_of(&pdf);
        let last = lines.last().expect("a last line");
        assert!(
            last.iter().all(|r| r.tw == 0.0),
            "the last line must not be stretched"
        );
        assert!(
            last.last().unwrap().end_x() < RIGHT_EDGE - 1.0,
            "the last line should fall short of the margin, not reach it"
        );
    }

    #[test]
    fn the_working_register_never_stretches_a_word_space() {
        // Its ragged right edge is a real design choice, not an omission --
        // this is the guard that a justification pass never leaks into it.
        let doc = Doc {
            page: PageSetup::default(),
            blocks: vec![Block::para(vec![Span::Text(PROSE.to_string())])],
        };
        assert!(
            !stream(&doc).contains(" Tw"),
            "the working register sets no word spacing"
        );
    }

    #[test]
    fn a_one_word_line_is_left_alone() {
        // Nothing to stretch: a single word has no internal space, so the
        // line cannot be justified and must not be mangled trying.
        let pdf = stream(&formal(vec![Block::para(vec![Span::Text(
            "Word".to_string(),
        )])]));
        assert!(
            !pdf.contains(" Tw"),
            "a one-word paragraph needs no word spacing"
        );
    }

    #[test]
    fn a_line_left_short_by_an_unbreakable_word_stays_ragged() {
        // Two short words followed by something far too long to fit beside
        // them: stretching that line onto the margin would put inches
        // between two words, so it is left ragged instead.
        let long = "A".repeat(120);
        let pdf = stream(&formal(vec![Block::para(vec![Span::Text(format!(
            "one two {long}"
        ))])]));
        let lines = lines_of(&pdf);
        assert_eq!(lines[0].last().unwrap().text, "one two");
        assert_eq!(
            lines[0][0].tw, 0.0,
            "an absurd stretch must be declined, not printed"
        );
    }

    #[test]
    fn body_leading_is_exactly_twelve_points_not_a_multiple_of_the_font_size() {
        let pdf = stream(&formal(vec![Block::para(vec![Span::Text(
            PROSE.to_string(),
        )])]));
        let lines = lines_of(&pdf);
        for pair in lines.windows(2) {
            let delta = pair[0][0].y - pair[1][0].y;
            assert!(
                (delta - 12.0).abs() < 0.01,
                "expected an exact 12pt line, got {delta}"
            );
        }
    }

    #[test]
    fn consecutive_paragraphs_are_ten_points_apart() {
        let one = Block::para(vec![Span::Text("First paragraph.".to_string())]);
        let two = Block::para(vec![Span::Text("Second paragraph.".to_string())]);
        let pdf = stream(&formal(vec![one, two]));
        let lines = lines_of(&pdf);
        let gap = lines[0][0].y - lines[1][0].y;
        // One 12pt line plus the style's own 10pt space after.
        assert!(
            (gap - 22.0).abs() < 0.01,
            "expected 12pt line + 10pt gap, got {gap}"
        );
    }

    #[test]
    fn a_tab_lands_the_text_after_it_on_the_registers_own_tab_stop() {
        // "(a)\tStatement of compliance" is a realistic literal paragraph
        // text, and its caption starts at the default tab stop -- not one
        // space past a label whose own width would then set the layout.
        let pdf = stream(&formal(vec![Block::Para {
            spans: vec![Span::Bold("(a)\tStatement of compliance".to_string())],
            style: ParaStyle::default(),
        }]));
        let lines = lines_of(&pdf);
        assert_eq!(
            lines[0].len(),
            2,
            "the label and the caption are separate runs"
        );
        assert!(
            (lines[0][0].x - 72.0).abs() < 0.01,
            "the label stays flush left"
        );
        assert!(
            (lines[0][1].x - (72.0 + FORMAL_STATEMENT.tab_stop_pt.unwrap())).abs() < 0.01,
            "the caption belongs on the 36pt stop, landed at {}",
            lines[0][1].x
        );
        assert!(
            !pdf.contains(" Tw"),
            "a tab positions the line; it is never justified"
        );
    }

    #[test]
    fn a_left_indent_moves_every_line_including_the_first() {
        // A defined-term paragraph's first line indents exactly as far as
        // its wrapped continuation lines do.
        let pdf = stream(&formal(vec![Block::Para {
            spans: vec![Span::Text(PROSE.to_string())],
            style: ParaStyle {
                left_indent_pt: 14.2,
                ..ParaStyle::default()
            },
        }]));
        let lines = lines_of(&pdf);
        assert!(lines.len() >= 2, "expected the paragraph to wrap");
        for (i, line) in lines.iter().enumerate() {
            assert!(
                (line[0].x - 86.2).abs() < 0.01,
                "line {i} should start at 86.2, got {}",
                line[0].x
            );
        }
        // ... and the indent narrows the measure rather than overhanging it.
        let end = lines[0].last().unwrap().end_x();
        assert!(
            (end - RIGHT_EDGE).abs() < 0.02,
            "an indented line still justifies to {RIGHT_EDGE}"
        );
    }

    #[test]
    fn a_bold_lead_in_keeps_its_stretch_in_step_with_the_rest_of_the_line() {
        // A mixed-face line is drawn as two runs; if the second one ignored
        // the stretch applied inside the first it would land back on the
        // unjustified position and overlap or gap.
        let pdf = stream(&formal(vec![Block::para(vec![
            Span::Bold("SAMPLE DEFINED TERM".to_string()),
            Span::Text(format!(" means {PROSE}")),
        ])]));
        let lines = lines_of(&pdf);
        let first = &lines[0];
        assert!(first.len() >= 2, "expected a bold run and a regular run");
        assert!(first[0].bold && !first[1].bold);
        assert_eq!(
            first[0].tw, first[1].tw,
            "both runs share the line's stretch"
        );
        let gap = first[1].x - first[0].end_x();
        let space = text_width_pt(" ", 10.0, false) + first[0].tw;
        assert!(
            (gap - space).abs() < 0.02,
            "the run gap should be one stretched space, got {gap}"
        );
    }

    fn timeline_doc(marks: Vec<crate::doc::TimelineRow>) -> Doc {
        let mut leader = Table::new(vec![Column {
            width: ColWidth::Pct(100.0),
            align: Align::Left,
        }]);
        for i in 0..marks.len() {
            leader
                .rows
                .push(Row::data(vec![Cell::text(format!("Task {i}"))]));
        }
        let axis = crate::doc::TimeAxis {
            start: 0.0,
            end: 100.0,
            bands: vec![crate::doc::AxisBand {
                label: "Q1".into(),
                start: 0.0,
                end: 100.0,
                level: 0,
            }],
            gridlines: vec![25.0, 50.0, 75.0],
            marker: Some(crate::doc::AxisMarker {
                at: 40.0,
                label: Some("Status date".into()),
            }),
        };
        let timeline = crate::doc::Timeline::with_leader(leader, ColWidth::Pct(30.0), axis, marks)
            .expect("valid timeline");
        Doc {
            page: PageSetup {
                landscape: true,
                ..Default::default()
            },
            blocks: vec![Block::Timeline(timeline)],
        }
    }

    #[test]
    fn a_bar_mark_reaches_the_stream_as_a_filled_rect() {
        let doc = timeline_doc(vec![crate::doc::TimelineRow::one(Mark::Bar {
            start: 10.0,
            end: 60.0,
            style: BarStyle::Task,
            progress: None,
            label: None,
        })]);
        let pdf = stream(&doc);
        assert!(
            pdf.contains(" re f Q"),
            "expected at least one filled rect for the bar"
        );
    }

    #[test]
    fn a_milestone_diamond_reaches_the_stream_as_a_filled_polygon() {
        let doc = timeline_doc(vec![crate::doc::TimelineRow::one(Mark::Milestone {
            at: 50.0,
            shape: MilestoneShape::Diamond,
            label: Some(crate::doc::MarkLabel {
                text: "Approval".into(),
                placement: LabelPlacement::After,
                bold: false,
            }),
        })]);
        let pdf = stream(&doc);
        assert!(
            pdf.contains(" m ") && pdf.contains(" h f Q"),
            "expected a closed filled polygon (m ... l ... h f Q) for the milestone"
        );
        assert!(
            pdf.contains("Approval"),
            "the mark's label must reach the stream"
        );
    }

    #[test]
    fn an_empty_row_still_occupies_its_slot_keeping_leader_and_plot_aligned() {
        // Two leader rows, only the second has a mark -- the first row must
        // still advance the plot by one full row_h, or the second mark
        // would land beside the wrong leader label.
        let doc = timeline_doc(vec![
            crate::doc::TimelineRow::empty(),
            crate::doc::TimelineRow::one(Mark::Milestone {
                at: 50.0,
                shape: MilestoneShape::Bar,
                label: None,
            }),
        ]);
        let pdf = stream(&doc);
        let places = placements(&pdf);
        assert!(
            places.iter().any(|(_, t)| t == "Task 0"),
            "leader row 0 must still render even though it has no marks"
        );
        assert!(places.iter().any(|(_, t)| t == "Task 1"));
    }
}
