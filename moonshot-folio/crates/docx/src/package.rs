// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Fixed package parts: content types, relationships, styles and document properties.

use crate::ctx::{style_id, Ctx, Part};
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::Result;
use moonshot_folio_ir::Meta;

pub(crate) const DECL: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
pub(crate) const NS_R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub(crate) const NS_W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const NS_REL: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const REL_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const REL_STYLES: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const REL_CORE: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";

pub(crate) fn content_types(parts: &[Part], numbering: bool, settings: bool) -> String {
    let mut extra = String::new();
    if settings {
        extra.push_str("<Override PartName=\"/word/settings.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml\"/>");
    }
    if numbering {
        extra.push_str("<Override PartName=\"/word/numbering.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml\"/>");
    }
    for p in parts {
        let kind = if p.footer { "footer" } else { "header" };
        write!(extra, "<Override PartName=\"/{}\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.{kind}+xml\"/>", p.name).ok();
    }
    format!(
        "{DECL}<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\
<Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
<Default Extension=\"xml\" ContentType=\"application/xml\"/>\
<Override PartName=\"/word/document.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\"/>\
<Override PartName=\"/word/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml\"/>\
<Override PartName=\"/docProps/core.xml\" ContentType=\"application/vnd.openxmlformats-package.core-properties+xml\"/>\
{extra}</Types>"
    )
}

pub(crate) fn root_rels() -> String {
    format!(
        "{DECL}<Relationships xmlns=\"{NS_REL}\">\
<Relationship Id=\"rId1\" Type=\"{REL_DOC}\" Target=\"word/document.xml\"/>\
<Relationship Id=\"rId2\" Type=\"{REL_CORE}\" Target=\"docProps/core.xml\"/>\
</Relationships>"
    )
}

pub(crate) fn document_rels(
    links: &[String],
    parts: &[Part],
    numbering: bool,
    settings: bool,
) -> Result<String> {
    let mut s = format!(
        "{DECL}<Relationships xmlns=\"{NS_REL}\"><Relationship Id=\"rId1\" Type=\"{REL_STYLES}\" Target=\"styles.xml\"/>"
    );
    for (i, t) in links.iter().enumerate() {
        write!(
            s,
            "<Relationship Id=\"rId{}\" Type=\"{NS_R}/hyperlink\" Target=\"",
            i.saturating_add(2)
        )
        .ok();
        escape_into(&mut s, t)?;
        s.push_str("\" TargetMode=\"External\"/>");
    }
    if settings {
        write!(
            s,
            "<Relationship Id=\"rIdS\" Type=\"{NS_R}/settings\" Target=\"settings.xml\"/>"
        )
        .ok();
    }
    if numbering {
        write!(
            s,
            "<Relationship Id=\"rIdN\" Type=\"{NS_R}/numbering\" Target=\"numbering.xml\"/>"
        )
        .ok();
    }
    for p in parts {
        let kind = if p.footer { "footer" } else { "header" };
        let file = p.name.strip_prefix("word/").unwrap_or(&p.name);
        write!(
            s,
            "<Relationship Id=\"{}\" Type=\"{NS_R}/{kind}\" Target=\"{file}\"/>",
            p.rel_id
        )
        .ok();
    }
    s.push_str("</Relationships>");
    Ok(s)
}

/// Default styles: one paragraph default, `Normal`, and the heading styles 1 to 6.
pub(crate) fn styles(cx: &Ctx) -> Result<String> {
    let mut s = format!("{DECL}<w:styles xmlns:w=\"{NS_W}\">");
    s.push_str("<w:docDefaults><w:rPrDefault><w:rPr><w:sz w:val=\"22\"/></w:rPr></w:rPrDefault></w:docDefaults>");
    s.push_str("<w:style w:type=\"paragraph\" w:default=\"1\" w:styleId=\"Normal\"><w:name w:val=\"Normal\"/></w:style>");
    for outline in 0..6_u8 {
        let level = outline.saturating_add(1);
        write!(
            s,
            "<w:style w:type=\"paragraph\" w:styleId=\"Heading{level}\"><w:name w:val=\"heading {level}\"/><w:basedOn w:val=\"Normal\"/><w:next w:val=\"Normal\"/><w:pPr><w:keepNext/><w:outlineLvl w:val=\"{outline}\"/></w:pPr><w:rPr><w:b/></w:rPr></w:style>"
        )
        .ok();
    }
    s.push_str("<w:style w:type=\"paragraph\" w:styleId=\"ListParagraph\"><w:name w:val=\"List Paragraph\"/><w:basedOn w:val=\"Normal\"/><w:pPr><w:ind w:left=\"720\"/></w:pPr></w:style>");
    s.push_str("<w:style w:type=\"character\" w:styleId=\"Hyperlink\"><w:name w:val=\"Hyperlink\"/><w:rPr><w:color w:val=\"0563C1\"/><w:u w:val=\"single\"/></w:rPr></w:style>");
    let mut names: Vec<String> = Vec::new();
    for r in &cx.para_roles {
        role_style(&mut s, "paragraph", r, &mut names)?;
    }
    for r in &cx.char_roles {
        role_style(&mut s, "character", r, &mut names)?;
    }
    s.push_str("</w:styles>");
    Ok(s)
}

/// A role style with no formatting of its own; tokens fill it in later (F3.4). Word compares style
/// names without regard to case, so a name that collides after case folding gets a numeric suffix.
fn role_style(s: &mut String, kind: &str, role: &str, names: &mut Vec<String>) -> Result<()> {
    let k = if kind == "paragraph" { 'P' } else { 'C' };
    let base = format!("TS {role}");
    let mut name = base.clone();
    let mut n = 1_u32;
    while names.contains(&name.to_lowercase()) {
        n = n.saturating_add(1);
        name = format!("{base} ({n})");
    }
    names.push(name.to_lowercase());
    write!(
        s,
        "<w:style w:type=\"{kind}\" w:customStyle=\"1\" w:styleId=\"{}\"><w:name w:val=\"",
        style_id(k, role)
    )
    .ok();
    escape_into(s, &name)?;
    s.push_str("\"/>");
    if kind == "paragraph" {
        s.push_str("<w:basedOn w:val=\"Normal\"/>");
    }
    s.push_str("</w:style>");
    Ok(())
}

/// Core properties. Every value comes from the caller; no clock is read.
pub(crate) fn core_props(m: &Meta) -> Result<String> {
    let mut s = format!(
        "{DECL}<cp:coreProperties xmlns:cp=\"http://schemas.openxmlformats.org/package/2006/metadata/core-properties\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:dcterms=\"http://purl.org/dc/terms/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"><dc:title>"
    );
    escape_into(&mut s, &m.title)?;
    s.push_str("</dc:title><dc:creator>");
    escape_into(&mut s, &m.author)?;
    s.push_str("</dc:creator><dc:language>");
    escape_into(&mut s, &m.lang)?;
    s.push_str("</dc:language><dcterms:created xsi:type=\"dcterms:W3CDTF\">");
    escape_into(&mut s, &m.date)?;
    s.push_str("T00:00:00Z</dcterms:created></cp:coreProperties>");
    Ok(s)
}

/// Document settings. `updateFields` makes Word refresh page-count and table-of-contents fields
/// when the file is opened, because cached results cannot know the final pagination.
pub(crate) fn settings() -> String {
    format!("{DECL}<w:settings xmlns:w=\"{NS_W}\"><w:updateFields w:val=\"true\"/></w:settings>")
}
