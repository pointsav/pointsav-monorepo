// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! IR inlines and cell values to styled segments.

use crate::ctx::Ctx;
use crate::headings::RESERVED_PREFIX;
use crate::text::Seg;
use crate::theme::Style;
use crate::tree::{NodeId, StructRole};
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::{CellValue, FieldKind, Inline};

/// What the conversion of one paragraph needs besides the inlines.
pub(crate) struct InlineCx<'a> {
    pub(crate) ctx: &'a Ctx<'a>,
    /// Role of ordinary text.
    pub(crate) role: StructRole,
    /// The page number, when it is known now (headers and footers are laid out per page).
    pub(crate) label: Option<u32>,
}

/// Convert `inlines`; the `i`-th inline gets node `node.child(i)`.
///
/// # Errors
/// `Unsupported` for a table-of-contents field, `Invalid` for an empty anchor name.
pub(crate) fn segs_of(
    ic: &InlineCx<'_>,
    inlines: &[Inline],
    base: Style,
    node: NodeId,
) -> Result<Vec<Seg>> {
    let theme = ic.ctx.theme;
    let mut out = Vec::new();
    for (i, inl) in inlines.iter().enumerate() {
        let n = node.child(i);
        let plain = |t: &str, st: Style| Seg::plain(t.to_string(), st, n, ic.role);
        let seg = match inl {
            Inline::Text(t) => plain(t, base),
            Inline::Run { role, text } => plain(text, theme.apply(role, base)),
            Inline::Highlight(t) => Seg {
                highlight: true,
                ..plain(t, base)
            },
            Inline::Link { target, text } => Seg {
                link: Some(target.clone()),
                role: StructRole::Link,
                ..plain(
                    text,
                    Style {
                        ink: theme.link_ink,
                        ..base
                    },
                )
            },
            Inline::Anchor(name) => {
                if name.is_empty() {
                    return Err(FolioError::Invalid("anchor name is empty"));
                }
                if name.starts_with(RESERVED_PREFIX) {
                    return Err(FolioError::Invalid(
                        "anchor name uses the reserved prefix folio-toc-",
                    ));
                }
                Seg {
                    anchor: Some(name.clone()),
                    ..plain("", base)
                }
            }
            Inline::FieldRef(k) => field_seg(ic, *k, base, n)?,
        };
        out.push(seg);
    }
    Ok(out)
}

/// A page-number field as a segment.
pub(crate) fn field_seg(
    ic: &InlineCx<'_>,
    kind: FieldKind,
    base: Style,
    node: NodeId,
) -> Result<Seg> {
    let ctx = ic.ctx;
    match kind {
        FieldKind::NumPages => {
            ctx.mark_num_pages();
            Ok(Seg::plain(ctx.num_pages.to_string(), base, node, ic.role))
        }
        FieldKind::Page => {
            if let Some(l) = ic.label {
                return Ok(Seg::plain(l.to_string(), base, node, ic.role));
            }
            ctx.mark_page_field();
            let digits = ctx.num_pages.to_string().len();
            Ok(Seg {
                field_digits: digits,
                ..Seg::plain("0".to_string(), base, node, ic.role)
            })
        }
        FieldKind::Toc => Err(FolioError::Unsupported(
            "table of contents inside a paragraph",
        )),
    }
}

/// The text of a table cell as one segment.
pub(crate) fn cell_segs(
    value: &CellValue,
    base: Style,
    node: NodeId,
    role: StructRole,
) -> Vec<Seg> {
    let text = match value {
        CellValue::Text(s) | CellValue::Number(s) => s.clone(),
        CellValue::Bool(b) => b.to_string(),
        CellValue::Empty => String::new(),
    };
    vec![Seg::plain(text, base, node, role)]
}
