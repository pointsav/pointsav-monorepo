use anyhow::{anyhow, Result};
use lbug::{Connection, Database, SystemConfig, Value};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEntity {
    pub entity_name: String,
    pub classification: String, // Person|Company|Project|Account|Location
    pub role_vector: Option<String>,
    pub location_vector: Option<String>,
    pub contact_vector: Option<String>,
    pub module_id: String,
    pub confidence: f64,
    /// Fine-grained type tag within a classification bucket. Defaults to
    /// empty string for backward-compatible deserialization. Sprint 2
    /// (P2-2.2) — enables RelatedTo edge traversal by type.
    #[serde(default)]
    pub node_type: String,
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
    /// Write a directed RelatedTo edge between two entities by ID. Idempotent.
    fn write_related_to(&self, from_id: &str, to_id: &str, relation_type: &str) -> Result<()>;
    /// Returns true if a CORPUS file (identified by its worm_id, e.g. the bare UUID
    /// portion of `CORPUS_<worm_id>.json`) has already been fully extracted into the
    /// graph — i.e. at least one non-Source entity exists whose `worm_id` field
    /// equals `source_worm_id`. Used by the corpus drain loop to skip re-extraction
    /// after a service restart without relying on the in-memory HashSet.
    fn is_already_processed(&self, source_worm_id: &str) -> Result<bool>;
    /// Flush the WAL to the main database file. Call after bulk write operations
    /// (taxonomy upsert, corpus drain) to prevent WAL accumulation that causes
    /// slow startup replay. Non-fatal: callers should warn on error but continue.
    fn checkpoint(&self) -> Result<()>;
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
            .max_db_size(256 * 1024 * 1024)
            // Checkpoint when WAL reaches 4 MB (C++ default is 16 MB — too large for our
            // write pattern; an 11 MB un-checkpointed WAL caused a 45-min replay hang).
            .checkpoint_threshold(4 * 1024 * 1024);
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
                cites_json STRING, \
                node_type STRING\
            )",
        )
        .map_err(|e| anyhow!("init_schema Entity table failed: {}", e))?;

        // Idempotent migrations — suppress errors when columns already exist.
        let _ = conn.query("ALTER TABLE Entity ADD worm_id STRING DEFAULT ''");
        let _ = conn.query("ALTER TABLE Entity ADD cites_json STRING DEFAULT ''");
        let _ = conn.query("ALTER TABLE Entity ADD node_type STRING DEFAULT ''");

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
                     e.cites_json = $cites_json, \
                     e.node_type = $node_type",
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
            let cites_json =
                serde_json::to_string(&entity.cites).unwrap_or_else(|_| "[]".to_string());

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
                    (
                        "worm_id",
                        Value::String(entity.worm_id.clone().unwrap_or_default()),
                    ),
                    ("cites_json", Value::String(cites_json)),
                    ("node_type", Value::String(entity.node_type.clone())),
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
                        e.worm_id, e.cites_json, e.node_type \
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
                        e.worm_id, e.cites_json, e.node_type",
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

    fn write_related_to(&self, from_id: &str, to_id: &str, relation_type: &str) -> Result<()> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "MATCH (a:Entity {id: $from_id}), (b:Entity {id: $to_id}) \
                 MERGE (a)-[r:RelatedTo]->(b) \
                 SET r.relation_type = $relation_type",
            )
            .map_err(|e| anyhow!("write_related_to prepare failed: {}", e))?;
        conn.execute(
            &mut stmt,
            vec![
                ("from_id", Value::String(from_id.to_string())),
                ("to_id", Value::String(to_id.to_string())),
                ("relation_type", Value::String(relation_type.to_string())),
            ],
        )
        .map_err(|e| anyhow!("write_related_to execute failed: {}", e))?;
        Ok(())
    }

    fn is_already_processed(&self, source_worm_id: &str) -> Result<bool> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "MATCH (e:Entity) \
                 WHERE e.worm_id = $worm_id AND e.classification <> 'Source' \
                 RETURN e.entity_name \
                 LIMIT 1",
            )
            .map_err(|e| anyhow!("is_already_processed prepare failed: {}", e))?;
        let result = conn
            .execute(
                &mut stmt,
                vec![("worm_id", Value::String(source_worm_id.to_string()))],
            )
            .map_err(|e| anyhow!("is_already_processed execute failed: {}", e))?;
        Ok(result.into_iter().next().is_some())
    }

    fn checkpoint(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.query("CHECKPOINT")
            .map(|_| ())
            .map_err(|e| anyhow!("lbug CHECKPOINT failed: {}", e))
    }
}

