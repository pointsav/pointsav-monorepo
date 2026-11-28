// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! IR validation tests (task F2.1).

#![allow(clippy::unwrap_used)]

use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_ir::{
    validate, Block, Body, Cell, CellValue, Document, Heading, Image, ImageFormat, Meta, PageSetup,
    Pagination, Row, Section, Sheet, Table,
};

fn meta() -> Meta {
    Meta {
        title: "T".into(),
        author: "A".into(),
        date: "2026-11-21".into(),
        lang: "en-CA".into(),
    }
}
fn page() -> PageSetup {
    PageSetup {
        width: Emu::from_inches(8).unwrap(),
        height: Emu::from_inches(11).unwrap(),
        margin: Emu::from_inches(1).unwrap(),
    }
}
fn cell(v: CellValue, span: u32) -> Cell {
    Cell {
        value: v,
        span,
        role: String::new(),
    }
}
fn row(cells: Vec<Cell>) -> Row {
    Row {
        cells,
        can_split: true,
    }
}
fn flow(blocks: Vec<Block>) -> Document {
    Document {
        meta: meta(),
        body: Body::Flow(vec![Section::new(page(), blocks)]),
    }
}
fn book(sheets: Vec<Sheet>) -> Document {
    Document {
        meta: meta(),
        body: Body::Workbook(sheets),
    }
}
fn sheet(name: &str) -> Sheet {
    Sheet {
        name: name.into(),
        columns: 2,
        freeze_rows: 0,
        rows: vec![row(vec![
            cell(CellValue::Text("a".into()), 1),
            cell(CellValue::Number("1.50".into()), 1),
        ])],
        ..Sheet::default()
    }
}
fn invalid(d: &Document) -> bool {
    matches!(validate(d, &Limits::default()), Err(FolioError::Invalid(_)))
}

// REQ: FOLIO-REQ-022
#[test]
fn valid_documents_pass() {
    let d = flow(vec![Block::Heading(Heading {
        level: 2,
        inlines: vec![],
        pagination: Pagination::default(),
    })]);
    validate(&d, &Limits::default()).unwrap();
    validate(
        &book(vec![sheet("Data"), sheet("Notes")]),
        &Limits::default(),
    )
    .unwrap();
}

// REQ: FOLIO-REQ-022
#[test]
fn meta_rules() {
    let mut d = flow(vec![]);
    d.meta.lang = String::new();
    assert!(invalid(&d));
    d.meta.lang = "en CA".into();
    assert!(invalid(&d));
    d.meta.lang = "en".into();
    d.meta.date = "2026-1-21".into();
    assert!(invalid(&d));
    d.meta.date = "2026/11/21".into();
    assert!(invalid(&d));
    assert!(invalid(&Document {
        meta: meta(),
        body: Body::Flow(vec![])
    }));
    assert!(invalid(&book(vec![])));
}

// REQ: FOLIO-REQ-022
#[test]
fn page_geometry_rules() {
    for (w, h, m) in [
        (0, 100, 0),
        (100, 0, 0),
        (100, 100, -1),
        (100, 100, 50),
        (100, 100, 60),
    ] {
        let mut d = flow(vec![]);
        if let Body::Flow(s) = &mut d.body {
            s[0].page = PageSetup {
                width: Emu::from_raw(w),
                height: Emu::from_raw(h),
                margin: Emu::from_raw(m),
            };
        }
        assert!(invalid(&d), "{w} {h} {m}");
    }
    // Margin whose double overflows i64 is an error, not a panic.
    let mut d = flow(vec![]);
    if let Body::Flow(s) = &mut d.body {
        s[0].page.margin = Emu::from_raw(i64::MAX);
    }
    assert!(validate(&d, &Limits::default()).is_err());
}

