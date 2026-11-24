// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#![forbid(unsafe_code)]
fn main() -> std::process::ExitCode {
    let mut args = Vec::new();
    for a in std::env::args_os().skip(1) {
        match a.into_string() {
            Ok(s) => args.push(s),
            Err(_) => {
                eprintln!("error: an argument is not valid UTF-8");
                return std::process::ExitCode::from(2);
            }
        }
    }
    tool_precedent_core::run(args, ".")
}
