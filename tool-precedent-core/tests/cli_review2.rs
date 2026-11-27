// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Gaps found by the second independent review: symlinks on the default write path, a dangling
//! symlink as a write target, a flag that swallows the next flag, `versions --dir` outside the
//! root, a short `components.csv` row, and manifest ids that climb out of `templates/`.
//! Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

struct Area(PathBuf);
impl Area {
    fn new(name: &str) -> Self {
        let b = std::env::temp_dir().join(format!("tpc-r2-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&b);
        fs::create_dir_all(b.join("root")).expect("root");
        fs::create_dir_all(b.join("outside")).expect("outside");
        Area(b)
    }
    fn root(&self) -> PathBuf {
        self.0.join("root")
    }
    fn outside(&self) -> PathBuf {
        self.0.join("outside")
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(self.root())
            .output()
            .expect("run")
    }
    fn outside_is_empty(&self) -> bool {
        fs::read_dir(self.outside())
            .map(|d| d.count() == 0)
            .unwrap_or(true)
    }
}
impl Drop for Area {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn change() -> Vec<&'static str> {
    vec![
        "changes-add",
        "--instrument",
        "M",
        "--version",
        "JW2",
        "--baseline",
        "JW1",
        "--section",
        "s",
        "--old",
        "a",
        "--new",
        "b",
        "--tag",
        "Replaced",
        "--decision",
        "D1",
        "--date",
        "2026-01-02",
    ]
}

#[cfg(unix)]
#[test]
fn the_default_target_is_refused_when_precedent_is_a_symlink_out_of_the_root() {
    let a = Area::new("f1");
    std::os::unix::fs::symlink(a.outside(), a.root().join("precedent")).expect("link");
    let o = a.run(&change());
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert!(a.outside_is_empty(), "wrote outside the root");
}

#[cfg(unix)]
#[test]
fn a_dangling_symlink_as_the_target_is_refused() {
    let a = Area::new("f2");
    fs::create_dir_all(a.root().join("precedent")).expect("mkdir");
    std::os::unix::fs::symlink(
        a.outside().join("new.csv"),
        a.root().join("precedent/changes.csv"),
    )
    .expect("link");
    let o = a.run(&change());
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert!(a.outside_is_empty(), "created a file outside the root");
}

#[test]
fn a_flag_followed_by_another_flag_has_no_value() {
    let a = Area::new("f7");
    let o = a.run(&[
        "terms-add",
        "--instrument",
        "M",
        "--version",
        "JW1",
        "--term",
        "--status",
        "retired",
    ]);
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert!(
        !a.root().join("precedent/terms.csv").exists(),
        "a row was recorded"
    );
}

#[test]
fn a_value_that_starts_with_dashes_but_is_not_a_flag_is_kept() {
    let a = Area::new("f7b");
    let mut args = change();
    let i = args.iter().position(|x| *x == "--old").expect("old");
    args[i + 1] = "-- an aside";
    assert!(a.run(&args).status.success());
    let text = fs::read_to_string(a.root().join("precedent/changes.csv")).expect("read");
    assert!(text.contains("-- an aside"), "{text}");
}

#[test]
fn versions_dir_outside_the_root_is_refused() {
    let a = Area::new("f11");
    fs::write(a.outside().join("Doc_JW1.html"), "<html></html>").expect("doc");
    let d = a.outside();
    let o = a.run(&[
        "versions",
        "--dir",
        d.to_str().expect("utf8"),
        "--out",
        "v.csv",
    ]);
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert!(!a.root().join("v.csv").exists());
    let o = a.run(&["versions", "--dir", "../outside", "--out", "v.csv"]);
    assert!(!o.status.success());
}

#[test]
fn a_short_components_row_is_a_finding_not_a_panic() {
    let a = Area::new("f5");
    let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
    fn copy(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("mkdir");
        for e in fs::read_dir(from).expect("dir").flatten() {
            let d = to.join(e.file_name());
            if e.path().is_dir() {
                copy(&e.path(), &d);
            } else {
                fs::copy(e.path(), d).expect("copy");
            }
        }
    }
    copy(&fx.join("precedent"), &a.root().join("precedent"));
    copy(&fx.join("templates"), &a.root().join("templates"));
    fs::write(
        a.root().join("precedent/components.csv"),
        "a,b,c,d,e,id,file,hash\n1,2,3,4,5\n",
    )
    .expect("components");
    let o = a.run(&["registry", "check"]);
    assert_eq!(
        o.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn manifest_ids_that_climb_out_of_templates_are_refused() {
    let a = Area::new("f10");
    fs::create_dir_all(a.root().join("precedent")).expect("mkdir");
    fs::create_dir_all(a.root().join("templates")).expect("mkdir");
    fs::write(a.0.join("secret.css"), "body { color: #123; }\n").expect("outside css");
    fs::write(
        a.root().join("precedent/manifest.csv"),
        "instrument,family,components\npage,../../secret,\n",
    )
    .expect("manifest");
    let html = "<html><head><style>\n.x{}\n</style></head><body>x</body></html>\n";
    fs::write(a.root().join("page.html"), html).expect("page");
    let o = a.run(&["apply", "page.html"]);
    assert!(!o.status.success());
    assert_eq!(
        fs::read_to_string(a.root().join("page.html")).expect("read"),
        html
    );
    assert!(!String::from_utf8_lossy(&o.stderr).contains("#123"));
}
