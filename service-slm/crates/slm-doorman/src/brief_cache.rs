// SPDX-License-Identifier: Apache-2.0 OR MIT

//! In-process cache from `(brief_id, attempt_id)` to the brief / attempt
//! pair that produced it.
//!
//! AS-3 / AS-4 background: a verdict POST or shadow POST needs the
//! original brief and the apprentice's attempt to assemble the
//! apprenticeship corpus tuple (per `apprenticeship-substrate.md` §8).
//! The cleanest wire shape — senior posts only the verdict body +
//! signature — relies on the Doorman remembering the brief / attempt
//! that produced this `(brief_id, attempt_id)`. That memory lives here.
//!
//! Eviction policy is FIFO at a configurable cap (default 1024).
//! Process restart loses pending briefs; the senior reissues. SQLite-
//! backed durability is a v0.5+ upgrade per design-pass Q3.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use slm_core::{ApprenticeshipAttempt, ApprenticeshipBrief};

/// Cache key — `(brief_id, attempt_id)`.
pub type Key = (String, String);

#[derive(Clone, Debug)]
pub struct CachedBrief {
    pub brief: ApprenticeshipBrief,
    pub attempt: ApprenticeshipAttempt,
}

pub struct BriefCache {
    inner: Mutex<Inner>,
    cap: usize,
}

struct Inner {
    by_key: HashMap<Key, CachedBrief>,
    fifo: VecDeque<Key>,
}

impl BriefCache {
    pub fn new(cap: usize) -> Self {
        Self {
            inner: Mutex::new(Inner {
                by_key: HashMap::new(),
                fifo: VecDeque::new(),
            }),
            cap: cap.max(1),
        }
    }

    pub fn insert(&self, brief: ApprenticeshipBrief, attempt: ApprenticeshipAttempt) {
        let key: Key = (brief.brief_id.clone(), attempt.attempt_id.clone());
        let mut g = self.inner.lock().expect("brief cache mutex poisoned");
        if g.by_key.contains_key(&key) {
            return; // idempotent on retry
        }
        g.by_key.insert(key.clone(), CachedBrief { brief, attempt });
        g.fifo.push_back(key);
        while g.fifo.len() > self.cap {
            if let Some(k) = g.fifo.pop_front() {
                g.by_key.remove(&k);
            }
        }
    }

    pub fn get(&self, brief_id: &str, attempt_id: &str) -> Option<CachedBrief> {
        let key = (brief_id.to_string(), attempt_id.to_string());
        let g = self.inner.lock().expect("brief cache mutex poisoned");
        g.by_key.get(&key).cloned()
    }

    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .expect("brief cache mutex poisoned")
            .by_key
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for BriefCache {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use slm_core::{BriefScope, SeniorRole, Tier};

    fn brief(id: &str) -> ApprenticeshipBrief {
        ApprenticeshipBrief {
            brief_id: id.into(),
            created: Utc::now(),
            senior_role: SeniorRole::Master,
            senior_identity: "ps-administrator".into(),
            task_type: "version-bump-manifest".into(),
            scope: BriefScope::default(),
            acceptance_test: "T".into(),
            doctrine_citations: vec![],
            shadow: false,
            body: "B".into(),
        }
    }

    fn attempt(brief_id: &str, attempt_id: &str) -> ApprenticeshipAttempt {
        ApprenticeshipAttempt {
            brief_id: brief_id.into(),
            attempt_id: attempt_id.into(),
            created: Utc::now(),
            model: "olmo-3-1125-7b-q4".into(),
            adapter_composition: vec![],
            self_confidence: 0.9,
            escalate: false,
            inference_ms: 100,
            tier: Tier::Local,
            cost_usd: 0.0,
            reasoning: String::new(),
            diff: String::new(),
        }
    }

    #[test]
    fn round_trips_one_entry() {
        let c = BriefCache::new(8);
        c.insert(brief("b1"), attempt("b1", "a1"));
        let got = c.get("b1", "a1").expect("present");
        assert_eq!(got.brief.brief_id, "b1");
        assert_eq!(got.attempt.attempt_id, "a1");
    }

    #[test]
    fn evicts_in_fifo_order_at_cap() {
        let c = BriefCache::new(2);
        c.insert(brief("b1"), attempt("b1", "a1"));
        c.insert(brief("b2"), attempt("b2", "a2"));
        c.insert(brief("b3"), attempt("b3", "a3")); // evicts (b1,a1)
        assert!(c.get("b1", "a1").is_none());
        assert!(c.get("b2", "a2").is_some());
        assert!(c.get("b3", "a3").is_some());
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn idempotent_insert_on_retry() {
        let c = BriefCache::new(8);
        c.insert(brief("b1"), attempt("b1", "a1"));
        c.insert(brief("b1"), attempt("b1", "a1"));
        assert_eq!(c.len(), 1);
    }
}
