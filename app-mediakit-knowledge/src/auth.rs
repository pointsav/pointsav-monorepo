//! Phase 5: authentication handlers, cookie session, and request extractors.
//!
//! Sessions use a signed cookie (`wiki_session`) carrying a UUID token.
//! The token is looked up in the `sessions` SQLite table on every request
//! that needs the current user. No separate session middleware is needed —
//! the extractors do the DB lookup directly.
//!
//! Routes served by this module:
//!   GET  /special/login          — login page
//!   POST /special/login          — authenticate + set cookie
//!   POST /special/logout         — delete session + clear cookie
//!   GET  /special/create-account — admin-only account creation form
//!   POST /special/create-account — create new user

use std::sync::Arc;
use axum::{
    extract::{FromRequestParts, Query, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;

use crate::server::AppState;
use crate::users::{self, User};

// ─────────────────────────────────────────────────────────────────────
// Cookie helpers
// ─────────────────────────────────────────────────────────────────────

pub const SESSION_COOKIE: &str = "wiki_session";
const SESSION_MAX_AGE: u32 = 7 * 24 * 3600;

pub fn set_session_cookie(token: &str) -> String {
    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
        SESSION_COOKIE, token, SESSION_MAX_AGE
    )
}

pub fn clear_session_cookie() -> String {
    format!("{}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0", SESSION_COOKIE)
}

fn parse_session_token(cookie_header: &str) -> Option<String> {
    cookie_header.split(';').find_map(|part| {
        let part = part.trim();
        part.strip_prefix(&format!("{}=", SESSION_COOKIE))
            .map(|v| v.to_string())
    })
}

// ─────────────────────────────────────────────────────────────────────
// Extractors
// ─────────────────────────────────────────────────────────────────────

/// Extracts the current user from the session cookie if logged in.
/// Always succeeds — returns None for unauthenticated requests.
pub struct CurrentUser(pub Option<User>);

impl FromRequestParts<Arc<AppState>> for CurrentUser {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let db = match &state.db {
            Some(db) => db.clone(),
            None => return Ok(CurrentUser(None)),
        };

        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let token = match parse_session_token(&cookie_header) {
            Some(t) => t,
            None => return Ok(CurrentUser(None)),
        };

        let user = tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            users::get_by_session(&conn, &token)
        })
        .await
        .unwrap_or(Ok(None))
        .unwrap_or(None);

        Ok(CurrentUser(user))
    }
}

/// Extracts a logged-in user. Redirects to /special/login on failure.
/// When no DB is configured (auth-less mode) every request is treated as admin.
pub struct LoggedInUser(pub User);

impl FromRequestParts<Arc<AppState>> for LoggedInUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if state.db.is_none() {
            return Ok(LoggedInUser(crate::users::User {
                id: String::new(),
                username: "admin".to_string(),
                password_hash: String::new(),
                role: "admin".to_string(),
                created_at: 0,
            }));
        }
        let CurrentUser(maybe_user) =
            CurrentUser::from_request_parts(parts, state).await.unwrap();
        match maybe_user {
            Some(user) => Ok(LoggedInUser(user)),
            None => {
                let path = parts
                    .uri
                    .path_and_query()
                    .map(|pq| pq.as_str())
                    .unwrap_or("/");
                Err(Redirect::to(&format!("/special/login?next={}", path)).into_response())
            }
        }
    }
}

/// Extracts an admin user. Returns 403 for non-admins, redirects for anon.
pub struct AdminUser(pub User);

impl FromRequestParts<Arc<AppState>> for AdminUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let LoggedInUser(user) = LoggedInUser::from_request_parts(parts, state).await?;
        if user.is_admin() {
            Ok(AdminUser(user))
        } else {
            Err((StatusCode::FORBIDDEN, "Admin access required.").into_response())
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Login page
// ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LoginQuery {
    pub next: Option<String>,
    pub error: Option<String>,
}

pub async fn get_login(Query(q): Query<LoginQuery>) -> Markup {
    let next = q.next.as_deref().unwrap_or("/");
    let error = q.error.as_deref();
    login_page(next, error)
}

