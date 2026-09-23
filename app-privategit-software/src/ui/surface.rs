// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Per-surface identity for the software.pointsav.com chrome.
//!
//! Mirrors the wiki/marketing `Tenant` enum pattern (app-mediakit-knowledge-2
//! `src/ui/tenant.rs`) but the dimension of variation here is the *binary
//! surface*, not the brand — software.pointsav.com is always PointSav-brand.
//!
//! Only `Marketplace` is constructed. `app-privategit-source-2` serves no HTML
//! (verified in the token-reconciliation research §d: zero `text/html` responses
//! in that crate — it is a pure machine surface), so a `Source` variant would be
//! dead code and is intentionally omitted rather than stubbed with a dead arm.

use crate::ui::Lang;

/// The served surfaces of software.pointsav.com.
///
/// `Source` is intentionally absent — see the module docs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SoftwareSurface {
    Marketplace,
}

impl SoftwareSurface {
    /// Accessible label / wordmark text for the masthead.
    ///
    /// **Redesigned 2026-07-07 (second pass, same day)** — was "PointSav Software"
    /// rendered as an icon + two-line stacked lockup, a structural pattern
    /// verified byte-for-byte identical to `documentation.pointsav.com`'s own
    /// masthead (same SVG glyph path, same "brand + small-caps descriptor below
    /// it" shape). Replaced with a single flat-text wordmark — no icon, matching
    /// `home.pointsav.com`'s real masthead pattern (`<a class="m-masthead__
    /// wordmark">PointSav Digital Systems</a>`, checked directly against its
    /// served HTML) — and folds in the "Binary Library" identity that used to be
    /// a separate, now-removed nav link (see `layout::masthead`'s doc comment):
    /// the wordmark already links to `/`, which redirects to `/software`
    /// (`main.rs`'s `root()`), so a second link to the same place added nothing.
    pub fn home_label(self) -> &'static str {
        match self {
            SoftwareSurface::Marketplace => "PointSav Binary Library",
        }
    }

    /// Verbatim trademark line.
    ///
    /// **Corrected 2026-07-07** — the 2026-07-04 "correction" documented in this
    /// function's prior version was itself wrong: it claimed `git log --
    /// TRADEMARK.md` showed the six-mark set it was replacing "has never contained
    /// any of those six strings." That check was mistaken. `TRADEMARK.md` was
    /// rewritten to the "Woodfine Capital Projects™, Woodfine Management Corp™,
    /// PointSav Digital Systems™, Totebox Orchestration™, Totebox Archive™" canonical
    /// notice format on **2026-05-16** (commit `925eaee`), and gained
    /// **Capability Geometry™** as a sixth mark on **2026-06-19** (commit `86f8b65`)
    /// — both well before the 2026-07-04 session ran its check, and unchanged since.
    /// Verified directly against the current file (`vendor/factory-release-
    /// engineering/TRADEMARK.md` §13, the canonical-notice section) — the prior
    /// seven-mark set here (`PointSav™, Foundry™, ToteboxOS™, ConsoleOS™,
    /// OrchestrationOS™, WorkplaceOS™, WoodfineGroup™`) has **never** appeared in
    /// `TRADEMARK.md` at any point since the April 2026 original; it predates even
    /// the May rewrite. The exact wording below (using the informal `MCorp™`
    /// abbreviation for Woodfine Management Corp, and folding Capability Geometry's
    /// unregistered-mark note into the single sentence rather than a separate one)
    /// matches `home.pointsav.com`'s real, live, served trademark line verbatim —
    /// checked directly against its HTML, not assumed. Do not paraphrase or shorten
    /// again without reading `TRADEMARK.md` §13 directly AND cross-checking a live
    /// family site — a claimed citation is not a substitute for checking either.
    ///
    /// **Translated for ES 2026-07-13** (full-site-parity pass) — home.pointsav.com's
    /// own served HTML was checked directly and its trademark/copyright line IS fully
    /// translated on its `/es` page (not left English), superseding this session's
    /// earlier, more cautious assumption that shared legal-adjacent copy should wait
    /// for factory-release-engineering's canonical Spanish DISCLAIMER text. That
    /// pending translation is the LP-offering disclaimer document specifically; the
    /// short trademark-notice sentence is not sourced from it, and home.pointsav.com's
    /// own practice is the concrete precedent to match here. Mark names themselves
    /// (™ terms) stay untranslated — they are proper nouns, same as home.pointsav.com's.
    ///
    /// **Cross-checked against `legal-tokens-pointsav.yaml` (2026-08-02), wired live
    /// 2026-08-25:** this text is verified byte-identical to `factory-release-
    /// engineering`'s `tokens/legal-tokens-pointsav.yaml` `statement`/`statement_es`
    /// fields (fixed in that repo's commit `f169e4e`, 2026-07-27) — a third independent
    /// source (alongside `TRADEMARK.md` and home.pointsav.com's live HTML) agreeing.
    ///
    /// Now reads that YAML live at `LEGAL_TOKENS_PATH` (default
    /// `/var/lib/local-software/legal-tokens-pointsav.yaml`) via
    /// [`load_trademark_statement`], falling back to the literal below — unchanged
    /// from the prior hardcoded value — whenever the file is absent, unreadable, or
    /// unparseable. This resolves the earlier blocker without a hard deploy
    /// dependency: `local-software-marketplace` doesn't deploy the YAML alongside the
    /// binary today, so this falls back to the exact current behavior in production
    /// until Command adds that deploy step — no new failure mode, no unreviewed
    /// deploy-order dependency introduced.
    pub fn trademark_line(self, lang: Lang) -> &'static str {
        let fallback = match lang {
            Lang::En => {
                "Woodfine Capital Projects\u{2122}, MCorp\u{2122}, PointSav Digital Systems\u{2122}, \
                 Totebox Orchestration\u{2122}, Totebox Archive\u{2122}, and Capability Geometry\u{2122} \
                 are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, \
                 Latin America, and Europe. All other trademarks are the property of their respective \
                 owners."
            }
            Lang::Es => {
                "Woodfine Capital Projects\u{2122}, MCorp\u{2122}, PointSav Digital Systems\u{2122}, \
                 Totebox Orchestration\u{2122}, Totebox Archive\u{2122} y Capability Geometry\u{2122} \
                 son marcas comerciales de Woodfine Capital Projects Inc., utilizadas en Canad\u{e1}, \
                 los Estados Unidos, Am\u{e9}rica Latina y Europa. Todas las dem\u{e1}s marcas \
                 comerciales son propiedad de sus respectivos titulares."
            }
        };
        load_trademark_statement(lang, fallback)
    }

    /// Copyright holder — the parent company, for every surface.
    pub fn copyright_holder(self) -> &'static str {
        "Woodfine Capital Projects Inc."
    }

    /// Office cities for the footer line (BRIEF footer anatomy; Berlin dropped
    /// 2026-07-07 per operator instruction).
    pub fn cities(self) -> &'static [&'static str] {
        &["Vancouver", "New York"]
    }

    /// Label for the single "Important information" disclosure slot in the
    /// footer accordion (see `layout::footer`). Matches the
    /// `app-mediakit-marketing-2` `DisclosureSlot` pattern (operator-directed
    /// 2026-07-02, "use the current footer setup like on the wiki/home
    /// sites") — a native `<details>` accordion, collapsed by default,
    /// on-page rather than hidden behind a link ("clear and prominent"),
    /// containing the one disclosure specific to what this site actually
    /// does: sell software licenses paid for in on-chain USDC.
    ///
    /// Supersedes the Checkpoint-3a `disclaimer_citation()` fix (2026-07-02)
    /// — that fix restored a citation line pointing at
    /// `factory-release-engineering/policies/DISCLAIMER.md`, an LP
    /// investment-offering document not applicable to a software
    /// marketplace. This site now has its own self-contained disclaimer
    /// page (`/page/disclaimer`, see `ui::disclaimer`) instead of citing
    /// someone else's.
    pub fn disclosure_label(self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Payment and licensing disclosure",
            Lang::Es => "Divulgaci\u{f3}n de pagos y licencias",
        }
    }
}

