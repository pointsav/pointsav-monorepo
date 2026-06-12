use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Contact {
    id: String,
    name: String,
    #[serde(default)]
    linkedin_url: Option<String>,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    communication_history: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Ledger {
    contacts: Vec<Contact>,
}

type AppState = Arc<Vec<Contact>>;

#[tokio::main]
async fn main() {
    let ledger_path = std::env::var("SERVICE_PEOPLE_LEDGER_PATH").unwrap_or_else(|_| {
        let dir = std::path::Path::new(file!())
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        dir.join("ledger_personnel.json")
            .to_string_lossy()
            .into_owned()
    });

    let port: u16 = std::env::var("SERVICE_PEOPLE_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(9091);

    let content = std::fs::read_to_string(&ledger_path)
        .unwrap_or_else(|e| panic!("Cannot read ledger at {ledger_path}: {e}"));
    let ledger: Ledger =
        serde_json::from_str(&content).unwrap_or_else(|e| panic!("Malformed ledger JSON: {e}"));

    let contacts: AppState = Arc::new(ledger.contacts);

    let app = Router::new()
        .route("/v1/people", get(list_people))
        .route("/v1/people/:id", get(get_person))
        .with_state(contacts);

    let addr = format!("0.0.0.0:{port}");
    println!("[service-people] listening on {addr} ({} contacts)", {
        let c = app.clone();
        drop(c);
        port
    });

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Cannot bind {addr}: {e}"));

    println!("[service-people] ready on :{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn list_people(State(contacts): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "contacts": *contacts }))
}

async fn get_person(
    State(contacts): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    contacts
        .iter()
        .find(|c| c.id == id)
        .map(|c| Json(serde_json::json!(c)))
        .ok_or(StatusCode::NOT_FOUND)
}
