// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use correspondent_core::{Error, Registry};
use std::path::PathBuf;

fn fx(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn open() -> Registry {
    Registry::open(&fx("service-people"), &fx("service-business")).unwrap()
}

#[test]
fn loads_people_and_businesses() {
    let r = open();
    assert_eq!(r.people().len(), 3);
    assert_eq!(r.businesses().len(), 2);
}

#[test]
fn unknown_columns_are_ignored() {
    // service-people fixture carries `future_column`; loading must still succeed.
    let r = open();
    assert_eq!(r.person("P1").unwrap().display_name, "Alder Example");
}

#[test]
fn empty_optional_fields_become_none() {
    let r = open();
    assert_eq!(r.person("P2").unwrap().status.as_deref(), Some("confirmed"));
    assert_eq!(r.person("P1").unwrap().kind.as_deref(), Some("principal"));
}

#[test]
fn search_is_case_insensitive_and_sorted() {
    let r = open();
    let names: Vec<_> = r
        .search_people("AMPLE")
        .iter()
        .map(|p| p.display_name.as_str())
        .collect();
    assert_eq!(names, vec!["Alder Example", "birch sample"]);
    assert_eq!(r.search_people("BIRCH").len(), 1);
    assert_eq!(r.search_people("").len(), 3);
    assert!(r.search_people("zzz").is_empty());
}

#[test]
fn search_businesses_by_legal_name() {
    let r = open();
    let hits = r.search_businesses("holdings");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].business_id, "B1");
    assert_eq!(hits[0].jurisdiction.as_deref(), Some("CA-BC"));
}

#[test]
fn missing_required_column_fails_loudly() {
    let e = Registry::open(&fx("bad-people"), &fx("service-business")).unwrap_err();
    assert!(
        matches!(
            e,
            Error::MissingColumn {
                column: "display_name",
                ..
            }
        ),
        "{e}"
    );
}

#[test]
fn missing_file_fails_loudly() {
    let e = Registry::open(&fx("nope"), &fx("service-business")).unwrap_err();
    assert!(matches!(e, Error::MissingFile(_)), "{e}");
}

#[test]
fn duplicate_ids_fail_loudly() {
    let e = Registry::open(&fx("dup-people"), &fx("service-business")).unwrap_err();
    assert!(matches!(e, Error::DuplicateId { .. }), "{e}");
}

fn merged() -> Registry {
    Registry::open(&fx("merge-people"), &fx("merge-business")).unwrap()
}

#[test]
fn search_hides_merged_and_retired_rows() {
    let r = merged();
    let ids: Vec<_> = r
        .search_people("")
        .iter()
        .map(|p| p.person_id.as_str())
        .collect();
    assert_eq!(ids, vec!["P3", "P5"]);
    let ids: Vec<_> = r
        .search_businesses("")
        .iter()
        .map(|b| b.business_id.as_str())
        .collect();
    assert_eq!(ids, vec!["B2"]);
}

#[test]
fn old_ids_still_resolve_to_the_current_record() {
    let r = merged();
    // exact lookup still returns the merged row itself
    assert_eq!(r.person("P1").unwrap().status.as_deref(), Some("merged"));
    // resolve follows the chain P1 -> P2 -> P3
    assert_eq!(r.resolve_person("P1").unwrap().person_id, "P3");
    assert_eq!(r.resolve_person("P3").unwrap().person_id, "P3");
    assert_eq!(r.resolve_business("B1").unwrap().business_id, "B2");
}

#[test]
fn retired_row_without_target_resolves_to_itself() {
    let r = merged();
    assert_eq!(r.resolve_person("P4").unwrap().person_id, "P4");
    assert!(!r.person("P4").unwrap().is_active());
}

#[test]
fn merge_loops_and_dangling_targets_return_none() {
    let r = Registry::open(&fx("loop-people"), &fx("service-business")).unwrap();
    assert!(r.resolve_person("P1").is_none());
    assert!(r.resolve_person("P3").is_none());
    assert!(r.resolve_person("nope").is_none());
}

fn open_people(dir: &str) -> Result<Registry, Error> {
    Registry::open(&fx(dir), &fx("service-business"))
}

#[test]
fn schema_matching_header_loads_and_extra_columns_are_tolerated() {
    let r = open_people("schema-ok").unwrap();
    assert_eq!(r.people().len(), 1);
}

#[test]
fn schema_column_order_mismatch_fails_loudly() {
    let e = open_people("schema-order").unwrap_err();
    assert!(matches!(e, Error::SchemaMismatch { .. }), "{e}");
    assert!(e.to_string().contains("schema.csv"));
}

#[test]
fn schema_column_missing_from_header_fails_loudly() {
    let e = open_people("schema-missing").unwrap_err();
    assert!(matches!(e, Error::SchemaMismatch { .. }), "{e}");
    assert!(e.to_string().contains("status"));
}

