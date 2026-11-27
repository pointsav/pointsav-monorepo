// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Numbers in the registry that cannot be read as finite numbers must stop `registry dtcg` and be
//! reported by `registry check`, never turn into 0.0 or `NaN` in the token file. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

struct Work(PathBuf);
impl Work {
    fn new(name: &str, size: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-numbers-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
        copy_dir(&fx.join("precedent"), &p.join("precedent"));
        copy_dir(&fx.join("templates"), &p.join("templates"));
        let f = p.join("precedent/registry/ts-families.csv");
        let text = fs::read_to_string(&f).expect("read");
        let bad = text.replacen(",11.5pt,", &format!(",{size},"), 1);
        assert_ne!(bad, text, "fixture body_size not found");
        fs::write(&f, bad).expect("write");
        Work(p)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()
            .expect("run")
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir");
    for e in fs::read_dir(from).expect("read_dir").flatten() {
        let dest = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &dest);
        } else {
            fs::copy(e.path(), dest).expect("copy");
        }
    }
}

#[test]
fn dtcg_refuses_a_size_that_is_not_a_finite_number() {
    for size in ["nanpt", "infpt", "abcpt", "1.2.3pt"] {
        let w = Work::new(size, size);
        let o = w.run(&["registry", "dtcg"]);
        let out = String::from_utf8_lossy(&o.stdout);
        assert!(!o.status.success(), "{size}: must refuse; stdout {out}");
        assert!(
            !out.contains("NaN") && !out.contains("inf"),
            "{size}: {out}"
        );
    }
}

#[test]
fn check_reports_a_size_that_is_not_a_finite_number() {
    let w = Work::new("check", "nanpt");
    let o = w.run(&["registry", "check"]);
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    assert_eq!(o.status.code(), Some(1), "{all}");
    assert!(all.contains("body_size"), "{all}");
}
