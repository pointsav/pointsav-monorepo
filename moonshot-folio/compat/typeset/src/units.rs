// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Point to EMU quantisation and page setup.

use moonshot_folio_core::{Emu, FolioError, Result, EMU_PER_POINT};
use moonshot_folio_ir::{Block, FieldKind, Inline, PageSetup, Pagination, Para, Section};
use tool_typeset::doc::PageSetup as TtPage;

/// Largest accepted magnitude in points (about 35 km): keeps every later product in range.
const MAX_POINTS: f64 = 1.0e8;

/// Quantise points to hundredths, then convert to EMU (half away from zero).
pub(crate) fn pt_to_emu(v: f32) -> Result<Emu> {
    let p = f64::from(v);
    if !p.is_finite() || p.abs() > MAX_POINTS {
        return Err(FolioError::Invalid("length is not finite or out of range"));
    }
    // In range by the check above (|p| * 100 <= 1e10), so the cast cannot truncate.
    #[allow(clippy::cast_possible_truncation)]
    let hundredths = (p * 100.0).round() as i64;
    Emu::from_ratio(hundredths, EMU_PER_POINT, 100)
}

/// Convert the page geometry; running header, page numbers and register are reported as lost.
pub(crate) fn page_setup(p: &TtPage, notes: &mut Vec<String>) -> Result<PageSetup> {
    let (w, h) = p.size.dims_pt();
    let (w, h) = if p.landscape { (h, w) } else { (w, h) };
    notes.push(format!(
        "register {:?} not carried (style role mapping is a tokens task)",
        p.register
    ));
    Ok(PageSetup {
        width: pt_to_emu(w)?,
        height: pt_to_emu(h)?,
        margin: pt_to_emu(p.margin_pt)?,
    })
}

/// Running header, page-number footer and first-page suppression map onto section chrome.
pub(crate) fn chrome(p: &TtPage, section: &mut Section) {
    if let Some(h) = &p.running_header {
        section.header.push(Block::Para(Para {
            role: "running-header".into(),
            inlines: vec![Inline::Text(h.clone())],
            pagination: Pagination::default(),
        }));
    }
    if p.page_numbers {
        section.footer.push(Block::Para(Para {
            role: "page-number".into(),
            inlines: vec![Inline::FieldRef(FieldKind::Page)],
            pagination: Pagination::default(),
        }));
    }
    section.different_first_page = p.suppress_header_on_first;
}
