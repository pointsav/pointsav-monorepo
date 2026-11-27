// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! A fixed corpus of hostile inputs, std only (no fuzzing crate): every decoder entry point in
//! `fuzz` gets every input, and must neither panic nor run away. The corpus is generated from a
//! deterministic LCG, so a failure reproduces exactly. Made-up data only.
use std::time::{Duration, Instant};
use tool_precedent_core::error::CoreError;
use tool_precedent_core::fuzz;
use tool_precedent_core::registry::MAX_HTML_DEPTH;

/// Per call, generous for a debug build on a heavily loaded machine (the slowest input, a
/// 2,000-version draw loop, takes about 1 to 5 s there). Before the fixes the same inputs took from
/// 20 s to several minutes (the runaway HTML and lineage paths).
const BOUND: Duration = Duration::from_secs(15);

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Tokens the decoders care about, mixed into the random inputs.
const PIECES: &[&str] = &[
    "\"",
    "\"\"",
    ",",
    "\n",
    "\r\n",
    "<",
    ">",
    "</",
    "/>",
    "<p>",
    "</p>",
    "<div class=\"x\">",
    "<span class=\"clm\">",
    "</span>",
    "<style>",
    "</style>",
    "<script>",
    "<!--",
    "-->",
    "<!",
    "&",
    "&amp;",
    "&#x",
    ";",
    "_",
    "_JW",
    "JW1",
    "MW2",
    "_FIN",
    ".html",
    ".",
    "\t",
    "é",
    "☐",
    "𝄞",
    "/* === PER-DOCUMENT OVERRIDES (preserved by apply: edit here, not above) === */",
    "/* === END PER-DOCUMENT OVERRIDES === */",
    "id,selector,component,status\n",
    "a",
    "0",
    "9pt",
];

