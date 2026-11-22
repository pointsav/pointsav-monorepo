// SPDX-License-Identifier: Apache-2.0

//! Derived, rebuildable index over a calendar mirror.
//!
//! The mirror holds one write-once `.ics` file per item at the
//! mailbox-relative path `calendar/<first 2 hex>/<64 hex>.ics`, where
//! the file name is the lowercase SHA-256 of the stored bytes. This
//! module turns those bytes into index rows that point back at the
//! file (relative path plus SHA-256); it never copies the bytes.
//!
//! Identity and ordering:
//! - the event key is the SHA-256 (hex) of the UID;
//! - each instance is a component keyed by RECURRENCE-ID (empty for
//!   the master);
//! - ORGANIZER is a consistency check only: one UID with two
//!   organizers keeps both groups, flags `uid_clash`, and SEQUENCE is
//!   never compared across the groups;
//! - one meeting seen in several mailboxes is one event with one
//!   sighting per mailbox;
//! - versions of a component are ordered by SEQUENCE, then
//!   LAST-MODIFIED, then DTSTAMP, then `change_seq` (within one
//!   mailbox only);
//! - a cancellation (METHOD:CANCEL or STATUS:CANCELLED) is a version,
//!   never a removal;
//! - DELETE and items leaving the server are [`Annotation`] rows
//!   supplied by the caller; they never remove a component.
//!
//! Every VEVENT in a file is indexed, not only the first.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use worm_ledger::blob::sha256_hex;

/// Value of the leading `schema_version` column. Bump on any change
/// to the column list or to the meaning of a column.
pub const SCHEMA_VERSION: &str = "1";

/// The index columns, in file order.
pub const CSV_COLUMNS: [&str; 31] = [
    "schema_version",
    "row_kind",
    "event_key",
    "uid",
    "recurrence_id",
    "recurrence_id_raw",
    "recurrence_id_tzid",
    "organizer",
    "uid_clash",
    "sequence",
    "last_modified",
    "dtstamp",
    "status",
    "method",
    "cancelled",
    "dtstart",
    "dtstart_tzid",
    "dtstart_utc",
    "dtend",
    "dtend_tzid",
    "dtend_utc",
    "mailbox",
    "change_seq",
    "folder_flag",
    "file",
    "sha256",
    "component_ordinal",
    "annotation_kind",
    "annotation_scope",
    "annotation_at",
    "annotation_note",
];

const UTC_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

/// The event key: lowercase hex SHA-256 of the UID's bytes.
pub fn event_key(uid: &str) -> String {
    sha256_hex(uid.as_bytes())
}

/// A date or date-time property value as written in the `.ics`, plus
/// the UTC instant when it can be resolved. A floating time, a DATE,
/// or a TZID that is not an IANA zone name leaves `utc` empty; no zone
/// is guessed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeValue {
    pub raw: String,
    pub tzid: Option<String>,
    pub utc: Option<DateTime<Utc>>,
}

impl TimeValue {
    pub fn new(raw: &str, tzid: Option<&str>) -> Self {
        let raw = raw.trim().to_string();
        let tzid = tzid
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_string);
        let utc = resolve_utc(&raw, tzid.as_deref());
        Self { raw, tzid, utc }
    }

    fn from_property(property: &icalendar::Property) -> Self {
        let tzid = property.params().get("TZID").map(|p| p.value());
        Self::new(property.value(), tzid)
    }

    /// The component key form of a RECURRENCE-ID: the UTC instant when
    /// resolvable (so the same instance written with a TZID in one
    /// copy and in UTC in another lands on one component), otherwise
    /// the raw value, prefixed by its TZID when it has one.
    pub fn component_key(&self) -> String {
        match (&self.utc, &self.tzid) {
            (Some(utc), _) => utc.format("%Y%m%dT%H%M%SZ").to_string(),
            (None, Some(tzid)) => format!("TZID={tzid}:{}", self.raw),
            (None, None) => self.raw.clone(),
        }
    }
}

