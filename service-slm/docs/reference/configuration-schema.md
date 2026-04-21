# Configuration schema reference

Authoritative reference for the TOML configuration file. The runtime
schema is generated from `schemars`-derived types in `slm-core` and
validated with the `validator` crate.

## Top-level

```toml
# Server — the HTTP listener
[server]
listen = "127.0.0.1:8080"              # required
shutdown_grace_seconds = 30            # default 30

# Doorman — the protocol
[doorman]
sanitisation_policy = "default"        # default "default"
timeout_seconds = 60                   # default 60
retry_max_attempts = 5                 # default 5

# Ledger — the audit trail
[ledger]
csv_path = "/var/lib/slm-cli/ledger/events.csv"         # required
sqlite_path = "/var/lib/slm-cli/ledger/events.sqlite"   # required
fsync_on_commit = true                 # default true; never disable in prod

# Compute — the GCP yo-yo node
[compute]
gcp_project = "pointsav-dev"           # required
gcp_region = "us-central1"             # required
manifest_path = "/etc/slm-cli/compute/manifest.yaml"   # required
warm_pool_enabled = false              # default false
cold_start_timeout_seconds = 60        # default 60
api_key_ref = "secret-manager://..."   # required; see Secrets below

# Memory: KV cache
[memory.kv]
mooncake_master_url = "tcp://mooncake.internal:50051"   # required
local_cpu_cache_mib = 4096             # default 4096
module_namespace = true                # default true; set false only for single-tenant test

# Memory: LoRA adapters
[memory.adapters]
registry_path = "/etc/slm-cli/memory/adapters/registry.yaml"   # required
verify_signatures = true               # default true; never disable in prod

# Observability
[observability]
metrics_path = "/metrics"              # default "/metrics"
json_logs = false                      # default false
```

## Types and validation

| Key | Type | Validation |
|---|---|---|
| `server.listen` | `SocketAddr` | parseable |
| `server.shutdown_grace_seconds` | `u32` | 0 ≤ n ≤ 3600 |
| `doorman.timeout_seconds` | `u32` | 1 ≤ n ≤ 600 |
| `doorman.retry_max_attempts` | `u8` | 0 ≤ n ≤ 20 |
| `ledger.csv_path` | `PathBuf` | parent directory must exist and be writable |
| `ledger.sqlite_path` | `PathBuf` | parent directory must exist and be writable |
| `compute.gcp_project` | `String` | `^[a-z][a-z0-9-]{4,28}[a-z0-9]$` |
| `compute.gcp_region` | `String` | non-empty |
| `compute.api_key_ref` | `String` | must parse as a valid secret reference (see Secrets) |
| `memory.kv.local_cpu_cache_mib` | `u32` | 64 ≤ n ≤ 65536 |

## Environment overrides

Every key above can be overridden by an environment variable of the
form `SLM_<SECTION>_<KEY>`, with dots replaced by underscores and all
upper-case. Nested sections use double underscores.

Examples:

- `SLM_SERVER_LISTEN=0.0.0.0:9090`
- `SLM_LEDGER_FSYNC_ON_COMMIT=false` (not recommended)
- `SLM_MEMORY__KV__MOONCAKE_MASTER_URL=tcp://10.0.0.5:50051`

## Secrets

Secret references use the URL scheme. Supported schemes:

- `env://VAR_NAME` — read from an environment variable.
- `secret-manager://projects/PROJ/secrets/NAME/versions/latest` —
  Google Secret Manager (Phase 2 production).
- `totebox://...` — os-totebox secret store (Phase 3+).

A missing secret fails closed at process start with a clear error
message. Log lines never include resolved secret values.

## Validating a configuration

```bash
slm-cli --config path.toml config validate
```

(Subcommand planned; see TASKS.md.)
