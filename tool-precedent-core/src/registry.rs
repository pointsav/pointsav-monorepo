// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The unified typesetting registry (operator decisions P-34..P-51, project-documents): the data tables
//! `precedent/registry/ts-*.csv` that say, once, how every class of every document family looks.
//!   ts-classes.csv   the closed class list (element, kind, families, aliases, Word style names, label and glyph links)
//!   ts-geometry.csv  one row per CSS rule of a family token or component (and the few Word-only rules)
//!   ts-labels.csv    label forms ((a), (i), A., the heading number ...) and their widths
//!   ts-glyphs.csv    typed glyphs (bullets, ballot box, placeholder sentinel)
//!   ts-sections.csv  page, folio, stamp, running-header rules      ts-families.csv  one row per document family
//!   ts-engine.csv    renderer corrections                          ts-changes.csv   change history
//! Zero dependencies, read-only: this module never writes into a template or a document. It is the Rust side of
//! project-documents `bin/script-typeset-reference.py` (css, dtcg) and `bin/script-typeset-lint.py` (the class rules),
//! byte-for-byte on the CSS and DTCG output (the parity tests compare with goldens made by those scripts).
//! Commands (`registry <sub>`): check | classes | css | dtcg | lint.
use crate::csv::parse as parse_csv;
use crate::fence::hash as fnv;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub type Row = BTreeMap<String, String>;

const TABLES: &[&str] = &[
    "classes", "geometry", "labels", "glyphs", "sections", "families", "engine", "changes",
];
const CLASS_STATUS: &[&str] = &["current", "alias", "retired", "word-only", "proposed"];
const GEOM_STATUS: &[&str] = &["current", "alias", "retired", "word-only", "proposed"];
const LABEL_STATUS: &[&str] = &["current", "review", "proposed"];
const GLYPH_STATUS: &[&str] = &["current", "proposed"];
const SECTION_STATUS: &[&str] = &["current", "reference"];
const FAMILY_STATUS: &[&str] = &["current", "reference"];
/// ts-families.csv columns a child family takes from its parent when its own cell is blank.
const FAMILY_INHERIT: &[&str] = &[
    "body_font_family",
    "word_font",
    "body_text_align",
    "body_color",
    "p_margin_bottom",
    "orphans",
    "widows",
    "lang",
    "hyphens",
    "page_size",
    "page_margin",
    "word_reference_source",
    "word_builder",
    "pdf_builder",
    "lua_filter",
    "word_defaults_cascade",
    "status",
];
const CHANGE_STATUS: &[&str] = &["applied", "pending-css"];
const GEOM_SOURCES: &[&str] = &["css", "word", "registry", "page", "ua"];
/// Geometry columns that become CSS declarations (and DTCG tokens), in output order.
pub const GEOM_COLS: &[&str] = &[
    "font_size",
    "line_height",
    "font_weight",
    "font_style",
    "text_align",
    "text_transform",
    "letter_spacing",
    "color",
    "margin_top",
    "margin_right",
    "margin_bottom",
    "margin_left",
    "padding_top",
    "padding_right",
    "padding_bottom",
    "padding_left",
    "border_top",
    "border_right",
    "border_bottom",
    "border_left",
    "background",
    "width",
    "display",
    "keep_next",
    "keep_together",
    "break_before",
    "break_after",
    "orphans",
    "widows",
    "hyphens",
];
const SIDES: [&str; 4] = ["top", "right", "bottom", "left"];
const HTML_ELEMENTS: &[&str] = &[
    "a", "body", "br", "div", "em", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "li", "mark", "ol",
    "p", "span", "strong", "table", "tbody", "td", "th", "thead", "tr", "ul", "col", "colgroup",
    "sup", "sub", "u", "i", "b",
];
pub const CSS_BEGIN: &str = "/* @generated ts-geometry begin — from precedent/registry/ts-geometry.csv by bin/script-typeset-reference.py; do not edit */\n";
pub const CSS_END: &str = "/* @generated ts-geometry end */\n";

