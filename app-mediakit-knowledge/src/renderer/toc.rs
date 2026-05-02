use pulldown_cmark::Event;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: u8,
    pub id: String,
    pub title: String,
    pub children: Vec<TocEntry>,
}

/// Walk the pulldown-cmark event stream, extract ## and ### headings into a
/// TOC tree, inject `<span id="...">` anchors, and return the modified stream.
///
/// TODO: implement this function.
///
/// Algorithm:
///   let mut in_heading: Option<u8> = None;
///   let mut heading_text = String::new();
///
///   for event in events {
///       match &event {
///           Event::Start(Tag::Heading { level, .. }) if *level == HeadingLevel::H2
///                                                    || *level == HeadingLevel::H3 => {
///               in_heading = Some(match level { H2 => 2, _ => 3 });
///               heading_text.clear();
///               out.push(event);
///           }
///           Event::Text(t) if in_heading.is_some() => {
///               heading_text.push_str(t);
///               out.push(event);
///           }
///           Event::End(TagEnd::Heading(_)) if in_heading.is_some() => {
///               let level = in_heading.take().unwrap();
///               let id    = slugify(&heading_text);
///               // Inject anchor span so TOC sidebar links resolve correctly
///               out.push(Event::Html(format!("<span id=\"{id}\"></span>").into()));
///               insert_entry(&mut toc, TocEntry { level, id, title: heading_text.clone(), children: vec![] }, level);
///               out.push(event);
///           }
///           _ => out.push(event),
///       }
///   }
pub fn extract<'a>(
    events: impl Iterator<Item = Event<'a>>,
) -> (Vec<TocEntry>, Vec<Event<'a>>) {
    let mut toc: Vec<TocEntry> = Vec::new();
    let mut out: Vec<Event<'a>> = Vec::new();

    // Placeholder: pass-through until implemented.
    for event in events {
        out.push(event);
    }

    (toc, out)
}

fn insert_entry(toc: &mut Vec<TocEntry>, entry: TocEntry, level: u8) {
    if level == 2 {
        toc.push(entry);
    } else if let Some(parent) = toc.last_mut() {
        parent.children.push(entry);
    } else {
        toc.push(entry); // orphan h3 — promote to root
    }
}

pub fn slugify(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("The Services Layer"), "the-services-layer");
        assert_eq!(slugify("ToteboxOS (`os-totebox`)"), "toteboxos-os-totebox");
    }

    #[test]
    fn insert_nesting() {
        let mut toc = vec![];
        let e = |t: &str, l: u8| TocEntry { level: l, id: t.into(), title: t.into(), children: vec![] };
        insert_entry(&mut toc, e("s1", 2), 2);
        insert_entry(&mut toc, e("s1-1", 3), 3);
        insert_entry(&mut toc, e("s2", 2), 2);
        assert_eq!(toc.len(), 2);
        assert_eq!(toc[0].children.len(), 1);
    }
}
