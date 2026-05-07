use std::path::Path;
use crate::error::WikiError;
use chrono::{TimeZone, Utc};

pub struct HistoryEntry {
    pub sha: String,
    pub author: String,
    pub email: String,
    pub timestamp_iso: String,
    /// First line of the commit message (the subject).
    pub message: String,
    /// User-supplied edit summary — the body paragraph after the blank line,
    /// stripped of any "Version: M.m.P" trailers. Empty when absent.
    pub edit_summary: String,
}

pub struct BlameLine {
    pub line_number: usize,
    pub line_text: String,
    pub sha: String,
    pub author: String,
    pub timestamp_iso: String,
}

pub fn topic_history(
    content_dir: &Path,
    slug: &str,
    limit: usize,
) -> Result<Vec<HistoryEntry>, WikiError> {
    let repo = gix::open(content_dir).map_err(|e| WikiError::WriteFailed(format!("gix open failed: {e}")))?;
    let path = Path::new(slug).with_extension("md");
    
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return Ok(Vec::new()),
    };
    
    let id = match head.id() {
        Some(id) => id,
        None => return Ok(Vec::new()),
    };

    let mut entries = Vec::new();
    let mut current_blob_id = None;

    // Walk ancestors to find commits that changed the file
    let ancestors = id.ancestors().all().map_err(|e| WikiError::WriteFailed(format!("gix ancestors failed: {e}")))?;
    
    for commit_item in ancestors {
        let commit_item = commit_item.map_err(|e| WikiError::WriteFailed(format!("gix commit walk failed: {e}")))?;
        let commit = commit_item.object().map_err(|e| WikiError::WriteFailed(format!("gix commit object failed: {e}")))?;
        
        let tree = commit.tree().map_err(|e| WikiError::WriteFailed(format!("gix tree failed: {e}")))?;
        
        let blob_id = match tree.lookup_entry_by_path(&path) {
            Ok(Some(entry)) => Some(entry.oid().to_owned()),
            _ => None,
        };

        if blob_id != current_blob_id {
            let author = commit.author().map_err(|e| WikiError::WriteFailed(format!("gix author failed: {e}")))?;
            let message = commit.message().map_err(|e| WikiError::WriteFailed(format!("gix message failed: {e}")))?;
            let time = commit.time().map_err(|e| WikiError::WriteFailed(format!("gix time failed: {e}")))?;
            
            let edit_summary = message.body()
                .map(|b| String::from_utf8_lossy(b.without_trailer().as_ref()).trim().to_string())
                .unwrap_or_default();
            entries.push(HistoryEntry {
                sha: commit_item.id().to_string(),
                author: author.name.to_string(),
                email: author.email.to_string(),
                timestamp_iso: format_time(time),
                message: message.summary().to_string(),
                edit_summary,
            });
            
            current_blob_id = blob_id;
            
            if entries.len() >= limit {
                break;
            }
        }
        
        if current_blob_id.is_none() && !entries.is_empty() {
            // File didn't exist before this or was deleted
            break;
        }
    }

    Ok(entries)
}

pub fn topic_blame(
    content_dir: &Path,
    slug: &str,
) -> Result<Vec<BlameLine>, WikiError> {
    let repo = gix::open(content_dir).map_err(|e| WikiError::WriteFailed(format!("gix open failed: {e}")))?;
    let path = Path::new(slug).with_extension("md");

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return Ok(Vec::new()),
    };
    
    let id = match head.id() {
        Some(id) => id,
        None => return Ok(Vec::new()),
    };

    let commit_obj = id.object().map_err(|e| WikiError::WriteFailed(format!("gix commit object failed: {e}")))?;
    let commit = commit_obj.try_into_commit().map_err(|e| WikiError::WriteFailed(format!("gix not a commit: {e}")))?;
    let tree = commit.tree().map_err(|e| WikiError::WriteFailed(format!("gix tree failed: {e}")))?;
    
    let entry = tree.lookup_entry_by_path(&path)
        .map_err(|e| WikiError::WriteFailed(format!("gix path lookup failed: {e}")))?
        .ok_or_else(|| WikiError::NotFound(slug.to_string()))?;
    
    let blob = entry.object().map_err(|e| WikiError::WriteFailed(format!("gix blob object failed: {e}")))?
        .into_blob();
    let content = String::from_utf8_lossy(&blob.data);
    
    let blame_res = repo.blame_file(
        gix::path::into_bstr(&path).as_ref(),
        id,
        Default::default(),
    ).map_err(|e| WikiError::WriteFailed(format!("gix blame failed: {e}")))?;

    let mut lines = Vec::new();
    let content_lines: Vec<&str> = content.lines().collect();

    for entry in blame_res.entries {
        let commit_obj = repo.find_object(entry.commit_id)
            .map_err(|e| WikiError::WriteFailed(format!("gix find commit failed: {e}")))?;
        let commit = commit_obj.try_into_commit()
            .map_err(|e| WikiError::WriteFailed(format!("gix not a commit: {e}")))?;
        
        let author = commit.author().map_err(|e| WikiError::WriteFailed(format!("gix author failed: {e}")))?;
        let time = commit.time().map_err(|e| WikiError::WriteFailed(format!("gix time failed: {e}")))?;
        let timestamp_iso = format_time(time);
        let author_name = author.name.to_string();
        let sha = entry.commit_id.to_string();

        for i in (entry.start_in_blamed_file as usize)..(entry.start_in_blamed_file as usize + entry.len.get() as usize) {
            if i < content_lines.len() {
                lines.push(BlameLine {
                    line_number: i + 1,
                    line_text: content_lines[i].to_string(),
                    sha: sha.clone(),
                    author: author_name.clone(),
                    timestamp_iso: timestamp_iso.clone(),
                });
            }
        }
    }
    
    Ok(lines)
}

