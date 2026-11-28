// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Golden corpus group `tt/` (task F2.4): synthetic documents modelled on the scenarios the
//! tool-typeset test suite exercises, converted through the adapter and compared with the stored
//! IR dumps in `assurance/golden/tt/`. Regenerate with `FOLIO_BLESS=1 cargo test --test tt_corpus`
//! and review the diff. All content is synthetic.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::too_many_lines
)]

use moonshot_folio_compat_typeset::convert;
use moonshot_folio_core::Limits;
use moonshot_folio_ir::{dump, validate, Meta};
use std::path::PathBuf;
use tool_typeset::{
    Align, AxisBand, BarStyle, Block, Cell, ColWidth, Column, Doc, LabelPlacement, Mark, MarkLabel,
    MilestoneShape, PageSize, ParaStyle, Register, Row, Span, Table, TimeAxis, Timeline,
    TimelineRow,
};

type Scenario = (&'static str, &'static str, Doc);

fn meta() -> Meta {
    Meta {
        title: "Synthetic".into(),
        author: "Synthetic".into(),
        date: "2026-01-01".into(),
        lang: "en".into(),
    }
}
fn col(w: f32, a: Align) -> Column {
    Column {
        width: ColWidth::Pct(w),
        align: a,
    }
}
fn doc(blocks: Vec<Block>) -> Doc {
    Doc {
        page: tool_typeset::PageSetup::default(),
        blocks,
    }
}
fn text(s: &str) -> Span {
    Span::Text(s.into())
}
fn para(spans: Vec<Span>, style: ParaStyle) -> Block {
    Block::Para { spans, style }
}
fn statement(register: Register, rules_from: Option<usize>) -> Doc {
    let mut t = Table::new(vec![
        col(60.0, Align::Left),
        col(20.0, Align::Right),
        col(20.0, Align::Right),
    ]);
    t.header = Some(vec![
        Cell::text("Item"),
        Cell::text("2026"),
        Cell::text("2025"),
    ]);
    t.subheader = Some(vec![Cell::blank(), Cell::text("$"), Cell::text("$")]);
    t.rules_from_col = rules_from;
    t.rows.push(Row::section_banner("Assets", 3));
    t.rows.push(Row::data(vec![
        Cell::text("Cash").indent(1),
        Cell::figure("100.00"),
        Cell::figure("90.00"),
    ]));
    t.rows.push(Row::data(vec![
        Cell::text("Receivables").indent(2),
        Cell::figure("(36.00)"),
        Cell::figure("-"),
    ]));
    t.rows.push(Row::subtotal(vec![
        Cell::text("Subtotal").bold(),
        Cell::figure("64.00"),
        Cell::figure("90.00"),
    ]));
    t.rows.push(Row::total(vec![
        Cell::text("Total").bold(),
        Cell::figure("64.00"),
        Cell::figure("90.00"),
    ]));
    let mut d = doc(vec![
        Block::Masthead {
            entity_name: "Example Entity".into(),
            statement_title: "Statement".into(),
            period_label: "For the year".into(),
        },
        Block::Table(t),
    ]);
    d.page.register = register;
    d
}
fn timeline(empty_row: bool, milestone: bool) -> Doc {
    let mut leader = Table::new(vec![col(100.0, Align::Left)]);
    let mut rows = Vec::new();
    leader.rows.push(Row::data(vec![Cell::text("Task one")]));
    rows.push(TimelineRow::one(Mark::Bar {
        start: 1.0,
        end: 4.0,
        style: BarStyle::Task,
        progress: Some(0.5),
        label: Some(MarkLabel {
            text: "A".into(),
            placement: LabelPlacement::After,
            bold: false,
        }),
    }));
    if empty_row {
        leader.rows.push(Row::data(vec![Cell::text("Empty")]));
        rows.push(TimelineRow::empty());
    }
    if milestone {
        leader.rows.push(Row::data(vec![Cell::text("Gate")]));
        rows.push(TimelineRow::one(Mark::Milestone {
            at: 2.0,
            shape: MilestoneShape::Diamond,
            label: None,
        }));
    }
    let axis = TimeAxis {
        start: 0.0,
        end: 6.0,
        bands: vec![AxisBand {
            label: "Q1".into(),
            start: 0.0,
            end: 6.0,
            level: 0,
        }],
        gridlines: vec![],
        marker: None,
    };
    doc(vec![Block::Timeline(
        Timeline::with_leader(leader, ColWidth::Pct(30.0), axis, rows).unwrap(),
    )])
}

