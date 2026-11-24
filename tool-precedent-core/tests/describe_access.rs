// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The access label of a command must not understate what it can do: a
//! command with an `--out` flag writes a file, so it is not `read`, and `describe --json` must
//! not tell an AI it may run it freely. Made-up data only.
use tool_precedent_core::commands::{Access, COMMANDS};

#[test]
fn a_command_with_an_out_flag_is_not_labelled_read() {
    for c in COMMANDS {
        if c.flags.iter().any(|f| f.name == "--out") {
            assert_ne!(
                c.access,
                Access::Read,
                "{} has --out but is labelled read",
                c.name
            );
        }
    }
}

#[test]
fn registry_is_reported_to_an_ai_as_at_a_persons_instruction() {
    let json = tool_precedent_core::commands::describe_json();
    let at = json.find("\"registry\"").expect("registry in describe");
    let chunk = &json[at..json[at..]
        .find("\"shared-hash\"")
        .map_or(json.len(), |e| at + e)];
    assert!(chunk.contains("at-a-persons-instruction"), "{chunk}");
    assert!(!chunk.contains("\"free\""), "{chunk}");
}
