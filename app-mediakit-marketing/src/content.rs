//! Content loading. A page is a Git-tracked manifest at
//! `<content_dir>/<slug>/page.yaml`. Loading is validation: a manifest either
//! deserializes into the typed section vocabulary or it is an error.

use std::path::{Path, PathBuf};

use app_mediakit_shell::Page;

/// Reasons a page cannot be served.
#[derive(Debug)]
pub enum LoadError {
    /// Slug contained a path-traversal attempt.
    InvalidSlug,
    /// No manifest at the derived path.
    NotFound,
    /// Manifest present but does not conform to the section contract.
    Invalid(String),
}

/// Filesystem path of a slug's manifest.
pub fn page_path(content_dir: &Path, slug: &str) -> PathBuf {
    content_dir.join(slug).join("page.yaml")
}

/// Load and validate the manifest for `slug`. The home page is `slug = "home"`.
pub fn load_page(content_dir: &Path, slug: &str) -> Result<Page, LoadError> {
    if slug.is_empty() || slug.contains("..") || slug.starts_with('/') {
        return Err(LoadError::InvalidSlug);
    }
    let path = page_path(content_dir, slug);
    let text = std::fs::read_to_string(&path).map_err(|_| LoadError::NotFound)?;
    let mut page = Page::from_yaml(&text).map_err(LoadError::Invalid)?;
    if page.slug.is_none() {
        page.slug = Some(slug.to_string());
    }
    Ok(page)
}

/// List all slugs that have a `page.yaml` (one directory level deep).
pub fn list_slugs(content_dir: &Path) -> Vec<String> {
    let mut slugs = Vec::new();
    let Ok(entries) = std::fs::read_dir(content_dir) else {
        return slugs;
    };
    for entry in entries.flatten() {
        if entry.path().join("page.yaml").is_file() {
            if let Some(name) = entry.file_name().to_str() {
                slugs.push(name.to_string());
            }
        }
    }
    slugs.sort();
    slugs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_traversal() {
        let dir = Path::new("/tmp/does-not-matter");
        assert!(matches!(
            load_page(dir, "../etc"),
            Err(LoadError::InvalidSlug)
        ));
    }
}
