//! CLI for tool-precedent-drawings. Phase 0 subcommand only:
//!   catalog [--root <data-root>] [--out <precedent/drawings.csv>]
//!
//! `--root` is never defaulted to a business-admin path -- same idiom as
//! tool-documents-core::main::run_catalog -- the caller supplies the real
//! data root (e.g. `business-admin/project-drawings`) at invocation
//! time; the compiled-in default is just `.`.
//!
//! Real document filenames/categories/structural counts must never land
//! somewhere that could reach GitHub, even though this crate's own code is
//! meant to be promoted there -- so `--out` is refused outright if it
//! resolves inside any git repo that has a remote configured. Deliberately
//! not a hardcoded path list (an archive name changed once already this
//! same day) -- walks up from the resolved path looking for the nearest
//! `.git`, and checks that repo's own config for a `[remote ...]` section.
//! A zero-remote repo (every business-admin archive) always passes.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use tool_precedent_drawings::catalog;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else {
        eprintln!("usage: tool-precedent-drawings <catalog> ...");
        return ExitCode::FAILURE;
    };
    match cmd.as_str() {
        "catalog" => run_catalog(&args[1..]),
        other => {
            eprintln!("unknown subcommand: {other}");
            ExitCode::FAILURE
        }
    }
}

fn flag_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

/// Walks up from `path` (resolved against the current directory if
/// relative; the path itself need not exist yet) looking for the nearest
/// ancestor directory containing a `.git` entry. Returns that ancestor,
/// or `None` if no `.git` is found before the filesystem root.
fn nearest_git_repo(path: &Path) -> Option<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };
    let mut dir = absolute.parent().map(Path::to_path_buf);
    while let Some(candidate) = dir {
        if candidate.join(".git").exists() {
            return Some(candidate);
        }
        dir = candidate.parent().map(Path::to_path_buf);
    }
    None
}

/// A business-admin archive's git repo has zero remotes by design. A
/// GitHub-tracked one always has at least `origin`. Checked by reading
/// `.git/config` directly rather than shelling out to `git remote` -- this
/// crate has a zero-third-party-dependency AND zero-external-process bar.
fn git_repo_has_a_remote(repo_root: &Path) -> bool {
    std::fs::read_to_string(repo_root.join(".git").join("config"))
        .map(|config| config.contains("[remote \""))
        .unwrap_or(false)
}

fn run_catalog(args: &[String]) -> ExitCode {
    let root = flag_value(args, "--root")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let out = flag_value(args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("precedent").join("drawings.csv"));

    if let Some(repo_root) = nearest_git_repo(&out) {
        if git_repo_has_a_remote(&repo_root) {
            eprintln!(
                "refusing to write --out {} -- {} is a git repo with a remote configured \
                 (real document names/categories/counts must never land somewhere that can \
                 reach GitHub); pass an --out path outside any remote-tracked repo",
                out.display(),
                repo_root.display()
            );
            return ExitCode::FAILURE;
        }
    }

    let rows = catalog::scan(&root);
    if let Some(parent) = out.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match catalog::write_csv(&rows, &out) {
        Ok(()) => {
            println!("wrote {} drawing rows to {}", rows.len(), out.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error writing {}: {e}", out.display());
            ExitCode::FAILURE
        }
    }
}
