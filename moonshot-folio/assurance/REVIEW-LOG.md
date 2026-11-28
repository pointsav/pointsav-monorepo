# Independent review log

One entry per merged phase. The reviewer must not be the author of the code under review.
Mandatory for anything that parses untrusted input (codec, font, PDF reader). Findings need a disposition: fixed, accepted (with reason) or deferred (with owner).

| Date | Phase | Reviewer | Scope | Findings | Disposition |
|---|---|---|---|---|---|
| 2026-11-21 | F1 (codec: ZIP, inflate/zlib, PNG, JPEG header, SHA-256, CRC-32, Adler-32; core Limits) | Fresh agent session (read-only; no prior context; own scratch dir) | Full read plus differential tests against zlib, hashlib, Python zipfile, unzip, PIL and an independent PNG oracle (3000 images); CRC-repairing mutation fuzz (5 x 60k); 49 source mutants against the test suite (all killed) | 11 findings: 1 high, 2 medium, 6 low, 2 info (details below) | See disposition table |

## F1 review findings and dispositions (2026-11-21)

| # | Sev | Finding | Disposition |
|---|---|---|---|
| 1 | high | `to_rgba8` quadratic in tRNS length (key rebuilt per pixel; tRNS length unchecked) | FIXED: tRNS length validated per colour type at decode; key parsed once to `Option<[u16;3]>`; test `a_huge_trns_chunk_is_rejected_at_decode_time` |
| 2 | medium | PNG chunk-order rules unenforced | FIXED: state machine (IHDR first, PLTE once and only where allowed, tRNS once after PLTE, consecutive IDAT, empty IEND, chunk-type letters and reserved bit); tests in `tests/png.rs` |
| 3 | med/low | Fixed Huffman tables rebuilt for every fixed block | FIXED: built once per stream; test `many_empty_fixed_blocks_decode_in_linear_time` |
| 4 | low | ZIP writer accepted names over 65,535 bytes (corrupt output) | FIXED: rejected regardless of `Limits`; test |
| 5 | low | Writer emitted 0xFFFF entries / 0xFFFFFFFF sizes and offsets that the reader treats as zip64 | FIXED: entries capped at 65,534, sizes and offsets below `u32::MAX`; boundary round-trip test |
| 6 | low | Entry data could overlap the central directory or other entries | FIXED: data end must not pass the central directory start; sorted span check rejects overlapping entries; tests |
| 7 | low | Duplicate JPEG SOF accepted (last wins) | FIXED: error on a second frame header |
| 8 | low | Incomplete single-symbol code of any length accepted | FIXED: matches zlib (incomplete only when the longest code is 1 bit); vectors verified against zlib |
| 9 | info | EOCD inside an archive comment is read as the archive | DOCUMENTED on `ZipArchive::parse`; test pins the behaviour (matches common tools) |
| 10 | low | Memory about 3x the entry cap; writer buffered past `max_file_bytes` | PARTLY FIXED: writer now checks `max_file_bytes` per entry. PNG path still holds IDAT, inflated and unfiltered copies. ACCEPTED for now; revisit with in-place unfilter and capacity hints in F5 when images are embedded (owner: project-source) |
| 11 | info | Names: drive letters and control characters; JPEG `Lf` and sampling factors; `Limits` setters missing | FIXED: names, JPEG length and sampling checks, setters for depth, glyphs, cells. NOT CHANGED, by design: names without the UTF-8 flag are read as UTF-8; central directory is authoritative over local headers; trailing bytes after the zlib trailer are ignored; CINFO window not enforced; several `unwrap_or(0)` defences remain (unreachable by construction) |
| T | gap | Test gaps: no CRC-repairing fuzz; no time watchdog; JPEG truncation asserted nothing; REQ-006 partly unmet | FIXED: `cargo xtask fuzz` repairs PNG CRCs on half the mutants and fails any input over 2 s; JPEG truncation now asserts an error; reader name limit tested; remaining REQ-006 fields (depth, glyphs, cells) have setters but no consumer yet, so REQ-006 stays `draft` until the font and layout crates use them |
| 2026-11-21 | F1.8 re-verification (commit 8de324972) | Same reviewer session (resumed) | Re-ran its own reproducers, CRC-repairing fuzz (5 formats x 30k), zlib/ZIP/PNG oracle suites, PIL and ImageMagick images; 22 new source mutants of the added code | Findings 1-8 and 11: all FIXED. No blocking new defects. 71 of 71 mutants killed (49 earlier + 22 new). Reviewer judges REQ-017..020 verifiable | REQ-017, 018, 019, 020 set to `verified`. Notes accepted: (a) decoder is stricter than libpng (refuses tRNS on alpha types, duplicate PLTE/tRNS, PLTE in gray) by design; (b) gray/RGB tRNS key below 16 bits is compared unmasked (libpng masks): only affects malformed encoders, revisit if a real file needs it; (c) drive-letter rule also rejects names like `a:b`, acceptable. Finding 10 (PNG memory ~3x cap) stays a known limit, owner project-source, revisit in F5 |


