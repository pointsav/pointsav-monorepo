//! service-search — the one production search service for the Totebox monorepo.
//!
//! Architecture (per `BRIEF-workplace-comprehensive-search`, ratified 2026-07-16):
//! ONE search, three parts with no overlap —
//!
//! - **`moonshot-index`** (owned) — the trigram substring **correctness floor**: the
//!   no-silent-miss guarantee a token index structurally cannot provide.
//! - **Tantivy** (vendored, MIT) — the **BM25 ranking** + on-disk persistence + the
//!   stored body used for snippets and content verification.
//! - **`service-search`** (this crate) — **fuses** them: the Forge builds both indexes
//!   to disk and exits; the Strike loads them read-only and answers queries. Other
//!   `service-*` and the workbench log into the Strike over HTTP — none embed an index.
//!
//! Memory model: the Strike holds only the trigram postings + filenames (small) and
//! mmaps Tantivy. The content copy lives once, in Tantivy's stored `body` field; trigram
//! content candidates are verified by reading that stored field back — never from a RAM
//! copy, never from a second filesystem pass.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, RwLock};

use serde::{Deserialize, Serialize};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter, TantivyDocument, Term};

/// Filename band: trigram floor keyed by path (one entry per file, no content).
pub const NAMES_FILE: &str = "names.msix";
/// Content band: trigram floor keyed by BLAKE3 content-OID (deduped — identical bodies across
/// the 23 template-heavy clones share ONE entry).
pub const CONTENTS_FILE: &str = "contents.msix";
/// `oid\tpath` per line — reconstructs both `path→oid` and `oid→paths` at load.
pub const PATHS_FILE: &str = "paths.tsv";
pub const TANTIVY_DIR: &str = "tantivy";
/// Per-file `(mtime,size)` watermark, persisted so the reconciliation sweep has a baseline
/// across restarts (and can catch changes that happened while the Strike was down).
pub const MTIMES_FILE: &str = "mtimes.tsv";

/// BLAKE3 content identity as lowercase hex. Empty content → empty string sentinel (no content
/// band entry): filename-only files (over the size cap, binary, unreadable) never share a body.
fn content_oid(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    blake3::hash(content.as_bytes()).to_hex().to_string()
}
/// Cap the fused result set — v1 shows the top hits per band, not the whole corpus.
pub const MAX_HITS: usize = 200;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// One indexable root. `url_prefix` qualifies doc ids so they don't collide across
/// roots (the trigram index ids are otherwise root-relative). `fs_path` is the tree.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootSpec {
    pub url_prefix: String,
    pub fs_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Where the Forge writes and the Strike reads the two indexes.
    pub index_path: String,
    /// Roots to index.
    pub roots: Vec<RootSpec>,
    /// Directory basenames to prune entirely (e.g. target, node_modules, vendor).
    #[serde(default)]
    pub exclude_dirs: Vec<String>,
    /// TCP bind for the Strike, e.g. "127.0.0.1:9310".
    #[serde(default = "default_bind")]
    pub bind: String,
    /// Per-file content cap (bytes); larger files are indexed by name only.
    #[serde(default = "default_max_bytes")]
    pub max_content_bytes: usize,
}

fn default_bind() -> String {
    "127.0.0.1:9310".to_string()
}
fn default_max_bytes() -> usize {
    moonshot_index::DEFAULT_MAX_CONTENT_BYTES
}

impl Config {
    pub fn from_toml_path(p: impl AsRef<Path>) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(p)?;
        toml::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }
    pub fn index_path(&self) -> PathBuf {
        PathBuf::from(&self.index_path)
    }
    pub fn is_excluded_dir(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| self.exclude_dirs.iter().any(|e| e == n))
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Result types (the Strike's JSON contract)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    /// Root-qualified id, e.g. `_clones/project-x/src/foo.rs`.
    pub id: String,
    /// Filename / path shown to the user.
    pub name: String,
    /// Excerpt around the first content match (empty for filename-only hits).
    pub snippet: String,
    /// BM25 relevance when the hit came from the ranked layer; 0.0 for a
    /// guarantee-only hit that Tantivy's tokenizer did not surface.
    pub score: f32,
}

/// Two bands + a coverage line — the v1 UX contract.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResponse {
    pub query: String,
    /// Matches in filenames / paths.
    pub filenames: Vec<Hit>,
    /// Matches in file contents.
    pub contents: Vec<Hit>,
    /// "Searched N files across R roots" — the trustworthy-zero coverage line.
    pub files_indexed: usize,
    pub roots_indexed: usize,
}

// ---------------------------------------------------------------------------
// Tantivy schema — shared by Forge (write) and Strike (read)
// ---------------------------------------------------------------------------

pub struct Fields {
    /// The BLAKE3 content-OID — the dedup key. One Tantivy doc per distinct body.
    pub id: tantivy::schema::Field,
    pub body: tantivy::schema::Field,
}

