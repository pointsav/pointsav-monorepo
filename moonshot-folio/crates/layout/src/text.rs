// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Inline content to lines: measure, break (UAX#14 greedy wrap), place on baselines.

use crate::ctx::{Ctx, Frag, RelDest};
use crate::theme::Style;
use crate::tree::{
    BoxRect, Item, ItemKind, LinkAnnot, LinkTarget, NodeId, RectItem, StructRole, TextItem,
};
use moonshot_folio_core::{Emu, FolioError, Result};
use moonshot_folio_text::{line_breaks, wrap};

/// Horizontal alignment inside the line box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Align {
    Left,
    Right,
}

/// A piece of inline text with one style.
#[derive(Clone, Debug)]
pub(crate) struct Seg {
    pub(crate) text: String,
    pub(crate) style: Style,
    pub(crate) link: Option<String>,
    pub(crate) highlight: bool,
    pub(crate) node: NodeId,
    pub(crate) role: StructRole,
    pub(crate) anchor: Option<String>,
    /// Digits reserved for a page-number placeholder (0 for ordinary text). The placeholder is
    /// one character as wide as that many of the widest digit, so wrapping can neither split it
    /// nor let any label of at most that many digits overflow; the real number and its real
    /// width replace it when the line is placed.
    pub(crate) field_digits: usize,
}

impl Seg {
    pub(crate) fn plain(text: String, style: Style, node: NodeId, role: StructRole) -> Self {
        Self {
            text,
            style,
            link: None,
            highlight: false,
            node,
            role,
            anchor: None,
            field_digits: 0,
        }
    }
}

/// Where lines go: left edge, width and alignment.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Geom {
    pub(crate) x0: Emu,
    pub(crate) width: Emu,
    pub(crate) align: Align,
}

/// Per-character data of a measured paragraph.
struct Measured {
    full: String,
    chars: Vec<(usize, char)>,
    seg_of: Vec<usize>,
    widths: Vec<i64>,
    /// Per character: drawn with a substitute width.
    subst: Vec<bool>,
    anchors: Vec<(usize, usize)>,
}

/// Map line terminators to `\n` and tabs to spaces.
fn normalise(c: char, prev_cr: bool) -> Option<char> {
    match c {
        '\n' if prev_cr => None,
        '\r' | '\u{b}' | '\u{c}' | '\u{85}' | '\u{2028}' | '\u{2029}' => Some('\n'),
        '\t' => Some(' '),
        other => Some(other),
    }
}

/// The widest advance of a decimal digit in the style: the width a page-number placeholder
/// reserves for each digit, so any label of at most as many digits fits.
fn widest_digit(ctx: &Ctx<'_>, st: Style) -> Result<i64> {
    let mut w = 0_i64;
    for d in '0'..='9' {
        let (dw, _) = ctx
            .fonts
            .width(st.slot, d, st.size, ctx.opts.missing_glyph)?;
        w = w.max(dw.raw());
    }
    Ok(w)
}

fn measure(ctx: &Ctx<'_>, segs: &[Seg]) -> Result<Measured> {
    let bytes = segs
        .iter()
        .fold(0_usize, |a, s| a.saturating_add(s.text.len()));
    ctx.charge_chars(bytes)?;
    let mut m = Measured {
        full: String::new(),
        chars: Vec::new(),
        seg_of: Vec::new(),
        widths: Vec::new(),
        subst: Vec::new(),
        anchors: Vec::new(),
    };
    for (si, seg) in segs.iter().enumerate() {
        if seg.anchor.is_some() {
            m.anchors.push((m.full.len(), si));
        }
        let mut prev_cr = false;
        let digit = if seg.field_digits > 0 {
            let n = i64::try_from(seg.field_digits).unwrap_or(i64::MAX);
            Some(widest_digit(ctx, seg.style)?.saturating_mul(n))
        } else {
            None
        };
        for raw in seg.text.chars() {
            let cr = raw == '\r';
            let step = normalise(raw, prev_cr);
            prev_cr = cr;
            let Some(c) = step else { continue };
            let (w, sub) = match (c, digit) {
                ('\n', _) => (0, false),
                (_, Some(d)) => (d, false),
                _ => {
                    let (w, sub) = ctx.fonts.width(
                        seg.style.slot,
                        c,
                        seg.style.size,
                        ctx.opts.missing_glyph,
                    )?;
                    (w.raw(), sub)
                }
            };
            m.chars.push((m.full.len(), c));
            m.full.push(c);
            m.seg_of.push(si);
            m.widths.push(w);
            m.subst.push(sub);
        }
    }
    Ok(m)
}

