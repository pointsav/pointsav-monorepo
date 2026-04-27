// SPDX-License-Identifier: Apache-2.0
//
// Frontmatter validator. Returns ALL detected errors in one pass rather
// than first-fail, so an editor surfaces every fix the author needs in
// one round-trip.

use crate::frontmatter::Frontmatter;
use crate::genre::GenreTemplate;

/// One validation failure. Keep variants narrow so call-sites can match
/// without wading through formatting noise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// `schema` field is empty.
    EmptySchema,
    /// `schema` carries an unknown discriminator. The argument is the value
    /// that was found.
    UnknownSchema(String),
    /// Genre that requires `language` did not declare one.
    LanguageRequired,
    /// Genre that requires `tenant` did not declare one.
    TenantRequired,
    /// `language` declared but not a recognised BCP 47 short form. Validator
    /// accepts `en`, `es`, `fr`, `en-CA`, `en-US`, `en-GB`, `es-MX`, `es-ES`,
    /// `fr-CA`, `fr-FR` for v0.1.0; the list expands as Foundry's language
    /// surface widens.
    UnknownLanguage(String),
}

const KNOWN_SCHEMAS: &[&str] = &[
    "foundry-doc-v1",
    "foundry-convention-v1",
    "foundry-cluster-manifest-v1",
    "foundry-mailbox-v1",
    "foundry-trajectory-log-v1",
];

const KNOWN_LANGUAGES: &[&str] = &[
    "en", "es", "fr", "en-CA", "en-US", "en-GB", "es-MX", "es-ES", "fr-CA", "fr-FR",
];

/// Validate frontmatter against the per-genre rules.
///
/// `genre = None` validates only the universal rules (schema present and
/// recognised). Pass `Some(...)` to additionally check per-genre required
/// fields.
pub fn validate_frontmatter(
    fm: &Frontmatter,
    genre: Option<GenreTemplate>,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if fm.schema.is_empty() {
        errors.push(ValidationError::EmptySchema);
    } else if !KNOWN_SCHEMAS.iter().any(|s| *s == fm.schema) {
        errors.push(ValidationError::UnknownSchema(fm.schema.clone()));
    }

    if let Some(lang) = fm.language.as_deref() {
        if !KNOWN_LANGUAGES.iter().any(|l| *l == lang) {
            errors.push(ValidationError::UnknownLanguage(lang.to_string()));
        }
    }

    if let Some(g) = genre {
        if requires_language(g) && fm.language.is_none() {
            errors.push(ValidationError::LanguageRequired);
        }
        if requires_tenant(g) && fm.tenant.is_none() {
            errors.push(ValidationError::TenantRequired);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Genres with bilingual siblings (`*.md` + `*.es.md`) require a language
/// declaration so propagation can pair them.
const fn requires_language(g: GenreTemplate) -> bool {
    use GenreTemplate::*;
    matches!(g, ReadmeWorkspace | ReadmeRoot | ReadmeProject | Topic)
}

/// Customer-facing genres require a tenant slug so per-tenant adapter
/// routing is unambiguous.
const fn requires_tenant(g: GenreTemplate) -> bool {
    use GenreTemplate::*;
    matches!(g, Email | Chat | TicketComment | Contract | Cla | Policy | Terms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontmatter::Frontmatter;

    fn ok_minimum() -> Frontmatter {
        Frontmatter::foundry_doc([])
    }

    #[test]
    fn happy_path_minimal_doc() {
        let fm = ok_minimum();
        validate_frontmatter(&fm, None).expect("minimal foundry-doc-v1 passes");
    }

    #[test]
    fn empty_schema_is_rejected() {
        let mut fm = ok_minimum();
        fm.schema.clear();
        let errs = validate_frontmatter(&fm, None).unwrap_err();
        assert!(errs.contains(&ValidationError::EmptySchema));
    }

    #[test]
    fn unknown_schema_is_rejected_with_value() {
        let mut fm = ok_minimum();
        fm.schema = "made-up".into();
        let errs = validate_frontmatter(&fm, None).unwrap_err();
        assert!(errs.contains(&ValidationError::UnknownSchema("made-up".into())));
    }

    #[test]
    fn topic_without_language_is_rejected() {
        let fm = ok_minimum();
        let errs = validate_frontmatter(&fm, Some(GenreTemplate::Topic)).unwrap_err();
        assert!(errs.contains(&ValidationError::LanguageRequired));
    }

    #[test]
    fn topic_with_known_language_passes() {
        let mut fm = ok_minimum();
        fm.language = Some("en".into());
        validate_frontmatter(&fm, Some(GenreTemplate::Topic)).expect("topic + en passes");
    }

    #[test]
    fn unknown_language_is_rejected() {
        let mut fm = ok_minimum();
        fm.language = Some("klingon".into());
        let errs = validate_frontmatter(&fm, None).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::UnknownLanguage(s) if s == "klingon")));
    }

    #[test]
    fn email_without_tenant_is_rejected() {
        let mut fm = ok_minimum();
        // Email does not require language, only tenant.
        fm.language = Some("en".into());
        let errs = validate_frontmatter(&fm, Some(GenreTemplate::Email)).unwrap_err();
        assert!(errs.contains(&ValidationError::TenantRequired));
    }

    #[test]
    fn validator_accumulates_multiple_errors() {
        // Empty schema + unknown language + language-requiring genre all at once.
        let fm = Frontmatter {
            schema: String::new(),
            cites: vec![],
            license: None,
            copyright: None,
            forward_looking: false,
            register: None,
            language: Some("klingon".into()),
            tenant: None,
        };
        let errs = validate_frontmatter(&fm, Some(GenreTemplate::Topic)).unwrap_err();
        assert!(errs.contains(&ValidationError::EmptySchema));
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::UnknownLanguage(_))));
        // Topic requires language. The `language` field IS present (`klingon`)
        // so LanguageRequired must NOT fire — only UnknownLanguage. This locks
        // the rule that "present-but-invalid" and "absent" are distinct
        // validator outcomes.
        assert!(!errs.contains(&ValidationError::LanguageRequired));
    }
}
