// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Error types for the compute manifest layer.

use thiserror::Error;

/// Errors produced when loading or validating a [`ComputeManifest`](crate::ComputeManifest).
#[derive(Debug, Error)]
pub enum ManifestError {
    /// The YAML could not be parsed.
    #[error("manifest YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// A field failed its validation constraint.
    ///
    /// The message names the field and states the constraint, for example:
    /// `"min_instances must be 0 or 1, got 2"`.
    #[error("manifest field invalid: {0}")]
    InvalidField(String),

    /// A cross-field constraint was violated.
    ///
    /// Used for `max_instances < min_instances`, which cannot be checked by a
    /// single-field rule.
    #[error("manifest invalid range: {0}")]
    InvalidRange(String),

    /// The manifest file could not be read from disk.
    #[error("manifest I/O error: {0}")]
    Io(#[from] std::io::Error),
}