/// One run of characters of one segment on one line.
struct Group {
    seg: usize,
    text: String,
    width: i64,
    subst: usize,
}

/// Split the characters of a line into per-segment groups, dropping trailing spaces and
/// terminators (they hang outside the line box).
fn groups(m: &Measured, range: core::ops::Range<usize>) -> (Vec<Group>, bool) {
    let mut end = range.end;
    while end > range.start
        && m.chars
            .get(end.saturating_sub(1))
            .is_some_and(|(_, c)| *c == ' ' || *c == '\n')
    {
        end = end.saturating_sub(1);
    }
    let dropped = end < range.end;
    let mut out: Vec<Group> = Vec::new();
    for i in range.start..end {
        let (Some(&(_, c)), Some(&si), Some(&w), Some(&sub)) = (
            m.chars.get(i),
            m.seg_of.get(i),
            m.widths.get(i),
            m.subst.get(i),
        ) else {
            continue;
        };
        let sub = usize::from(sub);
        match out.last_mut() {
            Some(g) if g.seg == si => {
                g.text.push(c);
                g.width = g.width.saturating_add(w);
                g.subst = g.subst.saturating_add(sub);
            }
            _ => out.push(Group {
                seg: si,
                text: c.to_string(),
                width: w,
                subst: sub,
            }),
        }
    }
    (out, dropped)
}

/// Line box metrics: height and baseline offset from the top.
fn line_metrics(ctx: &Ctx<'_>, styles: &[Style]) -> Result<(Emu, Emu)> {
    let (mut asc, mut desc, mut lead) = (Emu::ZERO, Emu::ZERO, Emu::ZERO);
    for s in styles {
        let (a, d) = ctx.fonts.extents(s.slot, s.size)?;
        asc = asc.max(a);
        desc = desc.max(d);
        let l = Emu::from_ratio(s.size.raw(), ctx.theme.leading_percent, 100)?;
        lead = lead.max(l);
    }
    let used = asc.checked_add(desc)?;
    let height = lead.max(used);
    let slack = height.checked_sub(used)?;
    let half = Emu::from_raw(slack.raw().checked_div(2).unwrap_or(0));
    Ok((height, half.checked_add(asc)?))
}

/// Everything needed to build the lines of one paragraph.
struct Build<'a, 'b> {
    ctx: &'a Ctx<'b>,
    segs: &'a [Seg],
    m: &'a Measured,
    g: Geom,
    base: Style,
}

impl Build<'_, '_> {
    fn line(&self, range: core::ops::Range<usize>) -> Result<Frag> {
        self.ctx.charge_line()?;
        let (gs, dropped) = groups(self.m, range);
        let total = gs.iter().fold(0_i64, |a, g| a.saturating_add(g.width));
        if total > self.g.width.raw() {
            return Err(FolioError::Unsupported("glyph wider than its line"));
        }
        let styles: Vec<Style> = if gs.is_empty() {
            vec![self.base]
        } else {
            gs.iter()
                .filter_map(|g| self.segs.get(g.seg).map(|s| s.style))
                .collect()
        };
        let (height, baseline) = line_metrics(self.ctx, &styles)?;
        let shift = match self.g.align {
            Align::Left => Emu::ZERO,
            Align::Right => self.g.width.checked_sub(Emu::from_raw(total))?,
        };
        let mut f = Frag {
            height,
            baseline,
            ..Frag::default()
        };
        let mut x = self.g.x0.checked_add(shift)?;
        for g in &gs {
            x = self.group(&mut f, g, x, baseline)?;
            f.subst = f.subst.saturating_add(g.subst);
        }
        if let Some(ItemKind::Text(t)) = f
            .items
            .iter_mut()
            .rev()
            .map(|i| &mut i.kind)
            .find(|k| matches!(k, ItemKind::Text(_)))
        {
            t.space_after = dropped;
        }
        Ok(f)
    }