## Review 2: TrueType parser, subsetter, metrics (2026-11-22, independent agent, read-only)

Method: line-by-line read; 600,000 structure-aware fuzz cases (about 1.0M subset calls) with allocation tracking and watchdog; cross-check against fontTools on 48 system fonts; fontTools validation of 576 subsets.

| # | Severity | Finding | Disposition |
|---|---|---|---|
| 1 | blocking | subset(): loca aliasing gave 339x output amplification (397 KB font to 134 MB; 64 KB glyphs aborted the process) | Fixed: Font::parse requires loca non-decreasing and inside glyf; build_glyf enforces a max_file_bytes budget; both reproductions now rejected at parse; REQ-054 tests |
| 2 | should-fix | closure work stack could hold 256 entries per glyph | Fixed: only unvisited glyphs are pushed |
| 3 | should-fix | cmap format 4 linear scan (125 us per character on a 65 KB font) | Fixed: endCode sortedness validated at parse; binary search |
| 4 | should-fix | check_embedding failed open on a short OS/2 | Fixed: OS/2 under 10 bytes is rejected at parse; missing OS/2 remains allowed (documented) |
| 5 | should-fix | postscript_name allowed PDF delimiters and any length | Fixed: delimiters rejected, 63 characters at most |
| 6 | note | parse accepted head tables subset rejected | Fixed: head must be 54 bytes or more |
| 7 | note | max_depth applied to non-composite glyphs | Fixed: only when following a component |
| 8 | note | truncated composites were copied | Fixed: rejected as Malformed |
| 9 | note | whole-font rejection on cmap oddities | Accepted: strict by design; revisit if real fonts hit it |
| 10 | note | std14 gaps (superscripts, Euro, 0x80-0x9F set) | Fixed: 31 extra WinAnsi entries per font from the AFM files |
| 11 | note | tests weak: fuzz target skipped subset; fontTools test unexecuted | Partly fixed: xtask fuzz ttf now runs subset, embedding check, map_text; new regression tests. Still open: committed fontTools subset check needs a CI hook (manual oracle only) |

Verdict before fixes: not acceptable for untrusted input (finding 1). After fixes: findings 1-8 and 10 closed with regression tests and the PoC fonts rejected; a short re-verification by a reviewer is still advisable before REQ-004 is closed.

## Review 3: DOCX, XLSX and CSV writers (2026-11-22, independent agent, read-only)

Method: read all sources; XSD validation of every part against ISO 29500 transitional schemas; python-docx and openpyxl round-trips; LibreOffice Writer rendering; injection and resource probes.

| # | Severity | Finding | Disposition |
|---|---|---|---|
| D1 | blocking | ordered lists did not restart | Fixed: startOverride per w:num |
| D2 | blocking | a section without header/footer inherited the previous one | Fixed: empty part written when any section has one |
| D3 | should-fix | paragraph and run styles shared an id | Fixed: TSP_ / TSC_ prefixes |
| D4 | should-fix | style names equal ignoring case | Fixed: numeric suffix on collision |
| D5 | should-fix | updateFields prompt in Word | Accepted by design (cached values are stale otherwise); to document for customers |
| D6 | should-fix | part or body ending in a bare table | Fixed: trailing paragraph |
| D7 | should-fix | widowControl switched off by default | Fixed: element only when requested |
| D8 | should-fix | no fonts in docDefaults, header/footer distance 0, no page size cap | Partly fixed: header/footer distance follows the margin; fonts come with tokens (F3.4 follow-up); page cap not yet added |
| D9-D12 | note | CR in text, bookmark rules, empty run role, capability matrix claimed images | Fixed (images now Rejected for DOCX until written) |
| X1 | blocking | defined names Excel repairs (_xlnm., R1C1) | Fixed |
| X2 | should-fix | number format syntax | Fixed: quotes, brackets, sections checked |
| X3 | should-fix | sheet-name rules (apostrophes, History, UTF-16 length, case fold) | Fixed |
| X4 | should-fix | formula whitelist bypass with a space before the parenthesis | Fixed |
| X5 | should-fix | no formula grammar, semicolons, depth | Partly fixed: semicolons rejected, depth 64, edge operators rejected; full grammar not done |
| X6 | should-fix | quadratic formula handling | Fixed: ordered map and set; counts capped by max_cells; 20,000-formula tests |
| X7 | note | unbounded number length | Fixed: 40 characters |
| C1 | should-fix | visible apostrophe in CSV | Documented; option to turn off |
| C2 | should-fix | control characters in CSV | Fixed: rejected |
| C3-C4 | note | empty row, size check after a huge row | Open (notes) |
| T | test quality | well_formed helper weak; tests do not use XSD or independent readers | Open: add XSD and python-docx/openpyxl checks as a gate (needs the schema files; currently found under a skills directory, not committed) |

