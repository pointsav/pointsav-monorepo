// SPDX-License-Identifier: Apache-2.0

//! Wire frames — decision 9's postcard-framed RPC/subscription layer.
//!
//! Frame *envelopes* (this file's types) are postcard: compact,
//! fixed-schema, versioned at connection setup. A signed `Envelope`
//! *body* carried inside a frame is embedded as raw canonical-CBOR
//! bytes via [`EnvelopeBytes`] — never re-serialized. This is the
//! "never re-serialize a signed Envelope" rule enforced at the type
//! level: `EnvelopeBytes` exposes only a parsed *view*
//! (`EnvelopeBytes::parse`), not a way to get a re-encodable
//! `envelope::Envelope` back out and reinsert it — round-tripping
//! through `envelope::Envelope::canonical_bytes()` again could change
//! the bytes on a future schema version and silently break the
//! signature it's carrying.

use serde::{Deserialize, Serialize};

/// Raw canonical-CBOR bytes of a signed `envelope::Envelope`, carried
/// verbatim. Constructed only from bytes already produced by
/// `envelope::Envelope::canonical_bytes()` plus its signature having
/// been appended by the caller (see `into_signed_bytes` on the
/// `envelope` crate side) — this type does not itself know how to
/// build one, only carry and parse one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeBytes(pub Vec<u8>);

impl EnvelopeBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Parse into a read-only view. Deliberately does not return
    /// anything that can be fed back into
    /// `envelope::Envelope::canonical_bytes()` and re-signed as if it
    /// were the original — callers that need to re-emit these bytes
    /// (e.g. forwarding to a subscriber) must forward `self.0`
    /// verbatim, not reconstruct from the parsed view.
    pub fn parse(&self) -> Result<envelope::Envelope, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(self.0.as_slice())
    }
}

/// One ledger's identifier on the wire — matches the `module_id`/
/// tenant + service-name shape every `worm-ledger` instance already
/// uses for its own on-disk root.
pub type LedgerId = String;

/// A QUIC bidi-stream request = one RPC call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Append a signed Envelope to `ledger`. The one genuinely shared
    /// ingestion surface per decision 15 — every "system of record"
    /// service's write path goes through this, not a bespoke
    /// per-service endpoint.
    Submit { ledger: LedgerId, envelope: EnvelopeBytes },
    /// Read entries with cursor > `since`. Also the offline-catch-up
    /// path — same code as live delivery, per decision 9's "biggest
    /// simplification" note.
    ReadSince { ledger: LedgerId, since: u64 },
    /// Capability-scoped query — the ONLY method fleet peers
    /// (through `os-orchestration`) are ever granted. Returns a
    /// computed result, never raw ledger entries.
    Query { ledger: LedgerId, query: String },
    /// Fetch one blob by content hash from `worm_ledger::blob`'s
    /// store.
    GetBlob { hash: String },
    /// Open a live subscription on `ledger` starting after `since`.
    /// The server responds by opening a uni-stream pushing
    /// `Push::EnvelopeAppended` frames — see `Response::Subscribed`.
    Subscribe { ledger: LedgerId, since: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Submitted { cursor: u64 },
    Entries { entries: Vec<EnvelopeBytes>, next_cursor: u64 },
    QueryResult { result: serde_json::Value },
    Blob { bytes: Vec<u8> },
    /// Acknowledges a `Subscribe` request; the actual pushed entries
    /// arrive as `Push::EnvelopeAppended` frames on a separate
    /// server-initiated uni-stream, not as further `Response`s on
    /// this bidi stream.
    Subscribed,
    /// A request was structurally valid but refused — see
    /// `RefusalReason`. Distinct from an infrastructure error
    /// (unknown ledger, I/O failure), which is `Error`.
    Refused { reason: RefusalReason },
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefusalReason {
    /// This connection's capability token doesn't permit this method
    /// at all (see `capability::Capability`).
    MethodNotPermitted,
    /// This connection's capability token permits the method in
    /// general, but not on this specific `ledger`.
    LedgerNotPermitted,
}

/// A server-initiated uni-stream push, following a `Subscribe`
/// request. One uni-stream per subscription (decision 9: "a long-
/// lived server-initiated uni stream = one subscription").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeAppended {
    pub ledger: LedgerId,
    pub seq: u64,
    pub body: EnvelopeBytes,
}