fn scenarios() -> Vec<Scenario> {
    let mut v: Vec<Scenario> = Vec::new();
    v.push(("01-working-register-statement", "the_working_register_keeps_its_ragged_edge; working_register_keeps_its_banner_fill_and_accent", statement(Register::WorkingDocument, None)));
    v.push(("02-formal-register-statement", "formal_register_paints_no_fill_and_no_accent_color; formal_register_rules_only_ordinary_rows_at_totals", statement(Register::FormalStatement, None)));
    v.push((
        "03-rules-from-column",
        "rules_start_at_the_named_column_not_the_margin",
        statement(Register::FormalStatement, Some(1)),
    ));
    let mut land = doc(vec![Block::Heading {
        level: 1,
        text: "Landscape".into(),
    }]);
    land.page.landscape = true;
    v.push((
        "04-landscape",
        "landscape_swaps_page_dimensions; landscape_emits_the_print_page_size_hint",
        land,
    ));
    for (n, size) in [
        ("05-tabloid", PageSize::Tabloid),
        ("06-legal", PageSize::Legal),
    ] {
        let mut d = doc(vec![Block::Heading {
            level: 2,
            text: "Wide report".into(),
        }]);
        d.page.size = size;
        d.page.landscape = true;
        v.push((
            n,
            "page_size_tabloid_and_legal_are_landscape_usable_for_a_wide_report",
            d,
        ));
    }
    v.push((
        "07-default-page",
        "every_doc_gets_an_explicit_portrait_print_page_size_and_sane_margins_by_default",
        doc(vec![]),
    ));
    let mut header = doc(vec![Block::Para {
        spans: vec![text("x")],
        style: ParaStyle::default(),
    }]);
    header.page.running_header = Some("Running".into());
    header.page.page_numbers = true;
    header.page.suppress_header_on_first = true;
    v.push((
        "08-running-header",
        "running header, page-number footer and first-page suppression",
        header,
    ));
    v.push(("09-bold-lead-in", "bold_lead_span_stays_bold_while_running_text_keeps_its_face; a_bold_lead_in_keeps_its_stretch_in_step_with_the_rest_of_the_line", doc(vec![para(vec![Span::Bold("Lead. ".into()), text("Running text follows.")], ParaStyle::default())])));
    v.push(("10-tab-in-paragraph", "a_tab_becomes_a_real_tab_stop_width_rather_than_collapsing; a_tab_lands_the_text_after_it_on_the_registers_own_tab_stop", doc(vec![para(vec![text("Label\tvalue")], ParaStyle::default())])));
    let styled = ParaStyle {
        left_indent_pt: 18.0,
        space_before_pt: 6.0,
        space_after_pt: Some(0.0),
    };
    v.push(("11-measured-paragraph", "measured_paragraph_values_ride_as_inline_style; a_left_indent_moves_every_line_including_the_first", doc(vec![para(vec![text("Indented paragraph.")], styled)])));
    v.push(("12-justified-paragraphs", "every_justified_line_but_the_last_ends_exactly_on_the_right_margin; consecutive_paragraphs_are_ten_points_apart; body_leading_is_exactly_twelve_points_not_a_multiple_of_the_font_size", doc(vec![
        para(vec![text("A long paragraph that wraps across several lines when set justified.")], ParaStyle::default()),
        para(vec![text("A second paragraph.")], ParaStyle::default()),
        Block::Note(vec![text("Note text.")]),
    ])));
    let mut ind = Table::new(vec![col(70.0, Align::Left), col(30.0, Align::Right)]);
    for lvl in 0..=4_u8 {
        ind.rows.push(Row::data(vec![
            Cell::text("Level").indent(lvl),
            Cell::figure("1.00"),
        ]));
    }
    v.push(("13-indent-levels", "indent_level_is_a_real_offset_not_leading_spaces; indent_levels_3_and_4_no_longer_collapse_onto_level_2; indent_level_3_gets_its_own_measured_offset_not_a_missing_class", doc(vec![Block::Table(ind)])));
    let mut wrap = Table::new(vec![col(20.0, Align::Right), col(80.0, Align::Right)]);
    wrap.header = Some(vec![
        Cell::text("A very long column heading that must wrap"),
        Cell::text("Supercalifragilisticexpialidocious"),
    ]);
    wrap.rows
        .push(Row::data(vec![Cell::figure("1.00"), Cell::figure("2.00")]));
    v.push(("14-long-headings", "long_column_headings_wrap_instead_of_truncating; single_word_heading_wider_than_column_is_not_truncated; wrapped_heading_lines_bottom_align_onto_the_rule", doc(vec![Block::Table(wrap)])));
    let mut span = Table::new(vec![
        col(40.0, Align::Left),
        col(30.0, Align::Right),
        col(30.0, Align::Right),
    ]);
    span.rows.push(Row::data(vec![
        Cell::text("Spans two").colspan(2),
        Cell::figure("5.00"),
    ]));
    span.rows.push(Row::data(vec![Cell::text("Short")]));
    v.push((
        "15-colspan-and-short-row",
        "span_geometry_sums_widths_across_colspan",
        doc(vec![Block::Table(span)]),
    ));
    v.push((
        "16-en-dash-placeholder",
        "en_dash_zero_placeholder_reaches_the_stream_as_winansi",
        doc(vec![Block::Table({
            let mut t = Table::new(vec![col(50.0, Align::Left), col(50.0, Align::Right)]);
            t.rows
                .push(Row::data(vec![Cell::text("Nil"), Cell::figure("\u{2013}")]));
            t
        })]),
    ));
    v.push(("17-timeline-bar", "a_timeline_renders_its_leader_and_a_positioned_bar; a_bar_mark_reaches_the_stream_as_a_filled_rect", timeline(false, false)));
    v.push(("18-timeline-milestone-and-empty-row", "a_milestone_diamond_reaches_the_stream_as_a_filled_polygon; an_empty_row_still_occupies_its_slot_keeping_leader_and_plot_aligned", timeline(true, true)));
    v.push((
        "19-blocks-misc",
        "page break, rule, spacer, footer, placeholder never silently dropped",
        doc(vec![
            Block::CoverTitle {
                entity_name: "E".into(),
                statement_title: "S".into(),
                period_label: "P".into(),
                currency_label: "C".into(),
            },
            Block::Rule,
            Block::Spacer(12.0),
            Block::PageBreak,
            Block::Placeholder {
                kind: "chart".into(),
                caption: "Figure".into(),
            },
            Block::Footer("Footer".into()),
        ]),
    ));
    v
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assurance/golden/tt")
        .join(format!("{name}.ir.txt"))
}

