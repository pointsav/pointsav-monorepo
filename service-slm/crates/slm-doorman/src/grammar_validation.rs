// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Lark grammar pre-validation at the Doorman boundary (PS.3 step 5).
//!
//! The Doorman relays Lark grammars to Tier B (vLLM ≥0.12) via
//! `extra_body.structured_outputs.grammar`. A malformed grammar would
//! produce an opaque upstream failure — typically a vLLM 400/500 with
//! no useful error message propagated back. Pre-validating at the
//! boundary gives callers a typed `DoormanError::MalformedLarkGrammar`
//! (→ HTTP 400) with the parse-error location included.
//!
//! ## Implementation
//!
//! We use `llguidance`'s own Lark compiler:
//!
//! ```text
//! TopLevelGrammar::from_lark(src)
//!   → ParserFactory::create_parser(grammar)   ← compilation happens here
//! ```
//!
//! `ParserFactory` requires a tokenizer environment. We use
//! `ApproximateTokEnv::single_byte_env()` (every byte = one token) to
//! satisfy the API without a real LLM tokenizer. This is the pattern
//! shown in `llguidance/sample_parser/src/minimal.rs` for test/validation
//! use-cases. The compiled `TokenParser` is immediately discarded; we
//! keep only the `Ok / Err` signal.
//!
//! ## Performance
//!
//! `ParserFactory` construction is ~O(1) with `new_simple` and is created
//! once at `Doorman` startup, then shared across requests as
//! `Arc<ParserFactory>`. Measured validation latency per call: ~1 ms
//! (release binary, no warm-up needed after first call). LLM inference
//! on any tier is measured in seconds, so this is negligible overhead.

use std::sync::Arc;

use llguidance::{api::TopLevelGrammar, toktrie::ApproximateTokEnv, ParserFactory};

/// A `ParserFactory` initialised with a single-byte tokenizer, suitable
/// for Lark grammar validation without an LLM tokenizer on hand.
///
/// Create once at startup with `LarkValidator::new()` and share via `Arc`.
#[derive(Clone)]
pub struct LarkValidator {
    factory: Arc<ParserFactory>,
}

impl LarkValidator {
    /// Construct a `LarkValidator` backed by an approximate single-byte
    /// tokenizer. This is cheap (~<1 ms) and should be called once at
    /// Doorman startup.
    ///
    /// Returns `Err(String)` if the underlying `ParserFactory` fails to
    /// initialise (unexpected but surfaced rather than panicking).
    pub fn new() -> Result<Self, String> {
        let tok_env = ApproximateTokEnv::single_byte_env();
        let factory = ParserFactory::new_simple(&tok_env)
            .map_err(|e| format!("LarkValidator: ParserFactory init failed: {e}"))?;
        Ok(Self {
            factory: Arc::new(factory),
        })
    }

    /// Validate `lark_src` by compiling it through llguidance's Lark
    /// compiler.
    ///
    /// - `Ok(())` — the Lark source is syntactically valid.
    /// - `Err(msg)` — the Lark source is malformed; `msg` contains the
    ///   parse-error location from llguidance (line, column, and context
    ///   snippet).
    pub fn validate(&self, lark_src: &str) -> Result<(), String> {
        let grammar = TopLevelGrammar::from_lark(lark_src.to_string());
        self.factory
            .create_parser(grammar)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validator() -> LarkValidator {
        LarkValidator::new().expect("LarkValidator construction must succeed in tests")
    }

    /// A well-formed Lark grammar passes validation without error.
    #[test]
    fn valid_lark_grammar_passes_validation() {
        let lark = r#"start: item+
item: WORD
WORD: /[a-z]+/
"#;
        assert!(
            validator().validate(lark).is_ok(),
            "valid Lark grammar must pass validation"
        );
    }

    /// Another common grammar shape: alternation at the start rule.
    #[test]
    fn valid_alternation_grammar_passes_validation() {
        let lark = r#"start: /yes/ | /no/ | /maybe/"#;
        assert!(
            validator().validate(lark).is_ok(),
            "simple alternation Lark grammar must pass validation"
        );
    }

    /// A malformed Lark grammar (unclosed bracket) produces an Err with a
    /// non-empty message describing the syntax error location.
    #[test]
    fn malformed_lark_grammar_returns_typed_error() {
        // Unclosed optional bracket — llguidance reports a parser error.
        let bad_lark = r#"start: item+
item: WORD
WORD: /[a-z]+/
bad_rule: [ unclosed
"#;
        let result = validator().validate(bad_lark);
        assert!(
            result.is_err(),
            "malformed Lark grammar must return Err, got Ok"
        );
        let msg = result.unwrap_err();
        assert!(
            !msg.is_empty(),
            "error message must be non-empty; got empty string"
        );
        // The message should reference the problematic line.
        assert!(
            msg.contains("bad_rule") || msg.contains("unclosed") || msg.contains("Expected"),
            "error message should contain context; got: {msg}"
        );
    }

    /// Completely garbled input produces a clear error rather than panicking.
    #[test]
    fn garbage_input_returns_error_not_panic() {
        let garbage = "!@#$% not lark at all <<<";
        // Must not panic; error content is unspecified.
        let result = validator().validate(garbage);
        assert!(result.is_err(), "garbage input must return Err");
    }
}
