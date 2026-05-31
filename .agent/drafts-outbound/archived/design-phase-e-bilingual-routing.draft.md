---
schema: foundry-draft-v1
draft_type: DESIGN-COMPONENT
title: "Wiki bilingual routing — Phase E (/es/ URL tree + locale-aware home_chrome)"
slug: design-phase-e-bilingual-routing
status: draft
created: 2026-05-20
author: project-editorial@claude-code
route_to: project-knowledge
destination_crate: pointsav-monorepo/app-mediakit-knowledge
requires_cosign: false
research_trail:
  method: "Derived from Phase D design spec (design-home-chrome-v2.draft.md §7) plus review of existing ES content inventory: index.es.md in content-wiki-documentation and content-wiki-corporate, ~174 article pairs with .es.md counterparts in content-wiki-documentation, topic-*.es.md pairs in content-wiki-projects."
  sources:
    - "design-home-chrome-v2.draft.md §7 — Phase E scope declared (2026-05-17)"
    - "content-wiki-documentation/index.es.md — exists; not yet served"
    - "content-wiki-corporate/index.es.md — committed 2026-05-19 (commit 188dabd)"
    - "leapfrog-facts.yaml pattern — load_dyk() at server.rs:885"
  prior_draft: "design-home-chrome-v2.draft.md §7 — scope summary only, no implementation detail"
  corpus_state: "Documentation wiki: ~174 EN+ES article pairs. Projects wiki: ~24 topics, most with .es.md pairs. Corporate wiki: 5 topics with .es.md pairs. All three wikis have index.es.md."
---

# Wiki bilingual routing — Phase E

## Scope

Add explicit `/es/` URL tree support to `app-mediakit-knowledge`. When a visitor hits
`/es/` or `/es/wiki/{slug}`, the server serves Spanish content where it exists, falls
back to English where it does not, and sets `<html lang="es">` throughout.

This is additive — the existing `/` and `/wiki/{slug}` routes are unchanged.

Phase E does **not** include:
- Accept-Language header negotiation (explicit URL only — simpler, cacheable, linkable)
- Category pages in Spanish (`/es/category/{name}`)
- Search results in Spanish
- Full UI string localization (nav labels, UI chrome text)

---

## 1. Locale type

Add near the top of `server.rs`, alongside the existing enums:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Locale {
    #[default]
    En,
    Es,
}

impl Locale {
    fn lang_attr(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Es => "es",
        }
    }
    fn suffix(self) -> &'static str {
        match self {
            Locale::En => "",
            Locale::Es => ".es",
        }
    }
}
```

---

## 2. New routes

In the Axum router, add alongside the existing `"/"` and `"/wiki/:slug"` routes:

```rust
.route("/es/",           get(home_es))
.route("/es/wiki/:slug", get(article_es))
```

No other `/es/` routes are needed for Phase E.

---

## 3. Refactor home handler — extract `home_inner()`

The current `index()` handler (or equivalent) becomes a thin wrapper. Extract the
shared logic into `home_inner()`:

```rust
async fn index(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    home_inner(Arc::clone(&app), Locale::En).await
}

async fn home_es(State(app): State<Arc<AppState>>) -> impl IntoResponse {
    home_inner(Arc::clone(&app), Locale::Es).await
}

