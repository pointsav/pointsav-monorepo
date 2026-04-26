use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum WikiError {
    #[error("page not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("frontmatter parse error: {0}")]
    Frontmatter(#[from] serde_yaml::Error),

    /// Phase 2: slug failed `^[a-z0-9._-]+$` validation (path-traversal,
    /// uppercase, spaces, leading dot, `..` sequence).
    #[error("invalid slug: {0}")]
    SlugInvalid(String),

    /// Phase 2: atomic write to disk failed (temp-file create, write, or
    /// persist/rename).
    #[error("write failed: {0}")]
    WriteFailed(String),

    /// Phase 2: `/create` invoked with a slug that already exists on disk.
    #[error("already exists: {0}")]
    AlreadyExists(String),

    /// Phase 2 Step 5: citation registry file could not be read or parsed.
    #[error("citation registry load failed: {0}")]
    CitationLoadFailed(String),
}

impl IntoResponse for WikiError {
    fn into_response(self) -> Response {
        let status = match &self {
            WikiError::NotFound(_) => StatusCode::NOT_FOUND,
            WikiError::SlugInvalid(_) => StatusCode::BAD_REQUEST,
            WikiError::AlreadyExists(_) => StatusCode::CONFLICT,
            WikiError::CitationLoadFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        tracing::warn!(error = %self, "request error");
        (status, self.to_string()).into_response()
    }
}
