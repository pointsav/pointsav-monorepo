// SPDX-License-Identifier: Apache-2.0
//
// The mirror index: every VEVENT read, components keyed by
// RECURRENCE-ID, version order, cancellations as versions, uid_clash,
// sightings, annotations, rebuild from the mirror, and serving the
// result through the CalDAV router.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use service_calendar::mirror_index::{
    compare_versions, event_key, parse_mirror_item, Annotation, AnnotationKind, MirrorIndex,
    MirrorItem, ProblemKind,
};
use service_calendar::mirror_rebuild::{
    build_from_mirror, rebuild_from_mirror, MailboxRoot, NoSidecar, RebuildError,
};
use service_calendar::MirrorView;
use tower::ServiceExt;
use worm_ledger::blob::sha256_hex;

static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn tmp_dir(label: &str) -> PathBuf {
    let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "service-calendar-mirror-{label}-{}-{}",
        std::process::id(),
        n
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Writes `ics` into the mailbox at its content-addressed path and
/// returns the mailbox-relative path.
fn put(root: &Path, ics: &str) -> String {
    let sha = sha256_hex(ics.as_bytes());
    let rel = format!("calendar/{}/{}.ics", &sha[..2], sha);
    let path = root.join(&rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, ics).unwrap();
    rel
}

fn mailbox(name: &str) -> MailboxRoot {
    MailboxRoot {
        mailbox: name.to_string(),
        root: tmp_dir(name),
    }
}

fn cal(method: Option<&str>, vevents: &[String]) -> String {
    let mut s = String::from("BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//EN\r\n");
    if let Some(m) = method {
        s.push_str(&format!("METHOD:{m}\r\n"));
    }
    for v in vevents {
        s.push_str(v);
    }
    s.push_str("END:VCALENDAR\r\n");
    s
}

fn vevent(uid: &str, extra: &[&str]) -> String {
    let mut s = format!("BEGIN:VEVENT\r\nUID:{uid}\r\nDTSTART:20260105T090000Z\r\n");
    for line in extra {
        s.push_str(line);
        s.push_str("\r\n");
    }
    s.push_str("END:VEVENT\r\n");
    s
}

fn item(mailbox: &str, change_seq: Option<u64>, bytes: &str) -> MirrorItem {
    let sha = sha256_hex(bytes.as_bytes());
    MirrorItem {
        mailbox: mailbox.to_string(),
        file: format!("calendar/{}/{}.ics", &sha[..2], sha),
        sha256: sha,
        change_seq,
        folder_flag: None,
    }
}

fn index_of(items: &[(MirrorItem, String)], annotations: Vec<Annotation>) -> MirrorIndex {
    let mut components = Vec::new();
    for (it, ics) in items {
        let (records, problems) = parse_mirror_item(ics.as_bytes(), it);
        assert!(problems.is_empty(), "{problems:?}");
        components.extend(records);
    }
    MirrorIndex::build(components, annotations)
}

const ORG_A: &str = "ORGANIZER:mailto:alice@example.com";

#[test]
fn master_and_override_in_one_file_are_two_components() {
    let ics = cal(
        None,
        &[
            vevent(
                "u1",
                &[
                    ORG_A,
                    "RRULE:FREQ=WEEKLY;COUNT=4",
                    "DTSTAMP:20260101T000000Z",
                ],
            ),
            vevent(
                "u1",
                &[
                    ORG_A,
                    "RECURRENCE-ID:20260112T090000Z",
                    "SEQUENCE:1",
                    "DTSTAMP:20260101T000000Z",
                ],
            ),
        ],
    );
    let index = index_of(&[(item("mb1", None, &ics), ics.clone())], vec![]);
    assert_eq!(index.components.len(), 2);
    let events = index.events();
    assert_eq!(events.len(), 1);
    let e = &events[0];
    assert_eq!(e.event_key, event_key("u1"));
    assert_eq!(e.components.len(), 2);
    assert_eq!(e.components[0].recurrence_id, "");
    assert_eq!(e.components[1].recurrence_id, "20260112T090000Z");
    assert_eq!(e.components[1].versions[0].ordinal, 1);
}

#[test]
fn every_vevent_is_read_not_only_the_first() {
    let ics = cal(
        None,
        &[
            vevent("first", &[]),
            vevent("second", &[]),
            vevent("second", &["RECURRENCE-ID:20260112T090000Z"]),
        ],
    );
    let index = index_of(&[(item("mb1", None, &ics), ics.clone())], vec![]);
    assert_eq!(index.components.len(), 3);
    let keys: Vec<&str> = index.events().iter().map(|e| e.uid).collect();
    assert!(keys.contains(&"first") && keys.contains(&"second"));
}

#[test]
fn versions_order_by_sequence_then_last_modified_then_dtstamp_then_change_seq() {
    let version = |seq: u32, lm: &str, stamp: &str, cs: Option<u64>, mb: &str| {
        let ics = cal(
            None,
            &[vevent(
                "u-order",
                &[
                    &format!("SEQUENCE:{seq}"),
                    &format!("LAST-MODIFIED:{lm}"),
                    &format!("DTSTAMP:{stamp}"),
                    // Make each version's bytes distinct.
                    &format!("SUMMARY:v{seq}-{lm}-{stamp}-{cs:?}-{mb}"),
                ],
            )],
        );
        let (mut r, _) = parse_mirror_item(ics.as_bytes(), &item(mb, cs, &ics));
        r.remove(0)
    };
    let base = version(1, "20260101T000000Z", "20260101T000000Z", Some(5), "mb");
    // SEQUENCE wins over a later LAST-MODIFIED.
    let higher_seq = version(2, "20250101T000000Z", "20250101T000000Z", Some(1), "mb");
    assert!(compare_versions(&higher_seq, &base).is_gt());
    // Same SEQUENCE: LAST-MODIFIED wins over a later DTSTAMP.
    let later_lm = version(1, "20260102T000000Z", "20250101T000000Z", Some(1), "mb");
    assert!(compare_versions(&later_lm, &base).is_gt());
    // Same SEQUENCE and LAST-MODIFIED: DTSTAMP decides.
    let later_stamp = version(1, "20260101T000000Z", "20260102T000000Z", Some(1), "mb");
    assert!(compare_versions(&later_stamp, &base).is_gt());
    // All equal in one mailbox: change_seq decides.
    let later_cs = version(1, "20260101T000000Z", "20260101T000000Z", Some(6), "mb");
    assert!(compare_versions(&later_cs, &base).is_gt());

    let index = MirrorIndex::build(
        vec![
            later_cs.clone(),
            higher_seq.clone(),
            base.clone(),
            later_lm.clone(),
            later_stamp.clone(),
        ],
        vec![],
    );
    let events = index.events();
    let versions = &events[0].components[0].versions;
    let order: Vec<&str> = versions.iter().map(|v| v.sha256.as_str()).collect();
    let expected: Vec<&str> = [&base, &later_cs, &later_stamp, &later_lm, &higher_seq]
        .iter()
        .map(|v| v.sha256.as_str())
        .collect();
    assert_eq!(order, expected);
    assert_eq!(
        events[0].components[0].latest().unwrap().sha256,
        higher_seq.sha256
    );
}

#[test]
fn a_cancellation_is_a_version_not_a_removal() {
    let v0 = cal(
        Some("REQUEST"),
        &[vevent("u-cancel", &[ORG_A, "SEQUENCE:0"])],
    );
    let v1 = cal(
        Some("CANCEL"),
        &[vevent(
            "u-cancel",
            &[ORG_A, "SEQUENCE:1", "STATUS:CANCELLED"],
        )],
    );
    let index = index_of(
        &[
            (item("mb", Some(1), &v0), v0.clone()),
            (item("mb", Some(2), &v1), v1.clone()),
        ],
        vec![],
    );
    assert_eq!(index.components.len(), 2, "both versions stay in the index");
    let events = index.events();
    let comp = &events[0].components[0];
    assert_eq!(comp.versions.len(), 2);
    assert!(!comp.versions[0].is_cancelled());
    assert!(comp.latest().unwrap().is_cancelled());
    assert!(index.to_csv().contains(",CANCELLED,CANCEL,true,"));
    // Not served to clients while the latest version is a cancellation.
    let view = MirrorView::from_index(&index, &[mailbox("mb")]);
    assert!(view.is_empty());
}

#[test]
fn same_uid_with_another_organizer_flags_uid_clash_and_keeps_both() {
    let a = cal(None, &[vevent("u-clash", &[ORG_A, "SEQUENCE:5"])]);
    let b = cal(
        None,
        &[vevent(
            "u-clash",
            &["ORGANIZER:mailto:bob@example.com", "SEQUENCE:0"],
        )],
    );
    let index = index_of(
        &[
            (item("mb", None, &a), a.clone()),
            (item("mb", None, &b), b.clone()),
        ],
        vec![],
    );
    let events = index.events();
    assert_eq!(
        events.len(),
        2,
        "one group per organizer; SEQUENCE is not compared across them"
    );
    assert!(events.iter().all(|e| e.uid_clash));
    assert!(events.iter().all(|e| e.components[0].versions.len() == 1));
    assert_ne!(events[0].resource_name(), events[1].resource_name());
    assert!(index.components.iter().all(|c| c.uid_clash));
}

#[test]
fn one_meeting_in_two_mailboxes_is_one_event_with_two_sightings() {
    let ics = cal(None, &[vevent("u-shared", &[ORG_A, "SEQUENCE:0"])]);
    let index = index_of(
        &[
            (item("alice", Some(1), &ics), ics.clone()),
            (item("bob", Some(9), &ics), ics.clone()),
        ],
        vec![],
    );
    let events = index.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].sightings.len(), 2);
    assert!(!events[0].uid_clash);
}

