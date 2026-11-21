// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Scans the made-up pages in tests/fixtures/pages. No real page is read by any test.

use tool_collate::scan::{scan_dir, Report};

fn fixtures() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pages")
}

#[test]
fn counts_the_made_up_pages() {
    let (pages, r) = scan_dir(&fixtures(), &[]).unwrap();
    assert_eq!(
        r,
        Report {
            files_found: 5,
            unreadable: 1,
            pages_read: 4,
            frontmatter_ok: 2,
            frontmatter_missing: 1,
            frontmatter_invalid: 1,
            links_checked: 4,
            links_broken: 2,
            links_not_checked: 3,
            skipped: 1,
        }
    );
    assert_eq!(pages.len(), 4);
}

#[test]
fn a_hidden_folder_is_entered_only_when_kept() {
    let (_, r) = scan_dir(&fixtures(), &[".hidden-folder"]).unwrap();
    assert_eq!(r.files_found, 6);
    assert_eq!(r.skipped, 0);
}

#[test]
fn the_report_prints_counts_and_no_names() {
    let (_, r) = scan_dir(&fixtures(), &[]).unwrap();
    let text = r.to_string();
    assert!(text.contains("broken links:         2"));
    assert!(!text.contains(".md"));
}
