// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Builders, checkers and a seeded document generator shared by the layout tests.

#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names
)]

use moonshot_folio_core::{Emu, Result};
use moonshot_folio_ir::{
    Block, Body, BreakKind, Cell, CellValue, Document, FieldKind, Heading, Inline, List, Meta,
    PageSetup, Pagination, Para, Row, Section, Table,
};
use moonshot_folio_layout::{layout, Fonts, ItemKind, Options, PageTree, Theme};

pub(crate) fn pts(n: i64) -> Emu {
    Emu::from_points(n).unwrap()
}

pub(crate) fn meta() -> Meta {
    Meta {
        title: "Synthetic".into(),
        author: "Synthetic".into(),
        date: "2026-01-01".into(),
        lang: "en".into(),
    }
}

/// A page of `w` by `h` points with an `m` point margin.
pub(crate) fn setup(w: i64, h: i64, m: i64) -> PageSetup {
    PageSetup {
        width: pts(w),
        height: pts(h),
        margin: pts(m),
    }
}

pub(crate) fn section(page: PageSetup, blocks: Vec<Block>) -> Section {
    Section::new(page, blocks)
}

pub(crate) fn doc(sections: Vec<Section>) -> Document {
    Document {
        meta: meta(),
        body: Body::Flow(sections),
    }
}

/// One section on a small page: 200 by 300 points, 20 point margin.
pub(crate) fn small(blocks: Vec<Block>) -> Document {
    doc(vec![section(setup(200, 300, 20), blocks)])
}

pub(crate) fn text(s: &str) -> Vec<Inline> {
    vec![Inline::Text(s.into())]
}

pub(crate) fn para(s: &str) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines: text(s),
        pagination: Pagination::default(),
    })
}

pub(crate) fn para_with(s: &str, pagination: Pagination) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines: text(s),
        pagination,
    })
}

pub(crate) fn para_in(inlines: Vec<Inline>) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines,
        pagination: Pagination::default(),
    })
}

pub(crate) fn heading(level: u8, s: &str) -> Block {
    Block::Heading(Heading {
        level,
        inlines: text(s),
        pagination: Pagination::default(),
    })
}

pub(crate) fn list(ordered: bool, items: &[&str]) -> Block {
    Block::List(List {
        numbering: 1,
        ordered,
        items: items.iter().map(|s| text(s)).collect(),
    })
}

pub(crate) fn cell(s: &str) -> Cell {
    Cell {
        value: CellValue::Text(s.into()),
        span: 1,
        role: String::new(),
    }
}

pub(crate) fn num(s: &str) -> Cell {
    Cell {
        value: CellValue::Number(s.into()),
        span: 1,
        role: String::new(),
    }
}

pub(crate) fn span(s: &str, n: u32) -> Cell {
    Cell { span: n, ..cell(s) }
}

pub(crate) fn row(cells: Vec<Cell>) -> Row {
    Row {
        cells,
        can_split: false,
    }
}

pub(crate) fn table(columns: u32, header: u32, rows: Vec<Row>) -> Block {
    Block::Table(Table {
        columns,
        repeat_header_rows: header,
        rows,
    })
}

/// `n` plain paragraphs of `words` words each.
pub(crate) fn paras(n: usize, words: usize) -> Vec<Block> {
    (0..n)
        .map(|i| para(&words_of(i, words)))
        .collect::<Vec<_>>()
}

const WORDS: [&str; 12] = [
    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliet",
    "kilo", "lima",
];

