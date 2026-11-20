# tool-typeset

A zero-dependency document rendering engine in Rust. One internal `Doc` model
(`doc.rs`), two independent backends: `render_html` (review) and `render_pdf`
(final deliverable, written directly, never HTML-to-PDF). Cells carry
pre-formatted strings, never money or floats, so the crate is domain-agnostic.

```rust
use tool_typeset::{render_html, render_pdf, Doc};
// Build a `Doc` (see doc.rs: Block, Table, Timeline, PageSetup), then:
// let pdf: Vec<u8> = render_pdf(&doc);
// let html: String = render_html(&doc);
```

## Status: frozen but maintained

This crate is the current renderer for several consumers. It is **frozen**:
no new features. It is **maintained**: bug fixes and security fixes continue
here. New capability (DOCX, XLSX, CSV, TrueType fonts, charts, PDF assembly)
is built in the planned native `moonshot-folio` family, which will replace
this crate one consumer at a time. Fixes found in the new family that apply
here come back as minimal patches with a reproducing test.

Known limits: Helvetica-family standard-14 fonts only, no images, links,
bookmarks or tables of contents, HTML and PDF only.

## Rules

- Zero dependencies; keep it that way.
- Domain-agnostic: no business content, names or figures in code, tests or
  comments. Consumers keep real documents in their own archives.
- Output is deterministic: the same `Doc` renders to the same bytes.

## Test

```
cargo test -p tool-typeset
cargo clippy -p tool-typeset --all-targets -- -D warnings
cargo fmt -p tool-typeset -- --check
```

## Licence

Apache-2.0.
