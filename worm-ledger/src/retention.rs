// SPDX-License-Identifier: Apache-2.0
//
// Retention-class vocabulary + expiry computation, for the two-tier
// content/metadata split ratified in BRIEF-cross-compliance-worm-design.md
// (project-input) Decisions locked #7. Same advisory posture as
// service-input's registry.rs/taxonomy.rs: a missing or malformed
// vocabulary file degrades to "every class is unknown," it never
// panics and never blocks the daemon from starting. An unknown
// retention class on an incoming append IS a hard 400 at the HTTP
// layer, though — see http.rs — advisory-at-load, strict-at-use.
//
// Anchor fix (moved into worm-ledger, BRIEF-os-totebox-communication-
// complex-2026-09.md): `compute_expiry` originally always anchored on
// record-creation time. Real regimes don't: FINRA 4511 and CRA/ITA
// s.230 anchor on fiscal-year-end / end of the fiscal year the last
// entry was made in, not on when a given record was created. A
// service's own creation timestamp is only ever the *right* anchor
// for a `Created`-anchored class; getting this wrong silently computes
// a wrong expiry (and therefore a wrong disposal date) for every
// consumer. `RetentionClass::anchor` makes the anchor type an explicit,
// per-class fact instead of an assumption `compute_expiry`'s caller
// has to already know; `compute_expiry` itself stays a pure
// "anchor date + duration" function (it doesn't know how to compute a
// fiscal-year-end or last-entry date — that's deployment/service-
// specific), it just now makes it impossible to pass the wrong kind of
// date without knowing you're doing it.

use chrono::{DateTime, Duration, Utc};
use std::collections::BTreeMap;

const EXPECTED_COLUMNS_MIN: usize = 4;
const EXPECTED_COLUMNS_MAX: usize = 5;

/// What a retention class's `duration_days` counts forward from.
/// Defaults to `Created` (5th CSV column blank/absent) for backward
/// compatibility with vocabularies written before this field existed
/// — those were all originally (and correctly, for their use case)
/// creation-anchored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RetentionAnchor {
    /// The record's own creation timestamp. Correct for classes with
    /// no regulatory fiscal-year framing (e.g. `internal-90d`).
    #[default]
    Created,
    /// The end of the fiscal year the record relates to (e.g. FINRA
    /// 4511's "books of original entry": 6 years from fiscal year end,
    /// not from creation).
    FiscalYearEnd,
    /// The end of the fiscal year in which the *last* entry in the
    /// same record group was made (e.g. CRA/ITA s.230's "six years
    /// from the end of the last taxation year to which the records
    /// relate").
    LastEntry,
}

impl RetentionAnchor {
    fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "" | "created" => Some(RetentionAnchor::Created),
            "fiscal-year-end" => Some(RetentionAnchor::FiscalYearEnd),
            "last-entry" => Some(RetentionAnchor::LastEntry),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetentionClass {
    pub class: String,
    /// `None` means no-expiry — permanent retention, never eligible
    /// for disposal (D11's "no expiry" per-deployment setting).
    pub duration_days: Option<i64>,
    pub regime: String,
    pub notes: String,
    /// What `duration_days` counts forward from. See the module doc
    /// for why this exists — do not assume `Created`.
    pub anchor: RetentionAnchor,
}

#[derive(Debug, Default)]
pub struct RetentionVocabulary {
    pub classes: BTreeMap<String, RetentionClass>,
    pub warnings: Vec<String>,
}

impl RetentionVocabulary {
    pub fn get(&self, class: &str) -> Option<&RetentionClass> {
        self.classes.get(class)
    }
}

pub fn load_retention_classes(path: &str) -> RetentionVocabulary {
    let mut warnings = Vec::new();

    let raw = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            warnings.push(format!(
                "retention-classes.csv not readable at {path}: {e} — all retention \
                 classes will be rejected as unknown until this is fixed"
            ));
            return RetentionVocabulary {
                classes: BTreeMap::new(),
                warnings,
            };
        }
    };

    let filtered: String = raw
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(filtered.as_bytes());

    let mut classes = BTreeMap::new();
    for (i, result) in reader.records().enumerate() {
        let rec = match result {
            Ok(r) => r,
            Err(e) => {
                warnings.push(format!("row {}: parse error: {e}", i + 1));
                continue;
            }
        };
        if rec.len() < EXPECTED_COLUMNS_MIN || rec.len() > EXPECTED_COLUMNS_MAX {
            warnings.push(format!(
                "row {} ('{}'): expected {}-{} columns (class,duration_days,regime,notes,anchor?), got {}",
                i + 1,
                rec.get(0).unwrap_or("?"),
                EXPECTED_COLUMNS_MIN,
                EXPECTED_COLUMNS_MAX,
                rec.len()
            ));
            continue;
        }
        let class = rec.get(0).unwrap_or("").trim().to_string();
        if class.is_empty() {
            warnings.push(format!("row {}: empty class name, skipping", i + 1));
            continue;
        }
        let duration_field = rec.get(1).unwrap_or("").trim();
        let duration_days = if duration_field.is_empty() {
            None
        } else {
            match duration_field.parse::<i64>() {
                Ok(d) if d > 0 => Some(d),
                _ => {
                    warnings.push(format!(
                        "row {} ('{class}'): duration_days {:?} is not a positive \
                         integer or blank (blank = no-expiry) — skipping",
                        i + 1,
                        duration_field
                    ));
                    continue;
                }
            }
        };
        if classes.contains_key(&class) {
            warnings.push(format!(
                "row {} ('{class}'): duplicate retention class — keeping first definition",
                i + 1
            ));
            continue;
        }
        let anchor_field = rec.get(4).unwrap_or("");
        let anchor = match RetentionAnchor::parse(anchor_field) {
            Some(a) => a,
            None => {
                warnings.push(format!(
                    "row {} ('{class}'): unrecognized anchor {anchor_field:?} \
                     (expected blank, created, fiscal-year-end, or last-entry) \
                     — defaulting to created",
                    i + 1
                ));
                RetentionAnchor::Created
            }
        };
        classes.insert(
            class.clone(),
            RetentionClass {
                class,
                duration_days,
                regime: rec.get(2).unwrap_or("").trim().to_string(),
                notes: rec.get(3).unwrap_or("").trim().to_string(),
                anchor,
            },
        );
    }

    RetentionVocabulary { classes, warnings }
}

