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
}

impl IntoResponse for WikiError {
    fn into_response(self) -> Response {
        let status = match &self {
            WikiError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        tracing::warn!(error = %self, "request error");
        (status, self.to_string()).into_response()
    }
}
