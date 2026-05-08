//! Phase 3 Step 3.1 — Tantivy full-text search backend.
//!
//! Per ARCHITECTURE.md §3 Phase 3:
//! - On-disk tantivy index at `<state_dir>/search/`
//! - Index rebuilt on startup from a tree walk of `<content_dir>`
//! - Incremental reindex on edit (Phase 3 Step 3.2 wires the call from
//!   `edit::post_edit` and `edit::post_create`)
//! - `GET /search?q=` with fuzzy + BM25 (Step 3.2 mounts the route)
//!
//! Schema:
//! - `slug` — STRING + STORED (exact-match field for delete-by-slug)
//! - `title` — TEXT + STORED (full-text searchable; rendered in results)
//! - `body` — TEXT + STORED (full-text searchable; snippet generated from
//!   stored value)
//!
//! Concurrency:
//! - IndexReader is Sync — wrapped in Arc, shared across handlers
//! - IndexWriter is exclusive — wrapped in `Arc<Mutex<...>>` and
//!   acquired briefly on each reindex call. Only Step 3.2's `post_edit`
//!   and `post_create` paths take the lock.
//!
//! Tantivy is fully synchronous — all index ops run inside
//! `tokio::task::spawn_blocking` so the async runtime is never blocked.

use std::path::Path;
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Field, Schema, STORED, STRING, TEXT},
    Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument, Term,
};

use crate::error::WikiError;
use crate::render::parse_page;

const WRITER_HEAP_BYTES: usize = 50_000_000; // 50 MB; tantivy default floor

/// Handle to the search index — clone-cheap (everything inside is `Arc`).
#[derive(Clone)]
pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    writer: Arc<Mutex<IndexWriter>>,
    fields: SearchFields,
}

#[derive(Clone, Copy)]
struct SearchFields {
    slug: Field,
    title: Field,
    body: Field,
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchHit {
    pub slug: String,
    pub title: String,
    pub score: f32,
    pub snippet: String,
}

fn build_schema() -> (Schema, SearchFields) {
    let mut sb = Schema::builder();
    let slug = sb.add_text_field("slug", STRING | STORED);
    let title = sb.add_text_field("title", TEXT | STORED);
    let body = sb.add_text_field("body", TEXT | STORED);
    (sb.build(), SearchFields { slug, title, body })
}

/// Build (or open) the on-disk index, then walk `content_dir` and add every
/// non-bilingual `.md` file to the index. Returns a clone-cheap handle.
///
/// Bilingual sibling files (`*.es.md`) are skipped — searching them would
/// surface duplicate hits for the same TOPIC.
pub async fn build_index(
    content_dir: &Path,
    state_dir: &Path,
) -> Result<SearchIndex, WikiError> {
    let index_dir = state_dir.join("search");
    tokio::fs::create_dir_all(&index_dir).await?;

    let (schema, fields) = build_schema();

    // Read all topics first — async I/O off-task — then hand off the blocking
    // tantivy ops to spawn_blocking in one batch.
    let topics = collect_topics(content_dir).await?;

    let index_dir_cl = index_dir.clone();
    let topics_cl = topics;
    let (index, reader, writer) = tokio::task::spawn_blocking(move || -> Result<_, WikiError> {
        let dir = MmapDirectory::open(&index_dir_cl)
            .map_err(|e| WikiError::SearchFailed(format!("open index dir: {e}")))?;
        let index = Index::open_or_create(dir, schema)
            .map_err(|e| WikiError::SearchFailed(format!("open_or_create index: {e}")))?;

        let mut writer: IndexWriter = index
            .writer(WRITER_HEAP_BYTES)
            .map_err(|e| WikiError::SearchFailed(format!("writer init: {e}")))?;

        // Clear any prior state on startup — index always reflects the
        // current content tree.
        writer
            .delete_all_documents()
            .map_err(|e| WikiError::SearchFailed(format!("delete_all_documents: {e}")))?;

        for (slug, title, body) in &topics_cl {
            writer
                .add_document(doc!(
                    fields.slug => slug.clone(),
                    fields.title => title.clone(),
                    fields.body => body.clone(),
                ))
                .map_err(|e| WikiError::SearchFailed(format!("add_document {slug}: {e}")))?;
        }
        writer
            .commit()
            .map_err(|e| WikiError::SearchFailed(format!("initial commit: {e}")))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| WikiError::SearchFailed(format!("reader build: {e}")))?;

        Ok((index, reader, writer))
    })
    .await
    .map_err(|e| WikiError::SearchFailed(format!("spawn_blocking join: {e}")))??;

    Ok(SearchIndex {
        index,
        reader,
        writer: Arc::new(Mutex::new(writer)),
        fields,
    })
}

