//! Phase 2 Step 7 — passthrough WebSocket relay for yjs collab.
//!
//! Per-slug rooms via `tokio::sync::broadcast`. The server holds no Y.Doc
//! state — clients sync directly through the relay using y-protocols
//! (sync + awareness messages). Persistence is via the existing
//! `POST /edit/{slug}` when a user clicks save.
//!
//! Default-off: only mounted when `--enable-collab` is set on the CLI.
//! Production deploys without the flag never touch this module.
//!
//! Why no `yrs` (Rust Yjs port): for Step 7 minimum, the relay needs no
//! awareness of doc state. Frames in are frames out. yrs would be needed
//! for snapshot-on-commit (Y.Doc → markdown server-side); deferred.

use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    Path, State,
};
use axum::response::Response;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

/// Per-slug broadcast rooms. Each room holds up to 256 lagging messages
/// before slow subscribers get dropped (reasonable for editor sessions).
#[derive(Default)]
pub struct CollabRooms {
    rooms: Mutex<HashMap<String, broadcast::Sender<Vec<u8>>>>,
}

impl CollabRooms {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(HashMap::new()),
        }
    }

    /// Get the broadcast sender for `slug`, creating an empty room if
    /// none exists.
    async fn sender_for(&self, slug: &str) -> broadcast::Sender<Vec<u8>> {
        let mut rooms = self.rooms.lock().await;
        rooms
            .entry(slug.to_string())
            .or_insert_with(|| broadcast::channel(256).0)
            .clone()
    }
}

/// `GET /ws/collab/{slug}` — WebSocket upgrade for collab on `slug`.
///
/// Only mounted when `--enable-collab` is set. The handler pumps messages
/// between this socket and the per-slug broadcast channel: incoming
/// binary frames fan out to all subscribers; broadcast frames stream
/// back to this socket. Text frames are ignored (yjs uses binary only).
pub async fn ws_collab(
    ws: WebSocketUpgrade,
    Path(_slug): Path<String>,
    State(state): State<Arc<crate::server::AppState>>,
) -> Response {
    let rooms = state.collab.clone();
    let slug_owned = _slug;
    ws.on_upgrade(move |socket| handle_socket(socket, slug_owned, rooms))
}

async fn handle_socket(socket: WebSocket, slug: String, rooms: Arc<CollabRooms>) {
    let tx = rooms.sender_for(&slug).await;
    let mut rx = tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(data) = rx.recv().await {
            if sender.send(Message::Binary(data.into())).await.is_err() {
                break;
            }
        }
    });

    let tx_clone = tx.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Binary(data) = msg {
                let _ = tx_clone.send(data.to_vec());
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rooms_are_created_on_demand() {
        let r = CollabRooms::new();
        let _tx1 = r.sender_for("topic-a").await;
        // Second access returns a clone of the same sender
        let _tx2 = r.sender_for("topic-a").await;
        let rooms = r.rooms.lock().await;
        assert_eq!(rooms.len(), 1);
        assert!(rooms.contains_key("topic-a"));
    }

    #[tokio::test]
    async fn separate_slugs_get_separate_rooms() {
        let r = CollabRooms::new();
        let _tx_a = r.sender_for("topic-a").await;
        let _tx_b = r.sender_for("topic-b").await;
        let rooms = r.rooms.lock().await;
        assert_eq!(rooms.len(), 2);
        assert!(rooms.contains_key("topic-a"));
        assert!(rooms.contains_key("topic-b"));
    }

    #[tokio::test]
    async fn broadcast_fans_out_to_subscribers() {
        let r = CollabRooms::new();
        let tx = r.sender_for("topic-a").await;
        let mut rx1 = tx.subscribe();
        let mut rx2 = tx.subscribe();
        tx.send(vec![1, 2, 3]).unwrap();
        let m1 = tokio::time::timeout(std::time::Duration::from_millis(100), rx1.recv())
            .await
            .unwrap()
            .unwrap();
        let m2 = tokio::time::timeout(std::time::Duration::from_millis(100), rx2.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(m1, vec![1, 2, 3]);
        assert_eq!(m2, vec![1, 2, 3]);
    }
}
