// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
//
// Smoke test: exercises the crate's public surface end-to-end at compile
// and run time.

#![allow(missing_docs)]
#![allow(clippy::disallowed_methods)]

use std::str::FromStr;

use slm_core::ModuleId;

#[test]
fn module_id_public_surface() {
    let id = ModuleId::new("woodfine-v1").expect("valid moduleId");
    assert_eq!(id.as_str(), "woodfine-v1");
    assert_eq!(id.to_string(), "woodfine-v1");

    let parsed = ModuleId::from_str("woodfine-v1").unwrap();
    assert_eq!(parsed, id);

    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"woodfine-v1\"");

    let back: ModuleId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}
