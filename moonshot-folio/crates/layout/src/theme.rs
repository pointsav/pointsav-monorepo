// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Typographic settings for layout. This stands in for the design-token snapshot (crate
//! `tokens` is still a skeleton): every value is an explicit length or ratio, nothing is read
//! from the environment.

use crate::tree::FontSlot;
use moonshot_folio_core::{Emu, FolioError, Ink, Result, EMU_PER_POINT};
use std::collections::BTreeMap;

/// How a role name changes the base text style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoleStyle {
    /// Use the bold face.
    pub bold: bool,
    /// Replace the font size.
    pub size: Option<Emu>,
    /// Replace the colour.
    pub ink: Option<Ink>,
}

/// Sizes, spacing and colours.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theme {
    /// Body text size.
    pub body_size: Emu,
    /// Heading sizes for levels 1 to 6.
    pub heading_sizes: [Emu; 6],
    /// Line height as a percentage of the font size (50 to 1000).
    pub leading_percent: i64,
    /// Space after a paragraph.
    pub para_after: Emu,
    /// Space before a heading.
    pub heading_before: Emu,
    /// Space after a heading.
    pub heading_after: Emu,
    /// Hanging indent of list items (widened when a marker needs more).
    pub list_indent: Emu,
    /// Space between a list marker and its text.
    pub marker_gap: Emu,
    /// Space after a list item other than the last.
    pub list_item_after: Emu,
    /// Padding inside a table cell.
    pub cell_padding: Emu,
    /// Width of table rules.
    pub rule_width: Emu,
    /// Colour of table rules.
    pub rule_ink: Ink,
    /// Fill of repeated header rows.
    pub header_fill: Ink,
    /// Text colour.
    pub text_ink: Ink,
    /// Hyperlink colour.
    pub link_ink: Ink,
    /// Highlight fill.
    pub highlight_ink: Ink,
    /// Size of text in headers and footers.
    pub chrome_size: Emu,
    /// Space between a header or footer and the body.
    pub chrome_gap: Emu,
    /// Indent of table-of-contents entries per heading level below 1.
    pub toc_indent: Emu,
    /// Space after a table-of-contents entry.
    pub toc_entry_after: Emu,
    /// Role names to styles; an unknown role is plain text.
    pub roles: BTreeMap<String, RoleStyle>,
}

/// Largest table rule width, in points.
const MAX_RULE_PT: i64 = 72;
/// Largest list indent, in points.
const MAX_INDENT_PT: i64 = 720;

fn pt(n: i64) -> Emu {
    Emu::from_raw(n.saturating_mul(EMU_PER_POINT))
}

impl Theme {
    /// A plain document style: 11 point body, 120 percent leading.
    #[must_use]
    pub fn standard() -> Self {
        let bold = RoleStyle {
            bold: true,
            ..RoleStyle::default()
        };
        let mut roles = BTreeMap::new();
        roles.insert("bold".to_string(), bold);
        roles.insert("strong".to_string(), bold);
        Self {
            body_size: pt(11),
            heading_sizes: [pt(24), pt(20), pt(16), pt(14), pt(12), pt(11)],
            leading_percent: 120,
            para_after: pt(6),
            heading_before: pt(12),
            heading_after: pt(6),
            list_indent: pt(18),
            marker_gap: pt(4),
            list_item_after: pt(2),
            cell_padding: pt(3),
            rule_width: Emu::from_raw(EMU_PER_POINT / 2),
            rule_ink: Ink::Gray(128),
            header_fill: Ink::Gray(230),
            text_ink: Ink::Gray(0),
            link_ink: Ink::Rgb(5, 99, 193),
            highlight_ink: Ink::Rgb(255, 255, 0),
            chrome_size: pt(9),
            chrome_gap: pt(6),
            toc_indent: pt(12),
            toc_entry_after: pt(2),
            roles,
        }
    }

    /// Check the values layout relies on.
    ///
    /// # Errors
    /// `Invalid` for a size that is not positive, a negative length or leading outside range.
    pub fn check(&self) -> Result<()> {
        let sizes_ok = self
            .heading_sizes
            .iter()
            .chain([&self.body_size, &self.chrome_size])
            .all(|s| *s > Emu::ZERO)
            && self
                .roles
                .values()
                .all(|r| r.size.is_none_or(|s| s > Emu::ZERO));
        let gaps_ok = [
            self.para_after,
            self.heading_before,
            self.heading_after,
            self.list_indent,
            self.marker_gap,
            self.list_item_after,
            self.cell_padding,
            self.rule_width,
            self.chrome_gap,
            self.toc_indent,
            self.toc_entry_after,
        ]
        .iter()
        .all(|g| *g >= Emu::ZERO);
        let bounds_ok = self.rule_width <= pt(MAX_RULE_PT)
            && self.list_indent <= pt(MAX_INDENT_PT)
            && self.toc_indent <= pt(MAX_INDENT_PT);
        if sizes_ok && gaps_ok && bounds_ok && (50..=1000).contains(&self.leading_percent) {
            Ok(())
        } else {
            Err(FolioError::Invalid("theme value out of range"))
        }
    }

    /// The style of text with role `role` on top of `base`.
    pub(crate) fn apply(&self, role: &str, base: Style) -> Style {
        let Some(r) = self.roles.get(role) else {
            return base;
        };
        Style {
            slot: if r.bold { FontSlot::Bold } else { base.slot },
            size: r.size.unwrap_or(base.size),
            ink: r.ink.unwrap_or(base.ink),
        }
    }
}

/// Resolved text style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Style {
    pub(crate) slot: FontSlot,
    pub(crate) size: Emu,
    pub(crate) ink: Ink,
}
