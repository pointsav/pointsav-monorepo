use crate::{state::AppState, tokens_gallery};
use serde_json::{json, Value};

pub fn list_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "get_component_recipe",
            "description": "Get a component's full recipe (HTML, CSS, ARIA guidance, tokens consumed, variants) by slug — the same recipe.json this site renders live previews from",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Component slug, e.g. 'button'" }
                },
                "required": ["name"]
            }
        }),
        json!({
            "name": "list_components",
            "description": "List all component slugs, optionally filtered by origin category",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "category": {
                        "type": "string",
                        "description": "Filter by origin: 'components' (generic substrate), 'map' (GIS-origin), 'wiki' (wiki-engine-origin). Omit for all."
                    }
                }
            }
        }),
        json!({
            "name": "get_token",
            "description": "Resolve a single design token by its canonical CSS custom property name (e.g. '--ps-primitive-color-primary-60') -- byte-identical to the token's own $extensions id, no translation needed -- or by its DTCG dot-path (e.g. 'color.primary-60'). Old ids/paths from a renamed token family (e.g. 'legal-agreement' before it became 'legal-subscription-agreement') still resolve via a server-side alias -- no need to know a token's current name to look it up.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "--ps-* canonical custom property name or dot-path -- current or pre-rename" }
                },
                "required": ["name"]
            }
        }),
        json!({
            "name": "search_design_system",
            "description": "Full-text search across every indexed vault document — components, tokens, research, guidelines, developing, designing, about",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string" }
                },
                "required": ["query"]
            }
        }),
        json!({
            "name": "list_token_families",
            "description": "List every token family (pillar/layer/family grouping, e.g. paper/semantic/financial-report-layout) with its member count — the taxonomy a producer archive uses to find the right token group before guessing a name. Sourced from the same generated registry get_token and the /tokens page read, never a hand-maintained list.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pillar": {
                        "type": "string",
                        "description": "Filter to one pillar, e.g. 'paper', 'writing', 'primitive', 'theme'. Omit for all pillars."
                    }
                }
            }
        }),
    ]
}

pub async fn call_tool(params: &Option<Value>, state: &AppState) -> Result<Value, String> {
    let p = params.as_ref().ok_or("missing params")?;
    let name = p
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or("missing tool name")?;
    let args = p.get("arguments").cloned().unwrap_or(Value::Null);

    match name {
        "get_component_recipe" => {
            let slug = args
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("missing name")?;
            if slug.contains("..") || slug.contains('/') {
                return Err("invalid name".to_string());
            }
            let path = state
                .vault
                .join("components")
                .join(slug)
                .join("recipe.json");
            let raw = std::fs::read_to_string(&path)
                .map_err(|_| format!("no recipe.json for component '{slug}'"))?;
            let recipe: Value = serde_json::from_str(&raw)
                .map_err(|_| "recipe.json is not valid JSON".to_string())?;
            Ok(json!({ "content": [{ "type": "text", "text": recipe.to_string() }] }))
        }
        "list_components" => {
            let category_filter = args.get("category").and_then(|v| v.as_str());
            let slugs: Vec<String> = match category_filter {
                None => state.nav.get("components").cloned().unwrap_or_default(),
                Some(cat) => state
                    .component_groups
                    .iter()
                    .find(|(label, _)| label_matches_category(label, cat))
                    .map(|(_, slugs)| slugs.clone())
                    .unwrap_or_default(),
            };
            Ok(
                json!({ "content": [{ "type": "text", "text": serde_json::to_string(&slugs).unwrap() }] }),
            )
        }
        "get_token" => {
            let query = args
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("missing name")?;
            let tiers = tokens_gallery::load_and_flatten(&state.vault);
            let entries: Vec<&tokens_gallery::TokenEntry> = tiers
                .iter()
                .flat_map(|tier| &tier.groups)
                .flat_map(|group| &group.entries)
                .collect();
            let direct_hit = entries
                .iter()
                .find(|e| e.css_var == query || e.path == query);
            let hit = match direct_hit {
                Some(e) => Some(*e),
                // A direct lookup miss doesn't necessarily mean the token never existed --
                // it may have been renamed. Check the alias registry before giving up, so a
                // consumer using a pre-rename id/path still resolves instead of getting a
                // false "not found" that pushes toward re-drafting a token that already
                // exists (the exact failure mode this whole pipeline exists to prevent).
                None => resolve_renamed_query(&state.vault, query).and_then(|resolved| {
                    entries
                        .iter()
                        .find(|e| e.css_var == resolved || e.path == resolved)
                        .copied()
                }),
            };
            match hit {
                Some(entry) => {
                    // Correction (2026-08-02 MCP functional-test finding): get_token used to
                    // return an alias token's raw $value verbatim -- e.g. "{color.neutral-100}"
                    // instead of the actual "#1a1a1a" it resolves to. An agent calling get_token
                    // to "get the tokens right from the website" then had to make a second,
                    // undocumented call to resolve the alias itself. Resolve it server-side;
                    // keep the raw alias string too so composition relationships aren't hidden.
                    let resolved_value = resolve_alias_value(&entry.value, &entries, 0);
                    Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": json!({
                                "path": entry.path,
                                "css_var": entry.css_var,
                                "value": resolved_value,
                                "raw_value": entry.value,
                                "kind": entry.kind,
                                "description": entry.description,
                            }).to_string()
                        }]
                    }))
                }
                None => Err(format!("no token found matching '{query}'")),
            }
        }
        "search_design_system" => {
            let query = args
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or("missing query")?;
            let idx = state.index.read().await;
            let hits: Vec<Value> = idx
                .search(query)
                .into_iter()
                .take(20)
                .map(|doc| json!({ "id": doc.id, "title": doc.title }))
                .collect();
            Ok(
                json!({ "content": [{ "type": "text", "text": serde_json::to_string(&hits).unwrap() }] }),
            )
        }
        "list_token_families" => {
            let pillar_filter = args.get("pillar").and_then(|v| v.as_str());
            let path = state.vault.join("exports").join("token-families.json");
            let raw = std::fs::read_to_string(&path).map_err(|_| {
                "token-families.json not found -- run bin/generate-tokens-export.py".to_string()
            })?;
            let registry: Value = serde_json::from_str(&raw)
                .map_err(|_| "token-families.json is not valid JSON".to_string())?;
            let all = registry
                .get("families")
                .and_then(|v| v.as_array())
                .ok_or("token-families.json missing 'families' array")?;
            let families: Vec<&Value> = match pillar_filter {
                None => all.iter().collect(),
                Some(p) => all
                    .iter()
                    .filter(|f| f.get("pillar").and_then(|v| v.as_str()) == Some(p))
                    .collect(),
            };
            Ok(
                json!({ "content": [{ "type": "text", "text": serde_json::to_string(&families).unwrap() }] }),
            )
        }
        other => Err(format!("unknown tool: {other}")),
    }
}