// ── SqliteGraphStore ─────────────────────────────────────────────────────────

/// SQLite-backed `GraphStore` for Micro ($7/mo) nodes.
///
/// Uses the same logical schema as `LbugGraphStore` (identical column set) so
/// entities are portable between backends. `rusqlite` is compiled with the
/// `bundled` feature — no runtime dependency on a system SQLite.
///
/// Selected at startup when `foundry_nodeclass::detect()` returns `Micro`, or
/// when `SERVICE_CONTENT_GRAPH_BACKEND=sqlite` is set explicitly.
pub struct SqliteGraphStore {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl SqliteGraphStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = rusqlite::Connection::open(db_path)
            .map_err(|e| anyhow!("Failed to open SQLite graph store at {}: {}", db_path, e))?;
        // WAL mode: readers don't block writers; NORMAL sync: safe on a single node.
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")
            .map_err(|e| anyhow!("SQLite PRAGMA setup failed: {}", e))?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

impl GraphStore for SqliteGraphStore {
    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entity (
                id              TEXT PRIMARY KEY,
                entity_name     TEXT NOT NULL,
                classification  TEXT NOT NULL,
                role_vector     TEXT NOT NULL DEFAULT '',
                location_vector TEXT NOT NULL DEFAULT '',
                contact_vector  TEXT NOT NULL DEFAULT '',
                module_id       TEXT NOT NULL,
                confidence      REAL NOT NULL DEFAULT 0.0,
                created_at      TEXT NOT NULL DEFAULT '',
                worm_id         TEXT NOT NULL DEFAULT '',
                cites_json      TEXT NOT NULL DEFAULT '[]',
                node_type       TEXT NOT NULL DEFAULT ''
            );
            CREATE TABLE IF NOT EXISTS related_to (
                from_id         TEXT NOT NULL,
                to_id           TEXT NOT NULL,
                relation_type   TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (from_id, to_id, relation_type)
            );
            CREATE INDEX IF NOT EXISTS idx_entity_module
                ON entity(module_id);
            CREATE INDEX IF NOT EXISTS idx_entity_module_cls
                ON entity(module_id, classification);",
        )
        .map_err(|e| anyhow!("init_schema failed: {}", e))?;
        // Idempotent migration for databases created before Sprint 2.
        let _ =
            conn.execute_batch("ALTER TABLE entity ADD COLUMN node_type TEXT NOT NULL DEFAULT ''");
        Ok(())
    }

    fn upsert_entities(&self, module_id: &str, entities: &[GraphEntity]) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        let mut count = 0usize;

        for entity in entities {
            let id = format!(
                "{}__{}",
                module_id,
                entity.entity_name.to_lowercase().replace(' ', "_")
            );
            let cites_json =
                serde_json::to_string(&entity.cites).unwrap_or_else(|_| "[]".to_string());

            // created_at is set only on first INSERT; ON CONFLICT leaves it unchanged.
            conn.execute(
                "INSERT INTO entity(
                    id, entity_name, classification, role_vector,
                    location_vector, contact_vector, module_id, confidence,
                    created_at, worm_id, cites_json, node_type)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                 ON CONFLICT(id) DO UPDATE SET
                    entity_name     = excluded.entity_name,
                    classification  = excluded.classification,
                    role_vector     = excluded.role_vector,
                    location_vector = excluded.location_vector,
                    contact_vector  = excluded.contact_vector,
                    module_id       = excluded.module_id,
                    confidence      = excluded.confidence,
                    worm_id         = excluded.worm_id,
                    cites_json      = excluded.cites_json,
                    node_type       = excluded.node_type",
                params![
                    id,
                    entity.entity_name,
                    entity.classification,
                    entity.role_vector.as_deref().unwrap_or(""),
                    entity.location_vector.as_deref().unwrap_or(""),
                    entity.contact_vector.as_deref().unwrap_or(""),
                    entity.module_id,
                    entity.confidence,
                    now,
                    entity.worm_id.as_deref().unwrap_or(""),
                    cites_json,
                    entity.node_type.as_str(),
                ],
            )
            .map_err(|e| anyhow!("Failed to upsert entity '{}': {}", entity.entity_name, e))?;

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
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{}%", query.to_lowercase());

        let mut stmt = conn
            .prepare(
                "SELECT entity_name, classification, role_vector, location_vector,
                        contact_vector, module_id, confidence, worm_id, cites_json,
                        node_type
                 FROM entity
                 WHERE module_id = ?1 AND lower(entity_name) LIKE ?2
                 LIMIT ?3",
            )
            .map_err(|e| anyhow!("query_context prepare failed: {}", e))?;

        let rows = stmt
            .query_map(
                params![module_id, pattern, limit as i64],
                sqlite_row_to_entity,
            )
            .map_err(|e| anyhow!("query_context execute failed: {}", e))?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| anyhow!("query_context row mapping failed: {}", e))
    }

    fn list_entities(&self, module_id: &str) -> Result<Vec<GraphEntity>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT entity_name, classification, role_vector, location_vector,
                        contact_vector, module_id, confidence, worm_id, cites_json,
                        node_type
                 FROM entity
                 WHERE module_id = ?1",
            )
            .map_err(|e| anyhow!("list_entities prepare failed: {}", e))?;

        let rows = stmt
            .query_map(params![module_id], sqlite_row_to_entity)
            .map_err(|e| anyhow!("list_entities execute failed: {}", e))?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| anyhow!("list_entities row mapping failed: {}", e))
    }

    fn count_all(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM entity", [], |r| r.get(0))
            .map_err(|e| anyhow!("count_all failed: {}", e))?;
        Ok(n as usize)
    }

    fn delete_by_classification(&self, module_id: &str, classification: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let n = conn
            .execute(
                "DELETE FROM entity WHERE module_id = ?1 AND classification = ?2",
                params![module_id, classification],
            )
            .map_err(|e| anyhow!("delete_by_classification failed: {}", e))?;
        Ok(n)
    }

    fn delete_by_classification_and_location(
        &self,
        module_id: &str,
        classification: &str,
        location: &str,
    ) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let n = conn
            .execute(
                "DELETE FROM entity
                 WHERE module_id = ?1 AND classification = ?2 AND location_vector = ?3",
                params![module_id, classification, location],
            )
            .map_err(|e| anyhow!("delete_by_classification_and_location failed: {}", e))?;
        Ok(n)
    }

    fn write_related_to(&self, from_id: &str, to_id: &str, relation_type: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO related_to(from_id, to_id, relation_type)
             VALUES (?1, ?2, ?3)",
            params![from_id, to_id, relation_type],
        )
        .map_err(|e| anyhow!("write_related_to failed: {}", e))?;
        Ok(())
    }

    fn is_already_processed(&self, source_worm_id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM entity \
                 WHERE worm_id = ?1 AND classification != 'Source'",
                params![source_worm_id],
                |r| r.get(0),
            )
            .map_err(|e| anyhow!("is_already_processed failed: {}", e))?;
        Ok(n > 0)
    }

    fn checkpoint(&self) -> Result<()> {
        // SQLite WAL checkpoints via PRAGMA wal_checkpoint on close; no manual call needed.
        Ok(())
    }
}

