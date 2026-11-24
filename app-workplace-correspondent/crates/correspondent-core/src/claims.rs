// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Claims: derived facts about a person (a name variant, a company), read from
//! `service-people/claims.csv` when that file exists.
//!
//! Rules taken from the registry's proposed rules (all PROPOSED, not
//! ratified, so this reader is tolerant and changes in one place):
//! - claims are append-only; nothing is edited. A correction, confirmation or
//!   rejection is a new row whose `supersedes` names the claim it acts on;
//! - `action` is `assert`, `confirm`, `reject` or `retract`;
//! - `subject` is `service-people.<person_id>`; a bare person id is accepted too.
//!   Strong-key subjects such as `email:...` belong to the extractor's own state
//!   tables and match no person here;
//! - a company is text, not a business id.

use crate::registry::{opt, Table};
use crate::Error;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// One row of `claims.csv`. Columns are found by name, so extra or missing
/// optional columns (for example `recorded_at`) do not matter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub claim_id: String,
    pub action: String,
    pub subject: String,
    pub attribute: String,
    pub value: String,
    pub confidence: Option<String>,
    pub method: Option<String>,
    pub source_ref: Option<String>,
    pub observed_at: Option<String>,
    pub supersedes: Option<String>,
}

impl Claim {
    /// The person id this claim is about, or `None` for any other kind of subject.
    pub fn person_subject(&self) -> Option<&str> {
        let s = self.subject.as_str();
        if let Some(id) = s.strip_prefix("service-people.") {
            return Some(id).filter(|i| !i.is_empty());
        }
        // Strong keys (`email:...`, `new:3`, `business:...`) and other services' ids are not people here.
        if s.is_empty() || s.contains(':') || s.contains('.') {
            None
        } else {
            Some(s)
        }
    }
}

/// All claims plus the facts derived from their `supersedes` chains.
#[derive(Debug, Default)]
pub(crate) struct Claims {
    rows: Vec<Claim>,
    confirmed: HashSet<String>,
    /// person id -> indices (file order) of live `assert` rows about that person
    by_person: HashMap<String, Vec<usize>>,
}

impl Claims {
    pub(crate) fn load(file: &Path) -> Result<Self, Error> {
        if !file.is_file() {
            return Ok(Self::default());
        }
        let t = Table::read(file)?;
        let id = t.required("claim_id")?;
        let action = t.required("action")?;
        let subject = t.required("subject")?;
        let attribute = t.required("attribute")?;
        let value = t.required("value")?;
        let (conf, method) = (t.column("confidence"), t.column("method"));
        let (src, obs) = (t.column("source_ref"), t.column("observed_at"));
        let sup = t.column("supersedes");
        let mut seen = HashSet::new();
        let mut rows = Vec::with_capacity(t.rows.len());
        for row in &t.rows {
            let claim_id = row.get(id).unwrap_or("").trim().to_string();
            // An empty id is not an identity: it can neither duplicate nor be superseded.
            if !claim_id.is_empty() && !seen.insert(claim_id.clone()) {
                return Err(Error::DuplicateId {
                    file: t.file.clone(),
                    id: claim_id,
                });
            }
            rows.push(Claim {
                claim_id,
                action: row.get(action).unwrap_or("").trim().to_lowercase(),
                subject: row.get(subject).unwrap_or("").trim().to_string(),
                attribute: row.get(attribute).unwrap_or("").to_string(),
                value: row.get(value).unwrap_or("").trim().to_string(),
                confidence: opt(row, conf),
                method: opt(row, method),
                source_ref: opt(row, src),
                observed_at: opt(row, obs),
                supersedes: opt(row, sup)
                    .map(|v| v.trim().to_string())
                    .filter(|v| !v.is_empty()),
            });
        }
        // Who acts on whom. A row acts only on a row of the SAME subject, so a row about one
        // person can never withdraw a claim about another.
        let index: HashMap<&str, usize> = rows
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.claim_id.is_empty())
            .map(|(i, c)| (c.claim_id.as_str(), i))
            .collect();
        let n = rows.len();
        let mut killers: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut confirmers: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (j, c) in rows.iter().enumerate() {
            let Some(t) = c.supersedes.as_deref().and_then(|s| index.get(s).copied()) else {
                continue;
            };
            if t == j || rows[t].subject != c.subject {
                continue;
            }
            match c.action.as_str() {
                // A correction replaces its target; a reject or retract withdraws it.
                "assert" | "reject" | "retract" => killers[t].push(j),
                "confirm" => confirmers[t].push(j),
                _ => {}
            }
        }
        // A row is alive unless a LIVE row replaces or withdraws it, so withdrawing a
        // withdrawal brings the original back. Repeat until nothing changes (bounded, so a
        // malformed cycle cannot loop forever).
        let mut alive = vec![true; n];
        for _ in 0..=100 {
            let mut changed = false;
            for i in 0..n {
                if killers[i].is_empty() {
                    continue;
                }
                let now = !killers[i].iter().any(|&j| alive[j]);
                if now != alive[i] {
                    alive[i] = now;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let confirmed: HashSet<String> = (0..n)
            .filter(|&i| confirmers[i].iter().any(|&j| alive[j]))
            .map(|i| rows[i].claim_id.clone())
            .collect();
        let mut by_person: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, c) in rows.iter().enumerate() {
            if alive[i] && c.action == "assert" {
                if let Some(p) = c.person_subject() {
                    by_person.entry(p.to_string()).or_default().push(i);
                }
            }
        }
        Ok(Self {
            rows,
            confirmed,
            by_person,
        })
    }

    /// Live asserted claims whose subject is any of `ids`, in file order.
    pub(crate) fn active_for(&self, ids: &[&str]) -> Vec<&Claim> {
        let mut ix: Vec<usize> = ids
            .iter()
            .filter_map(|id| self.by_person.get(*id))
            .flatten()
            .copied()
            .collect();
        if ids.len() > 1 {
            ix.sort_unstable();
        }
        ix.into_iter().map(|i| &self.rows[i]).collect()
    }

    pub(crate) fn is_confirmed(&self, claim_id: &str) -> bool {
        self.confirmed.contains(claim_id)
    }
}

/// Provisional name match for linking a company claim to a business: lower-case,
/// trimmed, runs of whitespace collapsed. "Ltd" and "Limited" stay different.
/// The registry has not defined its normalisation for this link; replace this
/// when it does.
pub fn normalise_name(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
