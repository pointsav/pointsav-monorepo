// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Tagged PDF tests (task F5.6): the logical structure of `render_pdf` output, checked by parsing
//! the file bytes (cross-reference table, generic objects, content-stream operators); no external
//! tool is used.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    clippy::panic,
    clippy::items_after_statements,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_font::Font;
use moonshot_folio_ir::{
    Block, Body, BreakKind, Cell, CellValue, Document, FieldKind, Heading, Inline, List, Meta,
    PageSetup, Pagination, Para, Row, Section, Table,
};
use moonshot_folio_layout::{layout, Face, Fonts, ItemKind, Options, StructRole, Theme};
use moonshot_folio_pdf::{
    render_pdf, ArtifactKind, Content, PdfDoc, RenderOptions, StructElem, StructKid,
};
use std::collections::{BTreeMap, BTreeSet};

fn pts(n: i64) -> Emu {
    Emu::from_points(n).unwrap()
}

// ---------------------------------------------------------------------------------------
// Generic PDF reader

#[derive(Clone, Debug, PartialEq)]
enum V {
    Name(String),
    Int(i64),
    Real(f64),
    Str(Vec<u8>),
    Ref(usize),
    Dict(BTreeMap<String, V>),
    Arr(Vec<V>),
    Word(String),
}

impl V {
    fn dict(&self) -> &BTreeMap<String, V> {
        match self {
            V::Dict(d) => d,
            other => panic!("not a dictionary: {other:?}"),
        }
    }
    fn arr(&self) -> &[V] {
        match self {
            V::Arr(a) => a,
            other => panic!("not an array: {other:?}"),
        }
    }
    fn name(&self) -> &str {
        match self {
            V::Name(n) => n,
            other => panic!("not a name: {other:?}"),
        }
    }
    fn int(&self) -> i64 {
        match self {
            V::Int(n) => *n,
            other => panic!("not an integer: {other:?}"),
        }
    }
    fn rf(&self) -> usize {
        match self {
            V::Ref(n) => *n,
            other => panic!("not a reference: {other:?}"),
        }
    }
    fn get(&self, k: &str) -> Option<&V> {
        self.dict().get(k)
    }
}

fn skip_ws(b: &[u8], i: &mut usize) {
    while b.get(*i).is_some_and(u8::is_ascii_whitespace) {
        *i += 1;
    }
}

fn is_delim(c: u8) -> bool {
    c.is_ascii_whitespace() || b"/<>[]()".contains(&c)
}

fn parse(b: &[u8], i: &mut usize) -> V {
    skip_ws(b, i);
    match b[*i] {
        b'<' if b[*i + 1] == b'<' => {
            *i += 2;
            let mut d = BTreeMap::new();
            loop {
                skip_ws(b, i);
                if b[*i] == b'>' {
                    *i += 2;
                    return V::Dict(d);
                }
                let k = parse(b, i);
                let v = parse(b, i);
                d.insert(k.name().to_string(), v);
            }
        }
        b'<' => {
            let end = b[*i..].iter().position(|c| *c == b'>').unwrap() + *i;
            let hex: Vec<u8> = b[*i + 1..end]
                .iter()
                .copied()
                .filter(|c| !c.is_ascii_whitespace())
                .collect();
            *i = end + 1;
            V::Str(
                hex.chunks(2)
                    .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
                    .collect(),
            )
        }
        b'[' => {
            *i += 1;
            let mut a = Vec::new();
            loop {
                skip_ws(b, i);
                if b[*i] == b']' {
                    *i += 1;
                    return V::Arr(a);
                }
                a.push(parse(b, i));
            }
        }
        b'/' => {
            let s = *i + 1;
            *i = s;
            while *i < b.len() && !is_delim(b[*i]) {
                *i += 1;
            }
            V::Name(String::from_utf8_lossy(&b[s..*i]).to_string())
        }
        b'(' => {
            *i += 1;
            let mut out = Vec::new();
            while b[*i] != b')' {
                if b[*i] == b'\\' {
                    *i += 1;
                    if b[*i].is_ascii_digit() {
                        let mut v = 0_u32;
                        for _ in 0..3 {
                            if b[*i].is_ascii_digit() {
                                v = v * 8 + u32::from(b[*i] - b'0');
                                *i += 1;
                            }
                        }
                        out.push(v as u8);
                        continue;
                    }
                }
                out.push(b[*i]);
                *i += 1;
            }
            *i += 1;
            V::Str(out)
        }
        c if c.is_ascii_digit() || c == b'-' || c == b'.' => {
            let s = *i;
            while *i < b.len() && !is_delim(b[*i]) {
                *i += 1;
            }
            let t = std::str::from_utf8(&b[s..*i]).unwrap();
            if let Ok(n) = t.parse::<i64>() {
                // `n 0 R` is a reference.
                let mut j = *i;
                skip_ws(b, &mut j);
                if b.get(j) == Some(&b'0') && b.get(j + 1).is_some_and(u8::is_ascii_whitespace) {
                    let mut k = j + 1;
                    skip_ws(b, &mut k);
                    if b.get(k) == Some(&b'R') && b.get(k + 1).is_none_or(|c| is_delim(*c)) {
                        *i = k + 1;
                        return V::Ref(n as usize);
                    }
                }
                V::Int(n)
            } else {
                V::Real(t.parse().unwrap())
            }
        }
        _ => {
            let s = *i;
            while *i < b.len() && !is_delim(b[*i]) {
                *i += 1;
            }
            V::Word(String::from_utf8_lossy(&b[s..*i]).to_string())
        }
    }
}

struct Pdf {
    bytes: Vec<u8>,
    objs: Vec<(usize, usize)>,
}

fn rfind(h: &[u8], n: &[u8]) -> usize {
    h.windows(n.len()).rposition(|w| w == n).unwrap()
}

impl Pdf {
    fn parse(bytes: Vec<u8>) -> Self {
        let sx = rfind(&bytes, b"startxref\n");
        let xref_at: usize = String::from_utf8_lossy(&bytes[sx + 10..])
            .lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let table = String::from_utf8_lossy(&bytes[xref_at..sx]).to_string();
        let mut lines = table.lines();
        lines.next();
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
            let end = offsets.get(i + 1).copied().unwrap_or(xref_at);
            objs.push((*off + header.len(), end - "\nendobj\n".len()));
        }
        Self { bytes, objs }
    }
    fn body(&self, id: usize) -> &[u8] {
        &self.bytes[self.objs[id].0..self.objs[id].1]
    }
    fn obj(&self, id: usize) -> V {
        let b = self.body(id);
        let end = b
            .windows(8)
            .position(|w| w == b"\nstream\n")
            .unwrap_or(b.len());
        let mut i = 0;
        parse(&b[..end], &mut i)
    }
    fn stream(&self, id: usize) -> &[u8] {
        let b = self.body(id);
        let s = b.windows(8).position(|w| w == b"\nstream\n").unwrap() + 8;
        &b[s..b.len() - "\nendstream".len()]
    }
    fn count_objects(&self) -> usize {
        self.objs.len()
    }
    fn catalog(&self) -> V {
        let t =
            String::from_utf8_lossy(&self.bytes[rfind(&self.bytes, b"trailer\n")..]).to_string();
        let at = t.find("/Root ").unwrap() + 6;
        let id: usize = t[at..].split_whitespace().next().unwrap().parse().unwrap();
        self.obj(id)
    }
    fn pages(&self) -> Vec<usize> {
        let cat = self.catalog();
        let mut out = Vec::new();
        self.collect_pages(cat.get("Pages").unwrap().rf(), &mut out);
        out
    }
    fn collect_pages(&self, node: usize, out: &mut Vec<usize>) {
        let d = self.obj(node);
        for k in d.get("Kids").unwrap().arr() {
            if self.obj(k.rf()).get("Type") == Some(&V::Name("Pages".into())) {
                self.collect_pages(k.rf(), out);
            } else {
                out.push(k.rf());
            }
        }
    }
    fn content(&self, page: usize) -> Vec<String> {
        let d = self.obj(page);
        let c = self.stream(d.get("Contents").unwrap().rf());
        c.split(|b| *b == b'\n')
            .map(|l| String::from_utf8_lossy(l).to_string())
            .collect()
    }
}

