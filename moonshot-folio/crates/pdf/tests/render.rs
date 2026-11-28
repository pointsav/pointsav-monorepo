// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! End-to-end tests for `render_pdf` (task F5.5). The output is checked by parsing the PDF
//! bytes with a small reader in this file (cross-reference table, page tree, content streams,
//! outline, annotations, named destinations, `ToUnicode` maps); no external tool is used.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    clippy::panic,
    clippy::range_plus_one,
    clippy::cast_precision_loss,
    clippy::items_after_statements
)]

use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_font::{Font, MissingGlyph};
use moonshot_folio_ir::{
    Block, Body, BreakKind, Cell, CellValue, Document, FieldKind, Heading, Inline, List, Meta,
    PageSetup, Pagination, Para, Row, Section, Table,
};
use moonshot_folio_layout::{headings, layout, Face, Fonts, ItemKind, Options, Theme};
use moonshot_folio_pdf::{bad_links, render_pdf, render_tree, RenderOptions};
use std::collections::BTreeMap;

const LIBERATION: &str = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf";
const LIBERATION_BOLD: &str = "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf";

fn pts(n: i64) -> Emu {
    Emu::from_points(n).unwrap()
}

// ---------------------------------------------------------------------------------------
// IR builders

fn meta() -> Meta {
    Meta {
        title: "Render test".into(),
        author: "Synthetic".into(),
        date: "2026-03-04".into(),
        lang: "en".into(),
    }
}

fn section(blocks: Vec<Block>) -> Section {
    Section::new(
        PageSetup {
            width: pts(200),
            height: pts(300),
            margin: pts(20),
        },
        blocks,
    )
}

fn doc(blocks: Vec<Block>) -> Document {
    Document {
        meta: meta(),
        body: Body::Flow(vec![section(blocks)]),
    }
}

fn text(s: &str) -> Vec<Inline> {
    vec![Inline::Text(s.into())]
}

fn para(s: &str) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines: text(s),
        pagination: Pagination::default(),
    })
}

fn para_in(i: Vec<Inline>) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines: i,
        pagination: Pagination::default(),
    })
}

fn heading(level: u8, s: &str) -> Block {
    Block::Heading(Heading {
        level,
        inlines: text(s),
        pagination: Pagination::default(),
    })
}

fn cell(s: &str) -> Cell {
    Cell {
        value: CellValue::Text(s.into()),
        span: 1,
        role: String::new(),
    }
}

