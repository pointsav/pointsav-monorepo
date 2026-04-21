# Installation

How to install service-slm as an operator. For developers, see
[dev-guide/getting-started.md](../dev-guide/getting-started.md).

## Phase 2 status note

As of Phase 2 scaffolding, service-slm is not yet shipped as a signed
release. This document describes the intended process once releases
are cut; for now, install from source.

## From a signed release (once available)

1. Visit [Releases](https://github.com/woodfinegroup/pointsav-monorepo/releases)
   and download the artefact for your platform:
   - `slm-cli-x86_64-unknown-linux-gnu` (most Linux servers)
   - `slm-cli-aarch64-unknown-linux-gnu` (Totebox hosts, ARM servers)
   - `slm-cli-x86_64-apple-darwin` / `slm-cli-aarch64-apple-darwin` (macOS)
2. Download the accompanying `.sig` and `.pem` files.
3. Verify the signature:

   ```bash
   cosign verify-blob \
     --signature slm-cli-<target>.sig \
     --certificate slm-cli-<target>.pem \
     --certificate-identity-regexp 'https://github.com/woodfinegroup/.+' \
     --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
     slm-cli-<target>
   ```

   Or use the convenience wrapper:
   `./scripts/verify-sigstore.sh slm-cli-<target>`.

4. Move the verified binary into your `$PATH`:

   ```bash
   chmod +x slm-cli-<target>
   sudo mv slm-cli-<target> /usr/local/bin/slm-cli
   ```

5. Verify:

   ```bash
   slm-cli --version
   ```

## From source

```bash
git clone git@github.com:woodfinegroup/pointsav-monorepo.git
cd pointsav-monorepo/service-slm
./scripts/bootstrap.sh
cargo build --release --bin slm-cli
sudo cp target/release/slm-cli /usr/local/bin/slm-cli
slm-cli --version
```

Building from source requires the Rust toolchain (managed by
`rustup`) and ~6 GB of disk space. First build takes 5–10 minutes;
subsequent builds are incremental.

## As a systemd service (Phase 3)

The systemd unit file ships in `packaging/systemd/` (to be added in
Phase 3). Install with:

```bash
sudo cp packaging/systemd/slm-cli.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now slm-cli
sudo journalctl -u slm-cli -f
```

## As an os-totebox component (Phase 3+)

When service-slm ships as part of os-totebox, installation is handled
by the os-totebox installer. See the os-totebox project documentation
for details.

## Next steps

- Configure the service: see [configuration.md](./configuration.md).
- Operate the service: see [operating.md](./operating.md).
- Understand the yo-yo substrate: see [yoyo-substrate.md](./yoyo-substrate.md).