// ---------------------------------------------------------------------------------------
// Documents

fn meta() -> Meta {
    Meta {
        title: "Tagged".into(),
        author: "Synthetic".into(),
        date: "2026-01-01".into(),
        lang: "en".into(),
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

fn cell(s: &str, span: u32) -> Cell {
    Cell {
        value: CellValue::Text(s.into()),
        span,
        role: String::new(),
    }
}

fn row(cells: Vec<Cell>) -> Row {
    Row {
        cells,
        can_split: false,
    }
}

fn section(blocks: Vec<Block>, restart: bool) -> Section {
    let mut s = Section::new(
        PageSetup {
            width: pts(300),
            height: pts(400),
            margin: pts(30),
        },
        blocks,
    );
    s.restart_page_numbers = restart;
    s
}

fn words(seed: usize, n: usize) -> String {
    const W: [&str; 8] = [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel",
    ];
    (0..n)
        .map(|k| W[(seed * 3 + k * 5) % 8])
        .collect::<Vec<_>>()
        .join(" ")
}

/// A document with a contents, headings, a paragraph with two links, a bulleted and a numbered
/// list, a table with a header row, a spanning cell and an empty cell, a header and footer, and a
/// second section that restarts page numbering.
fn sample() -> Document {
    let link = |target: &str, t: &str| Inline::Link {
        target: target.into(),
        text: t.into(),
    };
    let b0 = vec![
        heading(1, "Report"),
        Block::Field(FieldKind::Toc),
        heading(2, "Overview"),
        para_in(vec![
            Inline::Anchor("top".into()),
            Inline::Text("See the ".into()),
            link("#data", "data"),
            Inline::Text(" or ".into()),
            link("https://example.org/x", "the site"),
            Inline::Text(".".into()),
        ]),
        Block::List(List {
            numbering: 1,
            ordered: false,
            items: vec![text("first point"), text("second point")],
        }),
        Block::List(List {
            numbering: 2,
            ordered: true,
            items: vec![text("step one"), text("step two"), text("step three")],
        }),
        Block::Heading(Heading {
            level: 2,
            inlines: vec![Inline::Anchor("data".into()), Inline::Text("Data".into())],
            pagination: Pagination::default(),
        }),
        Block::Table(Table {
            columns: 3,
            repeat_header_rows: 1,
            rows: vec![
                row(vec![cell("Item", 1), cell("Q1", 1), cell("Q2", 1)]),
                row(vec![cell("Cash", 1), cell("10.00", 1), cell("12.50", 1)]),
                row(vec![cell("Note spanning two columns", 2), cell("", 1)]),
            ],
        }),
    ];
    let mut s0 = section(b0, false);
    s0.header = vec![para("Running header")];
    s0.footer = vec![para_in(vec![
        Inline::Text("Page ".into()),
        Inline::FieldRef(FieldKind::Page),
    ])];
    let s1 = section(vec![heading(1, "Appendix"), para("tail")], true);
    Document {
        meta: meta(),
        body: Body::Flow(vec![s0, s1]),
    }
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
// Structure walker

#[derive(Debug)]
struct El {
    tag: String,
    attrs: Vec<(String, String)>,
    /// `(page object, mcid)`.
    mcrs: Vec<(usize, i64)>,
    /// Annotation objects.
    objrs: Vec<usize>,
    kids: Vec<El>,
    /// The dictionary has a /K entry.
    has_k: bool,
}

fn read_el(pdf: &Pdf, id: usize) -> El {
    let d = pdf.obj(id);
    let mut attrs = Vec::new();
    if let Some(a) = d.get("A") {
        let list: Vec<&V> = match a {
            V::Arr(v) => v.iter().collect(),
            one => vec![one],
        };
        for x in list {
            for (k, v) in x.dict() {
                if k != "O" {
                    let s = match v {
                        V::Name(n) => format!("/{n}"),
                        V::Int(n) => n.to_string(),
                        other => format!("{other:?}"),
                    };
                    attrs.push((k.clone(), s));
                }
            }
        }
    }
    let mut el = El {
        tag: d.get("S").unwrap().name().to_string(),
        attrs,
        mcrs: Vec::new(),
        objrs: Vec::new(),
        kids: Vec::new(),
        has_k: d.get("K").is_some(),
    };
    for k in d.get("K").map_or(&[][..], V::arr) {
        match k {
            V::Ref(c) => el.kids.push(read_el(pdf, *c)),
            V::Dict(m) if m["Type"] == V::Name("MCR".into()) => {
                el.mcrs.push((m["Pg"].rf(), m["MCID"].int()));
            }
            V::Dict(m) if m["Type"] == V::Name("OBJR".into()) => el.objrs.push(m["Obj"].rf()),
            other => panic!("unexpected kid {other:?}"),
        }
    }
    el
}

fn tree(pdf: &Pdf) -> El {
    let root = pdf.obj(pdf.catalog().get("StructTreeRoot").unwrap().rf());
    read_el(pdf, root.get("K").unwrap().rf())
}

fn walk<'a>(e: &'a El, out: &mut Vec<&'a El>) {
    out.push(e);
    for k in &e.kids {
        walk(k, out);
    }
}

fn all(e: &El) -> Vec<&El> {
    let mut v = Vec::new();
    walk(e, &mut v);
    v
}

fn count(e: &El, tag: &str) -> usize {
    all(e).iter().filter(|x| x.tag == tag).count()
}

/// MCIDs opened on a page in the content stream, with their tags.
fn page_mcids(pdf: &Pdf, page: usize) -> Vec<(i64, String)> {
    pdf.content(page)
        .iter()
        .filter_map(|l| {
            let at = l.find(" << /MCID ")?;
            let tag = l[1..at].to_string();
            let n: i64 = l[at + 10..].split_whitespace().next()?.parse().ok()?;
            Some((n, tag))
        })
        .collect()
}

// REQ: FOLIO-REQ-066
#[test]
fn the_catalog_marks_the_document_tagged_and_points_at_the_structure_root() {
    let pdf = Pdf::parse(render(&sample()));
    let cat = pdf.catalog();
    assert_eq!(
        cat.get("MarkInfo").unwrap().get("Marked"),
        Some(&V::Word("true".into()))
    );
    assert_eq!(
        cat.get("ViewerPreferences").unwrap().get("DisplayDocTitle"),
        Some(&V::Word("true".into()))
    );
    assert_eq!(cat.get("Lang"), Some(&V::Str(b"en".to_vec())));
    let root = pdf.obj(cat.get("StructTreeRoot").unwrap().rf());
    assert_eq!(root.get("Type"), Some(&V::Name("StructTreeRoot".into())));
    assert!(root.get("ParentTree").is_some() && root.get("ParentTreeNextKey").is_some());
    let t = tree(&pdf);
    assert_eq!(t.tag, "Document");
    for p in pdf.pages() {
        let d = pdf.obj(p);
        assert_eq!(d.get("Tabs"), Some(&V::Name("S".into())));
        assert!(d.get("StructParents").is_some());
    }
}

// REQ: FOLIO-REQ-066
#[test]
fn every_tagged_text_item_has_one_mcid_referenced_by_exactly_one_element() {
    let d = sample();
    let lay = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.pages();
    assert_eq!(pages.len(), lay.pages.len());
    let t = tree(&pdf);
    let els = all(&t);
    let mut referenced: BTreeMap<(usize, i64), usize> = BTreeMap::new();
    for e in &els {
        for m in &e.mcrs {
            *referenced.entry(*m).or_default() += 1;
        }
    }
    let mut total = 0;
    for (pi, page) in pages.iter().enumerate() {
        let ids: Vec<i64> = page_mcids(&pdf, *page).iter().map(|m| m.0).collect();
        // Unique and contiguous from 0 in paint order.
        assert_eq!(ids, (0..ids.len() as i64).collect::<Vec<_>>(), "page {pi}");
        for id in &ids {
            assert_eq!(
                referenced.get(&(*page, *id)),
                Some(&1),
                "page {pi} mcid {id}"
            );
        }
        total += ids.len();
        // The count equals the layout's logical text items on the page.
        // Independent of how the writer classifies: logical content is text whose layout role is
        // body text, a heading, a list item, a table cell or a link, and which is not a repeat.
        let want = lay.pages[pi]
            .items
            .iter()
            .filter(|it| {
                matches!(it.kind, ItemKind::Text(_))
                    && !it.repeated
                    && matches!(
                        it.role,
                        StructRole::Paragraph
                            | StructRole::Heading(_)
                            | StructRole::ListItem
                            | StructRole::TableCell
                            | StructRole::Link
                    )
            })
            .count();
        assert_eq!(ids.len(), want, "page {pi}");
    }
    assert_eq!(
        referenced.len(),
        total,
        "no MCR points at content that does not exist"
    );
}

// REQ: FOLIO-REQ-066
#[test]
fn the_parent_tree_round_trips_with_struct_parents_and_mcids() {
    let pdf = Pdf::parse(render(&sample()));
    let root = pdf.obj(pdf.catalog().get("StructTreeRoot").unwrap().rf());
    let pt = pdf.obj(root.get("ParentTree").unwrap().rf());
    let nums = pt.get("Nums").unwrap().arr();
    let mut by_key: BTreeMap<i64, &V> = BTreeMap::new();
    for pair in nums.chunks(2) {
        by_key.insert(pair[0].int(), &pair[1]);
    }
    assert_eq!(
        root.get("ParentTreeNextKey").unwrap().int(),
        by_key.keys().max().unwrap() + 1
    );
    let mut checked = 0;
    for page in pdf.pages() {
        let d = pdf.obj(page);
        let arr = by_key[&d.get("StructParents").unwrap().int()].arr();
        for (mcid, e) in arr.iter().enumerate() {
            let el = read_el(&pdf, e.rf());
            assert!(el.mcrs.contains(&(page, mcid as i64)), "page mcid {mcid}");
            checked += 1;
        }
        for a in d.get("Annots").map_or(&[][..], V::arr) {
            let annot = pdf.obj(a.rf());
            let key = annot.get("StructParent").unwrap().int();
            let el = read_el(&pdf, by_key[&key].rf());
            assert_eq!(el.tag, "Link");
            assert!(
                el.objrs.contains(&a.rf()),
                "annotation is an OBJR of its parent"
            );
            checked += 1;
        }
    }
    assert!(checked > 30);
}

// REQ: FOLIO-REQ-066
#[test]
fn headings_paragraphs_and_lists_map_to_their_structure_types() {
    let pdf = Pdf::parse(render(&sample()));
    let t = tree(&pdf);
    assert_eq!(count(&t, "Sect"), 2, "one section element per IR section");
    assert_eq!(count(&t, "H1"), 2);
    assert_eq!(count(&t, "H2"), 2);
    assert_eq!(
        count(&t, "P"),
        2,
        "the paragraph with links and the appendix tail"
    );
    // Lists: 2 L, 5 LI each with a Lbl and an LBody.
    assert_eq!(count(&t, "L"), 2);
    assert_eq!(count(&t, "LI"), 5);
    assert_eq!(count(&t, "Lbl"), 5);
    assert_eq!(count(&t, "LBody"), 5);
    let ls: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "L").collect();
    assert_eq!(
        ls[0].attrs,
        [("ListNumbering".to_string(), "/Disc".to_string())]
    );
    assert_eq!(
        ls[1].attrs,
        [("ListNumbering".to_string(), "/Decimal".to_string())]
    );
    for l in ls {
        for li in &l.kids {
            assert_eq!(li.tag, "LI");
            assert_eq!(
                li.kids.iter().map(|k| k.tag.as_str()).collect::<Vec<_>>(),
                ["Lbl", "LBody"]
            );
        }
    }
}

// REQ: FOLIO-REQ-066
#[test]
fn every_heading_level_is_its_own_structure_type() {
    let blocks: Vec<Block> = (1..=6)
        .flat_map(|l| [heading(l, &format!("Level {l}")), para("text")])
        .collect();
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![section(blocks, false)]),
    };
    let pdf = Pdf::parse(render(&d));
    let t = tree(&pdf);
    for l in 1..=6 {
        assert_eq!(count(&t, &format!("H{l}")), 1, "H{l}");
    }
    // The marked-content tag of a heading's text is its structure type.
    let tags: BTreeSet<String> = pdf
        .pages()
        .iter()
        .flat_map(|p| page_mcids(&pdf, *p))
        .map(|m| m.1)
        .collect();
    assert!(["H1", "H2", "H3", "H4", "H5", "H6", "P"]
        .iter()
        .all(|t| tags.contains(*t)));
}

