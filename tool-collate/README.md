# tool-collate

The shared page reader. It walks a folder of Markdown pages, reads each page's frontmatter (optional: ok, missing or invalid) and structure (headings, links, tables, fenced code blocks), and reports counts. Read-only; it writes nothing and holds no rule about any one kind of page.

```
tool-collate scan <folder> [--keep <hidden-folder>]...
```

The report is counts only: pages found and read, unreadable files, frontmatter ok / missing / invalid, relative links checked and broken, links not checked (a scheme, a `#anchor` or a `/site/path`), and folders skipped by rule. Hidden folders, `target`, `node_modules` and `vendor-*` are skipped unless `--keep` names the hidden folder.

Tools for particular kinds of page (wiki, research, agents) depend on this crate for the reading and bring their own rules. The folder walk is the shared `walk` of `tool-precedent-core`. The only third-party dependency is `serde_yaml`, because real frontmatter (nested lists, block scalars, escapes) is misread by a hand-written parser.

Tests use made-up pages in `tests/fixtures/pages` only.

Licence: Apache-2.0.
