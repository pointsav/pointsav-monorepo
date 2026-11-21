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
