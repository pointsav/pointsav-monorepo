// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! `app-orchestration-graph` v0.0.1-stub
//!
//! Placeholder for the federated entity graph. Accepts connections and
//! responds to health probes only. Real implementation is planned for v0.1.0.
//!
//! Binds on 127.0.0.1:8021 by default.

use axum::routing::get;
use axum::Router;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let bind = std::env::var("GRAPH_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8021".to_string());

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/v1/graph/status",
            get(|| async {
                axum::Json(serde_json::json!({
                    "status": "stub",
                    "version": "0.0.1-stub"
                }))
            }),
        );

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    eprintln!("orchestration-graph stub listening on {bind}");
    axum::serve(listener, app).await.unwrap();
}
