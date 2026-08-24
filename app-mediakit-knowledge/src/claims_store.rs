// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Persistent claim graph + citation-verification metadata (Phase 3.3/3.4 of
//! `KNOWLEDGE-PLATFORM-PLAN.md`), backed by `redb`.
//!
//! Markdown remains the single source of truth for claim *text* — this store
//! holds derived, queryable state: the full claim set (rebuilt from a content
//! reindex, same lifecycle as the search index in `content::walk`), the
//! `cited_by` inverse index (a citation id → every claim that cites it), and
//! citation re-verification history, so both survive process restarts without
//! re-walking every file on every query.

use std::path::Path;
use std::sync::Arc;

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::content::Claim;

const CLAIMS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("claims");
const CITED_BY_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("cited_by");
const VERIFICATION_TABLE: TableDefinition<&str, &[u8]> =
    TableDefinition::new("citation_verification");

/// Result of the most recent re-fetch/re-hash check of one `citations.yaml`
/// entry (Phase 3.4 — continuous citation verification).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationVerification {
    pub citation_id: String,
    pub url: String,
    /// ISO 8601 UTC timestamp of the check.
    pub last_checked: String,
    /// blake3 of the fetched body; `None` if the fetch itself failed.
    pub content_hash: Option<String>,
    pub status: VerificationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// Hash matches the previously-recorded hash (or this is the first check).
    Ok,
    /// Hash changed since the last check — the cited source drifted.
    Drifted,
    /// The URL could not be fetched (network error, non-2xx, timeout).
    Unreachable,
}

/// The claim-graph store. Cheap to clone (wraps an `Arc<Database>`); safe to
/// share across the axum `AppState` and the background verification task.
#[derive(Clone)]
pub struct ClaimStore {
    db: Arc<Database>,
}