/// Walk `content_dir` recursively and collect `(slug, title, body)` triples
/// for every non-bilingual TOPIC. Descends one level into category subdirs.
/// Frontmatter is parsed; body is the markdown source after the delimiters.
/// Slugs for subdirectory files use `<category>/<stem>` form.
async fn collect_topics(content_dir: &Path) -> Result<Vec<(String, String, String)>, WikiError> {
    let mut out = Vec::new();
    let mut entries = tokio::fs::read_dir(content_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let path = entry.path();

        if file_type.is_dir() {
            // Descend into one-level subdirectory (category folder).
            let subdir_name = path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let mut sub_entries = match tokio::fs::read_dir(&path).await {
                Ok(e) => e,
                Err(_) => continue,
            };
            while let Some(sub_entry) = sub_entries.next_entry().await? {
                let sub_path = sub_entry.path();
                if sub_path.extension().and_then(|s| s.to_str()) != Some("md") {
                    continue;
                }
                let Some(stem) = sub_path.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                if stem.ends_with(".es") || stem == "index" || stem == "_index" || stem.starts_with('_') {
                    continue;
                }
                let slug = format!("{subdir_name}/{stem}");
                let text = match tokio::fs::read_to_string(&sub_path).await {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let parsed = match parse_page(&text) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let title = parsed.frontmatter.title.clone().unwrap_or_else(|| stem.to_string());
                out.push((slug, title, parsed.body_md));
            }
        } else {
            // File at root level.
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if stem.ends_with(".es") || stem == "index" || stem == "_index" || stem.starts_with('_') {
                continue;
            }
            let text = match tokio::fs::read_to_string(&path).await {
                Ok(t) => t,
                Err(_) => continue,
            };
            let parsed = match parse_page(&text) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let title = parsed.frontmatter.title.clone().unwrap_or_else(|| stem.to_string());
            out.push((stem.to_string(), title, parsed.body_md));
        }
    }
    Ok(out)
}

/// Run a search query and return the top-N hits, scored by BM25 against
/// the title + body fields. Snippets are a simple body-prefix (Phase 3
/// Step 3.1; Phase 3.2+ may upgrade to highlighter-based snippets).
pub fn search(idx: &SearchIndex, query_str: &str, limit: usize) -> Result<Vec<SearchHit>, WikiError> {
    if query_str.trim().is_empty() {
        return Ok(Vec::new());
    }

    let searcher = idx.reader.searcher();
    let parser = QueryParser::for_index(&idx.index, vec![idx.fields.title, idx.fields.body]);
    let query = parser
        .parse_query(query_str)
        .map_err(|e| WikiError::SearchFailed(format!("parse query: {e}")))?;
    let top = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| WikiError::SearchFailed(format!("search: {e}")))?;

    let mut hits = Vec::with_capacity(top.len());
    for (score, addr) in top {
        let doc: TantivyDocument = match searcher.doc(addr) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let slug = first_text_value(&doc, idx.fields.slug);
        let title = first_text_value(&doc, idx.fields.title);
        let body = first_text_value(&doc, idx.fields.body);
        hits.push(SearchHit {
            slug,
            title,
            score,
            snippet: snippet_from_body(&body),
        });
    }
    Ok(hits)
}

