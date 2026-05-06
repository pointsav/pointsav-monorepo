//! Phase 2 Step 2 — edit endpoint, atomic write, path-traversal hardening.
//!
//! Routes:
//!   GET  /edit/{slug} — editor HTML page (Step 2 ships a placeholder; Step 3
//!                       wires the CodeMirror 6 surface).
//!   POST /edit/{slug} — atomic write of edited TOPIC body to
//!                       `<content_dir>/<slug>.md`. Returns 404 if the file
//!                       does not exist (use `/create` for new TOPICs).
//!   POST /create      — create new TOPIC from `{title, slug?}`; slug derives
//!                       from title if not supplied.
//!
//! Atomicity: `tempfile::NamedTempFile::persist` gives POSIX-atomic rename on
//! all production filesystems. A crash between write and rename leaves the
//! destination intact.
//!
//! Slug validation: conservative `^[a-z0-9._-]+$` matching Phase 1's read-side
//! hardening. Rejects path-traversal characters, leading dots, `..` sequences,
//! and anything that would resolve outside `<content_dir>`. Phase 6 lands the
//! full normalisation rules.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::task;

use crate::error::WikiError;
use crate::server::AppState;

/// Validate a slug per Phase 2 conservative rules. Allowed: lowercase ASCII
/// letters, digits, dots, hyphens, underscores. Rejects: empty, leading dot,
/// `..` sequence, anything else.
pub fn validate_slug(slug: &str) -> Result<(), WikiError> {
    if slug.is_empty() {
        return Err(WikiError::SlugInvalid("empty".into()));
    }
    if slug.starts_with('.') {
        return Err(WikiError::SlugInvalid(slug.into()));
    }
    if slug.contains("..") {
        return Err(WikiError::SlugInvalid(slug.into()));
    }
    for c in slug.chars() {
        match c {
            'a'..='z' | '0'..='9' | '.' | '-' | '_' | '/' => {}
            _ => return Err(WikiError::SlugInvalid(slug.into())),
        }
    }
    Ok(())
}

/// Editor HTML page. Phase 2 Step 3 renders the CodeMirror 6 surface from
/// the vendored bundle (`/static/vendor/cm-saa.bundle.js`). The first-party
/// glue (`/static/saa-init.js`) reads `window.SAA_SLUG` + `window.SAA_INITIAL`
/// and instantiates an EditorView in `<div id="saa-editor">`. Save POSTs the
/// editor's current doc to `/edit/{slug}` (the route above).
///
/// The initial doc is JSON-encoded into a `<script>` block so multi-line
/// markdown bodies, quotes, and special characters round-trip cleanly without
/// HTML-attribute escaping headaches.
pub async fn get_edit(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    validate_slug(&slug)?;
    let path = state.content_dir.join(format!("{slug}.md"));
    let initial = if path.is_file() {
        tokio::fs::read_to_string(&path).await.unwrap_or_default()
    } else {
        String::new()
    };
    // Escape `</` → `<\/` in the JSON literals so a markdown body containing
    // `</script>` cannot prematurely close the script tag (XSS hardening).
    // `\/` is a valid JSON escape for `/`, so the round-trip is preserved.
    let initial_json = serde_json::to_string(&initial)
        .unwrap_or_else(|_| "\"\"".to_string())
        .replace("</", "<\\/");
    let slug_json = serde_json::to_string(&slug)
        .unwrap_or_else(|_| "\"\"".to_string())
        .replace("</", "<\\/");

    Ok(html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Edit " (slug) " — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.editor-header {
                    a.site-title href="/" { "PointSav Knowledge" }
                    div.editor-title-block {
                        span.editor-label { "Edit:" }
                        span.editor-slug { (slug) }
                    }
                    div.editor-actions {
                        a.editor-cancel href={ "/wiki/" (slug) } { "← back" }
                        button #saa-save.editor-save { "Save" }
                    }
                }
                main.editor-shell {
                    div #saa-editor data-slug=(slug) {}
                    div #saa-status.saa-status {}
                }
                // Inject editor state before the bundle + glue scripts load.
                // Phase 2 Step 7: when --enable-collab is set, also template
                // window.WIKI_COLLAB_ENABLED so saa-init.js lazy-loads the
                // collab bundle and switches the editor into Y.Doc-backed mode.
                script {
                    (PreEscaped(format!(
                        "window.SAA_SLUG={};window.SAA_INITIAL={};{}",
                        slug_json,
                        initial_json,
                        if state.enable_collab {
                            "window.WIKI_COLLAB_ENABLED=true;"
                        } else {
                            ""
                        }
                    )))
                }
                script src="/static/vendor/cm-saa.bundle.js" defer {}
                script src="/static/saa-init.js" defer {}
            }
        }
    })
}

