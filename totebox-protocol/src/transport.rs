// SPDX-License-Identifier: Apache-2.0

//! QUIC transport via `quinn`+`rustls`.
//!
//! Decision 9: stream multiplexing without head-of-line blocking is
//! the decisive property (one connection carries a live subscription,
//! a one-shot query, and a large attachment transfer concurrently
//! without the attachment stalling the feed). mTLS with device
//! Ed25519 identity: the client certificate **is** the capability
//! holder (see `capability.rs`) -- a connection's [`capability::Capability`]
//! is derived from which CA signed its peer certificate, not from any
//! in-band request field a compromised client could spoof.
//!
//! **Phase 0 scope, stated plainly:** real QUIC connection setup, real
//! bidi-stream request/response (`call`), real server-initiated
//! uni-stream push (`push`). Self-signed certs (via `rcgen` in tests)
//! are used for testing correctness; a real fleet PKI (CA issuance,
//! rotation, revocation) is deployment-time infrastructure, not a
//! protocol-crate concern, and isn't built here. The TCP+TLS fallback
//! for hostile-UDP networks (decision 9: "a carrier, not a second
//! protocol") is NOT implemented in this pass -- same frames, a
//! different transport, tracked as a real follow-up, not silently
//! dropped.

use std::net::SocketAddr;
use std::sync::Arc;

use quinn::{ClientConfig, Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::RootCertStore;

use crate::frames::{decode, encode, EnvelopeAppended, Request, Response};

/// ALPN protocol identifier both sides must agree on during the TLS
/// handshake -- required by `rustls`/QUIC (unlike plain HTTPS, QUIC
/// has no implicit default), and easy to silently omit on one side,
/// which fails the handshake immediately with an unhelpful "closed by
/// peer: 0" rather than a clear ALPN-mismatch error.
const ALPN_PROTOCOL: &[u8] = b"totebox-protocol/1";

#[derive(Debug)]
pub enum TransportError {
    Tls(String),
    Io(String),
    Connect(String),
    Connection(String),
    Stream(String),
    Codec(String),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::Tls(e) => write!(f, "TLS error: {e}"),
            TransportError::Io(e) => write!(f, "I/O error: {e}"),
            TransportError::Connect(e) => write!(f, "connect error: {e}"),
            TransportError::Connection(e) => write!(f, "connection error: {e}"),
            TransportError::Stream(e) => write!(f, "stream error: {e}"),
            TransportError::Codec(e) => write!(f, "codec error: {e}"),
        }
    }
}

impl std::error::Error for TransportError {}

/// Build a server-side QUIC endpoint bound to `addr`. `cert_chain`/
/// `key` are this endpoint's own TLS identity; `client_ca` is the
/// root store used to verify connecting clients' certificates --
/// mTLS, so a client without a certificate signed by this CA cannot
/// complete a handshake at all, let alone send a `Request`.
pub fn server_endpoint(
    addr: SocketAddr,
    cert_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    client_ca: RootCertStore,
) -> Result<Endpoint, TransportError> {
    let client_verifier = rustls::server::WebPkiClientVerifier::builder_with_provider(
        Arc::new(client_ca),
        ring_provider(),
    )
    .build()
    .map_err(|e| TransportError::Tls(format!("client verifier: {e}")))?;

    let mut tls_config = rustls::ServerConfig::builder_with_provider(ring_provider())
        .with_safe_default_protocol_versions()
        .map_err(|e| TransportError::Tls(format!("server tls versions: {e}")))?
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(cert_chain, key)
        .map_err(|e| TransportError::Tls(format!("server tls config: {e}")))?;
    tls_config.alpn_protocols = vec![ALPN_PROTOCOL.to_vec()];

    let quic_tls = quinn::crypto::rustls::QuicServerConfig::try_from(tls_config)
        .map_err(|e| TransportError::Tls(format!("quic tls config: {e}")))?;
    let server_config = ServerConfig::with_crypto(Arc::new(quic_tls));

    Endpoint::server(server_config, addr).map_err(|e| TransportError::Io(e.to_string()))
}

/// The crypto provider is named explicitly: when another crate in the same
/// build also enables `aws-lc-rs`, rustls cannot pick a process default and panics.
fn ring_provider() -> Arc<rustls::crypto::CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