fn resolve_utc(raw: &str, tzid: Option<&str>) -> Option<DateTime<Utc>> {
    const LOCAL: &str = "%Y%m%dT%H%M%S";
    if let Some(stripped) = raw.strip_suffix('Z').or_else(|| raw.strip_suffix('z')) {
        let naive = NaiveDateTime::parse_from_str(stripped, LOCAL).ok()?;
        return Some(Utc.from_utc_datetime(&naive));
    }
    let naive = NaiveDateTime::parse_from_str(raw, LOCAL).ok()?;
    let zone = chrono_tz::Tz::from_str(tzid?).ok()?;
    // An ambiguous (DST overlap) or nonexistent (DST gap) local time
    // has no single instant; leave it unresolved rather than pick one.
    zone.from_local_datetime(&naive)
        .single()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Where one mirror file sits and what the caller knows about it
/// beyond its bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorItem {
    pub mailbox: String,
    /// Mailbox-relative path, `calendar/<2 hex>/<64 hex>.ics`.
    pub file: String,
    pub sha256: String,
    /// Change order within this mailbox, when known.
    pub change_seq: Option<u64>,
    /// Source folder marker (for example Deleted Items or Recoverable
    /// Items). Such items are ordinary rows, not deletions.
    pub folder_flag: Option<String>,
}

/// One version of one component (one VEVENT) as found in one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRecord {
    pub event_key: String,
    pub uid: String,
    /// Component key; empty for the master.
    pub recurrence_id: String,
    pub recurrence_id_value: Option<TimeValue>,
    /// Normalised ORGANIZER (`mailto:` removed, lowercase); empty when
    /// absent.
    pub organizer: String,
    pub uid_clash: bool,
    pub sequence: i64,
    pub last_modified: Option<DateTime<Utc>>,
    pub dtstamp: Option<DateTime<Utc>>,
    pub status: String,
    pub method: String,
    pub dtstart: Option<TimeValue>,
    pub dtend: Option<TimeValue>,
    pub mailbox: String,
    pub change_seq: Option<u64>,
    pub folder_flag: Option<String>,
    pub file: String,
    pub sha256: String,
    /// Zero-based position of this VEVENT among the file's VEVENTs.
    pub ordinal: usize,
}

impl ComponentRecord {
    pub fn is_cancelled(&self) -> bool {
        self.status == "CANCELLED" || self.method == "CANCEL"
    }
}

/// Version order within one component: SEQUENCE, LAST-MODIFIED,
/// DTSTAMP, then `change_seq`. Mailbox sits before `change_seq` so
/// that `change_seq` is only ever compared within one mailbox; across
/// mailboxes, equal versions are ordered by mailbox name, then file
/// and position, for a deterministic result only. A missing
/// LAST-MODIFIED, DTSTAMP or `change_seq` sorts before a present one.
pub fn compare_versions(a: &ComponentRecord, b: &ComponentRecord) -> Ordering {
    a.sequence
        .cmp(&b.sequence)
        .then_with(|| a.last_modified.cmp(&b.last_modified))
        .then_with(|| a.dtstamp.cmp(&b.dtstamp))
        .then_with(|| a.mailbox.cmp(&b.mailbox))
        .then_with(|| a.change_seq.cmp(&b.change_seq))
        .then_with(|| a.sha256.cmp(&b.sha256))
        .then_with(|| a.file.cmp(&b.file))
        .then_with(|| a.ordinal.cmp(&b.ordinal))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind {
    /// A client or user deleted the item.
    Deleted,
    /// The item is no longer present on the source server.
    LeftServer,
}

impl AnnotationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AnnotationKind::Deleted => "deleted",
            AnnotationKind::LeftServer => "left_server",
        }
    }

    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            "deleted" => Some(AnnotationKind::Deleted),
            "left_server" => Some(AnnotationKind::LeftServer),
            _ => None,
        }
    }
}

