//! Phase 5: pending-edit review queue — storage, query, and HTTP handlers.
//!
//! Routes served by this module:
//!   GET  /special/pending-changes         — admin review queue (list)
//!   GET  /special/pending/{id}            — review detail + diff
//!   POST /special/pending/{id}/accept     — accept and publish
//!   POST /special/pending/{id}/reject     — reject with optional note
//!   GET  /special/contributions/{username} — editor's submission history

use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use maud::{html, Markup, DOCTYPE};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use uuid::Uuid;

use crate::auth::{AdminUser, LoggedInUser};
use crate::server::AppState;

// ─────────────────────────────────────────────────────────────────────
// Data model
// ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PendingEdit {
    pub id: String,
    pub slug: String,
    pub author_id: String,
    pub author_username: String,
    pub body: String,
    pub edit_summary: String,
    pub status: String,
    pub reviewer_note: Option<String>,
    pub submitted_at: i64,
    pub reviewed_at: Option<i64>,
    pub reviewed_by: Option<String>,
}

fn row_to_pending(row: &rusqlite::Row<'_>) -> rusqlite::Result<PendingEdit> {
    Ok(PendingEdit {
        id: row.get(0)?,
        slug: row.get(1)?,
        author_id: row.get(2)?,
        author_username: row.get(3)?,
        body: row.get(4)?,
        edit_summary: row.get(5)?,
        status: row.get(6)?,
        reviewer_note: row.get(7)?,
        submitted_at: row.get(8)?,
        reviewed_at: row.get(9)?,
        reviewed_by: row.get(10)?,
    })
}

const SELECT_PENDING: &str =
    "SELECT pe.id, pe.slug, pe.author_id, u.username, pe.body, pe.edit_summary,
            pe.status, pe.reviewer_note, pe.submitted_at, pe.reviewed_at, pe.reviewed_by
     FROM pending_edits pe JOIN users u ON u.id = pe.author_id";

fn fmt_ts(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|| ts.to_string())
}

// ─────────────────────────────────────────────────────────────────────
// DB operations (sync — call via spawn_blocking)
// ─────────────────────────────────────────────────────────────────────

pub fn insert_pending(
    conn: &Connection,
    slug: &str,
    author_id: &str,
    body: &str,
    edit_summary: &str,
) -> rusqlite::Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO pending_edits (id, slug, author_id, body, edit_summary, status, submitted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 'pending', ?6)",
        params![id, slug, author_id, body, edit_summary, now],
    )?;
    Ok(id)
}

pub fn list_pending(conn: &Connection) -> rusqlite::Result<Vec<PendingEdit>> {
    let query = format!(
        "{} WHERE pe.status = 'pending' ORDER BY pe.submitted_at ASC",
        SELECT_PENDING
    );
    let mut stmt = conn.prepare(&query)?;
    let edits = stmt
        .query_map([], row_to_pending)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(edits)
}

pub fn count_pending(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM pending_edits WHERE status = 'pending'",
        [],
        |row| row.get(0),
    )
}

pub fn get_pending_by_id(conn: &Connection, id: &str) -> rusqlite::Result<Option<PendingEdit>> {
    let query = format!("{} WHERE pe.id = ?1", SELECT_PENDING);
    conn.query_row(&query, params![id], row_to_pending)
        .optional()
}

pub fn list_by_author(conn: &Connection, author_id: &str) -> rusqlite::Result<Vec<PendingEdit>> {
    let query = format!(
        "{} WHERE pe.author_id = ?1 ORDER BY pe.submitted_at DESC",
        SELECT_PENDING
    );
    let mut stmt = conn.prepare(&query)?;
    let edits = stmt
        .query_map(params![author_id], row_to_pending)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(edits)
}

pub fn mark_accepted(conn: &Connection, id: &str, reviewer_id: &str) -> rusqlite::Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE pending_edits SET status='accepted', reviewed_at=?1, reviewed_by=?2 WHERE id=?3",
        params![now, reviewer_id, id],
    )?;
    Ok(())
}

pub fn mark_rejected(
    conn: &Connection,
    id: &str,
    reviewer_id: &str,
    note: &str,
) -> rusqlite::Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE pending_edits SET status='rejected', reviewed_at=?1, reviewed_by=?2, reviewer_note=?3 WHERE id=?4",
        params![now, reviewer_id, note, id],
    )?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// Diff rendering
// ─────────────────────────────────────────────────────────────────────

