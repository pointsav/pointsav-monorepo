// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! IR validation. Backends call [`validate`] first and may then assume every invariant here.

use crate::model::{
    Block, Body, Cell, CellValue, Document, Heading, Image, Meta, PageSetup, Row, Section, Sheet,
    Table,
};
use moonshot_folio_core::{Emu, FolioError, Limits, Result, EMU_PER_INCH};

/// Check a document against the IR invariants and the supplied limits.
///
/// # Errors
/// `Invalid` for a broken invariant (message names it), `Limit` when `limits` is exceeded.
pub fn validate(doc: &Document, limits: &Limits) -> Result<()> {
    check_meta(&doc.meta)?;
    let mut cells = 0_usize;
    match &doc.body {
        Body::Flow(sections) => {
            if sections.is_empty() {
                return Err(FolioError::Invalid("flow body has no sections"));
            }
            for s in sections {
                check_section(s, limits, &mut cells)?;
            }
        }
        Body::Workbook(sheets) => {
            if sheets.is_empty() {
                return Err(FolioError::Invalid("workbook has no sheets"));
            }
            for (i, s) in sheets.iter().enumerate() {
                check_sheet(s, limits, &mut cells)?;
                let dup = sheets
                    .iter()
                    .take(i)
                    .any(|p| p.name.to_lowercase() == s.name.to_lowercase());
                if dup {
                    return Err(FolioError::Invalid("duplicate sheet name"));
                }
                for n in &s.names {
                    let clash = sheets
                        .iter()
                        .take(i)
                        .flat_map(|p| &p.names)
                        .chain(s.names.iter().take_while(|m| !core::ptr::eq(*m, n)))
                        .any(|m| m.name.to_lowercase() == n.name.to_lowercase());
                    if clash {
                        return Err(FolioError::Invalid("duplicate defined name"));
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_meta(m: &Meta) -> Result<()> {
    let lang_ok = !m.lang.is_empty()
        && m.lang.len() <= 35
        && m.lang
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-');
    if !lang_ok {
        return Err(FolioError::Invalid("lang must be a BCP 47 tag"));
    }
    if !valid_date(&m.date) {
        return Err(FolioError::Invalid("date must be YYYY-MM-DD"));
    }
    Ok(())
}

/// True for a calendar date written `YYYY-MM-DD`: a month from 1 to 12 and a day within the
/// month (Gregorian leap years: divisible by 4, except centuries not divisible by 400).
#[must_use]
pub fn valid_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    let shape = bytes.len() == 10
        && bytes.iter().enumerate().all(|(i, c)| {
            if i == 4 || i == 7 {
                *c == b'-'
            } else {
                c.is_ascii_digit()
            }
        });
    if !shape {
        return false;
    }
    let num = |r: core::ops::Range<usize>| -> Option<u32> { text.get(r)?.parse().ok() };
    let (Some(year), Some(month), Some(day)) = (num(0..4), num(5..7), num(8..10)) else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

/// Largest page side the writers accept, in EMU (200 inches).
const MAX_PAGE_SIDE: i64 = EMU_PER_INCH * 200;

fn check_page(p: &PageSetup) -> Result<()> {
    let twice = p.margin.checked_mul(2)?;
    let ok = p.width > Emu::ZERO
        && p.height > Emu::ZERO
        && p.width.raw() <= MAX_PAGE_SIDE
        && p.height.raw() <= MAX_PAGE_SIDE
        && p.margin >= Emu::ZERO
        && twice < p.width
        && twice < p.height;
    if ok {
        Ok(())
    } else {
        Err(FolioError::Invalid("page size or margin out of range"))
    }
}

fn check_section(s: &Section, limits: &Limits, cells: &mut usize) -> Result<()> {
    check_page(&s.page)?;
    for b in s.blocks.iter().chain(&s.header).chain(&s.footer) {
        match b {
            Block::Heading(h) => check_heading(h)?,
            Block::Table(t) => check_table(t, limits, cells)?,
            Block::Image(i) => check_image(i, limits)?,
            Block::List(l) if l.items.is_empty() => {
                return Err(FolioError::Invalid("list has no items"));
            }
            Block::List(_) | Block::Para(_) | Block::Field(_) | Block::Break(_) => {}
        }
    }
    Ok(())
}

fn check_heading(h: &Heading) -> Result<()> {
    if (1..=6).contains(&h.level) {
        Ok(())
    } else {
        Err(FolioError::Invalid("heading level must be 1 to 6"))
    }
}

fn check_image(i: &Image, limits: &Limits) -> Result<()> {
    if i.alt_text.trim().is_empty() {
        return Err(FolioError::Invalid("image needs alt text"));
    }
    if i.data.is_empty() || i.width <= Emu::ZERO || i.height <= Emu::ZERO {
        return Err(FolioError::Invalid("image data or size is empty"));
    }
    if i.data.len() > limits.max_entry_bytes() {
        return Err(FolioError::Limit("image bytes"));
    }
    Ok(())
}

fn check_table(t: &Table, limits: &Limits, cells: &mut usize) -> Result<()> {
    check_grid(t.columns, &t.rows, limits, cells)?;
    let header = usize::try_from(t.repeat_header_rows).unwrap_or(usize::MAX);
    if header > t.rows.len() {
        return Err(FolioError::Invalid("more header rows than rows"));
    }
    Ok(())
}

fn check_sheet(s: &Sheet, limits: &Limits, cells: &mut usize) -> Result<()> {
    // Excel counts UTF-16 units, forbids a leading or trailing apostrophe and reserves "History".
    let n = s.name.encode_utf16().count();
    let bad = s
        .name
        .chars()
        .any(|c| "[]:*?/\\".contains(c) || c.is_control())
        || s.name.starts_with('\'')
        || s.name.ends_with('\'')
        || s.name.eq_ignore_ascii_case("history");
    if n == 0 || n > 31 || bad {
        return Err(FolioError::Invalid("sheet name rule violated"));
    }
    check_grid(s.columns, &s.rows, limits, cells)?;
    crate::validate_sheet::check_extras(s, limits)?;
    let freeze = usize::try_from(s.freeze_rows).unwrap_or(usize::MAX);
    if freeze > s.rows.len() {
        return Err(FolioError::Invalid("more frozen rows than rows"));
    }
    Ok(())
}

/// Shared grid rule: spans are at least 1 and each row spans exactly `columns`.
fn check_grid(columns: u32, rows: &[Row], limits: &Limits, cells: &mut usize) -> Result<()> {
    if columns == 0 {
        return Err(FolioError::Invalid("grid has zero columns"));
    }
    for r in rows {
        let mut span_sum = 0_u64;
        for c in &r.cells {
            check_cell(c)?;
            span_sum = span_sum.saturating_add(u64::from(c.span));
        }
        if span_sum != u64::from(columns) {
            return Err(FolioError::Invalid("row spans do not equal column count"));
        }
        *cells = cells.saturating_add(r.cells.len());
        if *cells > limits.max_cells() {
            return Err(FolioError::Limit("cells"));
        }
    }
    Ok(())
}

fn check_cell(c: &Cell) -> Result<()> {
    if c.span == 0 {
        return Err(FolioError::Invalid("cell span is zero"));
    }
    if let CellValue::Number(n) = &c.value {
        if !is_decimal(n) {
            return Err(FolioError::Invalid("number is not a plain decimal"));
        }
    }
    Ok(())
}

/// `-?digits(.digits)?` with no exponent, sign other than a leading minus, or separators.
pub(crate) fn is_decimal(s: &str) -> bool {
    let body = s.strip_prefix('-').unwrap_or(s);
    let (int, frac) = match body.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (body, None),
    };
    let digits = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    // Excel keeps 15 significant digits; 40 characters is generous and keeps values finite.
    s.len() <= 40 && digits(int) && frac.is_none_or(digits)
}