#[test]
fn schema_required_column_empty_fails_loudly() {
    let e = open_people("schema-empty").unwrap_err();
    assert!(matches!(e, Error::SchemaMismatch { .. }), "{e}");
    assert!(e.to_string().contains("display_name"));
}

#[test]
fn folder_without_schema_is_not_checked() {
    assert!(open_people("service-people").is_ok());
}

fn claims_reg() -> Registry {
    Registry::open(&fx("claims-people"), &fx("claims-business")).unwrap()
}

#[test]
fn folder_without_claims_file_has_no_claims() {
    let r = open();
    assert!(r.claims_for_person("P1").is_empty());
    assert!(r.names_of("P1").is_empty());
}

#[test]
fn name_variants_are_collected_and_withdrawn_ones_dropped() {
    let r = claims_reg();
    // c01 and the bare-id c13; c06 was rejected by c07 so P2 has none.
    assert_eq!(r.names_of("P1"), vec!["Al Example", "A. Example"]);
    assert!(r.names_of("P2").is_empty());
}

#[test]
fn strong_key_subjects_match_no_person() {
    let r = claims_reg();
    assert!(r
        .people()
        .iter()
        .all(|p| !r.names_of(&p.person_id).contains(&"Stranger Named")));
}

#[test]
fn search_finds_a_person_by_name_variant_and_says_which() {
    let r = claims_reg();
    let hits = r.search_people("al exa");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].person_id, "P1");
    // "Alder Example" does not contain "al exa", so the variant is what matched.
    assert_eq!(r.matched_alias(hits[0], "al exa"), Some("Al Example"));
    // A display-name match reports no alias.
    assert_eq!(r.matched_alias(hits[0], "alder"), None);
    // A retired person's variant stays hidden.
    assert!(r.search_people("cedar variant").is_empty());
}

#[test]
fn a_correction_replaces_the_claim_it_supersedes() {
    let r = claims_reg();
    // c10 (Old Name Ltd) is replaced by c11 (Example Holdings Ltd); c04 and c05 stand.
    let mut cos = r.companies_of("P2");
    cos.sort();
    assert_eq!(
        cos,
        vec![
            "Example Holdings Ltd",
            "Nowhere Corp",
            "sample   SUPPLY inc"
        ]
    );
}

#[test]
fn confirm_marks_a_claim_without_removing_it() {
    let r = claims_reg();
    let phone = r
        .claims_for_person("P1")
        .into_iter()
        .find(|c| c.attribute == "phone")
        .unwrap();
    assert!(r.is_confirmed(phone));
    let name = r
        .claims_for_person("P1")
        .into_iter()
        .find(|c| c.attribute == "name")
        .unwrap();
    assert!(!r.is_confirmed(name));
}

#[test]
fn a_retracted_company_claim_is_gone() {
    let r = claims_reg();
    assert_eq!(
        r.companies_of("P1"),
        vec!["Example Holdings Ltd", "Twin Co"]
    );
}

#[test]
fn company_links_to_a_business_only_on_an_exact_normalised_name() {
    let r = claims_reg();
    assert_eq!(
        r.business_for_company("  example   HOLDINGS ltd")
            .unwrap()
            .business_id,
        "B1"
    );
    // "Ltd" versus "Limited" is not guessed.
    assert!(r.business_for_company("Example Holdings Limited").is_none());
    assert!(r.business_for_company("Nowhere Corp").is_none());
    assert!(r.business_for_company("   ").is_none());
}

#[test]
fn an_ambiguous_company_name_links_to_nothing() {
    let r = claims_reg();
    // B3 and B4 are two active businesses with the same name.
    assert!(r.business_for_company("Twin Co").is_none());
}

#[test]
fn a_company_naming_a_merged_business_resolves_to_the_current_one() {
    let r = claims_reg();
    assert_eq!(r.business_for_company("Old Co").unwrap().business_id, "B6");
}

#[test]
fn people_at_a_business_come_from_company_claims_and_hide_inactive_people() {
    let r = claims_reg();
    let at_b1: Vec<&str> = r
        .people_at_business("B1")
        .iter()
        .map(|p| p.person_id.as_str())
        .collect();
    assert_eq!(at_b1, vec!["P1", "P2"]);
    let at_b2: Vec<&str> = r
        .people_at_business("B2")
        .iter()
        .map(|p| p.person_id.as_str())
        .collect();
    assert_eq!(at_b2, vec!["P2"]);
    assert!(r.people_at_business("B6").is_empty());
}

#[test]
fn claims_file_missing_a_required_column_fails_loudly() {
    let e = Registry::open(&fx("bad-claims"), &fx("service-business")).unwrap_err();
    assert!(
        matches!(
            e,
            Error::MissingColumn {
                column: "attribute",
                ..
            }
        ),
        "{e}"
    );
}

#[test]
fn duplicate_claim_ids_fail_loudly() {
    let e = Registry::open(&fx("dup-claims"), &fx("service-business")).unwrap_err();
    assert!(matches!(e, Error::DuplicateId { .. }), "{e}");
}