/// A note that something happened to an event or component. Never
/// removes a component from the index.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation {
    pub event_key: String,
    /// `None` targets the whole event; `Some("")` the master.
    pub recurrence_id: Option<String>,
    pub mailbox: String,
    pub kind: AnnotationKind,
    pub at: Option<DateTime<Utc>>,
    pub file: Option<String>,
    pub sha256: Option<String>,
    pub note: String,
}

impl Annotation {
    pub fn for_uid(kind: AnnotationKind, uid: &str, mailbox: &str) -> Self {
        Self {
            event_key: event_key(uid),
            recurrence_id: None,
            mailbox: mailbox.to_string(),
            kind,
            at: None,
            file: None,
            sha256: None,
            note: String::new(),
        }
    }
}

/// Why a file, or one VEVENT in it, was not indexed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProblemKind {
    /// The file name is not the SHA-256 of the file's bytes.
    NameMismatch {
        actual_sha256: String,
    },
    /// An `.ics` under `calendar/` whose path is not
    /// `calendar/<2 hex>/<64 hex>.ics`.
    BadPath,
    /// The `<2 hex>` directory is not the first two characters of the
    /// file name.
    WrongPrefixDir,
    NotUtf8,
    InvalidIcs(String),
    NoVevent,
    MissingUid {
        ordinal: usize,
    },
    Io(String),
}

impl std::fmt::Display for ProblemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemKind::NameMismatch { actual_sha256 } => {
                write!(f, "file name does not match its SHA-256 ({actual_sha256})")
            }
            ProblemKind::BadPath => write!(f, "path is not calendar/<2 hex>/<64 hex>.ics"),
            ProblemKind::WrongPrefixDir => write!(f, "prefix directory does not match file name"),
            ProblemKind::NotUtf8 => write!(f, "file is not UTF-8"),
            ProblemKind::InvalidIcs(e) => write!(f, "invalid ICS: {e}"),
            ProblemKind::NoVevent => write!(f, "no VEVENT component"),
            ProblemKind::MissingUid { ordinal } => write!(f, "VEVENT {ordinal} has no UID"),
            ProblemKind::Io(e) => write!(f, "read error: {e}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub mailbox: String,
    pub file: String,
    pub kind: ProblemKind,
}

fn normalise_organizer(value: &str) -> String {
    let v = value.trim();
    let v = match v.get(..7) {
        Some(prefix) if prefix.eq_ignore_ascii_case("mailto:") => &v[7..],
        _ => v,
    };
    v.trim().to_lowercase()
}

/// Parses every VEVENT in `bytes` into a component record for `item`.
/// A VEVENT without a UID is reported and skipped; the others are
/// still returned.
pub fn parse_mirror_item(bytes: &[u8], item: &MirrorItem) -> (Vec<ComponentRecord>, Vec<Problem>) {
    use icalendar::Component;

    let problem = |kind| Problem {
        mailbox: item.mailbox.clone(),
        file: item.file.clone(),
        kind,
    };
    let Ok(raw) = std::str::from_utf8(bytes) else {
        return (vec![], vec![problem(ProblemKind::NotUtf8)]);
    };
    let calendar: icalendar::Calendar = match raw.parse() {
        Ok(c) => c,
        Err(e) => return (vec![], vec![problem(ProblemKind::InvalidIcs(e))]),
    };
    let method = calendar
        .property_value("METHOD")
        .map(|m| m.trim().to_uppercase())
        .unwrap_or_default();

    let mut records = Vec::new();
    let mut problems = Vec::new();
    let mut seen_any = false;
    for (ordinal, event) in calendar.events().enumerate() {
        seen_any = true;
        let props = event.properties();
        let uid = props.get("UID").map(|p| p.value().trim()).unwrap_or("");
        if uid.is_empty() {
            problems.push(problem(ProblemKind::MissingUid { ordinal }));
            continue;
        }
        let recurrence_id_value = props.get("RECURRENCE-ID").map(TimeValue::from_property);
        let utc_of = |key: &str| {
            props
                .get(key)
                .and_then(|p| resolve_utc(p.value().trim(), None))
        };
        records.push(ComponentRecord {
            event_key: event_key(uid),
            uid: uid.to_string(),
            recurrence_id: recurrence_id_value
                .as_ref()
                .map(TimeValue::component_key)
                .unwrap_or_default(),
            recurrence_id_value,
            organizer: props
                .get("ORGANIZER")
                .map(|p| normalise_organizer(p.value()))
                .unwrap_or_default(),
            uid_clash: false,
            sequence: props
                .get("SEQUENCE")
                .and_then(|p| p.value().trim().parse().ok())
                .unwrap_or(0),
            last_modified: utc_of("LAST-MODIFIED"),
            dtstamp: utc_of("DTSTAMP"),
            status: props
                .get("STATUS")
                .map(|p| p.value().trim().to_uppercase())
                .unwrap_or_default(),
            method: method.clone(),
            dtstart: props.get("DTSTART").map(TimeValue::from_property),
            dtend: props.get("DTEND").map(TimeValue::from_property),
            mailbox: item.mailbox.clone(),
            change_seq: item.change_seq,
            folder_flag: item.folder_flag.clone(),
            file: item.file.clone(),
            sha256: item.sha256.clone(),
            ordinal,
        });
    }
    if !seen_any {
        problems.push(problem(ProblemKind::NoVevent));
    }
    (records, problems)
}

