// SPDX-License-Identifier: Apache-2.0 OR MIT

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid moduleId {value:?}: {reason}")]
    InvalidModuleId { value: String, reason: &'static str },

    #[error("invalid requestId: {0}")]
    InvalidRequestId(String),
}
