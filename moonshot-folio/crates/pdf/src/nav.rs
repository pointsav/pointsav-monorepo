// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Document metadata, links, destinations and bookmarks.

use moonshot_folio_core::{Emu, FolioError, Result};

/// Longest accepted URI in bytes.
pub(crate) const MAX_URI: usize = 4096;

/// A position in the document: a zero-based page index and a point on that page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dest {
    /// Zero-based page index.
    pub page: usize,
    /// Horizontal position, from the left page edge.
    pub x: Emu,
    /// Vertical position, from the bottom page edge.
    pub y: Emu,
}

/// A clickable rectangle, from the bottom-left corner of the page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    /// Left edge.
    pub x: Emu,
    /// Bottom edge.
    pub y: Emu,
    /// Width.
    pub w: Emu,
    /// Height.
    pub h: Emu,
}

/// Where a link goes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    /// An external address; only `http:`, `https:` and `mailto:` are accepted.
    Uri(String),
    /// A named destination added with `PdfDoc::add_named_dest`.
    Named(String),
    /// An explicit page and position.
    Position(Dest),
}

/// A link annotation on a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    /// The clickable area.
    pub rect: Rect,
    /// The target.
    pub target: LinkTarget,
    /// Alternate description (`/Contents`), which a tagged link needs.
    pub contents: Option<String>,
}

/// One bookmark and its children.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bookmark {
    /// Text shown in the outline panel.
    pub title: String,
    /// Where it leads.
    pub dest: Dest,
    /// Nested bookmarks.
    pub children: Vec<Bookmark>,
}

/// Document information dictionary values, all supplied by the caller (nothing is read from the
/// clock or the environment).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Info {
    /// Title.
    pub title: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Subject.
    pub subject: Option<String>,
    /// Keywords.
    pub keywords: Option<String>,
    /// Creating application; defaults to `moonshot-folio`.
    pub creator: Option<String>,
    /// Creation date as `YYYY-MM-DD`, written as `D:YYYYMMDD000000Z`.
    pub creation_date: Option<String>,
    /// Document language (BCP 47), written as the catalog's `/Lang`.
    pub lang: Option<String>,
}

/// Accept `http://`, `https://` and `mailto:` addresses made only of printable ASCII.
pub(crate) fn validate_uri(uri: &str) -> Result<()> {
    if uri.len() > MAX_URI {
        return Err(FolioError::Limit("URI length"));
    }
    if !uri.bytes().all(|b| (0x21..=0x7E).contains(&b)) {
        return Err(FolioError::Invalid(
            "URI must be printable ASCII without spaces (percent-encode the rest)",
        ));
    }
    let lower = uri.to_ascii_lowercase();
    let ok = ["http://", "https://", "mailto:"]
        .iter()
        .any(|p| lower.strip_prefix(p).is_some_and(|rest| !rest.is_empty()));
    if ok {
        Ok(())
    } else {
        Err(FolioError::Invalid("URI scheme not allowed"))
    }
}

/// Validate a calendar date `YYYY-MM-DD` and return it as `YYYYMMDD`.
pub(crate) fn date_digits(d: &str) -> Result<String> {
    if !moonshot_folio_ir::valid_date(d) {
        return Err(FolioError::Invalid(
            "creation date must be a calendar date YYYY-MM-DD",
        ));
    }
    Ok(d.replace('-', ""))
}
