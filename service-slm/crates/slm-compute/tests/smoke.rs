// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
//
// Integration smoke test: exercises the public API across the crate boundary.

#![allow(missing_docs, clippy::disallowed_methods)]

use slm_compute::{ComputeManifest, GpuTier};

const FIXTURE: &str = r#"
image: "us-central1-docker.pkg.dev/dka-project/slm/vllm:v0.6.2"
gpu_tier: rtx_pro_6000
region: "us-central1"
min_instances: 0
max_instances: 2
request_timeout_seconds: 1800
concurrency: 2
"#;

#[test]
fn manifest_parses_via_public_api() {
    let m = ComputeManifest::from_yaml(FIXTURE).expect("valid fixture");
    assert_eq!(m.gpu_tier, GpuTier::RtxPro6000);
    assert_eq!(m.region, "us-central1");
    assert_eq!(m.min_instances, 0);
    assert_eq!(m.max_instances, 2);
}

#[test]
fn manifest_rejects_malformed_input() {
    assert!(ComputeManifest::from_yaml("not: valid: yaml: [").is_err());
}

#[test]
fn manifest_rejects_constraint_violation() {
    let bad = FIXTURE.replace("min_instances: 0", "min_instances: 5");
    assert!(ComputeManifest::from_yaml(&bad).is_err());
}
