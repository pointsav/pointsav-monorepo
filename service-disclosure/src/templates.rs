// SPDX-License-Identifier: Apache-2.0
//
// Genre-template registry. Each `GenreTemplate` variant pairs with one
// `.toml` (machine-readable scaffolding for the Doorman) and one `.md`
// (human-readable description). Both are baked into the binary at
// compile time via `include_str!`.

use crate::genre::GenreTemplate;

/// Return the TOML scaffolding fragment for a genre template. Output is
/// valid TOML; callers parse with the `toml` crate at request time.
pub fn get_template(template: GenreTemplate) -> &'static str {
    use GenreTemplate::*;
    match template {
        ReadmeWorkspace => include_str!("../templates/readme-workspace.toml"),
        ReadmeRoot => include_str!("../templates/readme-root.toml"),
        ReadmeProject => include_str!("../templates/readme-project.toml"),
        Topic => include_str!("../templates/topic.toml"),
        Guide => include_str!("../templates/guide.toml"),
        Memo => include_str!("../templates/memo.toml"),
        Architecture => include_str!("../templates/architecture.toml"),
        Inventory => include_str!("../templates/inventory.toml"),
        LicenseExplainer => include_str!("../templates/license-explainer.toml"),
        Changelog => include_str!("../templates/changelog.toml"),
        Email => include_str!("../templates/email.toml"),
        Chat => include_str!("../templates/chat.toml"),
        TicketComment => include_str!("../templates/ticket-comment.toml"),
        MeetingNotes => include_str!("../templates/meeting-notes.toml"),
        Contract => include_str!("../templates/contract.toml"),
        Cla => include_str!("../templates/cla.toml"),
        Policy => include_str!("../templates/policy.toml"),
        Terms => include_str!("../templates/terms.toml"),
    }
}

/// Return the human-readable description of a genre template. Markdown.
pub fn get_template_description(template: GenreTemplate) -> &'static str {
    use GenreTemplate::*;
    match template {
        ReadmeWorkspace => include_str!("../templates/readme-workspace.md"),
        ReadmeRoot => include_str!("../templates/readme-root.md"),
        ReadmeProject => include_str!("../templates/readme-project.md"),
        Topic => include_str!("../templates/topic.md"),
        Guide => include_str!("../templates/guide.md"),
        Memo => include_str!("../templates/memo.md"),
        Architecture => include_str!("../templates/architecture.md"),
        Inventory => include_str!("../templates/inventory.md"),
        LicenseExplainer => include_str!("../templates/license-explainer.md"),
        Changelog => include_str!("../templates/changelog.md"),
        Email => include_str!("../templates/email.md"),
        Chat => include_str!("../templates/chat.md"),
        TicketComment => include_str!("../templates/ticket-comment.md"),
        MeetingNotes => include_str!("../templates/meeting-notes.md"),
        Contract => include_str!("../templates/contract.md"),
        Cla => include_str!("../templates/cla.md"),
        Policy => include_str!("../templates/policy.md"),
        Terms => include_str!("../templates/terms.md"),
    }
}

/// All genre templates in declaration order. Lets tests iterate without
/// duplicating the variant list and lets callers enumerate the registry.
pub const ALL: &[GenreTemplate] = &[
    GenreTemplate::ReadmeWorkspace,
    GenreTemplate::ReadmeRoot,
    GenreTemplate::ReadmeProject,
    GenreTemplate::Topic,
    GenreTemplate::Guide,
    GenreTemplate::Memo,
    GenreTemplate::Architecture,
    GenreTemplate::Inventory,
    GenreTemplate::LicenseExplainer,
    GenreTemplate::Changelog,
    GenreTemplate::Email,
    GenreTemplate::Chat,
    GenreTemplate::TicketComment,
    GenreTemplate::MeetingNotes,
    GenreTemplate::Contract,
    GenreTemplate::Cla,
    GenreTemplate::Policy,
    GenreTemplate::Terms,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_holds_eighteen_templates() {
        assert_eq!(ALL.len(), 18);
    }

    #[test]
    fn every_template_resolves_to_non_empty_toml() {
        for &g in ALL {
            let toml_src = get_template(g);
            assert!(
                !toml_src.trim().is_empty(),
                "template {g:?} returned empty TOML"
            );
        }
    }

    #[test]
    fn every_template_resolves_to_non_empty_description() {
        for &g in ALL {
            let md = get_template_description(g);
            assert!(
                !md.trim().is_empty(),
                "template {g:?} returned empty description"
            );
        }
    }

    #[test]
    fn every_toml_parses_cleanly() {
        for &g in ALL {
            let src = get_template(g);
            toml::from_str::<toml::Value>(src)
                .unwrap_or_else(|e| panic!("template {g:?} TOML parse failed: {e}\nsource:\n{src}"));
        }
    }

    #[test]
    fn every_template_carries_required_fields() {
        // Every template must declare at minimum: name, family,
        // prompt_scaffolding. These three fields are the contract the
        // Doorman relies on; missing any of them breaks request-time
        // composition.
        for &g in ALL {
            let v: toml::Value = toml::from_str(get_template(g)).unwrap();
            let table = v.as_table().expect("template root must be a TOML table");
            for required in ["name", "family", "prompt_scaffolding"] {
                assert!(
                    table.contains_key(required),
                    "template {g:?} missing required field `{required}`"
                );
                assert!(
                    table[required].as_str().is_some_and(|s| !s.trim().is_empty()),
                    "template {g:?} field `{required}` must be non-empty string"
                );
            }
        }
    }

    #[test]
    fn family_field_matches_genre_partition() {
        use crate::genre::Family;
        for &g in ALL {
            let v: toml::Value = toml::from_str(get_template(g)).unwrap();
            let declared = v["family"].as_str().unwrap();
            let expected = match g.family() {
                Family::Prose => "prose",
                Family::Comms => "comms",
                Family::Legal => "legal",
                Family::Translate => "translate",
            };
            assert_eq!(
                declared, expected,
                "template {g:?} declares family `{declared}` but genre partition says `{expected}`"
            );
        }
    }

    #[test]
    fn template_name_field_matches_kebab_case_serialisation() {
        // Sanity: the `name` in each TOML matches the kebab-case form that
        // GenreTemplate::serde produces. Catches typos in template authoring.
        for &g in ALL {
            let v: toml::Value = toml::from_str(get_template(g)).unwrap();
            let declared = v["name"].as_str().unwrap().to_string();
            let expected = serde_yaml::to_string(&g).unwrap().trim().to_string();
            assert_eq!(
                declared, expected,
                "template name mismatch: declared `{declared}` vs serialised `{expected}`"
            );
        }
    }
}