/// Build a client-side QUIC endpoint, presenting `cert_chain`/`key`
/// as its own client certificate (the capability the server will
/// derive `Capability` from), trusting `server_ca` to verify the
/// server it connects to.
pub fn client_endpoint(
    bind_addr: SocketAddr,
    cert_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    server_ca: RootCertStore,
) -> Result<Endpoint, TransportError> {
    let mut tls_config = rustls::ClientConfig::builder_with_provider(ring_provider())
        .with_safe_default_protocol_versions()
        .map_err(|e| TransportError::Tls(format!("client tls versions: {e}")))?
        .with_root_certificates(server_ca)
        .with_client_auth_cert(cert_chain, key)
        .map_err(|e| TransportError::Tls(format!("client tls config: {e}")))?;
    tls_config.alpn_protocols = vec![ALPN_PROTOCOL.to_vec()];

    let quic_tls = quinn::crypto::rustls::QuicClientConfig::try_from(tls_config)
        .map_err(|e| TransportError::Tls(format!("quic tls config: {e}")))?;
    let client_config = ClientConfig::new(Arc::new(quic_tls));

    let mut endpoint =
        Endpoint::client(bind_addr).map_err(|e| TransportError::Io(e.to_string()))?;
    endpoint.set_default_client_config(client_config);
    Ok(endpoint)
}

/// Send one `Request` over a fresh bidi stream on `connection`, wait
/// for and decode the `Response`. Each RPC call is its own bidi
/// stream (decision 9: "a QUIC bidi stream = one RPC") -- callers
/// making many calls concurrently on the same `connection` get real
/// multiplexing without head-of-line blocking, for free from QUIC
/// itself.
pub async fn call(
    connection: &quinn::Connection,
    request: &Request,
) -> Result<Response, TransportError> {
    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;

    let bytes = encode(request).map_err(|e| TransportError::Codec(e.to_string()))?;
    send.write_all(&bytes)
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    send.finish()
        .map_err(|e| TransportError::Stream(e.to_string()))?;

    let response_bytes = recv
        .read_to_end(16 * 1024 * 1024) // 16 MiB cap, matches the artifact-size class of ceiling used elsewhere in this workspace
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;

    decode(&response_bytes).map_err(|e| TransportError::Codec(e.to_string()))
}

/// Server-side counterpart to `call`: accept one bidi stream, decode
/// the `Request`, hand it to `handler`, encode and send back the
/// `Response` it returns.
///
/// **Caller discipline, verified the hard way (real test failure, not
/// theoretical):** `finish()` queues the response stream's FIN frame;
/// it does not wait for delivery or acknowledgment. If the caller
/// drops its last `Connection` handle immediately after this function
/// returns, that drop sends an immediate QUIC close that can race
/// ahead of the queued response actually reaching the peer, and the
/// peer's read fails with a connection-lost error despite the write
/// having "succeeded" on this side. Real long-lived usage (a server
/// accept-loop serving many requests over one connection, or a
/// connection also carrying `Subscribe` pushes) doesn't hit this --
/// the connection naturally stays alive across many calls. A caller
/// whose very last action on a connection is exactly one
/// `serve_one_request` must wait for the peer to close first (e.g.
/// `connection.closed().await`) before letting the `Connection` drop.
pub async fn serve_one_request<F>(
    connection: &quinn::Connection,
    handler: F,
) -> Result<(), TransportError>
where
    F: FnOnce(Request) -> Response,
{
    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;

    let request_bytes = recv
        .read_to_end(16 * 1024 * 1024)
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    let request: Request =
        decode(&request_bytes).map_err(|e| TransportError::Codec(e.to_string()))?;

    let response = handler(request);
    let response_bytes = encode(&response).map_err(|e| TransportError::Codec(e.to_string()))?;
    send.write_all(&response_bytes)
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    send.finish()
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    Ok(())
}

/// Open a uni-stream on `connection` and push one `EnvelopeAppended`
/// frame -- the server side of a `Subscribe` (decision 9: "a long-
/// lived server-initiated uni stream = one subscription"). Called
/// once per new ledger entry matching an active subscription; the
/// stream itself is expected to stay open across many pushes in real
/// use (a caller doing so opens the uni-stream once and calls this
/// function's write logic repeatedly -- this function opens-and-sends
/// in one call for the simple/test case).
pub async fn push(
    connection: &quinn::Connection,
    frame: &EnvelopeAppended,
) -> Result<(), TransportError> {
    let mut send = connection
        .open_uni()
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;
    let bytes = encode(frame).map_err(|e| TransportError::Codec(e.to_string()))?;
    send.write_all(&bytes)
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    send.finish()
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    Ok(())
}

