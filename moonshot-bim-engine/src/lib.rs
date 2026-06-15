//! moonshot-bim-engine — IFC (ISO 16739) STEP-file structural parser.
//!
//! Replaces the structural-parse role of web-ifc (MPL) and xeokit (commercial) — the
//! dependency recorded as the `app-workplace-bim` licensing gate. v0 parses the
//! IFC-SPF container (STEP Physical File, ISO 10303-21): the HEADER records and the
//! DATA instances (`#id = ENTITY(params)`), string- and paren-aware so commas and
//! parentheses inside string literals or nested lists never mis-split. The full STEP
//! value grammar (typed values, enums, refs) and geometry tessellation are the
//! documented next layers.
//!
//! **SYS-ADR-07:** IFC is structured geometric/property data — it must never transit
//! any AI inference layer. This parser is deterministic Rust only. Zero dependencies,
//! WASM-ready; the IFC fiduciary record stays plain, open, and 50-year-readable.

/// One `#id = ENTITY(params)` record from the DATA section. `params` is the raw,
/// unsplit inner text (use [`split_params`] for top-level fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub id: u64,
    pub entity: String,
    pub params: String,
}

/// One `KEYWORD(params)` record from the HEADER section (FILE_DESCRIPTION, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub keyword: String,
    pub params: String,
}

/// A parsed IFC-SPF file: header records and data instances.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StepFile {
    pub header: Vec<Record>,
    pub data: Vec<Instance>,
}

impl StepFile {
    /// The instance with the given `#id`, if present.
    pub fn instance(&self, id: u64) -> Option<&Instance> {
        self.data.iter().find(|i| i.id == id)
    }

    /// Every instance of a given IFC entity type (e.g. `"IFCWALL"`).
    pub fn instances_of<'a>(&'a self, entity: &str) -> Vec<&'a Instance> {
        self.data.iter().filter(|i| i.entity == entity).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No `DATA;` section was found.
    MissingData,
    /// Structural break at the given byte offset.
    Malformed(usize),
}

/// Parse an IFC-SPF source into its header records and data instances.
pub fn parse(source: &str) -> Result<StepFile, ParseError> {
    let data_range = section_body(source, "DATA;").ok_or(ParseError::MissingData)?;
    let data = parse_instances(source, data_range)?;
    let header = match section_body(source, "HEADER;") {
        Some(r) => parse_records(source, r)?,
        None => Vec::new(),
    };
    Ok(StepFile { header, data })
}

/// Split a raw `params` string into its top-level fields, respecting STEP string
/// literals (`'...'` with `''` escape) and nested parentheses. Empty params -> `[]`.
pub fn split_params(params: &str) -> Vec<String> {
    if params.trim().is_empty() {
        return Vec::new();
    }
    let b = params.as_bytes();
    let n = b.len();
    let mut out = Vec::new();
    let mut i = 0;
    let mut field_start = 0;
    let mut depth = 0i32;
    while i < n {
        match b[i] {
            b'\'' => i = scan_string(b, i, n),
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
            }
            b',' if depth == 0 => {
                out.push(params[field_start..i].trim().to_string());
                i += 1;
                field_start = i;
            }
            _ => i += 1,
        }
    }
    out.push(params[field_start..].trim().to_string());
    out
}

// ---- internals --------------------------------------------------------------

/// Byte range between a section keyword (e.g. `"DATA;"`) and its following `ENDSEC;`.
fn section_body(source: &str, kw: &str) -> Option<(usize, usize)> {
    let k = source.find(kw)?;
    let start = k + kw.len();
    let end_rel = source[start..].find("ENDSEC;")?;
    Some((start, start + end_rel))
}

fn skip_ws_comments(b: &[u8], mut i: usize, end: usize) -> usize {
    loop {
        while i < end && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i + 1 < end && b[i] == b'/' && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < end && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            i = if i + 1 < end { i + 2 } else { end };
        } else {
            break;
        }
    }
    i
}

/// `i` points at the opening `'`. Returns the index just past the closing quote,
/// treating `''` as an escaped quote.
fn scan_string(b: &[u8], mut i: usize, end: usize) -> usize {
    i += 1;
    while i < end {
        if b[i] == b'\'' {
            if i + 1 < end && b[i + 1] == b'\'' {
                i += 2;
            } else {
                return i + 1;
            }
        } else {
            i += 1;
        }
    }
    i
}