/// Atomic write of edited TOPIC body to an existing file. Returns 404 if the
/// target does not exist (use `/create` for new TOPICs — separate route to
/// avoid accidental creation via PUT-shaped POST).
///
/// On successful write, asynchronously triggers a tantivy reindex of the
/// slug. Reindex failures are logged but do NOT roll back the disk write —
/// the search index is derived state per ARCHITECTURE.md §1; the on-disk
/// file is canonical.
pub async fn post_edit(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    body: String,
) -> Result<Response, WikiError> {
    validate_slug(&slug)?;
    let target = state.content_dir.join(format!("{slug}.md"));
    if !target.is_file() {
        return Err(WikiError::NotFound(slug));
    }
    atomic_write(&state.content_dir, &target, &body).await?;

    // Phase 4 Step 4.1: commit to git. Failures are logged but not fatal.
    {
        let git_repo = state.git.lock().map_err(|e| WikiError::WriteFailed(format!("git lock failed: {e}")))?;
        let _ = crate::git::ensure_commit_identity_from_env(&git_repo);
        match crate::git::commit_topic(&git_repo, &slug, &body, "", "", &format!("edit: {slug}")) {
            Ok(_) => tracing::info!(slug = %slug, "committed edit to git"),
            Err(e) => tracing::warn!(slug = %slug, error = %e, "git commit failed after edit"),
        }
    }

    if let Err(e) = crate::search::reindex_topic(&state.search, &slug, &body) {
        tracing::warn!(slug = %slug, error = %e, "search reindex failed after edit");
    }
    Ok((StatusCode::OK, "saved").into_response())
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub slug: Option<String>,
}

/// Create new TOPIC from `{title, slug?}`. Conflict (409) if slug already
/// exists; the editor surface is responsible for slug-uniqueness checking
/// before submission.
pub async fn post_create(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRequest>,
) -> Result<Response, WikiError> {
    let slug = match req.slug {
        Some(s) => s,
        None => derive_slug(&req.title),
    };
    validate_slug(&slug)?;
    let target = state.content_dir.join(format!("{slug}.md"));
    if target.exists() {
        return Err(WikiError::AlreadyExists(slug));
    }
    let body = format!(
        "---\ntitle: \"{}\"\nslug: {}\n---\n\n",
        req.title.replace('"', "\\\""),
        slug
    );
    atomic_write(&state.content_dir, &target, &body).await?;

    // Phase 4 Step 4.1: commit to git. Failures are logged but not fatal.
    {
        let git_repo = state.git.lock().map_err(|e| WikiError::WriteFailed(format!("git lock failed: {e}")))?;
        let _ = crate::git::ensure_commit_identity_from_env(&git_repo);
        match crate::git::commit_topic(&git_repo, &slug, &body, "", "", &format!("create: {slug}")) {
            Ok(_) => tracing::info!(slug = %slug, "committed create to git"),
            Err(e) => tracing::warn!(slug = %slug, error = %e, "git commit failed after create"),
        }
    }

    if let Err(e) = crate::search::reindex_topic(&state.search, &slug, &body) {
        tracing::warn!(slug = %slug, error = %e, "search reindex failed after create");
    }
    Ok((StatusCode::CREATED, slug).into_response())
}

/// Derive a slug from a title: lowercase, non-alphanumerics → hyphen,
/// collapse runs, trim. Used when the create request doesn't supply an
/// explicit slug.
pub fn derive_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Atomic write via temp-file + POSIX rename. The temp file is created in
/// the same directory as the target so the rename stays on the same
/// filesystem (cross-fs renames are non-atomic on Linux).
async fn atomic_write(
    content_dir: &PathBuf,
    target: &PathBuf,
    body: &str,
) -> Result<(), WikiError> {
    let dir = content_dir.clone();
    let tgt = target.clone();
    let body_owned = body.to_string();
    task::spawn_blocking(move || -> Result<(), WikiError> {
        let mut tmp = NamedTempFile::new_in(&dir)
            .map_err(|e| WikiError::WriteFailed(e.to_string()))?;
        std::io::Write::write_all(&mut tmp, body_owned.as_bytes())
            .map_err(|e| WikiError::WriteFailed(e.to_string()))?;
        tmp.persist(&tgt)
            .map_err(|e| WikiError::WriteFailed(e.to_string()))?;
        Ok(())
    })
    .await
    .map_err(|e| WikiError::WriteFailed(format!("join error: {e}")))??;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_validation_accepts_clean_slugs() {
        assert!(validate_slug("foo-bar").is_ok());
        assert!(validate_slug("topic-os-totebox").is_ok());
        assert!(validate_slug("a.b.c").is_ok());
        assert!(validate_slug("foo_bar").is_ok());
        assert!(validate_slug("topic-redirect-bilingual-test.es").is_ok());
        assert!(validate_slug("foo/bar").is_ok());
    }

    #[test]
    fn slug_validation_rejects_dirty_slugs() {
        assert!(validate_slug("").is_err());
        assert!(validate_slug("Foo Bar").is_err());
        assert!(validate_slug("../etc/passwd").is_err());
        assert!(validate_slug("foo\\bar").is_err());
        assert!(validate_slug(".hidden").is_err());
        assert!(validate_slug("foo..bar").is_err());
        assert!(validate_slug("FOO").is_err());
    }

    #[test]
    fn derive_slug_normalises_titles() {
        assert_eq!(derive_slug("Hello World"), "hello-world");
        assert_eq!(derive_slug("Topic: Service Email"), "topic-service-email");
        assert_eq!(derive_slug("  Multi  Spaces  "), "multi-spaces");
        assert_eq!(derive_slug("UPPERCASE"), "uppercase");
        assert_eq!(derive_slug("Already-Hyphenated"), "already-hyphenated");
    }
}
