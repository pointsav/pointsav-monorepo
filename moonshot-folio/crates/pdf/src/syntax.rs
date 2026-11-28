// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! PDF lexical forms: escaped strings, names and integer-only decimals.

use core::fmt::Write;
use moonshot_folio_core::{Emu, FolioError, Ink, Result, EMU_PER_POINT};

/// A literal string `( ... )`: parentheses and backslashes are escaped, and every byte outside
/// printable ASCII becomes a three-digit octal escape, so the output is always 7-bit clean.
pub(crate) fn literal_string(bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![b'('];
    for &b in bytes {
        match b {
            b'(' | b')' | b'\\' => {
                out.push(b'\\');
                out.push(b);
            }
            0x20..=0x7E => out.push(b),
            _ => {
                let mut s = String::new();
                write!(s, "\\{b:03o}").ok();
                out.extend_from_slice(s.as_bytes());
            }
        }
    }
    out.push(b')');
    out
}

/// A hexadecimal string `<...>`.
pub(crate) fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::from("<");
    for b in bytes {
        write!(s, "{b:02X}").ok();
    }
    s.push('>');
    s
}

/// A PDF text string: printable ASCII as a literal string, anything else as UTF-16BE with a BOM.
pub(crate) fn text_string(s: &str) -> Vec<u8> {
    if s.bytes().all(|b| (0x20..=0x7E).contains(&b)) {
        return literal_string(s.as_bytes());
    }
    let mut raw = vec![0xFE, 0xFF];
    for u in s.encode_utf16() {
        raw.extend_from_slice(&u.to_be_bytes());
    }
    hex_string(&raw).into_bytes()
}

/// A name object. Bytes outside the regular printable range, delimiters and `#` become `#XX`.
pub(crate) fn name(s: &str) -> String {
    let mut out = String::from("/");
    for b in s.bytes() {
        let regular = (0x21..=0x7E).contains(&b) && !b"()<>[]{}/%#".contains(&b);
        if regular {
            out.push(char::from(b));
        } else {
            write!(out, "#{b:02X}").ok();
        }
    }
    out
}

/// Thousandths as a decimal with at most three fraction digits and no trailing zeros.
pub(crate) fn fixed3(milli: i64) -> String {
    let a = milli.unsigned_abs();
    let mut s = String::new();
    if milli < 0 {
        s.push('-');
    }
    write!(s, "{}", a / 1000).ok();
    let frac = a % 1000;
    if frac != 0 {
        let f = format!("{frac:03}");
        s.push('.');
        s.push_str(f.trim_end_matches('0'));
    }
    s
}

/// EMU to thousandths of a point, rounded half away from zero, in integer arithmetic only.
pub(crate) fn emu_to_milli(e: Emu) -> Result<i64> {
    let n = e
        .raw()
        .checked_mul(1000)
        .ok_or(FolioError::Limit("length overflow"))?;
    let (q, r) = (n / EMU_PER_POINT, n % EMU_PER_POINT);
    if r.unsigned_abs().saturating_mul(2) >= EMU_PER_POINT.unsigned_abs() {
        let step = if n < 0 { -1 } else { 1 };
        return q
            .checked_add(step)
            .ok_or(FolioError::Limit("length overflow"));
    }
    Ok(q)
}

/// A length in points, ready to write.
pub(crate) fn pt(e: Emu) -> Result<String> {
    Ok(fixed3(emu_to_milli(e)?))
}

/// A colour component 0..=255 as a decimal 0..=1.
fn comp(v: u8) -> String {
    let m = i64::from(v).saturating_mul(1000).saturating_add(127) / 255;
    fixed3(m)
}

/// The colour operands and the operator suffix for fill (`fill = true`) or stroke.
pub(crate) fn ink_ops(ink: Ink, fill: bool) -> String {
    match ink {
        Ink::Gray(g) => format!("{} {}\n", comp(g), if fill { "g" } else { "G" }),
        Ink::Rgb(r, g, b) => format!(
            "{} {} {} {}\n",
            comp(r),
            comp(g),
            comp(b),
            if fill { "rg" } else { "RG" }
        ),
        Ink::Cmyk(c, m, y, k) => format!(
            "{} {} {} {} {}\n",
            comp(c),
            comp(m),
            comp(y),
            comp(k),
            if fill { "k" } else { "K" }
        ),
    }
}

/// Map a Unicode character to its `WinAnsiEncoding` byte, if it has one.
pub(crate) fn winansi(c: char) -> Option<u8> {
    let cp = u32::from(c);
    if (0x20..=0x7E).contains(&cp) || (0xA0..=0xFF).contains(&cp) {
        return u8::try_from(cp).ok();
    }
    let b = match cp {
        0x20AC => 0x80,
        0x201A => 0x82,
        0x0192 => 0x83,
        0x201E => 0x84,
        0x2026 => 0x85,
        0x2020 => 0x86,
        0x2021 => 0x87,
        0x02C6 => 0x88,
        0x2030 => 0x89,
        0x0160 => 0x8A,
        0x2039 => 0x8B,
        0x0152 => 0x8C,
        0x017D => 0x8E,
        0x2018 => 0x91,
        0x2019 => 0x92,
        0x201C => 0x93,
        0x201D => 0x94,
        0x2022 => 0x95,
        0x2013 => 0x96,
        0x2014 => 0x97,
        0x02DC => 0x98,
        0x2122 => 0x99,
        0x0161 => 0x9A,
        0x203A => 0x9B,
        0x0153 => 0x9C,
        0x017E => 0x9E,
        0x0178 => 0x9F,
        _ => return None,
    };
    Some(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // REQ: FOLIO-REQ-048
    #[test]
    fn strings_and_names_are_escaped() {
        assert_eq!(
            literal_string(b"a(b)\\c\n\xE9"),
            b"(a\\(b\\)\\\\c\\012\\351)"
        );
        assert_eq!(name("A B/#(x)\u{e9}"), "/A#20B#2F#23#28x#29#C3#A9");
        assert_eq!(text_string("plain"), b"(plain)");
        assert_eq!(text_string("\u{e9}"), b"<FEFF00E9>");
        assert_eq!(text_string("a\nb"), b"<FEFF0061000A0062>");
    }

    // REQ: FOLIO-REQ-044
    #[test]
    fn numbers_come_from_integer_math() {
        assert_eq!(fixed3(0), "0");
        assert_eq!(fixed3(1500), "1.5");
        assert_eq!(fixed3(-250), "-0.25");
        assert_eq!(fixed3(595_276), "595.276");
        assert_eq!(emu_to_milli(Emu::from_raw(12_700)).ok(), Some(1000));
        assert_eq!(emu_to_milli(Emu::from_raw(-6_350)).ok(), Some(-500));
        assert_eq!(comp(255), "1");
        assert_eq!(comp(0), "0");
        assert_eq!(winansi('\u{20ac}'), Some(0x80));
        assert_eq!(winansi('\u{e9}'), Some(0xE9));
        assert_eq!(winansi('\u{2014}'), Some(0x97));
        assert_eq!(winansi('\u{65e5}'), None);
        assert_eq!(winansi('\n'), None);
    }
}
