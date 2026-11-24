// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use crate::claims::{normalise_name, Claim, Claims};
use crate::Error;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// One row of `service-people/index.csv`. `person_id` is opaque.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub person_id: String,
    pub display_name: String,
    pub status: Option<String>,
    pub merged_into: Option<String>,
    pub kind: Option<String>,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
}

/// One row of `service-business/index.csv`. `business_id` is opaque.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Business {
    pub business_id: String,
    pub legal_name: String,
    pub jurisdiction: Option<String>,
    pub status: Option<String>,
    pub merged_into: Option<String>,
    pub kind: Option<String>,
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
}

/// A row is active unless its status is `merged` or `retired`.
/// A missing status counts as active (older registries have no status column).
fn is_active(status: &Option<String>) -> bool {
    !matches!(
        status
            .as_deref()
            .map(|s| s.trim().to_lowercase())
            .as_deref(),
        Some("merged") | Some("retired")
    )
}

impl Person {
    pub fn is_active(&self) -> bool {
        is_active(&self.status)
    }
}

impl Business {
    pub fn is_active(&self) -> bool {
        is_active(&self.status)
    }
}

/// The two registries, loaded read-only from their folders.
#[derive(Debug, Default)]
pub struct Registry {
    people: Vec<Person>,
    businesses: Vec<Business>,
    claims: Claims,
    person_ix: HashMap<String, usize>,
    business_ix: HashMap<String, usize>,
    /// normalised legal name -> indices into `businesses` (file order)
    business_by_name: HashMap<String, Vec<usize>>,
    /// active winner id -> ids of inactive records that resolve to it (file order)
    absorbed: HashMap<String, Vec<String>>,
}

