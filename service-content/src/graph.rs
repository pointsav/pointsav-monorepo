use anyhow::{anyhow, Result};
use lbug::{Connection, Database, SystemConfig, Value};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEntity {
    pub entity_name: String,
    pub classification: String, // Person|Company|Project|Account|Location
    pub role_vector: Option<String>,
    pub location_vector: Option<String>,
    pub contact_vector: Option<String>,
    pub module_id: String,
    pub confidence: f64,
    /// WORM ledger ID of the CORPUS file this entity was extracted from
    /// (e.g. `CORPUS_01J9Q...`). Phase 2 of
    /// learning-loop-master-plan-2026-05-18.md (P2-2.1) — provenance for
    /// every entity so editorial citations can be traced back to source.
    /// Optional for backward compatibility with entities ingested before
    /// 2026-05-18; new extractions MUST populate this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worm_id: Option<String>,
    /// Citation IDs from `/srv/foundry/citations.yaml` that ground this
    /// entity. Resolved against the canonical citation registry. Phase 2
    /// of learning-loop-master-plan-2026-05-18.md (P2-2.1) — required for
    /// citation-faithful editorial output via /v1/editorial/seed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cites: Vec<String>,
}

pub trait GraphStore: Send + Sync {
    fn init_schema(&self) -> Result<()>;
    fn upsert_entities(&self, module_id: &str, entities: &[GraphEntity]) -> Result<usize>;
    fn query_context(&self, module_id: &str, query: &str, limit: usize)
        -> Result<Vec<GraphEntity>>;
    #[allow(dead_code)]
    fn list_entities(&self, module_id: &str) -> Result<Vec<GraphEntity>>;
    /// Count all entities across all module_ids. Used by healthz for monitoring.
    fn count_all(&self) -> Result<usize>;
    /// Delete all entities matching module_id + classification. Returns count deleted.
    fn delete_by_classification(&self, module_id: &str, classification: &str) -> Result<usize>;
    /// Delete entities matching module_id + classification + location_vector (used for
    /// per-domain glossary/topic reloads where classification is shared across domains).
    fn delete_by_classification_and_location(
        &self,
        module_id: &str,
        classification: &str,
        location: &str,
    ) -> Result<usize>;
}

pub struct LbugGraphStore {
    db: Arc<Database>,
}

impl LbugGraphStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let buffer_pool_bytes: u64 = std::env::var("SERVICE_CONTENT_LBUG_BUFFER_POOL_MB")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(64)
            * 1024
            * 1024;
        let config = SystemConfig::default()
            .buffer_pool_size(buffer_pool_bytes)
            .max_num_threads(2)
            .max_db_size(256 * 1024 * 1024);
        let db = Database::new(db_path, config)
            .map_err(|e| anyhow!("Failed to open LadybugDB at {}: {}", db_path, e))?;
        Ok(Self { db: Arc::new(db) })
    }

    /// Create a new connection from the stored database reference.
    /// Connection borrows `&Database`; since `Arc<Database>` keeps the Database alive
    /// and we hold a reference for the duration of the call, this is safe.
    fn conn(&self) -> Result<Connection<'_>> {
        Connection::new(&self.db).map_err(|e| anyhow!("Failed to create DB connection: {}", e))
    }
}

impl GraphStore for LbugGraphStore {
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn()?;
        // Note: P2-2.1 (2026-05-18) adds `worm_id` and `cites_json` columns
        // for entity provenance and citation grounding. `IF NOT EXISTS` keeps
        // pre-existing tables compatible; new columns gain a default of empty
        // string for existing rows (handled at query time by val_to_string).
        // A future migration step (P2-2.1-followup) populates worm_id for
        // legacy rows from the CORPUS ledger replay.
        conn.query(
            "CREATE NODE TABLE IF NOT EXISTS Entity(\
                id STRING PRIMARY KEY, \
                entity_name STRING, \
                classification STRING, \
                role_vector STRING, \
                location_vector STRING, \
                contact_vector STRING, \
                module_id STRING, \
                confidence DOUBLE, \
                created_at STRING, \
                worm_id STRING, \
                cites_json STRING\
            )",
        )
        .map_err(|e| anyhow!("init_schema Entity table failed: {}", e))?;

        conn.query(
            "CREATE REL TABLE IF NOT EXISTS RelatedTo(\
                FROM Entity TO Entity, \
                relation_type STRING\
            )",
        )
        .map_err(|e| anyhow!("init_schema RelatedTo table failed: {}", e))?;

