// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! XLSX writer tests (tasks F6.2, F6.3).

#![allow(clippy::unwrap_used, clippy::too_many_lines)]

mod common;
use common::{part, well_formed};
use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::{FolioError, Ink, Limits};
use moonshot_folio_ir::{Cell, CellValue, DefinedName, Document, Formula, Meta, Row, Sheet};
use moonshot_folio_xlsx::{render_xlsx, Align, CellStyle, Rule, StyleMap};

fn cell(v: CellValue, role: &str) -> Cell {
    Cell {
        value: v,
        span: 1,
        role: role.into(),
    }
}
fn text(s: &str, role: &str) -> Cell {
    cell(CellValue::Text(s.into()), role)
}
fn num(s: &str, role: &str) -> Cell {
    cell(CellValue::Number(s.into()), role)
}
fn row(cells: Vec<Cell>) -> Row {
    Row {
        cells,
        can_split: true,
    }
}
fn book(sheets: Vec<Sheet>) -> Document {
    Document {
        meta: Meta {
            title: "T & U".into(),
            author: "A".into(),
            date: "2026-11-22".into(),
            lang: "en".into(),
        },
        body: moonshot_folio_ir::Body::Workbook(sheets),
    }
}
fn render(d: &Document, m: &StyleMap) -> Vec<u8> {
    render_xlsx(d, m, &Limits::default()).unwrap()
}
fn styles() -> StyleMap {
    let mut m = StyleMap::new();
    m.insert(
        "head",
        CellStyle {
            bold: true,
            rule: Rule::Bottom,
            fill: Some(Ink::Rgb(0xDD, 0xEE, 0xFF)),
            ..CellStyle::default()
        },
    );
    m.insert(
        "money",
        CellStyle {
            num_fmt: Some("#,##0.00;(#,##0.00)".into()),
            align: Align::Right,
            ..CellStyle::default()
        },
    );
    m
}
fn sample() -> Document {
    let mut s = Sheet {
        name: "Data".into(),
        columns: 3,
        ..Sheet::default()
    };
    s.rows = vec![
        row(vec![
            text("Item", "head"),
            text("2026", "head"),
            text("Note", "head"),
        ]),
        row(vec![
            text("Cash & <co>", ""),
            num("1234.50", "money"),
            cell(CellValue::Bool(true), ""),
        ]),
        row(vec![
            text("Total", ""),
            num("1234.50", "money"),
            cell(CellValue::Empty, ""),
        ]),
        row(vec![Cell {
            value: text("Merged", "").value,
            span: 3,
            role: String::new(),
        }]),
    ];
    s.formulas = vec![Formula {
        row: 2,
        col: 1,
        text: "SUM(B2:B2)".into(),
    }];
    s.freeze_rows = 1;
    s.freeze_cols = 1;
    s.col_widths = vec![2500, 1400];
    s.landscape_fit_width = true;
    s.names = vec![DefinedName {
        name: "Totals".into(),
        row: 2,
        col: 1,
        rows: 1,
        cols: 1,
    }];
    book(vec![
        s,
        Sheet {
            name: "It's 2".into(),
            columns: 1,
            rows: vec![row(vec![num("1", "")])],
            ..Sheet::default()
        },
    ])
}

// REQ: FOLIO-REQ-035
#[test]
fn package_parts_and_well_formedness() {
    let b = render(&sample(), &styles());
    let z = ZipArchive::parse(&b, &Limits::default()).unwrap();
    let names: Vec<&str> = z
        .entries()
        .iter()
        .map(moonshot_folio_codec::ZipEntry::name)
        .collect();
    assert_eq!(
        names,
        [
            "[Content_Types].xml",
            "_rels/.rels",
            "xl/workbook.xml",
            "xl/_rels/workbook.xml.rels",
            "xl/styles.xml",
            "xl/worksheets/sheet1.xml",
            "xl/worksheets/sheet2.xml",
            "docProps/core.xml"
        ]
    );
    for n in &names {
        assert!(well_formed(&part(&b, n)), "{n}");
    }
    assert!(part(&b, "docProps/core.xml").contains("T &amp; U"));
    assert_eq!(b, render(&sample(), &styles()));
    // Manual oracle hook: FOLIO_XLSX_OUT=/path/x.xlsx writes the file for an external reader.
    if let Some(p) = std::env::var_os("FOLIO_XLSX_OUT") {
        std::fs::write(p, &b).unwrap();
    }
}