pub(crate) fn words_of(seed: usize, n: usize) -> String {
    (0..n)
        .map(|k| WORDS[(seed * 7 + k * 5) % WORDS.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn lay(d: &Document) -> PageTree {
    layout(d, &Fonts::std14(), &Theme::standard(), &Options::default()).unwrap()
}

pub(crate) fn try_lay(d: &Document) -> Result<PageTree> {
    layout(d, &Fonts::std14(), &Theme::standard(), &Options::default())
}

/// All text of a tree in item order, one entry per text item.
pub(crate) fn texts(t: &PageTree) -> Vec<String> {
    t.pages
        .iter()
        .flat_map(|p| &p.items)
        .filter_map(|i| match &i.kind {
            ItemKind::Text(x) => Some(x.text.clone()),
            ItemKind::Rect(_) => None,
        })
        .collect()
}

/// Text items of one page as `(x, baseline y, text)`.
pub(crate) fn page_texts(t: &PageTree, page: usize) -> Vec<(i64, i64, String)> {
    t.pages[page]
        .items
        .iter()
        .filter_map(|i| match &i.kind {
            ItemKind::Text(x) => Some((i.x.raw(), i.y.raw(), x.text.clone())),
            ItemKind::Rect(_) => None,
        })
        .collect()
}

/// Check the geometric invariants of a tree: every item and link inside the content box, and
/// no two text items overlapping both horizontally and vertically.
pub(crate) fn check_invariants(t: &PageTree) {
    for (pi, p) in t.pages.iter().enumerate() {
        let b = p.content_box().unwrap();
        let (l, r) = (b.x.raw(), b.x.raw() + b.w.raw());
        let (lo, hi) = (b.y.raw(), b.y.raw() + b.h.raw());
        let mut boxes = Vec::new();
        for it in &p.items {
            let (x0, x1, y0, y1) = match &it.kind {
                ItemKind::Text(x) => (
                    it.x.raw(),
                    it.x.raw() + x.width.raw(),
                    it.y.raw() - x.descent.raw(),
                    it.y.raw() + x.ascent.raw(),
                ),
                ItemKind::Rect(x) => {
                    // A stroke is centred on the edge, so it reaches half its width outside.
                    let hw = x.stroke.map_or(0, |(_, w)| w.raw() / 2);
                    (
                        it.x.raw() - hw,
                        it.x.raw() + x.w.raw() + hw,
                        it.y.raw() - hw,
                        it.y.raw() + x.h.raw() + hw,
                    )
                }
            };
            assert!(
                x0 >= l && x1 <= r && y0 >= lo && y1 <= hi,
                "page {pi}: item outside content box: {it:?}"
            );
            if matches!(it.kind, ItemKind::Text(_)) {
                boxes.push((x0, x1, y0, y1));
            }
        }
        for (i, a) in boxes.iter().enumerate() {
            for c in boxes.iter().skip(i + 1) {
                let apart_x = a.1 <= c.0 || c.1 <= a.0;
                let apart_y = a.3 <= c.2 || c.3 <= a.2;
                assert!(
                    apart_x || apart_y,
                    "page {pi}: text boxes overlap {a:?} {c:?}"
                );
            }
        }
        for lk in &p.links {
            let (x1, y1) = (
                lk.rect.x.raw() + lk.rect.w.raw(),
                lk.rect.y.raw() + lk.rect.h.raw(),
            );
            assert!(lk.rect.x.raw() >= l && x1 <= r && lk.rect.y.raw() >= lo && y1 <= hi);
        }
    }
}

/// xorshift64* for the generator.
pub(crate) struct Rng(pub(crate) u64);

impl Rng {
    pub(crate) fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub(crate) fn below(&mut self, n: usize) -> usize {
        (self.next() % (n.max(1) as u64)) as usize
    }
    pub(crate) fn chance(&mut self, pct: usize) -> bool {
        self.below(100) < pct
    }
}

fn gen_words(r: &mut Rng, max: usize) -> String {
    let n = 1 + r.below(max);
    let s = words_of(r.below(50), n);
    if r.chance(10) {
        format!("{s} supercalifragilisticexpialidocious")
    } else {
        s
    }
}

fn gen_pagination(r: &mut Rng) -> Pagination {
    Pagination {
        keep_with_next: r.chance(15),
        widow_orphan: r.chance(50),
        page_break_before: r.chance(5),
    }
}

fn gen_inlines(r: &mut Rng, with_links: bool) -> Vec<Inline> {
    let mut v = vec![Inline::Text(gen_words(r, 40))];
    if r.chance(25) {
        v.push(Inline::Run {
            role: "bold".into(),
            text: gen_words(r, 4),
        });
    }
    if with_links && r.chance(25) {
        let t = if r.chance(50) {
            "#top"
        } else {
            "https://example.org/x"
        };
        v.push(Inline::Link {
            target: t.into(),
            text: gen_words(r, 3),
        });
    }
    if r.chance(10) {
        v.push(Inline::Highlight(gen_words(r, 3)));
    }
    if r.chance(10) {
        v.push(Inline::FieldRef(FieldKind::Page));
    }
    v.push(Inline::Text(gen_words(r, 20)));
    v
}

fn gen_table(r: &mut Rng) -> Block {
    let cols = 1 + r.below(5) as u32;
    let rows = 1 + r.below(30);
    let hdr = r.below(3.min(rows + 1)) as u32;
    let mut out = Vec::new();
    for _ in 0..rows {
        let mut cells = Vec::new();
        let mut left = cols;
        while left > 0 {
            let s = 1 + r.below(left as usize) as u32;
            let c = if r.chance(30) {
                num(&format!("{}", r.below(100_000)))
            } else {
                cell(&gen_words(r, 8))
            };
            cells.push(Cell { span: s, ..c });
            left -= s;
        }
        out.push(row(cells));
    }
    table(cols, hdr.min(rows as u32), out)
}

fn gen_block(r: &mut Rng) -> Block {
    match r.below(100) {
        0..=44 => Block::Para(Para {
            role: String::new(),
            inlines: gen_inlines(r, true),
            pagination: gen_pagination(r),
        }),
        45..=59 => Block::Heading(Heading {
            level: 1 + r.below(6) as u8,
            inlines: text(&gen_words(r, 6)),
            pagination: gen_pagination(r),
        }),
        60..=69 => {
            let items: Vec<Vec<Inline>> = (0..=r.below(5)).map(|_| gen_inlines(r, false)).collect();
            Block::List(List {
                numbering: 1,
                ordered: r.chance(50),
                items,
            })
        }
        70..=89 => gen_table(r),
        90..=93 => Block::Break(BreakKind::Page),
        94..=96 => Block::Break(BreakKind::Line),
        _ => Block::Field(FieldKind::Page),
    }
}

fn gen_chrome(r: &mut Rng, num_pages: bool) -> Vec<Block> {
    if r.chance(40) {
        return Vec::new();
    }
    let mut v = vec![
        Inline::Text("Page ".into()),
        Inline::FieldRef(FieldKind::Page),
    ];
    if num_pages {
        v.push(Inline::Text(" of ".into()));
        v.push(Inline::FieldRef(FieldKind::NumPages));
    }
    vec![para_in(v)]
}

/// A valid random flow document. Geometry is chosen so layout must succeed.
pub(crate) fn generate(seed: u64, num_pages: bool) -> Document {
    let mut r = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let nsec = 1 + r.below(3);
    let mut sections = Vec::new();
    for s in 0..nsec {
        let (w, h) = [(240, 320), (300, 400), (612, 792)][r.below(3)];
        let mut blocks = Vec::new();
        if s == 0 {
            blocks.push(para_in(vec![
                Inline::Anchor("top".into()),
                Inline::Text("Top".into()),
            ]));
        }
        for _ in 0..=r.below(13) {
            blocks.push(gen_block(&mut r));
        }
        let mut sec = section(setup(w, h, 36), blocks);
        sec.header = gen_chrome(&mut r, num_pages);
        sec.footer = gen_chrome(&mut r, num_pages);
        sec.different_first_page = r.chance(30);
        sec.restart_page_numbers = r.chance(30);
        sections.push(sec);
    }
    doc(sections)
}