/// Postcard-encode a frame (either a `Request`, `Response`, or
/// `EnvelopeAppended`) into its wire bytes.
pub fn encode<T: Serialize>(frame: &T) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_stdvec(frame)
}

/// Decode wire bytes back into a frame type.
pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, postcard::Error> {
    postcard::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_signed_envelope_bytes() -> Vec<u8> {
        // A minimal, unsigned-for-test Envelope, canonical-CBOR
        // encoded -- frame-layer tests only need *some* CBOR bytes to
        // round-trip through EnvelopeBytes, not a real signature.
        let env = envelope::Envelope {
            envelope_id: "01JTEST0000000000000000000".to_string(),
            envelope_version: "0.1.0".to_string(),
            tenant: "woodfine".to_string(),
            created: chrono::DateTime::parse_from_rfc3339("2026-09-23T00:00:00Z")
                .unwrap()
                .with_timezone(&chrono::Utc),
            source: envelope::Source { adapter: "test".to_string(), origin_ref: "x".to_string() },
            entities: vec![],
            state: envelope::EnvelopeState::Observed,
            attested_by: None,
            confidence: None,
            payload: envelope::Payload {
                payload_type: "test.record".to_string(),
                body: serde_json::json!({"k": "v"}),
            },
            attachments: vec![],
            lineage: envelope::Lineage::default(),
            signature: String::new(),
        };
        env.canonical_bytes().unwrap()
    }

    #[test]
    fn envelope_bytes_round_trips_through_parse() {
        let raw = sample_signed_envelope_bytes();
        let eb = EnvelopeBytes::new(raw.clone());
        assert_eq!(eb.as_bytes(), raw.as_slice());
        let parsed = eb.parse().expect("must parse back");
        assert_eq!(parsed.envelope_id, "01JTEST0000000000000000000");
        assert_eq!(parsed.payload.payload_type, "test.record");
    }

    #[test]
    fn request_submit_encodes_and_decodes() {
        let req = Request::Submit {
            ledger: "service-minutebook".to_string(),
            envelope: EnvelopeBytes::new(sample_signed_envelope_bytes()),
        };
        let bytes = encode(&req).unwrap();
        let decoded: Request = decode(&bytes).unwrap();
        match decoded {
            Request::Submit { ledger, envelope } => {
                assert_eq!(ledger, "service-minutebook");
                assert_eq!(envelope.parse().unwrap().payload.payload_type, "test.record");
            }
            other => panic!("expected Submit, got {other:?}"),
        }
    }

    #[test]
    fn request_read_since_round_trips() {
        let req = Request::ReadSince { ledger: "service-email".to_string(), since: 42 };
        let bytes = encode(&req).unwrap();
        let decoded: Request = decode(&bytes).unwrap();
        assert!(matches!(decoded, Request::ReadSince { ledger, since } if ledger == "service-email" && since == 42));
    }

    #[test]
    fn response_refused_carries_reason() {
        let resp = Response::Refused { reason: RefusalReason::LedgerNotPermitted };
        let bytes = encode(&resp).unwrap();
        let decoded: Response = decode(&bytes).unwrap();
        assert!(matches!(decoded, Response::Refused { reason: RefusalReason::LedgerNotPermitted }));
    }

    #[test]
    fn envelope_appended_push_round_trips() {
        let push = EnvelopeAppended {
            ledger: "service-chat".to_string(),
            seq: 7,
            body: EnvelopeBytes::new(sample_signed_envelope_bytes()),
        };
        let bytes = encode(&push).unwrap();
        let decoded: EnvelopeAppended = decode(&bytes).unwrap();
        assert_eq!(decoded.ledger, "service-chat");
        assert_eq!(decoded.seq, 7);
    }

    #[test]
    fn malformed_envelope_bytes_fail_to_parse_not_panic() {
        let eb = EnvelopeBytes::new(vec![0xff, 0x00, 0x01]);
        assert!(eb.parse().is_err());
    }
}
