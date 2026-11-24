// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Reachable panics and unbounded loops found in review. Each must end with a
//! clear error or a result, never a panic (exit 101) or a hang. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

fn work(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("tpc-panics-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).expect("mkdir");
    p
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
fn a_huge_version_number_in_a_file_name_does_not_hang_the_lineage_report() {
    let w = work("lineage");
    let dir = w.join("service-wip");
    fs::create_dir_all(&dir).expect("mkdir");
    fs::write(
        dir.join("X_JW4294967295.html"),
        "<html><body>x</body></html>",
    )
    .expect("file");
    let mut child = Command::new(BIN)
        .args(["lineage-report", "--dir", "service-wip", "--root"])
        .arg(&w)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");
    let start = Instant::now();
    let finished = loop {
        if let Some(status) = child.try_wait().expect("wait") {
            break Some(status);
        }
        if start.elapsed() > Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let _ = fs::remove_dir_all(&w);
    let status = finished.expect("lineage-report ran longer than 10 s on one file");
    assert!(
        status.code().is_some_and(|c| c != 101),
        "panicked: {status:?}"
    );
}

#[cfg(unix)]
#[test]
fn a_command_line_argument_that_is_not_utf8_is_an_error_not_a_panic() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let bad = OsStr::from_bytes(b"find\xff");
    let o = Command::new(BIN).arg(bad).output().expect("run");
    assert_eq!(
        o.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(String::from_utf8_lossy(&o.stderr).contains("UTF-8"));
}

#[test]
fn an_evidence_line_of_zero_is_a_finding_not_a_panic() {
    let w = work("evidence");
    let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
    copy_dir(
        &fx.join("precedent/registry"),
        &w.join("precedent/registry"),
    );
    copy_dir(&fx.join("templates"), &w.join("templates"));
    fs::copy(
        fx.join("precedent/components.csv"),
        w.join("precedent/components.csv"),
    )
    .expect("components");
    let geo = w.join("precedent/registry/ts-geometry.csv");
    let text = fs::read_to_string(&geo).expect("read");
    let zero = text.replace("ca-classes.css:3;", "ca-classes.css:0;");
    assert_ne!(zero, text, "fixture evidence not found");
    fs::write(&geo, zero).expect("write");
    let o = Command::new(BIN)
        .args(["registry", "check", "--root"])
        .arg(&w)
        .output()
        .expect("run");
    let _ = fs::remove_dir_all(&w);
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    assert_eq!(o.status.code(), Some(1), "{all}");
    assert!(all.contains("evidence line 0"), "{all}");
}