/// The whole derived index: component rows plus annotation rows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MirrorIndex {
    pub components: Vec<ComponentRecord>,
    pub annotations: Vec<Annotation>,
}

/// One version history for one component of one event group.
#[derive(Debug)]
pub struct ComponentView<'a> {
    pub recurrence_id: &'a str,
    /// Oldest first.
    pub versions: Vec<&'a ComponentRecord>,
}

impl<'a> ComponentView<'a> {
    pub fn latest(&self) -> Option<&'a ComponentRecord> {
        self.versions.last().copied()
    }
}

/// One event: one UID under one organizer. Two organizers on one UID
/// give two views, both with `uid_clash` set.
#[derive(Debug)]
pub struct EventView<'a> {
    pub event_key: &'a str,
    pub uid: &'a str,
    pub organizer: &'a str,
    pub uid_clash: bool,
    /// Mailboxes in which any version of any component was seen.
    pub sightings: BTreeSet<&'a str>,
    /// Master (empty recurrence id) first, then by recurrence id.
    pub components: Vec<ComponentView<'a>>,
    pub annotations: Vec<&'a Annotation>,
}

impl EventView<'_> {
    /// Resource name (without `.ics`) under which this event is
    /// served: the event key, plus an organizer suffix on a clash so
    /// both groups stay addressable.
    pub fn resource_name(&self) -> String {
        if self.uid_clash {
            let org = sha256_hex(self.organizer.as_bytes());
            format!("{}-{}", self.event_key, &org[..8])
        } else {
            self.event_key.to_string()
        }
    }

    pub fn master(&self) -> Option<&ComponentView<'_>> {
        self.components.iter().find(|c| c.recurrence_id.is_empty())
    }
}

impl MirrorIndex {
    /// Builds the index: drops exact duplicates (same mailbox, file
    /// and position), flags `uid_clash`, and sorts rows into a stable
    /// order so the same input always yields the same CSV bytes.
    pub fn build(mut components: Vec<ComponentRecord>, mut annotations: Vec<Annotation>) -> Self {
        let mut organizers: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for c in &components {
            organizers
                .entry(c.event_key.clone())
                .or_default()
                .insert(c.organizer.clone());
        }
        for c in &mut components {
            c.uid_clash = organizers.get(&c.event_key).is_some_and(|o| o.len() > 1);
        }
        components.sort_by(|a, b| {
            a.event_key
                .cmp(&b.event_key)
                .then_with(|| a.organizer.cmp(&b.organizer))
                .then_with(|| a.recurrence_id.cmp(&b.recurrence_id))
                .then_with(|| compare_versions(a, b))
        });
        components
            .dedup_by(|a, b| a.mailbox == b.mailbox && a.file == b.file && a.ordinal == b.ordinal);
        annotations.sort();
        annotations.dedup();
        Self {
            components,
            annotations,
        }
    }