        Ok(())
    }

    fn upsert_entities(&self, module_id: &str, entities: &[GraphEntity]) -> Result<usize> {
        let conn = self.conn()?;

        // Two prepared statements per batch:
        // 1. MERGE updates all mutable fields; created_at excluded so it is never overwritten.
        // 2. A follow-up SET fills created_at only when the field is still empty (newly created node).
        let mut stmt_merge = conn
            .prepare(
                "MERGE (e:Entity {id: $id}) \
                 SET e.entity_name = $entity_name, \
                     e.classification = $classification, \
                     e.role_vector = $role_vector, \
                     e.location_vector = $location_vector, \
                     e.contact_vector = $contact_vector, \
                     e.module_id = $module_id, \
                     e.confidence = $confidence, \
                     e.worm_id = $worm_id, \
                     e.cites_json = $cites_json",
            )
            .map_err(|e| anyhow!("Failed to prepare upsert statement: {}", e))?;

        // Set created_at only when the field is empty (i.e., newly created node).
        let mut stmt_init_ts = conn
            .prepare(
                "MATCH (e:Entity) WHERE e.id = $id AND e.created_at = '' \
                 SET e.created_at = $created_at",
            )
            .map_err(|e| anyhow!("Failed to prepare created_at init statement: {}", e))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut count = 0usize;

        for entity in entities {
            let id = format!(
                "{}__{}",
                module_id,
                entity.entity_name.to_lowercase().replace(' ', "_")
            );

            // Serialise `cites` to JSON string for LadybugDB scalar column.
            // Empty vec → "[]" so query-side parsing is uniform.
            let cites_json = serde_json::to_string(&entity.cites)
                .unwrap_or_else(|_| "[]".to_string());

            conn.execute(
                &mut stmt_merge,
                vec![
                    ("id", Value::String(id.clone())),
                    ("entity_name", Value::String(entity.entity_name.clone())),
                    (
                        "classification",
                        Value::String(entity.classification.clone()),
                    ),
                    (
                        "role_vector",
                        Value::String(entity.role_vector.clone().unwrap_or_default()),
                    ),
                    (
                        "location_vector",
                        Value::String(entity.location_vector.clone().unwrap_or_default()),
                    ),
                    (
                        "contact_vector",
                        Value::String(entity.contact_vector.clone().unwrap_or_default()),
                    ),
                    ("module_id", Value::String(entity.module_id.clone())),
                    ("confidence", Value::Double(entity.confidence)),
                    ("worm_id", Value::String(entity.worm_id.clone().unwrap_or_default())),
                    ("cites_json", Value::String(cites_json)),
                ],
            )
            .map_err(|e| anyhow!("Failed to upsert entity '{}': {}", entity.entity_name, e))?;

            // Non-fatal if the init-timestamp step fails — the entity was still upserted.
            let _ = conn.execute(
                &mut stmt_init_ts,
                vec![
                    ("id", Value::String(id)),
                    ("created_at", Value::String(now.clone())),
                ],
            );

            count += 1;
        }

        Ok(count)
    }

    fn query_context(
        &self,
        module_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<GraphEntity>> {
        let conn = self.conn()?;
        let q_lower = query.to_lowercase();

        let mut stmt = conn
            .prepare(
                "MATCH (e:Entity) \
                 WHERE e.module_id = $module_id \
                   AND lower(e.entity_name) CONTAINS $query \
                 RETURN e.entity_name, e.classification, e.role_vector, \
                        e.location_vector, e.contact_vector, e.module_id, e.confidence, \
                        e.worm_id, e.cites_json \
                 LIMIT $limit",
            )
            .map_err(|e| anyhow!("Failed to prepare query_context statement: {}", e))?;

        let result = conn
            .execute(
                &mut stmt,
                vec![
                    ("module_id", Value::String(module_id.to_string())),
                    ("query", Value::String(q_lower)),
                    ("limit", Value::Int64(limit as i64)),
                ],
            )
            .map_err(|e| anyhow!("Failed to execute query_context: {}", e))?;

        rows_to_entities(result)
    }

    fn list_entities(&self, module_id: &str) -> Result<Vec<GraphEntity>> {
        let conn = self.conn()?;

        let mut stmt = conn
            .prepare(
                "MATCH (e:Entity) \
                 WHERE e.module_id = $module_id \
                 RETURN e.entity_name, e.classification, e.role_vector, \
                        e.location_vector, e.contact_vector, e.module_id, e.confidence, \
                        e.worm_id, e.cites_json",
            )
            .map_err(|e| anyhow!("Failed to prepare list_entities statement: {}", e))?;

        let result = conn
            .execute(
                &mut stmt,
                vec![("module_id", Value::String(module_id.to_string()))],
            )
            .map_err(|e| anyhow!("Failed to execute list_entities: {}", e))?;

        rows_to_entities(result)
    }

    fn count_all(&self) -> Result<usize> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("MATCH (e:Entity) RETURN e.entity_name")
            .map_err(|e| anyhow!("count_all prepare failed: {}", e))?;
        let result = conn
            .execute(&mut stmt, vec![])
            .map_err(|e| anyhow!("count_all execute failed: {}", e))?;
        let mut count = 0usize;
        for _ in result {
            count += 1;
        }
        Ok(count)
    }

    fn delete_by_classification(&self, module_id: &str, classification: &str) -> Result<usize> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "MATCH (e:Entity) \
                 WHERE e.module_id = $module_id AND e.classification = $cls \
                 DELETE e",
            )
            .map_err(|e| anyhow!("Failed to prepare delete_by_classification: {}", e))?;
        conn.execute(
            &mut stmt,
            vec![
                ("module_id", Value::String(module_id.to_string())),
                ("cls", Value::String(classification.to_string())),
            ],
        )
        .map_err(|e| anyhow!("Failed to execute delete_by_classification: {}", e))?;
        Ok(0)
    }

    fn delete_by_classification_and_location(
        &self,
        module_id: &str,
        classification: &str,
        location: &str,
    ) -> Result<usize> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "MATCH (e:Entity) \
                 WHERE e.module_id = $module_id \
                   AND e.classification = $cls \
                   AND e.location_vector = $loc \
                 DELETE e",
            )
            .map_err(|e| {
                anyhow!(
                    "Failed to prepare delete_by_classification_and_location: {}",
                    e
                )
            })?;
        conn.execute(
            &mut stmt,
            vec![
                ("module_id", Value::String(module_id.to_string())),
                ("cls", Value::String(classification.to_string())),
                ("loc", Value::String(location.to_string())),
            ],
        )
        .map_err(|e| {
            anyhow!(
                "Failed to execute delete_by_classification_and_location: {}",
                e
            )
        })?;
        Ok(0)
    }
}