pub struct DiffLine {
    pub change: char, // ' ', '+', '-'
    pub text: String,
}

pub fn topic_diff(
    content_dir: &Path,
    slug: &str,
    a_sha: &str,
    b_sha: &str,
) -> Result<Vec<DiffLine>, WikiError> {
    let repo = gix::open(content_dir).map_err(|e| WikiError::WriteFailed(format!("gix open failed: {e}")))?;
    let path = Path::new(slug).with_extension("md");

    let get_content = |sha: &str| -> Result<String, WikiError> {
        if sha == "" || sha == "0000000000000000000000000000000000000000" {
            return Ok(String::new());
        }
        
        // Handle ~ suffix for parent
        let (actual_sha, is_parent) = if sha.ends_with('~') {
            (&sha[..sha.len()-1], true)
        } else {
            (sha, false)
        };

        let id = repo.rev_parse_single(actual_sha)
            .map_err(|e| WikiError::WriteFailed(format!("gix rev-parse failed: {e}")))?;
        
        let commit = id.object().map_err(|e| WikiError::WriteFailed(format!("gix commit object failed: {e}")))?
            .try_into_commit().map_err(|e| WikiError::WriteFailed(format!("gix not a commit: {e}")))?;
        
        let target_commit = if is_parent {
            let parent_id = commit.parent_ids().next()
                .ok_or_else(|| WikiError::WriteFailed("no parent found".to_string()))?;
            parent_id.object().map_err(|e| WikiError::WriteFailed(format!("gix parent object failed: {e}")))?
                .try_into_commit().map_err(|e| WikiError::WriteFailed(format!("gix parent not a commit: {e}")))?
        } else {
            commit
        };

        let tree = target_commit.tree().map_err(|e| WikiError::WriteFailed(format!("gix tree failed: {e}")))?;
        let entry = tree.lookup_entry_by_path(&path)
            .map_err(|e| WikiError::WriteFailed(format!("gix path lookup failed: {e}")))?
            .ok_or_else(|| WikiError::NotFound(slug.to_string()))?;
        
        let blob = entry.object().map_err(|e| WikiError::WriteFailed(format!("gix blob object failed: {e}")))?
            .into_blob();
        Ok(String::from_utf8_lossy(&blob.data).to_string())
    };

    let content_a = get_content(a_sha).unwrap_or_default();
    let content_b = get_content(b_sha).unwrap_or_default();

    let mut lines = Vec::new();
    let diff = similar::TextDiff::from_lines(&content_a, &content_b);
    
    for change in diff.iter_all_changes() {
        let tag = match change.tag() {
            similar::ChangeTag::Delete => '-',
            similar::ChangeTag::Insert => '+',
            similar::ChangeTag::Equal => ' ',
        };
        lines.push(DiffLine {
            change: tag,
            text: change.to_string(),
        });
    }

    Ok(lines)
}

/// Return the raw Markdown content of `slug` at the given git revision.
/// `rev` may be a SHA prefix, a ref name, or a `SHA~` parent shorthand.
/// Returns an empty string when the file didn't exist at that revision.
pub fn get_file_at_rev(content_dir: &Path, slug: &str, rev: &str) -> Result<String, WikiError> {
    let repo = gix::open(content_dir)
        .map_err(|e| WikiError::WriteFailed(format!("gix open: {e}")))?;
    let path = Path::new(slug).with_extension("md");

    if rev.is_empty() || rev == "0000000000000000000000000000000000000000" {
        return Ok(String::new());
    }
    let (actual_rev, is_parent) = if rev.ends_with('~') {
        (&rev[..rev.len()-1], true)
    } else {
        (rev, false)
    };
    let id = repo.rev_parse_single(actual_rev)
        .map_err(|e| WikiError::WriteFailed(format!("gix rev-parse: {e}")))?;
    let commit = id.object().map_err(|e| WikiError::WriteFailed(format!("gix object: {e}")))?
        .try_into_commit().map_err(|e| WikiError::WriteFailed(format!("not a commit: {e}")))?;
    let target = if is_parent {
        let pid = commit.parent_ids().next()
            .ok_or_else(|| WikiError::WriteFailed("no parent".into()))?;
        pid.object().map_err(|e| WikiError::WriteFailed(format!("parent object: {e}")))?
            .try_into_commit().map_err(|e| WikiError::WriteFailed(format!("parent not commit: {e}")))?
    } else {
        commit
    };
    let tree = target.tree().map_err(|e| WikiError::WriteFailed(format!("tree: {e}")))?;
    match tree.lookup_entry_by_path(&path).map_err(|e| WikiError::WriteFailed(format!("path lookup: {e}")))? {
        Some(entry) => {
            let blob = entry.object().map_err(|e| WikiError::WriteFailed(format!("blob: {e}")))?.into_blob();
            Ok(String::from_utf8_lossy(&blob.data).into_owned())
        }
        None => Ok(String::new()),
    }
}

fn format_time(time: gix::date::Time) -> String {
    let dt = Utc.timestamp_opt(time.seconds, 0).unwrap();
    dt.to_rfc3339()
}
