use super::{EditResult, EditSubmission};
use anyhow::{Context, Result};
use std::path::Path;

/// Validate, reconstruct, and commit an editor submission to the
/// content-wiki-documentation repository.
///
/// ADR compliance:
///   SYS-ADR-19 — automated AI publishing to verified ledgers is prohibited.
///   The editor_identity field must be a verified human identity from the MBA
///   auth layer, validated by the calling handler before this function runs.
pub fn apply(repo_path: &Path, submission: EditSubmission) -> Result<EditResult> {
    if submission.edit_summary.trim().len() < 10 {
        return Ok(EditResult {
            success: false, new_sha: None,
            message: "Edit summary must be at least 10 characters.".into(),
        });
    }

    let repo = git2::Repository::open(repo_path).context("open repo")?;

    // Conflict detection
    let current_head = repo.head().and_then(|h| h.peel_to_commit())
        .map(|c| c.id().to_string()).context("resolve HEAD")?;

    if current_head != submission.base_sha {
        return Ok(EditResult {
            success: false, new_sha: None,
            message: format!(
                "The article was updated while you were editing. \
                 Reload and re-apply your changes. \
                 (Your base: {}, current: {})",
                &submission.base_sha[..8.min(submission.base_sha.len())],
                &current_head[..8],
            ),
        });
    }

    let file_path = find_file(repo_path, &submission.slug)
        .with_context(|| format!("no file for slug '{}'", submission.slug))?;

    let current = std::fs::read_to_string(&file_path).context("read file")?;
    let updated = replace_section(
        &current,
        submission.section_heading.as_deref(),
        &submission.updated_section_markdown,
    ).context("reconstruct file")?;

    std::fs::write(&file_path, &updated).context("write file")?;

    let mut index = repo.index().context("git index")?;
    let rel = file_path.strip_prefix(repo_path).context("strip prefix")?;
    index.add_path(rel).context("git add")?;
    index.write().context("index write")?;

    let tree_oid = index.write_tree().context("write_tree")?;
    let tree     = repo.find_tree(tree_oid).context("find_tree")?;
    let parent   = repo.head().and_then(|h| h.peel_to_commit()).context("parent commit")?;
    let sig      = git2::Signature::now(&submission.editor_identity, "editor@pointsav.internal")
        .context("signature")?;

    let msg = format!(
        "Edited: {} — {}\n\n{}",
        submission.slug,
        submission.section_heading.as_deref().unwrap_or("(full article)"),
        submission.edit_summary,
    );

    let oid = repo.commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&parent])
        .context("commit")?;

    Ok(EditResult { success: true, new_sha: Some(oid.to_string()), message: "Saved.".into() })
}

fn find_file(repo_path: &Path, slug: &str) -> Option<std::path::PathBuf> {
    // TODO: replace with PageIndex lookup from AppState — this walk is
    // acceptable for scaffolding only.
    walkdir::WalkDir::new(repo_path)
        .into_iter().filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "md"))
        .find(|e| e.path().file_stem().and_then(|s| s.to_str()) == Some(slug))
        .map(|e| e.path().to_path_buf())
}

fn replace_section(current: &str, heading: Option<&str>, updated: &str) -> Result<String> {
    let Some(h) = heading else { return Ok(updated.to_string()); };
    let target  = format!("## {h}");
    let lines: Vec<&str> = current.lines().collect();
    let start   = lines.iter().position(|l| l.trim() == target.trim())
        .with_context(|| format!("section '{h}' not found"))?;
    let end     = lines[start + 1..].iter().position(|l| l.starts_with("## "))
        .map(|p| start + 1 + p).unwrap_or(lines.len());
    let before  = lines[..start].join("\n");
    let after   = lines[end..].join("\n");
    Ok(if after.is_empty() {
        format!("{before}\n{updated}")
    } else {
        format!("{before}\n{updated}\n{after}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_middle_section() {
        let c = "---\ntitle: T\n---\nLead.\n\n## S1\n\nOld.\n\n## S2\n\nKept.\n";
        let r = replace_section(c, Some("S1"), "## S1\n\nNew.\n").unwrap();
        assert!(r.contains("New."));
        assert!(r.contains("Kept."));
        assert!(!r.contains("Old."));
    }

    #[test]
    fn missing_section_errors() {
        assert!(replace_section("## Real\n\nContent.\n", Some("Ghost"), "x").is_err());
    }
}