impl Registry {
    /// Load `people_dir/index.csv` and `business_dir/index.csv`.
    pub fn open(people_dir: &Path, business_dir: &Path) -> Result<Self, Error> {
        let people = load_people(&people_dir.join("index.csv"))?;
        let businesses = load_businesses(&business_dir.join("index.csv"))?;
        let claims = Claims::load(&people_dir.join("claims.csv"))?;
        let person_ix = people
            .iter()
            .enumerate()
            .map(|(i, p)| (p.person_id.clone(), i))
            .collect();
        let business_ix = businesses
            .iter()
            .enumerate()
            .map(|(i, b)| (b.business_id.clone(), i))
            .collect();
        let mut business_by_name: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, b) in businesses.iter().enumerate() {
            business_by_name
                .entry(normalise_name(&b.legal_name))
                .or_default()
                .push(i);
        }
        let mut reg = Self {
            people,
            businesses,
            claims,
            person_ix,
            business_ix,
            business_by_name,
            absorbed: HashMap::new(),
        };
        let mut absorbed: HashMap<String, Vec<String>> = HashMap::new();
        for p in reg.people.iter().filter(|p| !p.is_active()) {
            if let Some(w) = reg.resolve_person(&p.person_id) {
                if w.person_id != p.person_id {
                    absorbed
                        .entry(w.person_id.clone())
                        .or_default()
                        .push(p.person_id.clone());
                }
            }
        }
        reg.absorbed = absorbed;
        Ok(reg)
    }

    pub fn people(&self) -> &[Person] {
        &self.people
    }

    pub fn businesses(&self) -> &[Business] {
        &self.businesses
    }

    pub fn person(&self, id: &str) -> Option<&Person> {
        self.person_ix.get(id).map(|&i| &self.people[i])
    }

    /// Follow `merged_into` from `id` to the current record. Returns `None`
    /// if an id is unknown or the chain loops. An active record resolves to itself.
    pub fn resolve_person(&self, id: &str) -> Option<&Person> {
        let mut cur = self.person(id)?;
        for _ in 0..=self.people.len() {
            match cur.merged_into.as_deref() {
                Some(next) if !cur.is_active() => cur = self.person(next)?,
                _ => return Some(cur),
            }
        }
        None
    }

    /// Follow `merged_into` from `id` to the current business record.
    pub fn resolve_business(&self, id: &str) -> Option<&Business> {
        let mut cur = self.business(id)?;
        for _ in 0..=self.businesses.len() {
            match cur.merged_into.as_deref() {
                Some(next) if !cur.is_active() => cur = self.business(next)?,
                _ => return Some(cur),
            }
        }
        None
    }

    /// Case-insensitive substring search on display name and on name-variant
    /// claims, sorted by display name. Merged and retired rows are hidden. An
    /// empty query returns every active person.
    pub fn search_people(&self, query: &str) -> Vec<&Person> {
        let q = query.to_lowercase();
        let mut hits: Vec<&Person> = self
            .people
            .iter()
            .filter(|p| {
                p.is_active()
                    && (p.display_name.to_lowercase().contains(&q)
                        || self
                            .names_of(&p.person_id)
                            .iter()
                            .any(|n| n.to_lowercase().contains(&q)))
            })
            .collect();
        hits.sort_by_key(|p| p.display_name.to_lowercase());
        hits
    }

    /// The name variant that made `p` match `query`, when the display name did
    /// not (so a list can say "also known as ...").
    pub fn matched_alias(&self, p: &Person, query: &str) -> Option<&str> {
        let q = query.to_lowercase();
        if q.is_empty() || p.display_name.to_lowercase().contains(&q) {
            return None;
        }
        self.names_of(&p.person_id)
            .into_iter()
            .find(|n| n.to_lowercase().contains(&q))
    }

    /// Ids whose claims belong on `person_id`'s page: the id itself plus every
    /// merged record that resolves to it (through any number of merge steps).
    /// Claims stay on the losing id in the registry (nothing is copied or
    /// re-pointed; project-email's PROPOSED rule), so the reader gathers them.
    /// Only an active record absorbs others; loops and dangling targets
    /// absorb nothing.
    fn merge_chain<'a>(&'a self, person_id: &'a str) -> Vec<&'a str> {
        let mut ids = vec![person_id];
        if self.person(person_id).is_some_and(Person::is_active) {
            if let Some(v) = self.absorbed.get(person_id) {
                ids.extend(v.iter().map(String::as_str));
            }
        }
        ids
    }

    /// Active claims about a person (asserted, not withdrawn or replaced),
    /// including claims left on records that were merged into this one.
    pub fn claims_for_person(&self, person_id: &str) -> Vec<&Claim> {
        let ids = self.merge_chain(person_id);
        self.claims.active_for(&ids)
    }

    /// Whether a later `confirm` row acts on this claim.
    pub fn is_confirmed(&self, claim: &Claim) -> bool {
        self.claims.is_confirmed(&claim.claim_id)
    }

    /// Distinct name variants for a person, other than the display name: active
    /// `name` claims (including those left on merged-in records) and the display
    /// names of records merged into this one.
    pub fn names_of(&self, person_id: &str) -> Vec<&str> {
        let display = self.person(person_id).map(|p| p.display_name.as_str());
        let mut out: Vec<&str> = Vec::new();
        for c in self.claims_for_person(person_id) {
            if c.attribute == "name"
                && !c.value.is_empty()
                && Some(c.value.as_str()) != display
                && !out.contains(&c.value.as_str())
            {
                out.push(&c.value);
            }
        }
        for id in self.merge_chain(person_id).into_iter().skip(1) {
            if let Some(p) = self.person(id) {
                let n = p.display_name.as_str();
                if !n.is_empty() && Some(n) != display && !out.contains(&n) {
                    out.push(n);
                }
            }
        }
        out
    }

    /// Active claims worth showing as contact details: every attribute except
    /// the ones shown elsewhere (`name`, `company`), the ones the registry row
    /// already carries (`first-seen`, `last-seen`) and the extractor's counters
    /// (`count-*`). Duplicates of the same attribute and value are dropped.
    pub fn details_of(&self, person_id: &str) -> Vec<&Claim> {
        let mut out: Vec<&Claim> = Vec::new();
        for c in self.claims_for_person(person_id) {
            let a = c.attribute.as_str();
            if a.is_empty()
                || c.value.is_empty()
                || matches!(a, "name" | "company" | "first-seen" | "last-seen")
                || a.starts_with("count-")
            {
                continue;
            }
            if out
                .iter()
                .any(|o| o.attribute == c.attribute && o.value == c.value)
            {
                continue;
            }
            out.push(c);
        }
        out
    }

    /// Distinct active company claim values (text) for a person.
    pub fn companies_of(&self, person_id: &str) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for c in self.claims_for_person(person_id) {
            if c.attribute == "company" && !c.value.is_empty() && !out.contains(&c.value.as_str()) {
                out.push(&c.value);
            }
        }
        out
    }

    /// The business a company name refers to, by provisional normalised-name
    /// equality (see [`normalise_name`]). Merged businesses resolve to their
    /// current record. `None` when nothing matches or the name is ambiguous.
    pub fn business_for_company(&self, company: &str) -> Option<&Business> {
        let key = normalise_name(company);
        if key.is_empty() {
            return None;
        }
        let mut found: Option<&Business> = None;
        for b in self
            .business_by_name
            .get(&key)
            .into_iter()
            .flatten()
            .map(|&i| &self.businesses[i])
        {
            // A row whose merge chain is broken or looped is skipped, not fatal; only
            // active, resolved businesses count towards ambiguity.
            let Some(cur) = self.resolve_business(&b.business_id) else {
                continue;
            };
            if !cur.is_active() {
                continue;
            }
            match found {
                Some(f) if f.business_id != cur.business_id => return None,
                _ => found = Some(cur),
            }
        }
        found
    }

    /// Active people with a company claim that resolves to this business.
    pub fn people_at_business(&self, business_id: &str) -> Vec<&Person> {
        let mut hits: Vec<&Person> = self
            .people
            .iter()
            .filter(|p| p.is_active())
            .filter(|p| {
                self.companies_of(&p.person_id).iter().any(|c| {
                    self.business_for_company(c)
                        .is_some_and(|b| b.business_id == business_id)
                })
            })
            .collect();
        hits.sort_by_key(|p| p.display_name.to_lowercase());
        hits
    }

    pub fn business(&self, id: &str) -> Option<&Business> {
        self.business_ix.get(id).map(|&i| &self.businesses[i])
    }

    /// Case-insensitive substring search on legal name, sorted by name.
    /// Merged and retired rows are hidden.
    pub fn search_businesses(&self, query: &str) -> Vec<&Business> {
        let q = query.to_lowercase();
        let mut hits: Vec<&Business> = self
            .businesses
            .iter()
            .filter(|b| b.is_active() && b.legal_name.to_lowercase().contains(&q))
            .collect();
        hits.sort_by_key(|b| b.legal_name.to_lowercase());
        hits
    }
}

