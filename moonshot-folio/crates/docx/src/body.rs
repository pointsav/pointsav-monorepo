// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Block and inline writers.

use crate::ctx::{check_bookmark, check_target, style_id, Ctx};
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::{Block, BreakKind, FieldKind, Heading, Inline, Pagination, Para};

/// Write one block. Lists, tables and images are not written yet (F3.3, F3.5).
pub(crate) fn block(out: &mut String, b: &Block, cx: &mut Ctx) -> Result<()> {
    match b {
        Block::Para(p) => para(out, p, cx),
        Block::Heading(h) => heading(out, h, cx),
        Block::Break(BreakKind::Page) => {
            out.push_str("<w:p><w:r><w:br w:type=\"page\"/></w:r></w:p>");
            Ok(())
        }
        Block::Break(BreakKind::Line) => {
            out.push_str("<w:p><w:r><w:br/></w:r></w:p>");
            Ok(())
        }
        Block::Field(k) => {
            out.push_str("<w:p>");
            field(out, *k);
            out.push_str("</w:p>");
            Ok(())
        }
        Block::List(l) => crate::list::list(out, l, cx),
        Block::Table(t) => crate::table::table(out, t, cx),
        Block::Image(_) => Err(FolioError::Unsupported("DOCX images are not written yet")),
    }
}

fn ppr(out: &mut String, style: &str, p: Pagination) {
    out.push_str("<w:pPr>");
    if !style.is_empty() {
        out.push_str("<w:pStyle w:val=\"");
        out.push_str(style);
        out.push_str("\"/>");
    }
    if p.keep_with_next {
        out.push_str("<w:keepNext/>");
    }
    if p.page_break_before {
        out.push_str("<w:pageBreakBefore/>");
    }
    // Word's default is widow control on; the IR default (false) means "no preference".
    if p.widow_orphan {
        out.push_str("<w:widowControl/>");
    }
    out.push_str("</w:pPr>");
}

fn para(out: &mut String, p: &Para, cx: &mut Ctx) -> Result<()> {
    let style = if p.role.is_empty() {
        String::new()
    } else {
        cx.para_roles.insert(p.role.clone());
        style_id('P', &p.role)
    };
    out.push_str("<w:p>");
    ppr(out, &style, p.pagination);
    inlines(out, &p.inlines, cx)?;
    out.push_str("</w:p>");
    Ok(())
}

fn heading(out: &mut String, h: &Heading, cx: &mut Ctx) -> Result<()> {
    out.push_str("<w:p>");
    ppr(out, &format!("Heading{}", h.level), h.pagination);
    inlines(out, &h.inlines, cx)?;
    out.push_str("</w:p>");
    Ok(())
}

pub(crate) fn inlines(out: &mut String, v: &[Inline], cx: &mut Ctx) -> Result<()> {
    for i in v {
        match i {
            Inline::Text(t) => run(out, "", t)?,
            Inline::Run { role, text } => {
                if role.is_empty() {
                    run(out, "", text)?;
                } else if role == "bold" {
                    run(out, "<w:b/>", text)?;
                } else {
                    cx.char_roles.insert(role.clone());
                    let rpr = format!("<w:rStyle w:val=\"{}\"/>", style_id('C', role));
                    run(out, &rpr, text)?;
                }
            }
            Inline::Highlight(t) => run(out, "<w:highlight w:val=\"yellow\"/>", t)?,
            Inline::Link { target, text } => link(out, target, text, cx)?,
            Inline::Anchor(name) => {
                check_bookmark(name)?;
                if cx.anchors.contains(name) {
                    return Err(FolioError::Invalid("duplicate anchor name"));
                }
                cx.anchors.push(name.clone());
                let id = cx.bookmark_id();
                write!(out, "<w:bookmarkStart w:id=\"{id}\" w:name=\"{name}\"/><w:bookmarkEnd w:id=\"{id}\"/>").ok();
            }
            Inline::FieldRef(k) => field(out, *k),
        }
    }
    Ok(())
}

fn link(out: &mut String, target: &str, text: &str, cx: &mut Ctx) -> Result<()> {
    if let Some(anchor) = target.strip_prefix('#') {
        check_bookmark(anchor)?;
        cx.anchor_links.push(anchor.to_string());
        write!(out, "<w:hyperlink w:anchor=\"{anchor}\">").ok();
    } else {
        check_target(target)?;
        if cx.in_part {
            return Err(FolioError::Unsupported(
                "external links in headers and footers",
            ));
        }
        let id = cx.link_id(target);
        write!(out, "<w:hyperlink r:id=\"{id}\">").ok();
    }
    run(out, "<w:rStyle w:val=\"Hyperlink\"/>", text)?;
    out.push_str("</w:hyperlink>");
    Ok(())
}

/// One run; `\t` becomes `<w:tab/>` and `\n` becomes `<w:br/>`.
fn run(out: &mut String, rpr: &str, text: &str) -> Result<()> {
    // A bare CR or CRLF is a line break, as XML parsers would normalise it to one anyway.
    let text = &text.replace("\r\n", "\n").replace('\r', "\n");
    out.push_str("<w:r>");
    if !rpr.is_empty() {
        out.push_str("<w:rPr>");
        out.push_str(rpr);
        out.push_str("</w:rPr>");
    }
    let mut first = true;
    for piece in text.split_inclusive(['\t', '\n']) {
        let (body, sep) = match piece.char_indices().last() {
            Some((i, c @ ('\t' | '\n'))) => (piece.get(..i).unwrap_or(""), Some(c)),
            _ => (piece, None),
        };
        if !body.is_empty() {
            out.push_str("<w:t xml:space=\"preserve\">");
            escape_into(out, body)?;
            out.push_str("</w:t>");
        }
        match sep {
            Some('\t') => out.push_str("<w:tab/>"),
            Some(_) => out.push_str("<w:br/>"),
            None => {}
        }
        first = false;
    }
    if first {
        out.push_str("<w:t/>");
    }
    out.push_str("</w:r>");
    Ok(())
}

/// A complex-free simple field with a cached result, so viewers that skip updates show a value.
fn field(out: &mut String, k: FieldKind) {
    let (instr, cached) = match k {
        FieldKind::Page => (" PAGE ", "1"),
        FieldKind::NumPages => (" NUMPAGES ", "1"),
        FieldKind::Toc => (
            " TOC \\o &quot;1-3&quot; \\h ",
            "Update this field to build the table of contents.",
        ),
    };
    write!(
        out,
        "<w:fldSimple w:instr=\"{instr}\"><w:r><w:t>{cached}</w:t></w:r></w:fldSimple>"
    )
    .ok();
}
