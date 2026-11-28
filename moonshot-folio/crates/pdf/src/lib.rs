// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-pdf (L4): a deterministic, low-level PDF writer (tasks F5.2 to F5.4).
//!
//! The crate writes PDF 1.7 with a classic cross-reference table and uncompressed streams (no
//! compression option is offered: the codec crate has no deflate encoder). It covers the
//! standard Helvetica fonts, embedded TrueType subsets (`Type0` / `Identity-H` /
//! `CIDFontType2` with a `ToUnicode` map), link annotations, named destinations and bookmarks.
//! Page layout is not its job: a caller positions every glyph and line through [`Content`], or
//! uses [`render_pdf`], which lays a document out with `moonshot-folio-layout` and writes it.
//!
//! [`render_pdf`] writes tagged PDF by default (`RenderOptions::tagged`): marked content for every
//! text item, artifacts for everything else, and a structure tree (`structure` module for the
//! mapping) with the parent tree and the link annotations tied to it.
//!
//! Output is a pure function of the document: nothing reads a clock or the environment, and the
//! trailer `/ID` is a SHA-256 of the file body.
//!
//! ```text
//! let mut doc = PdfDoc::new(Limits::default());
//! let f = doc.add_std14(Std14::Helvetica);
//! let run = doc.encode_text(f, "Hello", TextPolicy::Reject)?;
//! let mut c = Content::new();
//! c.text(Emu::from_points(12)?, Emu::from_points(72)?, Emu::from_points(700)?, &run)?;
//! doc.add_page(Emu::from_points(612)?, Emu::from_points(792)?, &c, vec![])?;
//! let bytes = doc.build()?;
//! ```

#![forbid(unsafe_code)]

mod content;
mod doc;
mod embed;
mod nav;
mod objects;
mod render;
mod serialize;
mod structure;
mod syntax;
mod tags;
mod text;
mod trees;

pub use content::{ArtifactKind, Content};
pub use doc::PdfDoc;
pub use nav::{Bookmark, Dest, Info, Link, LinkTarget, Rect};
pub use render::{bad_links, render_pdf, render_tree, BadLink, RenderOptions};
pub use tags::{Attr, ListNumbering, StructElem, StructKid, TableAttrs, TableScope};
pub use text::{FontId, TextPolicy, TextRun};
