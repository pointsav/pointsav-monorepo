// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.


// DESIGN-BUNDLE directory mounts — list, serve, and zip-download an entire
// externally-owned directory (canonical-source-with-downstream-mount, DOCTRINE §IV.e).
// The source directory is never copied; `state.bundle_mounts` only holds a path.
use crate::{i18n::PageLang, render, schema, state::AppState, vault};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use std::{fs, io::Write, path::Path as StdPath};

#[derive(serde::Serialize)]
struct BundleFile {
    filename: String,
    title: String,
}

/// Lists any file in the mount (not just `.md`) — non-.md files (e.g. the tokens.css /
/// tokens.full.json bundle) get a title derived from the filename instead of frontmatter.
fn list_bundle_files(dir: &StdPath) -> Vec<BundleFile> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<BundleFile> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            let title = if name.ends_with(".md") {
                let raw = fs::read_to_string(e.path()).ok()?;
                let (fm, _) = vault::parse_frontmatter(&raw);
                fm.get("title")
                    .cloned()
                    .unwrap_or_else(|| vault::to_title(name.strip_suffix(".md").unwrap_or(&name)))
            } else {
                name.clone()
            };
            Some(BundleFile {
                filename: name,
                title,
            })
        })
        .collect();
    files.sort_by(|a, b| a.filename.cmp(&b.filename));
    files
}

fn content_type_for(filename: &str) -> &'static str {
    if filename.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if filename == "tokens.full.json" {
        // DTCG's own registered media type (Format Module, 2025.10) — every other
        // .json file served through this generic bundle route (component recipes,
        // other externally-mounted DESIGN-BUNDLE directories) stays plain
        // application/json below; only the DTCG token export itself gets this.
        "application/design-tokens+json; charset=utf-8"
    } else if filename.ends_with(".json") {
        "application/json; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    }
}

pub async fn list(Path(name): Path<String>, State(state): State<AppState>) -> Response {
    let Some(dir) = state.bundle_mounts.get(&name) else {
        return (StatusCode::NOT_FOUND, "bundle not found").into_response();
    };
    let files = list_bundle_files(dir);
    let title = vault::to_title(&name);
    let file_count = files.len();
    let path = format!("/bundles/{name}");

    let list_html = state
        .env
        .get_template("bundle.html")
        .expect("bundle.html missing")
        .render(minijinja::context! {
            name => name,
            file_count => file_count,
            files => files,
        })
        .expect("render bundle.html failed");

    let nav_html = render::render_nav(&state.env, &state.component_groups, "", "");
    Html(render::shell(
        &state.env,
        &state.vault,
        &state.component_groups,
        &state.site_origin,
        &format!("{title} — PointSav Design System"),
        &format!(
            "Download the {title} bundle from the PointSav Design System — {file_count} files."
        ),
        &path,
        &PageLang::en_only(),
        &nav_html,
        "",
        &title,
        &list_html,
    ))
    .into_response()
}

pub async fn file(
    Path((name, filename)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Response {
    if filename.contains("..") || filename.contains('/') {
        return (StatusCode::BAD_REQUEST, "invalid").into_response();
    }
    let Some(dir) = state.bundle_mounts.get(&name) else {
        return (StatusCode::NOT_FOUND, "bundle not found").into_response();
    };
    // Fable-audit finding (2026-08-02): this used to read the file as a String via
    // `fs::read_to_string`, which errors on any non-UTF-8 file -- silently 404ing
    // any binary file a DESIGN-BUNDLE mount happens to list (list_bundle_files lists
    // every file regardless of content type). Both mounts today are text-only, so no
    // live impact yet, but the route is meant to be a generic bundle-file server, not
    // text-only. Reading raw bytes first and only decoding to a String for the .md
    // rendering branch below (the one path that genuinely needs text) makes this
    // binary-safe for whatever gets mounted next.
    let Ok(raw_bytes) = fs::read(dir.join(&filename)) else {
        return (StatusCode::NOT_FOUND, "file not found").into_response();
    };

    if !filename.ends_with(".md") {
        return (
            [(header::CONTENT_TYPE, content_type_for(&filename))],
            raw_bytes,
        )
            .into_response();
    }

    let Ok(raw) = String::from_utf8(raw_bytes) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "not valid UTF-8").into_response();
    };
    let (frontmatter, body) = vault::parse_frontmatter(&raw);
    let schema_type = schema::detect(&frontmatter);
    let content = schema::render(schema_type, &frontmatter, &body);

    let nav_html = render::render_nav(&state.env, &state.component_groups, "", "");
    Html(render::shell(
        &state.env,
        &state.vault,
        &state.component_groups,
        &state.site_origin,
        &format!("{filename} — PointSav Design System"),
        &format!(
            "{filename}, from the {} bundle in the PointSav Design System.",
            vault::to_title(&name)
        ),
        &format!("/bundles/{name}/{filename}"),
        &PageLang::en_only(),
        &nav_html,
        "",
        "",
        &content,
    ))
    .into_response()
}

pub async fn download(Path(name): Path<String>, State(state): State<AppState>) -> Response {
    let Some(dir) = state.bundle_mounts.get(&name) else {
        return (StatusCode::NOT_FOUND, "bundle not found").into_response();
    };
    let files = list_bundle_files(dir);
    if files.is_empty() {
        return (StatusCode::NOT_FOUND, "bundle is empty").into_response();
    }

    let mut buf = std::io::Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut buf);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        for f in &files {
            let Ok(bytes) = fs::read(dir.join(&f.filename)) else {
                continue;
            };
            if zip.start_file(&f.filename, options).is_err() {
                continue;
            }
            let _ = zip.write_all(&bytes);
        }
        let _ = zip.finish();
    }

    (
        [
            (header::CONTENT_TYPE, "application/zip".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{name}.zip\""),
            ),
        ],
        buf.into_inner(),
    )
        .into_response()
}

/// `/tokens.json` — documentation (get-started.md, the Foundations page, and the
/// TOPIC-figma-tokens-studio-integration wiki article all promise this exact path) has
/// always named the export `/tokens.json`, but the real server route only ever served it
/// at `/bundles/tokens/tokens.full.json`. A 308 permanent redirect closes that gap without
/// duplicating the bundle route or renaming the canonical file.
pub async fn tokens_json_redirect() -> Redirect {
    Redirect::permanent("/bundles/tokens/tokens.full.json")
}
