// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![allow(missing_docs)]
#![allow(clippy::disallowed_methods)]

use slm_memory_adapters::{Registry, RegistryError};

const VALID_YAML: &str = r#"
adapters:
  - adapter_id: dka-coa
    version: "3.2.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:abc123"
    signature: "sigstore:deadbeef"
  - adapter_id: dka-archetype
    version: "2.1.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/archetype/v2.1"
    training_data_hash: "sha256:def456"
    signature: "sigstore:cafebabe"
"#;

#[test]
fn valid_yaml_parses_correctly() {
    let registry = Registry::from_yaml(VALID_YAML).expect("valid YAML must parse");
    assert_eq!(registry.adapters.len(), 2);

    let coa = &registry.adapters[0];
    assert_eq!(coa.adapter_id, "dka-coa");
    assert_eq!(coa.version, semver::Version::new(3, 2, 0));
    assert_eq!(coa.base_model, "gemma-4-27b");
    assert_eq!(coa.oci_ref, "gs://dka-adapters/coa/v3.2");
    assert_eq!(coa.training_data_hash, "sha256:abc123");
    assert_eq!(coa.signature, "sigstore:deadbeef");

    let arch = &registry.adapters[1];
    assert_eq!(arch.adapter_id, "dka-archetype");
    assert_eq!(arch.version, semver::Version::new(2, 1, 0));
}

#[test]
fn empty_adapters_list_is_valid() {
    let yaml = "adapters: []\n";
    let registry = Registry::from_yaml(yaml).expect("empty list must parse");
    assert!(registry.adapters.is_empty());
}

#[test]
fn invalid_semver_two_part_version_is_rejected() {
    let yaml = r#"
adapters:
  - adapter_id: dka-coa
    version: "3.2"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:abc123"
    signature: "sigstore:deadbeef"
"#;
    let err = Registry::from_yaml(yaml).expect_err("two-part version must be rejected");
    assert!(
        matches!(err, RegistryError::InvalidVersion { ref adapter_id, .. } if adapter_id == "dka-coa"),
        "unexpected error variant: {err}"
    );
    assert!(
        err.to_string().contains("3.2"),
        "error message must include the bad version"
    );
}

#[test]
fn invalid_semver_v_prefix_is_rejected() {
    let yaml = r#"
adapters:
  - adapter_id: dka-coa
    version: "v3.2.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:abc123"
    signature: "sigstore:deadbeef"
"#;
    let err = Registry::from_yaml(yaml).expect_err("v-prefixed version must be rejected");
    assert!(
        matches!(err, RegistryError::InvalidVersion { .. }),
        "unexpected error variant: {err}"
    );
}

#[test]
fn duplicate_adapter_version_is_rejected() {
    let yaml = r#"
adapters:
  - adapter_id: dka-coa
    version: "3.2.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:abc123"
    signature: "sigstore:deadbeef"
  - adapter_id: dka-coa
    version: "3.2.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:abc123"
    signature: "sigstore:deadbeef"
"#;
    let err = Registry::from_yaml(yaml).expect_err("duplicate must be rejected");
    assert!(
        matches!(err, RegistryError::DuplicateEntry { ref adapter_id, ref version }
            if adapter_id == "dka-coa" && version == "3.2.0"),
        "unexpected error variant: {err}"
    );
}

#[test]
fn same_adapter_id_different_versions_are_allowed() {
    let yaml = r#"
adapters:
  - adapter_id: dka-coa
    version: "3.1.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.1"
    training_data_hash: "sha256:aaa"
    signature: "sigstore:111"
  - adapter_id: dka-coa
    version: "3.2.0"
    base_model: gemma-4-27b
    oci_ref: "gs://dka-adapters/coa/v3.2"
    training_data_hash: "sha256:bbb"
    signature: "sigstore:222"
"#;
    let registry = Registry::from_yaml(yaml).expect("distinct versions must parse");
    assert_eq!(registry.adapters.len(), 2);
}

#[test]
fn malformed_yaml_is_rejected() {
    let err = Registry::from_yaml("adapters: [unclosed").expect_err("malformed YAML must fail");
    assert!(
        matches!(err, RegistryError::Yaml(_)),
        "unexpected error variant: {err}"
    );
}