    /// Groups the rows into events. Relies on the sort order `build`
    /// established.
    pub fn events(&self) -> Vec<EventView<'_>> {
        let mut events: Vec<EventView<'_>> = Vec::new();
        for c in &self.components {
            let same_event = events
                .last()
                .is_some_and(|e| e.event_key == c.event_key && e.organizer == c.organizer);
            if !same_event {
                events.push(EventView {
                    event_key: &c.event_key,
                    uid: &c.uid,
                    organizer: &c.organizer,
                    uid_clash: c.uid_clash,
                    sightings: BTreeSet::new(),
                    components: Vec::new(),
                    annotations: self
                        .annotations
                        .iter()
                        .filter(|a| a.event_key == c.event_key)
                        .collect(),
                });
            }
            let Some(event) = events.last_mut() else {
                continue;
            };
            event.uid_clash |= c.uid_clash;
            event.sightings.insert(&c.mailbox);
            match event.components.last_mut() {
                Some(view) if view.recurrence_id == c.recurrence_id => view.versions.push(c),
                _ => event.components.push(ComponentView {
                    recurrence_id: &c.recurrence_id,
                    versions: vec![c],
                }),
            }
        }
        events
    }

    pub fn to_csv(&self) -> String {
        let mut out = String::new();
        write_csv_record(&mut out, CSV_COLUMNS.iter().copied());
        for c in &self.components {
            let fields = component_fields(c);
            write_csv_record(&mut out, fields.iter().map(String::as_str));
        }
        for a in &self.annotations {
            let fields = annotation_fields(a);
            write_csv_record(&mut out, fields.iter().map(String::as_str));
        }
        out
    }

    pub fn from_csv(text: &str) -> Result<Self, CsvError> {
        let mut rows = parse_csv(text)?.into_iter();
        let header = rows.next().ok_or(CsvError::MissingHeader)?;
        if header
            .iter()
            .map(String::as_str)
            .ne(CSV_COLUMNS.iter().copied())
        {
            return Err(CsvError::UnexpectedHeader);
        }
        let mut components = Vec::new();
        let mut annotations = Vec::new();
        for (i, row) in rows.enumerate() {
            let line = i + 2;
            if row.len() != CSV_COLUMNS.len() {
                return Err(CsvError::BadRow {
                    line,
                    reason: "wrong column count".into(),
                });
            }
            let r = Row { fields: &row, line };
            if r.get("schema_version") != SCHEMA_VERSION {
                return Err(CsvError::BadRow {
                    line,
                    reason: "unsupported schema_version".into(),
                });
            }
            match r.get("row_kind") {
                "component" => components.push(r.component()?),
                "annotation" => annotations.push(r.annotation()?),
                other => {
                    return Err(CsvError::BadRow {
                        line,
                        reason: format!("unknown row_kind {other:?}"),
                    })
                }
            }
        }
        Ok(Self {
            components,
            annotations,
        })
    }
}

fn fmt_utc(dt: &Option<DateTime<Utc>>) -> String {
    dt.map(|d| d.format(UTC_FORMAT).to_string())
        .unwrap_or_default()
}

fn time_fields(t: &Option<TimeValue>) -> [String; 3] {
    match t {
        Some(t) => [
            t.raw.clone(),
            t.tzid.clone().unwrap_or_default(),
            fmt_utc(&t.utc),
        ],
        None => Default::default(),
    }
}