fn login_page(next: &str, error: Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Log in — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
            }
            body.login-body {
                div.login-layout {
                    // Left panel — branding
                    div.login-brand {
                        a.login-brand-logo href="/" {
                            span.login-brand-icon { "◈" }
                            span.login-brand-name { "PointSav Knowledge" }
                        }
                        p.login-brand-tagline {
                            "Contribute to the institutional knowledge platform. "
                            "Edits are reviewed before going live."
                        }
                        p.login-brand-note {
                            "Access is by invitation. Contact your administrator "
                            "to request an account."
                        }
                    }
                    // Right panel — form
                    div.login-form-panel {
                        h1.login-heading { "Log in" }
                        @if let Some(err) = error {
                            p.login-error { (err) }
                        }
                        form.login-form method="post" action="/special/login" {
                            input type="hidden" name="next" value=(next);
                            div.login-field {
                                label for="username" { "Username" }
                                input #username
                                    type="text"
                                    name="username"
                                    autocomplete="username"
                                    autofocus
                                    required;
                            }
                            div.login-field {
                                label for="password" { "Password" }
                                input #password
                                    type="password"
                                    name="password"
                                    autocomplete="current-password"
                                    required;
                            }
                            div.login-checkbox {
                                input #remember type="checkbox" name="remember" value="1";
                                label for="remember" { "Keep me signed in" }
                            }
                            button.login-submit type="submit" { "Log in" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
    pub remember: Option<String>,
}

pub async fn post_login(
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    let db = match &state.db {
        Some(db) => db.clone(),
        None => return Redirect::to("/special/login?error=Auth+not+configured").into_response(),
    };

    let username = form.username.trim().to_string();
    let password = form.password.clone();
    let next = form.next.as_deref().unwrap_or("/").to_string();

    let result = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        let user = users::get_by_username(&conn, &username)?;
        let Some(user) = user else {
            return Ok::<Option<(User, String)>, rusqlite::Error>(None);
        };
        if !users::verify_password(&password, &user.password_hash) {
            return Ok(None);
        }
        let token = users::create_session(&conn, &user.id)?;
        Ok(Some((user, token)))
    })
    .await;

    match result {
        Ok(Ok(Some((_user, token)))) => {
            let redirect_to = if next.starts_with('/') { next } else { "/".to_string() };
            let mut resp = Redirect::to(&redirect_to).into_response();
            resp.headers_mut().insert(
                axum::http::header::SET_COOKIE,
                set_session_cookie(&token).parse().unwrap(),
            );
            resp
        }
        _ => {
            let error_url = format!("/special/login?error=Invalid+username+or+password&next={}", next);
            Redirect::to(&error_url).into_response()
        }
    }
}

pub async fn post_logout(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Response {
    if let Some(db) = &state.db {
        let cookie_str = headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        if let Some(token) = parse_session_token(&cookie_str) {
            let db = db.clone();
            let _ = tokio::task::spawn_blocking(move || {
                let conn = db.lock().unwrap();
                users::delete_session(&conn, &token)
            })
            .await;
        }
    }
    let mut resp = Redirect::to("/").into_response();
    resp.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        clear_session_cookie().parse().unwrap(),
    );
    resp
}

// ─────────────────────────────────────────────────────────────────────
// Account creation (admin only)
// ─────────────────────────────────────────────────────────────────────

pub async fn get_create_account(
    State(state): State<Arc<AppState>>,
    admin: AdminUser,
) -> Markup {
    let db = state.db.as_ref().unwrap().clone();
    let users_list = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        users::list_users(&conn)
    })
    .await
    .unwrap_or(Ok(vec![]))
    .unwrap_or_default();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Create account — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
            }
            body.login-body {
                div.login-layout {
                    div.login-brand {
                        a.login-brand-logo href="/" {
                            span.login-brand-icon { "◈" }
                            span.login-brand-name { "PointSav Knowledge" }
                        }
                        p.login-brand-tagline { "Manage contributor accounts." }
                        p.login-brand-note {
                            "Logged in as "
                            strong { (admin.0.username) }
                            ". " a href="/" { "← Back to wiki" }
                        }
                    }
                    div.login-form-panel {
                        h1.login-heading { "Create account" }
                        form.login-form method="post" action="/special/create-account" {
                            div.login-field {
                                label for="username" { "Username" }
                                input #username type="text" name="username" required autofocus;
                            }
                            div.login-field {
                                label for="password" { "Password" }
                                input #password type="password" name="password" required;
                            }
                            div.login-field {
                                label for="role" { "Role" }
                                select #role name="role" {
                                    option value="editor" { "Editor (edits go to review queue)" }
                                    option value="admin" { "Admin (direct edits + review queue)" }
                                }
                            }
                            button.login-submit type="submit" { "Create account" }
                        }
                        @if !users_list.is_empty() {
                            h2 style="margin-top:2rem;font-size:1rem;" { "Existing accounts" }
                            table.pending-table {
                                thead { tr {
                                    th { "Username" } th { "Role" }
                                }}
                                tbody {
                                    @for u in &users_list {
                                        tr {
                                            td { (u.username) }
                                            td { (u.role) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct CreateAccountForm {
    pub username: String,
    pub password: String,
    pub role: String,
}

pub async fn post_create_account(
    State(state): State<Arc<AppState>>,
    _admin: AdminUser,
    Form(form): Form<CreateAccountForm>,
) -> Response {
    let db = match &state.db {
        Some(db) => db.clone(),
        None => return Redirect::to("/special/create-account").into_response(),
    };

    let username = form.username.trim().to_string();
    let password = form.password.clone();
    let role = if form.role == "admin" { "admin" } else { "editor" };

    let result = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        users::create_user(&conn, &username, &password, role)
    })
    .await;

    match result {
        Ok(Ok(())) => Redirect::to("/special/create-account").into_response(),
        _ => Redirect::to("/special/create-account?error=1").into_response(),
    }
}
