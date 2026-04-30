pub mod footnotes;
pub mod markdown;
pub mod toc;
pub mod wikilinks;

use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Mutex;
use anyhow::Result;

pub use markdown::render;

#[derive(Debug, Clone, Serialize)]
pub struct RenderedArticle {
    pub meta: ArticleMeta,
    pub body_html: String,
    pub toc: Vec<toc::TocEntry>,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArticleMeta {
    pub title: String,
    pub slug: String,
    pub category: String,
    #[serde(default)]
    pub subcategory: Option<String>,
    pub last_edited: Option<String>,
    pub editor: Option<String>,
    #[serde(default)]
    pub status: ArticleStatus,
    #[serde(default)]
    pub references: Vec<ReferenceDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ArticleStatus {
    #[default]
    Stable,
    PreBuild,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceDef {
    pub id: u32,
    pub text: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub internal: bool,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Reference {
    pub number: u32,
    pub text: String,
    pub url: Option<String>,
    pub internal: bool,
    /// Anchor id for back-arrow navigation from bibliography → body citation.
    pub anchor: String,
}

/// LRU render cache. Key: (slug, git_head_sha).
/// Entries are stale once HEAD advances — the sync daemon calls invalidate_all().
pub struct Cache {
    inner: Mutex<LruCache<(String, String), RenderedArticle>>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(256).unwrap());
        Self { inner: Mutex::new(LruCache::new(cap)) }
    }

    pub fn get(&self, slug: &str, head_sha: &str) -> Option<RenderedArticle> {
        self.inner.lock().unwrap()
            .get(&(slug.to_string(), head_sha.to_string()))
            .cloned()
    }

    pub fn insert(&self, slug: &str, head_sha: &str, article: RenderedArticle) {
        self.inner.lock().unwrap()
            .put((slug.to_string(), head_sha.to_string()), article);
    }

    pub fn invalidate_all(&self) {
        self.inner.lock().unwrap().clear();
    }
}

pub fn render_file(path: &Path, page_index: &wikilinks::PageIndex) -> Result<RenderedArticle> {
    let raw = std::fs::read_to_string(path)?;
    markdown::render(&raw, page_index)
}
