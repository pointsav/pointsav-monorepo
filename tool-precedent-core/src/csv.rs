//! Minimal CSV: quoted fields, doubled quotes, newlines inside quotes, "\n" line ends.
pub fn parse(s: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut f = String::new();
    let mut q = false;
    let mut it = s.chars().peekable();
    let mut any = false;
    while let Some(c) = it.next() {
        any = true;
        if q {
            if c == '"' {
                if it.peek() == Some(&'"') {
                    it.next();
                    f.push('"');
                } else {
                    q = false;
                }
            } else {
                f.push(c);
            }
        } else if c == '"' {
            q = true;
        } else if c == ',' {
            row.push(std::mem::take(&mut f));
        } else if c == '\n' {
            row.push(std::mem::take(&mut f));
            rows.push(std::mem::take(&mut row));
            any = false;
        } else if c == '\r' {
        } else {
            f.push(c);
        }
    }
    if any || !f.is_empty() || !row.is_empty() {
        row.push(f);
        rows.push(row);
    }
    rows
}
pub fn field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
pub fn write(rows: &[Vec<String>]) -> String {
    let mut o = String::new();
    for r in rows {
        o.push_str(&r.iter().map(|x| field(x)).collect::<Vec<_>>().join(","));
        o.push('\n');
    }
    o
}