fn words(seed: usize, n: usize) -> String {
    const W: [&str; 8] = [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel",
    ];
    (0..n)
        .map(|k| W[(seed * 3 + k * 5) % W.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

fn render(d: &Document) -> Vec<u8> {
    render_pdf(
        d,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default(),
    )
    .unwrap()
}

// ---------------------------------------------------------------------------------------
// A small PDF reader

struct Pdf {
    bytes: Vec<u8>,
    /// Body range of each object, by number (index 0 unused).
    objs: Vec<(usize, usize)>,
}

fn find(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    hay.get(from..)?
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + from)
}

fn rfind(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).rposition(|w| w == needle)
}

fn number_after(s: &str, key: &str) -> Option<usize> {
    let at = s.find(key)? + key.len();
    s[at..]
        .trim_start()
        .split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse()
        .ok()
}

/// Object numbers referenced as `N 0 R` in `s`, in order.
fn refs_in(s: &str) -> Vec<usize> {
    let t: Vec<&str> = s.split_whitespace().collect();
    let mut out = Vec::new();
    for w in t.windows(3) {
        if w[1] == "0" && w[2].trim_end_matches(']') == "R" {
            if let Ok(n) = w[0].trim_start_matches('[').parse() {
                out.push(n);
            }
        }
    }
    out
}

impl Pdf {
    fn parse(bytes: Vec<u8>) -> Self {
        assert!(bytes.starts_with(b"%PDF-1.7"));
        let sx = rfind(&bytes, b"startxref\n").unwrap();
        let xref_at: usize = String::from_utf8_lossy(&bytes[sx + 10..])
            .lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let table = String::from_utf8_lossy(&bytes[xref_at..sx]).to_string();
        let mut lines = table.lines();
        assert_eq!(lines.next(), Some("xref"));
        let n: usize = lines
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let offsets: Vec<usize> = lines
            .skip(1)
            .take(n - 1)
            .map(|l| l[..10].parse().unwrap())
            .collect();
        let mut objs = vec![(0, 0)];
        for (i, off) in offsets.iter().enumerate() {
            let header = format!("{} 0 obj\n", i + 1);
            assert_eq!(&bytes[*off..*off + header.len()], header.as_bytes());
            let end = offsets.get(i + 1).copied().unwrap_or(xref_at);
            objs.push((*off + header.len(), end - "\nendobj\n".len()));
        }
        Self { bytes, objs }
    }

    fn body(&self, id: usize) -> &[u8] {
        let (a, b) = self.objs[id];
        &self.bytes[a..b]
    }

    /// The dictionary text of an object (up to its stream, if any).
    fn dict(&self, id: usize) -> String {
        let b = self.body(id);
        let end = find(b, b"\nstream\n", 0).unwrap_or(b.len());
        String::from_utf8_lossy(&b[..end]).to_string()
    }

    fn stream(&self, id: usize) -> &[u8] {
        let b = self.body(id);
        let s = find(b, b"\nstream\n", 0).unwrap() + 8;
        let e = b.len() - "\nendstream".len();
        &b[s..e]
    }

    fn trailer(&self) -> String {
        let t = rfind(&self.bytes, b"trailer\n").unwrap();
        String::from_utf8_lossy(&self.bytes[t..]).to_string()
    }

    fn catalog(&self) -> usize {
        number_after(&self.trailer(), "/Root").unwrap()
    }

    fn page_ids(&self) -> Vec<usize> {
        let cat = self.dict(self.catalog());
        let tree = number_after(&cat, "/Pages").unwrap();
        let t = self.dict(tree);
        let count = number_after(&t, "/Count").unwrap();
        let kids = refs_in(&t[t.find("/Kids").unwrap()..t.find(']').unwrap()]);
        assert_eq!(kids.len(), count, "/Count equals the number of kids");
        kids
    }

    /// Font resource name (`F1`) to font object, for one page.
    fn page_fonts(&self, page: usize) -> BTreeMap<String, usize> {
        let d = self.dict(page);
        let at = d.find("/Font <<").unwrap() + 8;
        let end = d[at..].find(">>").unwrap() + at;
        let mut out = BTreeMap::new();
        let toks: Vec<&str> = d[at..end].split_whitespace().collect();
        for w in toks.chunks(4) {
            if w.len() == 4 {
                out.insert(
                    w[0].trim_start_matches('/').to_string(),
                    w[1].parse().unwrap(),
                );
            }
        }
        out
    }

    /// The glyph-id to character map of a `Type0` font's `ToUnicode`.
    fn to_unicode(&self, font: usize) -> BTreeMap<u16, char> {
        let d = self.dict(font);
        let Some(tu) = number_after(&d, "/ToUnicode") else {
            return BTreeMap::new();
        };
        let cmap = String::from_utf8_lossy(self.stream(tu)).to_string();
        let hex =
            |s: &str| u32::from_str_radix(s.trim_matches(|c| c == '<' || c == '>'), 16).unwrap();
        let mut map = BTreeMap::new();
        let mut mode = "";
        for line in cmap.lines() {
            if line.ends_with("beginbfchar") {
                mode = "char";
            } else if line.ends_with("beginbfrange") {
                mode = "range";
            } else if line.starts_with("end") {
                mode = "";
            } else if mode == "char" {
                let p: Vec<&str> = line.split_whitespace().collect();
                map.insert(hex(p[0]) as u16, char::from_u32(hex(p[1])).unwrap());
            } else if mode == "range" {
                let p: Vec<&str> = line.split_whitespace().collect();
                for k in hex(p[0])..=hex(p[1]) {
                    map.insert(
                        k as u16,
                        char::from_u32(hex(p[2]) + (k - hex(p[0]))).unwrap(),
                    );
                }
            }
        }
        map
    }

    /// The strings shown on a page, in order.
    fn page_text(&self, page: usize) -> Vec<String> {
        let d = self.dict(page);
        let content = number_after(&d, "/Contents").unwrap();
        let fonts = self.page_fonts(page);
        let maps: BTreeMap<String, BTreeMap<u16, char>> = fonts
            .iter()
            .map(|(k, v)| (k.clone(), self.to_unicode(*v)))
            .collect();
        let mut current = String::new();
        let mut out = Vec::new();
        for line in self.stream(content).split(|b| *b == b'\n') {
            if line.ends_with(b" Tf") {
                let s = String::from_utf8_lossy(line).to_string();
                current = s
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim_start_matches('/')
                    .to_string();
            } else if line.ends_with(b" Tj") {
                let body = &line[..line.len() - 3];
                if body.starts_with(b"(") {
                    out.push(literal(&body[1..body.len() - 1]));
                } else {
                    let hex = String::from_utf8_lossy(&body[1..body.len() - 1]).to_string();
                    let map = &maps[&current];
                    let mut s = String::new();
                    for ch in hex.as_bytes().chunks(4) {
                        let g = u16::from_str_radix(std::str::from_utf8(ch).unwrap(), 16).unwrap();
                        s.push(map[&g]);
                    }
                    out.push(s);
                }
            }
        }
        out
    }

    fn annots(&self, page: usize) -> Vec<String> {
        let d = self.dict(page);
        let Some(at) = d.find("/Annots [") else {
            return Vec::new();
        };
        let end = d[at..].find(']').unwrap() + at;
        refs_in(&d[at..end])
            .into_iter()
            .map(|r| self.dict(r))
            .collect()
    }

    /// Named destinations: name to page object and position (points).
    fn names(&self) -> BTreeMap<String, Pos> {
        let cat = self.dict(self.catalog());
        let Some(n) = number_after(&cat, "/Dests") else {
            return BTreeMap::new();
        };
        let d = self.body(n).to_vec();
        let start = find(&d, b"/Names [", 0).unwrap() + 8;
        let mut at = start;
        let mut out = BTreeMap::new();
        while d.get(at).is_some_and(|c| *c != b']') {
            if d[at] == b'(' {
                let (name, next) = read_literal(&d, at);
                let rest = String::from_utf8_lossy(&d[next..]).to_string();
                let arr = &rest[..rest.find(']').unwrap() + 1];
                out.insert(name, pos_of(arr));
                at = next + arr.len();
            } else {
                at += 1;
            }
        }
        out
    }

    /// The outline as nested `(title, page object, children)`.
    fn outline(&self) -> Vec<Outline> {
        let cat = self.dict(self.catalog());
        let Some(root) = number_after(&cat, "/Outlines") else {
            return Vec::new();
        };
        let r = self.dict(root);
        let first = number_after(&r, "/First").unwrap();
        self.siblings(first)
    }

    fn siblings(&self, first: usize) -> Vec<Outline> {
        let mut out = Vec::new();
        let mut cur = Some(first);
        while let Some(id) = cur {
            let d = self.dict(id);
            let t = d.find("/Title ").unwrap() + 7;
            let title = if d[t..].starts_with("<FEFF") {
                let hex = &d[t + 1..t + d[t..].find('>').unwrap()];
                let units: Vec<u16> = hex.as_bytes()[4..]
                    .chunks(4)
                    .map(|c| u16::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
                    .collect();
                String::from_utf16(&units).unwrap()
            } else {
                d[t + 1..t + d[t..].find(')').unwrap()].to_string()
            };
            let dest = &d[d.find("/Dest ").unwrap()..];
            let p = pos_of(&dest[6..dest.find(']').unwrap() + 1]);
            let page = p.page;
            let children = number_after(&d, "/First").map_or(Vec::new(), |f| self.siblings(f));
            out.push(Outline {
                title,
                page,
                x: p.x,
                y: p.y,
                children,
            });
            cur = number_after(&d, "/Next");
        }
        out
    }
}

#[derive(Debug, PartialEq)]
struct Outline {
    title: String,
    page: usize,
    x: f64,
    y: f64,
    children: Vec<Outline>,
}

/// A destination: page object and position in points.
#[derive(Debug, PartialEq)]
struct Pos {
    page: usize,
    x: f64,
    y: f64,
}

/// Parse `[N 0 R /XYZ x y null]`.
fn pos_of(arr: &str) -> Pos {
    let t: Vec<&str> = arr
        .trim()
        .trim_matches(|c| c == '[' || c == ']')
        .split_whitespace()
        .collect();
    Pos {
        page: t[0].parse().unwrap(),
        x: t[4].parse().unwrap(),
        y: t[5].parse().unwrap(),
    }
}

/// Read a literal string starting at `(` in `d`: the decoded text and the offset after `)`.
fn read_literal(d: &[u8], open: usize) -> (String, usize) {
    let mut i = open + 1;
    while d[i] != b')' {
        i += if d[i] == b'\\' { 2 } else { 1 };
    }
    (literal(&d[open + 1..i]), i + 1)
}

/// Points of a length, as the writer prints them (at most three decimals).
fn pt_f(e: Emu) -> f64 {
    e.raw() as f64 / 12700.0
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.002
}

/// A literal string body to text: escapes undone, `WinAnsi` mapped to characters.
fn literal(b: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        i += 1;
        let byte = if c == b'\\' {
            let n = b[i];
            i += 1;
            match n {
                b'0'..=b'7' => {
                    let mut v = u32::from(n - b'0');
                    for _ in 0..2 {
                        if i < b.len() && (b'0'..=b'7').contains(&b[i]) {
                            v = v * 8 + u32::from(b[i] - b'0');
                            i += 1;
                        }
                    }
                    v as u8
                }
                other => other,
            }
        } else {
            c
        };
        out.push(match byte {
            0x80 => '\u{20ac}',
            0x91 => '\u{2018}',
            0x92 => '\u{2019}',
            0x93 => '\u{201c}',
            0x94 => '\u{201d}',
            0x95 => '\u{2022}',
            0x96 => '\u{2013}',
            0x97 => '\u{2014}',
            other => char::from(other),
        });
    }
    out
}

fn page_index(pdf: &Pdf, page_obj: usize) -> usize {
    pdf.page_ids().iter().position(|p| *p == page_obj).unwrap()
}

// ---------------------------------------------------------------------------------------
// Tests

// REQ: FOLIO-REQ-063
#[test]
fn pages_text_rectangles_fonts_and_info_come_from_the_layout() {
    let mut rows = vec![Row {
        cells: vec![cell("Item"), cell("Amount")],
        can_split: false,
    }];
    for i in 0..30 {
        rows.push(Row {
            cells: vec![cell(&format!("row {i}")), cell("1.50")],
            can_split: false,
        });
    }
    let d = doc(vec![
        heading(1, "Title"),
        para(&words(1, 60)),
        Block::List(List {
            numbering: 1,
            ordered: false,
            items: vec![text("one"), text("two")],
        }),
        Block::Table(Table {
            columns: 2,
            repeat_header_rows: 1,
            rows,
        }),
    ]);
    let tree = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.page_ids();
    assert_eq!(pages.len(), tree.pages.len());
    assert!(pages.len() > 2);
    for (i, p) in pages.iter().enumerate() {
        let want: Vec<String> = tree.pages[i]
            .items
            .iter()
            .filter_map(|it| match &it.kind {
                moonshot_folio_layout::ItemKind::Text(t) => Some(t.text.clone()),
                moonshot_folio_layout::ItemKind::Rect(_) => None,
            })
            .collect();
        assert_eq!(pdf.page_text(*p), want, "page {i} text, in paint order");
        assert!(pdf.dict(*p).contains("/MediaBox [0 0 200 300]"));
        let rects = tree.pages[i]
            .items
            .iter()
            .filter(|it| matches!(it.kind, moonshot_folio_layout::ItemKind::Rect(_)))
            .count();
        let content = number_after(&pdf.dict(*p), "/Contents").unwrap();
        let ops = String::from_utf8_lossy(pdf.stream(content)).to_string();
        assert_eq!(ops.lines().filter(|l| l.ends_with(" re")).count(), rects);
    }
    let all: String = (0..pdf.objs.len()).skip(1).map(|i| pdf.dict(i)).collect();
    assert!(all.contains("/BaseFont /Helvetica "));
    assert!(all.contains("/BaseFont /Helvetica-Bold "));
    assert!(all.contains("/Title (Render test)"));
    assert!(all.contains("/Author (Synthetic)"));
    assert!(all.contains("/CreationDate (D:20260304000000Z)"));
    assert!(all.contains("/Count"));
}

// REQ: FOLIO-REQ-063
#[test]
fn info_dates_and_strings_come_from_the_options_and_never_a_clock() {
    let d = doc(vec![para("x")]);
    let opts = RenderOptions {
        creation_date: Some("2031-12-31".into()),
        subject: Some("Subject".into()),
        keywords: Some("a, b".into()),
        creator: Some("Tool".into()),
        ..RenderOptions::default()
    };
    let bytes = render_pdf(&d, &Fonts::std14(), &Theme::standard(), &opts).unwrap();
    let t = String::from_utf8_lossy(&bytes).to_string();
    assert!(t.contains("/CreationDate (D:20311231000000Z)"));
    assert!(t.contains("/Subject (Subject)") && t.contains("/Keywords (a, b)"));
    assert!(t.contains("/Creator (Tool)"));
    let bad = RenderOptions {
        creation_date: Some("yesterday".into()),
        ..RenderOptions::default()
    };
    assert!(render_pdf(&d, &Fonts::std14(), &Theme::standard(), &bad).is_err());
}

// REQ: FOLIO-REQ-063
#[test]
fn the_outline_nests_headings_including_skipped_levels() {
    // 1 A, 3 B (level skipped), 2 C, 1 D, 4 E (skips two), 3 F, 1 G.
    let mut blocks = Vec::new();
    for (l, t) in [
        (1, "A"),
        (3, "B"),
        (2, "C"),
        (1, "D"),
        (4, "E"),
        (3, "F"),
        (1, "G"),
    ] {
        blocks.push(heading(l, t));
        blocks.push(para(&words(l as usize, 40)));
    }
    let pdf = Pdf::parse(render(&doc(blocks)));
    let o = pdf.outline();
    let shape = |o: &[Outline]| -> String {
        fn go(o: &[Outline], out: &mut String) {
            for b in o {
                out.push_str(&b.title);
                if !b.children.is_empty() {
                    out.push('[');
                    go(&b.children, out);
                    out.push(']');
                }
                out.push(' ');
            }
        }
        let mut s = String::new();
        go(o, &mut s);
        s.trim_end().to_string()
    };
    // E nests under D, F (level 3) is a sibling of E's parent chain: F follows E (level 4) so it
    // returns to level 3 under D.
    assert_eq!(shape(&o), "A[B C ] D[E F ] G");
    let root = pdf.dict(number_after(&pdf.dict(pdf.catalog()), "/Outlines").unwrap());
    assert!(root.contains("/Count 7"), "{root}");
    assert!(pdf.dict(pdf.catalog()).contains("/PageMode /UseOutlines"));
}

// REQ: FOLIO-REQ-063
#[test]
fn outline_destinations_are_the_pages_of_the_headings() {
    let mut blocks = Vec::new();
    for i in 0..12 {
        blocks.push(heading(1, &format!("Part {i}")));
        blocks.push(para(&words(i, 90)));
    }
    let d = doc(blocks);
    let tree = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    let hs = headings(&tree);
    let pdf = Pdf::parse(render(&d));
    let o = pdf.outline();
    assert_eq!(o.len(), 12);
    for (b, h) in o.iter().zip(&hs) {
        assert_eq!(b.title, h.title);
        assert_eq!(page_index(&pdf, b.page), h.page);
    }
}

// REQ: FOLIO-REQ-063
#[test]
fn links_and_named_destinations_are_written_and_resolve_to_pages() {
    let d = doc(vec![
        para_in(vec![
            Inline::Anchor("start".into()),
            Inline::Text("see ".into()),
            Inline::Link {
                target: "#end".into(),
                text: "the end".into(),
            },
            Inline::Text(" or ".into()),
            Inline::Link {
                target: "https://example.org/a".into(),
                text: "site".into(),
            },
        ]),
        Block::Break(BreakKind::Page),
        para_in(vec![
            Inline::Anchor("end".into()),
            Inline::Text("End".into()),
        ]),
    ]);
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.page_ids();
    assert_eq!(pages.len(), 2);
    let names = pdf.names();
    assert_eq!(names.len(), 2);
    assert_eq!(names["start"].page, pages[0]);
    assert_eq!(names["end"].page, pages[1]);
    let annots = pdf.annots(pages[0]);
    assert_eq!(annots.len(), 2);
    assert!(annots[0].contains("/S /GoTo /D (end)"));
    assert!(annots[1].contains("/S /URI /URI (https://example.org/a)"));
    assert!(pdf.annots(pages[1]).is_empty());
}

// REQ: FOLIO-REQ-063
#[test]
fn repeated_header_rows_are_drawn_on_every_page() {
    let mut rows = vec![Row {
        cells: vec![cell("HeadA"), cell("HeadB")],
        can_split: false,
    }];
    for i in 0..40 {
        rows.push(Row {
            cells: vec![cell(&format!("r{i}")), cell("v")],
            can_split: false,
        });
    }
    let pdf = Pdf::parse(render(&doc(vec![Block::Table(Table {
        columns: 2,
        repeat_header_rows: 1,
        rows,
    })])));
    let pages = pdf.page_ids();
    assert!(pages.len() > 2);
    for p in &pages {
        let t = pdf.page_text(*p);
        assert!(t.contains(&"HeadA".to_string()) && t.contains(&"HeadB".to_string()));
    }
}

/// A system font file. A missing file fails the test unless `FOLIO_ALLOW_MISSING_FONT=1`.
fn font_file(path: &str) -> Option<Vec<u8>> {
    match std::fs::read(path) {
        Ok(d) => Some(d),
        Err(_) if std::env::var("FOLIO_ALLOW_MISSING_FONT").is_ok_and(|v| v == "1") => None,
        Err(e) => panic!("{path}: {e}; install the font or set FOLIO_ALLOW_MISSING_FONT=1"),
    }
}

fn liberation() -> Option<Vec<u8>> {
    font_file(LIBERATION)
}

// REQ: FOLIO-REQ-064
#[test]
fn truetype_text_is_embedded_once_and_extracts_exactly() {
    let Some(data) = liberation() else { return };
    let font = Font::parse(&data, &Limits::default()).unwrap();
    let fonts = Fonts::new(Face::TrueType(font.clone()), Face::TrueType(font));
    let lines = [
        "Caf\u{e9} cr\u{e8}me \u{20ac}5",
        "na\u{ef}ve \u{dc}ber \u{2013} \u{201c}quoted\u{201d}",
    ];
    let d = doc(vec![
        heading(1, "Gr\u{fc}\u{df}e"),
        para(lines[0]),
        para(lines[1]),
        para_in(vec![Inline::Run {
            role: "bold".into(),
            text: "Fett".into(),
        }]),
    ]);
    let bytes = render_pdf(&d, &fonts, &Theme::standard(), &RenderOptions::default()).unwrap();
    let pdf = Pdf::parse(bytes);
    let all: String = (1..pdf.objs.len()).map(|i| pdf.dict(i)).collect();
    assert_eq!(
        all.matches("/FontFile2").count(),
        1,
        "regular and bold share one embedded font"
    );
    assert!(all.contains("/Subtype /CIDFontType2") && all.contains("/ToUnicode"));
    let page = pdf.page_ids()[0];
    let t = pdf.page_text(page);
    assert_eq!(t, ["Gr\u{fc}\u{df}e", lines[0], lines[1], "Fett"]);
    // The outline title carries the non-ASCII text as UTF-16.
    assert_eq!(pdf.outline()[0].title, "Gr\u{fc}\u{df}e");
}

// REQ: FOLIO-REQ-064
#[test]
fn rendering_is_deterministic() {
    let mut blocks = vec![Block::Field(FieldKind::Toc)];
    for i in 0..10 {
        blocks.push(heading(1 + (i % 2) as u8, &format!("Heading {i}")));
        blocks.push(para(&words(i, 70)));
    }
    let d = doc(blocks);
    let (a, b) = (render(&d), render(&d.clone()));
    assert_eq!(a, b);
    let t = String::from_utf8_lossy(&a).to_string();
    let id = &t[t.rfind("/ID [<").unwrap() + 6..][..32];
    assert!(id.bytes().all(|c| c.is_ascii_hexdigit()));
    // A one-character change changes the bytes and the /ID.
    let mut other = d.clone();
    if let Body::Flow(s) = &mut other.body {
        s[0].blocks.push(para("x"));
    }
    assert_ne!(render(&other), a);
}

// REQ: FOLIO-REQ-064
#[test]
fn contents_page_numbers_match_the_pages_the_headings_are_on() {
    let mut blocks = vec![Block::Field(FieldKind::Toc)];
    for i in 0..14 {
        blocks.push(heading(3, &format!("Chapter {i} {}", words(i, 1))));
        blocks.push(para(&words(i, 90)));
    }
    let pdf = Pdf::parse(render(&doc(blocks)));
    let pages = pdf.page_ids();
    assert!(pages.len() > 10, "{} pages", pages.len());
    // Per page, the strings shown. Heading i is the last page holding its title as one string;
    // its contents entry is the title followed by dots and the number.
    let texts: Vec<Vec<String>> = pages.iter().map(|p| pdf.page_text(*p)).collect();
    let mut two_digit = 0;
    for i in 0..14 {
        let title = format!("Chapter {i} {}", words(i, 1));
        let (heading_page, last_at) = (0..texts.len())
            .rev()
            .find_map(|p| texts[p].iter().rposition(|s| *s == title).map(|k| (p, k)))
            .unwrap();
        let (entry_page, at) = (0..texts.len())
            .find_map(|p| texts[p].iter().position(|s| *s == title).map(|k| (p, k)))
            .unwrap();
        assert!(
            (entry_page, at) < (heading_page, last_at),
            "the entry comes first"
        );
        let number = &texts[entry_page][at + 2];
        assert!(texts[entry_page][at + 1].chars().all(|c| c == '.'));
        assert_eq!(*number, (heading_page + 1).to_string(), "entry for {title}");
        two_digit += usize::from(number.len() == 2);
    }
    assert!(two_digit > 0, "some page numbers have two digits");
    // The entries link to the headings' destinations.
    let names = pdf.names();
    assert_eq!(names.len(), 14);
    let goto: usize = pages
        .iter()
        .map(|p| {
            pdf.annots(*p)
                .iter()
                .filter(|a| a.contains("/S /GoTo"))
                .count()
        })
        .sum();
    assert!(goto >= 28);
}

// REQ: FOLIO-REQ-065
#[test]
fn writer_and_layout_errors_surface_unchanged() {
    let bad_link = doc(vec![para_in(vec![Inline::Link {
        target: "javascript:alert(1)".into(),
        text: "x".into(),
    }])]);
    let r = render_pdf(
        &bad_link,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default(),
    );
    assert!(matches!(r, Err(FolioError::Invalid(_))));
    let odd = doc(vec![para("na\u{101}ve")]);
    let r = render_pdf(
        &odd,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default(),
    );
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("character not in font")
    );
    let lenient = RenderOptions {
        missing_glyph: MissingGlyph::Notdef,
        ..RenderOptions::default()
    };
    let ok = render_pdf(&odd, &Fonts::std14(), &Theme::standard(), &lenient).unwrap();
    let pdf = Pdf::parse(ok);
    assert_eq!(pdf.page_text(pdf.page_ids()[0]), ["na?ve"]);
    let workbook = Document {
        meta: meta(),
        body: Body::Workbook(vec![]),
    };
    assert!(render_pdf(
        &workbook,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default()
    )
    .is_err());
    let few = RenderOptions {
        limits: Limits::default().with_max_zip_entries(2),
        ..RenderOptions::default()
    };
    let long = doc((0..30).map(|i| para(&words(i, 60))).collect());
    let r = render_pdf(&long, &Fonts::std14(), &Theme::standard(), &few);
    assert_eq!(r.unwrap_err(), FolioError::Limit("pages"));
}

