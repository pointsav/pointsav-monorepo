// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `trace` gate: requirements and tests agree.
//!
//! A test names the requirement it covers with a line `// REQ: FOLIO-REQ-nnn`.

use crate::util::{family_rust_files, read, Res, Verdict};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const PREFIX: &str = "FOLIO-REQ-";

/// Requirement id to status, from the `| FOLIO-REQ-nnn | ... | status |` rows.
fn requirements(text: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("| FOLIO-REQ-") else {
            continue;
        };
        let cells: Vec<&str> = rest.split('|').map(str::trim).collect();
        if let (Some(id), Some(status)) = (cells.first(), cells.get(4)) {
            map.insert(format!("{PREFIX}{id}"), (*status).to_string());
        }
    }
    map
}

/// Requirement ids named in `// REQ:` tag lines, with `file:line` locations.
fn tags(root: &Path) -> Res<BTreeMap<String, Vec<String>>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in family_rust_files(root)? {
        let text = read(&file)?;
        for (n, line) in text.lines().enumerate() {
            let Some(rest) = line.trim().strip_prefix("// REQ:") else {
                continue;
            };
            for id in rest.split([',', ' ']).filter(|t| t.starts_with(PREFIX)) {
                let place = format!("{}:{}", file.display(), n.saturating_add(1));
                map.entry(id.to_string()).or_default().push(place);
            }
        }
    }
    Ok(map)
}

/// Requirement ids named in `assurance/TRACE.tsv` (first column, header skipped).
fn trace_rows(root: &Path) -> Res<BTreeSet<String>> {
    let text = read(&root.join("assurance/TRACE.tsv"))?;
    let ids = text
        .lines()
        .skip(1)
        .filter_map(|l| l.split('\t').next())
        .filter(|c| !c.is_empty())
        .map(str::to_string)
        .collect();
    Ok(ids)
}

/// Run the gate.
pub(crate) fn run(root: &Path) -> Res<Verdict> {
    let reqs = requirements(&read(&root.join("assurance/REQUIREMENTS.md"))?);
    let tagged = tags(root)?;
    let rows = trace_rows(root)?;
    let mut problems = Vec::new();
    for id in tagged.keys().chain(rows.iter()) {
        if !reqs.contains_key(id) {
            problems.push(format!("unknown requirement named: {id}"));
        }
    }
    let mut draft_uncovered = 0_usize;
    for (id, status) in &reqs {
        if tagged.contains_key(id) || rows.contains(id) {
            continue;
        }
        if status == "draft" {
            draft_uncovered = draft_uncovered.saturating_add(1);
        } else {
            problems.push(format!("{id} is {status} but has no test or trace row"));
        }
    }
    if problems.is_empty() {
        let msg = format!(
            "{} requirements; {draft_uncovered} draft without tests",
            reqs.len()
        );
        return Ok(Verdict::Pass(msg));
    }
    Ok(Verdict::Fail(problems))
}