fn render_diff(old: &str, new_content: &str) -> Markup {
    let diff = TextDiff::from_lines(old, new_content);
    html! {
        div.pending-diff {
            @for group in diff.grouped_ops(3) {
                div.diff-group {
                    @for op in &group {
                        @for change in diff.iter_changes(op) {
                            @let line = change.value().trim_end_matches('\n');
                            @match change.tag() {
                                ChangeTag::Delete => div.diff-del { "−\u{a0}" (line) }
                                ChangeTag::Insert => div.diff-ins { "+\u{a0}" (line) }
                                ChangeTag::Equal  => div.diff-eq  { "\u{a0}\u{a0}" (line) }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Page shell (reused across all pending pages)
// ─────────────────────────────────────────────────────────────────────

fn pending_chrome(title: &str, site_title: &str, username: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " — " (site_title) }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.site-header {
                    a.site-title href="/" { (site_title) }
                    nav.site-nav {
                        a href="/" { "Home" }
                        " · "
                        a href="/special/pending-changes" { "Pending changes" }
                        " · "
                        span { (username) }
                        " · "
                        form method="post" action="/special/logout" style="display:inline;" {
                            button.nav-logout-btn type="submit" { "Log out" }
                        }
                    }
                }
                main style="max-width:960px;margin:2rem auto;padding:0 1.5rem;" {
                    (content)
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// HTTP handlers
// ─────────────────────────────────────────────────────────────────────

pub async fn review_queue(
    State(state): State<Arc<AppState>>,
    AdminUser(admin): AdminUser,
) -> Markup {
    let edits = if let Some(db) = state.db.as_ref() {
        let db = db.clone();
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            list_pending(&conn)
        })
        .await
        .unwrap_or(Ok(vec![]))
        .unwrap_or_default()
    } else {
        vec![]
    };

    let content = html! {
        h1 { "Pending changes (" (edits.len()) ")" }
        @if edits.is_empty() {
            p { "No pending edits awaiting review." }
        } @else {
            table.pending-table {
                thead {
                    tr {
                        th { "Article" }
                        th { "Author" }
                        th { "Submitted" }
                        th { "Summary" }
                        th {}
                    }
                }
                tbody {
                    @for e in &edits {
                        tr {
                            td { a href={ "/wiki/" (e.slug) } { (e.slug) } }
                            td { (e.author_username) }
                            td { (fmt_ts(e.submitted_at)) }
                            td.pending-summary { (e.edit_summary) }
                            td {
                                a.pending-review-link
                                    href={ "/special/pending/" (e.id) }
                                { "Review →" }
                            }
                        }
                    }
                }
            }
        }
    };
    pending_chrome("Pending changes", &state.site_title, &admin.username, content)
}

pub async fn review_detail(
    State(state): State<Arc<AppState>>,
    AdminUser(admin): AdminUser,
    Path(id): Path<String>,
) -> Response {
    let db = match state.db.as_ref() {
        Some(db) => db.clone(),
        None => return (StatusCode::NOT_FOUND, "Pending edit not found.").into_response(),
    };
    let id_c = id.clone();
    let edit = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        get_pending_by_id(&conn, &id_c)
    })
    .await
    .unwrap_or(Ok(None))
    .unwrap_or(None);

    let Some(edit) = edit else {
        return (StatusCode::NOT_FOUND, "Pending edit not found.").into_response();
    };

    let current_path = state.content_dir.join(format!("{}.md", edit.slug));
    let current = tokio::fs::read_to_string(&current_path)
        .await
        .unwrap_or_default();

    let diff_markup = render_diff(&current, &edit.body);
    let edit_id = edit.id.clone();

    let content = html! {
        h1 {
            "Review: "
            a href={ "/wiki/" (edit.slug) } { (edit.slug) }
        }
        p.pending-meta {
            "Submitted by "
            strong { (edit.author_username) }
            " · "
            (fmt_ts(edit.submitted_at))
        }
        @if !edit.edit_summary.is_empty() {
            p.pending-edit-summary {
                "Edit summary: "
                em { "\"" (edit.edit_summary) "\"" }
            }
        }

        h2 { "Changes" }
        (diff_markup)

        div.pending-review-forms {
            // Reject form (with optional reviewer note)
            form.pending-form method="post" action={ "/special/pending/" (edit_id) "/reject" } {
                div.pending-field {
                    label for="reviewer_note" { "Reviewer note (optional):" }
                    input #reviewer_note
                        type="text"
                        name="reviewer_note"
                        placeholder="Reason for rejection…";
                }
                button.pending-btn-reject type="submit" { "Reject" }
            }
            // Accept form (separate POST)
            form.pending-form method="post" action={ "/special/pending/" (edit_id) "/accept" } style="display:inline;" {
                button.pending-btn-accept
                    type="submit"
                    onclick="return confirm('Accept and publish this edit?')"
                { "Accept and publish" }
            }
        }
    };

    pending_chrome(
        &format!("Review: {}", edit.slug),
        &state.site_title,
        &admin.username,
        content,
    )
    .into_response()
}

pub async fn accept_edit(
    State(state): State<Arc<AppState>>,
    AdminUser(admin): AdminUser,
    Path(id): Path<String>,
) -> Response {
    let db = match state.db.as_ref() {
        Some(db) => db.clone(),
        None => return Redirect::to("/special/pending-changes").into_response(),
    };
    let id_c = id.clone();
    let edit = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        get_pending_by_id(&conn, &id_c)
    })
    .await
    .unwrap_or(Ok(None))
    .unwrap_or(None);

    let Some(edit) = edit else {
        return Redirect::to("/special/pending-changes").into_response();
    };

    let target = state.content_dir.join(format!("{}.md", edit.slug));
    if let Err(e) = crate::edit::atomic_write(&state.content_dir, &target, &edit.body).await {
        tracing::warn!(id = %id, error = %e, "accept_edit: disk write failed");
        return Redirect::to(&format!("/special/pending/{}?error=write", id)).into_response();
    }

    {
        match state.git.lock() {
            Ok(git_repo) => {
                let _ = crate::git::ensure_commit_identity_from_env(&git_repo);
                let msg = format!(
                    "accept: {}\n\nCo-Authored-By: {} <editor@wiki>",
                    edit.slug, edit.author_username
                );
                match crate::git::commit_topic(&git_repo, &edit.slug, &edit.body, "", "", &msg) {
                    Ok(_) => tracing::info!(slug = %edit.slug, "committed accepted edit"),
                    Err(e) => tracing::warn!(slug = %edit.slug, error = %e, "git commit failed"),
                }
            }
            Err(e) => tracing::warn!(error = %e, "git lock failed after accept"),
        }
    }

    if let Err(e) = crate::search::reindex_topic(&state.search, &edit.slug, &edit.body) {
        tracing::warn!(slug = %edit.slug, error = %e, "reindex failed after accept");
    }

    if let Some(db) = state.db.as_ref() {
        let db = db.clone();
        let admin_id = admin.id.clone();
        let _ = tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            mark_accepted(&conn, &id, &admin_id)
        })
        .await;
    }

    Redirect::to("/special/pending-changes").into_response()
}

#[derive(Deserialize)]
pub struct RejectForm {
    pub reviewer_note: Option<String>,
}

pub async fn reject_edit(
    State(state): State<Arc<AppState>>,
    AdminUser(admin): AdminUser,
    Path(id): Path<String>,
    Form(form): Form<RejectForm>,
) -> Response {
    let db = match state.db.as_ref() {
        Some(db) => db.clone(),
        None => return Redirect::to("/special/pending-changes").into_response(),
    };
    let admin_id = admin.id.clone();
    let note = form.reviewer_note.unwrap_or_default();
    let _ = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        mark_rejected(&conn, &id, &admin_id, &note)
    })
    .await;

    Redirect::to("/special/pending-changes").into_response()
}