// REQ: FOLIO-REQ-035
#[test]
fn cells_values_merges_and_escaping() {
    let b = render(&sample(), &styles());
    let s = part(&b, "xl/worksheets/sheet1.xml");
    assert!(s.contains("<c r=\"A2\" t=\"inlineStr\"><is><t xml:space=\"preserve\">Cash &amp; &lt;co&gt;</t></is></c>"));
    assert!(s.contains("<c r=\"B2\" s=\"2\"><v>1234.50</v></c>"));
    assert!(s.contains("<c r=\"C2\" t=\"b\"><v>1</v></c>"));
    assert!(s.contains("<c r=\"C3\"/>"));
    assert!(s.contains("<mergeCells count=\"1\"><mergeCell ref=\"A4:C4\"/></mergeCells>"));
}

// REQ: FOLIO-REQ-035
#[test]
fn widths_freeze_print_and_names() {
    let b = render(&sample(), &styles());
    let s = part(&b, "xl/worksheets/sheet1.xml");
    assert!(s.contains("<col min=\"1\" max=\"1\" width=\"25\" customWidth=\"1\"/>"));
    assert!(s.contains("<col min=\"2\" max=\"2\" width=\"14\" customWidth=\"1\"/>"));
    assert!(s.contains("<pane xSplit=\"1\" ySplit=\"1\" topLeftCell=\"B2\" activePane=\"bottomRight\" state=\"frozen\"/>"));
    assert!(
        s.contains("<pageSetUpPr fitToPage=\"1\"/>")
            && s.contains("orientation=\"landscape\" fitToWidth=\"1\" fitToHeight=\"0\"")
    );
    assert!(s.contains("tabSelected=\"1\""));
    assert!(!part(&b, "xl/worksheets/sheet2.xml").contains("tabSelected"));
    let w = part(&b, "xl/workbook.xml");
    assert!(
        w.contains("<definedName name=\"Totals\">&apos;Data&apos;!$B$3:$B$3</definedName>")
            || w.contains("<definedName name=\"Totals\">'Data'!$B$3:$B$3</definedName>")
    );
    assert!(w.contains("name=\"It's 2\""));
}

// REQ: FOLIO-REQ-035
#[test]
fn styles_are_deduplicated_and_complete() {
    let b = render(&sample(), &styles());
    let x = part(&b, "xl/styles.xml");
    assert!(x.contains("<numFmt numFmtId=\"164\" formatCode=\"#,##0.00;(#,##0.00)\"/>"));
    assert!(x.contains("<cellXfs count=\"3\">"));
    assert!(x.contains("<b/>") && x.contains("bgColor") && x.contains("horizontal=\"right\""));
    assert!(x.contains("<bottom style=\"thin\">"));
    assert!(x.contains("<fills count=\"3\">"));
}

// REQ: FOLIO-REQ-036
#[test]
fn formula_cells_carry_cached_values() {
    let b = render(&sample(), &styles());
    let s = part(&b, "xl/worksheets/sheet1.xml");
    assert!(s.contains("<c r=\"B3\" s=\"2\"><f>SUM(B2:B2)</f><v>1234.50</v></c>"));
    let mut d = sample();
    if let moonshot_folio_ir::Body::Workbook(sh) = &mut d.body {
        sh[0].formulas = vec![Formula {
            row: 1,
            col: 0,
            text: "IF(B2>0,\"a\",\"b\")".into(),
        }];
    }
    let s = part(&render(&d, &styles()), "xl/worksheets/sheet1.xml");
    assert!(s.contains("<c r=\"A2\" t=\"str\"><f>IF(B2&gt;0,&quot;a&quot;,&quot;b&quot;)</f><v>Cash &amp; &lt;co&gt;</v></c>"));
}