fn corpus() -> Vec<(String, Vec<u8>)> {
    let mut out: Vec<(String, Vec<u8>)> = Vec::new();
    let mut r = Lcg(0x005e_ed0f_f00d);
    // 1. random bytes, any value
    for k in 0..60 {
        let len = r.below(4096);
        let v: Vec<u8> = (0..len).map(|_| r.next() as u8).collect();
        out.push((format!("bytes-{k}"), v));
    }
    // 2. random sequences of meaningful tokens
    for k in 0..80 {
        let n = r.below(600);
        let mut s = String::new();
        for _ in 0..n {
            s.push_str(PIECES[r.below(PIECES.len())]);
        }
        out.push((format!("tokens-{k}"), s.into_bytes()));
    }
    // 3. truncated UTF-8: a multi-byte text cut at every byte
    let multi = "a,é\n\"☐\",𝄞_JW1.html<p class=\"é\">";
    for cut in 0..=multi.len() {
        out.push((format!("cut-{cut}"), multi.as_bytes()[..cut].to_vec()));
    }
    // 4. lone and unbalanced quotes
    for (k, s) in [
        "\"",
        "\"\"\"",
        "a,\"b",
        "a,b\n\"c\nd\ne",
        "\"\n",
        "x\"y\"z,\"",
        "h\n\"",
    ]
    .iter()
    .enumerate()
    {
        out.push((format!("quote-{k}"), s.as_bytes().to_vec()));
    }
    // 5. huge repeated tokens
    for (name, tok, times) in [
        ("lt-digit", "<1", 50_000),
        ("amp", "&", 100_000),
        ("style", "<style></style>", 10_000),
        ("script", "<script>", 10_000),
        ("comment", "<!--", 20_000),
        ("commas", ",", 200_000),
        ("quotes", "\"", 100_001),
        ("newlines", "\n", 200_000),
        ("underscores", "_JW1", 25_000),
        ("lt", "<", 100_000),
        ("close", "</b>", 30_000),
        ("attr", "<p class=\"", 10_000),
        ("markers", "/* === END PER-DOCUMENT OVERRIDES === */", 2_000),
    ] {
        out.push((format!("huge-{name}"), tok.repeat(times).into_bytes()));
    }
    // 6. deeply nested tags, up to 10,000 levels
    for d in [
        10usize,
        100,
        1_000,
        MAX_HTML_DEPTH,
        MAX_HTML_DEPTH + 1,
        10_000,
    ] {
        out.push((
            format!("deep-div-{d}"),
            format!("{}x{}", "<div>".repeat(d), "</div>".repeat(d)).into_bytes(),
        ));
        out.push((
            format!("deep-label-{d}"),
            format!("<body>{}", "<span class=\"clm\">xxxxxxxxxx".repeat(d)).into_bytes(),
        ));
        out.push((
            format!("deep-open-unmatched-close-{d}"),
            format!("{}{}", "<div>".repeat(d), "</b>".repeat(d)).into_bytes(),
        ));
    }
    // 7. very long lines
    out.push(("long-a".into(), "a".repeat(1_000_000).into_bytes()));
    out.push((
        "long-field".into(),
        format!("h\n\"{}\"", "x,".repeat(300_000)).into_bytes(),
    ));
    out.push((
        "long-name".into(),
        format!("{}_JW1.html", "B".repeat(500_000)).into_bytes(),
    ));
    // 8. lineage shapes: long chains, gaps, draw loops, long chained names
    let chain: String = (1..=2_000).map(|n| format!("D_JW{n}.html\n")).collect();
    out.push(("lineage-chain".into(), chain.clone().into_bytes()));
    let gaps: String = (1..=2_000)
        .map(|n| format!("D_JW{}.html\n", n * 1000))
        .collect();
    out.push(("lineage-gaps".into(), gaps.into_bytes()));
    let loops: String = (1..=2_000)
        .map(|n| format!("D\tJW{n}\tJW{}\n", n % 2_000 + 1))
        .collect();
    out.push((
        "lineage-loop".into(),
        format!("{chain}{loops}").into_bytes(),
    ));
    let chained: String = (1..=500)
        .map(|n| format!("D{}_MW{n}.html\n", "_JW1".repeat(n % 50 + 1)))
        .collect();
    out.push(("lineage-chained".into(), chained.into_bytes()));
    out.push((
        "lineage-big-number".into(),
        b"D_JW4294967295.html\nD_JW4294967296.html\nD_JW99999999999999999999.html\nD_JW0.html\n"
            .to_vec(),
    ));
    // 9. registry tables with real column names and hostile cells
    for k in 0..20 {
        let mut s = String::from(
            "id,class,selector,component,family,parent,status,source,evidence,font_size,line_height,margin_top,font_weight,body_size,page_margin,token_stem,token_css,key,value\n",
        );
        for _ in 0..r.below(40) {
            let cells: Vec<String> = (0..19)
                .map(|_| {
                    let mut c = String::new();
                    for _ in 0..r.below(4) {
                        c.push_str(
                            [
                                "a",
                                "9pt",
                                "-1.5pt",
                                "inf",
                                "NaN",
                                "1e308",
                                "nan",
                                "x:0",
                                ":",
                                ";",
                                "canada",
                                "all",
                                "css",
                                "word",
                                "bold",
                                "#fff",
                                "..",
                                "é",
                                "\"\"",
                                "-",
                                "0",
                                "18446744073709551616",
                            ][r.below(22)],
                        );
                    }
                    if c.contains([',', '"']) {
                        format!("\"{}\"", c.replace('"', "\"\""))
                    } else {
                        c
                    }
                })
                .collect();
            s.push_str(&cells.join(","));
            s.push('\n');
        }
        out.push((format!("table-{k}"), s.into_bytes()));
    }
    out
}