#[test]
fn delete_annotation_does_not_remove_the_component() {
    let ics = cal(None, &[vevent("u-del", &[ORG_A])]);
    let mut del = Annotation::for_uid(AnnotationKind::Deleted, "u-del", "mb");
    del.note = "client DELETE, \"quoted\"".to_string();
    let left = Annotation::for_uid(AnnotationKind::LeftServer, "u-del", "mb");
    let index = index_of(&[(item("mb", None, &ics), ics.clone())], vec![del, left]);
    assert_eq!(index.components.len(), 1);
    assert_eq!(index.annotations.len(), 2);
    let events = index.events();
    assert_eq!(events[0].annotations.len(), 2);
    assert_eq!(events[0].components.len(), 1);

    let round = MirrorIndex::from_csv(&index.to_csv()).unwrap();
    assert_eq!(
        round, index,
        "CSV round trip keeps every row, annotations included"
    );
}

#[test]
fn tzid_is_kept_and_a_utc_instant_is_added() {
    let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//EN\r\nBEGIN:VEVENT\r\nUID:u-tz\r\nDTSTART;TZID=America/New_York:20260115T120000\r\nDTEND;TZID=Pacific Standard Time:20260115T130000\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
    let (records, problems) = parse_mirror_item(ics.as_bytes(), &item("mb", None, ics));
    assert!(problems.is_empty());
    let start = records[0].dtstart.as_ref().unwrap();
    assert_eq!(start.raw, "20260115T120000");
    assert_eq!(start.tzid.as_deref(), Some("America/New_York"));
    assert_eq!(start.utc.unwrap().to_rfc3339(), "2026-01-15T17:00:00+00:00");
    // A zone name chrono-tz does not know is kept, not guessed.
    let end = records[0].dtend.as_ref().unwrap();
    assert_eq!(end.tzid.as_deref(), Some("Pacific Standard Time"));
    assert!(end.utc.is_none());
    let csv = MirrorIndex::build(records, vec![]).to_csv();
    assert!(csv.contains(",20260115T120000,America/New_York,2026-01-15T17:00:00Z,"));
}

