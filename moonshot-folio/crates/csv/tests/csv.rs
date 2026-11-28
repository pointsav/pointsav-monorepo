// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! CSV writer tests (task F6.5).

#![allow(clippy::unwrap_used)]

use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_csv::{render_csv, CsvOptions};
use moonshot_folio_ir::{
    Block, Body, Cell, CellValue, Document, Meta, PageSetup, Row, Section, Sheet,
};

fn meta() -> Meta {
    Meta {
        title: "T".into(),
        author: "A".into(),
        date: "2026-11-22".into(),
        lang: "en".into(),
    }
}
fn c(v: CellValue) -> Cell {
    Cell {
        value: v,
        span: 1,
        role: String::new(),
    }
}
fn text(s: &str) -> Cell {
    c(CellValue::Text(s.into()))
}
fn book(rows: Vec<Vec<Cell>>) -> Document {
    let columns = u32::try_from(rows.first().map_or(1, Vec::len)).unwrap();
    Document {
        meta: meta(),
        body: Body::Workbook(vec![Sheet {
            name: "S".into(),
            columns,
            freeze_rows: 0,
            rows: rows
                .into_iter()
                .map(|cells| Row {
                    cells,
                    can_split: true,
                })
                .collect(),
            ..Sheet::default()
        }]),
    }
}
fn csv(d: &Document, o: CsvOptions) -> String {
    String::from_utf8(render_csv(d, 0, o, &Limits::default()).unwrap()).unwrap()
}

// REQ: FOLIO-REQ-033
#[test]
fn rfc4180_quoting_and_crlf() {
    let d = book(vec![
        vec![
            text("plain"),
            text("a,b"),
            text("say \"hi\""),
            text("two\nlines"),
        ],
        vec![
            c(CellValue::Number("-1.50".into())),
            c(CellValue::Bool(true)),
            c(CellValue::Bool(false)),
            c(CellValue::Empty),
        ],
    ]);
    assert_eq!(
        csv(&d, CsvOptions::default()),
        "plain,\"a,b\",\"say \"\"hi\"\"\",\"two\nlines\"\r\n-1.50,TRUE,FALSE,\r\n"
    );
}

// REQ: FOLIO-REQ-033
#[test]
fn formula_text_is_neutralised_but_numbers_are_not() {
    let d = book(vec![vec![
        text("=SUM(A1)"),
        text("+1"),
        text("-x"),
        text("@cmd"),
        text("\tx"),
        c(CellValue::Number("-5".into())),
        text("safe"),
    ]]);
    let on = csv(&d, CsvOptions::default());
    assert_eq!(on, "'=SUM(A1),'+1,'-x,'@cmd,'\tx,-5,safe\r\n");
    let off = csv(
        &d,
        CsvOptions {
            neutralise_formulas: false,
            ..CsvOptions::default()
        },
    );
    assert!(off.starts_with("=SUM(A1),+1,-x,@cmd,"));
}

// REQ: FOLIO-REQ-033
#[test]
fn bom_option_and_determinism() {
    let d = book(vec![vec![text("é")]]);
    let b = render_csv(
        &d,
        0,
        CsvOptions {
            bom: true,
            ..CsvOptions::default()
        },
        &Limits::default(),
    )
    .unwrap();
    assert_eq!(&b[..3], [0xEF, 0xBB, 0xBF]);
    assert_eq!(
        csv(&d, CsvOptions::default()),
        csv(&d, CsvOptions::default())
    );
}

// REQ: FOLIO-REQ-033
#[test]
fn rejections() {
    let d = book(vec![vec![text("a")]]);
    assert!(matches!(
        render_csv(&d, 3, CsvOptions::default(), &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
    let mut merged = book(vec![vec![text("a"), text("b")]]);
    if let Body::Workbook(s) = &mut merged.body {
        s[0].rows[0].cells = vec![Cell {
            value: CellValue::Empty,
            span: 2,
            role: String::new(),
        }];
    }
    assert!(matches!(
        render_csv(&merged, 0, CsvOptions::default(), &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let page = PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::ZERO,
    };
    let flow = Document {
        meta: meta(),
        body: Body::Flow(vec![Section::new(page, Vec::<Block>::new())]),
    };
    assert!(matches!(
        render_csv(&flow, 0, CsvOptions::default(), &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let big = Limits::default().with_max_file_bytes(5);
    assert!(matches!(
        render_csv(
            &book(vec![vec![text("abcdefgh")]]),
            0,
            CsvOptions::default(),
            &big
        ),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-053
#[test]
fn control_characters_are_rejected() {
    for bad in ["a\0b", "x\u{1}", "\u{7F}"] {
        let d = book(vec![vec![text(bad)]]);
        assert!(
            matches!(
                render_csv(&d, 0, CsvOptions::default(), &Limits::default()),
                Err(FolioError::Invalid(_))
            ),
            "{bad:?}"
        );
    }
    let d = book(vec![vec![text("tab\there\r\nok")]]);
    assert!(render_csv(&d, 0, CsvOptions::default(), &Limits::default()).is_ok());
}
