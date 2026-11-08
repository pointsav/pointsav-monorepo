// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Proposal/commit edit store. An edit is first *proposed* (validated against the file as it is
//! now and previewed) and only written when *committed*. Per SYS-ADR-07 (development) a model may
//! propose any edit, but a write to structured data needs a human approver: `commit` takes who is
//! approving and the gate refuses a non-human approver for structured files. A commit is refused if
//! the file changed since the proposal. Nothing touches disk on propose.

use crate::adr07::{self, Origin};
use crate::error::{CoreError, Result};
use crate::span::{apply_edit, Edit};
use crate::tree::{Root, Stamp};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub path: String,
    pub origin: Origin,
    pub edit: Edit,
    /// The file text when proposed, and the text that commit will write.
    pub before: String,
    pub after: String,
    stamp: Stamp,
}

#[derive(Debug)]
pub struct EditStore {
    root: Root,
    next_id: u64,
    pending: Vec<Proposal>,
}

impl EditStore {
    pub fn new(root: Root) -> Self {
        Self {
            root,
            next_id: 1,
            pending: Vec::new(),
        }
    }

    pub fn pending(&self) -> &[Proposal] {
        &self.pending
    }

    /// Validate and stage an edit; returns the proposal (including the preview). Writes nothing.
    pub fn propose(&mut self, path: &str, edit: Edit, origin: Origin) -> Result<&Proposal> {
        let (before, stamp) = self.root.read(path)?;
        let after = apply_edit(&before, &edit)?;
        let id = self.next_id;
        self.next_id += 1;
        self.pending.push(Proposal {
            id,
            path: path.to_string(),
            origin,
            edit,
            before,
            after,
            stamp,
        });
        Ok(self.pending.last().expect("just pushed"))
    }

    /// Write a proposal to disk on `approver`'s authority, whoever proposed it. Fails with
    /// `StructuredBlocked` if the target is structured data and `approver` is not human, and with
    /// `Stale` if the file changed since it was proposed. On failure the proposal stays pending.
    pub fn commit(&mut self, id: u64, approver: Origin) -> Result<Stamp> {
        let i = self
            .pending
            .iter()
            .position(|p| p.id == id)
            .ok_or(CoreError::NoProposal(id))?;
        let p = &self.pending[i];
        adr07::check(&p.path, approver)?;
        let stamp = self.root.write_atomic(&p.path, &p.after, Some(p.stamp))?;
        self.pending.remove(i);
        Ok(stamp)
    }

    /// Discard a proposal without writing.
    pub fn reject(&mut self, id: u64) -> Result<()> {
        let i = self
            .pending
            .iter()
            .position(|p| p.id == id)
            .ok_or(CoreError::NoProposal(id))?;
        self.pending.remove(i);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn store() -> (EditStore, std::path::PathBuf) {
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!(
            "tpc-store-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir_all(&d).unwrap();
        (EditStore::new(Root::new(&d).unwrap()), d)
    }

    #[test]
    fn propose_writes_nothing_commit_writes() {
        let (mut s, d) = store();
        fs::write(d.join("a.md"), "hello world").unwrap();
        let id = s
            .propose("a.md", Edit::new(Span::new(6, 11), "there"), Origin::Ai)
            .unwrap()
            .id;
        assert_eq!(s.pending()[0].after, "hello there");
        assert_eq!(fs::read_to_string(d.join("a.md")).unwrap(), "hello world");
        s.commit(id, Origin::Ai).unwrap();
        assert_eq!(fs::read_to_string(d.join("a.md")).unwrap(), "hello there");
        assert!(s.pending().is_empty());
        assert_eq!(s.commit(id, Origin::Human), Err(CoreError::NoProposal(id)));
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn stale_commit_refused_and_stays_pending() {
        let (mut s, d) = store();
        fs::write(d.join("a.md"), "hello world").unwrap();
        let id = s
            .propose("a.md", Edit::new(Span::new(0, 5), "HELLO"), Origin::Human)
            .unwrap()
            .id;
        fs::write(d.join("a.md"), "changed underneath").unwrap();
        assert_eq!(
            s.commit(id, Origin::Human),
            Err(CoreError::Stale("a.md".into()))
        );
        assert_eq!(
            fs::read_to_string(d.join("a.md")).unwrap(),
            "changed underneath"
        );
        assert_eq!(s.pending().len(), 1);
        s.reject(id).unwrap();
        assert!(s.pending().is_empty());
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn ai_may_propose_structured_but_only_a_human_commits() {
        let (mut s, d) = store();
        fs::write(d.join("a.json"), "{}").unwrap();
        let id = s
            .propose("a.json", Edit::new(Span::new(1, 1), " "), Origin::Ai)
            .unwrap()
            .id;
        assert_eq!(s.pending()[0].after, "{ }");
        assert_eq!(fs::read_to_string(d.join("a.json")).unwrap(), "{}");
        // the proposing model cannot approve its own structured write
        assert_eq!(
            s.commit(id, Origin::Ai),
            Err(CoreError::StructuredBlocked("a.json".into()))
        );
        assert_eq!(fs::read_to_string(d.join("a.json")).unwrap(), "{}");
        assert_eq!(s.pending().len(), 1);
        // a person approving the model's proposal writes it
        s.commit(id, Origin::Human).unwrap();
        assert_eq!(fs::read_to_string(d.join("a.json")).unwrap(), "{ }");
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn validation_applies_at_propose() {
        let (mut s, d) = store();
        fs::write(d.join("a.md"), "abc").unwrap();
        assert!(matches!(
            s.propose("a.md", Edit::new(Span::new(2, 9), "x"), Origin::Ai),
            Err(CoreError::OutOfRange { .. })
        ));
        assert!(matches!(
            s.propose("../a.md", Edit::new(Span::new(0, 0), "x"), Origin::Ai),
            Err(CoreError::OutsideRoot(_))
        ));
        assert!(s.pending().is_empty());
        let _ = fs::remove_dir_all(d);
    }
}
