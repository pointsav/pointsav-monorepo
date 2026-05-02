pub mod commit;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EditSubmission {
    pub slug: String,
    pub section_heading: Option<String>,
    pub updated_section_markdown: String,
    /// Required. Minimum 10 characters. Becomes the Git commit message.
    pub edit_summary: String,
    /// HEAD SHA when the editor loaded. Used for conflict detection.
    pub base_sha: String,
    /// Verified human identity from MBA auth. Must be set by the handler
    /// from the auth token — never trusted from the form body directly.
    pub editor_identity: String,
}

#[derive(Debug, Serialize)]
pub struct EditResult {
    pub success: bool,
    pub new_sha: Option<String>,
    pub message: String,
}