/// Extract a `String` from a `Value::String`, or return empty string.
fn val_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => String::new(),
    }
}

/// Extract an `f64` from `Value::Double` or `Value::Float`, or return 0.0.
fn val_to_f64(v: &Value) -> f64 {
    match v {
        Value::Double(f) => *f,
        Value::Float(f) => *f as f64,
        _ => 0.0,
    }
}

/// Convert a `QueryResult` iterator into `Vec<GraphEntity>`.
/// Each row yields 9 columns in RETURN order:
/// 0 entity_name, 1 classification, 2 role_vector, 3 location_vector,
/// 4 contact_vector, 5 module_id, 6 confidence, 7 worm_id, 8 cites_json
///
/// P2-2.1 (2026-05-18) — worm_id + cites_json columns added for provenance
/// and citation grounding. Rows with fewer than 9 columns are pre-2026-05-18
/// schema versions; columns 7-8 default to None/empty for those rows.
fn rows_to_entities(result: lbug::QueryResult<'_>) -> Result<Vec<GraphEntity>> {
    let mut out = Vec::new();
    for row in result {
        if row.len() < 7 {
            continue;
        }
        let entity_name = val_to_string(&row[0]);
        let classification = val_to_string(&row[1]);
        let role_vector = {
            let s = val_to_string(&row[2]);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        let location_vector = {
            let s = val_to_string(&row[3]);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        let contact_vector = {
            let s = val_to_string(&row[4]);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        };
        let module_id = val_to_string(&row[5]);
        let confidence = val_to_f64(&row[6]);
        // worm_id + cites are optional — present in 9-column rows, absent
        // in legacy 7-column rows. Default to None / empty Vec.
        let worm_id = if row.len() >= 8 {
            let s = val_to_string(&row[7]);
            if s.is_empty() { None } else { Some(s) }
        } else {
            None
        };
        let cites: Vec<String> = if row.len() >= 9 {
            let json = val_to_string(&row[8]);
            if json.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&json).unwrap_or_default()
            }
        } else {
            Vec::new()
        };

        out.push(GraphEntity {
            entity_name,
            classification,
            role_vector,
            location_vector,
            contact_vector,
            module_id,
            confidence,
            worm_id,
            cites,
        });
    }
    Ok(out)
}