/// Resolves a DTCG `$value` that may itself be an alias reference (`{dotted.path}`) to
/// the literal value it ultimately points at, by looking the referenced path up in the
/// same flattened entries list `get_token` already loaded -- no second vault read.
/// Follows multi-level aliases (an alias pointing at another alias); `depth` guards
/// against a cyclic reference in the source data turning into an infinite loop, since
/// nothing upstream currently validates against that. Returns the value as-is,
/// unresolved, if it isn't an alias, if the referenced path doesn't exist, or if depth
/// is exceeded -- callers should treat that as "couldn't resolve further," not an error;
/// `get_token`'s response also carries the original raw value so nothing is hidden.
fn resolve_alias_value(value: &str, entries: &[&tokens_gallery::TokenEntry], depth: u8) -> String {
    if depth > 8 {
        return value.to_string();
    }
    let Some(inner) = value.strip_prefix('{').and_then(|v| v.strip_suffix('}')) else {
        return value.to_string();
    };
    match entries.iter().find(|e| e.path == inner) {
        Some(target) => resolve_alias_value(&target.value, entries, depth + 1),
        None => value.to_string(),
    }
}

/// Rename-deprecation alias resolution for `get_token`. Reads `dtcg-vault/renames.json`
/// directly (hand-maintained alongside the rename commit itself, not a generator output --
/// see that file's own $description) and, if `query` matches a known pre-rename id/path,
/// returns the current id/path to look up instead. Family renames (a whole
/// `paper.semantic.<family>.*` group moved at once, the common case) are checked before
/// individual leaf renames. Returns `None` on any read/parse error or no match -- callers
/// treat that the same as "not a renamed token," not a hard error, since most lookups won't
/// need this path at all.
fn resolve_renamed_query(vault: &std::path::Path, query: &str) -> Option<String> {
    let raw = std::fs::read_to_string(vault.join("renames.json")).ok()?;
    let renames: Value = serde_json::from_str(&raw).ok()?;

    if let Some(families) = renames.get("family_renames").and_then(|v| v.as_array()) {
        for f in families {
            let old_id_prefix = f.get("old_id_prefix").and_then(|v| v.as_str());
            let new_id_prefix = f.get("new_id_prefix").and_then(|v| v.as_str());
            if let (Some(old_p), Some(new_p)) = (old_id_prefix, new_id_prefix) {
                if let Some(suffix) = query.strip_prefix(old_p) {
                    return Some(format!("{new_p}{suffix}"));
                }
            }
            let old_path_prefix = f.get("old_path_prefix").and_then(|v| v.as_str());
            let new_path_prefix = f.get("new_path_prefix").and_then(|v| v.as_str());
            if let (Some(old_p), Some(new_p)) = (old_path_prefix, new_path_prefix) {
                if let Some(suffix) = query.strip_prefix(old_p) {
                    return Some(format!("{new_p}{suffix}"));
                }
            }
        }
    }

    if let Some(leaves) = renames.get("leaf_renames").and_then(|v| v.as_array()) {
        for l in leaves {
            let old_id = l.get("old_id").and_then(|v| v.as_str());
            let old_path = l.get("old_path").and_then(|v| v.as_str());
            let new_id = l.get("new_id").and_then(|v| v.as_str());
            if (old_id == Some(query) || old_path == Some(query)) && new_id.is_some() {
                return new_id.map(|s| s.to_string());
            }
        }
    }

    None
}

/// `component_groups` labels are human-readable prose ("Also used by the wiki engine"),
/// not the raw category string — match on the same keywords `vault::discover_component_groups`
/// uses to build them, plus the empty-label generic group.
fn label_matches_category(label: &str, category: &str) -> bool {
    match category {
        "components" => label.is_empty(),
        "map" => label.contains("gis.woodfinegroup.com"),
        "wiki" => label.contains("wiki engine"),
        other => label.to_lowercase().contains(&other.to_lowercase()),
    }
}
