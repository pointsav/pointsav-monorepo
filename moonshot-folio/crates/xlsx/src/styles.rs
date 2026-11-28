// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Cell styles: a role to look map supplied by the caller, and the generated `xl/styles.xml`.

use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Ink, Result};

/// Horizontal alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    /// Spreadsheet default (text left, numbers right).
    #[default]
    General,
    /// Left.
    Left,
    /// Right.
    Right,
    /// Centred.
    Center,
}

/// Which horizontal edges carry a thin rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Rule {
    /// No rule.
    #[default]
    None,
    /// Rule above.
    Top,
    /// Rule below.
    Bottom,
    /// Rules above and below.
    Both,
}

/// How a cell looks. Colours must be gray or RGB; CMYK is not representable in a worksheet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CellStyle {
    /// Bold text.
    pub bold: bool,
    /// Italic text.
    pub italic: bool,
    /// Number format code such as `#,##0.00` (none for General).
    pub num_fmt: Option<String>,
    /// Horizontal alignment.
    pub align: Align,
    /// Thin rules on the cell's top and bottom edges.
    pub rule: Rule,
    /// Background fill.
    pub fill: Option<Ink>,
    /// Text colour.
    pub color: Option<Ink>,
}

/// Role name to style, in insertion order.
#[derive(Clone, Debug, Default)]
pub struct StyleMap {
    entries: Vec<(String, CellStyle)>,
}

impl StyleMap {
    /// An empty map: every cell uses the default style.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add or replace the style for `role`.
    pub fn insert(&mut self, role: &str, style: CellStyle) {
        if let Some(e) = self.entries.iter_mut().find(|(r, _)| r == role) {
            e.1 = style;
        } else {
            self.entries.push((role.to_string(), style));
        }
    }
    /// The style for `role`, if any.
    #[must_use]
    pub fn get(&self, role: &str) -> Option<&CellStyle> {
        self.entries.iter().find(|(r, _)| r == role).map(|(_, s)| s)
    }
}

/// The distinct styles in use, in first-use order. Index 0 is the default style.
#[derive(Default)]
pub(crate) struct Registry {
    styles: Vec<CellStyle>,
    dxfs: Vec<CellStyle>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            styles: vec![CellStyle::default()],
            dxfs: Vec::new(),
        }
    }
    /// The `s` index for a cell role (0 when the role has no style or the style is the default).
    pub(crate) fn index(&mut self, map: &StyleMap, role: &str) -> usize {
        let Some(style) = map.get(role) else { return 0 };
        if let Some(i) = self.styles.iter().position(|s| s == style) {
            return i;
        }
        self.styles.push(style.clone());
        self.styles.len().saturating_sub(1)
    }
}

impl Registry {
    /// The `dxfId` for a conditional-format role.
    ///
    /// # Errors
    /// `Invalid` when the map has no style for `role`.
    pub(crate) fn dxf_index(&mut self, map: &StyleMap, role: &str) -> Result<usize> {
        let style = map
            .get(role)
            .ok_or(FolioError::Invalid("conditional format role has no style"))?;
        if let Some(i) = self.dxfs.iter().position(|s| s == style) {
            return Ok(i);
        }
        self.dxfs.push(style.clone());
        Ok(self.dxfs.len().saturating_sub(1))
    }
}

/// A differential format: only font weight, slant, colour and fill apply.
fn dxf(s: &CellStyle) -> Result<String> {
    let mut f = String::from("<dxf><font>");
    if s.bold {
        f.push_str("<b/>");
    }
    if s.italic {
        f.push_str("<i/>");
    }
    if let Some(c) = s.color {
        write!(f, "<color rgb=\"{}\"/>", rgb(c)?).ok();
    }
    f.push_str("</font>");
    if let Some(c) = s.fill {
        let c = rgb(c)?;
        write!(f, "<fill><patternFill patternType=\"solid\"><fgColor rgb=\"{c}\"/><bgColor rgb=\"{c}\"/></patternFill></fill>").ok();
    }
    f.push_str("</dxf>");
    Ok(f)
}

fn rgb(ink: Ink) -> Result<String> {
    match ink {
        Ink::Gray(g) => Ok(format!("FF{g:02X}{g:02X}{g:02X}")),
        Ink::Rgb(r, g, b) => Ok(format!("FF{r:02X}{g:02X}{b:02X}")),
        Ink::Cmyk(..) => Err(FolioError::Unsupported("CMYK colour in a worksheet")),
    }
}

fn font(s: &CellStyle) -> Result<String> {
    let mut f = String::from("<font>");
    if s.bold {
        f.push_str("<b/>");
    }
    if s.italic {
        f.push_str("<i/>");
    }
    f.push_str("<sz val=\"11\"/>");
    if let Some(c) = s.color {
        write!(f, "<color rgb=\"{}\"/>", rgb(c)?).ok();
    }
    f.push_str("<name val=\"Calibri\"/></font>");
    Ok(f)
}

fn border(s: &CellStyle) -> String {
    let side = |name: &str, on: bool| {
        if on {
            format!("<{name} style=\"thin\"><color auto=\"1\"/></{name}>")
        } else {
            format!("<{name}/>")
        }
    };
    format!(
        "<border><left/><right/>{}{}<diagonal/></border>",
        side("top", matches!(s.rule, Rule::Top | Rule::Both)),
        side("bottom", matches!(s.rule, Rule::Bottom | Rule::Both))
    )
}

