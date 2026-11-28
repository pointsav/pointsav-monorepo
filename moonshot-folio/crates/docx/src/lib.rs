// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-docx (L4): IR to `WordprocessingML`, packaged as a deterministic store-only ZIP.
//!
//! Design: BRIEF-moonshot-folio-architecture. Task F3.1 (package); body content follows in F3.2+.

#![forbid(unsafe_code)]

mod body;
mod ctx;
mod document;
mod list;
mod package;
mod table;
mod xml;

use moonshot_folio_codec::ZipWriter;
use moonshot_folio_core::{Limits, Result};
use moonshot_folio_ir::{check_target, features, validate, Document, Feature, Target};

/// Render a flow document as a `.docx` package.
///
/// # Errors
/// Any [`validate`] failure, `Unsupported` for a feature DOCX rejects, `Invalid` for text XML
/// cannot carry, and `Limit` when `limits` is exceeded.
pub fn render_docx(doc: &Document, limits: &Limits) -> Result<Vec<u8>> {
    validate(doc, limits)?;
    check_target(doc, Target::Docx)?;
    let (document_xml, cx) = document::document_xml(doc)?;
    let settings = features(doc)
        .iter()
        .any(|f| matches!(f, Feature::Toc | Feature::PageFields));
    let mut zip = ZipWriter::new(*limits);
    zip.add(
        "[Content_Types].xml",
        package::content_types(&cx.parts, !cx.nums.is_empty(), settings).as_bytes(),
    )?;
    zip.add("_rels/.rels", package::root_rels().as_bytes())?;
    zip.add("word/document.xml", document_xml.as_bytes())?;
    zip.add(
        "word/_rels/document.xml.rels",
        package::document_rels(&cx.links, &cx.parts, !cx.nums.is_empty(), settings)?.as_bytes(),
    )?;
    zip.add("word/styles.xml", package::styles(&cx)?.as_bytes())?;
    if settings {
        zip.add("word/settings.xml", package::settings().as_bytes())?;
    }
    if !cx.nums.is_empty() {
        zip.add(
            "word/numbering.xml",
            list::numbering_xml(&cx.nums).as_bytes(),
        )?;
    }
    for p in &cx.parts {
        zip.add(&p.name, p.xml.as_bytes())?;
    }
    zip.add(
        "docProps/core.xml",
        package::core_props(&doc.meta)?.as_bytes(),
    )?;
    zip.finish()
}