async fn home_inner(app: Arc<AppState>, locale: Locale) -> impl IntoResponse {
    // 1. Load index doc — prefer {locale}.md, fall back to index.md
    let index_filename = match locale {
        Locale::En => "index.md",
        Locale::Es => "index.es.md",
    };
    let index_path = app.content_dir.join(index_filename);
    let index_content = if index_path.exists() {
        fs::read_to_string(&index_path).await.unwrap_or_default()
    } else {
        // Fall back to English index
        fs::read_to_string(app.content_dir.join("index.md")).await.unwrap_or_default()
    };

    // 2. Load DYK — prefer leapfrog-facts.es.yaml, fall back to leapfrog-facts.yaml
    let dyk = load_dyk_localized(&app.content_dir, locale).await;

    // 3. Load reference invariants (unchanged from Phase D — no ES variant needed for v1)
    let ref_inv = load_reference_invariants(&app.content_dir).await;

    // 4. Load featured topic, stats, etc. (unchanged)
    // ...

    // 5. Render
    home_chrome(locale, &stats, &dyk, ref_inv, /* ... */)
}
```

---

## 4. Localized DYK loader

Replace or supplement the existing `load_dyk()` with a locale-aware version:

```rust
async fn load_dyk_localized(content_dir: &Path, locale: Locale) -> Option<LeapfrogFacts> {
    if locale == Locale::Es {
        let es_path = content_dir.join("leapfrog-facts.es.yaml");
        if es_path.exists() {
            let text = fs::read_to_string(&es_path).await.ok()?;
            if let Ok(facts) = serde_yaml::from_str(&text) {
                return Some(facts);
            }
        }
    }
    // Fall back to English (also the only path for Locale::En)
    load_dyk(content_dir).await
}
```

**New content files required** (editorial work, not code):

| File | Wiki |
|---|---|
| `content-wiki-documentation/leapfrog-facts.es.yaml` | documentation.pointsav.com |
| `content-wiki-projects/leapfrog-facts.es.yaml` | projects.woodfinegroup.com |
| `content-wiki-corporate/leapfrog-facts.es.yaml` | corporate.woodfinegroup.com |

These are Spanish translations of the existing `leapfrog-facts.yaml` files. project-editorial
should produce these before or alongside the Phase E code deploy.

---

## 5. `home_chrome()` signature change

Add `locale: Locale` as the first parameter:

```rust
fn home_chrome(
    locale: Locale,
    stats: &WikiStats,
    dyk: &Option<LeapfrogFacts>,
    ref_inv: Option<ReferenceInvariants>,
    // ... existing params
) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(locale.lang_attr()) {
            // ... rest unchanged
```

The `lang` attribute on `<html>` is the only change inside `home_chrome()` for Phase E.
All other chrome (nav, panels, category grid) remains English for v1 — full UI string
localization is Phase F scope.

---

## 6. Language switcher in nav

Inside `home_chrome()`, in the nav bar, add a language toggle link:

```rust
// In nav section — add alongside search/about links:
a.lang-toggle href=(match locale {
    Locale::En => "/es/",
    Locale::Es => "/",
}) {
    (match locale {
        Locale::En => "ES",
        Locale::Es => "EN",
    })
}
```

CSS (add to `static/wiki.css`):

```css
.lang-toggle {
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    padding: 0.2rem 0.5rem;
    border: 1px solid currentColor;
    border-radius: 3px;
    opacity: 0.7;
    text-decoration: none;
}
.lang-toggle:hover { opacity: 1; }
```

---

## 7. Article handler — extract `article_inner()`

Same refactor pattern as the home handler:

```rust
async fn article(
    Path(slug): Path<String>,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    article_inner(slug, Locale::En, Arc::clone(&app)).await
}

async fn article_es(
    Path(slug): Path<String>,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    article_inner(slug, Locale::Es, Arc::clone(&app)).await
}

async fn article_inner(
    slug: String,
    locale: Locale,
    app: Arc<AppState>,
) -> impl IntoResponse {
    // Try locale-specific file first
    let filename = format!("{}{}.md", slug, locale.suffix());
    let path = app.content_dir.join("wiki").join(&filename);
    let (content, effective_locale) = if path.exists() {
        (fs::read_to_string(&path).await.ok(), locale)
    } else {
        // Fall back to English
        let en_path = app.content_dir.join("wiki").join(format!("{}.md", slug));
        (fs::read_to_string(&en_path).await.ok(), Locale::En)
    };
    // Render with effective_locale for lang= attribute
    // ...
}
```

**Note:** `effective_locale` is used for the `<html lang="">` attribute and the language
switcher href. If the article fell back from ES to EN, the switcher should point to `/es/wiki/{slug}`
(so the visitor can switch back if they expect ES may be added later) — keep the toggle
always pointing to the other locale's URL regardless of fallback.

---

## 8. `<html lang="">` in article chrome

The `article_chrome()` function (or equivalent renderer) needs the same `locale: Locale`
parameter passed through, used only for the `lang=` attribute. No other article-page
changes are in scope for Phase E.

---

## 9. Canonical + hreflang `<link>` tags

Add to `<head>` in both `home_chrome()` and `article_chrome()`:

```rust
// For EN page at /wiki/{slug}:
link rel="alternate" hreflang="es" href={"/es/wiki/" (slug)} {}
link rel="canonical" href={"/wiki/" (slug)} {}

// For ES page at /es/wiki/{slug}:
link rel="alternate" hreflang="en" href={"/wiki/" (slug)} {}
link rel="canonical" href={"/es/wiki/" (slug)} {}
```

For home pages: `"/"` ↔ `"/es/"`.

This is a search-engine requirement for bilingual content — do not omit.

---

## 10. Featured topic — no change needed

`featured-topic.yaml` uses a `slug:` field. The `article_inner()` fallback already
handles ES preference, so the featured topic card can link to `/wiki/{slug}` (EN) with
no change. Phase F can add a `slug_es:` override field if needed.

---

## 11. leapfrog-facts.es.yaml — content spec

The ES DYK files are editorial artifacts, not code. project-editorial should produce
these files before Phase E ships. Each file mirrors the structure of `leapfrog-facts.yaml`:

```yaml
# leapfrog-facts.es.yaml
facts:
  - text: "Spanish translation of DYK fact 1."
    link_slug: slug-of-related-article
  # ...
```

The facts should be strategic adaptations (Spanish phrasing appropriate for LatAm +
Spain audiences), not literal translations. Apply the same BCSC disclosure posture:
forward-looking claims use "previsto/planeado/puede" language.

---

## 12. Implementation order

1. **Add `Locale` enum** — no behaviour change, no risk.
2. **Add `/es/` and `/es/wiki/:slug` routes** (return 404 stub) — smoke test that routing compiles.
3. **Refactor `index()` → `home_inner()`** — no behaviour change to existing `/` route.
4. **Wire `home_es()`** — test `/es/` with existing `index.es.md`.
5. **Add `locale.lang_attr()` to `home_chrome()`** — single attribute change.
6. **Add language switcher** — visual addition to nav.
7. **Add hreflang `<link>` tags** — `<head>` addition.
8. **Refactor `article()` → `article_inner()`** — no behaviour change to existing route.
9. **Wire `article_es()`** — test `/es/wiki/{slug}` with an existing `.es.md` pair.
10. **Add `load_dyk_localized()`** — replace `load_dyk()` call in `home_inner()`.
11. **Content: `leapfrog-facts.es.yaml`** — editorial work; project-editorial delivers.
12. **Ship** — single cargo build + service restart.

Steps 1–9 can land in one commit. Steps 10–11 in a second commit once the ES DYK
content is ready.

---

## 13. Files affected

| File | Change |
|---|---|
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs` | Add `Locale` enum; extract `home_inner()`, `article_inner()`; add `/es/` routes; add `lang=` to `home_chrome()` + `article_chrome()`; add hreflang tags; add `load_dyk_localized()` |
| `pointsav-monorepo/app-mediakit-knowledge/static/wiki.css` | Add `.lang-toggle` styles |
| `content-wiki-documentation/leapfrog-facts.es.yaml` | New — Spanish DYK content |
| `content-wiki-projects/leapfrog-facts.es.yaml` | New — Spanish DYK content |
| `content-wiki-corporate/leapfrog-facts.es.yaml` | New — Spanish DYK content |

`index.es.md` already exists in all three wikis — no editorial work required for home content.

---

## 14. What not to change in Phase E

- Existing `/` and `/wiki/{slug}` routes — unchanged
- Category pages — English only in Phase E
- Search — language-agnostic; `/search` unchanged
- Nav labels, panel headings, UI chrome text — English only in Phase E (Phase F scope)
- `reference-invariants.yaml` — no ES variant needed for v1; panel text is factual/structural
