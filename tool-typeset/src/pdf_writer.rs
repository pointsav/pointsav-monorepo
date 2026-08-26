//! Minimal, zero-dependency PDF object/xref/trailer writer. Emits
//! uncompressed streams deliberately -- bigger files, but the PDF stays
//! greppable and git-diffable, matching this platform's plain-text bias.

pub struct PdfWriter {
    objects: Vec<Vec<u8>>, // index 0 == object number 1
}

impl Default for PdfWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfWriter {
    pub fn new() -> Self {
        PdfWriter {
            objects: Vec::new(),
        }
    }

    /// Reserves the next object number without writing content yet --
    /// used to forward-reference an object (e.g. a Page referencing its
    /// not-yet-built content stream) before its bytes are known.
    pub fn reserve(&mut self) -> u32 {
        self.objects.push(Vec::new());
        self.objects.len() as u32
    }

    pub fn set_object(&mut self, obj_num: u32, body: Vec<u8>) {
        self.objects[(obj_num - 1) as usize] = body;
    }

    pub fn add_object(&mut self, body: Vec<u8>) -> u32 {
        self.objects.push(body);
        self.objects.len() as u32
    }

    /// `catalog_obj` must reference a `/Type /Catalog` object.
    pub fn finish(self, catalog_obj: u32) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"%PDF-1.4\n");
        // A binary-marker comment is conventional but not required; skip it
        // since this crate deliberately produces text-mostly, diffable PDFs.

        let mut offsets = vec![0u64; self.objects.len() + 1]; // 1-indexed
        for (i, body) in self.objects.iter().enumerate() {
            let obj_num = (i + 1) as u32;
            offsets[obj_num as usize] = out.len() as u64;
            out.extend_from_slice(format!("{obj_num} 0 obj\n").as_bytes());
            out.extend_from_slice(body);
            out.extend_from_slice(b"\nendobj\n");
        }

        let xref_offset = out.len() as u64;
        let count = self.objects.len() + 1;
        out.extend_from_slice(format!("xref\n0 {count}\n").as_bytes());
        out.extend_from_slice(b"0000000000 65535 f \n");
        for offset in &offsets[1..count] {
            out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size {count} /Root {catalog_obj} 0 R >>\nstartxref\n{xref_offset}\n%%EOF"
            )
            .as_bytes(),
        );
        out
    }
}

/// Maps a Unicode scalar value to its WinAnsiEncoding (Windows-1252) byte,
/// for the characters real prose content commonly uses beyond plain ASCII
/// (names/labels lean on em-dash constantly, e.g. "Fund — Cash"). WinAnsi is
/// NOT simply "Unicode codepoint == byte value for codepoints under 256": it
/// diverges from Latin-1 exactly in the 0x80-0x9F byte range, which is where
/// em/en-dash and curly quotes live. Returns None for anything genuinely
/// unmapped (v1 scope: WinAnsi only).
fn winansi_byte(c: char) -> Option<u8> {
    let code = c as u32;
    if code < 0x80 {
        return Some(code as u8); // plain ASCII, identity mapping
    }
    match c {
        '\u{2013}' => Some(0x96), // en dash
        '\u{2014}' => Some(0x97), // em dash
        '\u{2018}' => Some(0x91), // left single quote
        '\u{2019}' => Some(0x92), // right single quote
        '\u{201C}' => Some(0x93), // left double quote
        '\u{201D}' => Some(0x94), // right double quote
        '\u{2022}' => Some(0x95), // bullet
        '\u{2026}' => Some(0x85), // ellipsis
        // 0xA0-0xFF: WinAnsi matches Latin-1/ISO-8859-1 directly in this range.
        _ if (0xA0..=0xFF).contains(&code) => Some(code as u8),
        _ => None,
    }
}

/// Escapes a string for use inside a PDF literal string, `( ... )`, and
/// encodes it to WinAnsi bytes. Bytes >= 128 are written as PDF octal
/// escapes (`\ddd`) rather than pushed as literal Rust chars -- a Rust
/// `char` at codepoint 0x97 UTF-8-encodes to two bytes (0xC2 0x97), not
/// the single raw byte 0x97 a WinAnsi-encoded PDF font actually needs, so
/// octal escaping is the only correct way to reach those byte values while
/// still building this as an ordinary (ASCII-safe) Rust String.
pub fn escape_pdf_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c => match winansi_byte(c) {
                Some(b) if b < 0x80 => out.push(c),
                Some(b) => out.push_str(&format!("\\{b:03o}")),
                None => out.push('?'),
            },
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn em_dash_encodes_as_winansi_octal_escape() {
        assert_eq!(escape_pdf_string("Fund \u{2014} Cash"), "Fund \\227 Cash");
    }

    #[test]
    fn plain_ascii_passes_through() {
        assert_eq!(escape_pdf_string("Bank charges"), "Bank charges");
    }

    #[test]
    fn parens_and_backslash_are_escaped() {
        assert_eq!(escape_pdf_string("(a\\b)"), "\\(a\\\\b\\)");
    }
}
