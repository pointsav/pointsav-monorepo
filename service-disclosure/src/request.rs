// SPDX-License-Identifier: Apache-2.0
//
// Request shape consumed by the Doorman (service-slm) and any client
// driving service-proofreader. Carries the family + template selection,
// optional register / audience / target language, and the input content.

use serde::{Deserialize, Serialize};

use crate::genre::{Family, GenreTemplate};

/// Editorial register. Default for Foundry public-facing prose is
/// `Bloomberg` per `~/Foundry/CLAUDE.md` §6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Register {
    /// Bloomberg-grade — financially-literate non-technical reader; precise,
    /// professional, no AI-marketing vocabulary.
    Bloomberg,
    /// Internal-operational — runbooks, GUIDE-* files, terse imperative.
    Operational,
    /// Technical — code-comment register; assumes domain familiarity.
    Technical,
    /// Casual interpersonal — chat, friendly email.
    Casual,
    /// Legal-formal — contracts, CLAs, policies.
    Legal,
}

/// Inbound editorial request.
///
/// `family` is redundant with `template.family()` and is carried separately
/// so a client can declare an intent that does not yet name a specific
/// template (e.g., dispatching a TRANSLATE pass without selecting a genre).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolRequest {
    pub family: Family,
    pub template: GenreTemplate,
    /// Free-form audience descriptor (e.g., "SMB operator", "BCSC reviewer").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<String>,
    /// IETF BCP 47 language tag (e.g., `"en"`, `"es"`, `"en-CA"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register: Option<Register>,
    /// Input text. Always required; an empty string is a client error and
    /// will be rejected by the request handler (not by serde).
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_yaml() {
        let r = ProtocolRequest {
            family: Family::Prose,
            template: GenreTemplate::Topic,
            target_audience: Some("BCSC reviewer".to_string()),
            target_language: Some("en".to_string()),
            register: Some(Register::Bloomberg),
            content: "Sample TOPIC body.".to_string(),
        };
        let y = serde_yaml::to_string(&r).expect("serialise");
        let back: ProtocolRequest = serde_yaml::from_str(&y).expect("deserialise");
        assert_eq!(r, back);
    }

    #[test]
    fn round_trips_through_json_dev_dependency() {
        let r = ProtocolRequest {
            family: Family::Comms,
            template: GenreTemplate::Email,
            target_audience: None,
            target_language: None,
            register: Some(Register::Casual),
            content: "Hi.".to_string(),
        };
        let j = serde_json::to_string(&r).expect("serialise");
        let back: ProtocolRequest = serde_json::from_str(&j).expect("deserialise");
        assert_eq!(r, back);
        // Optional fields with `skip_serializing_if = "Option::is_none"`
        // must not appear in the wire form.
        assert!(!j.contains("target_audience"), "unexpected key: {j}");
        assert!(!j.contains("target_language"), "unexpected key: {j}");
    }

    #[test]
    fn template_family_matches_request_family_when_explicit() {
        let r = ProtocolRequest {
            family: Family::Prose,
            template: GenreTemplate::Memo,
            target_audience: None,
            target_language: None,
            register: None,
            content: "x".into(),
        };
        assert_eq!(r.template.family(), r.family);
    }
}
