use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use tantivy::{
    collector::TopDocs, doc,
    query::QueryParser,
    schema::{Schema, STORED, TEXT},
    Index, TantivyDocument,
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    /// ~160 character excerpt surrounding the first query match.
    pub excerpt: String,
}

struct Fields {
    slug:  tantivy::schema::Field,
    title: tantivy::schema::Field,
    body:  tantivy::schema::Field,
}

/// In-RAM Tantivy index over all Markdown files in the content directory.
/// Rebuilt from scratch at startup and after each git sync advance.
pub struct SearchIndex {
    index:  Index,
    fields: Fields,
}

impl SearchIndex {
    pub fn build(content_path: &Path) -> Result<Self> {
        let mut sb = Schema::builder();
        let slug_f  = sb.add_text_field("slug",  STORED);
        let title_f = sb.add_text_field("title", TEXT | STORED);
        let body_f  = sb.add_text_field("body",  TEXT);
        let schema  = sb.build();

        let index  = Index::create_in_ram(schema);
        let mut w  = index.writer(50_000_000).context("tantivy writer")?;

        let fields = Fields { slug: slug_f, title: title_f, body: body_f };

        for entry in WalkDir::new(content_path)
            .follow_links(false).into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |x| x == "md"))
        {
            if let Err(e) = index_file(&mut w, &fields, entry.path()) {
                tracing::warn!(path = %entry.path().display(), error = %e, "skipping");
            }
        }

        w.commit().context("tantivy commit")?;
        Ok(Self { index, fields })
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let reader   = self.index.reader().context("tantivy reader")?;
        let searcher = reader.searcher();
        let mut qp   = QueryParser::for_index(&self.index, vec![self.fields.title, self.fields.body]);
        qp.set_field_boost(self.fields.title, 2.0);
        let query    = qp.parse_query(query_str).context("query parse")?;
        let hits     = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut out = Vec::with_capacity(hits.len());
        for (_score, addr) in hits {
            let doc: TantivyDocument = searcher.doc(addr)?;
            let slug  = doc.get_first(self.fields.slug).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let title = doc.get_first(self.fields.title).and_then(|v| v.as_str()).unwrap_or("").to_string();
            // TODO: generate proper excerpt — find match position in body, extract 160-char window.
            out.push(SearchResult { slug, title, excerpt: String::new() });
        }
        Ok(out)
    }

    /// Title-only prefix search for wikilink autocomplete.
    /// TODO: replace with a proper Tantivy prefix/regex query on the title field.
    pub fn autocomplete(&self, partial: &str, limit: usize) -> Result<Vec<(String, String)>> {
        Ok(self.search(partial, limit)?.into_iter().map(|r| (r.slug, r.title)).collect())
    }
}

fn index_file(w: &mut tantivy::IndexWriter, f: &Fields, path: &Path) -> Result<()> {
    let raw    = std::fs::read_to_string(path)?;
    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let parsed = matter.parse(&raw);

    let (slug, title) = if let Some(data) = parsed.data {
        (data["slug"].as_str().unwrap_or("").to_string(),
         data["title"].as_str().unwrap_or("").to_string())
    } else {
        (path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string(), String::new())
    };

    w.add_document(doc!(f.slug => slug, f.title => title, f.body => parsed.content))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn builds_and_queries() {
        let tmp = TempDir::new().unwrap();
        let mut file = std::fs::File::create(tmp.path().join("test.md")).unwrap();
        file.write_all(b"---\ntitle: \"ToteboxOS\"\nslug: os-totebox\ncategory: architecture\n---\nThe archive OS.\n").unwrap();
        let idx = SearchIndex::build(tmp.path()).unwrap();
        let res = idx.search("archive", 10).unwrap();
        assert!(!res.is_empty());
        assert_eq!(res[0].slug, "os-totebox");
    }
}