/// `trademarks.statement`/`statement_es` from `legal-tokens-pointsav.yaml`'s real
/// shape — only the fields this crate actually consumes, not the whole schema.
#[derive(serde::Deserialize)]
struct LegalTokensFile {
    trademarks: LegalTokensTrademarks,
}

#[derive(serde::Deserialize)]
struct LegalTokensTrademarks {
    statement: String,
    statement_es: String,
}

/// Pure parse: YAML text -> the one statement this call wants, or `None` if the
/// file doesn't parse into the expected shape or the field is blank. No I/O, so
/// this is the part covered by unit tests below — the env-var/filesystem/caching
/// logic in [`load_trademark_statement`] is exercised in production, not tests,
/// to avoid process-global `OnceLock` state leaking between test runs.
fn parse_legal_tokens_statement(yaml_content: &str, lang: Lang) -> Option<String> {
    let parsed: LegalTokensFile = serde_yaml::from_str(yaml_content).ok()?;
    let s = match lang {
        Lang::En => parsed.trademarks.statement,
        Lang::Es => parsed.trademarks.statement_es,
    };
    let s = s.trim().to_string();
    (!s.is_empty()).then_some(s)
}

/// Reads `LEGAL_TOKENS_PATH` (default `/var/lib/local-software/legal-tokens-
/// pointsav.yaml`) once per process and caches the result — this is footer copy
/// rendered on every page, not something worth re-reading from disk per request.
/// Falls back to `fallback` (the crate's own hardcoded, already-correct text)
/// whenever the file is missing, unreadable, or fails to parse — deliberately
/// silent on "file not found" (the expected case today, not an error) but a
/// `tracing::warn!` on "file present but malformed" (a real drift worth knowing
/// about).
fn load_trademark_statement(lang: Lang, fallback: &'static str) -> &'static str {
    use std::sync::OnceLock;
    static EN: OnceLock<&'static str> = OnceLock::new();
    static ES: OnceLock<&'static str> = OnceLock::new();
    let cell = match lang {
        Lang::En => &EN,
        Lang::Es => &ES,
    };
    cell.get_or_init(|| {
        let path = std::env::var("LEGAL_TOKENS_PATH")
            .unwrap_or_else(|_| "/var/lib/local-software/legal-tokens-pointsav.yaml".to_string());
        match std::fs::read_to_string(&path) {
            Ok(raw) => match parse_legal_tokens_statement(&raw, lang) {
                Some(s) => Box::leak(s.into_boxed_str()),
                None => {
                    tracing::warn!(
                        "legal-tokens-pointsav.yaml at {path} present but did not yield a usable \
                         statement for this language — using built-in fallback"
                    );
                    fallback
                }
            },
            Err(_) => fallback,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_label_carries_binary_library_identity_without_wiki_lockup_shape() {
        // Redesigned 2026-07-07 (second pass): a single flat wordmark, not the
        // icon + stacked "PointSav / Software" lockup that matched the wiki's
        // masthead structure. See the doc comment on `home_label`.
        assert_eq!(
            SoftwareSurface::Marketplace.home_label(),
            "PointSav Binary Library"
        );
    }

    const SAMPLE_LEGAL_TOKENS_YAML: &str = "\
schema: foundry-legal-tokens-v1
trademarks:
  owned:
    - \"PointSav Digital Systems\u{2122}\"
  statement: >
    Sample EN trademark statement text.
  statement_es: >
    Texto de muestra en espa\u{f1}ol.
";

    #[test]
    fn parses_real_shaped_yaml_and_extracts_the_right_language() {
        assert_eq!(
            parse_legal_tokens_statement(SAMPLE_LEGAL_TOKENS_YAML, Lang::En).as_deref(),
            Some("Sample EN trademark statement text.")
        );
        assert_eq!(
            parse_legal_tokens_statement(SAMPLE_LEGAL_TOKENS_YAML, Lang::Es).as_deref(),
            Some("Texto de muestra en espa\u{f1}ol.")
        );
    }

    #[test]
    fn malformed_yaml_yields_none_not_a_panic() {
        assert_eq!(
            parse_legal_tokens_statement("not: [valid, yaml: shape", Lang::En),
            None
        );
    }

    #[test]
    fn wrong_shape_yaml_yields_none() {
        // Valid YAML, but missing the `trademarks` key this crate needs.
        assert_eq!(
            parse_legal_tokens_statement("schema: foo\nother: bar\n", Lang::En),
            None
        );
    }

    #[test]
    fn blank_statement_field_yields_none_not_an_empty_footer() {
        let yaml = "trademarks:\n  statement: \"   \"\n  statement_es: \"real text\"\n";
        assert_eq!(parse_legal_tokens_statement(yaml, Lang::En), None);
        assert_eq!(
            parse_legal_tokens_statement(yaml, Lang::Es).as_deref(),
            Some("real text")
        );
    }
}