pub fn build_schema() -> (Schema, Fields) {
    let mut b = Schema::builder();
    // id = content-OID: stored, exact (not tokenized) — the dedup/join key. Filenames are NOT
    // in Tantivy (they live in the path-keyed `names` trigram); Tantivy holds only deduped bodies.
    let id = b.add_text_field("id", STRING | STORED);
    // body: tokenized for BM25 content search, stored so we can verify trigram candidates and cut
    // snippets without touching the filesystem again.
    let body = b.add_text_field("body", TEXT | STORED);
    let schema = b.build();
    (schema, Fields { id, body })
}

// ---------------------------------------------------------------------------
// Forge — build both indexes to disk, then return (caller exits to free RAM)
// ---------------------------------------------------------------------------

pub struct ForgeStats {
    pub files: usize,
    pub content_skipped: usize,
    pub roots: usize,
    /// Distinct content bodies actually indexed (files − duplicates). The dedup dividend.
    pub distinct_bodies: usize,
}

/// Build the two trigram floors + the OID-keyed Tantivy index for every root and persist them.
/// Filenames go in the path-keyed `names` floor; each distinct body goes ONCE into the OID-keyed
/// `contents` floor + Tantivy (identical files across the clones dedup to one body).
pub fn forge(config: &Config) -> anyhow::Result<ForgeStats> {
    use std::collections::HashSet;
    use std::io::BufWriter;

    let out = config.index_path();
    std::fs::create_dir_all(&out)?;
    let tv_dir = out.join(TANTIVY_DIR);
    // Fresh build: clear any prior Tantivy segments so ids never duplicate.
    if tv_dir.exists() {
        std::fs::remove_dir_all(&tv_dir)?;
    }
    std::fs::create_dir_all(&tv_dir)?;

    let (schema, f) = build_schema();
    let index = Index::create_in_dir(&tv_dir, schema)?;
    // 200 MB writer heap → Tantivy parallelizes indexing across its own threads.
    let mut writer: IndexWriter = index.writer(200_000_000)?;

    let mut names =
        moonshot_index::TrigramIndex::with_max_content_bytes(config.max_content_bytes);
    let mut contents =
        moonshot_index::TrigramIndex::with_max_content_bytes(config.max_content_bytes);
    let mut indexed_oids: HashSet<String> = HashSet::new();
    // oid\tpath lines for the persisted map (also the reconciliation reverse index).
    let mut paths_map: Vec<(String, String)> = Vec::new();
    let mut stats = ForgeStats {
        files: 0,
        content_skipped: 0,
        roots: config.roots.len(),
        distinct_bodies: 0,
    };

    for root in &config.roots {
        let base = PathBuf::from(&root.fs_path);
        if !base.is_dir() {
            eprintln!("forge: skipping missing root {}", root.fs_path);
            continue;
        }
        let mut stack = vec![base.clone()];
        while let Some(dir) = stack.pop() {
            let entries = match std::fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let ft = match entry.file_type() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                if ft.is_symlink() {
                    continue;
                }
                if ft.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name == ".git" || config.is_excluded_dir(&path) {
                        continue;
                    }
                    stack.push(path);
                    continue;
                }
                if !ft.is_file() {
                    continue;
                }
                // Root-qualified id so paths never collide across roots.
                let rel = path.strip_prefix(&base).unwrap_or(&path).to_string_lossy();
                let id = format!("{}/{}", root.url_prefix, rel);

                let over_cap = entry
                    .metadata()
                    .map(|m| m.len() as usize > config.max_content_bytes)
                    .unwrap_or(true);
                let content = if over_cap {
                    stats.content_skipped += 1;
                    String::new()
                } else {
                    match std::fs::read(&path) {
                        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                        Err(_) => {
                            stats.content_skipped += 1;
                            String::new()
                        }
                    }
                };

                // Filename band: one entry per path, no content.
                names.add_document(id.clone(), id.clone(), "");

                let oid = content_oid(&content);
                if !oid.is_empty() && indexed_oids.insert(oid.clone()) {
                    // First time we see this body — index it ONCE (name empty; content only).
                    contents.add_document(oid.clone(), String::new(), &content);
                    writer.add_document(doc!(
                        f.id => oid.clone(),
                        f.body => content,
                    ))?;
                    stats.distinct_bodies += 1;
                }
                paths_map.push((oid, id));
                stats.files += 1;
            }
        }
    }

    writer.commit()?;

    // Persist both floors (lite) + the oid↔path map.
    let mut w = BufWriter::new(std::fs::File::create(out.join(NAMES_FILE))?);
    names.save_lite(&mut w)?;
    std::io::Write::flush(&mut w)?;
    let mut w = BufWriter::new(std::fs::File::create(out.join(CONTENTS_FILE))?);
    contents.save_lite(&mut w)?;
    std::io::Write::flush(&mut w)?;
    save_paths_map(&out.join(PATHS_FILE), &paths_map)?;

    Ok(stats)
}

