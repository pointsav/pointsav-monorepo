// SPDX-License-Identifier: Apache-2.0

//! Wire and on-disk types for the job system.

use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::ModuleId;
use uuid::Uuid;

/// Parses a strict hyphenated UUID only (36 chars). Anything else —
/// including path separators and `..` — fails, so a parsed id can be used as
/// a filename component without further checks.
fn parse_strict_uuid(s: &str) -> Option<Uuid> {
    if s.len() != 36 {
        return None;
    }
    Uuid::parse_str(s).ok()
}

macro_rules! uuid_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Uuid);

        impl $name {
            /// Server-generated id (UUIDv7: time-ordered).
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.as_hyphenated())
            }
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, String> {
                parse_strict_uuid(s)
                    .map(Self)
                    .ok_or_else(|| format!("invalid {}", stringify!($name)))
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                s.parse().map_err(serde::de::Error::custom)
            }
        }
    };
}

uuid_id!(JobId);
uuid_id!(StagedRef);

/// What a job does. The engine named in the request must support the kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobKind {
    OcrPage,
    ExtractSpans,
    ExtractLlm,
}

/// Terminal and in-flight job states. Terminal values double as the
/// `outcome` reported to callers and written to the audit ledger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Empty,
    NotApplicable,
    Unreadable,
    Deferred,
    Failed,
}

impl JobStatus {
    pub fn is_terminal(self) -> bool {
        !matches!(self, JobStatus::Queued | JobStatus::Running)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            JobStatus::Queued => "queued",
            JobStatus::Running => "running",
            JobStatus::Done => "done",
            JobStatus::Empty => "empty",
            JobStatus::NotApplicable => "not-applicable",
            JobStatus::Unreadable => "unreadable",
            JobStatus::Deferred => "deferred",
            JobStatus::Failed => "failed",
        }
    }
}

/// Caller-chosen model parameters. Defaults are deterministic.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobParams {
    pub prompt_id: Option<String>,
    pub schema_id: Option<String>,
    pub temperature: Option<f32>,
    pub seed: Option<u32>,
    pub max_tokens: Option<u32>,
    /// Caller's label for the page this job reads (e.g. `statement-2024-03.p3`);
    /// echoed back in the record so each result JSON names its page.
    pub page_ref: Option<String>,
}

/// Where the job's input lives.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputRef {
    /// Lowercase hex SHA-256 the caller expects the input to have.
    pub sha256: String,
    pub media_type: String,
    /// Upload mode: a ref returned by `POST /v1/jobs/inputs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staged_ref: Option<StagedRef>,
    /// Existing-file mode: a single file name under the credential's
    /// allow-listed `input_dir`. Never a path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
}

/// `POST /v1/jobs` body.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmitRequest {
    pub engine_id: String,
    pub kind: JobKind,
    pub input: InputRef,
    #[serde(default)]
    pub params: JobParams,
}

/// Hash-and-parameter record kept for every finished job (no content).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResultRecord {
    pub job_id: JobId,
    pub engine_id: String,
    pub model_id: String,
    pub weights_sha256: String,
    pub prompt_sha256: String,
    pub schema_sha256: String,
    pub params: JobParams,
    pub input_sha256: String,
    /// Empty when the job produced no output.
    pub output_sha256: String,
    pub outcome: JobStatus,
    pub latency_ms: u64,
}

/// On-disk job status file (`jobs/<id>.json`). Holds no content.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobRecord {
    pub job_id: JobId,
    pub module_id: ModuleId,
    pub credential_id: String,
    pub engine_id: String,
    pub kind: JobKind,
    pub params: JobParams,
    pub input: InputRef,
    pub input_size: u64,
    /// Resolved absolute path for existing-file inputs (server-resolved,
    /// never caller-supplied).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    pub status: JobStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defer_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<ResultRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_deleted_at: Option<DateTime<Utc>>,
    /// Set on replay jobs: the original job being re-checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_of: Option<JobId>,
    /// Replay jobs only, once finished: did `output_sha256` equal the original's?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_match: Option<bool>,
}

/// Byte offsets (UTF-8) into the full input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A model-proposed entity whose `witness` was found verbatim in the input
/// at `span`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub class: String,
    pub value: String,
    pub witness: String,
    pub span: Span,
}

/// Content of a finished job (`results/<id>.json`); deleted on ack.
/// For witness-validated prompts `entities`/`rejected` hold the validated
/// output and `text` is empty; otherwise `text` is the engine output.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobResult {
    pub job_id: JobId,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_reject_paths_and_garbage() {
        for bad in [
            "",
            "..",
            "../x",
            "a/b",
            "/etc/passwd",
            "0190b2c8-0000-7000-8000-00000000000g",
            "{0190b2c8-0000-7000-8000-000000000000}",
        ] {
            assert!(bad.parse::<JobId>().is_err(), "{bad:?}");
        }
        let id = JobId::new();
        assert_eq!(id.to_string().parse::<JobId>().unwrap(), id);
    }
}