fn g<'a>(r: &'a Row, k: &str) -> &'a str {
    r.get(k).map(|s| s.as_str()).unwrap_or("")
}
fn ident(r: &Row) -> String {
    for k in [
        "id", "class", "label_id", "glyph_id", "section", "family", "row_id",
    ] {
        if !g(r, k).is_empty() {
            return g(r, k).to_string();
        }
    }
    String::from("(row)")
}
fn toks(s: &str) -> Vec<&str> {
    s.split(';')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub struct Registry {
    pub classes: Vec<Row>,
    pub geometry: Vec<Row>,
    pub labels: Vec<Row>,
    pub glyphs: Vec<Row>,
    pub sections: Vec<Row>,
    pub families: Vec<Row>,
    pub engine: Vec<Row>,
    pub changes: Vec<Row>,
}

pub struct Finding {
    pub error: bool,
    pub msg: String,
}

fn read_table(path: &Path) -> Result<Vec<Row>, String> {
    let s = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut rows = parse_csv(&s)
        .into_iter()
        .filter(|r| !(r.len() == 1 && r[0].is_empty()));
    let head = rows
        .next()
        .ok_or_else(|| format!("{}: empty", path.display()))?;
    Ok(rows
        .map(|r| {
            head.iter()
                .enumerate()
                .map(|(i, h)| (h.clone(), r.get(i).cloned().unwrap_or_default()))
                .collect()
        })
        .collect())
}

impl Registry {
    /// Loads `ts-*.csv` from `dir` (normally `<root>/precedent/registry`).
    pub fn load(dir: &Path) -> Result<Registry, String> {
        let t = |n: &str| read_table(&dir.join(format!("ts-{n}.csv")));
        debug_assert_eq!(TABLES.len(), 8);
        let mut families = t("families")?;
        Self::inherit_families(&mut families);
        Ok(Registry {
            classes: t("classes")?,
            geometry: t("geometry")?,
            labels: t("labels")?,
            glyphs: t("glyphs")?,
            sections: t("sections")?,
            families,
            engine: t("engine")?,
            changes: t("changes")?,
        })
    }

    /// Components whose whole CSS file is generated (engine key `css.own-file-components`): their hand-written
    /// declarations are written into the generated rule, and `check` proves the file still contains the block.
    pub fn own_file_components(&self) -> Vec<String> {
        self.engine
            .iter()
            .filter(|r| g(r, "key") == "css.own-file-components")
            .flat_map(|r| {
                toks(g(r, "value"))
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    // ------------------------------------------------------------------------------------------------ CSS
    /// The generated geometry block of one registry row: CSS declarations, both break spellings (as the tokens use).
    pub fn geom_to_css(r: &Row) -> Vec<String> {
        let mut out = vec![];
        let v = |k: &str| -> Option<&str> {
            let s = g(r, k);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        for (k, p) in [
            ("font_size", "font-size"),
            ("line_height", "line-height"),
            ("font_weight", "font-weight"),
            ("font_style", "font-style"),
            ("text_align", "text-align"),
            ("text_transform", "text-transform"),
            ("letter_spacing", "letter-spacing"),
            ("color", "color"),
        ] {
            if let Some(x) = v(k) {
                out.push(format!("{p}:{x}"));
            }
        }
        for boxn in ["margin", "padding"] {
            let vals: Vec<&str> = SIDES.iter().map(|s| g(r, &format!("{boxn}_{s}"))).collect();
            if vals.iter().all(|x| !x.is_empty()) {
                out.push(format!("{boxn}:{}", vals.join(" ")));
            } else {
                for (s, x) in SIDES.iter().zip(&vals) {
                    if !x.is_empty() {
                        out.push(format!("{boxn}-{s}:{x}"));
                    }
                }
            }
        }
        let b: Vec<&str> = SIDES.iter().map(|s| g(r, &format!("border_{s}"))).collect();
        if b.iter().all(|x| !x.is_empty()) && b.iter().all(|x| *x == b[0]) {
            out.push(format!("border:{}", b[0]));
        } else {
            for (s, x) in SIDES.iter().zip(&b) {
                if !x.is_empty() {
                    out.push(format!("border-{s}:{x}"));
                }
            }
        }
        for k in ["background", "width", "display"] {
            if let Some(x) = v(k) {
                out.push(format!("{k}:{x}"));
            }
        }
        if let Some(x) = v("break_before") {
            out.push(format!("break-before:{x}"));
            out.push(format!(
                "page-break-before:{}",
                if x == "page" { "always" } else { x }
            ));
        }
        if v("break_after").is_some() {
            out.push("break-after:page".into());
            out.push("page-break-after:always".into());
        }
        if let Some(x) = v("keep_next") {
            out.push(format!("break-after:{x}"));
            out.push(format!("page-break-after:{x}"));
        }
        if let Some(x) = v("keep_together") {
            out.push(format!("break-inside:{x}"));
            out.push(format!("page-break-inside:{x}"));
        }
        for k in ["orphans", "widows", "hyphens"] {
            if let Some(x) = v(k) {
                out.push(format!("{k}:{x}"));
            }
        }
        out
    }

    /// Family inheritance (ts-families.csv column `parent`): a blank inheritable cell of a child takes the parent's value.
    fn inherit_families(rows: &mut [Row]) {
        let snapshot: Vec<Row> = rows.to_vec();
        for r in rows.iter_mut() {
            let mut cur = g(r, "parent").to_string();
            let mut guard = 0;
            while !cur.is_empty() && guard < 8 {
                guard += 1;
                let Some(p) = snapshot.iter().find(|x| g(x, "family") == cur) else {
                    break;
                };
                for k in FAMILY_INHERIT {
                    if g(r, k).is_empty() && !g(p, k).is_empty() {
                        r.insert((*k).to_string(), g(p, k).to_string());
                    }
                }
                cur = g(p, "parent").to_string();
            }
        }
    }

    /// The family and its ancestors, nearest first (`fideicomiso` -> `fideicomiso`, `mexico`).
    pub fn lineage(&self, family: &str) -> Vec<String> {
        let mut out = vec![family.to_string()];
        let mut cur = family.to_string();
        while let Some(r) = self.families.iter().find(|x| g(x, "family") == cur) {
            let p = g(r, "parent");
            if p.is_empty() || out.iter().any(|x| x == p) {
                break;
            }
            out.push(p.to_string());
            cur = p.to_string();
        }
        out
    }

    /// True when a `;` list of family tokens applies to `family`: the family itself, one of its ancestors, or `all`.
    pub fn applies(&self, tokens: &str, family: &str) -> bool {
        let lin = self.lineage(family);
        toks(tokens)
            .iter()
            .any(|t| *t == "all" || lin.iter().any(|l| l == t))
    }

    fn css_block(&self, r: &Row, own: &[String]) -> String {
        let mut decl = Self::geom_to_css(r);
        if own.iter().any(|c| c == g(r, "component")) && !g(r, "handwritten").is_empty() {
            decl.push(g(r, "handwritten").to_string());
        }
        let decl = decl.join(";");
        let sel = g(r, "selector");
        if decl.is_empty() {
            format!("/* {sel}: no generated geometry */")
        } else {
            format!("{sel}{{{decl};}}")
        }
    }

    /// The CSS-sourced rows that make up the token CSS of a family: its own rows and those of its ancestors (ts-families
    /// `parent`), in the order of the family's token file (line of the evidence in `token_css`; a family without ancestors keeps the table order).
    fn effective_rows(&self, family: &str) -> Vec<&Row> {
        let lin = self.lineage(family);
        let stems: Vec<&str> = lin
            .iter()
            .filter_map(|f| self.families.iter().find(|r| g(r, "family") == f))
            .map(|r| g(r, "token_stem"))
            .collect();
        let tok = self
            .families
            .iter()
            .find(|r| g(r, "family") == family)
            .map(|r| g(r, "token_css"))
            .unwrap_or("");
        let line = |r: &Row| -> usize {
            let ev: Vec<&str> = g(r, "evidence").split(';').collect();
            let hit = ev
                .iter()
                .find(|e| e.rsplit_once(':').map(|(p, _)| p == tok).unwrap_or(false))
                .or_else(|| ev.first());
            hit.and_then(|e| e.rsplit_once(':'))
                .and_then(|(_, l)| l.parse::<usize>().ok())
                .unwrap_or(0)
        };
        let mut rows: Vec<&Row> = self
            .geometry
            .iter()
            .filter(|r| {
                matches!(g(r, "source"), "css" | "registry")
                    && g(r, "status") != "retired"
                    && stems.contains(&g(r, "component"))
            })
            .collect();
        if lin.len() > 1 {
            rows.sort_by_key(|r| line(r));
        }
        rows
    }

    /// (component, generated CSS file text) for every component with CSS-sourced rows, in first-seen order; `only` selects one.
    /// A family token component (ts-families `token_stem` with a `token_css`) is written whole: its own rows and its ancestors'.
    /// The abstract parent of a family (no `token_css`) has no file of its own.
    pub fn css_files(&self, only: Option<&str>) -> Vec<(String, String)> {
        let own = self.own_file_components();
        let fam_of_stem: BTreeMap<&str, &str> = self
            .families
            .iter()
            .filter(|r| !g(r, "token_css").is_empty())
            .map(|r| (g(r, "token_stem"), g(r, "family")))
            .collect();
        let abstract_stems: Vec<&str> = self
            .families
            .iter()
            .filter(|r| g(r, "token_css").is_empty())
            .map(|r| g(r, "token_stem"))
            .collect();
        let mut order: Vec<String> = vec![];
        let mut blocks: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for r in &self.geometry {
            if !matches!(g(r, "source"), "css" | "registry") || g(r, "status") == "retired" {
                continue;
            }
            let comp = g(r, "component");
            if abstract_stems.contains(&comp) {
                continue;
            }
            if only.map(|c| c != comp).unwrap_or(false) {
                continue;
            }
            if blocks.contains_key(comp) && fam_of_stem.contains_key(comp) {
                continue;
            }
            if !blocks.contains_key(comp) {
                order.push(comp.to_string());
            }
            if let Some(fam) = fam_of_stem.get(comp) {
                let rows = self.effective_rows(fam);
                blocks.insert(
                    comp.to_string(),
                    rows.iter().map(|x| self.css_block(x, &own)).collect(),
                );
                continue;
            }
            let b = self.css_block(r, &own);
            blocks.entry(comp.to_string()).or_default().push(b);
        }
        order
            .into_iter()
            .map(|c| {
                let b = blocks.remove(&c).unwrap_or_default();
                let text = format!("{CSS_BEGIN}{}\n{CSS_END}", b.join("\n"));
                (c, text)
            })
            .collect()
    }

    // ------------------------------------------------------------------------------------------------ class list
    /// The closed class list as tab-separated text: class, element, kind, status, aliases, families (`family`: that family and `all`).
    pub fn classes_list(&self, family: Option<&str>) -> String {
        let mut o = String::from("# class\telement\tkind\tstatus\taliases\tfamilies\n");
        for r in &self.classes {
            if let Some(f) = family {
                if !self.applies(g(r, "families"), f) {
                    continue;
                }
            }
            o.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\n",
                g(r, "class"),
                g(r, "element"),
                g(r, "kind"),
                g(r, "status"),
                g(r, "aliases"),
                g(r, "families")
            ));
        }
        o
    }

    // ------------------------------------------------------------------------------------------------ check
    pub fn check(&self, root: &Path) -> Vec<Finding> {
        let mut f: Vec<Finding> = vec![];
        let mut err = |m: String| {
            f.push(Finding {
                error: true,
                msg: m,
            })
        };
        let class_names: Vec<&str> = self.classes.iter().map(|r| g(r, "class")).collect();
        let class_first: Vec<&str> = class_names
            .iter()
            .map(|c| c.split_whitespace().next().unwrap_or(""))
            .collect();
        let mut alias_names: Vec<String> = vec![];
        for r in &self.classes {
            for a in toks(g(r, "aliases")) {
                alias_names.push(a.rsplit('.').next().unwrap_or(a).to_string());
            }
        }
        let fam_names: Vec<&str> = self.families.iter().map(|r| g(r, "family")).collect();
        let glyph_ids: Vec<&str> = self.glyphs.iter().map(|r| g(r, "glyph_id")).collect();
        let label_classes: Vec<&str> = self
            .labels
            .iter()
            .flat_map(|r| {
                g(r, "label_class")
                    .split('/')
                    .map(|x| x.split_whitespace().next().unwrap_or(""))
                    .collect::<Vec<_>>()
            })
            .collect();
        let geom_ids: Vec<&str> = self
            .geometry
            .iter()
            .map(|r| g(r, "id").split('#').next().unwrap_or(""))
            .collect();
        // statuses
        for (name, rows, set) in [
            ("classes", &self.classes, CLASS_STATUS),
            ("geometry", &self.geometry, GEOM_STATUS),
            ("labels", &self.labels, LABEL_STATUS),
            ("glyphs", &self.glyphs, GLYPH_STATUS),
            ("sections", &self.sections, SECTION_STATUS),
            ("families", &self.families, FAMILY_STATUS),
            ("changes", &self.changes, CHANGE_STATUS),
        ] {
            for r in rows.iter() {
                let s = g(r, "status");
                if !set.contains(&s) {
                    err(format!(
                        "ts-{name}.csv: status '{s}' is not in the closed set ({}) at {}",
                        set.join("|"),
                        ident(r)
                    ));
                }
            }
        }
        // classes
        for r in &self.classes {
            let c = g(r, "class");
            let a = g(r, "alias_of");
            if g(r, "status") == "alias" && a.is_empty() {
                err(format!("ts-classes.csv: alias class '{c}' has no alias_of"));
            }
            if !a.is_empty() && !class_first.contains(&a) && !alias_names.iter().any(|x| x == a) {
                err(format!(
                    "ts-classes.csv: '{c}' alias_of '{a}' is not a class"
                ));
            }
            for al in toks(g(r, "aliases")) {
                let (tag, name) = match al.rsplit_once('.') {
                    Some((t, n)) => (t, n),
                    None => ("", al),
                };
                if !tag.is_empty() && !(class_first.contains(&name) || name == c) {
                    err(format!(
                        "ts-classes.csv: '{c}' alias '{al}' names an unknown class"
                    ));
                }
            }
            for l in toks(g(r, "label")) {
                if !label_classes.contains(&l) {
                    err(format!(
                        "ts-classes.csv: '{c}' label '{l}' is not a label_class in ts-labels.csv"
                    ));
                }
            }
            for gl in toks(g(r, "glyph")) {
                if !glyph_ids.contains(&gl) {
                    err(format!(
                        "ts-classes.csv: '{c}' glyph '{gl}' is not in ts-glyphs.csv"
                    ));
                }
            }
            for k in ["families", "word_families"] {
                for t in toks(g(r, k)) {
                    if t != "all" && !fam_names.contains(&t) {
                        err(format!(
                            "ts-classes.csv: '{c}' {k} names unknown family '{t}'"
                        ));
                    }
                }
            }
        }
        // geometry
        let mut seen: Vec<&str> = vec![];
        let elems: Vec<String> = self
            .classes
            .iter()
            .map(|r| g(r, "element").to_string())
            .chain(HTML_ELEMENTS.iter().map(|s| s.to_string()))
            .collect();
        for r in &self.geometry {
            let id = g(r, "id");
            if seen.contains(&id) {
                err(format!("ts-geometry.csv: duplicate id '{id}'"));
            }
            seen.push(id);
            if !GEOM_SOURCES.contains(&g(r, "source")) {
                err(format!(
                    "ts-geometry.csv: {id}: source '{}' is not in the closed set",
                    g(r, "source")
                ));
            }
            for t in toks(g(r, "family")) {
                if t != "all" && !fam_names.contains(&t) {
                    err(format!("ts-geometry.csv: {id}: unknown family '{t}'"));
                }
            }
            let cl = g(r, "class");
            if !cl.is_empty() && !matches!(g(r, "source"), "ua") {
                let base = cl.trim_start_matches("(word) ");
                let first = base.split('.').next().unwrap_or(base);
                let ok = class_names.contains(&cl)
                    || class_first.contains(&base)
                    || class_first.contains(&first)
                    || alias_names.iter().any(|x| x == base || x == first)
                    || elems.iter().any(|e| e == first)
                    || g(r, "source") == "word";
                if !ok {
                    err(format!(
                        "ts-geometry.csv: {id}: class '{cl}' is not in ts-classes.csv"
                    ));
                }
            }
            let a = g(r, "alias_of");
            if !a.is_empty()
                && !class_first.contains(&a)
                && !alias_names.iter().any(|x| x == a)
                && !geom_ids.contains(&a)
            {
                err(format!(
                    "ts-geometry.csv: {id}: alias_of '{a}' resolves to no class or row"
                ));
            }
        }
        // components.csv: files and hashes; generated own-file components reproduce from the table
        let comps = std::fs::read_to_string(root.join("precedent/components.csv"))
            .map(|s| parse_csv(&s))
            .unwrap_or_default();
        let mut comp_ids: Vec<String> = vec![];
        let mut comp_file: BTreeMap<String, String> = BTreeMap::new();
        if let Some((head, rows)) = comps.split_first() {
            let ix = |n: &str| head.iter().position(|h| h == n);
            for r in rows.iter().filter(|r| r.len() > 4) {
                let (id, file, hash) = (
                    &r[ix("id").unwrap_or(0)],
                    &r[ix("file").unwrap_or(2)],
                    &r[ix("hash").unwrap_or(4)],
                );
                comp_ids.push(id.clone());
                comp_file.insert(id.clone(), file.clone());
                match std::fs::read_to_string(root.join(file)) {
                    Err(e) => err(format!("components.csv: {id}: {file}: {e}")),
                    Ok(t) => {
                        if fnv(t.trim_end()) != *hash {
                            err(format!("components.csv: {id}: hash differs from {file} (file changed: re-run, then update the row)"));
                        }
                    }
                }
            }
        } else {
            err("precedent/components.csv missing or empty".into());
        }
        for r in &self.families {
            let p = g(r, "parent");
            if !p.is_empty() && !fam_names.contains(&p) {
                err(format!(
                    "ts-families.csv: {}: parent '{p}' is not a family",
                    g(r, "family")
                ));
            }
            let lin = self.lineage(g(r, "family"));
            if let Some(last) = lin.last() {
                if let Some(lr) = self.families.iter().find(|x| g(x, "family") == last) {
                    if !g(lr, "parent").is_empty() {
                        err(format!(
                            "ts-families.csv: {}: parent chain loops",
                            g(r, "family")
                        ));
                    }
                }
            }
            comp_file.insert(
                g(r, "token_stem").to_string(),
                g(r, "token_css").to_string(),
            );
        }
        let mut seen_comp: Vec<&str> = vec![];
        for r in &self.geometry {
            let c = g(r, "component");
            if seen_comp.contains(&c) {
                continue;
            }
            seen_comp.push(c);
            let css_rows = self
                .geometry
                .iter()
                .any(|x| g(x, "component") == c && g(x, "source") == "css");
            if css_rows && !comp_file.contains_key(c) {
                err(format!("ts-geometry.csv: component '{c}' has css rows but is neither in components.csv nor a family token (ts-families token_stem)"));
            }
        }
        for (c, text) in self
            .own_file_components()
            .iter()
            .map(|c| (c.clone(), self.css_files(Some(c))))
            .filter_map(|(c, v)| v.into_iter().next().map(|(_, t)| (c, t)))
        {
            let Some(file) = comp_file.get(&c) else {
                err(format!(
                    "engine css.own-file-components: '{c}' is not in components.csv"
                ));
                continue;
            };
            match std::fs::read_to_string(root.join(file)) {
                Err(e) => err(format!("{file}: {e}")),
                Ok(body) => {
                    if !body.contains(&text) {
                        err(format!("{file}: the generated block differs from ts-geometry.csv (re-run registry css --component {c})"));
                    }
                    let lines: Vec<&str> = body.lines().collect();
                    for r in self
                        .geometry
                        .iter()
                        .filter(|r| g(r, "component") == c && g(r, "source") == "css")
                    {
                        let ev = g(r, "evidence").split(';').next().unwrap_or("");
                        match ev
                            .rsplit_once(':')
                            .and_then(|(p, n)| n.parse::<usize>().ok().map(|n| (p, n)))
                        {
                            Some((p, n)) if file.ends_with(p) || p.ends_with(file.as_str()) => {
                                if !lines
                                    .get(n - 1)
                                    .map(|l| l.starts_with(&format!("{}{{", g(r, "selector"))))
                                    .unwrap_or(false)
                                {
                                    err(format!("ts-geometry.csv: {}: evidence line {n} of {file} does not start with '{}{{'", g(r, "id"), g(r, "selector")));
                                }
                            }
                            _ => err(format!(
                                "ts-geometry.csv: {}: evidence does not start with {file}:<line>",
                                g(r, "id")
                            )),
                        }
                    }
                }
            }
        }
        f
    }

    // ------------------------------------------------------------------------------------------------ DTCG
    /// Design-token JSON (W3C DTCG draft): dimension / typography / fontWeight tokens; byte-identical to `script-typeset-reference.py dtcg`.
    pub fn dtcg(&self, family: Option<&str>) -> String {
        let mut fam_node = J::obj();
        for f in &self.families {
            let name = g(f, "family");
            if family.map(|x| x != name).unwrap_or(false) {
                continue;
            }
            if g(f, "body_size").is_empty() {
                continue; // an abstract parent family (mexico) has no body of its own
            }
            let size = pf(g(f, "body_size").trim_end_matches(['p', 't']));
            let ff: Vec<J> = g(f, "body_font_family")
                .split(',')
                .map(|x| J::Str(x.trim_matches(|c| c == ' ' || c == '\'').to_string()))
                .collect();
            let body = J::obj().with("$type", J::s("typography")).with(
                "$value",
                J::obj()
                    .with("fontFamily", J::Arr(ff))
                    .with(
                        "fontSize",
                        J::obj()
                            .with("value", J::Raw(fl(round4(size * 4.0 / 3.0))))
                            .with("unit", J::s("px")),
                    )
                    .with("fontWeight", J::Raw("400".into()))
                    .with("lineHeight", J::Raw(fl(pf(g(f, "body_line_height")))))
                    .with(
                        "letterSpacing",
                        J::obj()
                            .with("value", J::Raw("0".into()))
                            .with("unit", J::s("px")),
                    ),
            );
            let margins: Vec<J> = g(f, "page_margin")
                .split_whitespace()
                .map(|x| dim(pf(x.trim_end_matches(['i', 'n'])) * 72.0))
                .collect();
            fam_node.set(
                name,
                J::obj()
                    .with("body", body)
                    .with("page-margin", J::Arr(margins)),
            );
        }
        let mut comp_node = J::obj();
        for r in &self.geometry {
            if g(r, "status") == "retired" || g(r, "source") == "ua" {
                continue;
            }
            if let Some(f) = family {
                if !self.applies(g(r, "family"), f) {
                    continue;
                }
            }
            let key = node_key(g(r, "selector"));
            let comp = comp_node.entry(g(r, "component"));
            let node = comp.entry(&key);
            node.set(
                "$description",
                J::Str(format!(
                    "{} ({}; {})",
                    g(r, "selector"),
                    g(r, "family"),
                    g(r, "evidence")
                )),
            );
            if !g(r, "alias_of").is_empty() {
                node.set(
                    "$extensions",
                    J::obj().with(
                        "com.pointsav.typeset",
                        J::obj()
                            .with("status", J::Str(g(r, "status").into()))
                            .with("alias-of", J::Str(g(r, "alias_of").into())),
                    ),
                );
            }
            for k in GEOM_COLS {
                let v = g(r, k);
                if v.is_empty() {
                    continue;
                }
                let dashed = k.replace('_', "-");
                if let Some(pt) = pt_value(v) {
                    node.set(&dashed, dim(pt));
                } else if *k == "font_weight" {
                    node.set(
                        "font-weight",
                        J::obj().with("$type", J::s("fontWeight")).with(
                            "$value",
                            if v == "bold" {
                                J::Raw("700".into())
                            } else if v == "normal" {
                                J::Raw("400".into())
                            } else {
                                J::Str(v.into())
                            },
                        ),
                    );
                } else if (*k == "orphans" || *k == "widows")
                    && v.chars().all(|c| c.is_ascii_digit())
                {
                    node.set(
                        k,
                        J::obj().with("$type", J::s("number")).with(
                            "$value",
                            J::Raw(
                                v.parse::<u64>()
                                    .map(|n| n.to_string())
                                    .unwrap_or_else(|_| v.to_string()),
                            ),
                        ),
                    );
                } else if *k == "line_height"
                    && !v.is_empty()
                    && v.chars().all(|c| c.is_ascii_digit() || c == '.')
                {
                    node.set(
                        "line-height",
                        J::obj()
                            .with("$type", J::s("number"))
                            .with("$value", J::Raw(fl(pf(v)))),
                    );
                } else if (*k == "color" || *k == "background") && v.starts_with('#') {
                    node.set(
                        k,
                        J::obj()
                            .with("$type", J::s("color"))
                            .with("$value", J::Str(v.into())),
                    );
                } else if v == "0" {
                    node.set(&dashed, dim(0.0));
                } else {
                    node.set(
                        &dashed,
                        J::obj().with("$value", J::Str(v.into())).with(
                            "$extensions",
                            J::obj().with(
                                "com.pointsav.typeset",
                                J::obj().with("css-keyword", J::Raw("true".into())),
                            ),
                        ),
                    );
                }
            }
        }
        let root = J::obj()
            .with("$description", J::s("Woodfine unified typesetting registry (P-34), generated from precedent/registry/ts-geometry.csv. Dimensions in px (DTCG allows px/rem only); the source pt and Word twips ride in $extensions."))
            .with("ts", J::obj().with("family", fam_node).with("component", comp_node));
        let mut out = String::new();
        root.dump(&mut out, 0);
        out
    }

    // ------------------------------------------------------------------------------------------------ lookup (lint)
    /// (status, canonical) of a class name: ("alias", canonical) | ("unknown", None) | (status, name).
    pub fn lookup(&self, cls: &str) -> (String, Option<String>) {
        let (classes, alias_of) = self.class_maps();
        Self::lookup_in(&classes, &alias_of, cls)
    }

    fn lookup_in(
        classes: &BTreeMap<String, &Row>,
        alias_of: &BTreeMap<String, String>,
        cls: &str,
    ) -> (String, Option<String>) {
        if let Some(c) = alias_of.get(cls) {
            return ("alias".into(), Some(c.clone()));
        }
        match classes.get(cls) {
            None => ("unknown".into(), None),
            Some(r) => {
                let s = g(r, "status");
                (
                    if s.is_empty() {
                        "current".into()
                    } else {
                        s.to_string()
                    },
                    Some(cls.to_string()),
                )
            }
        }
    }

    fn class_maps(&self) -> (BTreeMap<String, &Row>, BTreeMap<String, String>) {
        let mut classes: BTreeMap<String, &Row> = BTreeMap::new();
        let mut alias_of: BTreeMap<String, String> = BTreeMap::new();
        for r in &self.classes {
            let raw = g(r, "class");
            if raw.starts_with("(word)") || g(r, "element").starts_with('@') {
                continue;
            }
            let name = raw.split_whitespace().next().unwrap_or("").to_string();
            if g(r, "status") == "alias" && !g(r, "alias_of").is_empty() {
                alias_of.insert(name.clone(), g(r, "alias_of").to_string());
            }
            classes.entry(name.clone()).or_insert(r);
            for a in g(r, "aliases").split(';') {
                let a = a.trim().rsplit('.').next().unwrap_or("").to_string();
                if !a.is_empty() && a != name && g(r, "status") != "alias" {
                    alias_of.entry(a).or_insert_with(|| name.clone());
                }
            }
        }
        (classes, alias_of)
    }

    fn families_in(
        classes: &BTreeMap<String, &Row>,
        alias_of: &BTreeMap<String, String>,
        cls: &str,
    ) -> Vec<String> {
        let key = alias_of
            .get(cls)
            .cloned()
            .unwrap_or_else(|| cls.to_string());
        classes
            .get(&key)
            .map(|r| {
                toks(g(r, "families"))
                    .into_iter()
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Family named by the token header in the document's `<style>` (`constitutional-agreement.css —` ...), the earliest match.
    pub fn detect_family(&self, style_text: &str) -> Option<String> {
        let mut best: Option<(usize, String)> = None;
        for f in &self.families {
            let tok = g(f, "token_css").rsplit('/').next().unwrap_or("");
            if tok.is_empty() {
                continue;
            }
            let mut from = 0;
            while let Some(p) = style_text[from..].find(tok) {
                let at = from + p;
                let before_ok = style_text[..at]
                    .chars()
                    .next_back()
                    .map(|c| !(c.is_alphanumeric() || c == '_'))
                    .unwrap_or(true);
                let rest = style_text[at + tok.len()..].trim_start();
                if before_ok && rest.starts_with('—') {
                    if best.as_ref().map(|b| at < b.0).unwrap_or(true) {
                        best = Some((at, g(f, "family").to_string()));
                    }
                    break;
                }
                from = at + tok.len();
            }
        }
        best.map(|b| b.1)
    }

    /// Class families retired by a registry decision (`ts-changes.csv`: file ts-classes.csv, column families, kind `retired for <family>`).
    fn family_retired(&self) -> Vec<(String, String, String)> {
        self.changes
            .iter()
            .filter(|r| {
                g(r, "file") == "ts-classes.csv"
                    && g(r, "column") == "families"
                    && g(r, "kind").starts_with("retired for ")
            })
            .map(|r| {
                (
                    g(r, "row_id").to_string(),
                    g(r, "kind").trim_start_matches("retired for ").to_string(),
                    g(r, "decision").to_string(),
                )
            })
            .collect()
    }

    /// Lints one authored HTML document against the closed class list (the class rules of `script-typeset-lint.py`).
    pub fn lint_html(&self, src: &str, family: Option<&str>) -> LintReport {
        let scan = scan_html(src);
        let fam = family
            .map(String::from)
            .or_else(|| self.detect_family(&scan.styles))
            .unwrap_or_else(|| "unknown".into());
        let retired: Vec<String> = self
            .classes
            .iter()
            .filter(|r| g(r, "status") == "retired")
            .map(|r| {
                g(r, "class")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        let fam_retired = self.family_retired();
        let known_fams: Vec<&str> = self.families.iter().map(|r| g(r, "family")).collect();
        let (cmap, amap) = self.class_maps();
        let mut rep = LintReport {
            family: fam.clone(),
            findings: vec![],
        };
        for e in &scan.elements {
            if let Some(st) = &e.style {
                rep.add(
                    "inline-style",
                    e.line,
                    format!(
                        "<{} style=\"{}\">",
                        e.tag,
                        st.chars().take(50).collect::<String>()
                    ),
                );
            }
            if e.tag == "strong" && e.classes.iter().any(|c| c == "def-term") {
                rep.add(
                    "strong-def-term",
                    e.line,
                    "strong.def-term -> span.def-term".into(),
                );
            }
            for c in &e.classes {
                let (st, canon) = Self::lookup_in(&cmap, &amap, c);
                if retired.iter().any(|x| x == c) {
                    rep.add("retired-class", e.line, format!("{}.{}", e.tag, c));
                } else if let Some((_, _, dec)) =
                    fam_retired.iter().find(|(k, f, _)| k == c && *f == fam)
                {
                    rep.add(
                        "retired-class",
                        e.line,
                        format!("{}.{}: retired for {} ({})", e.tag, c, fam, dec),
                    );
                } else if st == "unknown" {
                    rep.add("unknown-class", e.line, format!("{}.{}", e.tag, c));
                } else if st == "alias" {
                    rep.add(
                        "alias-use",
                        e.line,
                        format!("{} -> {}", c, canon.unwrap_or_default()),
                    );
                } else {
                    if st == "proposed" {
                        rep.add("proposed-class", e.line, format!("{}.{}", e.tag, c));
                    }
                    let fams = Self::families_in(&cmap, &amap, c);
                    let lin = self.lineage(&fam);
                    if known_fams.contains(&fam.as_str())
                        && !fams.is_empty()
                        && !fams.iter().any(|x| x == "all")
                        && !fams.iter().any(|x| lin.contains(x))
                    {
                        let mut s = fams.clone();
                        s.sort();
                        rep.add(
                            "family-scope",
                            e.line,
                            format!("{} (registered for {})", c, s.join(";")),
                        );
                    }
                }
            }
            if let Some(t) = &e.label_text {
                if t.trim_end_matches([' ', '\t', '\n', '\u{a0}']) != t {
                    rep.add("label-space", e.line, format!("{:?}", t));
                }
            }
        }
        rep
    }
}

// ---------------------------------------------------------------------------------------------------- lint report
pub const LINT_RULES: &[(&str, &str)] = &[
    ("unknown-class", "error"),
    ("retired-class", "error"),
    ("inline-style", "error"),
    ("alias-use", "migration"),
    ("strong-def-term", "migration"),
    ("label-space", "migration"),
    ("family-scope", "info"),
    ("proposed-class", "info"),
];

pub struct LintReport {
    pub family: String,
    pub findings: Vec<(&'static str, usize, String)>,
}
impl LintReport {
    fn add(&mut self, rule: &str, line: usize, detail: String) {
        let r = LINT_RULES
            .iter()
            .find(|x| x.0 == rule)
            .map(|x| x.0)
            .unwrap_or("unknown-class");
        self.findings.push((r, line, detail));
    }
    pub fn count(&self, rule: &str) -> usize {
        self.findings.iter().filter(|f| f.0 == rule).count()
    }
    /// True when a finding fails the lint: errors always; migration findings with `strict` unless `legacy`; info with `strict`.
    pub fn failing(&self, legacy: bool, strict: bool) -> bool {
        LINT_RULES.iter().any(|(rule, sev)| {
            self.count(rule) > 0
                && match *sev {
                    "error" => true,
                    "migration" => strict && !legacy,
                    _ => strict,
                }
        })
    }
    pub fn print(&self, name: &str, legacy: bool, all_lines: bool) -> String {
        let mut o = format!("== {name}  [{}]\n", self.family);
        if self.findings.is_empty() {
            o.push_str("   clean\n");
        } else if legacy
            && self
                .findings
                .iter()
                .all(|f| LINT_RULES.iter().any(|r| r.0 == f.0 && r.1 == "migration"))
        {
            o.push_str(&format!(
                "   clean (--legacy: {} migration findings not shown)\n",
                self.findings.len()
            ));
        }
        for (rule, sev) in LINT_RULES {
            if legacy && *sev == "migration" {
                continue;
            }
            let it: Vec<&(&str, usize, String)> =
                self.findings.iter().filter(|f| f.0 == *rule).collect();
            if it.is_empty() {
                continue;
            }
            let shown = if all_lines { it.len() } else { it.len().min(5) };
            let list: Vec<String> = it
                .iter()
                .take(shown)
                .map(|f| format!("{} {}", f.1, f.2))
                .collect();
            o.push_str(&format!(
                "   {:<21} {:<9} {:>5}   {}{}\n",
                rule,
                sev,
                it.len(),
                list.join("; "),
                if shown == it.len() { "" } else { "; ..." }
            ));
        }
        o
    }
}

// ---------------------------------------------------------------------------------------------------- HTML scan
struct El {
    tag: String,
    line: usize,
    classes: Vec<String>,
    style: Option<String>,
    label_text: Option<String>,
}
struct Scan {
    elements: Vec<El>,
    styles: String,
}
const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

fn decode(s: &str) -> String {
    let mut o = String::new();
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        o.push_str(&rest[..i]);
        let tail = &rest[i..];
        match tail.find(';').filter(|e| *e <= 10) {
            Some(e) => {
                let ent = &tail[1..e];
                let rep = match ent {
                    "nbsp" => Some('\u{a0}'),
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    _ => ent
                        .strip_prefix("#x")
                        .or_else(|| ent.strip_prefix("#X"))
                        .and_then(|h| u32::from_str_radix(h, 16).ok())
                        .or_else(|| ent.strip_prefix('#').and_then(|d| d.parse().ok()))
                        .and_then(char::from_u32),
                };
                match rep {
                    Some(c) => {
                        o.push(c);
                        rest = &tail[e + 1..];
                    }
                    None => {
                        o.push('&');
                        rest = &tail[1..];
                    }
                }
            }
            None => {
                o.push('&');
                rest = &tail[1..];
            }
        }
    }
    o.push_str(rest);
    o
}

/// Minimal tolerant HTML scanner (no dependencies): elements of the `<body>` with line, class list, `style=` and (for span.clm / span.clm-r) their text.
fn scan_html(src: &str) -> Scan {
    let b = src.as_bytes();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut elements: Vec<El> = vec![];
    let mut styles = String::new();
    let mut stack: Vec<(String, Option<usize>)> = vec![]; // (tag, index into elements when a label capture is open)
    let mut in_body = !src.to_ascii_lowercase().contains("<body");
    let mut label_buf: BTreeMap<usize, String> = BTreeMap::new();
    while i < b.len() {
        if b[i] != b'<' {
            let j = src[i..].find('<').map(|p| i + p).unwrap_or(b.len());
            let text = &src[i..j];
            if !label_buf.is_empty() {
                let d = decode(text);
                for v in label_buf.values_mut() {
                    v.push_str(&d);
                }
            }
            line += text.matches('\n').count();
            i = j;
            continue;
        }
        let rest = &src[i..];
        if rest.starts_with("<!--") {
            let e = rest.find("-->").map(|p| p + 3).unwrap_or(rest.len());
            line += rest[..e].matches('\n').count();
            i += e;
            continue;
        }
        if rest.starts_with("<!") || rest.starts_with("<?") {
            let e = rest.find('>').map(|p| p + 1).unwrap_or(rest.len());
            line += rest[..e].matches('\n').count();
            i += e;
            continue;
        }
        let close = rest.starts_with("</");
        let name_start = if close { 2 } else { 1 };
        let name_len = rest[name_start..]
            .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
            .unwrap_or(rest.len() - name_start);
        if name_len == 0
            || !rest[name_start..]
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
        {
            i += 1;
            continue;
        }
        let tag = rest[name_start..name_start + name_len].to_ascii_lowercase();
        // end of the tag, honouring quotes
        let mut q = 0u8;
        let mut e = name_start + name_len;
        let rb = rest.as_bytes();
        while e < rb.len() {
            let c = rb[e];
            if q != 0 {
                if c == q {
                    q = 0;
                }
            } else if c == b'"' || c == b'\'' {
                q = c;
            } else if c == b'>' {
                break;
            }
            e += 1;
        }
        let inner = &rest[name_start + name_len..e.min(rest.len())];
        let tag_line = line;
        line += rest[..(e + 1).min(rest.len())].matches('\n').count();
        i += (e + 1).min(rest.len());
        if close {
            if tag == "body" {
                in_body = false;
            }
            if let Some(p) = stack.iter().rposition(|s| s.0 == tag) {
                for (_, cap) in stack.drain(p..) {
                    if let Some(ix) = cap {
                        if let Some(t) = label_buf.remove(&ix) {
                            elements[ix].label_text = Some(t);
                        }
                    }
                }
            }
            continue;
        }
        let (attrs, selfclose) = parse_attrs(inner);
        if tag == "body" {
            in_body = true;
            continue;
        }
        if tag == "style" || tag == "script" {
            let end = src[i..]
                .to_ascii_lowercase()
                .find(&format!("</{tag}"))
                .map(|p| i + p)
                .unwrap_or(b.len());
            if tag == "style" {
                styles.push_str(&src[i..end]);
                styles.push('\n');
            }
            line += src[i..end].matches('\n').count();
            i = end;
            continue;
        }
        let classes: Vec<String> = attrs
            .iter()
            .find(|a| a.0 == "class")
            .map(|a| a.1.split_whitespace().map(String::from).collect())
            .unwrap_or_default();
        let mut cap = None;
        if in_body {
            let ix = elements.len();
            let is_label = tag == "span" && classes.iter().any(|c| c == "clm" || c == "clm-r");
            elements.push(El {
                tag: tag.clone(),
                line: tag_line,
                classes,
                style: attrs.iter().find(|a| a.0 == "style").map(|a| a.1.clone()),
                label_text: None,
            });
            if is_label && !selfclose {
                cap = Some(ix);
                label_buf.insert(ix, String::new());
            }
        }
        if !selfclose && !VOID.contains(&tag.as_str()) {
            stack.push((tag, cap));
        }
    }
    Scan { elements, styles }
}

fn parse_attrs(s: &str) -> (Vec<(String, String)>, bool) {
    let mut out = vec![];
    let ch: Vec<char> = s.chars().collect();
    let selfclose = s.trim_end().ends_with('/');
    let mut i = 0;
    while i < ch.len() {
        while i < ch.len() && (ch[i].is_whitespace() || ch[i] == '/') {
            i += 1;
        }
        let st = i;
        while i < ch.len() && !ch[i].is_whitespace() && ch[i] != '=' && ch[i] != '/' {
            i += 1;
        }
        if st == i {
            break;
        }
        let name: String = ch[st..i].iter().collect::<String>().to_ascii_lowercase();
        while i < ch.len() && ch[i].is_whitespace() {
            i += 1;
        }
        let mut val = String::new();
        if i < ch.len() && ch[i] == '=' {
            i += 1;
            while i < ch.len() && ch[i].is_whitespace() {
                i += 1;
            }
            if i < ch.len() && (ch[i] == '"' || ch[i] == '\'') {
                let q = ch[i];
                i += 1;
                let vs = i;
                while i < ch.len() && ch[i] != q {
                    i += 1;
                }
                val = ch[vs..i].iter().collect();
                i += 1;
            } else {
                let vs = i;
                while i < ch.len() && !ch[i].is_whitespace() {
                    i += 1;
                }
                val = ch[vs..i].iter().collect();
            }
        }
        out.push((name, val));
    }
    (out, selfclose)
}

// ---------------------------------------------------------------------------------------------------- JSON (Python json.dumps(indent=1, ensure_ascii=False) layout)
enum J {
    Obj(Vec<(String, J)>),
    Arr(Vec<J>),
    Str(String),
    Raw(String),
}
impl J {
    fn obj() -> J {
        J::Obj(vec![])
    }
    fn s(x: &str) -> J {
        J::Str(x.to_string())
    }
    fn with(mut self, k: &str, v: J) -> J {
        self.set(k, v);
        self
    }
    /// Insert or replace, keeping the position of an existing key (Python dict semantics).
    fn set(&mut self, k: &str, v: J) {
        if let J::Obj(m) = self {
            if let Some(e) = m.iter_mut().find(|e| e.0 == k) {
                e.1 = v;
            } else {
                m.push((k.to_string(), v));
            }
        }
    }
    /// The child object under `k`, created empty when absent (`dict.setdefault(k, {})`).
    fn entry(&mut self, k: &str) -> &mut J {
        if let J::Obj(m) = self {
            if let Some(p) = m.iter().position(|e| e.0 == k) {
                return &mut m[p].1;
            }
            m.push((k.to_string(), J::obj()));
            let n = m.len() - 1;
            return &mut m[n].1;
        }
        unreachable!("entry on a non-object")
    }
    fn dump(&self, o: &mut String, lvl: usize) {
        let pad = |n: usize| " ".repeat(n);
        match self {
            J::Str(s) => {
                o.push('"');
                for c in s.chars() {
                    match c {
                        '"' => o.push_str("\\\""),
                        '\\' => o.push_str("\\\\"),
                        '\n' => o.push_str("\\n"),
                        '\r' => o.push_str("\\r"),
                        '\t' => o.push_str("\\t"),
                        '\u{8}' => o.push_str("\\b"),
                        '\u{c}' => o.push_str("\\f"),
                        c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
                        c => o.push(c),
                    }
                }
                o.push('"');
            }
            J::Raw(s) => o.push_str(s),
            J::Arr(a) => {
                if a.is_empty() {
                    o.push_str("[]");
                    return;
                }
                o.push_str("[\n");
                for (i, v) in a.iter().enumerate() {
                    o.push_str(&pad(lvl + 1));
                    v.dump(o, lvl + 1);
                    if i + 1 < a.len() {
                        o.push(',');
                    }
                    o.push('\n');
                }
                o.push_str(&pad(lvl));
                o.push(']');
            }
            J::Obj(m) => {
                if m.is_empty() {
                    o.push_str("{}");
                    return;
                }
                o.push_str("{\n");
                for (i, (k, v)) in m.iter().enumerate() {
                    o.push_str(&pad(lvl + 1));
                    J::Str(k.clone()).dump(o, 0);
                    o.push_str(": ");
                    v.dump(o, lvl + 1);
                    if i + 1 < m.len() {
                        o.push(',');
                    }
                    o.push('\n');
                }
                o.push_str(&pad(lvl));
                o.push('}');
            }
        }
    }
}

/// Python `float` repr of a finite number with at most four decimals (the only kind the tokens produce).
fn fl(x: f64) -> String {
    format!("{x:?}")
}
fn round4(x: f64) -> f64 {
    format!("{x:.4}").parse().unwrap_or(x)
}
fn pf(s: &str) -> f64 {
    s.trim().parse().unwrap_or(0.0)
}
fn tw(pt: f64) -> i64 {
    (pt * 20.0).round_ties_even() as i64
}
fn dim(pt: f64) -> J {
    J::obj()
        .with("$type", J::s("dimension"))
        .with(
            "$value",
            J::obj()
                .with("value", J::Raw(fl(round4(pt * 4.0 / 3.0))))
                .with("unit", J::s("px")),
        )
        .with(
            "$extensions",
            J::obj().with(
                "com.pointsav.typeset",
                J::obj()
                    .with("pt", J::Raw(fl(pt)))
                    .with("twips", J::Raw(tw(pt).to_string())),
            ),
        )
}
/// `^(-?[\d.]+)pt$`
fn pt_value(v: &str) -> Option<f64> {
    let n = v.strip_suffix("pt")?;
    let d = n.strip_prefix('-').unwrap_or(n);
    if d.is_empty() || !d.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
    }
    n.parse().ok()
}
/// `re.sub(r"[^A-Za-z0-9_-]+", "_", selector).strip("_")`
fn node_key(sel: &str) -> String {
    let mut o = String::new();
    let mut gap = false;
    for c in sel.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            o.push(c);
            gap = false;
        } else if !gap {
            o.push('_');
            gap = true;
        }
    }
    o.trim_matches('_').to_string()
}

// ---------------------------------------------------------------------------------------------------- CLI
const REG_USAGE: &str = "usage: registry check | classes [--family F] | css [--component C | --out DIR] | dtcg [--family F] [--out FILE] | lint FILE... [--family F] [--legacy] [--strict] [--all-lines]   (--root DIR, --registry DIR)";

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

/// `registry <sub> ...`: exit 0 = ok, 1 = findings, 2 = usage / read error.
pub fn run(args: &[String], default_root: &Path) -> i32 {
    let Some(sub) = args.first() else {
        eprintln!("{REG_USAGE}");
        return 2;
    };
    let rest = &args[1..];
    let root =
        PathBuf::from(flag(rest, "--root").unwrap_or_else(|| default_root.display().to_string()));
    let dir = flag(rest, "--registry")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("precedent/registry"));
    let reg = match Registry::load(&dir) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    let family = flag(rest, "--family");
    match sub.as_str() {
        "check" => {
            let f = reg.check(&root);
            for x in &f {
                println!("{} {}", if x.error { "ERROR" } else { "warn " }, x.msg);
            }
            let errs = f.iter().filter(|x| x.error).count();
            println!(
                "registry check: {} classes, {} geometry rows, {} errors, {} warnings",
                reg.classes.len(),
                reg.geometry.len(),
                errs,
                f.len() - errs
            );
            if errs > 0 {
                1
            } else {
                0
            }
        }
        "classes" => {
            print!("{}", reg.classes_list(family.as_deref()));
            0
        }
        "css" => {
            let comp = flag(rest, "--component");
            let files = reg.css_files(comp.as_deref());
            if files.is_empty() {
                eprintln!(
                    "error: no CSS-sourced rows{}",
                    comp.map(|c| format!(" for component {c}"))
                        .unwrap_or_default()
                );
                return 2;
            }
            match flag(rest, "--out") {
                Some(out) => {
                    let d = Path::new(&out).join("css");
                    if let Err(e) = std::fs::create_dir_all(&d) {
                        eprintln!("error: {e}");
                        return 2;
                    }
                    for (c, t) in &files {
                        if let Err(e) = std::fs::write(d.join(format!("{c}.css")), t) {
                            eprintln!("error: {e}");
                            return 2;
                        }
                    }
                    println!("wrote {}", d.display());
                }
                None => {
                    if files.len() > 1 {
                        eprintln!("error: several components: name one with --component C or write all with --out DIR");
                        return 2;
                    }
                    print!("{}", files[0].1);
                }
            }
            0
        }
        "dtcg" => {
            let js = reg.dtcg(family.as_deref());
            match flag(rest, "--out") {
                Some(p) => {
                    if let Err(e) = std::fs::write(&p, format!("{js}\n")) {
                        eprintln!("error: {e}");
                        return 2;
                    }
                    println!("wrote {p}");
                }
                None => println!("{js}"),
            }
            0
        }
        "lint" => {
            let value_flags = ["--root", "--registry", "--family"];
            let mut files: Vec<String> = vec![];
            let mut skip = false;
            for a in rest {
                if skip {
                    skip = false;
                    continue;
                }
                if value_flags.contains(&a.as_str()) {
                    skip = true;
                    continue;
                }
                if !a.starts_with("--") {
                    files.push(a.clone());
                }
            }
            if files.is_empty() {
                eprintln!("{REG_USAGE}");
                return 2;
            }
            let (legacy, strict, all) = (
                rest.iter().any(|a| a == "--legacy"),
                rest.iter().any(|a| a == "--strict"),
                rest.iter().any(|a| a == "--all-lines"),
            );
            let mut bad = false;
            for f in &files {
                let src = match std::fs::read_to_string(f) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: {f}: {e}");
                        return 2;
                    }
                };
                let rep = reg.lint_html(&src, family.as_deref());
                print!("{}", rep.print(f, legacy, all));
                bad |= rep.failing(legacy, strict);
            }
            if bad {
                1
            } else {
                0
            }
        }
        _ => {
            eprintln!("{REG_USAGE}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry")
    }
    fn golden(n: &str) -> String {
        std::fs::read_to_string(fixture().join("golden").join(n))
            .unwrap_or_else(|e| panic!("golden {n}: {e}"))
    }
    fn reg() -> Registry {
        Registry::load(&fixture().join("precedent/registry")).expect("fixture registry")
    }

    #[test]
    fn python_float_repr() {
        assert_eq!(fl(36.0), "36.0");
        assert_eq!(fl(14.6667), "14.6667");
        assert_eq!(fl(-36.0), "-36.0");
        assert_eq!(fl(round4(11.5 * 4.0 / 3.0)), "15.3333");
    }
    #[test]
    fn twips_round_half_even() {
        assert_eq!(tw(0.025), 0);
        assert_eq!(tw(36.0), 720);
        assert_eq!(tw(0.075), 2);
    }
    #[test]
    fn node_key_matches_python() {
        assert_eq!(node_key("p.recital span.clm-r"), "p_recital_span_clm-r");
        assert_eq!(node_key("table.crosswalk td"), "table_crosswalk_td");
        assert_eq!(node_key(".a, .b"), "a_b");
    }
    #[test]
    fn pt_values() {
        assert_eq!(pt_value("-36pt"), Some(-36.0));
        assert_eq!(pt_value("1.5pt"), Some(1.5));
        assert_eq!(pt_value("1pt solid black"), None);
        assert_eq!(pt_value("80%"), None);
    }

    fn mk(kv: &[(&str, &str)]) -> Row {
        kv.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn parent_family_inherits_cells_lineage_and_css() {
        // parent `mx` (no token file), children `a` (own body size) and `b`; one row shared by both, one row only a has.
        let fam = |f: &str, parent: &str, stem: &str, tok: &str, size: &str, lang: &str| {
            mk(&[
                ("family", f),
                ("parent", parent),
                ("token_stem", stem),
                ("token_css", tok),
                ("body_size", size),
                ("lang", lang),
                ("status", "current"),
            ])
        };
        let geo = |id: &str, comp: &str, fam: &str, sel: &str, ev: &str, size: &str| {
            mk(&[
                ("id", id),
                ("component", comp),
                ("family", fam),
                ("selector", sel),
                ("source", "css"),
                ("status", "current"),
                ("evidence", ev),
                ("font_size", size),
            ])
        };
        let mut families = vec![
            fam("mx", "", "mx", "", "", "es-MX"),
            fam("a", "mx", "a", "t/a.css", "11pt", ""),
            fam("b", "mx", "b", "t/b.css", "10pt", ""),
        ];
        Registry::inherit_families(&mut families);
        let r = Registry {
            classes: vec![mk(&[("class", "x"), ("families", "mx;letter")])],
            geometry: vec![
                geo("a:body", "a", "a", "body", "t/a.css:3", "11pt"),
                geo("mx:p", "mx", "mx", "p", "t/a.css:9;t/b.css:5", "9pt"),
                geo("mx:em", "mx", "mx", "em", "t/a.css:7;t/b.css:8", "8pt"),
                geo("b:body", "b", "b", "body", "t/b.css:2", "10pt"),
            ],
            labels: vec![],
            glyphs: vec![],
            sections: vec![],
            families,
            engine: vec![],
            changes: vec![],
        };
        assert_eq!(g(&r.families[1], "lang"), "es-MX");
        assert_eq!(g(&r.families[1], "body_size"), "11pt");
        assert_eq!(r.lineage("a"), vec!["a", "mx"]);
        assert!(
            r.applies("mx;letter", "b") && r.applies("all", "b") && !r.applies("letter;b", "a")
        );
        assert!(r.classes_list(Some("a")).contains("x\t"));
        let files = r.css_files(None);
        let names: Vec<&str> = files.iter().map(|(c, _)| c.as_str()).collect();
        assert_eq!(names, vec!["a", "b"], "the abstract parent writes no file");
        // a: its token file order = line 3 body, 7 em, 9 p; b: 2 body, 5 p, 8 em
        let pos = |t: &str, s: &str| t.find(s).unwrap();
        let a = &files[0].1;
        assert!(pos(a, "body{") < pos(a, "em{") && pos(a, "em{") < pos(a, "p{"));
        let b = &files[1].1;
        assert!(pos(b, "body{") < pos(b, "p{") && pos(b, "p{") < pos(b, "em{"));
    }

    #[test]
    fn css_matches_golden_for_every_fixture_component() {
        let r = reg();
        let files = r.css_files(None);
        assert!(files.len() >= 3, "fixture should hold several components");
        for (c, t) in files {
            assert_eq!(t, golden(&format!("css/{c}.css")), "component {c}");
        }
    }
    #[test]
    fn dtcg_matches_golden() {
        let r = reg();
        assert_eq!(format!("{}\n", r.dtcg(None)), golden("dtcg-all.json"));
        assert_eq!(
            format!("{}\n", r.dtcg(Some("canada"))),
            golden("dtcg-canada.json")
        );
        assert_eq!(
            format!("{}\n", r.dtcg(Some("letter"))),
            golden("dtcg-letter.json")
        );
    }
    #[test]
    fn classes_list_by_family() {
        let r = reg();
        assert_eq!(r.classes_list(Some("canada")), golden("classes-canada.tsv"));
        assert!(
            r.classes_list(None).lines().count() > r.classes_list(Some("letter")).lines().count()
        );
    }
    #[test]
    fn check_passes_on_fixture_and_catches_planted_faults() {
        let root = fixture();
        let r = reg();
        let f = r.check(&root);
        assert!(
            f.iter().all(|x| !x.error),
            "{:?}",
            f.iter().map(|x| x.msg.clone()).collect::<Vec<_>>()
        );
        let mut bad = reg();
        bad.geometry[0].insert("class".into(), "no-such-class".into());
        bad.classes[0].insert("status".into(), "bogus".into());
        bad.classes[1].insert("glyph".into(), "no-such-glyph".into());
        let m: Vec<String> = bad
            .check(&root)
            .into_iter()
            .filter(|x| x.error)
            .map(|x| x.msg)
            .collect();
        assert!(m.iter().any(|x| x.contains("no-such-class")), "{m:?}");
        assert!(m.iter().any(|x| x.contains("closed set")), "{m:?}");
        assert!(m.iter().any(|x| x.contains("no-such-glyph")), "{m:?}");
    }
    #[test]
    fn check_catches_a_changed_generated_block() {
        let root = fixture();
        let mut r = reg();
        let own = r.own_file_components();
        assert_eq!(own, vec!["ca-classes".to_string()]);
        let row = r
            .geometry
            .iter_mut()
            .find(|x| g(x, "component") == "ca-classes" && g(x, "selector") == "p.cl1c")
            .unwrap();
        row.insert("margin_left".into(), "99pt".into());
        assert!(r
            .check(&root)
            .iter()
            .any(|x| x.error && x.msg.contains("generated block differs")));
    }
    #[test]
    fn lint_counts() {
        let r = reg();
        let html = "<html><head><style>/* constitutional-agreement.css — header */ .x{}</style></head><body>\n<p class=\"recital\" style=\"text-align:left\">A</p>\n<p class=\"cl1 nosuch\"><span class=\"clm\">(a)&nbsp;&nbsp;</span>x</p>\n<div class=\"change-annex\"></div><h3 style='a'>T</h3></body></html>";
        let rep = r.lint_html(html, None);
        assert_eq!(rep.family, "canada");
        assert_eq!(rep.count("inline-style"), 2);
        assert_eq!(rep.count("retired-class"), 2, "{:?}", rep.findings);
        assert_eq!(rep.count("unknown-class"), 1);
        assert_eq!(rep.count("label-space"), 1);
        assert!(rep.failing(false, false));
        assert!(rep.print("t.html", false, false).contains("retired-class"));
    }
    #[test]
    fn lint_ignores_head_style_attribute_and_style_text() {
        let r = reg();
        let rep = r.lint_html(
            "<html><head><style style=\"x\">p{}</style></head><body><p>ok</p></body></html>",
            Some("canada"),
        );
        assert!(rep.findings.is_empty(), "{:?}", rep.findings);
    }
    #[test]
    fn json_layout_is_python_indent_1() {
        let mut o = String::new();
        J::obj()
            .with("a", J::Arr(vec![J::Raw("1".into()), J::obj()]))
            .with("b", J::s("x\"y"))
            .dump(&mut o, 0);
        assert_eq!(o, "{\n \"a\": [\n  1,\n  {}\n ],\n \"b\": \"x\\\"y\"\n}");
    }
}
