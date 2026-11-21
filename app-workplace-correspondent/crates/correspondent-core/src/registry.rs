// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use crate::Error;
use std::collections::HashSet;
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
        status.as_deref().map(str::to_lowercase).as_deref(),
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
}

impl Registry {
    /// Load `people_dir/index.csv` and `business_dir/index.csv`.
    pub fn open(people_dir: &Path, business_dir: &Path) -> Result<Self, Error> {
        Ok(Self {
            people: load_people(&people_dir.join("index.csv"))?,
            businesses: load_businesses(&business_dir.join("index.csv"))?,
        })
    }

    pub fn people(&self) -> &[Person] {
        &self.people
    }

    pub fn businesses(&self) -> &[Business] {
        &self.businesses
    }

    pub fn person(&self, id: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.person_id == id)
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
        let mut cur = self.businesses.iter().find(|b| b.business_id == id)?;
        for _ in 0..=self.businesses.len() {
            match cur.merged_into.as_deref() {
                Some(next) if !cur.is_active() => {
                    cur = self.businesses.iter().find(|b| b.business_id == next)?
                }
                _ => return Some(cur),
            }
        }
        None
    }

    /// Case-insensitive substring search on display name, sorted by name.
    /// Merged and retired rows are hidden. An empty query returns every
    /// active person.
    pub fn search_people(&self, query: &str) -> Vec<&Person> {
        let q = query.to_lowercase();
        let mut hits: Vec<&Person> = self
            .people
            .iter()
            .filter(|p| p.is_active() && p.display_name.to_lowercase().contains(&q))
            .collect();
        hits.sort_by_key(|p| p.display_name.to_lowercase());
        hits
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

struct Table {
    file: PathBuf,
    headers: csv::StringRecord,
    rows: Vec<csv::StringRecord>,
}

impl Table {
    fn read(file: &Path) -> Result<Self, Error> {
        if !file.is_file() {
            return Err(Error::MissingFile(file.to_path_buf()));
        }
        let csv_err = |source| Error::Csv {
            file: file.to_path_buf(),
            source,
        };
        let mut rdr = csv::Reader::from_path(file).map_err(csv_err)?;
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

    fn column(&self, name: &str) -> Option<usize> {
        self.headers.iter().position(|h| h == name)
    }

    fn required(&self, name: &'static str) -> Result<usize, Error> {
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

fn opt(row: &csv::StringRecord, idx: Option<usize>) -> Option<String> {
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
