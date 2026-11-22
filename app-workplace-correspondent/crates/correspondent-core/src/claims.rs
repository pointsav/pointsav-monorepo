// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Claims: derived facts about a person (a name variant, a company), read from
//! `service-people/claims.csv` when that file exists.
//!
//! Rules taken from project-email's registry constitution (all PROPOSED, not
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
use std::collections::HashSet;
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
    dead: HashSet<String>,
    confirmed: HashSet<String>,
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
            let claim_id = row.get(id).unwrap_or("").to_string();
            if !seen.insert(claim_id.clone()) {
                return Err(Error::DuplicateId {
                    file: t.file.clone(),
                    id: claim_id,
                });
            }
            rows.push(Claim {
                claim_id,
                action: row.get(action).unwrap_or("").trim().to_lowercase(),
                subject: row.get(subject).unwrap_or("").to_string(),
                attribute: row.get(attribute).unwrap_or("").to_string(),
                value: row.get(value).unwrap_or("").trim().to_string(),
                confidence: opt(row, conf),
                method: opt(row, method),
                source_ref: opt(row, src),
                observed_at: opt(row, obs),
                supersedes: opt(row, sup),
            });
        }
        let (mut dead, mut confirmed) = (HashSet::new(), HashSet::new());
        for c in &rows {
            let Some(target) = &c.supersedes else {
                continue;
            };
            match c.action.as_str() {
                // A correction replaces its target; a reject or retract withdraws it.
                "assert" | "reject" | "retract" => {
                    dead.insert(target.clone());
                }
                "confirm" => {
                    confirmed.insert(target.clone());
                }
                _ => {}
            }
        }
        Ok(Self {
            rows,
            dead,
            confirmed,
        })
    }

    /// Asserted claims that nothing has withdrawn or replaced.
    pub(crate) fn active(&self) -> impl Iterator<Item = &Claim> {
        self.rows
            .iter()
            .filter(|c| c.action == "assert" && !self.dead.contains(&c.claim_id))
    }

    pub(crate) fn is_confirmed(&self, claim_id: &str) -> bool {
        self.confirmed.contains(claim_id)
    }
}

/// Provisional name match for linking a company claim to a business: lower-case,
/// trimmed, runs of whitespace collapsed. "Ltd" and "Limited" stay different.
/// project-email has not defined its normalisation for this link; replace this
/// when it does.
pub fn normalise_name(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
