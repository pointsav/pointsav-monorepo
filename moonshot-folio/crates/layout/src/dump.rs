// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Canonical text form of a `PageTree`: one line per item, byte-stable, for golden files.

use crate::tree::{BlockKind, Item, ItemKind, LinkAnnot, LinkTarget, NodeId, PageTree, StructRole};
use core::fmt::Write;
use moonshot_folio_core::Ink;

/// Render a page tree as canonical text. Lengths are raw EMU.
#[must_use]
pub fn dump(tree: &PageTree) -> String {
    let mut o = String::new();
    writeln!(
        o,
        "pagetree pages={} substituted={}",
        tree.pages.len(),
        tree.substituted
    )
    .ok();
    for (i, p) in tree.pages.iter().enumerate() {
        writeln!(
            o,
            "page {i} section={} label={} size={}x{} margin={}",
            p.section,
            p.label,
            p.width.raw(),
            p.height.raw(),
            p.margin.raw()
        )
        .ok();
        p.items.iter().for_each(|it| item(&mut o, it));
        p.links.iter().for_each(|l| link(&mut o, l));
    }
    for b in &tree.blocks {
        writeln!(o, "block {} {}", node(&b.node), kind(&b.kind)).ok();
    }
    for d in &tree.dests {
        writeln!(
            o,
            "dest {:?} page={} x={} y={}",
            d.name,
            d.page,
            d.x.raw(),
            d.y.raw()
        )
        .ok();
    }
    o
}

fn node(n: &NodeId) -> String {
    let parts: Vec<String> = n.path().iter().map(u32::to_string).collect();
    parts.join(".")
}

fn role(r: StructRole) -> String {
    match r {
        StructRole::Heading(l) => format!("Heading{l}"),
        other => format!("{other:?}"),
    }
}

#[allow(clippy::many_single_char_names)]
fn ink(i: Ink) -> String {
    match i {
        Ink::Gray(g) => format!("gray({g})"),
        Ink::Rgb(r, g, b) => format!("rgb({r},{g},{b})"),
        Ink::Cmyk(c, m, y, k) => format!("cmyk({c},{m},{y},{k})"),
    }
}

fn item(o: &mut String, it: &Item) {
    let rep = if it.repeated { " repeated" } else { "" };
    let head = format!(
        "  {} {}{rep} x={} y={}",
        node(&it.node),
        role(it.role),
        it.x.raw(),
        it.y.raw()
    );
    match &it.kind {
        ItemKind::Text(t) => writeln!(
            o,
            "{head} text font={:?} size={} w={} asc={} desc={} ink={}{} {:?}",
            t.font,
            t.size.raw(),
            t.width.raw(),
            t.ascent.raw(),
            t.descent.raw(),
            ink(t.ink),
            if t.space_after { " hung" } else { "" },
            t.text
        ),
        ItemKind::Rect(r) => writeln!(
            o,
            "{head} rect w={} h={} fill={} stroke={}",
            r.w.raw(),
            r.h.raw(),
            r.fill.map_or("none".to_string(), ink),
            r.stroke.map_or("none".to_string(), |(c, w)| format!(
                "{}@{}",
                ink(c),
                w.raw()
            ))
        ),
    }
    .ok();
}

fn link(o: &mut String, l: &LinkAnnot) {
    let target = match &l.target {
        LinkTarget::Uri(u) => format!("uri {u:?}"),
        LinkTarget::Named(n) => format!("named {n:?}"),
    };
    writeln!(
        o,
        "  {} link x={} y={} w={} h={} {target}",
        node(&l.node),
        l.rect.x.raw(),
        l.rect.y.raw(),
        l.rect.w.raw(),
        l.rect.h.raw()
    )
    .ok();
}

fn kind(k: &BlockKind) -> String {
    match k {
        BlockKind::Paragraph => "Paragraph".to_string(),
        BlockKind::Heading(l) => format!("Heading{l}"),
        BlockKind::List { ordered } => format!("List ordered={ordered}"),
        BlockKind::Table(t) => format!("Table header_rows={} spans={:?}", t.header_rows, t.spans),
        BlockKind::Toc => "Toc".to_string(),
    }
}