Verdicts before fixes: DOCX no, XLSX no, CSV yes with caveats. After fixes the blocking items are closed with regression tests (REQ-049, REQ-053).

## Review 4: page layout (crates/layout), commit 37a277dfa

Independent reviewer, attack-based, PoCs outside the repository. Findings and dispositions:

- B1 body page field kept a stale width after the real label was placed: FIXED (placeholder sized to widest digits, real label measured at placement, later items shifted).
- B2 trailing and section-final page breaks left blank pages: FIXED (break is pending; an empty section gives no page).
- B3 oversize table row split was superlinear (80000 words: 22 s): FIXED (shared cells, prefix heights, lazy fragments; 0.19 s release).
- S1 memory and time grew before limits were charged: FIXED (text and line charges; default text cap stays loose, small limits fail fast).
- S2 heading stranded before an unsplittable widow/orphan block: FIXED.
- S3 long keep-with-next chain stranded last heading: FIXED (best effort when the chain is taller than a page, stated in REQ-056).
- S4 short row cut because a repeated header took space: FIXED (repeat suppressed on that page).
- S5 substituted-glyph count depended on location: FIXED (counted per drawn occurrence).
- S6 cell strokes left the content box: FIXED (inset by half the rule width; invariants include stroke).
- N3 repeated header items flagged `repeated`; N4 empty link target Invalid; N5 marker width over all markers; N9 chrome size in header/footer tables and theme bounds: FIXED.
- N1, N2, N6 (Std14 ascent/descent hard-coded), N7, N8: not changed. N8 hostile geometry swept clean.
- Open limits (in REQ text): shared table borders draw twice; anchors in headers and footers are not recorded; wrap breaks per character not per grapheme cluster; default text cap loose.

A short re-verification by a fresh reviewer is advisable before REQ-055/056/058/061 are treated as closed for tagged PDF work.

## Review 5: render_pdf and table of contents (commit 542a61c9a)

Independent reviewer; structure checked by own parser plus gs, pdfinfo, pdftotext, pdffonts; 3300 random TOC documents and a 1500-seed layout fuzz found no invariant violation; output identical across debug/release, time zone and locale. Dispositions:

- F1 wrapped headings gained a space at every break: FIXED (TextItem.space_after; titles exact against IR text).
- F2 link-only heading dropped from TOC/outline: FIXED (keyed by IR block; invisible-text headings skipped everywhere, including the destination).
- F3 no /PageLabels: FIXED (runs from Page.label; absent when no restart).
- F4 TrueType re-parsed per text item: FIXED (glyph lookup cache; debug 14.9 s to 2.4 s writer part on 5000 headings; small gain in release).
- F5 reserved folio-toc- prefix: FIXED (rejected for user anchors).
- F6a bookmark title cut at 4096 bytes; F6b destination and bookmark counts use max_cells; F6c calendar-correct dates shared by IR and writer; F6d page size cap 200 in in IR: FIXED. F6d link naming NOT FIXED (FolioError carries static text); `bad_links()` lists offending links instead.
- N1 pass limit wording and fixed-point driver test; N2 clickable gap and indent clamp; N4 /Lang: FIXED.
- Test quality: TrueType tests in layout and pdf now fail without Liberation unless FOLIO_ALLOW_MISSING_FONT=1; font crate tests still skip (open). Destination x/y and annotation Rects now checked.
- N3 loose default text cap: open, documented.

## Review 6: tagged PDF (commit f0ead1cc4)

Independent reviewer with its own PDF parser and validator; no blocking defect. Structure (MCIDs, ParentTree, OBJR, table/list/TOC shapes, paint order) validated in all scenarios including 130-page and 5000-heading TOCs, 60k paragraphs and 20k-row tables; output identical across runs and debug/release. No third-party PDF/UA validator exists on this VM, so no PDF/UA conformance is claimed. Dispositions:

- 1 structure limits reused input limits: FIXED (element cap 4 x max_cells, depth max_depth + 8).
- 2 DisplayDocTitle without a title: FIXED (written only with a title).
- 3 restore() inside an open marked sequence: FIXED (refused).
- 4 marks without a structure tree: FIXED (rejected at build).
- 5 empty /K: FIXED (omitted).
- 6 arrays over 8191 elements: FIXED (grouping into Div/Part/TBody/Span, split L, number and name trees with /Kids and /Limits, page tree grouping). Large lists become several L elements (open).
- 7 header/footer links: kept as bare Link elements in one trailing Div, now tested.
- 8 heading level skips are input properties; stated in REQ-066.
- 9 BDC tags are standard types; no RoleMap needed; noted.
- Test gaps filled (12 tests); circular count replaced.
- Open: XMP/PDF-UA identification (F5.7); no real validator; pages over 8191 marked-content ids rejected.