type Entry = (&'static str, fn(&[u8]) -> bool);

/// Each entry point, reduced to "returned" (the value itself does not matter here).
const ENTRIES: &[Entry] = &[
    ("csv", |b| fuzz::csv(b).is_ok()),
    ("registry_table", |b| fuzz::registry_table(b).is_ok()),
    ("html", |b| fuzz::html(b).is_ok()),
    ("fence_overrides", |b| fuzz::fence_overrides(b).is_ok()),
    ("lineage_name", |b| fuzz::lineage_name(b).is_ok()),
    ("lineage_build", |b| fuzz::lineage_build(b).is_ok()),
];

#[test]
fn every_decoder_survives_the_hostile_corpus_within_the_time_bound() {
    let corpus = corpus();
    assert!(corpus.len() >= 200, "corpus has {} inputs", corpus.len());
    let mut slow: Vec<String> = Vec::new();
    for (name, input) in &corpus {
        for (entry, f) in ENTRIES {
            let t = Instant::now();
            let res = std::panic::catch_unwind(|| f(input));
            let took = t.elapsed();
            assert!(res.is_ok(), "{entry} panicked on {name}");
            if took > BOUND {
                slow.push(format!("{entry} on {name}: {took:?}"));
            }
        }
    }
    assert!(slow.is_empty(), "past the time bound: {slow:#?}");
}

#[test]
fn nesting_past_the_depth_limit_is_a_typed_error_and_at_the_limit_is_read() {
    let at = format!("<body>{}", "<div>".repeat(MAX_HTML_DEPTH));
    assert_eq!(
        fuzz::html(at.as_bytes()).expect("at the limit"),
        MAX_HTML_DEPTH
    );
    let past = format!("<body>{}", "<div>".repeat(MAX_HTML_DEPTH + 1));
    match fuzz::html(past.as_bytes()) {
        Err(CoreError::TooDeep { limit, line }) => {
            assert_eq!((limit, line), (MAX_HTML_DEPTH, 1));
        }
        other => panic!("expected TooDeep, got {other:?}"),
    }
    // closed elements do not accumulate: 10,000 siblings are fine
    let flat = format!("<body>{}", "<div>x</div>".repeat(10_000));
    assert_eq!(fuzz::html(flat.as_bytes()).expect("flat"), 10_000);
}

#[test]
fn an_entry_point_refuses_input_past_its_cap_and_accepts_it_at_the_cap() {
    assert_eq!(fuzz::MAX_INPUT_BYTES, 64 * 1024 * 1024);
    let at = vec![b'a'; 1000];
    assert_eq!(
        fuzz::text_capped(&at, 1000).expect("at the cap").len(),
        1000
    );
    let past = vec![b'a'; 1001];
    assert!(matches!(
        fuzz::text_capped(&past, 1000),
        Err(CoreError::TooLarge(_))
    ));
    assert!(matches!(
        fuzz::text(&[0xff, 0xfe]),
        Err(CoreError::Other(m)) if m.contains("UTF-8")
    ));
}

#[test]
fn well_formed_inputs_decode_as_before() {
    assert_eq!(fuzz::csv(b"a,b\n\"c,\"\"d\"\"\",e\n").expect("csv"), 2);
    assert!(fuzz::csv(b"a,\"b\n").is_err());
    assert_eq!(
        fuzz::lineage_name(b"Made_Up_JW3_MW1.html").expect("name"),
        Some("JW3_MW1".to_string())
    );
    assert_eq!(fuzz::lineage_name(b"plain.html").expect("name"), None);
    assert_eq!(
        fuzz::lineage_build(b"D_JW1.html\nD_JW2.html\nE_FIN.pdf\n").expect("build"),
        2
    );
    assert_eq!(
        fuzz::registry_table(b"id,status\na,current\n\nb,current\n").expect("table"),
        2
    );
    assert!(fuzz::registry_table(b"id,status\na\n").is_err());
    assert_eq!(fuzz::fence_overrides(b"p{}").expect("none"), "");
    assert!(fuzz::fence_overrides(b"/* === END PER-DOCUMENT OVERRIDES === */").is_err());
    assert_eq!(
        fuzz::html(b"<html><body><p class=\"x\">a</p><br></body></html>").expect("html"),
        2
    );
}