// REQ: FOLIO-REQ-022
#[test]
fn heading_and_image_rules() {
    for level in [0, 7] {
        assert!(invalid(&flow(vec![Block::Heading(Heading {
            level,
            inlines: vec![],
            pagination: Pagination::default()
        })])));
    }
    let img = |alt: &str, data: Vec<u8>, w: i64| Image {
        alt_text: alt.into(),
        format: ImageFormat::Png,
        data,
        width: Emu::from_raw(w),
        height: Emu::from_raw(1),
    };
    validate(
        &flow(vec![Block::Image(img("logo", vec![1], 1))]),
        &Limits::default(),
    )
    .unwrap();
    assert!(invalid(&flow(vec![Block::Image(img("  ", vec![1], 1))])));
    assert!(invalid(&flow(vec![Block::Image(img("x", vec![], 1))])));
    assert!(invalid(&flow(vec![Block::Image(img("x", vec![1], 0))])));
    let tiny = Limits::default().with_max_entry_bytes(2);
    let d = flow(vec![Block::Image(img("x", vec![1, 2, 3], 1))]);
    assert!(matches!(validate(&d, &tiny), Err(FolioError::Limit(_))));
}

// REQ: FOLIO-REQ-022
#[test]
fn table_grid_rules() {
    let t = |columns, header, rows| {
        flow(vec![Block::Table(Table {
            columns,
            repeat_header_rows: header,
            rows,
        })])
    };
    let ok = vec![
        row(vec![cell(CellValue::Empty, 2)]),
        row(vec![
            cell(CellValue::Bool(true), 1),
            cell(CellValue::Empty, 1),
        ]),
    ];
    validate(&t(2, 1, ok.clone()), &Limits::default()).unwrap();
    assert!(invalid(&t(0, 0, vec![])));
    assert!(invalid(&t(2, 3, ok.clone())));
    assert!(invalid(&t(3, 0, ok)));
    assert!(invalid(&t(
        1,
        0,
        vec![row(vec![
            cell(CellValue::Empty, 0),
            cell(CellValue::Empty, 1)
        ])]
    )));
    // Span sum must not wrap: u32::MAX + 1 would equal 0 or 1 under wrapping arithmetic.
    assert!(invalid(&t(
        1,
        0,
        vec![row(vec![
            cell(CellValue::Empty, u32::MAX),
            cell(CellValue::Empty, 2)
        ])]
    )));
}

// REQ: FOLIO-REQ-022
#[test]
fn number_syntax() {
    let num = |s: &str| {
        book(vec![Sheet {
            name: "S".into(),
            columns: 1,
            freeze_rows: 0,
            rows: vec![row(vec![cell(CellValue::Number(s.into()), 1)])],
            ..Sheet::default()
        }])
    };
    for good in ["0", "-1", "12.50", "-0.001", "007"] {
        validate(&num(good), &Limits::default()).unwrap();
    }
    for bad in [
        "", "-", ".5", "5.", "1e3", "+1", "1,000", "1.2.3", "--1", "NaN", " 1", "١",
    ] {
        assert!(invalid(&num(bad)), "{bad:?}");
    }
}

// REQ: FOLIO-REQ-022
#[test]
fn sheet_rules() {
    for bad in [
        "",
        "a/b",
        "a[b",
        "a:b",
        "a*b",
        "a?b",
        "a\\b",
        "a\nb",
        &"x".repeat(32),
    ] {
        assert!(invalid(&book(vec![sheet(bad)])), "{bad:?}");
    }
    validate(&book(vec![sheet(&"x".repeat(31))]), &Limits::default()).unwrap();
    assert!(invalid(&book(vec![sheet("Data"), sheet("DATA")])));
    let mut s = sheet("S");
    s.freeze_rows = 2;
    assert!(invalid(&book(vec![s])));
}

// REQ: FOLIO-REQ-022
#[test]
fn cell_limit_counts_across_sheets() {
    let d = book(vec![sheet("A"), sheet("B")]);
    let lim = Limits::default().with_max_cells(3);
    assert!(matches!(validate(&d, &lim), Err(FolioError::Limit(_))));
    validate(&d, &Limits::default().with_max_cells(4)).unwrap();
}

