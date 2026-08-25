//! strike — the LIVE search server: loads the index, serves queries, and updates the index
//! in place as the filesystem changes (no periodic rebuild).
//!
//! Usage: strike <config.toml>
//! Endpoints:
//!   GET /search?q=<query>   → JSON SearchResponse (two bands + coverage)
//!   GET /healthz            → "ok"

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use serde::Deserialize;
use service_search::{Change, Config, SearchResponse, Strike};

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: strike <config.toml>");
        std::process::exit(2);
    });
    let config = Config::from_toml_path(&config_path)?;
    let bind = config.bind.clone();

    eprintln!("strike: loading index from {}", config.index_path);
    let strike = Arc::new(Strike::load(&config)?);
    eprintln!(
        "strike: ready — {} docs across {} roots, listening on {bind}",
        strike.files_indexed(),
        config.roots.len()
    );

    // ── Live filesystem watcher → debounce → apply ────────────────────────────
    let (raw_tx, mut raw_rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
    // Reconciliation trigger: the callback signals this on an inotify overflow/error so the sweep
    // self-corrects the events it dropped.
    let (recon_tx, mut recon_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    let mut watcher = recommended_watcher(move |res: notify::Result<notify::Event>| match res {
        Ok(event) => {
            for p in event.paths {
                let _ = raw_tx.send(p);
            }
        }
        Err(_) => {
            let _ = recon_tx.send(());
        }
    })?;
    let mut watched = 0;
    for root in &config.roots {
        let (n, partial) = watch_resilient(&mut watcher, Path::new(&root.fs_path));
        if n > 0 {
            watched += 1;
            if partial {
                eprintln!(
                    "strike: watching {} PARTIALLY ({n} subtrees; some dirs unreadable)",
                    root.fs_path
                );
            }
        } else {
            eprintln!("strike: could not watch {} at all", root.fs_path);
        }
    }
    eprintln!("strike: watching {watched}/{} roots for live changes", config.roots.len());

    // Debounce loop: coalesce events per path over a ~300ms window (editors write-temp-rename;
    // git touches a file several times), then apply each once. apply() does blocking fs + index
    // work, so it runs on spawn_blocking; the writer Mutex serializes them.
    {
        let strike = strike.clone();
        tokio::spawn(async move {
            while let Some(first) = raw_rx.recv().await {
                let mut batch: HashSet<PathBuf> = HashSet::new();
                batch.insert(first);
                let deadline = tokio::time::sleep(Duration::from_millis(300));
                tokio::pin!(deadline);
                loop {
                    tokio::select! {
                        _ = &mut deadline => break,
                        maybe = raw_rx.recv() => match maybe {
                            Some(p) => { batch.insert(p); }
                            None => break,
                        },
                    }
                }
                for path in batch {
                    let strike = strike.clone();
                    let _ = tokio::task::spawn_blocking(move || {
                        // Decide upsert vs delete by the current on-disk state.
                        let change = if path.is_file() {
                            Change::Upsert(path)
                        } else if !path.exists() {
                            Change::Delete(path)
                        } else {
                            return; // a directory or special file — its files get their own events
                        };
                        if let Err(e) = strike.apply(change) {
                            eprintln!("strike: apply error: {e}");
                        }
                    })
                    .await;
                }
            }
        });
    }

    // Commit ticker: publish pending Tantivy ops every ~2s so content hits go live (the reader
    // auto-reloads). Filename hits were already live. Bursts batch into one commit.
    {
        let strike = strike.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(2));
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                let s = strike.clone();
                if let Ok(Err(e)) = tokio::task::spawn_blocking(move || s.commit_if_dirty()).await {
                    eprintln!("strike: commit error: {e}");
                }
            }
        });
    }

    // Reconciliation sweep: a startup pass (seeds the watermark on a fresh index; catches any
    // drift from while the Strike was down), then every 15 min, plus on inotify overflow. This is
    // the backstop that lets us delete the rebuild entirely — event loss self-corrects.
    {
        let strike = strike.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(20)).await; // let the watcher settle first
            run_reconcile(&strike).await;
            let mut ticker = tokio::time::interval(Duration::from_secs(900));
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            ticker.tick().await; // consume the immediate first tick (already reconciled above)
            loop {
                tokio::select! {
                    _ = ticker.tick() => run_reconcile(&strike).await,
                    _ = recon_rx.recv() => {
                        eprintln!("strike: inotify overflow/error — running reconciliation");
                        run_reconcile(&strike).await;
                    }
                }
            }
        });
    }

    let app = Router::new()
        .route("/search", get(search))
        .route("/healthz", get(|| async { "ok" }))
        .with_state(strike);

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    // Keep the watcher alive for the life of the server (dropping it stops watching).
    let _watcher = watcher;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Run one reconciliation sweep off the async reactor and log what it did.
async fn run_reconcile(strike: &Arc<Strike>) {
    let s = strike.clone();
    match tokio::task::spawn_blocking(move || s.reconcile()).await {
        Ok(Ok(st)) if st.seeding => {
            eprintln!("strike: reconcile seeded {} watermarks (no reindex)", st.seeded)
        }
        Ok(Ok(st)) => eprintln!(
            "strike: reconcile scanned {} — {} upserted, {} deleted",
            st.scanned, st.upserted, st.deleted
        ),
        Ok(Err(e)) => eprintln!("strike: reconcile error: {e}"),
        Err(e) => eprintln!("strike: reconcile task panicked: {e}"),
    }
}

/// Watch `dir` recursively, resilient to unreadable subdirectories. notify aborts an entire
/// recursive `watch()` on the first `EACCES` it hits during its scan, so a single `0700` subtree
/// (another user's dir, a build overlay) would otherwise cost live coverage of the whole root.
/// On failure we descend one level and watch each readable child subtree on its own, skipping
/// only the unreadable branch. Returns `(top-level watches established, was-partial)`.
fn watch_resilient(watcher: &mut notify::RecommendedWatcher, dir: &Path) -> (usize, bool) {
    if watcher.watch(dir, RecursiveMode::Recursive).is_ok() {
        return (1, false);
    }
    // Recursive watch failed — descend and watch each readable child subtree individually.
    let mut n = 0;
    let mut partial = true;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                let (cn, _) = watch_resilient(watcher, &p);
                n += cn;
            }
        }
    }
    if n == 0 {
        partial = false; // nothing here was watchable at all
    }
    (n, partial)
}

async fn search(
    State(strike): State<Arc<Strike>>,
    Query(params): Query<SearchParams>,
) -> Json<SearchResponse> {
    // Blocking CPU work (Tantivy + trigram) off the async reactor.
    let resp = tokio::task::spawn_blocking(move || strike.search(&params.q))
        .await
        .unwrap_or_else(|e| Err(anyhow::anyhow!("search task panicked: {e}")))
        .unwrap_or_else(|e| {
            SearchResponse {
                query: String::new(),
                filenames: Vec::new(),
                contents: Vec::new(),
                files_indexed: 0,
                roots_indexed: 0,
            }
            .with_error(e)
        });
    Json(resp)
}

// Small ergonomic helper so a query error still returns a well-formed (empty) response
// rather than a 500 — the UI shows a trustworthy zero, never a broken state.
trait WithError {
    fn with_error(self, e: anyhow::Error) -> SearchResponse;
}
impl WithError for SearchResponse {
    fn with_error(mut self, e: anyhow::Error) -> SearchResponse {
        eprintln!("strike: search error: {e}");
        self.query = format!("(error) {e}");
        self
    }
}
