use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug)]
pub enum PullResult {
    Advanced { new_sha: String },
    UpToDate,
}

/// Fetch + fast-forward merge from the named remote.
///
/// Constraints:
///   - NEVER pushes, NEVER creates commits, NEVER branches.
///   - Fast-forward only — diverged remote returns an error.
///   - Consistent with the one-way diode standard for os-mediakit deployments.
pub fn pull(repo_path: &Path, remote_name: &str) -> Result<PullResult> {
    let repo = git2::Repository::open(repo_path).context("open repo")?;

    let mut remote = repo.find_remote(remote_name)
        .with_context(|| format!("remote '{remote_name}' not found"))?;

    remote.fetch(&[] as &[&str], Some(&mut git2::FetchOptions::new()), None)
        .context("git fetch")?;
    drop(remote);

    let fetch_head   = repo.find_reference("FETCH_HEAD").context("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).context("resolve FETCH_HEAD")?;
    let (analysis, _) = repo.merge_analysis(&[&fetch_commit]).context("merge analysis")?;

    if analysis.is_up_to_date() { return Ok(PullResult::UpToDate); }
    if !analysis.is_fast_forward() {
        anyhow::bail!("remote has diverged — fast-forward not possible");
    }

    let refname = repo.head().context("HEAD")?.name()
        .context("HEAD has no name")?.to_string();
    repo.find_reference(&refname).context("find ref")?
        .set_target(fetch_commit.id(), "fast-forward").context("set target")?;
    repo.set_head(&refname).context("set HEAD")?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        .context("checkout")?;

    Ok(PullResult::Advanced { new_sha: fetch_commit.id().to_string() })
}