impl ClaimStore {
    /// Open (creating if absent) the redb file at `path`.
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let db = Database::create(path)?;
        // Touch every table once so later opens-for-read never hit "table not found".
        let write = db.begin_write()?;
        {
            write.open_table(CLAIMS_TABLE)?;
            write.open_table(CITED_BY_TABLE)?;
            write.open_table(VERIFICATION_TABLE)?;
        }
        write.commit()?;
        Ok(Self { db: Arc::new(db) })
    }

    /// Replace this topic's claim set (called whenever the topic is
    /// (re)indexed). Stale `cited_by` entries from a prior version of the
    /// same topic are not pruned here — the inverse index is a monotonic
    /// membership list; a full-corpus `rebuild_cited_by` (below) is the
    /// correct way to fully reconcile it after bulk content changes.
    pub fn upsert_topic_claims(&self, claims: &[Claim]) -> anyhow::Result<()> {
        let write = self.db.begin_write()?;
        {
            let mut table = write.open_table(CLAIMS_TABLE)?;
            for c in claims {
                let bytes = serde_json::to_vec(c)?;
                table.insert(c.global_id().as_str(), bytes.as_slice())?;
            }
        }
        {
            let mut cited_by = write.open_table(CITED_BY_TABLE)?;
            for c in claims {
                for citation_id in &c.cites {
                    let mut list: Vec<String> = cited_by
                        .get(citation_id.as_str())?
                        .map(|v| serde_json::from_slice(v.value()))
                        .transpose()?
                        .unwrap_or_default();
                    if !list.contains(&c.global_id()) {
                        list.push(c.global_id());
                        let bytes = serde_json::to_vec(&list)?;
                        cited_by.insert(citation_id.as_str(), bytes.as_slice())?;
                    }
                }
            }
        }
        write.commit()?;
        Ok(())
    }

    /// Rebuild the `cited_by` inverse index from scratch against the current
    /// full claim set. Call after a full-corpus reindex so removed/renamed
    /// citations don't leave stale entries behind — this actually prunes
    /// every existing key absent from the freshly-computed index (a prior
    /// version of this function only ever inserted, never removed, so a
    /// citation that stopped being cited kept its stale `cited_by` entry
    /// forever; fixed 2026-07-13 per audit finding).
    pub fn rebuild_cited_by(&self) -> anyhow::Result<()> {
        let all = self.all_claims()?;
        let mut index: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        for c in &all {
            for citation_id in &c.cites {
                index
                    .entry(citation_id.clone())
                    .or_default()
                    .push(c.global_id());
            }
        }

        let write = self.db.begin_write()?;
        {
            let mut cited_by = write.open_table(CITED_BY_TABLE)?;
            let existing_keys: Vec<String> = cited_by
                .iter()?
                .map(|entry| entry.map(|(k, _)| k.value().to_string()))
                .collect::<Result<_, _>>()?;
            for key in &existing_keys {
                if !index.contains_key(key) {
                    cited_by.remove(key.as_str())?;
                }
            }
            for (citation_id, list) in &index {
                let bytes = serde_json::to_vec(list)?;
                cited_by.insert(citation_id.as_str(), bytes.as_slice())?;
            }
        }
        write.commit()?;
        Ok(())
    }

    pub fn get_claim(&self, global_id: &str) -> anyhow::Result<Option<Claim>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(CLAIMS_TABLE)?;
        Ok(table
            .get(global_id)?
            .map(|v| serde_json::from_slice(v.value()))
            .transpose()?)
    }

    /// Every claim's global id that cites `citation_id`.
    pub fn claims_citing(&self, citation_id: &str) -> anyhow::Result<Vec<String>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(CITED_BY_TABLE)?;
        Ok(table
            .get(citation_id)?
            .map(|v| serde_json::from_slice(v.value()))
            .transpose()?
            .unwrap_or_default())
    }

    pub fn all_claims(&self) -> anyhow::Result<Vec<Claim>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(CLAIMS_TABLE)?;
        let mut out = Vec::new();
        for entry in table.iter()? {
            let (_, v) = entry?;
            out.push(serde_json::from_slice(v.value())?);
        }
        Ok(out)
    }

    /// Every claim belonging to `topic_slug`.
    pub fn claims_for_topic(&self, topic_slug: &str) -> anyhow::Result<Vec<Claim>> {
        Ok(self
            .all_claims()?
            .into_iter()
            .filter(|c| c.topic_slug == topic_slug)
            .collect())
    }

    /// Every citation id currently referenced by at least one claim — the
    /// re-verification scheduler's work list.
    pub fn all_cited_citation_ids(&self) -> anyhow::Result<Vec<String>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(CITED_BY_TABLE)?;
        let mut out = Vec::new();
        for entry in table.iter()? {
            let (k, _) = entry?;
            out.push(k.value().to_string());
        }
        Ok(out)
    }

    pub fn record_verification(&self, v: &CitationVerification) -> anyhow::Result<()> {
        let write = self.db.begin_write()?;
        {
            let mut table = write.open_table(VERIFICATION_TABLE)?;
            let bytes = serde_json::to_vec(v)?;
            table.insert(v.citation_id.as_str(), bytes.as_slice())?;
        }
        write.commit()?;
        Ok(())
    }

    pub fn get_verification(
        &self,
        citation_id: &str,
    ) -> anyhow::Result<Option<CitationVerification>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(VERIFICATION_TABLE)?;
        Ok(table
            .get(citation_id)?
            .map(|v| serde_json::from_slice(v.value()))
            .transpose()?)
    }

    /// Every citation currently in `Drifted` state — surfaced in the citation
    /// ribbon (Phase 3.4).
    pub fn drifted_citations(&self) -> anyhow::Result<Vec<CitationVerification>> {
        let read = self.db.begin_read()?;
        let table = read.open_table(VERIFICATION_TABLE)?;
        let mut out = Vec::new();
        for entry in table.iter()? {
            let (_, v) = entry?;
            let cv: CitationVerification = serde_json::from_slice(v.value())?;
            if cv.status == VerificationStatus::Drifted {
                out.push(cv);
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Confidence;

    fn sample_claim(id: &str, topic: &str, cites: Vec<&str>) -> Claim {
        Claim {
            id: id.to_string(),
            topic_slug: topic.to_string(),
            cites: cites.into_iter().map(String::from).collect(),
            confidence: Confidence::Established,
            valid_at: None,
            depends_on: Vec::new(),
            text: "Sample claim text.".to_string(),
            content_hash: blake3::hash(b"Sample claim text.").to_hex().to_string(),
            published_at: None,
            revision_sha: None,
        }
    }

    #[test]
    fn upsert_and_get_claim_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let c = sample_claim("a", "topic-x", vec!["cite-1"]);
        store.upsert_topic_claims(std::slice::from_ref(&c)).unwrap();

        let got = store.get_claim("topic-x:a").unwrap().unwrap();
        assert_eq!(got.id, "a");
        assert_eq!(got.cites, vec!["cite-1"]);
    }

    #[test]
    fn cited_by_index_is_populated_on_upsert() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let a = sample_claim("a", "topic-x", vec!["shared-cite"]);
        let b = sample_claim("b", "topic-y", vec!["shared-cite"]);
        store.upsert_topic_claims(&[a, b]).unwrap();

        let mut citing = store.claims_citing("shared-cite").unwrap();
        citing.sort();
        assert_eq!(
            citing,
            vec!["topic-x:a".to_string(), "topic-y:b".to_string()]
        );
    }

    #[test]
    fn claims_for_topic_filters_correctly() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        store
            .upsert_topic_claims(&[
                sample_claim("a", "topic-x", vec![]),
                sample_claim("b", "topic-y", vec![]),
            ])
            .unwrap();

        let x_claims = store.claims_for_topic("topic-x").unwrap();
        assert_eq!(x_claims.len(), 1);
        assert_eq!(x_claims[0].id, "a");
    }

    #[test]
    fn verification_roundtrips_and_drift_query_works() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        let ok = CitationVerification {
            citation_id: "c1".into(),
            url: "https://example.com/a".into(),
            last_checked: "2026-07-13T00:00:00Z".into(),
            content_hash: Some("abc".into()),
            status: VerificationStatus::Ok,
        };
        let drifted = CitationVerification {
            citation_id: "c2".into(),
            url: "https://example.com/b".into(),
            last_checked: "2026-07-13T00:00:00Z".into(),
            content_hash: Some("def".into()),
            status: VerificationStatus::Drifted,
        };
        store.record_verification(&ok).unwrap();
        store.record_verification(&drifted).unwrap();

        assert_eq!(
            store.get_verification("c1").unwrap().unwrap().status,
            VerificationStatus::Ok
        );
        let drift_list = store.drifted_citations().unwrap();
        assert_eq!(drift_list.len(), 1);
        assert_eq!(drift_list[0].citation_id, "c2");
    }

    #[test]
    fn missing_claim_returns_none_not_error() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        assert!(store.get_claim("nope:nope").unwrap().is_none());
    }

    #[test]
    fn rebuild_cited_by_reflects_current_claim_set() {
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        store
            .upsert_topic_claims(&[sample_claim("a", "topic-x", vec!["c1"])])
            .unwrap();
        store.rebuild_cited_by().unwrap();
        assert_eq!(
            store.claims_citing("c1").unwrap(),
            vec!["topic-x:a".to_string()]
        );
    }

    #[test]
    fn rebuild_cited_by_prunes_orphaned_citation_keys() {
        // A citation id is cited, then the claim citing it is replaced by
        // one that no longer cites it (simulating an edited/deleted article
        // via upsert_topic_claims overwriting the same global_id). Before
        // the fix, claims_citing("stale-cite") would still return the old
        // claim forever.
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        store
            .upsert_topic_claims(&[sample_claim("a", "topic-x", vec!["stale-cite"])])
            .unwrap();
        store.rebuild_cited_by().unwrap();
        assert_eq!(
            store.claims_citing("stale-cite").unwrap(),
            vec!["topic-x:a".to_string()]
        );

        // Article re-edited: same claim id, no longer cites "stale-cite".
        store
            .upsert_topic_claims(&[sample_claim("a", "topic-x", vec![])])
            .unwrap();
        store.rebuild_cited_by().unwrap();
        assert!(
            store.claims_citing("stale-cite").unwrap().is_empty(),
            "orphaned citation key should be pruned, not left stale"
        );
    }
}