/// Check a number format code: printable, at most 255 characters.
fn check_fmt(code: &str) -> Result<()> {
    let bad = || FolioError::Invalid("number format code is not valid");
    if code.is_empty() || code.chars().count() > 255 || code.chars().any(char::is_control) {
        return Err(bad());
    }
    // Balanced quotes and brackets, and at most four `;` sections, ignoring escaped characters.
    let (mut quote, mut bracket, mut sections) = (false, false, 1_u8);
    let mut esc = false;
    for c in code.chars() {
        if esc {
            esc = false;
            continue;
        }
        match c {
            '\\' | '_' | '*' if !quote => esc = true,
            '"' if !bracket => quote = !quote,
            '[' if !quote => {
                if bracket {
                    return Err(bad());
                }
                bracket = true;
            }
            ']' if !quote => {
                if !bracket {
                    return Err(bad());
                }
                bracket = false;
            }
            ';' if !quote && !bracket => sections = sections.saturating_add(1),
            _ => {}
        }
    }
    if quote || bracket || esc || sections > 4 {
        return Err(bad());
    }
    Ok(())
}

/// Dedupe helper: index of `item` in `v`, pushing it when absent.
fn intern(v: &mut Vec<String>, item: String) -> usize {
    if let Some(i) = v.iter().position(|x| *x == item) {
        i
    } else {
        v.push(item);
        v.len().saturating_sub(1)
    }
}

/// Deduplicated style tables.
struct Tables {
    fonts: Vec<String>,
    fills: Vec<String>,
    borders: Vec<String>,
    fmts: Vec<String>,
}

impl Tables {
    fn new() -> Self {
        Self {
            fonts: Vec::new(),
            fills: vec![
                "<fill><patternFill patternType=\"none\"/></fill>".to_string(),
                "<fill><patternFill patternType=\"gray125\"/></fill>".to_string(),
            ],
            borders: Vec::new(),
            fmts: Vec::new(),
        }
    }

    /// The `<xf>` element for one style, registering its font, fill, border and format.
    fn xf(&mut self, s: &CellStyle) -> Result<String> {
        let fi = intern(&mut self.fonts, font(s)?);
        let bi = intern(&mut self.borders, border(s));
        let fill = match s.fill {
            Some(c) => intern(
                &mut self.fills,
                format!("<fill><patternFill patternType=\"solid\"><fgColor rgb=\"{}\"/><bgColor indexed=\"64\"/></patternFill></fill>", rgb(c)?),
            ),
            None => 0,
        };
        let nf = match &s.num_fmt {
            Some(code) => {
                check_fmt(code)?;
                intern(&mut self.fmts, code.clone()).saturating_add(164)
            }
            None => 0,
        };
        let align = match s.align {
            Align::General => "",
            Align::Left => "<alignment horizontal=\"left\"/>",
            Align::Right => "<alignment horizontal=\"right\"/>",
            Align::Center => "<alignment horizontal=\"center\"/>",
        };
        Ok(format!(
            "<xf numFmtId=\"{nf}\" fontId=\"{fi}\" fillId=\"{fill}\" borderId=\"{bi}\" xfId=\"0\" applyNumberFormat=\"1\" applyFont=\"1\" applyFill=\"1\" applyBorder=\"1\" applyAlignment=\"1\">{align}</xf>"
        ))
    }

    fn num_fmts(&self, out: &mut String) -> Result<()> {
        if self.fmts.is_empty() {
            return Ok(());
        }
        write!(out, "<numFmts count=\"{}\">", self.fmts.len()).ok();
        for (i, code) in self.fmts.iter().enumerate() {
            write!(
                out,
                "<numFmt numFmtId=\"{}\" formatCode=\"",
                i.saturating_add(164)
            )
            .ok();
            escape_into(out, code)?;
            out.push_str("\"/>");
        }
        out.push_str("</numFmts>");
        Ok(())
    }
}

/// Render `xl/styles.xml` for the registry.
pub(crate) fn styles_xml(reg: &Registry) -> Result<String> {
    let mut t = Tables::new();
    let mut xfs = String::new();
    for s in &reg.styles {
        xfs.push_str(&t.xf(s)?);
    }
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">");
    t.num_fmts(&mut out)?;
    write!(
        out,
        "<fonts count=\"{}\">{}</fonts>",
        t.fonts.len(),
        t.fonts.concat()
    )
    .ok();
    write!(
        out,
        "<fills count=\"{}\">{}</fills>",
        t.fills.len(),
        t.fills.concat()
    )
    .ok();
    write!(
        out,
        "<borders count=\"{}\">{}</borders>",
        t.borders.len(),
        t.borders.concat()
    )
    .ok();
    out.push_str("<cellStyleXfs count=\"1\"><xf numFmtId=\"0\" fontId=\"0\" fillId=\"0\" borderId=\"0\"/></cellStyleXfs>");
    write!(
        out,
        "<cellXfs count=\"{}\">{xfs}</cellXfs>",
        reg.styles.len()
    )
    .ok();
    out.push_str("<cellStyles count=\"1\"><cellStyle name=\"Normal\" xfId=\"0\" builtinId=\"0\"/></cellStyles>");
    if !reg.dxfs.is_empty() {
        write!(out, "<dxfs count=\"{}\">", reg.dxfs.len()).ok();
        for d in &reg.dxfs {
            out.push_str(&dxf(d)?);
        }
        out.push_str("</dxfs>");
    }
    out.push_str("</styleSheet>");
    Ok(out)
}