#[test]
fn rebuild_is_idempotent_and_reports_a_misnamed_file() {
    let mb = mailbox("rebuild");
    put(
        &mb.root,
        &cal(
            None,
            &[
                vevent("u-r1", &[ORG_A]),
                vevent("u-r1", &["RECURRENCE-ID:20260112T090000Z"]),
            ],
        ),
    );
    put(&mb.root, &cal(None, &[vevent("u-r2", &[])]));
    // Right layout, wrong name: the bytes do not hash to the name.
    let wrong = "0".repeat(64);
    let wrong_rel = format!("calendar/00/{wrong}.ics");
    std::fs::create_dir_all(mb.root.join("calendar/00")).unwrap();
    std::fs::write(mb.root.join(&wrong_rel), cal(None, &[vevent("u-bad", &[])])).unwrap();

    let out = tmp_dir("rebuild-out").join("calendar-index.csv");
    let mailboxes = [mb.clone()];
    let first = rebuild_from_mirror(&mailboxes, &[], &NoSidecar, &out).unwrap();
    let bytes1 = std::fs::read(&out).unwrap();
    let second = rebuild_from_mirror(&mailboxes, &[], &NoSidecar, &out).unwrap();
    let bytes2 = std::fs::read(&out).unwrap();
    assert_eq!(bytes1, bytes2, "rebuilding twice gives identical CSV bytes");

    assert_eq!(first.components, 3);
    assert_eq!(first.files_indexed, 2);
    assert_eq!(second.components, 3);
    assert_eq!(first.problems.len(), 1);
    assert_eq!(first.problems[0].file, wrong_rel);
    assert!(matches!(
        first.problems[0].kind,
        ProblemKind::NameMismatch { .. }
    ));
    let text = String::from_utf8(bytes1).unwrap();
    assert!(text.starts_with("schema_version,row_kind,event_key,"));
    assert!(
        !text.contains(&event_key("u-bad")),
        "the misnamed file is skipped"
    );
    // Nothing was written into the mirror.
    let (index, _) = build_from_mirror(&mailboxes, &[], &NoSidecar).unwrap();
    assert_eq!(index.components.len(), 3);
}

