// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `word/document.xml`.

use crate::body::block;
use crate::ctx::{Ctx, Part};
use crate::package::{DECL, NS_R, NS_W};
use core::fmt::Write;
use moonshot_folio_core::{Emu, FolioError, Result};
use moonshot_folio_ir::{Block, Body, Document, Section};

/// EMU per twip, the unit `WordprocessingML` page geometry uses.
const EMU_PER_TWIP: i64 = 635;

/// Convert EMU to whole twips, rounding half away from zero.
fn twips(e: Emu) -> Result<i64> {
    Emu::from_ratio(e.raw(), 1, EMU_PER_TWIP).map(Emu::raw)
}

/// The `w:sectPr` for one section: header and footer references, page geometry, `w:titlePg`.
fn sect_pr(sec: &Section, refs: &str) -> Result<String> {
    let p = &sec.page;
    let (w, h, m) = (twips(p.width)?, twips(p.height)?, twips(p.margin)?);
    let restart = if sec.restart_page_numbers {
        "<w:pgNumType w:start=\"1\"/>"
    } else {
        ""
    };
    // Header and footer sit half a margin from the edge, at most half an inch.
    let hf = m.checked_div(2).unwrap_or(0).clamp(0, 720);
    let title = if sec.different_first_page {
        "<w:titlePg/>"
    } else {
        ""
    };
    Ok(format!(
        "<w:sectPr>{refs}<w:pgSz w:w=\"{w}\" w:h=\"{h}\"/><w:pgMar w:top=\"{m}\" w:right=\"{m}\" w:bottom=\"{m}\" w:left=\"{m}\" w:header=\"{hf}\" w:footer=\"{hf}\" w:gutter=\"0\"/>{restart}{title}</w:sectPr>"
    ))
}

/// Write a header or footer part for section `n` (1-based) and return its reference element(s).
///
/// A section with no reference would inherit the previous section's header or footer, so when any
/// section of the document has one of this kind, a section without gets an empty part.
fn chrome(
    blocks: &[Block],
    footer: bool,
    n: usize,
    first: bool,
    any: bool,
    cx: &mut Ctx,
) -> Result<String> {
    if blocks.is_empty() && !any {
        return Ok(String::new());
    }
    let (tag, kind) = if footer {
        ("ftr", "footer")
    } else {
        ("hdr", "header")
    };
    let mut xml = format!("{DECL}<w:{tag} xmlns:w=\"{NS_W}\" xmlns:r=\"{NS_R}\">");
    cx.in_part = true;
    for b in blocks {
        block(&mut xml, b, cx)?;
    }
    // Every header, footer and table cell must end with a paragraph (Word insists).
    if blocks.is_empty() || matches!(blocks.last(), Some(Block::Table(_))) {
        xml.push_str("<w:p/>");
    }
    cx.in_part = false;
    write!(xml, "</w:{tag}>").ok();
    let rel_id = format!("rId{}{n}", if footer { "F" } else { "H" });
    let mut refs = format!("<w:{kind}Reference w:type=\"default\" r:id=\"{rel_id}\"/>");
    // With a distinct first page, the footer repeats on page one; the header does not.
    if footer && first {
        write!(
            refs,
            "<w:footerReference w:type=\"first\" r:id=\"{rel_id}\"/>"
        )
        .ok();
    }
    cx.parts.push(Part {
        name: format!("word/{kind}{n}.xml"),
        rel_id,
        footer,
        xml,
    });
    Ok(refs)
}

/// The document part, and the context the other parts need.
pub(crate) fn document_xml(doc: &Document) -> Result<(String, Ctx)> {
    let Body::Flow(sections) = &doc.body else {
        return Err(FolioError::Unsupported("DOCX needs a flow body"));
    };
    let mut cx = Ctx::default();
    let mut s = format!("{DECL}<w:document xmlns:w=\"{NS_W}\" xmlns:r=\"{NS_R}\"><w:body>");
    let count = sections.len();
    let any_header = sections.iter().any(|s| !s.header.is_empty());
    let any_footer = sections.iter().any(|s| !s.footer.is_empty());
    for (i, sec) in sections.iter().enumerate() {
        cx.content_twips =
            twips(sec.page.width)?.saturating_sub(twips(sec.page.margin)?.saturating_mul(2));
        for b in &sec.blocks {
            block(&mut s, b, &mut cx)?;
        }
        if matches!(sec.blocks.last(), Some(Block::Table(_))) {
            s.push_str("<w:p/>");
        }
        let n = i.saturating_add(1);
        let first = sec.different_first_page;
        let mut refs = chrome(&sec.header, false, n, first, any_header, &mut cx)?;
        refs.push_str(&chrome(&sec.footer, true, n, first, any_footer, &mut cx)?);
        let pr = sect_pr(sec, &refs)?;
        if i.saturating_add(1) == count {
            s.push_str(&pr);
        } else {
            // A non-final section's properties ride the last paragraph of that section.
            write!(s, "<w:p><w:pPr>{pr}</w:pPr></w:p>").ok();
        }
    }
    s.push_str("</w:body></w:document>");
    if cx.anchor_links.iter().any(|a| !cx.anchors.contains(a)) {
        return Err(FolioError::Invalid(
            "internal link points to an anchor that does not exist",
        ));
    }
    Ok((s, cx))
}