// ---------------------------------------------------------------------------------------
// Review findings

fn link_para(parts: Vec<Inline>) -> Block {
    para_in(parts)
}

fn titles_of(o: &[Outline]) -> Vec<String> {
    o.iter().map(|b| b.title.clone()).collect()
}

// REQ: FOLIO-REQ-063
#[test]
fn wrapped_heading_titles_are_exact_in_the_outline() {
    let titles = [
        (
            1_u8,
            "A well-known hyphenated-compound state-of-the-art-example",
        ),
        (
            3,
            "https://example.org/alpha/bravo/charlie/delta/echo/foxtrot/golf/hotel",
        ),
        (2, "plain words that wrap at ordinary spaces only"),
    ];
    let long = "x".repeat(300);
    let mut blocks = vec![Block::Field(FieldKind::Toc)];
    for (l, t) in &titles {
        blocks.push(heading(*l, t));
        blocks.push(para("body"));
    }
    blocks.push(heading(2, &long));
    let pdf = Pdf::parse(render(&doc(blocks)));
    let mut want: Vec<String> = titles.iter().map(|t| t.1.to_string()).collect();
    want.push(long);
    // Nesting aside, the bookmarks in document order carry exactly the IR text.
    fn flat(o: &[Outline], out: &mut Vec<String>) {
        for b in o {
            out.push(b.title.clone());
            flat(&b.children, out);
        }
    }
    let mut got = Vec::new();
    flat(&pdf.outline(), &mut got);
    assert_eq!(got, want);
}