/// Write the `oid\tpath` map (atomic rename). Paths never contain a tab or newline in practice;
/// any that would are skipped (reconcile re-seeds them harmlessly).
fn save_paths_map(path: &Path, pairs: &[(String, String)]) -> anyhow::Result<()> {
    use std::io::Write;
    let tmp = path.with_extension("tsv.tmp");
    {
        let mut w = std::io::BufWriter::new(std::fs::File::create(&tmp)?);
        for (oid, p) in pairs {
            if p.contains('\t') || p.contains('\n') {
                continue;
            }
            writeln!(w, "{oid}\t{p}")?;
        }
        w.flush()?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Strike — load both indexes read-only and answer queries
// ---------------------------------------------------------------------------

/// A filesystem change to apply to the live index.
pub enum Change {
    /// Create or modify — read the file and (re)index it in both bands.
    Upsert(PathBuf),
    /// Delete — drop it from both bands.
    Delete(PathBuf),
}

/// The live, self-updating search server, content-addressed. Two trigram floors — `names`
/// (path-keyed, filename band) and `contents` (BLAKE3-OID-keyed, deduped content band) — plus an
/// OID-keyed Tantivy for BM25 + stored bodies, and the `path↔oid` map that ties them together.
/// Shared as `Arc<Strike>`; all mutation goes through interior locks so `search`/`apply` overlap.
pub struct Strike {
    names: RwLock<moonshot_index::TrigramIndex>, // key = path; filename band
    contents: RwLock<moonshot_index::TrigramIndex>, // key = oid; deduped content band
    index: Index,
    writer: Mutex<IndexWriter>,
    reader: IndexReader, // one persistent reader; auto-reloads on commit (OnCommitWithDelay)
    fields: Fields,
    config: Config,
    /// path -> content-OID (empty-string oid = filename-only, no body indexed).
    path_to_oid: Mutex<HashMap<String, String>>,
    /// content-OID -> the set of paths sharing that body (the dedup refcount + display map).
    oid_paths: Mutex<HashMap<String, std::collections::HashSet<String>>>,
    files_indexed: AtomicUsize, // = number of paths (files), not distinct bodies
    dirty: AtomicUsize,         // pending uncommitted Tantivy ops
    /// path -> (mtime_secs, size) — the watermark the reconciliation sweep diffs against.
    mtimes: Mutex<HashMap<String, (u64, u64)>>,
}

impl Strike {
    /// Load the content-addressed index and open it for live updates. Reads the two persisted
    /// trigram floors + the `path↔oid` map; rebuilds the `contents` floor from Tantivy if its
    /// sidecar is missing (Tantivy is the durable source of truth for bodies).
    pub fn load(config: &Config) -> anyhow::Result<Self> {
        use std::io::BufReader;
        let out = config.index_path();
        let index = Index::open_in_dir(out.join(TANTIVY_DIR))?;
        let (_schema, fields) = build_schema();

        let names = {
            let mut r = BufReader::new(std::fs::File::open(out.join(NAMES_FILE))?);
            moonshot_index::TrigramIndex::load_lite(&mut r)?
        };
        let contents_path = out.join(CONTENTS_FILE);
        let contents = if contents_path.exists() {
            let mut r = BufReader::new(std::fs::File::open(&contents_path)?);
            moonshot_index::TrigramIndex::load_lite(&mut r)?
        } else {
            trigram_from_tantivy(&index, &fields, config.max_content_bytes)?
        };

        // Rebuild path_to_oid + oid_paths from the persisted map.
        let (path_to_oid, oid_paths) = load_paths_map(&out.join(PATHS_FILE))?;

        let reader = index.reader()?; // persistent, auto-reloading
        let writer = index.writer(200_000_000)?;
        let files_indexed = path_to_oid.len();
        // Restore the reconciliation watermark if a prior run persisted one. Absent → the first
        // reconcile seeds it from disk instead of reparsing everything.
        let mtimes = load_mtimes(&out.join(MTIMES_FILE)).unwrap_or_default();

        Ok(Strike {
            names: RwLock::new(names),
            contents: RwLock::new(contents),
            index,
            writer: Mutex::new(writer),
            reader,
            fields,
            config: config.clone(),
            path_to_oid: Mutex::new(path_to_oid),
            oid_paths: Mutex::new(oid_paths),
            files_indexed: AtomicUsize::new(files_indexed),
            dirty: AtomicUsize::new(0),
            mtimes: Mutex::new(mtimes),
        })
    }

    pub fn files_indexed(&self) -> usize {
        self.files_indexed.load(Ordering::Relaxed)
    }

    /// Apply one filesystem change. The content-addressing dividend lives here: a body whose OID
    /// is already indexed costs NO body work (dedup / free rename → just a path→oid edge); an
    /// unchanged body (touch/chmod) costs nothing at all.
    pub fn apply(&self, change: Change) -> anyhow::Result<()> {
        match change {
            Change::Upsert(path) => {
                let id = match self.qualify(&path) {
                    Some(id) => id,
                    None => return Ok(()),
                };
                let (content, meta) = read_capped(&path, self.config.max_content_bytes);
                let new_oid = content_oid(&content);
                let old_oid = self.path_to_oid.lock().unwrap().get(&id).cloned();

                // Filename band: idempotent — creates on first sight, no-op on modify.
                self.names.write().unwrap().upsert(&id, &id, "");

                if old_oid.as_deref() == Some(new_oid.as_str()) {
                    // Content identity unchanged (metadata-only touch) — nothing to reindex.
                    if let Some(m) = meta {
                        self.mtimes.lock().unwrap().insert(id, m);
                    }
                    return Ok(());
                }

                // Content changed (or new file): detach the old body, attach the new.
                if let Some(oid) = old_oid {
                    self.detach_oid(&id, &oid)?;
                }
                self.path_to_oid.lock().unwrap().insert(id.clone(), new_oid.clone());
                if !new_oid.is_empty() {
                    let first_ref = {
                        let mut op = self.oid_paths.lock().unwrap();
                        let set = op.entry(new_oid.clone()).or_default();
                        let was_empty = set.is_empty();
                        set.insert(id.clone());
                        was_empty
                    };
                    if first_ref {
                        // Body not yet indexed anywhere — index it ONCE.
                        let w = self.writer.lock().unwrap();
                        w.delete_term(Term::from_field_text(self.fields.id, &new_oid));
                        w.add_document(doc!(
                            self.fields.id => new_oid.clone(),
                            self.fields.body => content.clone(),
                        ))?;
                        drop(w);
                        self.contents.write().unwrap().upsert(&new_oid, "", &content);
                        self.dirty.fetch_add(1, Ordering::Relaxed);
                    }
                    // else: dedup / rename hit — the body is already indexed; just the edge above.
                }
                if let Some(m) = meta {
                    self.mtimes.lock().unwrap().insert(id.clone(), m);
                }
                self.files_indexed
                    .store(self.path_to_oid.lock().unwrap().len(), Ordering::Relaxed);
            }
            Change::Delete(path) => {
                let id = match self.qualify(&path) {
                    Some(id) => id,
                    None => return Ok(()),
                };
                self.remove_id(&id)?;
            }
        }
        Ok(())
    }

    /// Remove one path from both bands (and its body if it was the last reference). Shared by
    /// `apply(Delete)` and the reconciliation delete pass.
    fn remove_id(&self, id: &str) -> anyhow::Result<()> {
        self.names.write().unwrap().remove(id);
        let old_oid = self.path_to_oid.lock().unwrap().remove(id);
        if let Some(oid) = old_oid {
            self.detach_oid(id, &oid)?;
        }
        self.mtimes.lock().unwrap().remove(id);
        self.files_indexed
            .store(self.path_to_oid.lock().unwrap().len(), Ordering::Relaxed);
        Ok(())
    }

    /// Drop `id`'s reference to `oid`; if that was the body's last path, tombstone the body in
    /// both the content floor and Tantivy. Empty-oid (filename-only) is a no-op.
    fn detach_oid(&self, id: &str, oid: &str) -> anyhow::Result<()> {
        if oid.is_empty() {
            return Ok(());
        }
        let orphaned = {
            let mut op = self.oid_paths.lock().unwrap();
            if let Some(set) = op.get_mut(oid) {
                set.remove(id);
                if set.is_empty() {
                    op.remove(oid);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if orphaned {
            self.writer
                .lock()
                .unwrap()
                .delete_term(Term::from_field_text(self.fields.id, oid));
            self.contents.write().unwrap().remove(oid);
            self.dirty.fetch_add(1, Ordering::Relaxed);
        }
        Ok(())
    }

    /// Commit pending Tantivy ops (the reader auto-reloads → content hits become visible) and
    /// persist the trigram floors + maps so a restart is O(load), not O(re-forge). Filename hits
    /// were already live. Returns whether anything was committed.
    pub fn commit_if_dirty(&self) -> anyhow::Result<bool> {
        if self.dirty.load(Ordering::Relaxed) == 0 {
            return Ok(false);
        }
        self.writer.lock().unwrap().commit()?;
        self.dirty.store(0, Ordering::Relaxed);
        self.files_indexed
            .store(self.path_to_oid.lock().unwrap().len(), Ordering::Relaxed);
        Ok(true)
    }

    /// Reconciliation sweep — the backstop for inotify event loss, and the sole mechanism that
    /// catches changes made while the Strike was down. Metadata-only: stat-walk every root, diff
    /// `(mtime,size)` against the watermark, and re-read content ONLY for files that are new or
    /// genuinely changed. O(N stat) ~1-2s over 88K files, not an O(N parse) rebuild.
    ///
    /// First run against a freshly-forged index (empty watermark) SEEDS the watermark from disk
    /// without reindexing — the index is already current as of the forge, so a blanket upsert of
    /// every file would be wasted work.
    pub fn reconcile(&self) -> anyhow::Result<ReconcileStats> {
        let seeding = self.mtimes.lock().unwrap().is_empty();
        let mut stats = ReconcileStats {
            seeding,
            ..Default::default()
        };
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

        for root in &self.config.roots {
            let base = PathBuf::from(&root.fs_path);
            if !base.is_dir() {
                continue;
            }
            let mut stack = vec![base.clone()];
            while let Some(dir) = stack.pop() {
                let entries = match std::fs::read_dir(&dir) {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                for entry in entries.flatten() {
                    let path = entry.path();
                    let ft = match entry.file_type() {
                        Ok(t) => t,
                        Err(_) => continue,
                    };
                    if ft.is_symlink() {
                        continue;
                    }
                    if ft.is_dir() {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        if name == ".git" || self.config.is_excluded_dir(&path) {
                            continue;
                        }
                        stack.push(path);
                        continue;
                    }
                    if !ft.is_file() {
                        continue;
                    }
                    let rel = path.strip_prefix(&base).unwrap_or(&path).to_string_lossy();
                    let id = format!("{}/{}", root.url_prefix, rel);
                    seen.insert(id.clone());
                    stats.scanned += 1;

                    let meta = match entry.metadata() {
                        Ok(m) => m,
                        Err(_) => continue,
                    };
                    let mtime = meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let wm = (mtime, meta.len());

                    if seeding {
                        // Trust the freshly-forged index; just record the baseline, no reindex.
                        self.mtimes.lock().unwrap().insert(id, wm);
                        stats.seeded += 1;
                        continue;
                    }
                    // Drop the lock before apply() (which re-takes it) to avoid deadlock.
                    let prior = self.mtimes.lock().unwrap().get(&id).copied();
                    if prior != Some(wm) {
                        self.apply(Change::Upsert(path))?;
                        stats.upserted += 1;
                    }
                }
            }
        }

        if !seeding {
            // Anything in the watermark we did NOT see on disk has vanished — drop it directly
            // (we already hold the id, no need to round-trip through a path).
            let gone: Vec<String> = {
                let mt = self.mtimes.lock().unwrap();
                mt.keys().filter(|k| !seen.contains(*k)).cloned().collect()
            };
            for id in gone {
                // Guard against a transient dir-read error masquerading as a deletion: `stat`
                // needs only `x` on the parent (not `r`), so it still resolves a file whose
                // directory this sweep failed to `read_dir`. Only drop the entry if the path
                // genuinely no longer exists — never merely because the walk couldn't see it.
                if let Some(p) = self.id_to_path(&id) {
                    if p.symlink_metadata().is_ok() {
                        continue; // still on disk; the walk just missed it — keep it
                    }
                }
                self.remove_id(&id)?;
                stats.deleted += 1;
            }
        }

        self.commit_if_dirty()?;
        // Persist the floors + maps + watermark so a restart is O(load), not O(re-forge).
        if let Err(e) = self.persist() {
            eprintln!("strike: could not persist index state: {e}");
        }
        Ok(stats)
    }

    /// Persist both trigram floors, the `oid↔path` map, and the reconciliation watermark next to
    /// the index. Called after each sweep so the on-disk sidecars track live state.
    fn persist(&self) -> anyhow::Result<()> {
        use std::io::{BufWriter, Write};
        let out = self.config.index_path();
        {
            let mut w = BufWriter::new(std::fs::File::create(out.join(NAMES_FILE))?);
            self.names.read().unwrap().save_lite(&mut w)?;
            w.flush()?;
        }
        {
            let mut w = BufWriter::new(std::fs::File::create(out.join(CONTENTS_FILE))?);
            self.contents.read().unwrap().save_lite(&mut w)?;
            w.flush()?;
        }
        let pairs: Vec<(String, String)> = self
            .path_to_oid
            .lock()
            .unwrap()
            .iter()
            .map(|(p, o)| (o.clone(), p.clone()))
            .collect();
        save_paths_map(&out.join(PATHS_FILE), &pairs)?;
        self.save_mtimes()?;
        Ok(())
    }

    /// Persist the reconciliation watermark next to the index (atomic rename). Lines are
    /// `mtime\tsize\tpath`; paths never contain a tab or newline in practice (any that would are
    /// skipped — reconcile re-seeds them next run, harmlessly).
    fn save_mtimes(&self) -> anyhow::Result<()> {
        use std::io::Write;
        let path = self.config.index_path().join(MTIMES_FILE);
        let tmp = path.with_extension("tsv.tmp");
        {
            let mt = self.mtimes.lock().unwrap();
            let mut w = std::io::BufWriter::new(std::fs::File::create(&tmp)?);
            for (id, (mtime, size)) in mt.iter() {
                if id.contains('\t') || id.contains('\n') {
                    continue;
                }
                writeln!(w, "{mtime}\t{size}\t{id}")?;
            }
            w.flush()?;
        }
        std::fs::rename(&tmp, &path)?;
        Ok(())
    }

    /// Map an absolute filesystem path → root-qualified id (`<url_prefix>/<rel>`), or None if
    /// it isn't under a configured root or lies in an excluded directory.
    pub fn qualify(&self, path: &Path) -> Option<String> {
        for comp in path.components() {
            if let std::path::Component::Normal(c) = comp {
                if let Some(s) = c.to_str() {
                    if self.config.exclude_dirs.iter().any(|e| e == s) {
                        return None;
                    }
                }
            }
        }
        for root in &self.config.roots {
            if let Ok(rel) = path.strip_prefix(Path::new(&root.fs_path)) {
                let rel = rel.to_string_lossy();
                if rel.is_empty() {
                    return None;
                }
                return Some(format!("{}/{}", root.url_prefix, rel));
            }
        }
        None
    }

    /// Reverse of `qualify`: root-qualified id → absolute filesystem path, or None if no root's
    /// `url_prefix` matches. Used only to confirm a file is truly gone before a reconcile delete.
    fn id_to_path(&self, id: &str) -> Option<PathBuf> {
        for root in &self.config.roots {
            let prefix = format!("{}/", root.url_prefix);
            if let Some(rel) = id.strip_prefix(&prefix) {
                return Some(Path::new(&root.fs_path).join(rel));
            }
        }
        None
    }

    /// Two bands. Filename band = the path-keyed `names` floor. Content band = the OID-keyed
    /// `contents` floor ∪ Tantivy BM25, verified against the stored body, then **expanded across
    /// every path sharing that body** (dedup means one indexed body can back many files). Recall
    /// is never traded for ranking: every trigram-verified hit is returned.
    pub fn search(&self, query: &str) -> anyhow::Result<SearchResponse> {
        use std::collections::HashSet;
        const PATHS_PER_BODY: usize = 25; // cap the fan-out of one shared body

        let searcher = self.reader.searcher(); // persistent reader; sees latest commit
        let ql = query.to_lowercase();

        // ── Filename band — path-keyed trigram floor ─────────────────────────
        let mut filenames: Vec<Hit> = Vec::new();
        for cand in self.names.read().unwrap().candidate_ids(query) {
            if cand.name_matches {
                // Rank basename matches above mid-path matches; shorter paths first (tie-break).
                let basename = cand.id.rsplit('/').next().unwrap_or(&cand.id);
                let score = if basename.to_lowercase().contains(&ql) { 1.0 } else { 0.5 };
                filenames.push(Hit {
                    id: cand.id.clone(),
                    name: cand.id,
                    snippet: String::new(),
                    score,
                });
            }
        }

        // ── Content band — OID-keyed: Tantivy BM25 ∪ trigram floor, verified, then path-expanded ─
        // 1) Ranked layer — Tantivy BM25 over body → oid -> (score, body).
        let parser = QueryParser::for_index(&self.index, vec![self.fields.body]);
        let mut ranked: HashMap<String, (f32, String)> = HashMap::new();
        if let Ok(q) = parser.parse_query(query) {
            let top = searcher.search(&q, &TopDocs::with_limit(MAX_HITS))?;
            for (score, addr) in top {
                let d: TantivyDocument = searcher.doc(addr)?;
                ranked.insert(first_text(&d, self.fields.id), (score, first_text(&d, self.fields.body)));
            }
        }
        // 2) Guaranteed layer — trigram content candidates (oids). Union the two oid sets.
        let mut oids: Vec<String> = ranked.keys().cloned().collect();
        let mut seen_oid: HashSet<String> = oids.iter().cloned().collect();
        for cand in self.contents.read().unwrap().candidate_ids(query) {
            if seen_oid.insert(cand.id.clone()) {
                oids.push(cand.id);
            }
        }
        // 3) Verify each body once, then expand to every path sharing that oid.
        let mut contents: Vec<Hit> = Vec::new();
        for oid in oids {
            let body = match ranked.get(&oid) {
                Some((_, b)) => Some(b.clone()),
                None => self.stored_body(&searcher, &oid)?,
            };
            let Some(body) = body else { continue };
            if !body.to_lowercase().contains(&ql) {
                continue;
            }
            let snippet = snippet_around(&body, &ql);
            let score = ranked.get(&oid).map(|(s, _)| *s).unwrap_or(0.0);
            let paths: Vec<String> = {
                let op = self.oid_paths.lock().unwrap();
                op.get(&oid)
                    .map(|s| {
                        let mut v: Vec<String> = s.iter().cloned().collect();
                        v.sort();
                        v.truncate(PATHS_PER_BODY);
                        v
                    })
                    .unwrap_or_default()
            };
            for p in paths {
                contents.push(Hit {
                    id: p.clone(),
                    name: p,
                    snippet: snippet.clone(),
                    score,
                });
            }
        }

        // Rank each band: score desc, then name.
        let by_score = |a: &Hit, b: &Hit| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.name.cmp(&b.name))
        };
        filenames.sort_by(by_score);
        contents.sort_by(by_score);
        filenames.truncate(MAX_HITS);
        contents.truncate(MAX_HITS);

        Ok(SearchResponse {
            query: query.to_string(),
            filenames,
            contents,
            files_indexed: self.files_indexed(),
            roots_indexed: self.config.roots.len(),
        })
    }

    /// Read the stored body for one id back out of Tantivy (no filesystem access).
    fn stored_body(
        &self,
        searcher: &tantivy::Searcher,
        id: &str,
    ) -> anyhow::Result<Option<String>> {
        use tantivy::query::TermQuery;
        use tantivy::schema::IndexRecordOption;
        let term = Term::from_field_text(self.fields.id, id);
        let q = TermQuery::new(term, IndexRecordOption::Basic);
        let top = searcher.search(&q, &TopDocs::with_limit(1))?;
        if let Some((_, addr)) = top.first() {
            let d: TantivyDocument = searcher.doc(*addr)?;
            return Ok(Some(first_text(&d, self.fields.body)));
        }
        Ok(None)
    }
}

/// What a reconciliation sweep did. `seeding` is true on the first run against a freshly-forged
/// index — the watermark was populated from disk without reindexing.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ReconcileStats {
    pub seeding: bool,
    pub scanned: usize,
    pub seeded: usize,
    pub upserted: usize,
    pub deleted: usize,
}

/// Load the persisted reconciliation watermark (`mtime\tsize\tid` per line). Missing file → Err
/// (caller treats it as an empty watermark → first reconcile seeds from disk).
fn load_mtimes(path: &Path) -> anyhow::Result<HashMap<String, (u64, u64)>> {
    use std::io::BufRead;
    let f = std::fs::File::open(path)?;
    let mut map = HashMap::new();
    for line in std::io::BufReader::new(f).lines() {
        let line = line?;
        let mut it = line.splitn(3, '\t');
        let (Some(mt), Some(sz), Some(id)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        if let (Ok(mt), Ok(sz)) = (mt.parse::<u64>(), sz.parse::<u64>()) {
            map.insert(id.to_string(), (mt, sz));
        }
    }
    Ok(map)
}

/// Rebuild the OID-keyed `contents` floor from Tantivy's stored (already-deduped) bodies — no
/// filesystem walk. Used at startup when the `contents` sidecar is missing (Tantivy is the durable
/// source of truth for bodies). Keyed by oid, name empty.
fn trigram_from_tantivy(
    index: &Index,
    fields: &Fields,
    max_bytes: usize,
) -> anyhow::Result<moonshot_index::TrigramIndex> {
    let mut t = moonshot_index::TrigramIndex::with_max_content_bytes(max_bytes);
    let searcher = index.reader()?.searcher();
    for seg in searcher.segment_readers() {
        let store = seg.get_store_reader(50)?;
        for doc in store.iter::<TantivyDocument>(seg.alive_bitset()) {
            let d = doc?;
            let oid = first_text(&d, fields.id);
            let body = first_text(&d, fields.body);
            t.upsert(oid, "", &body);
        }
    }
    Ok(t)
}

/// Read the `oid\tpath` map back into `(path→oid, oid→paths)`. An empty-oid line means a
/// filename-only file (no body indexed). Missing file → Err (a Stage-4 index requires this map;
/// the caller surfaces that the index needs a re-forge).
fn load_paths_map(
    path: &Path,
) -> anyhow::Result<(
    HashMap<String, String>,
    HashMap<String, std::collections::HashSet<String>>,
)> {
    use std::io::BufRead;
    let f = std::fs::File::open(path)?;
    let mut path_to_oid: HashMap<String, String> = HashMap::new();
    let mut oid_paths: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
    for line in std::io::BufReader::new(f).lines() {
        let line = line?;
        let mut it = line.splitn(2, '\t');
        let (Some(oid), Some(p)) = (it.next(), it.next()) else {
            continue;
        };
        path_to_oid.insert(p.to_string(), oid.to_string());
        if !oid.is_empty() {
            oid_paths.entry(oid.to_string()).or_default().insert(p.to_string());
        }
    }
    Ok((path_to_oid, oid_paths))
}

/// Read a file for indexing: `(content, Some((mtime,size)))`. Content is empty (filename-only,
/// per the anti-Spotlight rule) if the file is over the size cap, not a regular file, or
/// unreadable; `None` metadata means the path doesn't exist (treat as a delete upstream).
fn read_capped(path: &Path, max: usize) -> (String, Option<(u64, u64)>) {
    let meta = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return (String::new(), None),
    };
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let watermark = Some((mtime, meta.len()));
    if !meta.is_file() || meta.len() as usize > max {
        return (String::new(), watermark); // filename indexed, body skipped
    }
    let content = std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    (content, watermark)
}

fn first_text(d: &TantivyDocument, field: tantivy::schema::Field) -> String {
    d.get_first(field)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// A short excerpt around the first occurrence of `needle` (lowercased) in `hay`.
fn snippet_around(hay: &str, needle: &str) -> String {
    const PAD: usize = 40;
    let hay_lc = hay.to_lowercase();
    let Some(pos) = hay_lc.find(needle) else {
        return String::new();
    };
    let start = floor_boundary(hay, pos.saturating_sub(PAD));
    let end = ceil_boundary(hay, (pos + needle.len() + PAD).min(hay.len()));
    let mut s = String::new();
    if start > 0 {
        s.push('…');
    }
    s.push_str(hay[start..end].trim());
    if end < hay.len() {
        s.push('…');
    }
    s
}
fn floor_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}
fn ceil_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

#[cfg(test)]
mod stage4_tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("ss-s4-{}-{}", std::process::id(), name));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    fn write(root: &Path, rel: &str, content: &str) {
        let f = root.join(rel);
        if let Some(p) = f.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(f, content).unwrap();
    }
    fn config_for(root: &Path, index: &Path) -> Config {
        Config {
            index_path: index.to_string_lossy().into_owned(),
            roots: vec![RootSpec {
                url_prefix: "_r".into(),
                fs_path: root.to_string_lossy().into_owned(),
            }],
            exclude_dirs: vec![".git".into(), "target".into()],
            bind: "127.0.0.1:0".into(),
            max_content_bytes: 5_000_000,
        }
    }
    fn content_ids(hits: &[Hit]) -> std::collections::HashSet<&str> {
        hits.iter().map(|h| h.id.as_str()).collect()
    }

    // The whole content-addressed contract in one flow: dedup, path-expansion, free rename,
    // ref-counted body survival, and orphan tombstoning. Robust to reader-reload lag because the
    // path-expansion is driven by the RAM `oid_paths` map, not the Tantivy reader.
    #[test]
    fn dedup_rename_refcount_delete() {
        let root = tmp("root");
        let index = tmp("index");
        write(&root, "a.txt", "IDENTICAL BODY hello world");
        write(&root, "b.txt", "IDENTICAL BODY hello world"); // same bytes as a.txt
        write(&root, "c.txt", "unique zebra content");
        let cfg = config_for(&root, &index);

        let stats = forge(&cfg).unwrap();
        assert_eq!(stats.files, 3);
        assert_eq!(stats.distinct_bodies, 2, "two identical bodies dedup to one");

        let strike = Strike::load(&cfg).unwrap();
        assert_eq!(strike.files_indexed(), 3);

        // A content hit on the shared body expands to BOTH paths.
        let r = strike.search("IDENTICAL BODY").unwrap();
        let ids = content_ids(&r.contents);
        assert!(ids.contains("_r/a.txt") && ids.contains("_r/b.txt"), "shared body → both paths");

        // Free rename c.txt → d.txt (same content): no new distinct body.
        write(&root, "d.txt", "unique zebra content");
        std::fs::remove_file(root.join("c.txt")).unwrap();
        strike.apply(Change::Upsert(root.join("d.txt"))).unwrap();
        strike.apply(Change::Delete(root.join("c.txt"))).unwrap();
        strike.commit_if_dirty().unwrap();
        let r = strike.search("zebra").unwrap();
        let ids = content_ids(&r.contents);
        assert!(ids.contains("_r/d.txt"), "renamed path present");
        assert!(!ids.contains("_r/c.txt"), "old path gone");

        // Delete one of the shared-body files: the body survives (b.txt still references it).
        strike.apply(Change::Delete(root.join("a.txt"))).unwrap();
        strike.commit_if_dirty().unwrap();
        let r = strike.search("IDENTICAL BODY").unwrap();
        let ids = content_ids(&r.contents);
        assert!(ids.contains("_r/b.txt") && !ids.contains("_r/a.txt"), "body survives via b.txt");

        // Delete the last reference: body orphaned → no content hit.
        strike.apply(Change::Delete(root.join("b.txt"))).unwrap();
        strike.commit_if_dirty().unwrap();
        assert!(
            strike.search("IDENTICAL BODY").unwrap().contents.is_empty(),
            "body orphaned after last ref removed"
        );

        // Filename band still resolves.
        assert!(strike
            .search("d.txt")
            .unwrap()
            .filenames
            .iter()
            .any(|h| h.id == "_r/d.txt"));

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&index);
    }

    // A metadata-only touch (same content) must not create a new distinct body or duplicate.
    #[test]
    fn touch_same_content_is_cheap() {
        let root = tmp("root2");
        let index = tmp("index2");
        write(&root, "x.txt", "stable body content foxtrot");
        let cfg = config_for(&root, &index);
        forge(&cfg).unwrap();
        let strike = Strike::load(&cfg).unwrap();

        // Re-apply upsert with identical content (a touch).
        strike.apply(Change::Upsert(root.join("x.txt"))).unwrap();
        strike.commit_if_dirty().unwrap();
        let r = strike.search("foxtrot").unwrap();
        assert_eq!(r.contents.len(), 1, "no duplicate path from a touch");
        assert_eq!(strike.files_indexed(), 1);

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&index);
    }
}
