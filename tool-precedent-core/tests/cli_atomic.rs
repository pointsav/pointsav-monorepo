// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Files that a command rewrites are replaced in one step, never truncated in place, so a crash
//! or a full disk cannot leave half a file. The test hard-links an alias to each target first:
//! an in-place write changes what the alias sees, an atomic replace leaves the alias on the old
//! content. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

struct Work(PathBuf);
impl Work {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-atomic-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(p.join("precedent")).expect("mkdir");
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
    /// Link `alias` to the existing `target` and return the old content.
    fn alias(&self, target: &str, alias: &str) -> String {
        fs::hard_link(self.0.join(target), self.0.join(alias)).expect("link");
        fs::read_to_string(self.0.join(target)).expect("read")
    }
    fn leftovers(&self) -> Vec<String> {
        fn walk(d: &Path, out: &mut Vec<String>) {
            for e in fs::read_dir(d).expect("dir").flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n.contains(".tmp.") {
                    out.push(n);
                }
                if e.path().is_dir() {
                    walk(&e.path(), out);
                }
            }
        }
        let mut v = vec![];
        walk(&self.0, &mut v);
        v
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
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry")
}

#[test]
fn versions_replaces_its_file_in_one_step() {
    let w = Work::new("versions");
    fs::create_dir_all(w.0.join("outputs")).expect("mkdir");
    fs::write(
        w.0.join("outputs/Memo_A_JW1.html"),
        "<html><body>x</body></html>",
    )
    .expect("doc");
    fs::write(w.0.join("precedent/versions.csv"), "old content\n").expect("old");
    let old = w.alias("precedent/versions.csv", "alias.csv");
    assert!(w.run(&["versions"]).status.success());
    assert_eq!(
        fs::read_to_string(w.0.join("alias.csv")).expect("alias"),
        old
    );
    assert_ne!(
        fs::read_to_string(w.0.join("precedent/versions.csv")).expect("new"),
        old
    );
    assert!(w.leftovers().is_empty(), "{:?}", w.leftovers());
}

#[test]
fn apply_replaces_the_page_in_one_step() {
    let w = Work::new("apply");
    copy_dir(&fixtures().join("templates"), &w.0.join("templates"));
    fs::write(
        w.0.join("templates/letter.css"),
        ":root { --made-up: 1; }\n",
    )
    .expect("token");
    fs::write(
        w.0.join("precedent/manifest.csv"),
        "instrument,family,components\npage,letter,recital\n",
    )
    .expect("manifest");
    fs::write(
        w.0.join("page.html"),
        "<html><head><style>\n.x { color: red; }\n</style></head><body>x</body></html>\n",
    )
    .expect("page");
    let old = w.alias("page.html", "alias.html");
    let o = w.run(&["apply", "page.html"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(
        fs::read_to_string(w.0.join("alias.html")).expect("alias"),
        old
    );
    assert!(w.leftovers().is_empty(), "{:?}", w.leftovers());
}

#[test]
fn registry_outputs_are_replaced_in_one_step() {
    let w = Work::new("registry");
    copy_dir(
        &fixtures().join("precedent/registry"),
        &w.0.join("precedent/registry"),
    );
    fs::copy(
        fixtures().join("precedent/components.csv"),
        w.0.join("precedent/components.csv"),
    )
    .expect("components");
    fs::create_dir_all(w.0.join("gen/css")).expect("mkdir");
    fs::write(w.0.join("gen/css/recital.css"), "old css\n").expect("old css");
    fs::write(w.0.join("tokens.json"), "old json\n").expect("old json");
    let old_css = w.alias("gen/css/recital.css", "alias.css");
    let old_json = w.alias("tokens.json", "alias.json");
    assert!(w.run(&["registry", "css", "--out", "gen"]).status.success());
    assert!(w
        .run(&["registry", "dtcg", "--out", "tokens.json"])
        .status
        .success());
    assert_eq!(
        fs::read_to_string(w.0.join("alias.css")).expect("css"),
        old_css
    );
    assert_eq!(
        fs::read_to_string(w.0.join("alias.json")).expect("json"),
        old_json
    );
    assert!(w.leftovers().is_empty(), "{:?}", w.leftovers());
}