pub(crate) struct Table {
    pub(crate) file: PathBuf,
    pub(crate) headers: csv::StringRecord,
    pub(crate) rows: Vec<csv::StringRecord>,
}

impl Table {
    pub(crate) fn read(file: &Path) -> Result<Self, Error> {
        if !file.is_file() {
            return Err(Error::MissingFile(file.to_path_buf()));
        }
        let csv_err = |source| Error::Csv {
            file: file.to_path_buf(),
            source,
        };
        // Flexible: one short or long row must not take the whole view down; a missing cell
        // reads as empty. Header problems still fail loudly through `required`.
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(file)
            .map_err(csv_err)?;
        let headers = rdr.headers().map_err(csv_err)?.clone();
        let rows = rdr
            .records()
            .collect::<Result<Vec<_>, _>>()
            .map_err(csv_err)?;
        Ok(Self {
            file: file.to_path_buf(),
            headers,
            rows,
        })
    }

    pub(crate) fn column(&self, name: &str) -> Option<usize> {
        self.headers.iter().position(|h| h == name)
    }

    pub(crate) fn required(&self, name: &'static str) -> Result<usize, Error> {
        self.column(name).ok_or_else(|| Error::MissingColumn {
            file: self.file.clone(),
            column: name,
        })
    }
}