/// Compute expiry from an anchor timestamp and a retention class.
/// `None` means no-expiry (never eligible for disposal).
///
/// `anchor_date` MUST be the date appropriate to `class.anchor` — this
/// function does not compute fiscal-year-end or last-entry dates
/// itself (that requires deployment/service-specific context this
/// module doesn't have), it only adds `duration_days` to whatever
/// anchor date it's given. Passing a record's creation timestamp for
/// a `FiscalYearEnd`- or `LastEntry`-anchored class computes a wrong
/// expiry silently — check `class.anchor` before calling this.
pub fn compute_expiry(anchor_date: DateTime<Utc>, class: &RetentionClass) -> Option<DateTime<Utc>> {
    class.duration_days.map(|d| anchor_date + Duration::days(d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_csv(contents: &str) -> std::path::PathBuf {
        static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = CTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "svc-fs-retention-test-{}-{}.csv",
            std::process::id(),
            n
        ));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        path
    }

    #[test]
    fn missing_file_yields_empty_vocab_with_warning() {
        let vocab = load_retention_classes("/nonexistent/path/retention-classes.csv");
        assert!(vocab.classes.is_empty());
        assert_eq!(vocab.warnings.len(), 1);
        assert!(vocab.get("finra-6y").is_none());
    }

    #[test]
    fn loads_real_classes_with_and_without_expiry() {
        let path = write_temp_csv(
            "class,duration_days,regime,notes\n\
             finra-6y,2192,FINRA Rule 4511,6-year default retention\n\
             no-expiry,,NARA-style permanent,never eligible for disposal\n",
        );
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert!(vocab.warnings.is_empty(), "warnings: {:?}", vocab.warnings);
        assert_eq!(vocab.classes.len(), 2);

        let finra = vocab.get("finra-6y").unwrap();
        assert_eq!(finra.duration_days, Some(2192));
        assert_eq!(finra.regime, "FINRA Rule 4511");

        let no_expiry = vocab.get("no-expiry").unwrap();
        assert_eq!(no_expiry.duration_days, None);
    }

    #[test]
    fn unknown_class_is_none() {
        let path = write_temp_csv("class,duration_days,regime,notes\nfinra-6y,2192,FINRA,x\n");
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert!(vocab.get("no-such-class").is_none());
    }

    #[test]
    fn invalid_duration_row_is_skipped_with_warning() {
        let path = write_temp_csv(
            "class,duration_days,regime,notes\n\
             finra-6y,2192,FINRA,x\n\
             bad-class,not-a-number,Foo,y\n\
             negative-class,-5,Foo,z\n",
        );
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert_eq!(vocab.classes.len(), 1);
        assert_eq!(vocab.warnings.len(), 2);
        assert!(vocab.get("bad-class").is_none());
        assert!(vocab.get("negative-class").is_none());
    }

    #[test]
    fn duplicate_class_keeps_first_and_warns() {
        let path = write_temp_csv(
            "class,duration_days,regime,notes\n\
             finra-6y,2192,First,x\n\
             finra-6y,1096,Second,y\n",
        );
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert_eq!(vocab.classes.len(), 1);
        assert_eq!(vocab.warnings.len(), 1);
        assert_eq!(vocab.get("finra-6y").unwrap().regime, "First");
    }

    #[test]
    fn compute_expiry_no_expiry_class_returns_none() {
        let class = RetentionClass {
            class: "no-expiry".into(),
            duration_days: None,
            regime: "NARA".into(),
            notes: String::new(),
            anchor: RetentionAnchor::Created,
        };
        let created = DateTime::parse_from_rfc3339("2026-09-04T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(compute_expiry(created, &class), None);
    }

    #[test]
    fn real_shipped_fixture_loads_with_zero_warnings() {
        // Regression test for a real bug caught only by actually running
        // the daemon end-to-end: the shipped CSV's finra-6y notes field
        // originally had an unquoted internal comma, which `flexible(true)`
        // CSV parsing silently split into a 5th column, causing the whole
        // row (and therefore the class) to be dropped with only a log
        // warning — never a hard failure. Assert the real file loads
        // clean and every expected class is present.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/retention-classes.csv");
        let vocab = load_retention_classes(path);
        assert!(
            vocab.warnings.is_empty(),
            "real shipped retention-classes.csv must load with zero warnings: {:?}",
            vocab.warnings
        );
        for expected in ["finra-6y", "finra-3y", "mifid-5y", "no-expiry", "internal-90d"] {
            assert!(
                vocab.get(expected).is_some(),
                "expected class {expected:?} missing from real shipped fixture"
            );
        }
        assert_eq!(vocab.get("no-expiry").unwrap().duration_days, None);
        assert_eq!(vocab.get("finra-6y").unwrap().duration_days, Some(2192));
        assert_eq!(
            vocab.get("finra-6y").unwrap().anchor,
            RetentionAnchor::LastEntry,
            "books-of-original-entry must anchor on last-entry, not creation"
        );
        assert_eq!(vocab.get("finra-3y").unwrap().anchor, RetentionAnchor::Created);
    }

    #[test]
    fn compute_expiry_bounded_class_adds_days() {
        let class = RetentionClass {
            class: "finra-3y".into(),
            duration_days: Some(1096),
            regime: "FINRA Rule 4511".into(),
            notes: String::new(),
            anchor: RetentionAnchor::Created,
        };
        let created = DateTime::parse_from_rfc3339("2026-09-04T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let expires = compute_expiry(created, &class).unwrap();
        assert_eq!(expires, created + Duration::days(1096));
        assert!(expires.to_rfc3339().starts_with("2029-"));
    }

    #[test]
    fn anchor_defaults_to_created_when_column_absent() {
        let path = write_temp_csv("class,duration_days,regime,notes\nfinra-6y,2192,FINRA,x\n");
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert_eq!(vocab.get("finra-6y").unwrap().anchor, RetentionAnchor::Created);
        assert!(vocab.warnings.is_empty());
    }

    #[test]
    fn anchor_parses_fiscal_year_end_and_last_entry() {
        let path = write_temp_csv(
            "class,duration_days,regime,notes,anchor\n\
             finra-books,2192,FINRA 4511,books of original entry,fiscal-year-end\n\
             cra-6y,2192,ITA s.230,,last-entry\n",
        );
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert!(vocab.warnings.is_empty(), "warnings: {:?}", vocab.warnings);
        assert_eq!(vocab.get("finra-books").unwrap().anchor, RetentionAnchor::FiscalYearEnd);
        assert_eq!(vocab.get("cra-6y").unwrap().anchor, RetentionAnchor::LastEntry);
    }

    #[test]
    fn unrecognized_anchor_defaults_to_created_with_warning() {
        let path = write_temp_csv(
            "class,duration_days,regime,notes,anchor\nweird,10,X,y,not-a-real-anchor\n",
        );
        let vocab = load_retention_classes(path.to_str().unwrap());
        assert_eq!(vocab.get("weird").unwrap().anchor, RetentionAnchor::Created);
        assert_eq!(vocab.warnings.len(), 1);
        assert!(vocab.warnings[0].contains("unrecognized anchor"));
    }
}