// REQ: FOLIO-REQ-063
#[test]
fn headings_made_only_of_links_are_bookmarked_and_empty_ones_are_not() {
    let link = |t: &str| Inline::Link {
        target: "https://example.org/x".into(),
        text: t.into(),
    };
    let h = |inlines: Vec<Inline>| {
        Block::Heading(Heading {
            level: 1,
            inlines,
            pagination: Pagination::default(),
        })
    };
    let d = doc(vec![
        Block::Field(FieldKind::Toc),
        h(vec![link("Only a link")]),
        para("a"),
        h(vec![
            Inline::Text("Pre ".into()),
            link("mid"),
            Inline::Text(" post".into()),
        ]),
        h(vec![Inline::Anchor("just-anchor".into())]),
        h(vec![]),
    ]);
    let pdf = Pdf::parse(render(&d));
    assert_eq!(titles_of(&pdf.outline()), ["Only a link", "Pre mid post"]);
    let names = pdf.names();
    let reserved = names.keys().filter(|n| n.starts_with("folio-toc-")).count();
    assert_eq!(reserved, 2, "{names:?}");
    assert!(names.contains_key("just-anchor"));
}

/// Page label runs of a catalog: per page, the label number the PDF gives it.
fn page_labels(pdf: &Pdf) -> Option<Vec<u32>> {
    let cat = pdf.dict(pdf.catalog());
    let at = cat.find("/PageLabels")?;
    let rest = &cat[at..];
    let nums = &rest[rest.find('[')? + 1..rest.find("] >>")?];
    let t: Vec<&str> = nums.split_whitespace().collect();
    let mut runs = Vec::new();
    for c in t.chunks(7) {
        // `i << /S /D /St n >>`
        assert_eq!((c[1], c[2], c[3], c[4]), ("<<", "/S", "/D", "/St"), "{c:?}");
        runs.push((c[0].parse::<usize>().unwrap(), c[5].parse::<u32>().unwrap()));
    }
    let n = pdf.page_ids().len();
    let mut out = Vec::new();
    for i in 0..n {
        let (start_page, start) = runs.iter().rev().find(|(p, _)| *p <= i).unwrap();
        out.push(start + (i - start_page) as u32);
    }
    Some(out)
}

