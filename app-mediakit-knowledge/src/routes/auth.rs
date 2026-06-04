//! Authentication routes.
//!
//! Phase 6: cookie session login/logout. Wires route stubs to real handler
//! implementations in `crate::auth`.
//!
//! Routes owned by this module (registered in server::router at /special/*):
//! - `GET  /special/login`          — login page
//! - `POST /special/login`          — authenticate + set session cookie
//! - `POST /special/logout`         — destroy session + clear cookie
//! - `GET  /special/create-account` — admin-only account creation form
//! - `POST /special/create-account` — create new user

pub use crate::auth::get_create_account;
pub use crate::auth::get_login as login_page;
pub use crate::auth::post_create_account;
pub use crate::auth::post_login as login_post;
pub use crate::auth::post_logout as logout;