/// Check `index.csv` against the folder's `schema.csv`, when one exists.
///
/// A folder without `schema.csv` is not checked. With one, the header must
/// begin with the schema's `index.csv` columns in the same order (extra
/// trailing columns are tolerated: columns are append-only), and a column the
/// schema marks `required = Y` must not be empty in any row.
fn check_schema(t: &Table) -> Result<(), Error> {
    let schema_file = match t.file.parent() {
        Some(dir) => dir.join("schema.csv"),
        None => return Ok(()),
    };
    if !schema_file.is_file() {
        return Ok(());
    }
    let s = Table::read(&schema_file)?;
    let col = s.required("column")?;
    let req = s.column("required");
    let table = s.column("table");
    let bad = |detail: String| Error::SchemaMismatch {
        file: t.file.clone(),
        detail,
    };
    let mut expected: Vec<(&str, bool)> = Vec::new();
    for row in &s.rows {
        let for_index = match table.and_then(|i| row.get(i)) {
            None | Some("") | Some("index.csv") => true,
            Some(_) => false,
        };
        if for_index {
            let name = row.get(col).unwrap_or("");
            let required = req
                .and_then(|i| row.get(i))
                .is_some_and(|v| v.eq_ignore_ascii_case("y"));
            expected.push((name, required));
        }
    }
    for (pos, (name, _)) in expected.iter().enumerate() {
        match t.headers.get(pos) {
            Some(h) if h == *name => {}
            Some(h) => {
                return Err(bad(format!(
                    "column {} is `{h}`, schema says `{name}`",
                    pos + 1
                )))
            }
            None => return Err(bad(format!("column `{name}` is missing from the header"))),
        }
    }
    for (pos, (name, required)) in expected.iter().enumerate() {
        if !required {
            continue;
        }
        for (n, row) in t.rows.iter().enumerate() {
            if row.get(pos).unwrap_or("").is_empty() {
                return Err(bad(format!(
                    "required column `{name}` is empty in data row {}",
                    n + 1
                )));
            }
        }
    }
    Ok(())
}

pub(crate) fn opt(row: &csv::StringRecord, idx: Option<usize>) -> Option<String> {
    idx.and_then(|i| row.get(i))
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}

fn load_people(file: &Path) -> Result<Vec<Person>, Error> {
    let t = Table::read(file)?;
    check_schema(&t)?;
    let id = t.required("person_id")?;
    let name = t.required("display_name")?;
    let merged = t.column("merged_into");
    let (status, kind) = (t.column("status"), t.column("kind"));
    let (first, last) = (t.column("first_seen"), t.column("last_seen"));
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(t.rows.len());
    for row in &t.rows {
        let person_id = row.get(id).unwrap_or("").to_string();
        if !seen.insert(person_id.clone()) {
            return Err(Error::DuplicateId {
                file: t.file.clone(),
                id: person_id,
            });
        }
        out.push(Person {
            person_id,
            display_name: row.get(name).unwrap_or("").to_string(),
            status: opt(row, status),
            merged_into: opt(row, merged),
            kind: opt(row, kind),
            first_seen: opt(row, first),
            last_seen: opt(row, last),
        });
    }
    Ok(out)
}

fn load_businesses(file: &Path) -> Result<Vec<Business>, Error> {
    let t = Table::read(file)?;
    check_schema(&t)?;
    let id = t.required("business_id")?;
    let name = t.required("legal_name")?;
    let juris = t.column("jurisdiction");
    let merged = t.column("merged_into");
    let (status, kind) = (t.column("status"), t.column("kind"));
    let (first, last) = (t.column("first_seen"), t.column("last_seen"));
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(t.rows.len());
    for row in &t.rows {
        let business_id = row.get(id).unwrap_or("").to_string();
        if !seen.insert(business_id.clone()) {
            return Err(Error::DuplicateId {
                file: t.file.clone(),
                id: business_id,
            });
        }
        out.push(Business {
            business_id,
            legal_name: row.get(name).unwrap_or("").to_string(),
            jurisdiction: opt(row, juris),
            status: opt(row, status),
            merged_into: opt(row, merged),
            kind: opt(row, kind),
            first_seen: opt(row, first),
            last_seen: opt(row, last),
        });
    }
    Ok(out)
}
