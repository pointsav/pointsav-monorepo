// SPDX-License-Identifier: Apache-2.0

//! The ingest-adapter boundary.
//!
//! `BRIEF-os-totebox-communication-complex-2026-09.md` Phase 1, step
//! 1a: this crate absorbs external mail sources as one `IngestAdapter`
//! impl among several, rather than treating EWS/Exchange harvesting as
//! the architecture itself. `exchange` (moved verbatim from the former
//! top-level `ews_client.rs`) is the first and, as of Phase 1, only
//! implementation; `service-email-egress`'s onboarding-migration
//! output is a named future adapter, not built here.

pub mod exchange;

/// One external mail source this crate can poll for new messages.
/// Mirrors the three EWS operations `exchange::EwsClient` already
/// implemented (`find_unread_ids`/`get_mime`/`mark_read`) — this trait
/// just names that shape so a second adapter can be added without
/// touching the poll loop in `main.rs`.
#[allow(async_fn_in_trait)]
pub trait IngestAdapter {
    /// IDs of messages not yet ingested (e.g. EWS "unread" items).
    async fn find_new(&self) -> Result<Vec<String>, IngestError>;
    /// Raw MIME bytes for one message.
    async fn fetch_raw(&self, id: &str) -> Result<Vec<u8>, IngestError>;
    /// Mark one message as ingested at the source (e.g. EWS "mark read"),
    /// so a re-poll doesn't return it again.
    async fn mark_ingested(&self, id: &str) -> Result<(), IngestError>;
}

#[derive(Debug)]
pub struct IngestError(pub String);

impl std::fmt::Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for IngestError {}

impl From<Box<dyn std::error::Error>> for IngestError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        IngestError(e.to_string())
    }
}
