// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Structural elements of a Markdown body: headings, links, fenced code blocks and pipe tables.
//! Plain Markdown only. Wiki links and generated blocks are read by the tools for those kinds.

/// One ATX heading (`#` through `######`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    /// Byte offset of the heading line's start within the body, for callers that need to
    /// report a location (e.g. "heading at byte 412").
    pub offset: usize,
}

/// A standard `[text](url)` link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLink {
    pub text: String,
    pub url: String,
    pub offset: usize,
}

/// A fenced code block, with its optional language tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
    /// Byte offset of the opening fence line's start.
    pub offset: usize,
    /// Byte offset just past the closing fence line (inclusive of its trailing newline) —
    /// callers that need to remove the whole block, fences included, use `offset..end`
    /// rather than reconstructing it from `content.len()` (which covers only the inner
    /// lines, not the ` ``` ` delimiters themselves).
    pub end: usize,
}

/// A pipe table's shape (columns and body rows), not its cell content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub columns: usize,
    pub rows: usize,
    pub offset: usize,
}

pub fn headings(body: &str) -> Vec<Heading> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        let stripped = trimmed.trim_start();
        if let Some(hashes) = leading_hashes(stripped) {
            let level = hashes as u8;
            let text = stripped[hashes..].trim().to_string();
            if !text.is_empty() {
                out.push(Heading {
                    level,
                    text,
                    offset,
                });
            }
        }
        offset += line.len();
    }
    out
}

fn leading_hashes(s: &str) -> Option<usize> {
    let count = s.chars().take_while(|&c| c == '#').count();
    if count == 0 || count > 6 {
        return None;
    }
    // Must be followed by a space (or be the whole line) to count as a real ATX heading,
    // not e.g. a stray "###" inside prose.
    match s.as_bytes().get(count) {
        Some(b' ') | None => Some(count),
        _ => None,
    }
}

pub fn markdown_links(body: &str) -> Vec<MarkdownLink> {
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Skip double brackets -- `[[...]]` would otherwise be misread as `[` + a nested `[...]`.
        if bytes[i] == b'[' && bytes.get(i + 1) == Some(&b'[') {
            i += 2;
            continue;
        }
        if bytes[i] == b'[' {
            if let Some(close) = body[i + 1..].find(']') {
                let text_end = i + 1 + close;
                if body.as_bytes().get(text_end + 1) == Some(&b'(') {
                    if let Some(paren_close) = body[text_end + 2..].find(')') {
                        let url_end = text_end + 2 + paren_close;
                        out.push(MarkdownLink {
                            text: body[i + 1..text_end].to_string(),
                            url: body[text_end + 2..url_end].to_string(),
                            offset: i,
                        });
                        i = url_end + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    out
}

pub fn code_blocks(body: &str) -> Vec<CodeBlock> {
    let mut out = Vec::new();
    let mut offset = 0usize;
    let mut lines = body.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if let Some(lang) = trimmed.strip_prefix("```") {
            let language = if lang.trim().is_empty() {
                None
            } else {
                Some(lang.trim().to_string())
            };
            let block_offset = offset;
            offset += line.len();
            let mut content = String::new();
            for inner in lines.by_ref() {
                let inner_trimmed = inner.trim_end_matches(['\n', '\r']);
                offset += inner.len();
                if inner_trimmed.trim_start() == "```" {
                    break;
                }
                content.push_str(inner);
            }
            out.push(CodeBlock {
                language,
                content,
                offset: block_offset,
                end: offset,
            });
            continue;
        }
        offset += line.len();
    }
    out
}

pub fn tables(body: &str) -> Vec<Table> {
    let mut out = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    let mut offset = 0usize;
    let mut line_offsets = Vec::with_capacity(lines.len());
    for l in &lines {
        line_offsets.push(offset);
        offset += l.len() + 1;
    }
    let mut i = 0;
    while i + 1 < lines.len() {
        let header = lines[i].trim();
        let sep = lines[i + 1].trim();
        if is_pipe_row(header) && is_separator_row(sep) {
            let columns = header.trim_matches('|').split('|').count();
            let mut rows = 0;
            let mut j = i + 2;
            while j < lines.len() && is_pipe_row(lines[j].trim()) {
                rows += 1;
                j += 1;
            }
            out.push(Table {
                columns,
                rows,
                offset: line_offsets[i],
            });
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

fn is_pipe_row(line: &str) -> bool {
    line.starts_with('|') && line.len() > 1
}

fn is_separator_row(line: &str) -> bool {
    is_pipe_row(line)
        && line.trim_matches('|').split('|').all(|cell| {
            !cell.trim().is_empty() && cell.trim().chars().all(|c| c == '-' || c == ':')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_by_level() {
        let hs = headings("# H1\n\nSome text\n\n## H2 One\n\ntext\n\n### H3\n");
        assert_eq!(hs.len(), 3);
        assert_eq!(
            hs[0],
            Heading {
                level: 1,
                text: "H1".into(),
                offset: 0
            }
        );
        assert_eq!((hs[1].level, hs[1].text.as_str()), (2, "H2 One"));
        assert_eq!(hs[2].level, 3);
    }

    #[test]
    fn hashes_without_a_space_are_not_headings() {
        let hs = headings("This is #notaheading in prose.\n## Real Heading\n");
        assert_eq!(hs.len(), 1);
        assert_eq!(hs[0].text, "Real Heading");
    }

    #[test]
    fn links_text_and_url() {
        let ls = markdown_links("See [Guide](docs/guide.md) and [Site](https://example.org/).");
        assert_eq!(ls.len(), 2);
        assert_eq!(
            (ls[0].text.as_str(), ls[0].url.as_str()),
            ("Guide", "docs/guide.md")
        );
        assert_eq!(ls[1].url, "https://example.org/");
    }

    #[test]
    fn double_brackets_are_not_read_as_links() {
        let ls = markdown_links("[[some-target|Shown]] and [Real](/real/).");
        assert_eq!(ls.len(), 1);
        assert_eq!(ls[0].url, "/real/");
    }

    #[test]
    fn fenced_block_with_language_and_span() {
        let body = "Some text.\n\n```bash\ngit status\ngit push\n```\n\nMore text.\n";
        let bs = code_blocks(body);
        assert_eq!(bs.len(), 1);
        assert_eq!(bs[0].language, Some("bash".into()));
        assert_eq!(bs[0].content, "git status\ngit push\n");
        assert!(body[bs[0].offset..bs[0].end].ends_with("```\n"));
    }

    #[test]
    fn pipe_table_shape() {
        let ts = tables("| a | b | c |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n");
        assert_eq!(ts.len(), 1);
        assert_eq!((ts[0].columns, ts[0].rows), (3, 2));
    }
}
