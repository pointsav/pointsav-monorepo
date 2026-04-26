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
use maud::{html, Markup, DOCTYPE};
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
            'a'..='z' | '0'..='9' | '.' | '-' | '_' => {}
            _ => return Err(WikiError::SlugInvalid(slug.into())),
        }
    }
    Ok(())
}

/// Editor HTML page. Phase 2 Step 2 returns a minimal placeholder confirming
/// routing + slug validation. Step 3 replaces the body with the CodeMirror 6
/// surface from the vendored bundle.
pub async fn get_edit(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    validate_slug(&slug)?;
    let path = state.content_dir.join(format!("{slug}.md"));
    let exists = path.is_file();
    let existing = if exists {
        tokio::fs::read_to_string(&path).await.unwrap_or_default()
    } else {
        String::new()
    };
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
                main.editor-placeholder {
                    h1 { "Edit " (slug) }
                    p {
                        "Phase 2 Step 2 placeholder. CodeMirror 6 surface "
                        "lands in Step 3."
                    }
                    @if exists {
                        p { "File exists; current size: " (existing.len()) " bytes." }
                    } @else {
                        p { "File does not exist. POST /create to add a new TOPIC." }
                    }
                    p { a href={ "/wiki/" (slug) } { "← back to article" } }
                }
            }
        }
    })
}

/// Atomic write of edited TOPIC body to an existing file. Returns 404 if the
/// target does not exist (use `/create` for new TOPICs — separate route to
/// avoid accidental creation via PUT-shaped POST).
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
    }

    #[test]
    fn slug_validation_rejects_dirty_slugs() {
        assert!(validate_slug("").is_err());
        assert!(validate_slug("Foo Bar").is_err());
        assert!(validate_slug("../etc/passwd").is_err());
        assert!(validate_slug("foo/bar").is_err());
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