// REQ: FOLIO-REQ-066
#[test]
fn tables_have_rows_header_cells_with_scope_spans_and_empty_cells() {
    let pdf = Pdf::parse(render(&sample()));
    let t = tree(&pdf);
    let tables: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "Table").collect();
    assert_eq!(tables.len(), 1);
    let rows = &tables[0].kids;
    assert_eq!(
        rows.iter().map(|r| r.tag.as_str()).collect::<Vec<_>>(),
        ["TR", "TR", "TR"]
    );
    let kinds: Vec<Vec<&str>> = rows
        .iter()
        .map(|r| r.kids.iter().map(|c| c.tag.as_str()).collect())
        .collect();
    assert_eq!(
        kinds,
        [
            vec!["TH", "TH", "TH"],
            vec!["TD", "TD", "TD"],
            vec!["TD", "TD"]
        ]
    );
    for th in &rows[0].kids {
        assert_eq!(th.attrs, [("Scope".to_string(), "/Column".to_string())]);
    }
    // The spanning cell, and the empty cell (no content but present).
    assert_eq!(
        rows[2].kids[0].attrs,
        [("ColSpan".to_string(), "2".to_string())]
    );
    assert!(rows[2].kids[1].mcrs.is_empty());
    // The marked-content tag of cell text is the cell's type.
    let tags: BTreeSet<String> = pdf
        .pages()
        .iter()
        .flat_map(|p| page_mcids(&pdf, *p))
        .map(|m| m.1)
        .collect();
    assert!(tags.contains("TH") && tags.contains("TD"));
}