// REQ: FOLIO-REQ-064
#[test]
fn restarted_page_numbers_are_written_as_page_labels() {
    let sec = |blocks, restart| {
        let mut s = section(blocks);
        s.restart_page_numbers = restart;
        s
    };
    let body =
        |seed: usize| -> Vec<Block> { (0..4).map(|i| para(&words(seed + i, 120))).collect() };
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![
            sec(body(0), false),
            sec(body(5), true),
            sec(body(9), false),
        ]),
    };
    let tree = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    let want: Vec<u32> = tree.pages.iter().map(|p| p.label).collect();
    assert!(
        want.windows(2).any(|w| w[1] != w[0] + 1),
        "a restart happened: {want:?}"
    );
    let pdf = Pdf::parse(render(&d));
    assert_eq!(page_labels(&pdf).unwrap(), want);
    // Without a restart the labels are the default, so nothing is written.
    let plain = Pdf::parse(render(&doc(body(0))));
    assert!(page_labels(&plain).is_none());
}

// REQ: FOLIO-REQ-063
#[test]
fn the_document_language_is_written_to_the_catalog() {
    let pdf = Pdf::parse(render(&doc(vec![para("x")])));
    assert!(pdf.dict(pdf.catalog()).contains("/Lang (en)"));
    let mut d = doc(vec![para("x")]);
    d.meta.lang = "de-CH".into();
    let pdf = Pdf::parse(render(&d));
    assert!(pdf.dict(pdf.catalog()).contains("/Lang (de-CH)"));
}