/// Accept one uni-stream and decode it as an `EnvelopeAppended` push
/// -- the client side of a subscription.
pub async fn accept_push(
    connection: &quinn::Connection,
) -> Result<EnvelopeAppended, TransportError> {
    let mut recv = connection
        .accept_uni()
        .await
        .map_err(|e| TransportError::Connection(e.to_string()))?;
    let bytes = recv
        .read_to_end(16 * 1024 * 1024)
        .await
        .map_err(|e| TransportError::Stream(e.to_string()))?;
    decode(&bytes).map_err(|e| TransportError::Codec(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frames::EnvelopeBytes;
    use rcgen::{CertificateParams, KeyPair};

    /// Generate a minimal self-signed CA + one leaf certificate signed
    /// by it. Real fleet PKI is out of scope for this crate (see
    /// module doc); this is exactly enough for a real mTLS handshake
    /// in tests.
    fn make_ca_and_leaf(
        name: &str,
    ) -> (
        RootCertStore,
        Vec<CertificateDer<'static>>,
        PrivateKeyDer<'static>,
    ) {
        let ca_key = KeyPair::generate().unwrap();
        let mut ca_params = CertificateParams::new(vec![]).unwrap();
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let ca_cert = ca_params.self_signed(&ca_key).unwrap();

        let leaf_key = KeyPair::generate().unwrap();
        let leaf_params = CertificateParams::new(vec![name.to_string()]).unwrap();
        let leaf_cert = leaf_params.signed_by(&leaf_key, &ca_cert, &ca_key).unwrap();

        let mut roots = RootCertStore::empty();
        roots.add(ca_cert.der().clone()).unwrap();

        let chain = vec![leaf_cert.der().clone()];
        let key = PrivateKeyDer::Pkcs8(leaf_key.serialize_der().into());
        (roots, chain, key)
    }

    #[tokio::test]
    async fn client_submits_and_server_responds() {
        let (server_ca, server_chain, server_key) = make_ca_and_leaf("localhost");
        let (client_ca, client_chain, client_key) = make_ca_and_leaf("test-client");

        // Server trusts the client's CA; client trusts the server's CA.
        // (Two independent single-cert CAs, cross-trusted -- simplest
        // real mTLS shape for a test.)
        let server_ep = server_endpoint(
            "127.0.0.1:0".parse().unwrap(),
            server_chain,
            server_key,
            client_ca,
        )
        .expect("server endpoint");
        let server_addr = server_ep.local_addr().unwrap();

        let client_ep = client_endpoint(
            "127.0.0.1:0".parse().unwrap(),
            client_chain,
            client_key,
            server_ca,
        )
        .expect("client endpoint");

        let server_task = tokio::spawn(async move {
            let incoming = server_ep.accept().await.expect("incoming connection");
            let connection = incoming.await.expect("handshake completes");
            serve_one_request(&connection, |req| match req {
                Request::Submit { ledger, .. } => {
                    assert_eq!(ledger, "service-minutebook");
                    Response::Submitted { cursor: 1 }
                }
                other => panic!("unexpected request: {other:?}"),
            })
            .await
            .expect("serve one request");
            // Do NOT drop `connection` immediately after finish()ing the
            // response stream -- finish() queues the FIN, it does not
            // wait for delivery/ack, and dropping the last Connection
            // handle sends an immediate QUIC close that can race ahead
            // of that queued data actually reaching the peer (verified
            // real failure mode, not theoretical -- this exact race
            // produced "connection lost" on the client's read_to_end
            // before this fix). Wait for the peer to close first.
            let _ = connection.closed().await;
        });

        let connection = client_ep
            .connect(server_addr, "localhost")
            .expect("connect starts")
            .await
            .expect("handshake completes");

        let request = Request::Submit {
            ledger: "service-minutebook".to_string(),
            envelope: EnvelopeBytes::new(vec![1, 2, 3]),
        };
        let response = call(&connection, &request).await.expect("call succeeds");
        assert!(matches!(response, Response::Submitted { cursor: 1 }));
        connection.close(0u32.into(), b"done");

        server_task.await.expect("server task completes");
    }

    #[tokio::test]
    async fn push_and_accept_push_round_trip() {
        let (server_ca, server_chain, server_key) = make_ca_and_leaf("localhost");
        let (client_ca, client_chain, client_key) = make_ca_and_leaf("test-client");

        let server_ep = server_endpoint(
            "127.0.0.1:0".parse().unwrap(),
            server_chain,
            server_key,
            client_ca,
        )
        .expect("server endpoint");
        let server_addr = server_ep.local_addr().unwrap();

        let client_ep = client_endpoint(
            "127.0.0.1:0".parse().unwrap(),
            client_chain,
            client_key,
            server_ca,
        )
        .expect("client endpoint");

        let server_task = tokio::spawn(async move {
            let incoming = server_ep.accept().await.unwrap();
            let connection = incoming.await.unwrap();
            let frame = EnvelopeAppended {
                ledger: "service-chat".to_string(),
                seq: 5,
                body: EnvelopeBytes::new(vec![9, 9, 9]),
            };
            push(&connection, &frame).await.expect("push succeeds");
            // See the matching note in client_submits_and_server_responds --
            // wait for the peer's close rather than dropping `connection`
            // right after finish()ing the push stream.
            let _ = connection.closed().await;
        });

        let connection = client_ep
            .connect(server_addr, "localhost")
            .unwrap()
            .await
            .unwrap();
        let received = accept_push(&connection)
            .await
            .expect("accept_push succeeds");
        assert_eq!(received.ledger, "service-chat");
        assert_eq!(received.seq, 5);
        connection.close(0u32.into(), b"done");

        server_task.await.unwrap();
    }
}
