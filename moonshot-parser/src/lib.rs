//! moonshot-parser — span-based source tokenizer.
//!
//! Replaces tree-sitter and the hand-rolled JavaScript regex tokenizer in
//! `app-privategit-workbench` (which is brittle on nested constructs and bypasses
//! highlighting above a fixed file size). This v0 is the tokenizer foundation:
//! a single pass that classifies the source into [`Token`]s, each carrying its
//! exact byte [`Span`]. The token stream drives syntax highlighting — every token
//! maps to one colored span — and feeds source ranges to the document model.
//!
//! Two invariants, matching the source-anchoring philosophy of `moonshot-docengine`:
//!
//! 1. **Contiguous coverage.** Every byte of the source belongs to exactly one
//!    token (whitespace and newlines are tokens too), so concatenating the token
//!    spans reproduces the input — highlighting is exact and reversible.
//! 2. **UTF-8 safe.** Token boundaries land only on ASCII delimiters or full-char
//!    boundaries; multibyte content is spanned, never split.
//!
//! Zero dependencies, WASM-ready. Incremental re-lex (re-tokenizing only the lines
//! an edit touched, until the lexer state reconverges) is the documented next
//! increment; the single-pass [`tokenize`] here is its correctness oracle.

/// A byte range `[start, end)` into the source. (Mirrors `moonshot-docengine::Span`;
/// a shared span crate is a planned consolidation once the cores integrate.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

/// Token classification, sufficient to drive a highlighter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Ident,
    Number,
    Str,
    Comment,
    Punct,
    Whitespace,
}

/// A classified token plus its exact source span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Source language. `Generic` skips keyword classification (everything word-like is
/// an `Ident`); add languages by extending [`is_keyword`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Rust,
    Generic,
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

fn is_keyword(lang: Lang, word: &str) -> bool {
    match lang {
        Lang::Rust => RUST_KEYWORDS.contains(&word),
        Lang::Generic => false,
    }
}

fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Byte length of the UTF-8 char starting at `i` (1 if `i` is out of range or not a
/// char boundary head — defensive).
fn char_len(src: &str, i: usize) -> usize {
    src[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1)
}

/// Tokenize `source` in a single pass. The result contiguously covers the whole
/// source (see module invariants).
pub fn tokenize(source: &str, lang: Lang) -> Vec<Token> {
    let b = source.as_bytes();
    let n = b.len();
    let mut out = Vec::new();
    let mut i = 0;

    while i < n {
        let start = i;
        let c = b[i];

        if is_ws(c) {
            while i < n && is_ws(b[i]) {
                i += 1;
            }
            push(&mut out, TokenKind::Whitespace, start, i);
        } else if c == b'/' && i + 1 < n && b[i + 1] == b'/' {
            while i < n && b[i] != b'\n' {
                i += 1;
            }
            push(&mut out, TokenKind::Comment, start, i);
        } else if c == b'/' && i + 1 < n && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < n && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            i = if i + 1 < n { i + 2 } else { n }; // consume closing */ or run to EOF
            push(&mut out, TokenKind::Comment, start, i);
        } else if c == b'"' {
            i += 1;
            while i < n {
                if b[i] == b'\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if b[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push(&mut out, TokenKind::Str, start, i);
        } else if c.is_ascii_digit() {
            while i < n && (b[i].is_ascii_alphanumeric() || b[i] == b'.' || b[i] == b'_') {
                i += 1;
            }
            push(&mut out, TokenKind::Number, start, i);
        } else if source[i..].chars().next().is_some_and(is_ident_start) {
            while let Some(ch) = source[i..].chars().next() {
                if is_ident_continue(ch) {
                    i += ch.len_utf8();
                } else {
                    break;
                }
            }
            let kind = if is_keyword(lang, &source[start..i]) {
                TokenKind::Keyword
            } else {
                TokenKind::Ident
            };
            push(&mut out, kind, start, i);
        } else {
            i += char_len(source, i); // single (possibly multibyte) punctuation char
            push(&mut out, TokenKind::Punct, start, i);
        }
    }
    out
}

fn push(out: &mut Vec<Token>, kind: TokenKind, start: usize, end: usize) {
    out.push(Token {
        kind,
        span: Span::new(start, end),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(toks: &[Token]) -> Vec<TokenKind> {
        toks.iter().map(|t| t.kind).collect()
    }

    /// Concatenating token spans must reproduce the source exactly.
    fn assert_covers(src: &str, lang: Lang) {
        let toks = tokenize(src, lang);
        let rebuilt: String = toks.iter().map(|t| t.span.text(src)).collect();
        assert_eq!(rebuilt, src, "token coverage is not contiguous for {src:?}");
    }

    #[test]
    fn classifies_rust_tokens() {
        let src = "let x = 42; // hi";
        let toks = tokenize(src, Lang::Rust);
        // let, ws, x, ws, =, ws, 42, ;, ws, //hi
        assert_eq!(toks[0].kind, TokenKind::Keyword);
        assert_eq!(toks[0].span.text(src), "let");
        assert!(toks
            .iter()
            .any(|t| t.kind == TokenKind::Number && t.span.text(src) == "42"));
        let comment = toks.iter().find(|t| t.kind == TokenKind::Comment).unwrap();
        assert_eq!(comment.span.text(src), "// hi");
        assert_covers(src, Lang::Rust);
    }

    #[test]
    fn block_comment_spans_multiple_lines() {
        let src = "a /* one\n two */ b";
        let toks = tokenize(src, Lang::Rust);
        let comment = toks.iter().find(|t| t.kind == TokenKind::Comment).unwrap();
        assert_eq!(comment.span.text(src), "/* one\n two */");
        assert_covers(src, Lang::Rust);
    }

    #[test]
    fn unterminated_block_comment_runs_to_eof() {
        let src = "x /* never closed";
        let toks = tokenize(src, Lang::Rust);
        let comment = toks.iter().find(|t| t.kind == TokenKind::Comment).unwrap();
        assert_eq!(comment.span.text(src), "/* never closed");
        assert_covers(src, Lang::Rust);
    }

    #[test]
    fn strings_handle_escaped_quotes() {
        let src = r#"s = "he\"llo";"#;
        let toks = tokenize(src, Lang::Rust);
        let string = toks.iter().find(|t| t.kind == TokenKind::Str).unwrap();
        assert_eq!(string.span.text(src), r#""he\"llo""#);
        assert_covers(src, Lang::Rust);
    }

    #[test]
    fn generic_lang_does_not_classify_keywords() {
        let src = "fn x";
        let rust = tokenize(src, Lang::Rust);
        let generic = tokenize(src, Lang::Generic);
        assert_eq!(rust[0].kind, TokenKind::Keyword);
        assert_eq!(generic[0].kind, TokenKind::Ident); // "fn" is just a word in Generic
    }

    #[test]
    fn utf8_content_is_spanned_not_split() {
        // The é and ☕ are multibyte; tokens must stay on char boundaries.
        let src = "let café = \"☕\"; // déjà";
        let toks = tokenize(src, Lang::Rust);
        assert_covers(src, Lang::Rust); // would panic on a mid-char slice
        assert!(toks
            .iter()
            .any(|t| t.kind == TokenKind::Ident && t.span.text(src) == "café"));
        let _ = kinds(&toks);
    }

    #[test]
    fn numbers_cover_hex_and_separators_and_suffixes() {
        for num in ["0xFF", "3.14", "1_000", "42u32"] {
            let toks = tokenize(num, Lang::Rust);
            assert_eq!(toks.len(), 1);
            assert_eq!(toks[0].kind, TokenKind::Number);
            assert_eq!(toks[0].span.text(num), num);
        }
    }
}
