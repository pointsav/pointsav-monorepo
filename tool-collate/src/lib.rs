// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! tool-collate: the shared page reader. Walks a folder of Markdown pages, reads each page's
//! frontmatter (optional: Ok, Missing or Invalid) and structure (headings, links, tables, code
//! blocks), and reports counts. Read-only. It holds no rule about any one kind of page: wiki
//! links, generated blocks and index checks belong to the tools for those kinds, which depend on
//! this crate for the reading.

pub mod frontmatter;
pub mod markdown;
pub mod page;
pub mod scan;

pub use frontmatter::{Frontmatter, FrontmatterState};
pub use page::Page;
pub use scan::{scan, Report};

/// Command line: `tool-collate scan <folder> [--keep <hidden-folder>]...`. Returns the exit code.
pub fn run(args: Vec<String>) -> std::process::ExitCode {
    use std::process::ExitCode;
    let usage = "usage: tool-collate scan <folder> [--keep <hidden-folder>]...";
    let mut it = args.into_iter();
    if it.next().as_deref() != Some("scan") {
        eprintln!("{usage}");
        return ExitCode::from(2);
    }
    let Some(dir) = it.next() else {
        eprintln!("{usage}");
        return ExitCode::from(2);
    };
    let mut keep = Vec::new();
    while let Some(a) = it.next() {
        match (a.as_str(), it.next()) {
            ("--keep", Some(name)) => keep.push(name),
            _ => {
                eprintln!("{usage}");
                return ExitCode::from(2);
            }
        }
    }
    let keep_refs: Vec<&str> = keep.iter().map(String::as_str).collect();
    match scan::scan_dir(std::path::Path::new(&dir), &keep_refs) {
        Ok((_, report)) => {
            print!("{report}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("tool-collate: {e}");
            ExitCode::from(2)
        }
    }
}