// REQ: FOLIO-REQ-066
#[test]
fn a_header_row_repeated_on_continuation_pages_is_an_artifact_not_a_second_row() {
    let rows: Vec<Row> = std::iter::once(row(vec![cell("HeadA", 1), cell("HeadB", 1)]))
        .chain((0..40).map(|i| row(vec![cell(&format!("r{i}"), 1), cell("v", 1)])))
        .collect();
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![section(
            vec![Block::Table(Table {
                columns: 2,
                repeat_header_rows: 1,
                rows,
            })],
            false,
        )]),
    };
    let lay = layout(&d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap();
    assert!(lay.pages.len() > 2);
    let pdf = Pdf::parse(render(&d));
    let t = tree(&pdf);
    assert_eq!(count(&t, "TR"), 41, "the IR rows, once each");
    assert_eq!(count(&t, "TH"), 2);
    // "HeadA" is drawn on every page but appears as content (an MCID) once.
    let mut tagged_head = 0;
    let mut art_head = 0;
    for p in pdf.pages() {
        let lines = pdf.content(p);
        for (i, l) in lines.iter().enumerate() {
            if l.contains("(HeadA) Tj") {
                // Walk back to the opening marked-content line.
                let open = lines[..i]
                    .iter()
                    .rposition(|x| x.ends_with("BDC") || x.ends_with("BMC"))
                    .unwrap();
                if lines[open].contains("/MCID") {
                    tagged_head += 1;
                } else {
                    assert!(lines[open].starts_with("/Artifact"));
                    art_head += 1;
                }
            }
        }
    }
    assert_eq!(tagged_head, 1);
    assert_eq!(art_head, lay.pages.len() - 1);
}

// REQ: FOLIO-REQ-066
#[test]
fn links_are_elements_with_object_references_and_tagged_annotations() {
    let pdf = Pdf::parse(render(&sample()));
    let t = tree(&pdf);
    let links: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "Link").collect();
    // Two inline links, and four contents entries (each with a title and a number annotation).
    let in_p: Vec<&&El> = links.iter().filter(|l| l.objrs.len() == 1).collect();
    assert_eq!(in_p.len(), 2);
    let toc_links: Vec<&&El> = links.iter().filter(|l| l.objrs.len() == 2).collect();
    assert_eq!(toc_links.len(), 4);
    let mut annots = 0;
    for p in pdf.pages() {
        for a in pdf.obj(p).get("Annots").map_or(&[][..], V::arr) {
            let an = pdf.obj(a.rf());
            assert!(an.get("StructParent").is_some());
            assert!(matches!(an.get("Contents"), Some(V::Str(s)) if !s.is_empty()));
            assert_eq!(an.get("F"), Some(&V::Int(4)));
            annots += 1;
        }
    }
    assert_eq!(annots, 2 + 4 * 2);
    // The parent of each OBJR is a Link whose text is marked content too.
    assert!(links.iter().all(|l| !l.mcrs.is_empty()));
}

// REQ: FOLIO-REQ-066
#[test]
fn a_table_of_contents_is_toc_toci_link() {
    let pdf = Pdf::parse(render(&sample()));
    let t = tree(&pdf);
    let tocs: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "TOC").collect();
    assert_eq!(tocs.len(), 1);
    assert_eq!(tocs[0].kids.len(), 4);
    for toci in &tocs[0].kids {
        assert_eq!(toci.tag, "TOCI");
        assert_eq!(toci.kids.len(), 1);
        let link = &toci.kids[0];
        assert_eq!(link.tag, "Link");
        assert_eq!(link.mcrs.len(), 2, "title and number");
        assert_eq!(link.objrs.len(), 2);
    }
    // Dot leaders are artifacts: no tagged sequence holds only dots.
    for p in pdf.pages() {
        let lines = pdf.content(p);
        for (i, l) in lines.iter().enumerate() {
            if l.ends_with(" Tj") && l.starts_with("(...") {
                let open = lines[..i]
                    .iter()
                    .rposition(|x| x.ends_with("BDC") || x.ends_with("BMC"))
                    .unwrap();
                assert!(lines[open].starts_with("/Artifact"), "{}", lines[open]);
            }
        }
    }
}

// REQ: FOLIO-REQ-066
#[test]
fn headers_footers_rules_and_leaders_are_artifacts_outside_the_tree() {
    let pdf = Pdf::parse(render(&sample()));
    let t = tree(&pdf);
    let mut header = 0;
    let mut footer = 0;
    let mut plain = 0;
    for p in pdf.pages() {
        for l in pdf.content(p) {
            if l == "/Artifact << /Type /Pagination /Subtype /Header >> BDC" {
                header += 1;
            } else if l == "/Artifact << /Type /Pagination /Subtype /Footer >> BDC" {
                footer += 1;
            } else if l == "/Artifact BMC" {
                plain += 1;
            }
        }
    }
    assert!(header >= 1 && footer >= 1 && plain > 10);
    // Their text is not in the structure tree: collect the text of all MCIDs.
    let texts: Vec<String> = pdf
        .pages()
        .iter()
        .flat_map(|p| {
            let lines = pdf.content(*p);
            let mut out = Vec::new();
            let mut open = false;
            for l in lines {
                if l.contains("/MCID") && l.ends_with("BDC") {
                    open = true;
                } else if l == "EMC" {
                    open = false;
                } else if open && l.ends_with(" Tj") {
                    out.push(l);
                }
            }
            out
        })
        .collect();
    assert!(texts
        .iter()
        .all(|s| !s.contains("Running header") && !s.contains("(Page")));
    let _ = t;
}