/// `i` points at `(`. Returns `(inner_start, inner_end, after_close)` for the
/// balanced group, string-aware.
fn scan_group(b: &[u8], mut i: usize, end: usize) -> Result<(usize, usize, usize), ParseError> {
    let open = i;
    i += 1;
    let inner_start = i;
    let mut depth = 1i32;
    while i < end && depth > 0 {
        match b[i] {
            b'\'' => i = scan_string(b, i, end),
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok((inner_start, i, i + 1));
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    Err(ParseError::Malformed(open))
}

fn parse_instances(
    source: &str,
    (start, end): (usize, usize),
) -> Result<Vec<Instance>, ParseError> {
    let b = source.as_bytes();
    let mut cur = start;
    let mut out = Vec::new();
    loop {
        cur = skip_ws_comments(b, cur, end);
        if cur >= end {
            break;
        }
        if b[cur] != b'#' {
            // Resync to the next instance rather than failing the whole parse.
            match (cur + 1..end).find(|&j| b[j] == b'#') {
                Some(p) => {
                    cur = p;
                    continue;
                }
                None => break,
            }
        }
        cur += 1;
        let id_start = cur;
        while cur < end && b[cur].is_ascii_digit() {
            cur += 1;
        }
        if cur == id_start {
            return Err(ParseError::Malformed(cur));
        }
        let id: u64 = source[id_start..cur]
            .parse()
            .map_err(|_| ParseError::Malformed(id_start))?;
        cur = skip_ws_comments(b, cur, end);
        if cur >= end || b[cur] != b'=' {
            return Err(ParseError::Malformed(cur));
        }
        cur = skip_ws_comments(b, cur + 1, end);
        let kw_start = cur;
        while cur < end && (b[cur].is_ascii_alphanumeric() || b[cur] == b'_') {
            cur += 1;
        }
        let entity = source[kw_start..cur].to_string();
        cur = skip_ws_comments(b, cur, end);
        if cur >= end || b[cur] != b'(' {
            return Err(ParseError::Malformed(cur));
        }
        let (ps, pe, after) = scan_group(b, cur, end)?;
        out.push(Instance {
            id,
            entity,
            params: source[ps..pe].to_string(),
        });
        cur = skip_ws_comments(b, after, end);
        if cur < end && b[cur] == b';' {
            cur += 1;
        }
    }
    Ok(out)
}

fn parse_records(source: &str, (start, end): (usize, usize)) -> Result<Vec<Record>, ParseError> {
    let b = source.as_bytes();
    let mut cur = start;
    let mut out = Vec::new();
    loop {
        cur = skip_ws_comments(b, cur, end);
        if cur >= end {
            break;
        }
        let kw_start = cur;
        while cur < end && (b[cur].is_ascii_alphanumeric() || b[cur] == b'_') {
            cur += 1;
        }
        if cur == kw_start {
            break; // not a record keyword
        }
        let keyword = source[kw_start..cur].to_string();
        cur = skip_ws_comments(b, cur, end);
        if cur >= end || b[cur] != b'(' {
            return Err(ParseError::Malformed(cur));
        }
        let (ps, pe, after) = scan_group(b, cur, end)?;
        out.push(Record {
            keyword,
            params: source[ps..pe].to_string(),
        });
        cur = skip_ws_comments(b, after, end);
        if cur < end && b[cur] == b';' {
            cur += 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('example.ifc','2024-01-01T00:00:00',(''),(''),'','','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
#1=IFCPROJECT('0YvhhKNkr0kugbFTf53O9L',#2,'My Project',$,$,$,$,(#11),#7);
#2=IFCOWNERHISTORY(#3,#6,$,.ADDED.,$,$,$,1234);
#11=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.0E-5,#12,$);
ENDSEC;
END-ISO-10303-21;
";

    #[test]
    fn parses_data_instances() {
        let f = parse(SAMPLE).unwrap();
        assert_eq!(f.data.len(), 3);
        assert_eq!(f.data[0].id, 1);
        assert_eq!(f.data[0].entity, "IFCPROJECT");
        assert_eq!(f.instance(2).unwrap().entity, "IFCOWNERHISTORY");
        assert_eq!(
            f.instance(11).unwrap().entity,
            "IFCGEOMETRICREPRESENTATIONCONTEXT"
        );
        assert!(f.instance(99).is_none());
    }

    #[test]
    fn parses_header_records() {
        let f = parse(SAMPLE).unwrap();
        let kws: Vec<&str> = f.header.iter().map(|r| r.keyword.as_str()).collect();
        assert_eq!(kws, ["FILE_DESCRIPTION", "FILE_NAME", "FILE_SCHEMA"]);
        assert_eq!(f.header[2].params, "('IFC4')");
    }

    #[test]
    fn instances_of_filters_by_entity() {
        let f = parse(SAMPLE).unwrap();
        let projects = f.instances_of("IFCPROJECT");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, 1);
    }

    #[test]
    fn split_params_is_string_and_paren_aware() {
        let f = parse(SAMPLE).unwrap();
        let fields = split_params(&f.instance(1).unwrap().params);
        // 9 fields; the list (#11) and the strings stay intact.
        assert_eq!(fields.len(), 9);
        assert_eq!(fields[0], "'0YvhhKNkr0kugbFTf53O9L'");
        assert_eq!(fields[1], "#2");
        assert_eq!(fields[2], "'My Project'");
        assert_eq!(fields[3], "$");
        assert_eq!(fields[7], "(#11)"); // nested list not split
    }

    #[test]
    fn commas_and_parens_inside_strings_do_not_split() {
        // The string field contains a comma and parens — must remain one field.
        let p = "'a, b (c)',#5,'x'";
        let fields = split_params(p);
        assert_eq!(fields, vec!["'a, b (c)'", "#5", "'x'"]);
    }

    #[test]
    fn escaped_quotes_in_strings_are_handled() {
        let src = "DATA;\n#1=IFCLABEL('it''s a wall',#2);\nENDSEC;\n";
        let f = parse(src).unwrap();
        let fields = split_params(&f.instance(1).unwrap().params);
        assert_eq!(fields[0], "'it''s a wall'");
        assert_eq!(fields[1], "#2");
    }

    #[test]
    fn missing_data_section_is_an_error() {
        assert_eq!(
            parse("ISO-10303-21;\nHEADER;\nENDSEC;\n"),
            Err(ParseError::MissingData)
        );
    }

    #[test]
    fn split_params_empty_is_no_fields() {
        assert!(split_params("").is_empty());
        assert!(split_params("   ").is_empty());
    }
}
