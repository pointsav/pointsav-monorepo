// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("registry file not found: {0}")]
    MissingFile(PathBuf),
    #[error("{file}: required column `{column}` is missing")]
    MissingColumn { file: PathBuf, column: &'static str },
    #[error("{file}: duplicate id `{id}`")]
    DuplicateId { file: PathBuf, id: String },
    #[error("{file}: does not match schema.csv: {detail}")]
    SchemaMismatch { file: PathBuf, detail: String },
    #[error("{file}: {source}")]
    Csv {
        file: PathBuf,
        #[source]
        source: csv::Error,
    },
}
