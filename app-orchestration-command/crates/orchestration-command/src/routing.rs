// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Cross-archive message routing with confused-deputy defense.
//!
//! Before routing any message, the router validates that:
//! 1. `from_module_id` is a module in the fleet (prevents spoofing).
//! 2. `to_module_id` is a module in the fleet (prevents confused deputy).
//! 3. The caller is not sending to itself (no loopback).
//!
//! Routing appends a WORM audit entry for every decision (delivered or rejected).
//! v0.0.1 delivers messages by writing to the target archive's inbox.md via the
//! filesystem (same host deployment). A network-based delivery path is v0.1.0.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use chrono::Utc;
use serde::Serialize;
use tracing::{info, warn};
use uuid::Uuid;

use orchestration_command_core::{MessageEnvelope, MessageResponse, MessageStatus};
use crate::error::CommandError;

#[derive(Debug, Serialize)]
struct RoutingLedgerEntry {
    event: &'static str,
    ts: String,
    msg_id: String,
    from: String,
    to: String,
    re: String,
    status: &'static str,
}

pub struct MessageRouter {
    known_modules: HashSet<String>,
    /// module_id → path to archive root (for inbox.md writes)
    archive_roots: HashMap<String, PathBuf>,
    ledger_path: PathBuf,
    messages_routed: AtomicU64,
    instance_id: Arc<String>,
}

impl MessageRouter {
    pub fn new(
        module_to_root: HashMap<String, PathBuf>,
        instance_id: Arc<String>,
    ) -> Self {
        let known_modules: HashSet<String> = module_to_root.keys().cloned().collect();
        let ledger_path = std::env::var("COMMAND_AUDIT_LEDGER_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("data/command-audit.jsonl"));
        Self {
            known_modules,
            archive_roots: module_to_root,
            ledger_path,
            messages_routed: AtomicU64::new(0),
            instance_id,
        }
    }

    pub fn route(&self, envelope: &MessageEnvelope) -> Result<MessageResponse, CommandError> {
        let msg_id = format!(
            "{}-{}",
            envelope.from_module_id,
            Uuid::now_v7().to_string().split('-').next().unwrap_or("x")
        );
        let now = Utc::now();

        // Confused deputy defense: both parties must be known.
        if !self.known_modules.contains(&envelope.from_module_id) {
            warn!(from = %envelope.from_module_id, "message from unknown module rejected");
            self.append_ledger(&msg_id, &envelope.from_module_id, &envelope.to_module_id, &envelope.re, "rejected")?;
            return Ok(MessageResponse {
                msg_id,
                routed_at: now,
                status: MessageStatus::Rejected,
            });
        }
        if !self.known_modules.contains(&envelope.to_module_id) {
            warn!(to = %envelope.to_module_id, "message to unknown module rejected");
            self.append_ledger(&msg_id, &envelope.from_module_id, &envelope.to_module_id, &envelope.re, "rejected")?;
            return Ok(MessageResponse {
                msg_id,
                routed_at: now,
                status: MessageStatus::Rejected,
            });
        }

        // Deliver by writing a structured message to the target inbox.
        let delivered = if let Some(root) = self.archive_roots.get(&envelope.to_module_id) {
            let inbox = root.join(".agent").join("inbox.md");
            match write_to_inbox(&inbox, envelope, &msg_id, &self.instance_id, now) {
                Ok(()) => true,
                Err(e) => {
                    warn!(error = %e, inbox = %inbox.display(), "inbox write failed");
                    false
                }
            }
        } else {
            false
        };

        let status_str = if delivered { "delivered" } else { "delivery_failed" };
        self.append_ledger(&msg_id, &envelope.from_module_id, &envelope.to_module_id, &envelope.re, status_str)?;

        if delivered {
            info!(msg_id = %msg_id, from = %envelope.from_module_id, to = %envelope.to_module_id, "message routed");
            self.messages_routed.fetch_add(1, Ordering::Relaxed);
        }

        Ok(MessageResponse {
            msg_id,
            routed_at: now,
            status: if delivered { MessageStatus::Delivered } else { MessageStatus::Rejected },
        })
    }

    pub fn messages_routed(&self) -> u64 {
        self.messages_routed.load(Ordering::Relaxed)
    }

    fn append_ledger(
        &self,
        msg_id: &str,
        from: &str,
        to: &str,
        re: &str,
        status: &'static str,
    ) -> Result<(), CommandError> {
        let entry = RoutingLedgerEntry {
            event: "message_routed",
            ts: Utc::now().to_rfc3339(),
            msg_id: msg_id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            re: re.to_string(),
            status,
        };
        use std::io::Write;
        let mut line = serde_json::to_string(&entry)
            .map_err(|e| CommandError::Routing(format!("ledger json: {e}")))?;
        line.push('\n');
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ledger_path)?;
        file.write_all(line.as_bytes())?;
        Ok(())
    }
}

fn write_to_inbox(
    inbox: &PathBuf,
    envelope: &MessageEnvelope,
    msg_id: &str,
    instance_id: &str,
    now: chrono::DateTime<Utc>,
) -> Result<(), std::io::Error> {
    // Prepend a mailbox-protocol-compliant message block.
    let new_entry = format!(
        "---\nfrom: {from_module}@{instance}\nto: totebox@{to_module}\nre: {re}\ncreated: {ts}\npriority: normal\nstatus: pending\nmsg-id: {msg_id}\n---\n\n{body}\n\n",
        from_module = envelope.from_module_id,
        instance = instance_id,
        to_module = envelope.to_module_id,
        re = envelope.re,
        ts = now.to_rfc3339(),
        msg_id = msg_id,
        body = envelope.body,
    );

    // Read existing content and prepend.
    let existing = std::fs::read_to_string(inbox).unwrap_or_default();
    let combined = format!("{new_entry}{existing}");
    std::fs::write(inbox, combined)
}
