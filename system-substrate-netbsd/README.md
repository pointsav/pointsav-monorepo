# system-substrate-netbsd

NetBSD 10.1 compat-bottom substrate shim for the `os-totebox` and `os-orchestration` image builders.

Exposes build-time constants (version strings, binary lists) and a `VeriexecEntry` type for generating `/etc/signatures` manifests.

## Compat-bottom role

NetBSD 10.1 is the canonical compat-bottom OS for all `os-*` guest VM images. It provides:

- `wg(4)` — WireGuard kernel driver (in-kernel since NetBSD 9.3).
- `Veriexec` — OS-level binary signature verification.
- FFS2 — canonical filesystem; built with `nbmakefs` from NetBSD cross tools.
- `build.sh tools` — reproducible cross-compilation on the GCP Ubuntu host.

## Build

```
cargo build -p system-substrate-netbsd
```

## License

AGPL-3.0-or-later. See [LICENSE](../LICENSE).
