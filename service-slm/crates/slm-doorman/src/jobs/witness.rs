// SPDX-License-Identifier: Apache-2.0

//! Span-witness validation and chunking for entity-mining jobs.
//!
//! The model proposes `{class, value, witness}`; the Doorman accepts an
//! entity only if `witness` occurs verbatim in the input, and derives the
//! span itself (UTF-8 byte offsets into the full input). An entity with no
//! exact match is rejected and counted — never repaired.

use serde::Deserialize;

use super::types::{Entity, Span};

pub const ENTITY_CLASSES: &[&str] = &[
    "Person",
    "Company",
    "EmailAddress",
    "Location",
    "Account",
    "BookTitle",
    "EventDate",
];

#[derive(Deserialize)]
struct Proposed {
    entities: Vec<ProposedEntity>,
}

#[derive(Deserialize)]
struct ProposedEntity {
    class: String,
    value: String,
    witness: String,
}

/// Validate one chunk's model output. `chunk_offset` is the chunk's byte
/// offset in the full input. Returns accepted entities and the rejected count.
pub fn validate_chunk(
    chunk: &str,
    chunk_offset: usize,
    model_json: &str,
) -> Result<(Vec<Entity>, u32), String> {
    let proposed: Proposed = serde_json::from_str(model_json.trim())
        .map_err(|e| format!("unparseable-model-output: {e}"))?;
    let mut accepted: Vec<Entity> = Vec::new();
    let mut rejected = 0u32;
    for p in proposed.entities {
        let class_ok = ENTITY_CLASSES.contains(&p.class.as_str());
        let found = if p.witness.is_empty() || p.value.trim().is_empty() || !class_ok {
            None
        } else {
            chunk.find(&p.witness)
        };
        match found {
            Some(rel) => {
                let start = chunk_offset + rel;
                let e = Entity {
                    class: p.class,
                    value: p.value,
                    witness: p.witness.clone(),
                    span: Span {
                        start,
                        end: start + p.witness.len(),
                    },
                };
                if !accepted.contains(&e) {
                    accepted.push(e);
                }
            }
            None => rejected += 1,
        }
    }
    Ok((accepted, rejected))
}

/// Split `text` into `(byte_offset, chunk)` pieces of at most `chunk_bytes`
/// bytes, overlapping by about `overlap` bytes, always on char boundaries and
/// preferring to break at whitespace. A text that fits is one chunk.
pub fn split_chunks(text: &str, chunk_bytes: usize, overlap: usize) -> Vec<(usize, &str)> {
    let chunk_bytes = chunk_bytes.max(16);
    let overlap = overlap.min(chunk_bytes / 2);
    if text.len() <= chunk_bytes {
        return vec![(0, text)];
    }
    let floor = |mut i: usize| {
        while i > 0 && !text.is_char_boundary(i) {
            i -= 1;
        }
        i
    };
    let ceil = |mut i: usize| {
        while i < text.len() && !text.is_char_boundary(i) {
            i += 1;
        }
        i
    };
    let mut out = Vec::new();
    let mut start = 0usize;
    loop {
        let mut end = floor((start + chunk_bytes).min(text.len()));
        if end < text.len() {
            // Prefer a whitespace break in the last quarter of the chunk.
            let window_start = floor(start + chunk_bytes * 3 / 4);
            if let Some(ws) = text[window_start..end].rfind(char::is_whitespace) {
                end = window_start + ws;
            }
        }
        out.push((start, &text[start..end]));
        if end >= text.len() {
            break;
        }
        let next = ceil(end.saturating_sub(overlap));
        start = if next > start { next } else { end };
    }
    out
}

/// Agreement between two engines' validated entity sets, keyed by
/// `(class, span)`; values are not compared (two engines may normalise a
/// value differently for the same witnessed span).
#[derive(Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct Comparison {
    pub agreed: Vec<Entity>,
    pub only_a: Vec<Entity>,
    pub only_b: Vec<Entity>,
}

