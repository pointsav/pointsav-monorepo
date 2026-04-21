// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
//
// Integration smoke test: exercises the public API across the crate boundary.

#![allow(missing_docs, clippy::disallowed_methods)]

use slm_doorman::{NoOp, SanitisationPolicy};

#[test]
fn noop_round_trip_via_public_api() {
    let payload = "PointSav doorman smoke test".to_owned();
    let (sanitised, ctx) = NoOp.sanitise(payload.clone()).expect("sanitise");
    let recovered = NoOp.rehydrate(sanitised, ctx).expect("rehydrate");
    assert_eq!(recovered, payload);
}

#[test]
fn noop_is_identity_on_empty_string() {
    let (sanitised, ctx) = NoOp.sanitise(String::new()).expect("sanitise");
    let recovered = NoOp.rehydrate(sanitised, ctx).expect("rehydrate");
    assert_eq!(recovered, String::new());
}
