use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Glossary {
    pub map: HashMap<String, String>,
    re_terms: Option<regex::Regex>,
}

pub fn load_glossary(content_dir: &Path) -> Glossary {
    let mut map = HashMap::new();
    
    // Discover and load any glossary-*.csv files in the content directory
    if let Ok(entries) = std::fs::read_dir(content_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.starts_with("glossary-") {
                        load_glossary_file(&path, &mut map);
                    }
                }
            }
        }
    }
    
    let re_terms = if map.is_empty() {
        None
    } else {
        let mut terms: Vec<&String> = map.keys().collect();
        terms.sort_by(|a, b| b.len().cmp(&a.len()));
        let escaped_terms: Vec<String> = terms.iter().map(|t| regex::escape(t)).collect();
        let pattern = format!(r"(?i)\b({})\b", escaped_terms.join("|"));
        regex::Regex::new(&pattern).ok()
    };
    
    Glossary { map, re_terms }
}

fn load_glossary_file(path: &Path, map: &mut HashMap<String, String>) {
    if let Ok(mut rdr) = csv::Reader::from_path(path) {
        for result in rdr.records() {
            if let Ok(record) = result {
                if let (Some(en), Some(es), Some(defn)) = (record.get(0), record.get(1), record.get(2)) {
                    let defn = defn.trim();
                    if !defn.is_empty() {
                        let en_term = en.trim();
                        if !en_term.is_empty() {
                            map.insert(en_term.to_lowercase(), defn.to_string());
                        }
                        let es_term = es.trim();
                        if !es_term.is_empty() {
                            map.insert(es_term.to_lowercase(), defn.to_string());
                        }
                    }
                }
            }
        }
    }
}

pub fn inject_glossary_tooltips(html: &str, glossary: &Glossary) -> String {
    if glossary.map.is_empty() {
        return html.to_string();
    }
    
    // First, handle explicit {{gli|Term}} syntax with placeholders
    let mut placeholders = Vec::new();
    let re_explicit = regex::Regex::new(r"\{\{gli\|([^}]+)\}\}").unwrap();
    let result = re_explicit.replace_all(html, |caps: &regex::Captures| {
        let term = &caps[1];
        let defn = glossary.map.get(&term.to_lowercase()).map(String::as_str).unwrap_or("Definition not found");
        let replacement = format!("<span class=\"wiki-glossary-term\" title=\"{}\" aria-label=\"{}\">{}</span>", html_escape(defn), html_escape(defn), term);
        let id = placeholders.len();
        placeholders.push(replacement);
        format!("GLOSSARY_PLACEHOLDER_{}_GLOSSARY", id)
    }).to_string();
    
    // Now auto-link
    let mut result = if let Some(ref re_terms) = glossary.re_terms {
        let mut out = String::with_capacity(result.len() * 2);
        let mut inside_a = false;
        let mut last_idx = 0;
        
        let re_tags = regex::Regex::new(r"(<[^>]+>)").unwrap();
        for mat in re_tags.find_iter(&result) {
            let text_segment = &result[last_idx..mat.start()];
            if !inside_a && !text_segment.is_empty() {
                let replaced = re_terms.replace_all(text_segment, |caps: &regex::Captures| {
                    let term = &caps[1];
                    let defn = glossary.map.get(&term.to_lowercase()).map(String::as_str).unwrap_or("Definition not found");
                    format!("<span class=\"wiki-glossary-term\" title=\"{}\" aria-label=\"{}\">{}</span>", html_escape(defn), html_escape(defn), term)
                });
                out.push_str(&replaced);
            } else {
                out.push_str(text_segment);
            }
            
            let tag = mat.as_str();
            if tag.starts_with("<a ") || tag == "<a>" {
                inside_a = true;
            } else if tag == "</a>" {
                inside_a = false;
            }
            out.push_str(tag);
            last_idx = mat.end();
        }
        
        let remaining = &result[last_idx..];
        if !inside_a && !remaining.is_empty() {
            let replaced = re_terms.replace_all(remaining, |caps: &regex::Captures| {
                let term = &caps[1];
                let defn = glossary.map.get(&term.to_lowercase()).map(String::as_str).unwrap_or("Definition not found");
                format!("<span class=\"wiki-glossary-term\" title=\"{}\" aria-label=\"{}\">{}</span>", html_escape(defn), html_escape(defn), term)
            });
            out.push_str(&replaced);
        } else {
            out.push_str(remaining);
        }
        out
    } else {
        result
    };

    // Restore placeholders
    for (id, replacement) in placeholders.iter().enumerate() {
        let placeholder = format!("GLOSSARY_PLACEHOLDER_{}_GLOSSARY", id);
        result = result.replace(&placeholder, replacement);
    }
    
    result
}

fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
}
