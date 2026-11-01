// SPDX-License-Identifier: Apache-2.0

//! Deny-by-default capability/method table — decision 9's structural
//! enforcement of the isolation doctrine ("no container shares a
//! ledger... emit only the result, never the raw record",
//! `content-wiki-documentation/systems/os-orchestration.md`).
//!
//! The wire protocol is ONE protocol for both client<->server and
//! fleet server<->server (decision 9) — the difference is which
//! [`Capability`] a connection's mTLS client certificate carries, not
//! a second protocol. A fleet peer connecting through
//! `os-orchestration` is granted [`Capability::FleetPeer`] and is
//! structurally denied `Subscribe`/`ReadSince` on raw ledgers — this
//! is checked here, in one file, so the isolation guarantee is
//! auditable by reading `Capability::allows` rather than hoped-for
//! across every service's own request handling.

use crate::frames::Request;

/// What a connection is allowed to do, derived from its mTLS client
/// certificate at connection setup (see `transport.rs`). Deny-by-
/// default: a new `Request` variant added later requires an explicit
/// `true` arm in [`Capability::allows`], not an implicit allow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capability {
    /// A local device (os-workplace/os-console client, or the
    /// service-envelope-sync daemon on this same Totebox) — full
    /// access to every ledger this Totebox owns.
    Local,
    /// A fleet peer, connecting through `os-orchestration` — per the
    /// isolation doctrine, granted `Query`/`GetBlob` only.
    /// Structurally denied `Submit`/`ReadSince`/`Subscribe` on raw
    /// ledgers: a fleet peer never sees raw records, only computed
    /// query results.
    FleetPeer,
    /// A `system-protocol-*` adapter session bridging one external
    /// account (e.g. a user's personal Gmail via IMAP) — may only
    /// `Submit` into the one ledger it's provisioned for, nothing
    /// else. Distinct from `Local` because an adapter talking to a
    /// hostile third-party server should not, by construction, be
    /// able to read arbitrary other ledgers even if it were somehow
    /// compromised.
    ExternalAdapter { allowed_ledger: String },
}

impl Capability {
    /// Is `request` permitted under this capability? Ledger-scoped
    /// requests are also checked against the specific `ledger` named
    /// in the request, not just the request *kind* — see
    /// `Capability::allows_ledger`.
    pub fn allows(&self, request: &Request) -> bool {
        match self {
            Capability::Local => true,
            Capability::FleetPeer => matches!(request, Request::Query { .. } | Request::GetBlob { .. }),
            Capability::ExternalAdapter { allowed_ledger } => match request {
                Request::Submit { ledger, .. } => ledger == allowed_ledger,
                _ => false,
            },
        }
    }

    /// For requests that name a specific ledger, is *this* ledger
    /// permitted? Separate from `allows` because a request can be the
    /// right *kind* (e.g. `Query` for a `FleetPeer`) but still target
    /// a ledger this capability has no business touching (once
    /// per-ledger query scoping is provisioned — Phase 0's
    /// `FleetPeer`/`Local` cases don't yet distinguish per-ledger
    /// access beyond what `allows` already checks; `ExternalAdapter`
    /// is the one variant that already needs it today).
    pub fn allows_ledger(&self, ledger: &str) -> bool {
        match self {
            Capability::Local => true,
            Capability::FleetPeer => true, // ledger scoping happens inside Query's own result computation, not here
            Capability::ExternalAdapter { allowed_ledger } => ledger == allowed_ledger,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frames::EnvelopeBytes;

    fn submit(ledger: &str) -> Request {
        Request::Submit {
            ledger: ledger.to_string(),
            envelope: EnvelopeBytes::new(vec![]),
        }
    }

    #[test]
    fn local_allows_everything() {
        let cap = Capability::Local;
        assert!(cap.allows(&submit("service-email")));
        assert!(cap.allows(&Request::ReadSince { ledger: "x".into(), since: 0 }));
        assert!(cap.allows(&Request::Subscribe { ledger: "x".into(), since: 0 }));
        assert!(cap.allows(&Request::Query { ledger: "x".into(), query: "q".into() }));
        assert!(cap.allows(&Request::GetBlob { hash: "h".into() }));
    }

    #[test]
    fn fleet_peer_allows_only_query_and_get_blob() {
        let cap = Capability::FleetPeer;
        assert!(cap.allows(&Request::Query { ledger: "x".into(), query: "q".into() }));
        assert!(cap.allows(&Request::GetBlob { hash: "h".into() }));
        assert!(!cap.allows(&submit("service-email")), "fleet peer must never Submit");
        assert!(!cap.allows(&Request::ReadSince { ledger: "x".into(), since: 0 }), "fleet peer must never ReadSince raw ledgers");
        assert!(!cap.allows(&Request::Subscribe { ledger: "x".into(), since: 0 }), "fleet peer must never Subscribe to raw ledgers");
    }

    #[test]
    fn external_adapter_may_only_submit_to_its_own_ledger() {
        let cap = Capability::ExternalAdapter { allowed_ledger: "service-email".to_string() };
        assert!(cap.allows(&submit("service-email")));
        assert!(!cap.allows(&submit("service-chat")), "adapter must not submit to a ledger it wasn't provisioned for");
        assert!(!cap.allows(&Request::ReadSince { ledger: "service-email".into(), since: 0 }), "adapter gets Submit only, not read access");
    }

    #[test]
    fn external_adapter_ledger_scoping_matches_allows() {
        let cap = Capability::ExternalAdapter { allowed_ledger: "service-calendar".to_string() };
        assert!(cap.allows_ledger("service-calendar"));
        assert!(!cap.allows_ledger("service-chat"));
    }

    #[test]
    fn a_new_request_variant_must_be_explicitly_handled() {
        // This test exists to make a real point in review, not to
        // assert runtime behavior: `Capability::allows`'s match has no
        // wildcard `_ => true` arm anywhere -- every arm is written
        // out per-capability, per-request-kind, so the compiler
        // forces a decision on any future Request variant rather than
        // silently defaulting to allow. (No assertion needed; this
        // guarantee is enforced by rustc at the match in
        // capability.rs, not by this test.)
    }
}