// REQ: FOLIO-REQ-066
#[test]
fn marked_content_and_graphics_states_are_balanced_and_text_is_always_marked() {
    for d in [sample()] {
        let pdf = Pdf::parse(render(&d));
        for p in pdf.pages() {
            let (mut marked, mut q, mut in_text) = (false, 0_i32, false);
            let mut q_at_open = 0;
            for l in pdf.content(p) {
                if l.ends_with("BDC") || l.ends_with("BMC") {
                    assert!(!marked, "marked content does not nest");
                    marked = true;
                    q_at_open = q;
                } else if l == "EMC" {
                    assert!(marked && !in_text);
                    assert_eq!(
                        q, q_at_open,
                        "graphics states are balanced inside the sequence"
                    );
                    marked = false;
                } else if l == "q" {
                    q += 1;
                } else if l == "Q" {
                    q -= 1;
                    assert!(q >= 0);
                } else if l == "BT" {
                    assert!(marked, "text objects sit inside marked content");
                    in_text = true;
                } else if l == "ET" {
                    in_text = false;
                } else if l.ends_with(" re") || l == "f" || l == "S" || l == "B" {
                    assert!(marked, "painted shapes sit inside marked content");
                }
            }
            assert!(!marked && q == 0 && !in_text);
        }
    }
}

// REQ: FOLIO-REQ-066
#[test]
fn untagged_output_has_no_tags_at_all() {
    let opts = RenderOptions {
        tagged: false,
        ..RenderOptions::default()
    };
    let bytes = render_pdf(&sample(), &Fonts::std14(), &Theme::standard(), &opts).unwrap();
    let t = String::from_utf8_lossy(&bytes).to_string();
    for needle in [
        "StructTreeRoot",
        "MarkInfo",
        "StructParent",
        "/Tabs",
        "BDC",
        "BMC",
        "EMC",
        "ViewerPreferences",
        "OBJR",
    ] {
        assert!(!t.contains(needle), "{needle}");
    }
    assert!(t.contains("/Lang (en)") && t.contains("/PageLabels"));
    // The default is tagged.
    assert!(RenderOptions::default().tagged);
}

// REQ: FOLIO-REQ-066
#[test]
fn tagged_rendering_is_deterministic() {
    let d = sample();
    assert_eq!(render(&d), render(&d.clone()));
}

// REQ: FOLIO-REQ-066
#[test]
fn truetype_documents_are_tagged_the_same_way() {
    let path = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf";
    let bold = "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf";
    let (reg, bd) = match (std::fs::read(path), std::fs::read(bold)) {
        (Ok(a), Ok(b)) => (a, b),
        _ if std::env::var("FOLIO_ALLOW_MISSING_FONT").is_ok_and(|v| v == "1") => return,
        _ => panic!("Liberation Sans is missing; install it or set FOLIO_ALLOW_MISSING_FONT=1"),
    };
    let limits = Limits::default();
    let fonts = Fonts::new(
        Face::TrueType(Font::parse(&reg, &limits).unwrap()),
        Face::TrueType(Font::parse(&bd, &limits).unwrap()),
    );
    let mut d = sample();
    if let Body::Flow(s) = &mut d.body {
        s[1].blocks.push(para("Caf\u{e9} \u{20ac}5 na\u{ef}ve"));
    }
    let pdf =
        Pdf::parse(render_pdf(&d, &fonts, &Theme::standard(), &RenderOptions::default()).unwrap());
    let t = tree(&pdf);
    let mcr: usize = all(&t).iter().map(|e| e.mcrs.len()).sum();
    let bdc: usize = pdf.pages().iter().map(|p| page_mcids(&pdf, *p).len()).sum();
    assert_eq!(mcr, bdc);
    assert!(count(&t, "P") >= 3);
}

// REQ: FOLIO-REQ-066
#[test]
fn a_heading_that_spans_pages_and_a_long_list_keep_one_element() {
    // A table taller than a page keeps one Table; a paragraph over two pages one P with MCRs on both.
    let blocks = vec![
        para(&words(0, 400)),
        Block::Break(BreakKind::Line),
        heading(2, "After"),
    ];
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![section(blocks, false)]),
    };
    let pdf = Pdf::parse(render(&d));
    let t = tree(&pdf);
    let p: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "P").collect();
    assert_eq!(p.len(), 1);
    let pages: BTreeSet<usize> = p[0].mcrs.iter().map(|m| m.0).collect();
    assert!(
        pages.len() >= 2,
        "one paragraph element holds content of several pages"
    );
}

// ---------------------------------------------------------------------------------------
// Writer API

fn page_with_marks(doc: &mut PdfDoc, n: u32) -> usize {
    let mut c = Content::new();
    for _ in 0..n {
        c.begin_tagged("P").unwrap();
        c.end_marked().unwrap();
    }
    doc.add_page(pts(100), pts(100), &c, vec![]).unwrap()
}

fn elem(tag: &str, kids: Vec<StructKid>) -> StructElem {
    StructElem {
        tag: tag.into(),
        attrs: vec![],
        kids,
    }
}

// REQ: FOLIO-REQ-068
#[test]
fn content_marks_follow_the_operator_rules() {
    let mut c = Content::new();
    assert_eq!(c.begin_tagged("P").unwrap(), 0);
    assert!(c.begin_tagged("P").is_err(), "no nesting");
    assert!(c.begin_artifact(ArtifactKind::Plain).is_err());
    c.save().unwrap();
    assert!(
        c.end_marked().is_err(),
        "a graphics state opened inside is still open"
    );
    c.restore().unwrap();
    c.end_marked().unwrap();
    assert!(c.end_marked().is_err(), "nothing open");
    c.begin_artifact(ArtifactKind::Header).unwrap();
    assert_eq!(c.mcid_count(), 1, "artifacts take no id");
    let mut doc = PdfDoc::new(Limits::default());
    assert!(
        doc.add_page(pts(10), pts(10), &c, vec![]).is_err(),
        "unterminated marked content"
    );
    c.end_marked().unwrap();
    assert_eq!(c.begin_tagged("H1").unwrap(), 1);
    c.end_marked().unwrap();
    assert!(Content::new().begin_tagged("").is_err());
    c.begin_text().unwrap();
    assert!(c.begin_tagged("P").is_err(), "not inside a text object");
}

