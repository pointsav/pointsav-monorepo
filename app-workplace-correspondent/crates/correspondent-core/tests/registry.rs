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

#[test]
fn details_list_contact_facts_but_not_names_companies_or_counters() {
    let r = claims_reg();
    let attrs: Vec<(&str, &str)> = r
        .details_of("P1")
        .iter()
        .map(|c| (c.attribute.as_str(), c.value.as_str()))
        .collect();
    // phone appears once although it was asserted twice (c08, c21); name, company,
    // first-seen and count-* are left out.
    assert_eq!(
        attrs,
        vec![
            ("phone", "555-0100"),
            ("website", "https://example.com/alder")
        ]
    );
    assert!(r.details_of("P2").is_empty());
}

fn merge_reg() -> Registry {
    Registry::open(&fx("claims-merge"), &fx("claims-business")).unwrap()
}

#[test]
fn claims_left_on_merged_records_show_on_the_winner_through_two_merge_steps() {
    let r = merge_reg();
    // A1 -> A2 -> A3. A1's rejected nickname is gone; its second one stands.
    let mut names = r.names_of("A3");
    names.sort();
    assert_eq!(
        names,
        vec![
            "Middle Display Name",
            "Old Display Name",
            "Second Nickname",
            "Winner Alias"
        ]
    );
    let attrs: Vec<&str> = r
        .details_of("A3")
        .iter()
        .map(|c| c.value.as_str())
        .collect();
    assert_eq!(attrs, vec!["555-0101"], "A2's phone shows on A3");
    assert_eq!(r.companies_of("A3"), vec!["Example Holdings Ltd"]);
}

#[test]
fn a_person_does_not_inherit_from_unrelated_retired_or_looping_records() {
    let r = merge_reg();
    assert!(r.names_of("A5").is_empty());
    assert_eq!(r.companies_of("A5"), vec!["Sample Supply Inc"]);
    // A4 is retired with no target: its claim stays on A4 only.
    assert_eq!(r.names_of("A4"), vec!["Retired Alias"]);
    // A6 and A7 merge into each other: no winner, nothing is attributed, no panic.
    assert!(r.names_of("A7").is_empty());
    assert_eq!(r.names_of("A6"), vec!["Loop Alias"], "stays on A6 only");
    assert!(r.resolve_person("A6").is_none());
}

#[test]
fn search_and_business_links_follow_the_merge_chain() {
    let r = merge_reg();
    let hits = r.search_people("old display");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].person_id, "A3");
    assert_eq!(
        r.matched_alias(hits[0], "old display"),
        Some("Old Display Name")
    );
    // A1's company claim links A3 (not A1) to the business.
    let at: Vec<&str> = r
        .people_at_business("B1")
        .iter()
        .map(|p| p.person_id.as_str())
        .collect();
    assert_eq!(at, vec!["A3"]);
}

fn chain_reg() -> Registry {
    Registry::open(&fx("claims-chain"), &fx("claims-business2")).unwrap()
}

#[test]
fn withdrawing_a_withdrawal_brings_the_claim_back() {
    let r = chain_reg();
    // a1 asserted, r1 rejected it, r2 rejected the rejection: the name is live again.
    assert!(r.names_of("X1").contains(&"Revived Name"));
}

#[test]
fn a_retracted_confirmation_leaves_the_claim_unconfirmed() {
    let r = chain_reg();
    let phone = r
        .claims_for_person("X1")
        .into_iter()
        .find(|c| c.attribute == "phone")
        .expect("phone claim stays");
    assert!(!r.is_confirmed(phone));
}

#[test]
fn a_row_about_one_person_cannot_withdraw_another_persons_claim() {
    let r = chain_reg();
    // r3 is filed under X1 but points at X2's claim a3: it is ignored.
    assert_eq!(r.names_of("X2"), vec!["Cross Target"]);
}

#[test]
fn whitespace_around_ids_and_status_does_not_break_matching() {
    let r = chain_reg();
    // r6's supersedes is "a6 " (trailing space): the website is still withdrawn.
    assert!(r.details_of("X1").iter().all(|c| c.attribute != "website"));
    // " merged " counts as merged, so X3 is hidden from search.
    assert!(!r.person("X3").unwrap().is_active());
    // Its old name finds the person it merged into, never the hidden record itself.
    let hits = r.search_people("spaced");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].person_id, "X1");
}

#[test]
fn rows_with_a_blank_claim_id_do_not_count_as_duplicates() {
    let r = chain_reg();
    let attrs: Vec<&str> = r
        .details_of("X1")
        .iter()
        .map(|c| c.attribute.as_str())
        .collect();
    assert!(attrs.contains(&"headline") && attrs.contains(&"postal-address"));
}

#[test]
fn a_short_csv_row_does_not_take_the_registry_down() {
    let r = chain_reg();
    assert_eq!(r.person("X4").unwrap().display_name, "Short Row");
}

#[test]
fn a_broken_or_retired_namesake_does_not_block_a_company_link() {
    let r = chain_reg();
    // B2 is a retired duplicate of B1; B3 merges into an unknown id.
    assert_eq!(r.business_for_company("Dup Co").unwrap().business_id, "B1");
    assert_eq!(
        r.business_for_company("Broken Co").unwrap().business_id,
        "B4"
    );
}
