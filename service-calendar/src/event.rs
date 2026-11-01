// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A calendar event — `BRIEF-os-totebox-communication-complex-2026-
/// 09.md` §12's `service-calendar` (absorbs the `service-scheduling`
/// roster-amendment proposal). The raw ICS text is kept verbatim
/// (matching `service-email`'s "keep the raw EML" discipline) — GET
/// and REPORT responses serve `raw_ics` byte-for-byte, never a
/// re-serialized reconstruction. `summary`/`dtstart`/`dtend` are a
/// parsed index over it for lookups/free-busy, not the source of
/// truth.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// The VEVENT's own UID (RFC5545 UID property) — the resource's
    /// stable identity, independent of its `.ics` filename.
    pub uid: String,
    pub raw_ics: String,
    pub summary: String,
    pub dtstart: DateTime<Utc>,
    pub dtend: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidIcs(String),
    NoEventComponent,
    MissingUid,
    MissingDtstart,
    InvalidRrule(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidIcs(e) => write!(f, "invalid ICS: {e}"),
            ParseError::NoEventComponent => write!(f, "no VEVENT component found"),
            ParseError::MissingUid => write!(f, "VEVENT is missing UID"),
            ParseError::MissingDtstart => write!(f, "VEVENT is missing DTSTART"),
            ParseError::InvalidRrule(e) => write!(f, "invalid RRULE: {e}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// `(uid, summary, dtstart, dtend)`, as parsed out of one VEVENT.
pub type ParsedEvent = (String, String, DateTime<Utc>, Option<DateTime<Utc>>);

/// Parses the first VEVENT out of `raw` (one event per CalDAV
/// resource, per RFC4791).
pub fn parse_ics(raw: &str) -> Result<ParsedEvent, ParseError> {
    use icalendar::Component;

    // `icalendar::parser::read_calendar` returns a parser-local
    // `parser::Calendar<'_>` type, distinct from the top-level
    // `icalendar::Calendar` builder type that has `.events()` --
    // `str::parse::<icalendar::Calendar>()` goes through
    // `FromStr::from_str` (which itself calls `read_calendar` +
    // `.into()`), landing directly on the type with `.events()`.
    let calendar: icalendar::Calendar = raw.parse().map_err(ParseError::InvalidIcs)?;
    let event = calendar.events().next().ok_or(ParseError::NoEventComponent)?;
    let uid = event.get_uid().ok_or(ParseError::MissingUid)?.to_string();
    let summary = event.get_summary().unwrap_or("").to_string();
    let dtstart = event
        .get_start()
        .and_then(date_perhaps_time_to_utc)
        .ok_or(ParseError::MissingDtstart)?;
    let dtend = event.get_end().and_then(date_perhaps_time_to_utc);
    Ok((uid, summary, dtstart, dtend))
}

fn date_perhaps_time_to_utc(d: icalendar::DatePerhapsTime) -> Option<DateTime<Utc>> {
    match d {
        icalendar::DatePerhapsTime::DateTime(cdt) => Some(calendar_date_time_to_utc(&cdt)),
        icalendar::DatePerhapsTime::Date(date) => {
            date.and_hms_opt(0, 0, 0).map(|ndt| Utc.from_utc_datetime(&ndt))
        }
    }
}

/// Resolves a `CalendarDateTime` to UTC, honoring a real `TZID=...`
/// IANA zone when present (via `chrono-tz`'s zone database) instead of
/// treating the naive local time as if it were already UTC. Falls back
/// to the naive-as-UTC behavior only if the TZID string doesn't
/// resolve to a real zone (e.g. a non-standard/vendor-specific TZID) —
/// a real, deliberately soft degradation for a value this crate can't
/// interpret, not a parse failure severe enough to reject the whole
/// event over.
fn calendar_date_time_to_utc(cdt: &icalendar::CalendarDateTime) -> DateTime<Utc> {
    match cdt {
        icalendar::CalendarDateTime::Utc(dt) => *dt,
        icalendar::CalendarDateTime::Floating(naive) => Utc.from_utc_datetime(naive),
        icalendar::CalendarDateTime::WithTimezone { date_time, tzid } => {
            match chrono_tz::Tz::from_str(tzid) {
                Ok(zone) => match zone.from_local_datetime(date_time).single() {
                    Some(local_dt) => local_dt.with_timezone(&Utc),
                    // Ambiguous (DST fall-back overlap) or nonexistent
                    // (DST spring-forward gap) local time -- a real,
                    // rare edge case chrono's own API surfaces, not
                    // something this crate can resolve on the caller's
                    // behalf without more context (which instant the
                    // client meant). Falls back to naive-as-UTC rather
                    // than erroring the whole event out.
                    None => Utc.from_utc_datetime(date_time),
                },
                Err(_) => Utc.from_utc_datetime(date_time),
            }
        }
    }
}

/// Expands a VEVENT's `RRULE` recurrence into concrete UTC occurrence
/// instants within `[window_start, window_end]`, inclusive. Returns
/// `Ok(vec![dtstart])` for a non-recurring event (no `RRULE` property)
/// if `dtstart` itself falls in the window, `Ok(vec![])` otherwise --
/// expansion is a windowed query, not "give me every occurrence
/// forever," since an unbounded `RRULE` (no `COUNT`/`UNTIL`) would
/// otherwise recur indefinitely.
///
/// TZID is resolved to UTC first (via [`calendar_date_time_to_utc`])
/// and the `RRULE` is then expanded purely in UTC — DST-transition
/// wall-clock-time drift across a long recurrence series is a real,
/// known, deliberately out-of-scope edge case for this pass (the
/// `rrule` crate's own `Tz` type could carry the zone through instead,
/// a real follow-up if a client ever depends on wall-clock-stable
/// recurrence across a DST boundary).
pub fn expand_occurrences(
    raw: &str,
    window_start: DateTime<Utc>,
    window_end: DateTime<Utc>,
) -> Result<Vec<DateTime<Utc>>, ParseError> {
    use icalendar::Component;

    let calendar: icalendar::Calendar = raw.parse().map_err(ParseError::InvalidIcs)?;
    let event = calendar.events().next().ok_or(ParseError::NoEventComponent)?;
    let dtstart = event
        .get_start()
        .and_then(date_perhaps_time_to_utc)
        .ok_or(ParseError::MissingDtstart)?;

    let rrule_value = event.properties().get("RRULE").map(icalendar::Property::value);
    let Some(rrule_value) = rrule_value else {
        return Ok(if dtstart >= window_start && dtstart <= window_end {
            vec![dtstart]
        } else {
            vec![]
        });
    };

    // rrule's own parser expects an RFC5545 DTSTART+RRULE text block,
    // not a full VCALENDAR/VEVENT -- built from the already-UTC-
    // resolved dtstart (sidesteps re-parsing TZID inside rrule itself,
    // per the DST-scoping note above) plus the raw RRULE property text
    // verbatim.
    let block = format!(
        "DTSTART:{}\nRRULE:{}",
        dtstart.format("%Y%m%dT%H%M%SZ"),
        rrule_value
    );
    let rrule_set = rrule::RRuleSet::from_str(&block).map_err(|e| ParseError::InvalidRrule(e.to_string()))?;

    let tz_start = window_start.with_timezone(&rrule::Tz::UTC);
    let tz_end = window_end.with_timezone(&rrule::Tz::UTC);
    let result = rrule_set.after(tz_start).before(tz_end).all(10_000);
    Ok(result.dates.into_iter().map(|dt| dt.with_timezone(&Utc)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EVENT: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//EN\r\nBEGIN:VEVENT\r\nUID:event-1@example.com\r\nDTSTART:20260101T120000Z\r\nDTEND:20260101T130000Z\r\nSUMMARY:New Year Meeting\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";

    #[test]
    fn parses_uid_summary_and_times() {
        let (uid, summary, dtstart, dtend) = parse_ics(SIMPLE_EVENT).unwrap();
        assert_eq!(uid, "event-1@example.com");
        assert_eq!(summary, "New Year Meeting");
        assert_eq!(dtstart.to_rfc3339(), "2026-01-01T12:00:00+00:00");
        assert_eq!(dtend.unwrap().to_rfc3339(), "2026-01-01T13:00:00+00:00");
    }

    #[test]
    fn missing_uid_is_an_error() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nDTSTART:20260101T120000Z\r\nSUMMARY:No UID\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        assert!(matches!(parse_ics(ics), Err(ParseError::MissingUid)));
    }

    #[test]
    fn no_vevent_is_an_error() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nEND:VCALENDAR\r\n";
        assert!(matches!(parse_ics(ics), Err(ParseError::NoEventComponent)));
    }

    #[test]
    fn dtend_optional() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:e2@example.com\r\nDTSTART:20260101T120000Z\r\nSUMMARY:No end\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (_, _, _, dtend) = parse_ics(ics).unwrap();
        assert!(dtend.is_none());
    }

    // ── TZID resolution ──────────────────────────────────────────────

    #[test]
    fn named_timezone_resolves_to_the_real_utc_offset() {
        // America/New_York is UTC-5 in January (standard time, no DST).
        // A naive-as-UTC misinterpretation would read this as 12:00 UTC;
        // the real, correct UTC instant is 17:00.
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:tz1@example.com\r\nDTSTART;TZID=America/New_York:20260115T120000\r\nSUMMARY:NY Meeting\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (_, _, dtstart, _) = parse_ics(ics).unwrap();
        assert_eq!(dtstart.to_rfc3339(), "2026-01-15T17:00:00+00:00");
    }

    #[test]
    fn named_timezone_resolves_correctly_across_dst() {
        // America/New_York is UTC-4 in July (daylight time).
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:tz2@example.com\r\nDTSTART;TZID=America/New_York:20260715T120000\r\nSUMMARY:NY Summer Meeting\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (_, _, dtstart, _) = parse_ics(ics).unwrap();
        assert_eq!(dtstart.to_rfc3339(), "2026-07-15T16:00:00+00:00");
    }

    #[test]
    fn unknown_tzid_falls_back_to_naive_as_utc_rather_than_erroring() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:tz3@example.com\r\nDTSTART;TZID=Not/ARealZone:20260101T120000\r\nSUMMARY:Bad TZID\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (_, _, dtstart, _) = parse_ics(ics).unwrap();
        assert_eq!(dtstart.to_rfc3339(), "2026-01-01T12:00:00+00:00");
    }

    // ── RRULE expansion ──────────────────────────────────────────────

    fn window(start: &str, end: &str) -> (DateTime<Utc>, DateTime<Utc>) {
        (
            DateTime::parse_from_rfc3339(start).unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339(end).unwrap().with_timezone(&Utc),
        )
    }

    #[test]
    fn non_recurring_event_expands_to_itself_when_in_window() {
        let (start, end) = window("2026-01-01T00:00:00Z", "2026-01-31T00:00:00Z");
        let occurrences = expand_occurrences(SIMPLE_EVENT, start, end).unwrap();
        assert_eq!(occurrences.len(), 1);
        assert_eq!(occurrences[0].to_rfc3339(), "2026-01-01T12:00:00+00:00");
    }

    #[test]
    fn non_recurring_event_outside_window_expands_to_nothing() {
        let (start, end) = window("2026-02-01T00:00:00Z", "2026-02-28T00:00:00Z");
        let occurrences = expand_occurrences(SIMPLE_EVENT, start, end).unwrap();
        assert!(occurrences.is_empty());
    }

    #[test]
    fn daily_rrule_expands_to_the_real_count_of_occurrences() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:daily@example.com\r\nDTSTART:20260101T090000Z\r\nRRULE:FREQ=DAILY;COUNT=5\r\nSUMMARY:Daily Standup\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (start, end) = window("2026-01-01T00:00:00Z", "2026-01-31T00:00:00Z");
        let occurrences = expand_occurrences(ics, start, end).unwrap();
        assert_eq!(occurrences.len(), 5);
        assert_eq!(occurrences[0].to_rfc3339(), "2026-01-01T09:00:00+00:00");
        assert_eq!(occurrences[4].to_rfc3339(), "2026-01-05T09:00:00+00:00");
        // Consecutive days, same time-of-day each occurrence.
        for pair in occurrences.windows(2) {
            assert_eq!((pair[1] - pair[0]).num_days(), 1);
        }
    }

    #[test]
    fn weekly_rrule_respects_the_query_window_boundaries() {
        // Weekly on Thursdays, unbounded (no COUNT/UNTIL) -- expansion
        // must still terminate because it's clipped to the window, not
        // because the rule itself is bounded.
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:weekly@example.com\r\nDTSTART:20260101T140000Z\r\nRRULE:FREQ=WEEKLY;BYDAY=TH\r\nSUMMARY:Weekly Sync\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        // Jan 2026: Thursdays fall on the 1st, 8th, 15th, 22nd, 29th.
        let (start, end) = window("2026-01-01T00:00:00Z", "2026-01-20T00:00:00Z");
        let occurrences = expand_occurrences(ics, start, end).unwrap();
        assert_eq!(occurrences.len(), 3);
        assert_eq!(occurrences[0].to_rfc3339(), "2026-01-01T14:00:00+00:00");
        assert_eq!(occurrences[1].to_rfc3339(), "2026-01-08T14:00:00+00:00");
        assert_eq!(occurrences[2].to_rfc3339(), "2026-01-15T14:00:00+00:00");
    }

    #[test]
    fn invalid_rrule_is_a_real_error_not_a_silent_empty_result() {
        let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:bad@example.com\r\nDTSTART:20260101T090000Z\r\nRRULE:THIS IS NOT VALID\r\nSUMMARY:Broken\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let (start, end) = window("2026-01-01T00:00:00Z", "2026-01-31T00:00:00Z");
        assert!(matches!(expand_occurrences(ics, start, end), Err(ParseError::InvalidRrule(_))));
    }
}
