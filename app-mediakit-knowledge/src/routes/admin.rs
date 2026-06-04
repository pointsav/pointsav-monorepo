//! Admin routes.
//!
//! Phase 6: pending edit review queue. Wires route stubs to real handler
//! implementations in `crate::pending`.
//!
//! Routes owned by this module (registered in server::router at /special/*):
//! - `GET  /special/pending-changes`         — pending edit review queue
//! - `GET  /special/pending/{id}`            — review detail + diff
//! - `POST /special/pending/{id}/accept`     — accept and publish
//! - `POST /special/pending/{id}/reject`     — reject with optional note
//! - `GET  /special/contributions/{username}` — editor's submission history

pub use crate::pending::accept_edit;
pub use crate::pending::contributions;
pub use crate::pending::reject_edit;
pub use crate::pending::review_detail;
pub use crate::pending::review_queue as pending_list;