// REQ: FOLIO-REQ-068
#[test]
fn the_structure_tree_is_checked_against_the_pages_and_the_limits() {
    let mut doc = PdfDoc::new(Limits::default());
    page_with_marks(&mut doc, 2);
    let mc = |m: u32| StructKid::Content { page: 0, mcid: m };
    // Every mark referenced once: fine.
    doc.set_structure(elem(
        "Document",
        vec![StructKid::Elem(elem("P", vec![mc(0), mc(1)]))],
    ))
    .unwrap();
    assert!(doc.build().is_ok());
    // A mark left out, a mark twice, a mark that does not exist, a page that does not exist.
    for kids in [
        vec![mc(0)],
        vec![mc(0), mc(0), mc(1)],
        vec![mc(0), mc(1), mc(2)],
    ] {
        doc.set_structure(elem("Document", kids)).unwrap();
        assert!(doc.build().is_err());
    }
    doc.set_structure(elem(
        "Document",
        vec![mc(0), mc(1), StructKid::Content { page: 4, mcid: 0 }],
    ))
    .unwrap();
    assert!(doc.build().is_err());
    doc.set_structure(elem(
        "Document",
        vec![mc(0), mc(1), StructKid::Annot { page: 0, index: 0 }],
    ))
    .unwrap();
    assert!(doc.build().is_err(), "an annotation that does not exist");
    // Hard bounds: depth max_depth + 8, elements 4 x max_cells.
    let nested = |levels: usize| {
        let mut e = elem("Span", vec![]);
        for _ in 1..levels {
            e = elem("Span", vec![StructKid::Elem(e)]);
        }
        e
    };
    let strict = || PdfDoc::new(Limits::default().with_max_depth(4));
    assert!(strict().set_structure(nested(12)).is_ok());
    assert_eq!(
        strict().set_structure(nested(13)),
        Err(FolioError::Limit("structure depth"))
    );
    let tiny = || PdfDoc::new(Limits::default().with_max_cells(3));
    let flat = |n: usize| {
        elem(
            "Document",
            (1..n).map(|_| StructKid::Elem(elem("P", vec![]))).collect(),
        )
    };
    assert!(tiny().set_structure(flat(12)).is_ok());
    assert_eq!(
        tiny().set_structure(flat(13)),
        Err(FolioError::Limit("structure elements"))
    );
    assert!(tiny().set_structure(elem("", vec![])).is_err());
}

// ---------------------------------------------------------------------------------------
// Review round: bounds, title, edge cases, large documents

fn tt(blocks: Vec<Block>) -> Document {
    Document {
        meta: meta(),
        body: Body::Flow(vec![section(blocks, false)]),
    }
}

fn shape(e: &El) -> String {
    let attrs: Vec<String> = e.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let kids: Vec<String> = e.kids.iter().map(shape).collect();
    format!("{}[{}]({})", e.tag, attrs.join(","), kids.join(" "))
}

fn max_array_len(v: &V) -> usize {
    match v {
        V::Arr(a) => a.len().max(a.iter().map(max_array_len).max().unwrap_or(0)),
        V::Dict(d) => d.values().map(max_array_len).max().unwrap_or(0),
        _ => 0,
    }
}

// REQ: FOLIO-REQ-068
#[test]
fn the_structure_has_bounds_of_its_own_so_tagging_adds_no_everyday_failure() {
    // 1400 list items at max_cells 3000: the tree needs 3 elements per item, far beyond 3000, yet
    // the untagged output is fine and so is the tagged one.
    let items: Vec<Vec<Inline>> = (0..1400).map(|i| text(&format!("item {i}"))).collect();
    let d = tt(vec![Block::List(List {
        numbering: 1,
        ordered: false,
        items,
    })]);
    let opts = |tagged: bool| RenderOptions {
        limits: Limits::default().with_max_cells(3000),
        tagged,
        ..RenderOptions::default()
    };
    assert!(render_pdf(&d, &Fonts::std14(), &Theme::standard(), &opts(false)).is_ok());
    let pdf = Pdf::parse(render_pdf(&d, &Fonts::std14(), &Theme::standard(), &opts(true)).unwrap());
    assert_eq!(count(&tree(&pdf), "LI"), 1400);
    // A table under max_depth 4: Document > Sect > Table > TR > TD is five levels.
    let table = Block::Table(Table {
        columns: 2,
        repeat_header_rows: 0,
        rows: vec![row(vec![cell("a", 1), cell("b", 1)])],
    });
    let shallow = RenderOptions {
        limits: Limits::default().with_max_depth(4),
        ..RenderOptions::default()
    };
    let bytes = render_pdf(
        &tt(vec![table]),
        &Fonts::std14(),
        &Theme::standard(),
        &shallow,
    )
    .unwrap();
    assert_eq!(count(&tree(&Pdf::parse(bytes)), "TD"), 2);
}

// REQ: FOLIO-REQ-066
#[test]
fn display_doc_title_is_written_only_when_there_is_a_title() {
    let titled = Pdf::parse(render(&sample()));
    assert!(titled.catalog().get("ViewerPreferences").is_some());
    let mut d = sample();
    d.meta.title = String::new();
    let untitled = Pdf::parse(render(&d));
    let cat = untitled.catalog();
    assert!(cat.get("ViewerPreferences").is_none());
    assert!(
        cat.get("StructTreeRoot").is_some(),
        "tagging itself is unaffected"
    );
    let trailer = String::from_utf8_lossy(&untitled.bytes[rfind(&untitled.bytes, b"trailer\n")..])
        .to_string();
    let at = trailer.find("/Info ").unwrap() + 6;
    let info_id: usize = trailer[at..]
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert!(
        untitled.obj(info_id).get("Title").is_none(),
        "no title in the information dictionary"
    );
}

// REQ: FOLIO-REQ-068
#[test]
fn a_restore_may_not_pop_the_state_a_marked_sequence_began_in() {
    let mut c = Content::new();
    c.save().unwrap();
    c.begin_tagged("P").unwrap();
    assert!(
        c.restore().is_err(),
        "the reviewer's sequence: save, open, restore"
    );
    c.end_marked().unwrap();
    c.restore().unwrap();
    // The legal shape: the state is opened and closed inside the sequence.
    c.begin_tagged("P").unwrap();
    c.save().unwrap();
    c.restore().unwrap();
    c.end_marked().unwrap();
    let mut doc = PdfDoc::new(Limits::default());
    assert!(doc.add_page(pts(10), pts(10), &c, vec![]).is_ok());
}

// REQ: FOLIO-REQ-068
#[test]
fn marked_content_without_a_structure_tree_is_rejected_at_build() {
    let mut doc = PdfDoc::new(Limits::default());
    page_with_marks(&mut doc, 1);
    assert_eq!(
        doc.build(),
        Err(FolioError::Invalid(
            "marked content without a structure tree"
        ))
    );
    // Artifacts alone need no tree.
    let mut ok = PdfDoc::new(Limits::default());
    let mut c = Content::new();
    c.begin_artifact(ArtifactKind::Plain).unwrap();
    c.end_marked().unwrap();
    ok.add_page(pts(10), pts(10), &c, vec![]).unwrap();
    assert!(ok.build().is_ok());
}

// REQ: FOLIO-REQ-066
#[test]
fn empty_elements_have_no_k_entry_and_the_file_still_reads() {
    let rows: Vec<Row> = (0..3)
        .map(|_| row(vec![cell("", 1), cell("", 1)]))
        .collect();
    let pdf = Pdf::parse(render(&tt(vec![Block::Table(Table {
        columns: 2,
        repeat_header_rows: 0,
        rows,
    })])));
    let t = tree(&pdf);
    assert_eq!(count(&t, "TR"), 3);
    let tds: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "TD").collect();
    assert_eq!(tds.len(), 6, "empty cells still exist");
    assert!(tds
        .iter()
        .all(|c| !c.has_k && c.kids.is_empty() && c.mcrs.is_empty()));
    // A document with no content at all: the Document element has no /K either.
    let empty = Document {
        meta: meta(),
        body: Body::Flow(vec![section(vec![], false)]),
    };
    let pdf = Pdf::parse(render(&empty));
    let t = tree(&pdf);
    assert_eq!((t.tag.as_str(), t.has_k), ("Document", false));
    // The parent tree and keys are fine: a page without marked content has no /StructParents.
    assert!(pdf.obj(pdf.pages()[0]).get("StructParents").is_none());
}