fn component_fields(c: &ComponentRecord) -> Vec<String> {
    let [rid_raw, rid_tzid, _] = time_fields(&c.recurrence_id_value);
    let [ds, ds_tz, ds_utc] = time_fields(&c.dtstart);
    let [de, de_tz, de_utc] = time_fields(&c.dtend);
    vec![
        SCHEMA_VERSION.to_string(),
        "component".to_string(),
        c.event_key.clone(),
        c.uid.clone(),
        c.recurrence_id.clone(),
        rid_raw,
        rid_tzid,
        c.organizer.clone(),
        c.uid_clash.to_string(),
        c.sequence.to_string(),
        fmt_utc(&c.last_modified),
        fmt_utc(&c.dtstamp),
        c.status.clone(),
        c.method.clone(),
        c.is_cancelled().to_string(),
        ds,
        ds_tz,
        ds_utc,
        de,
        de_tz,
        de_utc,
        c.mailbox.clone(),
        c.change_seq.map(|n| n.to_string()).unwrap_or_default(),
        c.folder_flag.clone().unwrap_or_default(),
        c.file.clone(),
        c.sha256.clone(),
        c.ordinal.to_string(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ]
}

fn annotation_fields(a: &Annotation) -> Vec<String> {
    let mut f = vec![String::new(); CSV_COLUMNS.len()];
    let mut set = |name: &str, value: String| {
        if let Some(i) = CSV_COLUMNS.iter().position(|c| *c == name) {
            f[i] = value;
        }
    };
    set("schema_version", SCHEMA_VERSION.to_string());
    set("row_kind", "annotation".to_string());
    set("event_key", a.event_key.clone());
    set("recurrence_id", a.recurrence_id.clone().unwrap_or_default());
    set("mailbox", a.mailbox.clone());
    set("file", a.file.clone().unwrap_or_default());
    set("sha256", a.sha256.clone().unwrap_or_default());
    set("annotation_kind", a.kind.as_str().to_string());
    let scope = if a.recurrence_id.is_some() {
        "component"
    } else {
        "event"
    };
    set("annotation_scope", scope.to_string());
    set("annotation_at", fmt_utc(&a.at));
    set("annotation_note", a.note.clone());
    f
}

struct Row<'a> {
    fields: &'a [String],
    line: usize,
}

impl Row<'_> {
    fn get(&self, name: &str) -> &str {
        CSV_COLUMNS
            .iter()
            .position(|c| *c == name)
            .and_then(|i| self.fields.get(i))
            .map(String::as_str)
            .unwrap_or("")
    }

    fn bad(&self, column: &str) -> CsvError {
        CsvError::BadRow {
            line: self.line,
            reason: format!("bad value in {column}"),
        }
    }

    fn opt(&self, name: &str) -> Option<String> {
        Some(self.get(name))
            .filter(|v| !v.is_empty())
            .map(str::to_string)
    }

    fn utc(&self, name: &str) -> Result<Option<DateTime<Utc>>, CsvError> {
        match self.get(name) {
            "" => Ok(None),
            v => DateTime::parse_from_rfc3339(v)
                .map(|d| Some(d.with_timezone(&Utc)))
                .map_err(|_| self.bad(name)),
        }
    }

    fn parsed<T: FromStr>(&self, name: &str) -> Result<T, CsvError> {
        self.get(name).parse().map_err(|_| self.bad(name))
    }

    fn time(&self, raw: &str, tzid: &str) -> Option<TimeValue> {
        let raw = self.get(raw);
        if raw.is_empty() {
            return None;
        }
        // Recomputed from raw and TZID so a reader never trusts a stale
        // resolution; the stored UTC column is for other consumers.
        Some(TimeValue::new(raw, Some(self.get(tzid))))
    }

    fn component(&self) -> Result<ComponentRecord, CsvError> {
        let change_seq = match self.get("change_seq") {
            "" => None,
            _ => Some(self.parsed("change_seq")?),
        };
        Ok(ComponentRecord {
            event_key: self.get("event_key").to_string(),
            uid: self.get("uid").to_string(),
            recurrence_id: self.get("recurrence_id").to_string(),
            recurrence_id_value: self.time("recurrence_id_raw", "recurrence_id_tzid"),
            organizer: self.get("organizer").to_string(),
            uid_clash: self.parsed("uid_clash")?,
            sequence: self.parsed("sequence")?,
            last_modified: self.utc("last_modified")?,
            dtstamp: self.utc("dtstamp")?,
            status: self.get("status").to_string(),
            method: self.get("method").to_string(),
            dtstart: self.time("dtstart", "dtstart_tzid"),
            dtend: self.time("dtend", "dtend_tzid"),
            mailbox: self.get("mailbox").to_string(),
            change_seq,
            folder_flag: self.opt("folder_flag"),
            file: self.get("file").to_string(),
            sha256: self.get("sha256").to_string(),
            ordinal: self.parsed("component_ordinal")?,
        })
    }

    fn annotation(&self) -> Result<Annotation, CsvError> {
        let kind = AnnotationKind::parse(self.get("annotation_kind"))
            .ok_or_else(|| self.bad("annotation_kind"))?;
        let recurrence_id = match self.get("annotation_scope") {
            "event" => None,
            "component" => Some(self.get("recurrence_id").to_string()),
            _ => return Err(self.bad("annotation_scope")),
        };
        Ok(Annotation {
            event_key: self.get("event_key").to_string(),
            recurrence_id,
            mailbox: self.get("mailbox").to_string(),
            kind,
            at: self.utc("annotation_at")?,
            file: self.opt("file"),
            sha256: self.opt("sha256"),
            note: self.get("annotation_note").to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsvError {
    MissingHeader,
    UnexpectedHeader,
    UnterminatedQuote,
    BadRow { line: usize, reason: String },
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CsvError::MissingHeader => write!(f, "index has no header row"),
            CsvError::UnexpectedHeader => write!(f, "index header does not match this schema"),
            CsvError::UnterminatedQuote => write!(f, "unterminated quoted field"),
            CsvError::BadRow { line, reason } => write!(f, "row {line}: {reason}"),
        }
    }
}

impl std::error::Error for CsvError {}

/// Appends one RFC 4180 record (CRLF-terminated). A field is quoted
/// when it holds a comma, quote, CR or LF; quotes are doubled.
fn write_csv_record<'a>(out: &mut String, fields: impl Iterator<Item = &'a str>) {
    for (i, field) in fields.enumerate() {
        if i > 0 {
            out.push(',');
        }
        if field.contains([',', '"', '\r', '\n']) {
            out.push('"');
            out.push_str(&field.replace('"', "\"\""));
            out.push('"');
        } else {
            out.push_str(field);
        }
    }
    out.push_str("\r\n");
}

