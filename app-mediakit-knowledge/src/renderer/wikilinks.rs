use std::collections::HashMap;

/// In-memory page index built at startup by walking the content directory.
/// slug → PageEntry. Used by the wikilink resolver and search autocomplete.
#[derive(Debug, Clone, Default)]
pub struct PageIndex(pub HashMap<String, PageEntry>);

#[derive(Debug, Clone)]
pub struct PageEntry {
    pub title: String,
    pub category: String,
    /// Relative URL path, e.g. "architecture/os-totebox"
    pub path: String,
}

impl PageIndex {
    pub fn get(&self, slug: &str) -> Option<&PageEntry> { self.0.get(slug) }

    pub fn all_titles(&self) -> Vec<(String, String)> {
        self.0.iter().map(|(s, e)| (s.clone(), e.title.clone())).collect()
    }
}

/// Convert [[slug]] and [[slug|Display Text]] wikilink syntax to standard
/// Markdown links before the pulldown-cmark parser runs.
///
/// Known slug  → [Title](../category/slug)
/// Known slug with display text → [Display Text](../category/slug)
/// Unknown slug → <span class="wiki-redlink">slug</span>
///               (red links signal missing articles — consistent with Wikipedia)
pub fn resolve(markdown: &str, index: &PageIndex) -> String {
    let re = regex::Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();
    re.replace_all(markdown, |caps: &regex::Captures| {
        let slug    = caps[1].trim();
        let display = caps.get(2).map(|m| m.as_str().trim());
        match index.get(slug) {
            Some(entry) => {
                let label = display.unwrap_or(&entry.title);
                format!("[{}](../{})", label, entry.path)
            }
            None => {
                let label = display.unwrap_or(slug);
                format!(r#"<span class="wiki-redlink" title="Article does not exist">{label}</span>"#)
            }
        }
    }).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index() -> PageIndex {
        let mut m = HashMap::new();
        m.insert("os-totebox".into(), PageEntry {
            title: "ToteboxOS".into(), category: "architecture".into(),
            path: "architecture/os-totebox".into(),
        });
        PageIndex(m)
    }

    #[test]
    fn resolves_known_slug() {
        assert!(resolve("See [[os-totebox]].", &index()).contains("[ToteboxOS](../architecture/os-totebox)"));
    }

    #[test]
    fn resolves_with_display_text() {
        assert!(resolve("See [[os-totebox|the archive OS]].", &index()).contains("[the archive OS]"));
    }

    #[test]
    fn red_link_for_unknown() {
        let r = resolve("[[nonexistent]]", &index());
        assert!(r.contains("wiki-redlink"));
    }
}