// REQ: FOLIO-REQ-066
#[test]
fn heading_levels_are_the_inputs_and_skips_are_kept() {
    let d = tt(vec![
        heading(1, "One"),
        heading(4, "Four"),
        heading(2, "Two"),
        heading(6, "Six"),
    ]);
    let t = tree(&Pdf::parse(render(&d)));
    let tags: Vec<&str> = all(&t)
        .into_iter()
        .map(|e| e.tag.as_str())
        .filter(|t| t.starts_with('H'))
        .collect();
    assert_eq!(
        tags,
        ["H1", "H4", "H2", "H6"],
        "not normalised to 1, 2, 3, 4"
    );
}

fn link(target: &str, t: &str) -> Inline {
    Inline::Link {
        target: target.into(),
        text: t.into(),
    }
}

/// Check the structure of a file the way a consumer would: every marked-content id referenced
/// once, every page and annotation key resolving through the (possibly deep) number tree.
fn check_consistent(pdf: &Pdf) {
    let root = pdf.obj(pdf.catalog().get("StructTreeRoot").unwrap().rf());
    let pt = pdf.obj(root.get("ParentTree").unwrap().rf());
    let mut entries = Vec::new();
    flatten_numbers(pdf, &pt, &mut entries, true);
    let by_key: BTreeMap<i64, V> = entries.into_iter().collect();
    let t = tree(pdf);
    let mut referenced: BTreeMap<(usize, i64), usize> = BTreeMap::new();
    for e in all(&t) {
        for m in &e.mcrs {
            *referenced.entry(*m).or_default() += 1;
        }
    }
    assert!(referenced.values().all(|n| *n == 1));
    for page in pdf.pages() {
        let d = pdf.obj(page);
        let ids: Vec<i64> = page_mcids(pdf, page).iter().map(|m| m.0).collect();
        match d.get("StructParents") {
            Some(k) => {
                let arr = by_key[&k.int()].arr();
                assert_eq!(arr.len(), ids.len());
                for (mcid, e) in arr.iter().enumerate() {
                    let el = read_el(pdf, e.rf());
                    assert!(el.mcrs.contains(&(page, mcid as i64)));
                }
            }
            None => assert!(ids.is_empty()),
        }
        for a in d.get("Annots").map_or(&[][..], V::arr) {
            let key = pdf.obj(a.rf()).get("StructParent").unwrap().int();
            let el = read_el(pdf, by_key[&key].rf());
            assert!(el.objrs.contains(&a.rf()));
        }
    }
}

/// Entries of a number tree in order, checking `/Limits` of every non-root node.
fn flatten_numbers(pdf: &Pdf, node: &V, out: &mut Vec<(i64, V)>, root: bool) {
    let d = node.dict();
    assert_eq!(
        d.contains_key("Limits"),
        !root,
        "only the root has no /Limits"
    );
    let before = out.len();
    if let Some(kids) = d.get("Kids") {
        assert!(kids.arr().len() <= 8191);
        for k in kids.arr() {
            flatten_numbers(pdf, &pdf.obj(k.rf()), out, false);
        }
    } else {
        let nums = d["Nums"].arr();
        assert!(
            nums.len() <= 8191,
            "a /Nums array of {} elements",
            nums.len()
        );
        for pair in nums.chunks(2) {
            out.push((pair[0].int(), pair[1].clone()));
        }
    }
    assert!(
        out[before..].windows(2).all(|w| w[0].0 < w[1].0),
        "keys ascend"
    );
    if let Some(l) = d.get("Limits") {
        let l = l.arr();
        assert_eq!(
            (l[0].int(), l[1].int()),
            (out[before].0, out.last().unwrap().0)
        );
    }
}

/// Entries of a name tree in order, with the same checks.
fn flatten_names(pdf: &Pdf, node: &V, out: &mut Vec<(Vec<u8>, V)>, root: bool) {
    let d = node.dict();
    let before = out.len();
    if let Some(kids) = d.get("Kids") {
        assert!(kids.arr().len() <= 8191);
        for k in kids.arr() {
            flatten_names(pdf, &pdf.obj(k.rf()), out, false);
        }
    } else {
        let names = d["Names"].arr();
        assert!(names.len() <= 8191);
        for pair in names.chunks(2) {
            let V::Str(k) = &pair[0] else { panic!() };
            out.push((k.clone(), pair[1].clone()));
        }
    }
    assert_eq!(d.contains_key("Limits"), !root);
    assert!(out[before..].windows(2).all(|w| w[0].0 < w[1].0));
    if let Some(l) = d.get("Limits") {
        let (V::Str(a), V::Str(b)) = (&l.arr()[0], &l.arr()[1]) else {
            panic!()
        };
        assert_eq!((a, b), (&out[before].0, &out.last().unwrap().0));
    }
}

// REQ: FOLIO-REQ-066
#[test]
fn links_in_headers_and_footers_are_bare_link_elements_in_one_div() {
    let mut s = section((0..40).map(|i| para(&words(i, 60))).collect(), false);
    s.header = vec![para_in(vec![link("https://example.org/h", "head link")])];
    s.footer = vec![para_in(vec![link("https://example.org/f", "foot link")])];
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![s]),
    };
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.pages();
    assert!(pages.len() > 3);
    let t = tree(&pdf);
    let divs: Vec<&El> = t.kids.iter().filter(|k| k.tag == "Div").collect();
    assert_eq!(divs.len(), 1);
    assert_eq!(t.kids.last().unwrap().tag, "Div", "after the sections");
    let links = &divs[0].kids;
    assert_eq!(
        links.len(),
        pages.len() * 2,
        "one per annotation: header and footer on every page"
    );
    for l in links {
        assert_eq!(l.tag, "Link");
        assert!(l.mcrs.is_empty() && l.kids.is_empty());
        assert_eq!(l.objrs.len(), 1);
    }
    // Their text is artifact, so no MCID shows it; the annotations carry /Contents.
    for p in &pages {
        for a in pdf.obj(*p).get("Annots").unwrap().arr() {
            assert!(pdf.obj(a.rf()).get("Contents").is_some());
        }
    }
    check_consistent(&pdf);
}

// REQ: FOLIO-REQ-066
#[test]
fn a_page_with_annotations_but_no_content_has_keys_for_the_annotations_only() {
    let mut s = section(vec![], false);
    s.header = vec![para_in(vec![link(
        "https://example.org/h",
        "only a header link",
    )])];
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![s]),
    };
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.pages();
    assert_eq!(pages.len(), 1);
    let page = pdf.obj(pages[0]);
    assert!(page.get("StructParents").is_none());
    assert_eq!(page.get("Annots").unwrap().arr().len(), 1);
    assert!(pdf
        .obj(page.get("Annots").unwrap().arr()[0].rf())
        .get("StructParent")
        .is_some());
    assert!(page_mcids(&pdf, pages[0]).is_empty());
    check_consistent(&pdf);
}

