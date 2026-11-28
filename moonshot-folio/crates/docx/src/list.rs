// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Lists: item paragraphs reference `word/numbering.xml` through `w:numPr`.

use crate::body::inlines;
use crate::ctx::Ctx;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::List;

pub(crate) fn list(out: &mut String, l: &List, cx: &mut Ctx) -> Result<()> {
    let num_id = if let Some(i) = cx.nums.iter().position(|(n, _)| *n == l.numbering) {
        if cx.nums.get(i).is_some_and(|(_, ord)| *ord != l.ordered) {
            return Err(FolioError::Invalid(
                "one numbering id used for both ordered and bulleted lists",
            ));
        }
        i
    } else {
        cx.nums.push((l.numbering, l.ordered));
        cx.nums.len().saturating_sub(1)
    }
    .saturating_add(1);
    for item in &l.items {
        write!(
            out,
            "<w:p><w:pPr><w:pStyle w:val=\"ListParagraph\"/><w:numPr><w:ilvl w:val=\"0\"/><w:numId w:val=\"{num_id}\"/></w:numPr></w:pPr>"
        )
        .ok();
        inlines(out, item, cx)?;
        out.push_str("</w:p>");
    }
    Ok(())
}

/// `word/numbering.xml`: abstract 0 is bulleted, abstract 1 is decimal; one `w:num` per list.
pub(crate) fn numbering_xml(nums: &[(u32, bool)]) -> String {
    use crate::package::{DECL, NS_W};
    let mut s = format!("{DECL}<w:numbering xmlns:w=\"{NS_W}\">");
    for (id, fmt, text) in [(0, "bullet", "\u{2022}"), (1, "decimal", "%1.")] {
        write!(
            s,
            "<w:abstractNum w:abstractNumId=\"{id}\"><w:multiLevelType w:val=\"singleLevel\"/><w:lvl w:ilvl=\"0\"><w:start w:val=\"1\"/><w:numFmt w:val=\"{fmt}\"/><w:lvlText w:val=\"{text}\"/><w:lvlJc w:val=\"left\"/><w:pPr><w:ind w:left=\"720\" w:hanging=\"360\"/></w:pPr></w:lvl></w:abstractNum>"
        )
        .ok();
    }
    for (i, (_, ordered)) in nums.iter().enumerate() {
        let abs = u8::from(*ordered);
        write!(
            s,
            "<w:num w:numId=\"{}\"><w:abstractNumId w:val=\"{abs}\"/><w:lvlOverride w:ilvl=\"0\"><w:startOverride w:val=\"1\"/></w:lvlOverride></w:num>",
            i.saturating_add(1)
        )
        .ok();
    }
    s.push_str("</w:numbering>");
    s
}