// REQ: FOLIO-REQ-014
// REQ: FOLIO-REQ-025
#[test]
fn tt_corpus_matches_goldens() {
    let bless = std::env::var_os("FOLIO_BLESS").is_some();
    let mut failures = Vec::new();
    for (name, covers, d) in scenarios() {
        let c = convert(&d, meta()).unwrap();
        validate(&c.document, &Limits::default()).unwrap();
        let mut out = format!("# covers: {covers}\n");
        for n in &c.notes {
            out.push_str("# note: ");
            out.push_str(n);
            out.push('\n');
        }
        out.push_str(&dump(&c.document));
        let p = golden_path(name);
        if bless {
            std::fs::write(&p, &out).unwrap();
        } else if std::fs::read_to_string(&p).ok().as_deref() != Some(out.as_str()) {
            failures.push(name);
        }
    }
    assert!(
        failures.is_empty(),
        "golden mismatch: {failures:?} (FOLIO_BLESS=1 to regenerate)"
    );
}

// REQ: FOLIO-REQ-025
#[test]
fn conversion_is_deterministic() {
    for (name, _, d) in scenarios() {
        let a = dump(&convert(&d, meta()).unwrap().document);
        let b = dump(&convert(&d, meta()).unwrap().document);
        assert_eq!(a, b, "{name}");
    }
}