// REQ: FOLIO-REQ-034
#[test]
fn sheet_extras_widths_freeze_names() {
    use moonshot_folio_ir::DefinedName;
    let ok = |f: &dyn Fn(&mut Sheet)| {
        let mut s = sheet("S");
        f(&mut s);
        validate(&book(vec![s]), &Limits::default()).is_ok()
    };
    assert!(ok(&|s| s.col_widths = vec![1200, 0]));
    assert!(!ok(&|s| s.col_widths = vec![1, 2, 3]));
    assert!(!ok(&|s| s.col_widths = vec![30_000]));
    assert!(ok(&|s| s.freeze_cols = 2));
    assert!(!ok(&|s| s.freeze_cols = 3));
    let name = |n: &str, row, col, rows, cols| DefinedName {
        name: n.into(),
        row,
        col,
        rows,
        cols,
    };
    assert!(ok(&|s| s.names = vec![name("Total_2026", 0, 0, 1, 2)]));
    for bad in ["", "1abc", "A1", "ab12", "R", "c", "has space", "x-y"] {
        assert!(!ok(&|s| s.names = vec![name(bad, 0, 0, 1, 1)]), "{bad:?}");
    }
    assert!(!ok(&|s| s.names = vec![name("Out", 0, 0, 2, 1)]));
    assert!(!ok(&|s| s.names = vec![name("Out", 0, 1, 1, 2)]));
    assert!(!ok(&|s| s.names = vec![name("Zero", 0, 0, 0, 1)]));
    assert!(!ok(
        &|s| s.names = vec![name("Dup", 0, 0, 1, 1), name("dup", 0, 1, 1, 1)]
    ));
    let a = {
        let mut s = sheet("A");
        s.names = vec![name("Shared", 0, 0, 1, 1)];
        s
    };
    let b = {
        let mut s = sheet("B");
        s.names = vec![name("SHARED", 0, 0, 1, 1)];
        s
    };
    assert!(invalid(&book(vec![a, b])));
}

// REQ: FOLIO-REQ-034
#[test]
fn formulas_need_cached_value_and_whitelist() {
    use moonshot_folio_ir::{check_formula_text, Formula};
    let f = |row, col, t: &str| Formula {
        row,
        col,
        text: t.into(),
    };
    let with = |fs: Vec<Formula>| {
        let mut s = sheet("S");
        s.formulas = fs;
        validate(&book(vec![s]), &Limits::default())
    };
    assert!(with(vec![f(0, 1, "SUM(A1:A3)*2")]).is_ok());
    assert!(with(vec![f(0, 1, "IF(A1>0,\"yes (\",\"no\")")]).is_ok());
    assert!(with(vec![f(1, 0, "1")]).is_err()); // no such row
    assert!(with(vec![f(0, 5, "1")]).is_err()); // no such cell
    assert!(with(vec![f(0, 0, "1"), f(0, 0, "2")]).is_err()); // duplicate
                                                              // Empty cached value is refused.
    let mut s = sheet("S");
    s.rows[0].cells[0].value = CellValue::Empty;
    s.formulas = vec![f(0, 0, "1")];
    assert!(invalid(&book(vec![s])));
    for bad in [
        "",
        "=1",
        "HYPERLINK(\"x\")",
        "cmd|' /c calc'!A1",
        "[1]Sheet!A1",
        "SUM(A1",
        "A1\n+1",
        "1+@A",
        "WEBSERVICE(A1)",
        "\"open",
    ] {
        assert!(check_formula_text(bad).is_err(), "{bad:?}");
    }
    for good in [
        "A1+B2",
        "ROUND(A1/3,2)",
        "Sheet2!A1",
        "AND(A1>1,B1<=2)",
        "A1&\" x\"",
        "-A1^2%",
    ] {
        assert!(check_formula_text(good).is_ok(), "{good:?}");
    }
}

