# Configuration

service-slm is configured via a TOML file. This document describes the
top-level configuration sections and their options. The full machine-
readable schema is in [reference/configuration-schema.md](../reference/configuration-schema.md).

## Default configuration locations

In order of precedence:

1. `--config <path>` on the command line.
2. `SLM_CONFIG` environment variable.
3. `./slm-cli.toml` in the current directory.
4. `~/.config/slm-cli/config.toml`.
5. `/etc/slm-cli/config.toml` (system-wide; Phase 3).

Example: `slm-cli --config /etc/pointsav/slm-cli.toml serve`.

## Sections

### `[server]`

```toml
[server]
listen = "127.0.0.1:8080"
shutdown_grace_seconds = 30
```

### `[doorman]`

```toml
[doorman]
sanitisation_policy = "default"  # see user-guide/operating.md
timeout_seconds = 60
retry_max_attempts = 5
```

### `[ledger]`

```toml
[ledger]
csv_path = "/var/lib/slm-cli/ledger/events.csv"
sqlite_path = "/var/lib/slm-cli/ledger/events.sqlite"
fsync_on_commit = true    # do not disable in production
```

### `[compute]`

```toml
[compute]
gcp_project = "pointsav-dev"
gcp_region = "us-central1"
manifest_path = "/etc/slm-cli/compute/manifest.yaml"
warm_pool_enabled = false
```

### `[memory.kv]`

```toml
[memory.kv]
mooncake_master_url = "tcp://mooncake.internal:50051"
local_cpu_cache_mib = 4096
module_namespace = true    # always true; set false only in single-tenant test
```

### `[memory.adapters]`

```toml
[memory.adapters]
registry_path = "/etc/slm-cli/memory/adapters/registry.yaml"
verify_signatures = true   # never disable in production
```

## Environment variables

Any option in the config file can be overridden by an environment
variable of the form `SLM_<SECTION>_<KEY>`, upper-cased with dots
replaced by underscores.

Example: `SLM_SERVER_LISTEN=0.0.0.0:9090 slm-cli serve`.

## Secrets

Secrets do not live in the config file. They are referenced by ID and
resolved at runtime against:

- In Phase 1/2 development: environment variables.
- In Phase 2 production: Google Secret Manager.
- In Phase 3 appliance: the Totebox secret store.

Example:

```toml
[compute]
api_key_ref = "secret-manager://projects/pointsav-dev/secrets/claude-api-key/versions/latest"
```

The reference is resolved at process start. A missing secret fails
closed with a clear error message.

## Validating a config

```bash
slm-cli --config my-config.toml config validate
```

(Subcommand planned; see TASKS.md.)