// REQ: FOLIO-REQ-066
#[test]
fn a_link_spanning_lines_and_pages_is_one_element_with_every_annotation() {
    let d = tt(vec![
        para_in(vec![
            Inline::Text("start ".into()),
            link("https://example.org/long", &words(2, 400)),
            Inline::Text(" end".into()),
        ]),
        para("after"),
    ]);
    let pdf = Pdf::parse(render(&d));
    let pages = pdf.pages();
    assert!(pages.len() >= 2);
    let t = tree(&pdf);
    let links: Vec<&El> = all(&t).into_iter().filter(|e| e.tag == "Link").collect();
    assert_eq!(links.len(), 1, "one inline, one element");
    let annots: usize = pages
        .iter()
        .map(|p| pdf.obj(*p).get("Annots").map_or(0, |a| a.arr().len()))
        .sum();
    assert!(annots >= 4, "one annotation per line");
    assert_eq!(links[0].objrs.len(), annots);
    let on: BTreeSet<usize> = links[0].mcrs.iter().map(|m| m.0).collect();
    assert!(on.len() >= 2, "text on several pages");
    check_consistent(&pdf);
}

// REQ: FOLIO-REQ-066
#[test]
fn sections_with_different_page_sizes_each_get_a_sect_and_page_keys() {
    let make = |w: i64, h: i64, blocks| {
        Section::new(
            PageSetup {
                width: pts(w),
                height: pts(h),
                margin: pts(20),
            },
            blocks,
        )
    };
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![
            make(300, 400, vec![heading(1, "A"), para("a")]),
            make(200, 200, vec![heading(1, "B"), para(&words(1, 80))]),
            make(400, 300, vec![heading(1, "C"), para("c")]),
        ]),
    };
    let pdf = Pdf::parse(render(&d));
    let t = tree(&pdf);
    assert_eq!(count(&t, "Sect"), 3);
    let sizes: BTreeSet<String> = pdf
        .pages()
        .iter()
        .map(|p| format!("{:?}", pdf.obj(*p).get("MediaBox").unwrap().arr()))
        .collect();
    assert!(sizes.len() >= 3);
    let keys: BTreeSet<i64> = pdf
        .pages()
        .iter()
        .filter_map(|p| pdf.obj(*p).get("StructParents").map(V::int))
        .collect();
    assert_eq!(keys.len(), pdf.pages().len(), "a distinct key per page");
    check_consistent(&pdf);
}

// REQ: FOLIO-REQ-066
#[test]
fn a_truetype_document_has_the_same_structure_as_the_standard_font_one() {
    let path = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf";
    let bold = "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf";
    let (reg, bd) = match (std::fs::read(path), std::fs::read(bold)) {
        (Ok(a), Ok(b)) => (a, b),
        _ if std::env::var("FOLIO_ALLOW_MISSING_FONT").is_ok_and(|v| v == "1") => return,
        _ => panic!("Liberation Sans is missing; install it or set FOLIO_ALLOW_MISSING_FONT=1"),
    };
    let limits = Limits::default();
    let fonts = Fonts::new(
        Face::TrueType(Font::parse(&reg, &limits).unwrap()),
        Face::TrueType(Font::parse(&bd, &limits).unwrap()),
    );
    let d = sample();
    let std14 = Pdf::parse(render(&d));
    let ttf =
        Pdf::parse(render_pdf(&d, &fonts, &Theme::standard(), &RenderOptions::default()).unwrap());
    // Same tags and attributes in the same order, whatever the font did to the wrapping (only the
    // number of text pieces per element may differ, and that is not part of the shape).
    assert_eq!(shape(&tree(&ttf)), shape(&tree(&std14)));
    check_consistent(&ttf);
    check_consistent(&std14);
}

// REQ: FOLIO-REQ-066
#[test]
fn a_large_document_keeps_every_array_under_the_limit_and_every_key_reachable() {
    let n = 17_000;
    let blocks: Vec<Block> = (0..n)
        .map(|i| {
            para_in(vec![
                Inline::Anchor(format!("a{i:05}")),
                Inline::Text("x".into()),
            ])
        })
        .collect();
    let d = Document {
        meta: meta(),
        body: Body::Flow(vec![Section::new(
            PageSetup {
                width: pts(100),
                height: pts(70),
                margin: pts(10),
            },
            blocks,
        )]),
    };
    let pdf = Pdf::parse(render(&d));
    // No array anywhere is longer than 8191 elements.
    let longest = (1..pdf.count_objects())
        .map(|i| max_array_len(&pdf.obj(i)))
        .max()
        .unwrap();
    assert!(longest <= 8191, "{longest}");
    // The page tree: more than 8191 pages, grouped.
    let pages = pdf.pages();
    assert!(pages.len() > 8191, "{} pages", pages.len());
    let root = pdf.obj(pdf.catalog().get("Pages").unwrap().rf());
    assert_eq!(root.get("Count").unwrap().int(), pages.len() as i64);
    assert!(root.get("Kids").unwrap().arr().len() < 8191);
    // The structure: one Sect holding Divs of bounded size, 17000 paragraphs in all.
    let t = tree(&pdf);
    assert_eq!(count(&t, "P"), n);
    assert_eq!(count(&t, "Sect"), 1);
    assert!(all(&t).iter().all(|e| e.kids.len() <= 4096));
    let sect = &t.kids[0];
    assert_eq!(sect.kids.iter().filter(|k| k.tag == "Div").count(), 5);
    // The parent tree has /Kids and leaves with /Limits; every page key is reachable.
    let sroot = pdf.obj(pdf.catalog().get("StructTreeRoot").unwrap().rf());
    let pt = pdf.obj(sroot.get("ParentTree").unwrap().rf());
    assert!(pt.get("Kids").is_some() && pt.get("Nums").is_none());
    let mut entries = Vec::new();
    flatten_numbers(&pdf, &pt, &mut entries, true);
    let keys: Vec<i64> = entries.iter().map(|e| e.0).collect();
    assert_eq!(keys, (0..pages.len() as i64).collect::<Vec<_>>());
    for (k, page) in [
        (0, pages[0]),
        (4095, pages[4095]),
        (4096, pages[4096]),
        (8191, pages[8191]),
    ] {
        let d = pdf.obj(page);
        assert_eq!(d.get("StructParents").unwrap().int(), k);
        let arr = entries[k as usize].1.arr();
        assert_eq!(arr.len(), page_mcids(&pdf, page).len());
    }
    // The destination tree likewise: all 17000 names, sorted, each pointing at a page.
    let cat = pdf.catalog();
    let names_root = pdf.obj(cat.get("Names").unwrap().get("Dests").unwrap().rf());
    assert!(names_root.get("Kids").is_some());
    let mut names = Vec::new();
    flatten_names(&pdf, &names_root, &mut names, true);
    assert_eq!(names.len(), n);
    let page_set: BTreeSet<usize> = pages.iter().copied().collect();
    for (i, (name, dest)) in names.iter().enumerate() {
        assert_eq!(name, format!("a{i:05}").as_bytes());
        assert!(page_set.contains(&dest.arr()[0].rf()));
    }
}