pub async fn contributions(
    State(state): State<Arc<AppState>>,
    LoggedInUser(user): LoggedInUser,
    Path(username): Path<String>,
) -> Response {
    if !user.is_admin() && user.username != username {
        return (StatusCode::FORBIDDEN, "You can only view your own contributions.").into_response();
    }

    let edits = if let Some(db) = state.db.as_ref() {
        let db = db.clone();
        let author_id = user.id.clone();
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            list_by_author(&conn, &author_id)
        })
        .await
        .unwrap_or(Ok(vec![]))
        .unwrap_or_default()
    } else {
        vec![]
    };

    let content = html! {
        h1 { "Contributions by " (username) }
        @if edits.is_empty() {
            p { "No edits submitted yet." }
        } @else {
            table.pending-table {
                thead {
                    tr {
                        th { "Article" }
                        th { "Status" }
                        th { "Submitted" }
                        th { "Summary" }
                    }
                }
                tbody {
                    @for e in &edits {
                        tr {
                            td { a href={ "/wiki/" (e.slug) } { (e.slug) } }
                            td {
                                span class={ "pending-status-" (e.status) } { (e.status) }
                            }
                            td { (fmt_ts(e.submitted_at)) }
                            td.pending-summary { (e.edit_summary) }
                        }
                    }
                }
            }
        }
    };

    pending_chrome(
        &format!("Contributions: {}", username),
        &state.site_title,
        &user.username,
        content,
    )
    .into_response()
}