// REQ: FOLIO-REQ-065
#[test]
fn oversize_elements_degrade_or_fail_early_and_clearly() {
    // (a) A heading title over the bookmark limit is cut at a character boundary.
    let long: String = "\u{e9}".repeat(3000);
    let pdf = Pdf::parse(render(&doc(vec![heading(3, &long), para("x")])));
    let title = pdf.outline().remove(0).title;
    assert_eq!(
        title.chars().count(),
        2048,
        "4096 bytes of two-byte characters"
    );
    assert!(title.chars().all(|c| c == '\u{e9}'));
    // (c) Dates must be real calendar dates, in the metadata and in the options.
    let mut d = doc(vec![para("x")]);
    for bad in ["2026-13-45", "0000-00-00", "2026-02-31"] {
        d.meta.date = bad.into();
        let r = render_pdf(
            &d,
            &Fonts::std14(),
            &Theme::standard(),
            &RenderOptions::default(),
        );
        assert!(matches!(r, Err(FolioError::Invalid(_))), "{bad}");
    }
    d.meta.date = "2024-02-29".into();
    assert!(render_pdf(
        &d,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default()
    )
    .is_ok());
    let opts = RenderOptions {
        creation_date: Some("2026-04-31".into()),
        ..RenderOptions::default()
    };
    assert!(render_pdf(&d, &Fonts::std14(), &Theme::standard(), &opts).is_err());
    // (d) A page side over 200 inches fails in layout, before any page is built.
    let mut huge = doc(vec![para("x")]);
    if let Body::Flow(s) = &mut huge.body {
        s[0].page.width = Emu::from_inches(201).unwrap();
    }
    let r = render_pdf(
        &huge,
        &Fonts::std14(),
        &Theme::standard(),
        &RenderOptions::default(),
    );
    assert_eq!(
        r.unwrap_err(),
        FolioError::Invalid("page size or margin out of range")
    );
}

