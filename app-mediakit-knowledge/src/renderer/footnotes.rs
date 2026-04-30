use super::{Reference, ReferenceDef};
use pulldown_cmark::Event;

pub type InlineAnchors = std::collections::HashMap<String, String>;

/// Replace pulldown-cmark FootnoteReference events with Wikipedia-style
/// superscript anchor links and record inline anchor ids for back-arrows.
///
/// TODO: implement this function.
///
/// Wikipedia footnote contract (both directions required):
///   Body:          [^1] → <sup id="ref-cite-1"><a href="#cite-1">[1]</a></sup>
///   Bibliography:  entry has <a href="#ref-cite-1">↑</a> back-arrow
///
/// Algorithm:
///   for event in events {
///       match event {
///           Event::FootnoteReference(label) => {
///               let n           = label.as_ref();
///               let inline_id   = format!("ref-cite-{n}");
///               let target_id   = format!("cite-{n}");
///               anchors.insert(n.to_string(), inline_id.clone());
///               let html = format!(
///                   r#"<sup id="{inline_id}"><a href="#{target_id}">[{n}]</a></sup>"#
///               );
///               out.push(Event::Html(html.into()));
///           }
///           other => out.push(other),
///       }
///   }
pub fn process_inline<'a>(
    events: impl Iterator<Item = Event<'a>>,
    _reference_defs: &[ReferenceDef],
) -> (Vec<Event<'a>>, InlineAnchors) {
    let mut out = Vec::new();
    let anchors = InlineAnchors::new();
    for event in events { out.push(event); } // passthrough until implemented
    (out, anchors)
}

/// Build the resolved Reference list from front matter defs + inline anchors.
/// Only references that actually appear in the body are included.
pub fn build_bibliography(defs: &[ReferenceDef], anchors: &InlineAnchors) -> Vec<Reference> {
    let mut refs: Vec<Reference> = defs.iter()
        .filter(|d| anchors.contains_key(&d.id.to_string()))
        .map(|d| Reference {
            number: d.id,
            text: d.text.clone(),
            url: d.url.clone(),
            internal: d.internal,
            anchor: anchors.get(&d.id.to_string()).cloned().unwrap_or_default(),
        })
        .collect();
    refs.sort_by_key(|r| r.number);
    refs
}

/// Render the bibliography section as an HTML string appended to the article body.
/// Structure: div.wiki-references > h2#references > ol.references > li#cite-N
pub fn render_bibliography(references: &[Reference]) -> String {
    if references.is_empty() { return String::new(); }
    let mut html = String::from(
        r#"<div class="wiki-references"><h2 id="references">References</h2><ol class="references">"#
    );
    for r in references {
        let back = format!(r#"<a href="#{}" class="ref-back">↑</a>"#, r.anchor);
        let cite = match &r.url {
            Some(url) => format!(r#"{back}. <a href="{url}" class="ref-external" rel="noopener">{}</a>"#, r.text),
            None      => format!("{back}. {}", r.text),
        };
        html.push_str(&format!(r#"<li id="cite-{}">{cite}</li>"#, r.number));
    }
    html.push_str("</ol></div>");
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bibliography_excludes_unused_refs() {
        let defs = vec![
            ReferenceDef { id: 1, text: "A".into(), url: None, internal: false, path: None },
            ReferenceDef { id: 2, text: "B".into(), url: None, internal: false, path: None },
        ];
        let mut anchors = InlineAnchors::new();
        anchors.insert("1".into(), "ref-cite-1".into());
        let bib = build_bibliography(&defs, &anchors);
        assert_eq!(bib.len(), 1);
        assert_eq!(bib[0].number, 1);
    }

    #[test]
    fn bibliography_sorted_by_number() {
        let defs = vec![
            ReferenceDef { id: 2, text: "B".into(), url: None, internal: false, path: None },
            ReferenceDef { id: 1, text: "A".into(), url: None, internal: false, path: None },
        ];
        let mut anchors = InlineAnchors::new();
        anchors.insert("1".into(), "ref-cite-1".into());
        anchors.insert("2".into(), "ref-cite-2".into());
        let bib = build_bibliography(&defs, &anchors);
        assert_eq!(bib[0].number, 1);
        assert_eq!(bib[1].number, 2);
    }

    #[test]
    fn render_bibliography_has_back_arrows() {
        let refs = vec![Reference {
            number: 1, text: "Citation.".into(), url: Some("https://example.com".into()),
            internal: false, anchor: "ref-cite-1".into(),
        }];
        let html = render_bibliography(&refs);
        assert!(html.contains("ref-back"));
        assert!(html.contains('↑'));
        assert!(html.contains(r#"id="cite-1""#));
        assert!(html.contains(r#"href="#ref-cite-1""#));
    }
}