// REQ: FOLIO-REQ-036
#[test]
fn bad_formulas_and_missing_cache_are_rejected() {
    let mut d = sample();
    if let moonshot_folio_ir::Body::Workbook(sh) = &mut d.body {
        sh[0].formulas = vec![Formula {
            row: 2,
            col: 2,
            text: "1".into(),
        }]; // C3 is Empty
    }
    assert!(matches!(
        render_xlsx(&d, &styles(), &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
    if let moonshot_folio_ir::Body::Workbook(sh) = &mut d.body {
        sh[0].formulas = vec![Formula {
            row: 2,
            col: 1,
            text: "HYPERLINK(\"x\")".into(),
        }];
    }
    assert!(render_xlsx(&d, &styles(), &Limits::default()).is_err());
}

// REQ: FOLIO-REQ-035
#[test]
fn style_rejections_and_flow_body() {
    let mut m = StyleMap::new();
    m.insert(
        "bad",
        CellStyle {
            fill: Some(Ink::Cmyk(0, 0, 0, 0)),
            ..CellStyle::default()
        },
    );
    let d = book(vec![Sheet {
        name: "S".into(),
        columns: 1,
        rows: vec![row(vec![text("x", "bad")])],
        ..Sheet::default()
    }]);
    assert!(matches!(
        render_xlsx(&d, &m, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let mut m = StyleMap::new();
    m.insert(
        "bad",
        CellStyle {
            num_fmt: Some("a\u{1}b".into()),
            ..CellStyle::default()
        },
    );
    assert!(matches!(
        render_xlsx(&d, &m, &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
    let mut m = StyleMap::new();
    m.insert(
        "bad",
        CellStyle {
            num_fmt: Some(String::new()),
            ..CellStyle::default()
        },
    );
    assert!(render_xlsx(&d, &m, &Limits::default()).is_err());
    let wide = book(vec![Sheet {
        name: "S".into(),
        columns: 16_385,
        rows: vec![],
        ..Sheet::default()
    }]);
    assert!(matches!(
        render_xlsx(&wide, &StyleMap::new(), &Limits::default()),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-035
#[test]
fn column_letters_at_boundaries() {
    // Exercised through a 28-column row: cells 26 and 27 are AA and AB.
    let cells: Vec<Cell> = (0..28).map(|i| num(&i.to_string(), "")).collect();
    let d = book(vec![Sheet {
        name: "S".into(),
        columns: 28,
        rows: vec![row(cells)],
        ..Sheet::default()
    }]);
    let s = part(&render(&d, &StyleMap::new()), "xl/worksheets/sheet1.xml");
    for r in ["r=\"Z1\"", "r=\"AA1\"", "r=\"AB1\""] {
        assert!(s.contains(r), "{r}");
    }
}

// REQ: FOLIO-REQ-037
#[test]
fn validations_and_conditional_formats() {
    use moonshot_folio_ir::{CmpOp, CondFormat, Range, Validation, ValidationKind};
    let mut d = sample();
    if let moonshot_folio_ir::Body::Workbook(sh) = &mut d.body {
        sh[0].validations = vec![
            Validation {
                range: Range {
                    row: 1,
                    col: 0,
                    rows: 2,
                    cols: 1,
                },
                kind: ValidationKind::List(vec!["Yes".into(), "No & maybe".into()]),
            },
            Validation {
                range: Range {
                    row: 1,
                    col: 1,
                    rows: 1,
                    cols: 1,
                },
                kind: ValidationKind::WholeNumber { min: 0, max: 100 },
            },
        ];
        sh[0].cond_formats = vec![CondFormat {
            range: Range {
                row: 1,
                col: 1,
                rows: 2,
                cols: 1,
            },
            op: CmpOp::Gt,
            value: "1000".into(),
            role: "head".into(),
        }];
    }
    let b = render(&d, &styles());
    let s = part(&b, "xl/worksheets/sheet1.xml");
    assert!(well_formed(&s), "{s}");
    assert!(s.contains("<conditionalFormatting sqref=\"B2:B3\"><cfRule type=\"cellIs\" dxfId=\"0\" priority=\"1\" operator=\"greaterThan\"><formula>1000</formula></cfRule></conditionalFormatting>"));
    assert!(s.contains("<dataValidations count=\"2\">"));
    assert!(s.contains("sqref=\"A2:A3\"><formula1>&quot;Yes,No &amp; maybe&quot;</formula1>"));
    assert!(s.contains("type=\"whole\" operator=\"between\" allowBlank=\"1\" showErrorMessage=\"1\" sqref=\"B2\"><formula1>0</formula1><formula2>100</formula2>"));
    // Schema order: merges, conditional formatting, validations, margins.
    let pos = |t: &str| s.find(t).unwrap();
    assert!(
        pos("<mergeCells") < pos("<conditionalFormatting")
            && pos("<conditionalFormatting") < pos("<dataValidations")
            && pos("<dataValidations") < pos("<pageMargins")
    );
    let x = part(&b, "xl/styles.xml");
    assert!(x.contains(
        "<dxfs count=\"1\"><dxf><font><b/></font><fill><patternFill patternType=\"solid\">"
    ));
    assert!(well_formed(&x));
    if let Some(p) = std::env::var_os("FOLIO_XLSX_OUT2") {
        std::fs::write(p, &b).unwrap();
    }
}

// REQ: FOLIO-REQ-037
#[test]
fn bad_validations_and_formats_are_rejected() {
    use moonshot_folio_ir::{CmpOp, CondFormat, Range, Validation, ValidationKind};
    let one = Range {
        row: 0,
        col: 0,
        rows: 1,
        cols: 1,
    };
    let with = |f: &dyn Fn(&mut Sheet)| {
        let mut d = sample();
        if let moonshot_folio_ir::Body::Workbook(sh) = &mut d.body {
            f(&mut sh[0]);
        }
        render_xlsx(&d, &styles(), &Limits::default())
    };
    for bad in [
        vec![],
        vec!["a,b".to_string()],
        vec!["q\"".to_string()],
        vec![String::new()],
        vec!["x".repeat(256)],
    ] {
        assert!(with(&|s| s.validations = vec![Validation {
            range: one,
            kind: ValidationKind::List(bad.clone())
        }])
        .is_err());
    }
    assert!(with(&|s| s.validations = vec![Validation {
        range: one,
        kind: ValidationKind::WholeNumber { min: 5, max: 1 }
    }])
    .is_err());
    assert!(with(&|s| s.validations = vec![Validation {
        range: Range { rows: 99, ..one },
        kind: ValidationKind::WholeNumber { min: 0, max: 1 }
    }])
    .is_err());
    let cf = |value: &str, role: &str| CondFormat {
        range: one,
        op: CmpOp::Eq,
        value: value.into(),
        role: role.into(),
    };
    assert!(with(&|s| s.cond_formats = vec![cf("1e3", "head")]).is_err());
    assert!(with(&|s| s.cond_formats = vec![cf("1", "")]).is_err());
    assert!(matches!(
        with(&|s| s.cond_formats = vec![cf("1", "no-such-role")]),
        Err(FolioError::Invalid(_))
    ));
    assert!(with(&|s| s.cond_formats = vec![cf("1", "head")]).is_ok());
}

// REQ: FOLIO-REQ-053
#[test]
fn number_format_syntax_is_checked() {
    let with = |code: &str| {
        let mut m = StyleMap::new();
        m.insert(
            "f",
            CellStyle {
                num_fmt: Some(code.into()),
                ..CellStyle::default()
            },
        );
        let d = book(vec![Sheet {
            name: "S".into(),
            columns: 1,
            rows: vec![row(vec![num("1", "f")])],
            ..Sheet::default()
        }]);
        render_xlsx(&d, &m, &Limits::default())
    };
    for good in [
        "#,##0.00;(#,##0.00)",
        "0.00\"EUR\"",
        "[Red]0.0;[Blue]-0.0;0;@",
        "_(* #,##0_)",
        "0\\%",
        "yyyy-mm-dd",
    ] {
        assert!(with(good).is_ok(), "{good}");
    }
    for bad in ["0.00\"x", "[Red", "Red]0", "[[Red]]0", "0;0;0;0;0", "0\\"] {
        assert!(matches!(with(bad), Err(FolioError::Invalid(_))), "{bad}");
    }
}

// REQ: FOLIO-REQ-053
#[test]
fn many_formulas_render_in_linear_time() {
    let n: u32 = 20_000;
    let mut s = Sheet {
        name: "S".into(),
        columns: 1,
        ..Sheet::default()
    };
    s.rows = (0..n).map(|_| row(vec![num("1", "")])).collect();
    s.formulas = (0..n)
        .map(|r| Formula {
            row: r,
            col: 0,
            text: "1+1".into(),
        })
        .collect();
    let t = std::time::Instant::now();
    let b = render(&book(vec![s]), &StyleMap::new());
    assert!(t.elapsed().as_secs() < 5, "{:?}", t.elapsed());
    assert!(part(&b, "xl/worksheets/sheet1.xml").contains("<c r=\"A20000\"><f>1+1</f><v>1</v></c>"));
}
