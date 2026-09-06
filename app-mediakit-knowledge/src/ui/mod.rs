// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! UI layer — the visible website: per-tenant identity and the chrome shell.
//! The visual system lives in `static/{tokens,app}.css`; this module emits the
//! matching `k-*` HTML structure (Wikipedia Vector 2022 pattern).

pub mod layout;
pub mod tenant;

pub use layout::{
    article, article_jsonld, breadcrumb, breadcrumb_jsonld, category_index, diff_page, doc_head,
    footer, header, history_page, home_page, hreflang_links, index_topic_body, index_topic_header,
    masthead, mobile_nav, notice_banner, page, redacted_notice, research_fulltext,
    research_landing, search_results, simple_message, special_list, utility_bar,
};
pub use tenant::{SiblingWiki, Tenant};
