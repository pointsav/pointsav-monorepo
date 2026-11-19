// SPDX-License-Identifier: Apache-2.0

//! Versioned prompts and output schemas for model-backed jobs. Callers name
//! a `prompt_id` / `schema_id`; the Doorman records the sha256 of what ran.
//! The `/v1/extract` prompt is deliberately separate and unchanged.

use serde_json::{json, Value};

use super::sha256_hex;

pub struct Prompt {
    pub id: &'static str,
    pub text: &'static str,
    /// Output is `{entities:[{class,value,witness}]}` and is validated by
    /// verbatim-witness check (and chunked when long).
    pub witness: bool,
}

pub struct Schema {
    pub id: &'static str,
    pub json: fn() -> Value,
}

pub const DEFAULT_PROMPT_ID: &str = "entities-v1";
pub const DEFAULT_SCHEMA_ID: &str = "entities-v1";

static PROMPTS: &[Prompt] = &[
    Prompt {
        id: "entities-v1",
        text: "Extract named entities from the user's text. Return only JSON matching the \
schema. Use only text that appears verbatim in the input; do not infer or invent. \
If there are no entities, return an empty list.",
        witness: false,
    },
    Prompt {
        id: "entities-v2",
        text: "Extract entities from the user's text. Classes: Person, Company, \
EmailAddress, Location, Account, BookTitle, EventDate. For each entity return its \
class, a normalised value, and a witness: the exact characters copied from the input \
that mention it (for an EventDate, the date exactly as written, with the year if \
present). Use only text that appears verbatim in the input; do not infer or invent. \
Return only JSON matching the schema. If there are no entities, return an empty list.",
        witness: true,
    },
    // The span extractor has no prompt; this id names the contract (and its
    // hash pins it) so records stay uniform. Output is validated like v2.
    Prompt {
        id: "spans-v1",
        text: "span-extractor: labels come from the engine's own ontology; no prompt",
        witness: true,
    },
];

fn entities_v1_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "entities": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "class": {"type": "string"},
                        "value": {"type": "string"}
                    },
                    "required": ["class", "value"]
                }
            }
        },
        "required": ["entities"]
    })
}

fn entities_v2_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "entities": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "class": {"type": "string", "enum": super::witness::ENTITY_CLASSES},
                        "value": {"type": "string"},
                        "witness": {"type": "string"}
                    },
                    "required": ["class", "value", "witness"]
                }
            }
        },
        "required": ["entities"]
    })
}

static SCHEMAS: &[Schema] = &[
    Schema {
        id: "entities-v1",
        json: entities_v1_schema,
    },
    Schema {
        id: "entities-v2",
        json: entities_v2_schema,
    },
    Schema {
        id: "spans-v1",
        json: entities_v2_schema,
    },
];

pub fn prompt(id: &str) -> Option<&'static Prompt> {
    PROMPTS.iter().find(|p| p.id == id)
}

pub fn schema(id: &str) -> Option<&'static Schema> {
    SCHEMAS.iter().find(|s| s.id == id)
}

pub fn prompt_sha256(p: &Prompt) -> String {
    sha256_hex(p.text.as_bytes())
}

/// Hash of the canonical serialisation of the schema.
pub fn schema_sha256(s: &Schema) -> String {
    sha256_hex(
        serde_json::to_string(&(s.json)())
            .unwrap_or_default()
            .as_bytes(),
    )
}