// REQ: FOLIO-REQ-065
#[test]
fn many_headings_are_bookmarked_and_the_offending_links_can_be_named() {
    // (b) More than 10000 bookmarks used to hit the object limit meant for pages.
    let blocks: Vec<Block> = (0..10_050).map(|i| heading(3, &format!("H{i}"))).collect();
    let pdf = Pdf::parse(render(&doc(blocks)));
    let root = pdf.dict(number_after(&pdf.dict(pdf.catalog()), "/Outlines").unwrap());
    assert!(root.contains("/Count 10050"), "{root}");
    // Bad links: the failure is a reason only (errors carry static text), `bad_links` names them.
    let d = doc(vec![
        para_in(vec![Inline::Link {
            target: "javascript:alert(1)".into(),
            text: "bad one".into(),
        }]),
        para_in(vec![Inline::Link {
            target: "https://example.org/has space".into(),
            text: "bad two".into(),
        }]),
        para_in(vec![Inline::Link {
            target: "https://example.org/fine".into(),
            text: "good".into(),
        }]),
    ]);
    let tree = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    let bad = bad_links(&tree);
    assert_eq!(bad.len(), 2);
    assert_eq!(bad[0].target, "javascript:alert(1)");
    assert_eq!(bad[0].node.path(), &[0, 0, 0, 0]);
    assert_eq!(bad[0].page, 0);
    assert_eq!(bad[0].reason, "URI scheme not allowed");
    assert_eq!(bad[1].target, "https://example.org/has space");
    let r = render_tree(&tree, &d.meta, &Fonts::std14(), &RenderOptions::default());
    assert!(matches!(r, Err(FolioError::Invalid(_))));
}