#[test]
fn index_inside_the_mirror_is_refused() {
    let mb = mailbox("inside");
    let out = mb.root.join("calendar-index.csv");
    let err = rebuild_from_mirror(std::slice::from_ref(&mb), &[], &NoSidecar, &out).unwrap_err();
    assert!(matches!(err, RebuildError::IndexInsideMirror(_)));
    assert!(!out.exists());
}

fn tmp_vocab() -> worm_ledger::retention::RetentionVocabulary {
    let path = tmp_dir("vocab").join("retention.csv");
    std::fs::write(
        &path,
        "class,duration_days,regime,notes\ncalendar-event,,internal calendar ledger,permanent\n",
    )
    .unwrap();
    worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
}

async fn send(router: &axum::Router, method: &str, uri: &str, body: &str) -> (StatusCode, String) {
    let mut req = Request::builder().method(method).uri(uri);
    if method == "PROPFIND" {
        req = req
            .header("depth", "1")
            .header("content-type", "application/xml");
    }
    let resp = router
        .clone()
        .oneshot(req.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn mirrored_events_are_served_beside_native_ones() {
    let mb = mailbox("serve");
    // One file served as is.
    let whole = cal(
        Some("REQUEST"),
        &[vevent("u-whole", &[ORG_A, "SUMMARY:Whole"])],
    );
    put(&mb.root, &whole);
    // A master in one file and a newer override in another: assembled.
    let master = cal(
        None,
        &[vevent(
            "u-split",
            &[ORG_A, "RRULE:FREQ=DAILY;COUNT=3", "SUMMARY:Master"],
        )],
    );
    let override_ = cal(
        None,
        &[vevent(
            "u-split",
            &[
                ORG_A,
                "RECURRENCE-ID:20260106T090000Z",
                "SEQUENCE:1",
                "SUMMARY:Moved",
            ],
        )],
    );
    put(&mb.root, &master);
    put(&mb.root, &override_);

    let index_path = tmp_dir("serve-out").join("index.csv");
    rebuild_from_mirror(std::slice::from_ref(&mb), &[], &NoSidecar, &index_path).unwrap();
    let view = Arc::new(MirrorView::load(&index_path, std::slice::from_ref(&mb)).unwrap());
    assert_eq!(view.len(), 2);

    let ledger_dir = tmp_dir("serve-ledger");
    let ledger = worm_ledger::InMemoryLedger::open(ledger_dir, "t").unwrap();
    let backing = Arc::new(service_calendar::LedgerBacking::new(
        Box::new(ledger),
        tmp_vocab(),
        "t",
    ));
    let store = Arc::new(service_calendar::EventStore::new());
    let router = service_calendar::router_with_mirror(backing, store, view);

    let native = cal(None, &[vevent("native-1", &["SUMMARY:Native"])]);
    let (status, _) = send(&router, "PUT", "/native-1.ics", &native).await;
    assert!(status.is_success(), "native PUT still works: {status}");

    let (status, xml) = send(
        &router,
        "PROPFIND",
        "/",
        r#"<?xml version="1.0"?><D:propfind xmlns:D="DAV:"><D:prop><D:getcontentlength/></D:prop></D:propfind>"#,
    )
    .await;
    assert_eq!(status, StatusCode::MULTI_STATUS);
    let whole_key = event_key("u-whole");
    let split_key = event_key("u-split");
    assert!(xml.contains("native-1.ics"), "{xml}");
    assert!(xml.contains(&format!("{whole_key}.ics")), "{xml}");
    assert!(xml.contains(&format!("{split_key}.ics")), "{xml}");

    let (status, body) = send(&router, "GET", &format!("/{whole_key}.ics"), "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, whole, "a single-file event is served byte for byte");

    let (status, body) = send(&router, "GET", &format!("/{split_key}.ics"), "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.matches("BEGIN:VEVENT").count(), 2, "{body}");
    assert!(body.contains("SUMMARY:Master") && body.contains("SUMMARY:Moved"));

    let (status, _) = send(&router, "GET", "/native-1.ics", "").await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = send(&router, "DELETE", &format!("/{whole_key}.ics"), "").await;
    assert!(
        !status.is_success(),
        "the mirror is read-only through CalDAV: {status}"
    );
    let (status, _) = send(&router, "GET", &format!("/{whole_key}.ics"), "").await;
    assert_eq!(
        status,
        StatusCode::OK,
        "a refused DELETE leaves the mirrored event served"
    );
}

fn mirror_router(
    mb: &MailboxRoot,
    log: Option<Arc<service_calendar::AnnotationLog>>,
) -> axum::Router {
    let index_path = tmp_dir("del-out").join("index.csv");
    rebuild_from_mirror(std::slice::from_ref(mb), &[], &NoSidecar, &index_path).unwrap();
    let view = Arc::new(MirrorView::load(&index_path, std::slice::from_ref(mb)).unwrap());
    let ledger = worm_ledger::InMemoryLedger::open(tmp_dir("del-ledger"), "t").unwrap();
    let backing = Arc::new(service_calendar::LedgerBacking::new(
        Box::new(ledger),
        tmp_vocab(),
        "t",
    ));
    let store = Arc::new(service_calendar::EventStore::new());
    let mut fs = service_calendar::WormCalendarFs::with_mirror(backing, store, view);
    if let Some(log) = log {
        fs = fs.with_annotation_log(log);
    }
    service_calendar::router_for(fs)
}

#[tokio::test]
async fn a_client_delete_of_a_mirrored_event_is_recorded_and_refused() {
    let mb = mailbox("del");
    put(
        &mb.root,
        &cal(
            Some("REQUEST"),
            &[vevent("u-del", &[ORG_A, "SUMMARY:Keep"])],
        ),
    );
    let log_path = tmp_dir("del-log").join("annotations.jsonl");
    let log = Arc::new(
        service_calendar::AnnotationLog::open(&log_path, std::slice::from_ref(&mb)).unwrap(),
    );
    let router = mirror_router(&mb, Some(log.clone()));
    let key = event_key("u-del");

    let (status, _) = send(&router, "DELETE", &format!("/{key}.ics"), "").await;
    assert!(!status.is_success(), "{status}");

    let read = log.read().unwrap();
    assert_eq!(read.skipped, 0);
    assert_eq!(read.annotations.len(), 1);
    let a = &read.annotations[0];
    assert_eq!(a.event_key, key);
    assert_eq!(a.mailbox, "del");
    assert_eq!(a.kind, AnnotationKind::Deleted);
    assert!(a.at.is_some());

    let (status, _) = send(&router, "GET", &format!("/{key}.ics"), "").await;
    assert_eq!(status, StatusCode::OK, "the event is still served");
}

#[tokio::test]
async fn a_delete_whose_annotation_cannot_be_written_fails_and_keeps_the_event() {
    let mb = mailbox("delfail");
    put(
        &mb.root,
        &cal(Some("REQUEST"), &[vevent("u-f", &[ORG_A, "SUMMARY:Keep"])]),
    );
    // A directory where the log file should be: every append fails.
    let dir = tmp_dir("delfail-log");
    let log_path = dir.join("annotations.jsonl");
    std::fs::create_dir_all(&log_path).unwrap();
    let log = Arc::new(
        service_calendar::AnnotationLog::open(&log_path, std::slice::from_ref(&mb)).unwrap(),
    );
    let router = mirror_router(&mb, Some(log));
    let key = event_key("u-f");

    let (status, _) = send(&router, "DELETE", &format!("/{key}.ics"), "").await;
    assert!(!status.is_success(), "{status}");
    let (status, _) = send(&router, "GET", &format!("/{key}.ics"), "").await;
    assert_eq!(status, StatusCode::OK);
}