fn sqlite_row_to_entity(row: &rusqlite::Row<'_>) -> rusqlite::Result<GraphEntity> {
    let role: String = row.get(2)?;
    let loc: String = row.get(3)?;
    let contact: String = row.get(4)?;
    let worm: String = row.get(7)?;
    let cites_json: String = row.get(8)?;
    let node_type: String = row.get(9).unwrap_or_default();
    Ok(GraphEntity {
        entity_name: row.get(0)?,
        classification: row.get(1)?,
        role_vector: if role.is_empty() { None } else { Some(role) },
        location_vector: if loc.is_empty() { None } else { Some(loc) },
        contact_vector: if contact.is_empty() {
            None
        } else {
            Some(contact)
        },
        module_id: row.get(5)?,
        confidence: row.get(6)?,
        node_type,
        worm_id: if worm.is_empty() { None } else { Some(worm) },
        cites: serde_json::from_str(&cites_json).unwrap_or_default(),
    })
}

// ── LbugGraphStore helpers ────────────────────────────────────────────────────

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
/// Each row yields 10 columns in RETURN order:
/// 0 entity_name, 1 classification, 2 role_vector, 3 location_vector,
/// 4 contact_vector, 5 module_id, 6 confidence, 7 worm_id, 8 cites_json,
/// 9 node_type
///
/// Columns 7-9 are optional — absent in legacy rows; default to
/// None / empty Vec / "" for backward compat.
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
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
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
        let node_type = if row.len() >= 10 {
            val_to_string(&row[9])
        } else {
            String::new()
        };

        out.push(GraphEntity {
            entity_name,
            classification,
            role_vector,
            location_vector,
            contact_vector,
            module_id,
            confidence,
            node_type,
            worm_id,
            cites,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory_store() -> SqliteGraphStore {
        let store = SqliteGraphStore::new(":memory:").expect("open :memory:");
        store.init_schema().expect("init_schema");
        store
    }

    fn entity(name: &str, cls: &str, module: &str) -> GraphEntity {
        GraphEntity {
            entity_name: name.to_string(),
            classification: cls.to_string(),
            role_vector: Some(format!("{} role", name)),
            location_vector: Some("Vancouver".to_string()),
            contact_vector: None,
            module_id: module.to_string(),
            confidence: 0.9,
            node_type: "TestType".to_string(),
            worm_id: Some("CORPUS_TEST".to_string()),
            cites: vec!["cite-1".to_string()],
        }
    }

    #[test]
    fn round_trip_upsert_and_list() {
        let store = in_memory_store();
        let entities = vec![
            entity("Alice", "Person", "mod-a"),
            entity("Acme Corp", "Company", "mod-a"),
        ];
        let n = store.upsert_entities("mod-a", &entities).unwrap();
        assert_eq!(n, 2);

        let listed = store.list_entities("mod-a").unwrap();
        assert_eq!(listed.len(), 2);
        let names: Vec<&str> = listed.iter().map(|e| e.entity_name.as_str()).collect();
        assert!(names.contains(&"Alice"));
        assert!(names.contains(&"Acme Corp"));
    }

    #[test]
    fn upsert_idempotent() {
        let store = in_memory_store();
        let e = vec![entity("Alice", "Person", "mod-a")];
        store.upsert_entities("mod-a", &e).unwrap();
        store.upsert_entities("mod-a", &e).unwrap();
        assert_eq!(store.count_all().unwrap(), 1);
    }

    #[test]
    fn count_all_across_modules() {
        let store = in_memory_store();
        store
            .upsert_entities("mod-a", &[entity("Alice", "Person", "mod-a")])
            .unwrap();
        store
            .upsert_entities(
                "mod-b",
                &[
                    entity("Bob", "Person", "mod-b"),
                    entity("Carol", "Person", "mod-b"),
                ],
            )
            .unwrap();
        assert_eq!(store.count_all().unwrap(), 3);
    }

    #[test]
    fn list_entities_scoped_to_module() {
        let store = in_memory_store();
        store
            .upsert_entities("mod-a", &[entity("Alice", "Person", "mod-a")])
            .unwrap();
        store
            .upsert_entities("mod-b", &[entity("Bob", "Person", "mod-b")])
            .unwrap();
        let a = store.list_entities("mod-a").unwrap();
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].entity_name, "Alice");
    }

    #[test]
    fn query_context_returns_matches() {
        let store = in_memory_store();
        store
            .upsert_entities(
                "mod-a",
                &[
                    entity("Alice", "Person", "mod-a"),
                    entity("Bob", "Person", "mod-a"),
                ],
            )
            .unwrap();
        let results = store.query_context("mod-a", "Alice", 10).unwrap();
        assert!(
            !results.is_empty(),
            "query_context should return at least one result"
        );
        assert!(results.iter().any(|e| e.entity_name == "Alice"));
    }

    #[test]
    fn delete_by_classification_removes_correct_entities() {
        let store = in_memory_store();
        store
            .upsert_entities(
                "mod-a",
                &[
                    entity("Alice", "Person", "mod-a"),
                    entity("Acme Corp", "Company", "mod-a"),
                ],
            )
            .unwrap();
        store.delete_by_classification("mod-a", "Person").unwrap();
        let remaining = store.list_entities("mod-a").unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].classification, "Company");
    }

    #[test]
    fn delete_by_classification_and_location_scoped() {
        let store = in_memory_store();
        let mut e_vancouver = entity("Alice", "Person", "mod-a");
        e_vancouver.location_vector = Some("Vancouver".to_string());
        let mut e_toronto = entity("Bob", "Person", "mod-a");
        e_toronto.location_vector = Some("Toronto".to_string());
        store
            .upsert_entities("mod-a", &[e_vancouver, e_toronto])
            .unwrap();
        store
            .delete_by_classification_and_location("mod-a", "Person", "Vancouver")
            .unwrap();
        let remaining = store.list_entities("mod-a").unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].entity_name, "Bob");
    }

    #[test]
    fn worm_id_and_cites_preserved() {
        let store = in_memory_store();
        let mut e = entity("Alice", "Person", "mod-a");
        e.worm_id = Some("CORPUS_01ABCDEF".to_string());
        e.cites = vec![
            "cite-doctrine-49".to_string(),
            "cite-doctrine-54".to_string(),
        ];
        store.upsert_entities("mod-a", &[e]).unwrap();
        let listed = store.list_entities("mod-a").unwrap();
        assert_eq!(listed[0].worm_id.as_deref(), Some("CORPUS_01ABCDEF"));
        assert_eq!(
            listed[0].cites,
            vec!["cite-doctrine-49", "cite-doctrine-54"]
        );
    }

    #[test]
    fn node_type_preserved() {
        let store = in_memory_store();
        let mut e = entity("Alice", "Person", "mod-a");
        e.node_type = "LegalEntity".to_string();
        store.upsert_entities("mod-a", &[e]).unwrap();
        let listed = store.list_entities("mod-a").unwrap();
        assert_eq!(listed[0].node_type, "LegalEntity");
    }

    #[test]
    fn write_related_to_idempotent() {
        let store = in_memory_store();
        store
            .upsert_entities("mod-a", &[entity("Alice", "Person", "mod-a")])
            .unwrap();
        store
            .upsert_entities("mod-a", &[entity("Acme Corp", "Company", "mod-a")])
            .unwrap();
        let from_id = "mod-a__alice";
        let to_id = "mod-a__acme_corp";
        store.write_related_to(from_id, to_id, "employs").unwrap();
        store.write_related_to(from_id, to_id, "employs").unwrap(); // idempotent
    }

    #[test]
    fn is_already_processed_detects_extracted_entities() {
        let store = in_memory_store();
        // Source node alone does not count as "processed".
        let mut source = entity("CORPUS_TESTABC", "Source", "mod-a");
        source.worm_id = None;
        store.upsert_entities("mod-a", &[source]).unwrap();
        assert!(!store.is_already_processed("TESTABC").unwrap());

        // Once a non-Source entity with the worm_id is written, it counts.
        let mut extracted = entity("Acme Corp", "Company", "mod-a");
        extracted.worm_id = Some("TESTABC".to_string());
        store.upsert_entities("mod-a", &[extracted]).unwrap();
        assert!(store.is_already_processed("TESTABC").unwrap());

        // Unrelated worm_id returns false.
        assert!(!store.is_already_processed("OTHERID").unwrap());
    }
}
