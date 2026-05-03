use std::path::Path;
use crate::error::WikiError;
use chrono::{TimeZone, Utc};

pub struct HistoryEntry {
    pub sha: String,
    pub author: String,
    pub email: String,
    pub timestamp_iso: String,
    pub message: String,
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
            
            entries.push(HistoryEntry {
                sha: commit_item.id().to_string(),
                author: author.name.to_string(),
                email: author.email.to_string(),
                timestamp_iso: format_time(time),
                message: message.summary().to_string(),
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
    
    let author = commit.author().map_err(|e| WikiError::WriteFailed(format!("gix author failed: {e}")))?;
    let author_name = author.name.to_string();
    let sha = id.to_string();
    let time = commit.time().map_err(|e| WikiError::WriteFailed(format!("gix time failed: {e}")))?;
    let timestamp_iso = format_time(time);
    
    // TODO: Implement actual per-line blame using gix-blame 0.13.0
    // For now, return lines annotated with the HEAD commit as a high-fidelity placeholder
    let mut lines = Vec::new();
    for (i, line) in content.lines().enumerate() {
        lines.push(BlameLine {
            line_number: i + 1,
            line_text: line.to_string(),
            sha: sha.clone(),
            author: author_name.clone(),
            timestamp_iso: timestamp_iso.clone(),
        });
    }
    
    Ok(lines)
}

fn format_time(time: gix::date::Time) -> String {
    let dt = Utc.timestamp_opt(time.seconds, 0).unwrap();
    dt.to_rfc3339()
}
