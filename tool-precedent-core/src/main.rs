// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
fn main() -> std::process::ExitCode {
    tool_precedent_core::run(std::env::args().skip(1).collect(), ".")
}
