// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Root confinement of CLI write targets.
//! Every path a write command writes to must stay inside `--root`, and a relative path means
//! relative to `--root`, not to the current folder. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

struct Area {
    base: PathBuf,
}
impl Area {
    fn new(name: &str) -> Self {
        let base = std::env::temp_dir().join(format!("tpc-confine-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("root")).expect("root");
        fs::create_dir_all(base.join("outside")).expect("outside");
        fs::create_dir_all(base.join("elsewhere")).expect("elsewhere");
        Area { base }
    }
    fn root(&self) -> PathBuf {
        self.base.join("root")
    }
    fn outside(&self) -> PathBuf {
        self.base.join("outside")
    }
    /// Run from `elsewhere`, so a current-folder-relative path would land there.
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(self.root())
            .current_dir(self.base.join("elsewhere"))
            .output()
            .expect("run")
    }
    fn names(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .map(|d| {
                d.flatten()
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        v.sort();
        v
    }
}
impl Drop for Area {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn change(file: &str) -> Vec<&str> {
    vec![
        "changes-add",
        "--instrument",
        "Memo_A",
        "--version",
        "JW2",
        "--baseline",
        "JW1",
        "--section",
        "s1",
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
        "--file",
        file,
    ]
}
fn term(file: &str) -> Vec<&str> {
    vec![
        "terms-add",
        "--instrument",
        "Memo_A",
        "--version",
        "JW2",
        "--term",
        "holder",
        "--file",
        file,
    ]
}
fn draw(file: &str) -> Vec<&str> {
    vec![
        "draw-add",
        "--instrument",
        "Memo_B",
        "--version",
        "JW1",
        "--from",
        "Memo_A",
        "--from-version",
        "JW3",
        "--file",
        file,
    ]
}

#[test]
fn a_relative_file_flag_means_relative_to_root_not_to_the_current_folder() {
    let a = Area::new("rel");
    let o = a.run(&change("precedent/c.csv"));
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(
        a.root().join("precedent/c.csv").exists(),
        "written under --root"
    );
    assert!(
        Area::names(&a.base.join("elsewhere")).is_empty(),
        "nothing in the current folder"
    );
}

#[test]
fn append_commands_refuse_a_parent_escape_and_write_nothing_outside() {
    for (label, args) in [
        ("changes-add", change("../outside/x.csv")),
        ("terms-add", term("../outside/x.csv")),
        ("draw-add", draw("../outside/x.csv")),
    ] {
        let a = Area::new(label);
        let o = a.run(&args);
        assert!(!o.status.success(), "{label} must refuse");
        assert!(
            String::from_utf8_lossy(&o.stderr).contains("escapes root"),
            "{label}: {}",
            String::from_utf8_lossy(&o.stderr)
        );
        assert!(
            Area::names(&a.outside()).is_empty(),
            "{label} wrote outside the root"
        );
    }
}

#[test]
fn append_commands_refuse_an_absolute_path_outside_the_root() {
    let a = Area::new("abs");
    let target = a.outside().join("abs.csv");
    let o = a.run(&change(target.to_str().expect("utf8")));
    assert!(!o.status.success());
    assert!(!target.exists());
}

#[test]
fn versions_out_refuses_an_escape_and_writes_nothing_outside() {
    let a = Area::new("versions");
    let o = a.run(&["versions", "--out", "../outside/v.csv"]);
    assert!(!o.status.success());
    assert!(Area::names(&a.outside()).is_empty());
    let o = a.run(&["versions", "--out", "precedent/v.csv"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(a.root().join("precedent/v.csv").exists());
}

#[cfg(unix)]
#[test]
fn a_symlink_that_leaves_the_root_is_refused() {
    let a = Area::new("symlink");
    std::os::unix::fs::symlink(a.outside(), a.root().join("link")).expect("symlink");
    let o = a.run(&change("link/x.csv"));
    assert!(!o.status.success());
    assert!(Area::names(&a.outside()).is_empty());
}

#[test]
fn a_new_folder_inside_the_root_is_still_created() {
    let a = Area::new("nested");
    let o = a.run(&change("precedent/new/deep/c.csv"));
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(a.root().join("precedent/new/deep/c.csv").exists());
}

const STYLE_PAGE: &str =
    "<html><head><style>\n.x { color: red; }\n</style></head><body><p>made-up</p></body></html>\n";

fn set_up_apply(a: &Area) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
    copy_dir(&fixtures.join("templates"), &a.root().join("templates"));
    fs::write(
        a.root().join("templates/letter.css"),
        ":root { --made-up: 1; }\n",
    )
    .expect("token");
    fs::create_dir_all(a.root().join("precedent")).expect("mkdir");
    fs::write(
        a.root().join("precedent/manifest.csv"),
        "instrument,family,components\npage,letter,recital\n",
    )
    .expect("manifest");
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
fn apply_reads_and_writes_its_target_relative_to_root() {
    let a = Area::new("apply-rel");
    set_up_apply(&a);
    fs::write(a.root().join("page.html"), STYLE_PAGE).expect("page");
    let o = a.run(&["apply", "page.html"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_ne!(
        fs::read_to_string(a.root().join("page.html")).expect("read"),
        STYLE_PAGE
    );
    assert!(Area::names(&a.base.join("elsewhere")).is_empty());
}

#[test]
fn apply_refuses_a_target_outside_the_root_and_leaves_it_unchanged() {
    let a = Area::new("apply-escape");
    set_up_apply(&a);
    let outside = a.outside().join("page.html");
    fs::write(&outside, STYLE_PAGE).expect("page");
    for t in ["../outside/page.html", outside.to_str().expect("utf8")] {
        let o = a.run(&["apply", t]);
        assert!(!o.status.success(), "{t} must be refused");
        assert_eq!(fs::read_to_string(&outside).expect("read"), STYLE_PAGE);
    }
    let o = a.run(&["apply", "../outside/page.html", "--check"]);
    assert!(
        !o.status.success(),
        "--check reads the same path, so it is confined too"
    );
}

fn set_up_registry(a: &Area) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
    copy_dir(
        &fixtures.join("precedent/registry"),
        &a.root().join("precedent/registry"),
    );
}

#[test]
fn registry_out_is_root_relative_and_refuses_an_escape() {
    let a = Area::new("reg-out");
    set_up_registry(&a);
    let o = a.run(&["registry", "css", "--out", "gen"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(a.root().join("gen/css").is_dir(), "written under --root");
    assert!(Area::names(&a.base.join("elsewhere")).is_empty());
    for args in [
        ["registry", "css", "--out", "../outside/gen"],
        ["registry", "dtcg", "--out", "../outside/t.json"],
    ] {
        let o = a.run(&args);
        assert!(!o.status.success(), "{args:?} must be refused");
    }
    assert!(Area::names(&a.outside()).is_empty());
    let o = a.run(&["registry", "dtcg", "--out", "tokens.json"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(a.root().join("tokens.json").exists());
}

#[test]
fn registry_css_refuses_a_component_name_that_climbs_out_of_the_output_folder() {
    let a = Area::new("reg-name");
    set_up_registry(&a);
    let geo = a.root().join("precedent/registry/ts-geometry.csv");
    let text = fs::read_to_string(&geo).expect("read");
    let hostile = text.replacen(",canada,constitutional-agreement,", ",canada,../escape,", 1);
    assert_ne!(hostile, text, "fixture row not found");
    fs::write(&geo, hostile).expect("write");
    let o = a.run(&["registry", "css", "--out", "gen"]);
    assert!(!o.status.success(), "must refuse the hostile name");
    assert!(
        !a.root().join("gen/escape.css").exists(),
        "no file outside the css folder"
    );
}
