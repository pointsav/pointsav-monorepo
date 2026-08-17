// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.


use axum::{extract::State, response::Html};

use crate::{render, state::AppState};

pub async fn home_handler(State(state): State<AppState>) -> Html<String> {
    let content = render::catalog::render_home(&state);
    Html(render::shell::page_shell("", "/", &content, &state))
}
