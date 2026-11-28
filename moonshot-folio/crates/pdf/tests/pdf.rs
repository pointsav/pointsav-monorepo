// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Behaviour tests for the PDF writer (tasks F5.2 to F5.4), checked against independent
//! readers (`pdfinfo`, `pdftotext`, `pdftoppm`, Ghostscript) when they are installed.

#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines
)]

use moonshot_folio_core::{Emu, FolioError, Ink, Limits};
use moonshot_folio_font::Std14;
use moonshot_folio_pdf::{
    Bookmark, Content, Dest, Info, Link, LinkTarget, PdfDoc, Rect, TextPolicy,
};
use std::path::PathBuf;
use std::process::Command;

const LIBERATION: &str = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf";

fn pts(v: i64) -> Emu {
    Emu::from_points(v).unwrap()
}

/// Write a sample for the oracle commands when `FOLIO_PDF_OUT` names a directory.
fn sample(name: &str, bytes: &[u8]) -> Option<PathBuf> {
    let dir = std::env::var_os("FOLIO_PDF_OUT")?;
    let p = PathBuf::from(dir).join(name);
    std::fs::write(&p, bytes).unwrap();
    Some(p)
}

/// Write `bytes` to a scratch file for tools (always), returning its path.
fn scratch(name: &str, bytes: &[u8]) -> PathBuf {
    let p = std::env::temp_dir().join(format!("folio-pdf-test-{}-{name}", std::process::id()));
    std::fs::write(&p, bytes).unwrap();
    p
}

fn have(tool: &str) -> bool {
    Command::new(tool).arg("-v").output().is_ok()
}

