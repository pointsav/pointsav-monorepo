// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Live parity: the Rust registry module against project-documents `bin/script-typeset-reference.py` on the REAL registry.
//! Runs only when TS_PROJECT_DOCUMENTS names a project-documents checkout (no default path);
//! elsewhere it prints a skip line and passes. The fixture goldens in tests/fixtures/registry cover every machine.
use std::path::PathBuf;
use std::process::Command;
use tool_precedent_core::registry::Registry;

fn project() -> Option<PathBuf> {
    // Opt-in: only when TS_PROJECT_DOCUMENTS names a checkout that has the Python generator and a real registry.
    let p = PathBuf::from(std::env::var("TS_PROJECT_DOCUMENTS").ok()?);
    if p.join("bin/script-typeset-reference.py").exists()
        && p.join("precedent/registry/ts-geometry.csv").exists()
    {
        Some(p)
    } else {
        None
    }
}
fn py(root: &PathBuf, args: &[&str]) -> Option<String> {
    let o = Command::new("python3")
        .arg(root.join("bin/script-typeset-reference.py"))
        .args(args)
        .current_dir(root)
        .output()
        .ok()?;
    if o.status.success() {
        Some(String::from_utf8_lossy(&o.stdout).into_owned())
    } else {
        None
    }
}

#[test]
fn css_and_dtcg_match_the_python_generator_on_the_real_registry() {
    let Some(root) = project() else {
        eprintln!("skip: project-documents not present");
        return;
    };
    let reg = Registry::load(&root.join("precedent/registry")).expect("registry");
    let tmp = std::env::temp_dir().join(format!("tpc-registry-parity-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    if py(&root, &["css", "--out", tmp.to_str().unwrap()]).is_none() {
        eprintln!("skip: python3 generator not runnable");
        return;
    }
    let files = reg.css_files(None);
    assert!(
        files.len() > 20,
        "expected the full component list, got {}",
        files.len()
    );
    for (c, t) in &files {
        assert_eq!(
            &std::fs::read_to_string(tmp.join("css").join(format!("{c}.css"))).expect("python css"),
            t,
            "css {c}"
        );
    }
    assert_eq!(
        std::fs::read_dir(tmp.join("css")).unwrap().count(),
        files.len(),
        "component lists differ"
    );
    assert_eq!(
        format!("{}\n", reg.dtcg(None)),
        py(&root, &["dtcg"]).unwrap(),
        "dtcg all"
    );
    for fam in ["canada", "fideicomiso", "prospecto", "letter", "memo"] {
        assert_eq!(
            format!("{}\n", reg.dtcg(Some(fam))),
            py(&root, &["dtcg", "--family", fam]).unwrap(),
            "dtcg {fam}"
        );
    }
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn real_registry_checks_clean() {
    let Some(root) = project() else {
        eprintln!("skip: project-documents not present");
        return;
    };
    let reg = Registry::load(&root.join("precedent/registry")).expect("registry");
    let errs: Vec<String> = reg
        .check(&root)
        .into_iter()
        .filter(|f| f.error)
        .map(|f| f.msg)
        .collect();
    assert!(errs.is_empty(), "{errs:?}");
}
