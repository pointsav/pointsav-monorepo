// SPDX-License-Identifier: Apache-2.0

use crate::event::CalendarEvent;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct EventStore {
    by_uid: RwLock<HashMap<String, CalendarEvent>>,
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            by_uid: RwLock::new(HashMap::new()),
        }
    }

    pub fn upsert(&self, event: CalendarEvent) {
        self.by_uid.write().unwrap().insert(event.uid.clone(), event);
    }

    pub fn remove(&self, uid: &str) -> Option<CalendarEvent> {
        self.by_uid.write().unwrap().remove(uid)
    }

    pub fn lookup_by_uid(&self, uid: &str) -> Option<CalendarEvent> {
        self.by_uid.read().unwrap().get(uid).cloned()
    }

    pub fn list_all(&self) -> Vec<CalendarEvent> {
        self.by_uid.read().unwrap().values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_event(uid: &str) -> CalendarEvent {
        CalendarEvent {
            uid: uid.to_string(),
            raw_ics: "BEGIN:VCALENDAR...".to_string(),
            summary: "Test".to_string(),
            dtstart: Utc::now(),
            dtend: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn upsert_then_lookup() {
        let store = EventStore::new();
        store.upsert(sample_event("e1"));
        assert!(store.lookup_by_uid("e1").is_some());
    }

    #[test]
    fn remove_deletes_entry() {
        let store = EventStore::new();
        store.upsert(sample_event("e1"));
        store.remove("e1");
        assert!(store.lookup_by_uid("e1").is_none());
    }

    #[test]
    fn list_all_returns_every_event() {
        let store = EventStore::new();
        store.upsert(sample_event("e1"));
        store.upsert(sample_event("e2"));
        assert_eq!(store.list_all().len(), 2);
    }
}