fn pdf_rect(a: &str) -> [f64; 4] {
    let at = a.find("/Rect [").unwrap() + 7;
    let v: Vec<f64> = a[at..a[at..].find(']').unwrap() + at]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    [v[0], v[1], v[2], v[3]]
}

// REQ: FOLIO-REQ-064
#[test]
fn distinct_bold_file_with_contents_and_links_puts_every_position_where_layout_did() {
    let (Some(reg), Some(bold)) = (liberation(), font_file(LIBERATION_BOLD)) else {
        return;
    };
    let limits = Limits::default();
    let fonts = Fonts::new(
        Face::TrueType(Font::parse(&reg, &limits).unwrap()),
        Face::TrueType(Font::parse(&bold, &limits).unwrap()),
    );
    let mut blocks = vec![Block::Field(FieldKind::Toc), heading(1, "Gr\u{fc}\u{df}e")];
    for i in 0..6 {
        blocks.push(heading(2, &format!("Abschnitt {i}")));
        blocks.push(link_para(vec![
            Inline::Anchor(format!("a{i}")),
            Inline::Text(format!("Text {i} caf\u{e9} ")),
            Inline::Link {
                target: format!("#a{}", (i + 1) % 6),
                text: "weiter".into(),
            },
            Inline::Text(" und ".into()),
            Inline::Link {
                target: "https://example.org/z".into(),
                text: "extern".into(),
            },
        ]));
        blocks.push(para(&words(i, 120)));
    }
    let d = doc(blocks);
    let tree = layout(&d, &fonts, &Theme::standard(), &Options::default()).unwrap();
    let bytes = render_pdf(&d, &fonts, &Theme::standard(), &RenderOptions::default()).unwrap();
    let pdf = Pdf::parse(bytes);
    let all: String = (1..pdf.objs.len()).map(|i| pdf.dict(i)).collect();
    assert_eq!(
        all.matches("/FontFile2").count(),
        2,
        "regular and bold are different files"
    );
    // Named destinations: page object and position equal the tree's.
    let pages = pdf.page_ids();
    let names = pdf.names();
    assert_eq!(names.len(), tree.dests.len());
    for dst in &tree.dests {
        let got = &names[&dst.name];
        assert_eq!(got.page, pages[dst.page], "{}", dst.name);
        assert!(
            near(got.x, pt_f(dst.x)) && near(got.y, pt_f(dst.y)),
            "{}: {got:?}",
            dst.name
        );
    }
    // Bookmarks: title, page and position equal the headings'.
    let hs = headings(&tree);
    let mut flat = Vec::new();
    fn walk<'a>(o: &'a [Outline], out: &mut Vec<&'a Outline>) {
        for b in o {
            out.push(b);
            walk(&b.children, out);
        }
    }
    let outline = pdf.outline();
    walk(&outline, &mut flat);
    assert_eq!(flat.len(), hs.len());
    for (b, h) in flat.iter().zip(&hs) {
        assert_eq!(b.title, h.title);
        assert_eq!(b.page, pages[h.page]);
        assert!(
            near(b.x, pt_f(h.x)) && near(b.y, pt_f(h.y)),
            "{}: ({}, {})",
            h.title,
            b.x,
            b.y
        );
    }
    // Annotations: every link's rectangle, in order, on every page.
    let mut checked = 0;
    for (pi, page) in tree.pages.iter().enumerate() {
        let annots = pdf.annots(pages[pi]);
        assert_eq!(annots.len(), page.links.len(), "page {pi}");
        for (a, l) in annots.iter().zip(&page.links) {
            let r = pdf_rect(a);
            let want = [
                pt_f(l.rect.x),
                pt_f(l.rect.y),
                pt_f(l.rect.x) + pt_f(l.rect.w),
                pt_f(l.rect.y) + pt_f(l.rect.h),
            ];
            assert!(
                r.iter().zip(&want).all(|(a, b)| near(*a, *b)),
                "{r:?} vs {want:?}"
            );
            checked += 1;
        }
    }
    assert!(checked > 12);
    // The text still extracts exactly on the first page.
    let t = pdf.page_text(pages[0]);
    assert!(t.iter().any(|s| s == "Gr\u{fc}\u{df}e"));
    let _ = ItemKind::Rect;
}