fn first_text_value(doc: &TantivyDocument, field: Field) -> String {
    use tantivy::schema::Value;
    doc.get_first(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn snippet_from_body(body: &str) -> String {
    // First non-empty paragraph, capped at ~180 chars. Phase 3.2 may
    // upgrade to query-aware snippet via tantivy's SnippetGenerator.
    let trimmed = body
        .lines()
        .find(|l| !l.trim().is_empty() && !l.trim_start().starts_with("---"))
        .unwrap_or("")
        .trim();
    if trimmed.chars().count() <= 180 {
        trimmed.to_string()
    } else {
        let mut out: String = trimmed.chars().take(180).collect();
        out.push_str("…");
        out
    }
}

/// Replace the indexed entry for `slug`, then commit. Called from Phase 3
/// Step 3.2's `post_edit` and `post_create` paths after the on-disk write
/// succeeds.
pub fn reindex_topic(idx: &SearchIndex, slug: &str, raw_text: &str) -> Result<(), WikiError> {
    let parsed = parse_page(raw_text)?;
    let title = parsed
        .frontmatter
        .title
        .clone()
        .unwrap_or_else(|| slug.to_string());
    let body = parsed.body_md;

    let mut writer = idx
        .writer
        .lock()
        .map_err(|e| WikiError::SearchFailed(format!("writer lock: {e}")))?;

    let term = Term::from_field_text(idx.fields.slug, slug);
    writer.delete_term(term);
    writer
        .add_document(doc!(
            idx.fields.slug => slug.to_string(),
            idx.fields.title => title,
            idx.fields.body => body,
        ))
        .map_err(|e| WikiError::SearchFailed(format!("add_document {slug}: {e}")))?;
    writer
        .commit()
        .map_err(|e| WikiError::SearchFailed(format!("reindex commit: {e}")))?;
    drop(writer);

    // Force the reader to pick up the new commit synchronously. Default
    // ReloadPolicy::OnCommitWithDelay batches reloads — searches issued
    // immediately after reindex would otherwise see pre-reindex state.
    idx.reader
        .reload()
        .map_err(|e| WikiError::SearchFailed(format!("reader reload: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn fixture_index() -> (SearchIndex, tempfile::TempDir, tempfile::TempDir) {
        let content_dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        for (slug, title, body) in &[
            ("topic-alpha", "Alpha Subject", "Alpha discusses the substrate. Substrate is the load-bearing concept."),
            ("topic-beta", "Beta Subject", "Beta covers continuous disclosure and the BCSC posture in detail."),
            ("topic-gamma", "Gamma Subject", "Gamma is unrelated content for control."),
        ] {
            tokio::fs::write(
                content_dir.path().join(format!("{slug}.md")),
                format!("---\ntitle: \"{title}\"\nslug: {slug}\n---\n{body}\n"),
            )
            .await
            .unwrap();
        }
        let index = build_index(content_dir.path(), state_dir.path()).await.unwrap();
        (index, content_dir, state_dir)
    }

    #[tokio::test]
    async fn empty_query_returns_no_hits() {
        let (index, _c, _s) = fixture_index().await;
        let hits = search(&index, "", 10).unwrap();
        assert!(hits.is_empty());
    }

    #[tokio::test]
    async fn matches_body_terms() {
        let (index, _c, _s) = fixture_index().await;
        let hits = search(&index, "substrate", 10).unwrap();
        assert!(!hits.is_empty());
        assert_eq!(hits[0].slug, "topic-alpha");
    }

    #[tokio::test]
    async fn matches_title_terms() {
        let (index, _c, _s) = fixture_index().await;
        let hits = search(&index, "Beta", 10).unwrap();
        assert!(!hits.is_empty());
        assert_eq!(hits[0].slug, "topic-beta");
    }

    #[tokio::test]
    async fn returns_no_hits_for_unrelated_query() {
        let (index, _c, _s) = fixture_index().await;
        let hits = search(&index, "xyzzy-no-such-term", 10).unwrap();
        assert!(hits.is_empty());
    }

    #[tokio::test]
    async fn snippet_is_first_paragraph_truncated() {
        let (index, _c, _s) = fixture_index().await;
        let hits = search(&index, "BCSC", 10).unwrap();
        assert!(!hits.is_empty());
        assert!(hits[0].snippet.contains("Beta covers"));
    }

    #[tokio::test]
    async fn skips_bilingual_siblings() {
        let content_dir = tempfile::tempdir().unwrap();
        let state_dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            content_dir.path().join("topic-bi.md"),
            "---\ntitle: English\nslug: topic-bi\n---\nEnglish body content.\n",
        )
        .await
        .unwrap();
        tokio::fs::write(
            content_dir.path().join("topic-bi.es.md"),
            "---\ntitle: Spanish\nslug: topic-bi.es\n---\nContenido en español.\n",
        )
        .await
        .unwrap();
        let index = build_index(content_dir.path(), state_dir.path()).await.unwrap();
        let hits = search(&index, "English", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].slug, "topic-bi");
        let hits_es = search(&index, "español", 10).unwrap();
        // The Spanish sibling was not indexed, so its body terms don't match.
        assert!(hits_es.is_empty());
    }

    #[tokio::test]
    async fn reindex_replaces_existing_entry() {
        let (index, _c, _s) = fixture_index().await;
        let new_body = "---\ntitle: \"Alpha v2\"\nslug: topic-alpha\n---\nReindex changed the body completely. New keywords: phoenix, eagle.\n";
        reindex_topic(&index, "topic-alpha", new_body).unwrap();
        // Old body had "substrate"; new body doesn't.
        let hits_old = search(&index, "substrate", 10).unwrap();
        assert!(
            hits_old.iter().all(|h| h.slug != "topic-alpha"),
            "old indexed body should be gone for topic-alpha"
        );
        let hits_new = search(&index, "phoenix", 10).unwrap();
        assert!(hits_new.iter().any(|h| h.slug == "topic-alpha"));
    }
}
