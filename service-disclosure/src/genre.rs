// SPDX-License-Identifier: Apache-2.0
//
// 4-family adapter taxonomy + genre-template registry.
// Source: `~/Foundry/conventions/language-protocol-substrate.md` §2.

use serde::{Deserialize, Serialize};

/// One of the four adapter families. Composed at request time as the
/// `protocol_adapter` slot in `base ⊕ tenant ⊕ protocol` (claim #22, max 3
/// adapters per request).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Family {
    /// Long-form English prose (READMEs, TOPICs, GUIDEs, MEMOs, etc.).
    Prose,
    /// Short-form interpersonal (email, chat, ticket comments, meeting notes).
    Comms,
    /// Volume-gated; routes to Tier C (Claude / GPT) via Doorman unless SMB
    /// legal-corpus volume justifies a dedicated adapter.
    Legal,
    /// Meta-protocol — transforms output of any other family across language
    /// pair (en↔es, register-shift). Layered on top, not a separate generation
    /// track.
    Translate,
}

/// Genre templates carried as prompt scaffolding (NOT adapters) per the
/// "fewer adapters, richer scaffolding" production-feasible pattern.
///
/// Variants are grouped by family in the order: PROSE, COMMS, LEGAL.
/// TRANSLATE has no genre variants (it is a meta-protocol layer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GenreTemplate {
    // PROSE family
    /// `README.md` at a workspace root (e.g., `~/Foundry/`).
    ReadmeWorkspace,
    /// `README.md` at a repo root.
    ReadmeRoot,
    /// `README.md` inside a project directory.
    ReadmeProject,
    Topic,
    Guide,
    Memo,
    Architecture,
    Inventory,
    LicenseExplainer,
    Changelog,

    // COMMS family
    Email,
    Chat,
    TicketComment,
    MeetingNotes,

    // LEGAL family
    Contract,
    Cla,
    Policy,
    Terms,
}

impl GenreTemplate {
    /// Resolve the family this genre belongs to.
    pub const fn family(self) -> Family {
        use GenreTemplate::*;
        match self {
            ReadmeWorkspace | ReadmeRoot | ReadmeProject | Topic | Guide
            | Memo | Architecture | Inventory | LicenseExplainer | Changelog => Family::Prose,
            Email | Chat | TicketComment | MeetingNotes => Family::Comms,
            Contract | Cla | Policy | Terms => Family::Legal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_partition_is_total_and_matches_convention_counts() {
        // PROSE has 10 templates (8 base + readme split into 3 sub-variants).
        // COMMS has 4. LEGAL has 4. TRANSLATE has 0.
        let all = [
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
        let prose = all.iter().filter(|g| g.family() == Family::Prose).count();
        let comms = all.iter().filter(|g| g.family() == Family::Comms).count();
        let legal = all.iter().filter(|g| g.family() == Family::Legal).count();
        let translate = all.iter().filter(|g| g.family() == Family::Translate).count();
        assert_eq!(prose, 10);
        assert_eq!(comms, 4);
        assert_eq!(legal, 4);
        assert_eq!(translate, 0);
        assert_eq!(prose + comms + legal + translate, all.len());
    }

    #[test]
    fn family_round_trip_yaml() {
        for f in [Family::Prose, Family::Comms, Family::Legal, Family::Translate] {
            let y = serde_yaml::to_string(&f).expect("serialise");
            let back: Family = serde_yaml::from_str(&y).expect("deserialise");
            assert_eq!(f, back);
        }
    }

    #[test]
    fn genre_serialises_kebab_case() {
        let y = serde_yaml::to_string(&GenreTemplate::ReadmeProject).unwrap();
        assert!(y.trim() == "readme-project", "unexpected serialisation: {y}");
        let y = serde_yaml::to_string(&GenreTemplate::TicketComment).unwrap();
        assert!(y.trim() == "ticket-comment", "unexpected serialisation: {y}");
    }
}