fn gs_clean(path: &PathBuf) {
    if !have("gs") {
        return;
    }
    let out = Command::new("gs")
        .args(["-q", "-dNOPAUSE", "-dBATCH", "-sDEVICE=nullpage"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success(), "gs failed");
    assert!(
        out.stderr.is_empty() && out.stdout.is_empty(),
        "gs warned: {}{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
}

fn pdftotext(path: &PathBuf) -> Option<String> {
    if !have("pdftotext") {
        return None;
    }
    let out = Command::new("pdftotext")
        .args(["-layout"])
        .arg(path)
        .arg("-")
        .output()
        .unwrap();
    assert!(out.status.success());
    Some(String::from_utf8(out.stdout).unwrap())
}

fn std_doc() -> Vec<u8> {
    let mut doc = PdfDoc::new(Limits::default());
    doc.set_info(Info {
        title: Some("Std14 sample (1)".into()),
        author: Some("Folio".into()),
        creation_date: Some("2026-11-22".into()),
        ..Info::default()
    })
    .unwrap();
    let reg = doc.add_std14(Std14::Helvetica);
    let bold = doc.add_std14(Std14::HelveticaBold);
    let mut c = Content::new();
    c.save().unwrap();
    c.set_fill(Ink::Rgb(200, 30, 30)).unwrap();
    c.rect((pts(72), pts(600)), (pts(200), pts(40))).unwrap();
    c.fill();
    c.restore().unwrap();
    c.set_stroke(Ink::Cmyk(255, 0, 0, 0)).unwrap();
    c.set_line_width(pts(2)).unwrap();
    c.move_to(pts(72), pts(580)).unwrap();
    c.line_to(pts(540), pts(580)).unwrap();
    c.stroke();
    c.set_fill(Ink::Gray(0)).unwrap();
    let t1 = doc
        .encode_text(
            reg,
            "Caf\u{e9} \u{20ac}5 \u{2013} \u{201c}quoted\u{201d} \u{2022} (paren) \\",
            TextPolicy::Reject,
        )
        .unwrap();
    c.text(pts(14), pts(72), pts(700), &t1).unwrap();
    let t2 = doc
        .encode_text(bold, "Bold Heading", TextPolicy::Reject)
        .unwrap();
    c.text(pts(20), pts(72), pts(660), &t2).unwrap();
    doc.add_page(pts(612), pts(792), &c, vec![]).unwrap();
    doc.build().unwrap()
}

// REQ: FOLIO-REQ-044
#[test]
fn structure_and_xref_offsets_are_exact() {
    let b = std_doc();
    assert!(b.starts_with(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n"));
    assert!(b.ends_with(b"%%EOF\n"));
    let text: String = b
        .iter()
        .map(|c| if c.is_ascii() { char::from(*c) } else { '?' })
        .collect();
    let sx = text.rfind("startxref\n").unwrap();
    let xref_at: usize = text[sx + 10..].lines().next().unwrap().parse().unwrap();
    assert!(b[xref_at..].starts_with(b"xref\n0 "));
    let mut lines = text[xref_at..].lines();
    lines.next();
    let count: usize = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(lines.next().unwrap(), "0000000000 65535 f ");
    for n in 1..count {
        let l = lines.next().unwrap();
        assert_eq!(l.len(), 19, "entry plus newline is 20 bytes");
        let off: usize = l[..10].parse().unwrap();
        assert!(
            b[off..].starts_with(format!("{n} 0 obj\n").as_bytes()),
            "object {n}"
        );
    }
    assert!(text.contains("/ID [<"));
    assert!(text.contains("/CreationDate (D:20261122000000Z)"));
    assert!(text.contains("/Title (Std14 sample \\(1\\))"));
    if let Some(p) = sample("std14.pdf", &b) {
        assert!(p.exists());
    }
}

// REQ: FOLIO-REQ-044
#[test]
fn std14_page_is_accepted_and_text_extracts() {
    let b = std_doc();
    let p = scratch("std14.pdf", &b);
    gs_clean(&p);
    if let Some(t) = pdftotext(&p) {
        assert!(
            t.contains("Caf\u{e9} \u{20ac}5 \u{2013} \u{201c}quoted\u{201d} \u{2022} (paren) \\"),
            "{t}"
        );
        assert!(t.contains("Bold Heading"));
    }
    if have("pdfinfo") {
        let o = Command::new("pdfinfo").arg(&p).output().unwrap();
        assert!(o.status.success());
        let s = String::from_utf8_lossy(&o.stdout);
        assert!(s.contains("Pages:           1"), "{s}");
        assert!(s.contains("612 x 792 pts"), "{s}");
        assert!(s.contains("Std14 sample (1)"), "{s}");
    }
    if have("pdftoppm") {
        let out = std::env::temp_dir().join(format!("folio-pdf-render-{}", std::process::id()));
        let st = Command::new("pdftoppm")
            .args(["-gray", "-r", "40"])
            .arg(&p)
            .arg(&out)
            .status()
            .unwrap();
        assert!(st.success());
        let pgm = std::fs::read(format!("{}-1.pgm", out.display())).unwrap();
        let dark = pgm.iter().skip(20).filter(|v| **v < 200).count();
        assert!(dark > 0, "page rendered blank");
    }
}

fn font_bytes() -> Option<Vec<u8>> {
    match std::fs::read(LIBERATION) {
        Ok(d) => Some(d),
        Err(_) if std::env::var("FOLIO_ALLOW_MISSING_FONT").is_ok_and(|v| v == "1") => None,
        Err(e) => panic!("{LIBERATION}: {e}; install the font or set FOLIO_ALLOW_MISSING_FONT=1"),
    }
}

fn embedded_doc(font: Vec<u8>) -> Vec<u8> {
    let mut doc = PdfDoc::new(Limits::default());
    let f = doc.add_truetype(font).unwrap();
    let mut c = Content::new();
    let l1 = doc
        .encode_text(f, "Caf\u{e9} \u{20ac}100 na\u{ef}ve", TextPolicy::Reject)
        .unwrap();
    let l2 = doc
        .encode_text(f, "Second line ABC abc", TextPolicy::Reject)
        .unwrap();
    c.text(pts(16), pts(72), pts(700), &l1).unwrap();
    c.text(pts(16), pts(72), pts(670), &l2).unwrap();
    doc.add_page(pts(595), pts(842), &c, vec![]).unwrap();
    doc.build().unwrap()
}

// REQ: FOLIO-REQ-045
#[test]
fn embedded_liberation_subset_extracts_exactly() {
    let Some(font) = font_bytes() else {
        return;
    };
    let b = embedded_doc(font.clone());
    assert!(
        b.len() < font.len() / 4,
        "subset must be much smaller than the font"
    );
    let s = String::from_utf8_lossy(&b);
    assert!(s.contains("/Subtype /Type0"));
    assert!(s.contains("/Encoding /Identity-H"));
    assert!(s.contains("/CIDToGIDMap /Identity"));
    assert!(s.contains("/FontFile2"));
    assert!(s.contains("beginbfrange") || s.contains("beginbfchar"));
    let p = scratch("embedded.pdf", &b);
    sample("embedded.pdf", &b);
    gs_clean(&p);
    if let Some(t) = pdftotext(&p) {
        let lines: Vec<&str> = t.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        assert_eq!(
            lines,
            ["Caf\u{e9} \u{20ac}100 na\u{ef}ve", "Second line ABC abc"]
        );
    }
    if have("pdftoppm") {
        let out = std::env::temp_dir().join(format!("folio-pdf-render-e-{}", std::process::id()));
        Command::new("pdftoppm")
            .args(["-gray", "-r", "40"])
            .arg(&p)
            .arg(&out)
            .status()
            .unwrap();
        let pgm = std::fs::read(format!("{}-1.pgm", out.display())).unwrap();
        assert!(pgm.iter().skip(20).any(|v| *v < 128), "blank render");
    }
}

// REQ: FOLIO-REQ-045
#[test]
fn embedded_font_is_deterministic_and_registered_per_use() {
    let Some(font) = font_bytes() else {
        return;
    };
    assert_eq!(embedded_doc(font.clone()), embedded_doc(font));
}

// REQ: FOLIO-REQ-045
#[test]
fn embedded_font_rejects_or_substitutes_missing_glyphs() {
    let Some(font) = font_bytes() else {
        return;
    };
    let mut doc = PdfDoc::new(Limits::default());
    let f = doc.add_truetype(font).unwrap();
    let e = doc
        .encode_text(f, "A\u{65e5}", TextPolicy::Reject)
        .unwrap_err();
    assert!(matches!(e, FolioError::Unsupported(_)));
    let run = doc
        .encode_text(f, "A\u{65e5}", TextPolicy::Substitute)
        .unwrap();
    assert_eq!(run.substituted(), 1);
    assert_eq!(run.bytes().len(), 4);
}

fn nav_doc() -> Vec<u8> {
    let mut doc = PdfDoc::new(Limits::default());
    let f = doc.add_std14(Std14::Helvetica);
    let mut pages = Vec::new();
    for i in 0..3 {
        let mut c = Content::new();
        let r = doc
            .encode_text(f, &format!("Page {}", i + 1), TextPolicy::Reject)
            .unwrap();
        c.text(pts(18), pts(72), pts(700), &r).unwrap();
        pages.push(c);
    }
    let area = Rect {
        x: pts(72),
        y: pts(690),
        w: pts(100),
        h: pts(24),
    };
    let links = vec![
        Link {
            contents: None,
            rect: area,
            target: LinkTarget::Uri("https://example.org/a(b)?q=1".into()),
        },
        Link {
            contents: None,
            rect: area,
            target: LinkTarget::Uri("mailto:someone@example.org".into()),
        },
        Link {
            contents: None,
            rect: area,
            target: LinkTarget::Named("chapter-2".into()),
        },
        Link {
            contents: None,
            rect: area,
            target: LinkTarget::Position(Dest {
                page: 2,
                x: pts(0),
                y: pts(792),
            }),
        },
    ];
    for (i, c) in pages.iter().enumerate() {
        let l = if i == 0 { links.clone() } else { vec![] };
        doc.add_page(pts(612), pts(792), c, l).unwrap();
    }
    doc.add_named_dest(
        "chapter-2",
        Dest {
            page: 1,
            x: pts(72),
            y: pts(720),
        },
    )
    .unwrap();
    let d = |page| Dest {
        page,
        x: pts(0),
        y: pts(792),
    };
    doc.set_outline(vec![
        Bookmark {
            title: "Part one".into(),
            dest: d(0),
            children: vec![
                Bookmark {
                    title: "Chapter 2 \u{2013} \u{e9}".into(),
                    dest: d(1),
                    children: vec![],
                },
                Bookmark {
                    title: "Chapter 3".into(),
                    dest: d(2),
                    children: vec![],
                },
            ],
        },
        Bookmark {
            title: "End".into(),
            dest: d(2),
            children: vec![],
        },
    ])
    .unwrap();
    doc.build().unwrap()
}

// REQ: FOLIO-REQ-046
#[test]
fn links_destinations_and_outline_are_accepted_by_readers() {
    let b = nav_doc();
    let s = String::from_utf8_lossy(&b).into_owned();
    assert!(s.contains("/S /URI /URI (https://example.org/a\\(b\\)?q=1)"));
    assert!(s.contains("/S /GoTo /D (chapter-2)"));
    assert!(s.contains("/Type /Outlines"));
    assert!(s.contains("/Count 4"), "root counts all descendants");
    assert!(s.contains("/PageMode /UseOutlines"));
    let p = scratch("nav.pdf", &b);
    sample("nav.pdf", &b);
    gs_clean(&p);
    if have("pdfinfo") {
        let o = Command::new("pdfinfo").arg(&p).output().unwrap();
        assert!(o.status.success());
        assert!(
            o.stderr.is_empty(),
            "{}",
            String::from_utf8_lossy(&o.stderr)
        );
        assert!(String::from_utf8_lossy(&o.stdout).contains("Pages:           3"));
    }
    if have("gs") {
        // Ghostscript's pdfwrite reads the outline and links; a clean run proves they parse.
        let o = Command::new("gs")
            .args([
                "-q",
                "-dNOPAUSE",
                "-dBATCH",
                "-sDEVICE=pdfwrite",
                "-sOutputFile=/dev/null",
            ])
            .arg(&p)
            .output()
            .unwrap();
        assert!(o.status.success() && o.stderr.is_empty());
    }
}

// REQ: FOLIO-REQ-047
#[test]
fn output_is_byte_deterministic() {
    assert_eq!(std_doc(), std_doc());
    assert_eq!(nav_doc(), nav_doc());
    assert_ne!(std_doc(), nav_doc());
}

// REQ: FOLIO-REQ-048
#[test]
fn unsafe_uris_are_rejected() {
    let mut doc = PdfDoc::new(Limits::default());
    for bad in [
        "javascript:alert(1)",
        "file:///etc/passwd",
        "ftp://example.org/",
        "data:text/html,x",
        "https://exa mple.org/",
        "http://example.org/\n",
        "http://",
        "https://caf\u{e9}.example/",
        "relative/path",
    ] {
        let l = Link {
            contents: None,
            rect: Rect {
                x: pts(0),
                y: pts(0),
                w: pts(10),
                h: pts(10),
            },
            target: LinkTarget::Uri(bad.into()),
        };
        let e = doc
            .add_page(pts(100), pts(100), &Content::new(), vec![l])
            .unwrap_err();
        assert!(matches!(e, FolioError::Invalid(_)), "{bad}");
    }
    let ok = Link {
        contents: None,
        rect: Rect {
            x: pts(0),
            y: pts(0),
            w: pts(10),
            h: pts(10),
        },
        target: LinkTarget::Uri("HTTPS://Example.org/".into()),
    };
    doc.add_page(pts(100), pts(100), &Content::new(), vec![ok])
        .unwrap();
}

// REQ: FOLIO-REQ-048
#[test]
fn negative_cases_are_errors_not_panics() {
    let mut doc = PdfDoc::new(Limits::default());
    let f = doc.add_std14(Std14::Helvetica);
    assert!(matches!(
        doc.encode_text(f, "\u{65e5}\u{672c}", TextPolicy::Reject),
        Err(FolioError::Unsupported(_))
    ));
    let sub = doc
        .encode_text(f, "a\u{65e5}b", TextPolicy::Substitute)
        .unwrap();
    assert_eq!((sub.bytes(), sub.substituted()), (&b"a?b"[..], 1));
    assert!(doc.build().is_err(), "no pages");
    let mut c = Content::new();
    assert!(c.restore().is_err());
    c.save().unwrap();
    assert!(
        doc.add_page(pts(100), pts(100), &c, vec![]).is_err(),
        "unbalanced q"
    );
    assert!(doc
        .add_page(pts(0), pts(100), &Content::new(), vec![])
        .is_err());
    let mut c = Content::new();
    assert!(c.show(&sub).is_err(), "text outside BT");
    c.begin_text().unwrap();
    assert!(c.show(&sub).is_err(), "no font selected");
    assert!(c.set_font(f, pts(0)).is_err());
    assert!(doc
        .set_info(Info {
            creation_date: Some("2026-13-01".into()),
            ..Info::default()
        })
        .is_err());
    assert!(doc
        .set_info(Info {
            creation_date: Some("26-1-1".into()),
            ..Info::default()
        })
        .is_err());
    doc.add_page(pts(100), pts(100), &Content::new(), vec![])
        .unwrap();
    let dest = Dest {
        page: 5,
        x: pts(0),
        y: pts(0),
    };
    doc.add_named_dest("x", dest).unwrap();
    assert!(doc.add_named_dest("x", dest).is_err(), "duplicate");
    assert!(doc.build().is_err(), "destination page missing");
    let mut d2 = PdfDoc::new(Limits::default());
    let l = Link {
        contents: None,
        rect: Rect {
            x: pts(0),
            y: pts(0),
            w: pts(1),
            h: pts(1),
        },
        target: LinkTarget::Named("nope".into()),
    };
    d2.add_page(pts(100), pts(100), &Content::new(), vec![l])
        .unwrap();
    assert!(d2.build().is_err(), "unknown named destination");
}

// REQ: FOLIO-REQ-048
#[test]
fn sizes_are_bounded_by_limits() {
    let small = Limits::default().with_max_entry_bytes(64);
    let mut doc = PdfDoc::new(small);
    let mut c = Content::new();
    for _ in 0..20 {
        c.rect((pts(1), pts(1)), (pts(2), pts(2))).unwrap();
    }
    assert!(matches!(
        doc.add_page(pts(100), pts(100), &c, vec![]),
        Err(FolioError::Limit(_))
    ));
    let few = Limits::default().with_max_zip_entries(2);
    let mut doc = PdfDoc::new(few);
    doc.add_page(pts(100), pts(100), &Content::new(), vec![])
        .unwrap();
    doc.add_page(pts(100), pts(100), &Content::new(), vec![])
        .unwrap();
    assert!(doc
        .add_page(pts(100), pts(100), &Content::new(), vec![])
        .is_err());
    let tiny = Limits::default().with_max_file_bytes(200);
    let mut doc = PdfDoc::new(tiny);
    doc.add_page(pts(100), pts(100), &Content::new(), vec![])
        .unwrap();
    assert!(matches!(doc.build(), Err(FolioError::Limit(_))));
    let shallow = Limits::default().with_max_depth(2);
    let mut doc = PdfDoc::new(shallow);
    let d = Dest {
        page: 0,
        x: pts(0),
        y: pts(0),
    };
    let leaf = Bookmark {
        title: "c".into(),
        dest: d,
        children: vec![],
    };
    let mid = Bookmark {
        title: "b".into(),
        dest: d,
        children: vec![leaf],
    };
    let top = Bookmark {
        title: "a".into(),
        dest: d,
        children: vec![mid],
    };
    assert!(matches!(
        doc.set_outline(vec![top]),
        Err(FolioError::Limit(_))
    ));
    let huge = Emu::from_raw(i64::MAX);
    assert!(Content::new().rect((huge, huge), (huge, huge)).is_err());
}

// REQ: FOLIO-REQ-044
#[test]
fn page_size_uses_integer_point_conversion() {
    let mut doc = PdfDoc::new(Limits::default());
    // A4: 210 mm x 297 mm = 595.276 x 841.890 pt.
    let w = Emu::from_mm(210).unwrap();
    let h = Emu::from_mm(297).unwrap();
    doc.add_page(w, h, &Content::new(), vec![]).unwrap();
    let b = doc.build().unwrap();
    let s = String::from_utf8_lossy(&b);
    assert!(s.contains("/MediaBox [0 0 595.276 841.89]"), "{s}");
}

// REQ: FOLIO-REQ-063
#[test]
fn page_labels_language_and_large_counts_follow_their_rules() {
    let page = |doc: &mut PdfDoc| {
        doc.add_page(pts(100), pts(100), &Content::new(), vec![])
            .unwrap();
    };
    let mut doc = PdfDoc::new(Limits::default());
    for _ in 0..4 {
        page(&mut doc);
    }
    for bad in [
        vec![(1, 1)],
        vec![(0, 1), (0, 1)],
        vec![(0, 1), (3, 0)],
        vec![(0, 1), (3, 1), (2, 1)],
    ] {
        assert!(doc.set_page_labels(bad.clone()).is_err(), "{bad:?}");
    }
    doc.set_page_labels(vec![(0, 1), (2, 1)]).unwrap();
    let t = String::from_utf8_lossy(&doc.build().unwrap()).to_string();
    assert!(t.contains("/PageLabels << /Nums [0 << /S /D /St 1 >> 2 << /S /D /St 1 >> ] >>"));
    doc.set_page_labels(vec![(0, 1), (9, 1)]).unwrap();
    assert!(doc.build().is_err(), "a label run past the last page");
    // The language is a plain tag.
    for bad in ["", "en CA", "en_GB"] {
        let r = doc.set_info(Info {
            lang: Some(bad.into()),
            ..Info::default()
        });
        assert!(r.is_err(), "{bad:?}");
    }
    // Destinations and bookmarks are bounded by max_cells, not by the page limit.
    let mut big = PdfDoc::new(Limits::default());
    page(&mut big);
    for i in 0..10_050 {
        big.add_named_dest(
            &format!("d{i}"),
            Dest {
                page: 0,
                x: pts(1),
                y: pts(1),
            },
        )
        .unwrap();
    }
    let marks: Vec<Bookmark> = (0..10_050)
        .map(|i| Bookmark {
            title: format!("b{i}"),
            dest: Dest {
                page: 0,
                x: pts(1),
                y: pts(1),
            },
            children: vec![],
        })
        .collect();
    big.set_outline(marks).unwrap();
    assert!(big.build().is_ok());
    let mut small = PdfDoc::new(Limits::default().with_max_cells(3));
    page(&mut small);
    for i in 0..3 {
        small
            .add_named_dest(
                &format!("d{i}"),
                Dest {
                    page: 0,
                    x: pts(1),
                    y: pts(1),
                },
            )
            .unwrap();
    }
    assert_eq!(
        small.add_named_dest(
            "d3",
            Dest {
                page: 0,
                x: pts(1),
                y: pts(1)
            }
        ),
        Err(FolioError::Limit("destination count"))
    );
}

// REQ: FOLIO-REQ-064
#[test]
fn an_embedded_font_is_parsed_only_when_a_new_character_appears() {
    let Some(data) = font_bytes() else { return };
    let mut doc = PdfDoc::new(Limits::default());
    let f = doc.add_truetype(data).unwrap();
    assert_eq!(doc.font_parses(), 0, "registering does not count");
    for i in 0..500 {
        let t = format!("Line {} caf\u{e9} \u{20ac}5", i % 10);
        doc.encode_text(f, &t, TextPolicy::Reject).unwrap();
    }
    // New characters arrive only over the first ten lines (the digits).
    assert!(
        doc.font_parses() <= 11,
        "{} parses for 500 lines",
        doc.font_parses()
    );
    let steady = doc.font_parses();
    for _ in 0..500 {
        doc.encode_text(f, "Line 3 caf\u{e9}", TextPolicy::Reject)
            .unwrap();
    }
    assert_eq!(doc.font_parses(), steady, "repeated text never re-parses");
    // A character the font lacks is looked up once and remembered too.
    let r = doc
        .encode_text(f, "\u{4e00}", TextPolicy::Substitute)
        .unwrap();
    assert_eq!(r.substituted(), 1);
    let n = doc.font_parses();
    doc.encode_text(f, "\u{4e00}", TextPolicy::Substitute)
        .unwrap();
    assert_eq!(doc.font_parses(), n);
}