// REQ: FOLIO-REQ-053
#[test]
fn review_names_sheets_formulas_and_numbers() {
    use moonshot_folio_ir::{check_formula_text, DefinedName};
    let name_ok = |n: &str| {
        let mut s = sheet("S");
        s.names = vec![DefinedName {
            name: n.into(),
            row: 0,
            col: 0,
            rows: 1,
            cols: 1,
        }];
        validate(&book(vec![s]), &Limits::default()).is_ok()
    };
    for bad in [
        "_xlnm.Print_Area",
        "_XLNM.x",
        "R1C1",
        "RC",
        "R2C3",
        "rc5",
        "R12",
        "C3",
        "r",
        "c",
    ] {
        assert!(!name_ok(bad), "{bad}");
    }
    for good in ["Total", "Rate", "Cost", "Roc", "My.Name", "_private"] {
        assert!(name_ok(good), "{good}");
    }
    let sheet_ok = |n: &str| validate(&book(vec![sheet(n)]), &Limits::default()).is_ok();
    for bad in ["'a", "a'", "History", "HISTORY", &"\u{1F600}".repeat(16)] {
        assert!(!sheet_ok(bad), "{bad}");
    }
    assert!(sheet_ok("it's ok") && sheet_ok(&"\u{1F600}".repeat(15)));
    assert!(invalid(&book(vec![
        sheet("\u{C9}t\u{E9}"),
        sheet("\u{E9}t\u{E9}")
    ])));
    for bad in [
        "HYPERLINK (\"x\",\"y\")",
        "SUM(A1;A2)",
        "1+",
        "*2",
        "SUM(A1,)+",
        "SUM (",
        &"SUM(".repeat(65),
    ] {
        assert!(check_formula_text(bad).is_err(), "{bad}");
    }
    assert!(check_formula_text(&format!("{}1{}", "SUM(".repeat(64), ")".repeat(64))).is_ok());
    assert!(check_formula_text("SUM (A1)").is_ok());
    let num = |s: &str| {
        let mut sh = sheet("S");
        sh.rows[0].cells[1].value = CellValue::Number(s.into());
        validate(&book(vec![sh]), &Limits::default()).is_ok()
    };
    assert!(num(&"9".repeat(40)) && !num(&"9".repeat(41)));
}

// REQ: FOLIO-REQ-053
#[test]
fn many_formulas_validate_in_linear_time() {
    use moonshot_folio_ir::Formula;
    let n: u32 = 20_000;
    let mut s = Sheet {
        name: "S".into(),
        columns: 1,
        ..Sheet::default()
    };
    s.rows = (0..n)
        .map(|_| row(vec![cell(CellValue::Number("1".into()), 1)]))
        .collect();
    s.formulas = (0..n)
        .map(|r| Formula {
            row: r,
            col: 0,
            text: "1+1".into(),
        })
        .collect();
    let t = std::time::Instant::now();
    validate(&book(vec![s]), &Limits::default()).unwrap();
    assert!(t.elapsed().as_secs() < 3, "{:?}", t.elapsed());
    let mut small = Sheet {
        name: "S".into(),
        columns: 1,
        rows: vec![row(vec![cell(CellValue::Number("1".into()), 1)])],
        ..Sheet::default()
    };
    small.formulas = vec![
        Formula {
            row: 0,
            col: 0,
            text: "1".into()
        };
        3
    ];
    assert!(matches!(
        validate(&book(vec![small]), &Limits::default().with_max_cells(2)),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-022
#[test]
fn dates_are_calendar_correct_and_page_sides_are_capped_at_200_inches() {
    use moonshot_folio_ir::valid_date;
    for ok in [
        "2026-01-31",
        "2024-02-29",
        "2000-02-29",
        "2026-12-31",
        "2026-04-30",
    ] {
        assert!(valid_date(ok), "{ok}");
        let mut d = flow(vec![]);
        d.meta.date = ok.into();
        assert!(!invalid(&d), "{ok}");
    }
    for bad in [
        "2026-13-45",
        "0000-00-00",
        "2026-02-31",
        "2026-02-29",
        "1900-02-29",
        "2026-04-31",
        "2026-00-10",
        "2026-01-00",
        "2026-1-21",
        "2026-01-3a",
        "",
        "20260131",
        "2026-01-311",
    ] {
        assert!(!valid_date(bad), "{bad}");
        let mut d = flow(vec![]);
        d.meta.date = bad.into();
        assert!(invalid(&d), "{bad}");
    }
    let page = |w: i64, h: i64| {
        let mut d = flow(vec![]);
        if let Body::Flow(s) = &mut d.body {
            s[0].page = PageSetup {
                width: Emu::from_raw(w),
                height: Emu::from_raw(h),
                margin: Emu::ZERO,
            };
        }
        d
    };
    let max = moonshot_folio_core::EMU_PER_INCH * 200;
    assert!(!invalid(&page(max, max)));
    assert!(invalid(&page(max + 1, 100)));
    assert!(invalid(&page(100, max + 1)));
}