    /// Emit the items of one group at `x`; returns the x after it.
    fn group(&self, f: &mut Frag, g: &Group, x: Emu, baseline: Emu) -> Result<Emu> {
        let Some(seg) = self.segs.get(g.seg) else {
            return Ok(x);
        };
        let st = seg.style;
        let (asc, desc) = self.ctx.fonts.extents(st.slot, st.size)?;
        let w = Emu::from_raw(g.width);
        let top = baseline.checked_sub(asc)?;
        let boxh = asc.checked_add(desc)?;
        if seg.highlight {
            f.items.push(Item {
                node: seg.node,
                role: StructRole::Artifact,
                x,
                y: top,
                kind: ItemKind::Rect(RectItem {
                    w,
                    h: boxh,
                    fill: Some(self.ctx.theme.highlight_ink),
                    stroke: None,
                }),
                repeated: false,
            });
        }
        if seg.field_digits > 0 {
            f.fields.push(f.items.len());
        }
        f.items.push(Item {
            node: seg.node,
            role: seg.role,
            x,
            y: baseline,
            kind: ItemKind::Text(TextItem {
                font: st.slot,
                size: st.size,
                ink: st.ink,
                width: w,
                ascent: asc,
                descent: desc,
                text: g.text.clone(),
                space_after: false,
            }),
            repeated: false,
        });
        if let Some(target) = &seg.link {
            f.links.push(LinkAnnot {
                node: seg.node,
                rect: BoxRect {
                    x,
                    y: top,
                    w,
                    h: boxh,
                },
                target: link_target(target)?,
            });
        }
        x.checked_add(w)
    }
}

/// `#name` is a named destination, anything else an external address.
pub(crate) fn link_target(t: &str) -> Result<LinkTarget> {
    match t.strip_prefix('#') {
        _ if t.is_empty() => Err(FolioError::Invalid("link target is empty")),
        Some("") => Err(FolioError::Invalid("link to an empty anchor name")),
        Some(name) => Ok(LinkTarget::Named(name.to_string())),
        None => Ok(LinkTarget::Uri(t.to_string())),
    }
}

/// Lay out `segs` as lines no wider than `g.width`.
///
/// # Errors
/// `Unsupported` for a character the font lacks (under `Reject`) or a glyph wider than the line,
/// `Limit` when the item budget is spent.
pub(crate) fn layout_lines(ctx: &Ctx<'_>, segs: &[Seg], g: Geom, base: Style) -> Result<Vec<Frag>> {
    let m = measure(ctx, segs)?;
    let breaks = line_breaks(&m.full);
    let ends = wrap(&m.full, &m.widths, g.width.raw(), &breaks);
    let b = Build {
        ctx,
        segs,
        m: &m,
        g,
        base,
    };
    let mut out = Vec::new();
    if ends.is_empty() {
        let mut f = b.line(0..0)?;
        attach_anchors(&b, &mut f, 0, 0, true);
        out.push(f);
        return Ok(out);
    }
    let (mut ci, mut ai) = (0_usize, 0_usize);
    let last = ends.len().saturating_sub(1);
    for (k, end) in ends.iter().enumerate() {
        let mut cj = ci;
        while m.chars.get(cj).is_some_and(|(o, _)| *o < *end) {
            cj = cj.saturating_add(1);
        }
        let mut frag = b.line(ci..cj)?;
        ai = attach_anchors(&b, &mut frag, ai, *end, k == last);
        out.push(frag);
        ci = cj;
    }
    Ok(out)
}

/// Give `frag` the anchors that start before byte `end` (all the rest on the last line).
fn attach_anchors(
    b: &Build<'_, '_>,
    frag: &mut Frag,
    from: usize,
    end: usize,
    last: bool,
) -> usize {
    let mut ai = from;
    while let Some(&(off, si)) = b.m.anchors.get(ai) {
        if !(last || off < end) {
            break;
        }
        if let Some(name) = b.segs.get(si).and_then(|s| s.anchor.clone()) {
            frag.dests.push(RelDest {
                name,
                x: b.g.x0,
                y: Emu::ZERO,
            });
        }
        ai = ai.saturating_add(1);
    }
    ai
}