/// Parses RFC 4180 text into records. Accepts CRLF or LF line ends.
fn parse_csv(text: &str) -> Result<Vec<Vec<String>>, CsvError> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_quotes {
            match ch {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    field.push('"');
                }
                '"' => in_quotes = false,
                _ => field.push(ch),
            }
            continue;
        }
        match ch {
            '"' => in_quotes = true,
            ',' => record.push(std::mem::take(&mut field)),
            '\r' if chars.peek() == Some(&'\n') => {}
            '\n' => {
                record.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut record));
            }
            _ => field.push(ch),
        }
    }
    if in_quotes {
        return Err(CsvError::UnterminatedQuote);
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_quoting_round_trips() {
        let mut out = String::new();
        write_csv_record(
            &mut out,
            ["a", "b,c", "say \"hi\"", "two\r\nlines", ""].into_iter(),
        );
        let rows = parse_csv(&out).unwrap();
        assert_eq!(
            rows,
            vec![vec!["a", "b,c", "say \"hi\"", "two\r\nlines", ""]]
        );
    }

    #[test]
    fn unresolvable_tzid_is_kept_without_a_utc_instant() {
        let t = TimeValue::new("20260115T120000", Some("Pacific Standard Time"));
        assert_eq!(t.tzid.as_deref(), Some("Pacific Standard Time"));
        assert!(t.utc.is_none());
        let floating = TimeValue::new("20260115T120000", None);
        assert!(floating.utc.is_none());
        let date = TimeValue::new("20260115", None);
        assert!(date.utc.is_none());
    }

    #[test]
    fn organizer_is_normalised() {
        assert_eq!(
            normalise_organizer("MAILTO:Alice@Example.com"),
            "alice@example.com"
        );
        assert_eq!(
            normalise_organizer("alice@example.com"),
            "alice@example.com"
        );
    }
}