pub fn compare(a: &[Entity], b: &[Entity]) -> Comparison {
    let key = |e: &Entity| (e.class.clone(), e.span.start, e.span.end);
    let mut out = Comparison::default();
    for e in a {
        if b.iter().any(|o| key(o) == key(e)) {
            out.agreed.push(e.clone());
        } else {
            out.only_a.push(e.clone());
        }
    }
    for e in b {
        if !a.iter().any(|o| key(o) == key(e)) {
            out.only_b.push(e.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_exact_witness_and_derives_span() {
        let chunk = "Alice read Dune on 3 March 1998.";
        let json = r#"{"entities":[
            {"class":"Person","value":"Alice","witness":"Alice"},
            {"class":"BookTitle","value":"Dune","witness":"Dune"},
            {"class":"EventDate","value":"1998-03-03","witness":"3 March 1998"}]}"#;
        let (ents, rej) = validate_chunk(chunk, 100, json).unwrap();
        assert_eq!(rej, 0);
        assert_eq!(ents.len(), 3);
        assert_eq!(
            ents[0].span,
            Span {
                start: 100,
                end: 105
            }
        );
        let d = &ents[2];
        assert_eq!(&chunk[d.span.start - 100..d.span.end - 100], "3 March 1998");
    }

    #[test]
    fn rejects_invented_unknown_class_and_empty() {
        let chunk = "Alice met Bob.";
        let json = r#"{"entities":[
            {"class":"Person","value":"Carol","witness":"Carol"},
            {"class":"Person","value":"Bob","witness":"bob"},
            {"class":"Planet","value":"Alice","witness":"Alice"},
            {"class":"Person","value":"","witness":"Alice"},
            {"class":"Person","value":"Alice","witness":""},
            {"class":"Person","value":"Alice","witness":"Alice"}]}"#;
        let (ents, rej) = validate_chunk(chunk, 0, json).unwrap();
        assert_eq!(ents.len(), 1);
        assert_eq!(rej, 5);
    }

    #[test]
    fn unparseable_output_is_an_error() {
        assert!(validate_chunk("x", 0, "not json").is_err());
        assert!(validate_chunk("x", 0, r#"{"nope":1}"#).is_err());
    }

    #[test]
    fn duplicates_collapse() {
        let json = r#"{"entities":[
            {"class":"Person","value":"Alice","witness":"Alice"},
            {"class":"Person","value":"Alice","witness":"Alice"}]}"#;
        assert_eq!(validate_chunk("Alice", 0, json).unwrap().0.len(), 1);
    }

    #[test]
    fn chunks_cover_text_overlap_and_respect_char_boundaries() {
        let text = "héllo wörld ".repeat(200); // multibyte chars
        let chunks = split_chunks(&text, 100, 20);
        assert!(chunks.len() > 5);
        let mut covered_to = 0usize;
        for (off, c) in &chunks {
            assert!(c.len() <= 100);
            assert!(text.is_char_boundary(*off));
            assert_eq!(&text[*off..*off + c.len()], *c);
            assert!(*off <= covered_to, "gap before chunk at {off}");
            covered_to = off + c.len();
        }
        assert_eq!(covered_to, text.len());
        assert_eq!(split_chunks("short", 100, 20), vec![(0, "short")]);
    }

    #[test]
    fn compare_splits_agreement_and_disagreement_by_class_and_span() {
        let e = |c: &str, v: &str, s: usize| Entity {
            class: c.into(),
            value: v.into(),
            witness: v.into(),
            span: Span {
                start: s,
                end: s + v.len(),
            },
        };
        let a = vec![e("Person", "Alice", 0), e("Location", "Paris", 20)];
        let b = vec![
            Entity {
                value: "ALICE-normalised".into(),
                ..e("Person", "Alice", 0)
            },
            e("Person", "Paris", 20),
        ];
        let c = compare(&a, &b);
        assert_eq!(
            c.agreed.len(),
            1,
            "same class+span agrees even if value differs"
        );
        assert_eq!(c.only_a.len(), 1, "Paris is Location in a, Person in b");
        assert_eq!(c.only_b.len(), 1);
    }
}
