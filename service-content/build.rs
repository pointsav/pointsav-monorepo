// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

// lbug's vendored httplib is compiled with OpenSSL support whenever cmake's
// find_package(OpenSSL) succeeds on the build host (this VM has libssl-dev
// installed). `openssl-sys` (declared in Cargo.toml) correctly detects the
// system OpenSSL and emits its own `cargo:rustc-link-lib=ssl`/`=crypto`, but
// that alone isn't sufficient here: liblbug.a is a static archive scanned
// once by rust-lld, and openssl-sys's link-lib directives can land in the
// wrong position relative to it on the final link command, leaving lbug's
// SSLClient symbol references unresolved. Emitting the same libs again here,
// from service-content's own build script, places them where they actually
// resolve against liblbug.a's unresolved references.
fn main() {
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
}
