---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Private Binary Download Endpoint for Paying Customers"
slug: topic-private-git-paid-customer-endpoint
language: en
status: draft
paired_with: TOPIC-private-git-paid-customer-endpoint.es.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-private-git-paid-customer-endpoint.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-source/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# Private Binary Download Endpoint for Paying Customers

The binary release server is the component of `software.pointsav.com` that delivers
compiled binaries to paying customers. It is a thin, stateless gate: it holds no payment
records, no customer data, and no signing keys. Its sole responsibility is to verify that
a presented license token is genuine and authorises the requested product, then stream
the binary file. A customer who has completed a purchase and holds a valid token can
download the corresponding binary without any further interaction with the payment
infrastructure.

## Route structure

The release server organises its endpoints into four categories:

**Product and version discovery.** An unauthenticated product index lists all products
whose releases are available on the server, and a version index lists all available
versions for a given product. These endpoints require no license token and are designed
for consumption by tooling such as package managers, installer scripts, and CI pipelines.

**Versioned binary download.** The primary gated endpoint serves a binary for a specific
product, version, and platform. Requests to this endpoint require a valid license token.
A detached Ed25519 signature file for each binary is available at a corresponding path
and is always unauthenticated — detached signatures are public by design, allowing any
party to verify the authenticity of a binary without holding a license.

**Latest-version redirect.** A convenience endpoint resolves the highest available version
for a given product and platform and issues a redirect to the versioned download path. The
license token is forwarded through the redirect. The redirect only targets platform and
version combinations for which a release actually exists; it will not redirect to a version
that lacks a binary for the requested platform.

**Release manifest.** A per-version metadata endpoint serves a structured manifest
describing the contents of a release. No authentication is required. This endpoint is
useful for tooling that needs to inspect what a release contains before initiating a
download.

## Authentication

The release server accepts a license token in two forms:

**HTTP Authorization header.** The token is passed as a Bearer credential in the
`Authorization` header. This is the standard form for programmatic clients, automated
installers, and command-line tools that can set arbitrary request headers.

**Query parameter.** The token is passed as a `token` parameter appended to the URL.
This form exists specifically to enable browser-initiated one-click download links: a
storefront can generate a URL that includes the token, allowing a customer to download
a binary directly from their browser without configuring any HTTP headers. Both forms are
equally secure — neither the header nor the query parameter form exposes the token to
additional parties beyond the client and the server.

## Verification logic

The server decodes the base64url token string, splits off the first 64 bytes as an Ed25519
signature, and verifies the signature over the remaining bytes using the server's stored
public verification key. It then parses the payload and checks two things: that the
product field in the payload matches the product being requested, and that the expiry date
has not passed. A token for a different product returns 403. A token whose signature does
not verify returns 401. An expired token returns 403 with a reason indicating the channel
has expired. A detailed description of the token format is in the companion TOPIC on
crypto payment and license issuance architecture.

## Platform strings

Platform strings follow the Rust target triple convention. Examples include
`x86_64-unknown-linux-gnu` for 64-bit Linux on x86, `aarch64-unknown-linux-gnu` for
64-bit ARM Linux, and `x86_64-apple-darwin` for macOS on Intel. The server maps the
product name, version string, and platform triple directly to a file path in the releases
directory. If no binary has been built for the requested combination of product, version,
and platform, the server returns 404 with a note that the build pipeline has not yet
produced that release. The latest-version redirect endpoint only redirects to platform
strings for which a release file actually exists.

## Key management and fail-safe behaviour

The server loads the public Ed25519 verification key at startup from a configuration
source. If no key is configured, the server does not silently accept all tokens: instead,
the download and verification endpoints return a service-unavailable response. This
fail-safe behaviour means a misconfigured or freshly deployed instance that has not yet
been given a verification key will reject all requests rather than accidentally grant
access. A correctly configured instance with a valid key will accept tokens signed by the
corresponding private key and reject all others.

## What the server does not do

The release server does not track individual downloads or maintain any download history.
It does not implement token revocation: once a token is issued, it remains valid until its
expiry date, and there is currently no revocation list. Customers who need to prevent a
compromised token from being used must wait for the token to expire; key rotation by the
storefront invalidates all previously issued tokens at the cost of requiring existing
customers to re-issue their tokens.

The server does not serve source code. It does not act as a live Git server: a stub
endpoint exists at the Git protocol path and returns a redirect to the public GitHub
repository rather than attempting to proxy Git operations. This route is reserved for a
future version that may offer authenticated Git access to private repositories.
