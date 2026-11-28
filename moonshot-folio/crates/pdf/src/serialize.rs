// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Serialising a [`PdfDoc`]: catalog, page tree, pages, annotations, names and outlines.

use crate::doc::{FontKind, PdfDoc};
use crate::embed::write_font;
use crate::nav::{date_digits, Bookmark, Dest, Link, LinkTarget};
use crate::objects::{stream, Objects};
use crate::syntax::{literal_string, name, pt, text_string};
use crate::tags::TagKeys;
use crate::trees::{write_tree, Entry, MAX_ARRAY};
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use std::collections::BTreeMap;

fn walk(items: &[Bookmark], pages: usize) -> bool {
    items
        .iter()
        .all(|b| b.dest.page < pages && walk(&b.children, pages))
}

fn bytes_join(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

impl PdfDoc {
    /// Serialise the document to PDF bytes. Streams are stored uncompressed. The same document
    /// always produces the same bytes; the file `/ID` is the first 16 bytes of the SHA-256 of
    /// everything written before the cross-reference table.
    ///
    /// # Errors
    /// `Invalid` for a document without pages, a link or bookmark that points at a missing page
    /// or named destination; `Limit` when the output exceeds `Limits::max_file_bytes`; plus any
    /// font error from subsetting.
    pub fn build(&self) -> Result<Vec<u8>> {
        if self.pages.is_empty() {
            return Err(FolioError::Invalid("document has no pages"));
        }
        self.check_refs()?;
        let mut o = Objects::default();
        let catalog = o.alloc()?;
        let tree = o.alloc()?;
        let info = o.alloc()?;
        let page_ids = self
            .pages
            .iter()
            .map(|_| o.alloc())
            .collect::<Result<Vec<_>>>()?;
        let font_ids = self.write_fonts(&mut o)?;
        let keys = self.tag_keys()?;
        let annot_ids = self.write_pages(&mut o, tree, (&page_ids, &font_ids, &keys))?;
        let mut extra = self.write_nav(&mut o, &page_ids)?;
        if keys.enabled {
            let root = self.write_structure(&mut o, &page_ids, &annot_ids, &keys)?;
            write!(
                extra,
                " /MarkInfo << /Marked true >> /StructTreeRoot {root} 0 R"
            )
            .ok();
            // The viewer is asked to show the title only when there is one.
            if self.info.title.is_some() {
                extra.push_str(" /ViewerPreferences << /DisplayDocTitle true >>");
            }
        }
        o.set(
            catalog,
            format!("<< /Type /Catalog /Pages {tree} 0 R{extra} >>").into_bytes(),
        )?;
        o.set(info, self.info_dict()?)?;
        o.serialize(catalog, info, self.limits.max_file_bytes())
    }

    /// Write every page and the page tree node `tree`; returns the annotation object of each
    /// `(page, link index)`. A tree with more than 8191 pages gets intermediate `/Pages` nodes,
    /// so no `/Kids` array outgrows the implementation limit.
    fn write_pages(
        &self,
        o: &mut Objects,
        tree: usize,
        (page_ids, font_ids, keys): (&[usize], &[Option<usize>], &TagKeys),
    ) -> Result<BTreeMap<(usize, usize), usize>> {
        let group = if page_ids.len() > MAX_ARRAY {
            4096
        } else {
            page_ids.len().max(1)
        };
        let mut groups: Vec<(usize, Vec<usize>)> = Vec::new();
        if page_ids.len() > MAX_ARRAY {
            for chunk in page_ids.chunks(group) {
                groups.push((o.alloc()?, chunk.to_vec()));
            }
        }
        let mut annot_ids = BTreeMap::new();
        for (i, id) in page_ids.iter().enumerate() {
            let parent = groups
                .get(i.checked_div(group).unwrap_or(0))
                .map_or(tree, |g| g.0);
            let ids = self.write_page(o, i, *id, parent, (page_ids, font_ids, keys))?;
            for (k, a) in ids.into_iter().enumerate() {
                annot_ids.insert((i, k), a);
            }
        }
        let mut kids = String::new();
        if groups.is_empty() {
            for k in page_ids {
                write!(kids, "{k} 0 R ").ok();
            }
        } else {
            for (gid, pages) in &groups {
                write!(kids, "{gid} 0 R ").ok();
                let mut inner = String::new();
                for p in pages {
                    write!(inner, "{p} 0 R ").ok();
                }
                let body = format!(
                    "<< /Type /Pages /Parent {tree} 0 R /Kids [{inner}] /Count {} >>",
                    pages.len()
                );
                o.set(*gid, body.into_bytes())?;
            }
        }
        let body = format!(
            "<< /Type /Pages /Kids [{kids}] /Count {} >>",
            page_ids.len()
        );
        o.set(tree, body.into_bytes())?;
        Ok(annot_ids)
    }

    fn check_refs(&self) -> Result<()> {
        let pages = self.pages.len();
        let dest_ok = |d: &Dest| d.page < pages;
        let bad = FolioError::Invalid("destination refers to a missing page");
        if !self.dests.values().all(dest_ok) {
            return Err(bad);
        }
        for l in self.pages.iter().flat_map(|p| p.links.iter()) {
            match &l.target {
                LinkTarget::Position(d) if !dest_ok(d) => return Err(bad),
                LinkTarget::Named(n) if !self.dests.contains_key(n) => {
                    return Err(FolioError::Invalid("link names an unknown destination"));
                }
                _ => {}
            }
        }
        if self.labels.iter().any(|(i, _)| *i >= pages) {
            return Err(FolioError::Invalid("page label refers to a missing page"));
        }
        if walk(&self.outline, pages) {
            Ok(())
        } else {
            Err(bad)
        }
    }

    fn info_dict(&self) -> Result<Vec<u8>> {
        let i = &self.info;
        let mut out = b"<<".to_vec();
        for (key, v) in [
            ("Title", &i.title),
            ("Author", &i.author),
            ("Subject", &i.subject),
            ("Keywords", &i.keywords),
        ] {
            if let Some(s) = v {
                out.extend_from_slice(format!(" /{key} ").as_bytes());
                out.extend_from_slice(&text_string(s));
            }
        }
        let creator = i.creator.as_deref().unwrap_or("moonshot-folio");
        out.extend_from_slice(b" /Creator ");
        out.extend_from_slice(&text_string(creator));
        out.extend_from_slice(b" /Producer (moonshot-folio-pdf)");
        if let Some(d) = &i.creation_date {
            out.extend_from_slice(
                format!(" /CreationDate (D:{}000000Z)", date_digits(d)?).as_bytes(),
            );
        }
        out.extend_from_slice(b" >>");
        Ok(out)
    }

    /// Write every font some page uses; returns the object number per registered font.
    fn write_fonts(&self, o: &mut Objects) -> Result<Vec<Option<usize>>> {
        let mut ids = Vec::new();
        for (idx, kind) in self.fonts.iter().enumerate() {
            if !self.pages.iter().any(|p| p.fonts.contains(&idx)) {
                ids.push(None);
                continue;
            }
            let id = match kind {
                FontKind::Std(f) => o.add(
                    format!(
                        "<< /Type /Font /Subtype /Type1 /BaseFont {} /Encoding /WinAnsiEncoding >>",
                        name(f.base_font())
                    )
                    .into_bytes(),
                )?,
                FontKind::Ttf { data, used, .. } => write_font(o, data, used, &self.limits)?,
            };
            ids.push(Some(id));
        }
        Ok(ids)
    }

    fn write_page(
        &self,
        o: &mut Objects,
        index: usize,
        id: usize,
        tree: usize,
        (page_ids, font_ids, keys): (&[usize], &[Option<usize>], &TagKeys),
    ) -> Result<Vec<usize>> {
        let p = self
            .pages
            .get(index)
            .ok_or(FolioError::Invalid("unknown page"))?;
        let content = o.add(stream("", &p.content))?;
        let mut fonts = String::new();
        for f in &p.fonts {
            if let Some(n) = font_ids.get(*f).copied().flatten() {
                write!(fonts, " /F{} {n} 0 R", f.saturating_add(1)).ok();
            }
        }
        let mut annots = String::new();
        let mut annot_ids = Vec::new();
        for (k, l) in p.links.iter().enumerate() {
            let key = keys.annot.get(&(index, k)).copied();
            let a = o.add(annot(l, page_ids, keys.enabled, key)?)?;
            write!(annots, "{a} 0 R ").ok();
            annot_ids.push(a);
        }
        let tagged = if keys.enabled {
            let parents = keys
                .page
                .get(index)
                .copied()
                .flatten()
                .map_or(String::new(), |k| format!(" /StructParents {k}"));
            format!("{parents} /Tabs /S")
        } else {
            String::new()
        };
        let annots = if annots.is_empty() {
            String::new()
        } else {
            format!(" /Annots [{annots}]")
        };
        let body = format!(
            "<< /Type /Page /Parent {tree} 0 R /MediaBox [0 0 {} {}] \
/Resources << /Font <<{fonts} >> >> /Contents {content} 0 R{annots}{tagged} >>",
            pt(p.width)?,
            pt(p.height)?
        );
        o.set(id, body.into_bytes())?;
        Ok(annot_ids)
    }

    /// Names tree and outlines; returns the extra catalog entries.
    fn write_nav(&self, o: &mut Objects, page_ids: &[usize]) -> Result<String> {
        let mut extra = String::new();
        if !self.dests.is_empty() {
            let mut entries: Vec<Entry> = Vec::new();
            for (n, d) in &self.dests {
                let lit = literal_string(n.as_bytes());
                let mut body = lit.clone();
                body.extend_from_slice(format!(" {} ", dest_array(d, page_ids)?).as_bytes());
                entries.push((lit, body));
            }
            let id = write_tree(o, "Names", &entries)?;
            write!(extra, " /Names << /Dests {id} 0 R >>").ok();
        }
        if !self.labels.is_empty() {
            let mut nums = String::new();
            for (i, n) in &self.labels {
                write!(nums, "{i} << /S /D /St {n} >> ").ok();
            }
            write!(extra, " /PageLabels << /Nums [{nums}] >>").ok();
        }
        if let Some(lang) = &self.info.lang {
            extra.push_str(" /Lang ");
            extra.push_str(&String::from_utf8_lossy(&text_string(lang)));
        }
        if !self.outline.is_empty() {
            let root = o.alloc()?;
            let (first, last, count) = write_outline(o, &self.outline, root, page_ids)?;
            o.set(
                root,
                format!("<< /Type /Outlines /First {first} 0 R /Last {last} 0 R /Count {count} >>")
                    .into_bytes(),
            )?;
            write!(extra, " /Outlines {root} 0 R /PageMode /UseOutlines").ok();
        }
        Ok(extra)
    }
}

fn annot(l: &Link, page_ids: &[usize], tagged: bool, key: Option<usize>) -> Result<Vec<u8>> {
    let (x2, y2) = (
        l.rect.x.checked_add(l.rect.w)?,
        l.rect.y.checked_add(l.rect.h)?,
    );
    let head = format!(
        "<< /Type /Annot /Subtype /Link /Rect [{} {} {} {}] /Border [0 0 0] ",
        pt(l.rect.x)?,
        pt(l.rect.y)?,
        pt(x2)?,
        pt(y2)?
    );
    let mut head = head;
    if tagged {
        head.push_str("/F 4 ");
    }
    if let Some(c) = &l.contents {
        head.push_str("/Contents ");
        head.push_str(&String::from_utf8_lossy(&text_string(c)));
        head.push(' ');
    }
    if let Some(k) = key {
        write!(head, "/StructParent {k} ").ok();
    }
    let tail: Vec<u8> = match &l.target {
        LinkTarget::Uri(u) => bytes_join(&[
            b"/A << /S /URI /URI ",
            &literal_string(u.as_bytes()),
            b" >>",
        ]),
        LinkTarget::Named(n) => {
            bytes_join(&[b"/A << /S /GoTo /D ", &literal_string(n.as_bytes()), b" >>"])
        }
        LinkTarget::Position(d) => format!("/Dest {}", dest_array(d, page_ids)?).into_bytes(),
    };
    Ok(bytes_join(&[head.as_bytes(), &tail, b" >>"]))
}

fn dest_array(d: &Dest, page_ids: &[usize]) -> Result<String> {
    let page = page_ids
        .get(d.page)
        .ok_or(FolioError::Invalid("destination refers to a missing page"))?;
    Ok(format!("[{page} 0 R /XYZ {} {} null]", pt(d.x)?, pt(d.y)?))
}

/// Write one sibling level; returns `(first id, last id, total descendants including siblings)`.
fn write_outline(
    o: &mut Objects,
    items: &[Bookmark],
    parent: usize,
    page_ids: &[usize],
) -> Result<(usize, usize, usize)> {
    let ids = items
        .iter()
        .map(|_| o.alloc())
        .collect::<Result<Vec<_>>>()?;
    let mut total = items.len();
    for (i, (b, id)) in items.iter().zip(&ids).enumerate() {
        let mut body = b"<< /Title ".to_vec();
        body.extend_from_slice(&text_string(&b.title));
        body.extend_from_slice(format!(" /Parent {parent} 0 R").as_bytes());
        if let Some(prev) = i.checked_sub(1).and_then(|k| ids.get(k)) {
            body.extend_from_slice(format!(" /Prev {prev} 0 R").as_bytes());
        }
        if let Some(next) = ids.get(i.saturating_add(1)) {
            body.extend_from_slice(format!(" /Next {next} 0 R").as_bytes());
        }
        if !b.children.is_empty() {
            let (f, l, c) = write_outline(o, &b.children, *id, page_ids)?;
            total = total.saturating_add(c);
            body.extend_from_slice(format!(" /First {f} 0 R /Last {l} 0 R /Count {c}").as_bytes());
        }
        body.extend_from_slice(format!(" /Dest {} >>", dest_array(&b.dest, page_ids)?).as_bytes());
        o.set(*id, body)?;
    }
    let first = ids
        .first()
        .copied()
        .ok_or(FolioError::Invalid("empty outline level"))?;
    let last = ids
        .last()
        .copied()
        .ok_or(FolioError::Invalid("empty outline level"))?;
    Ok((first, last, total))
}
