# Enabling the `/v1/jobs` family on a Doorman

Jobs are **off** unless `SLM_JOBS_ENABLED=true`. Any misconfiguration is a boot
error (the Doorman refuses to start rather than run jobs half-configured).

## Environment (add to `/etc/local-doorman/local-doorman.env`)

| Variable | Meaning |
|---|---|
| `SLM_JOBS_ENABLED=true` | Turn the family on. |
| `SLM_ENGINE_REGISTRY_FILE` | Path to the engine registry JSON (below). Required. |
| `SLM_JOBS_CREDENTIALS_FILE` | Path to the credential table JSON. Required; refused if group/world accessible (`chmod 600`, owned by the service user). |
| `SLM_JOBS_DIR` | Buffer directory (default `$FOUNDRY_ROOT/data/jobs`). Created `0700`. |
| `SLM_JOBS_MAX_UPLOAD_BYTES` | Per-upload cap (default 64 MiB). |
| `SLM_JOBS_MAX_BUFFER_BYTES` | Total input+result buffer cap (default 2 GiB; uploads get HTTP 507 until jobs are acked). |
| `SLM_JOBS_BUFFER_TTL_SECS` | Reaper TTL for finished-job buffers and unused uploads (default 7 days). |

The host Doorman binds `0.0.0.0:19080`. With jobs on, every `/v1/jobs*` and
`/v1/engines` request needs `Authorization: Bearer <token>`; other routes are unchanged.

## Adapter pulls (`/v1/adapters/pull`)

`ADAPTER_SOURCE_ALLOWLIST` is a comma-separated list of approved source hosts. It is
**empty by default**: until an operator sets it, every pull is refused (HTTP 403, fail
closed) and no connection is made. Set it in `/etc/local-doorman/local-doorman.env`.

- Entry forms: `host`, `host:port`, `[v6addr]:port`, or a full `http(s)://host[:port]` URL
  (only its host and port are used). An entry without a port admits any port on that host;
  an entry with a port admits only that port.
- The `source_url` is parsed first and the host (and port) compared after parsing, not by
  string prefix. `http://listed@evil.example/` and `http://listed.evil.example/` do not match.
- Only `http` and `https` are accepted. A URL carrying `user[:password]@` is refused (400).
- Redirects are never followed, so a listed host cannot redirect a pull to an unlisted one;
  a 3xx answer is reported as a failed fetch (HTTP 502).
- `ADAPTER_MAX_BYTES` caps a pull (default 512 MiB).

## Engine registry (`SLM_ENGINE_REGISTRY_FILE`)

```json
{
  "engines": [
    {
      "engine_id": "olmo-3-7b-q4",
      "kind": "llm-local",
      "endpoint": "http://127.0.0.1:8080",
      "model": "OLMo-3-7B-Instruct",
      "weights_sha256": "<sha256 of the gguf>",
      "licence": "<verified licence id>",
      "permits": 1,
      "max_job_secs": 900,
      "max_input_bytes": 20971520,
      "chunk_bytes": 8192,
      "chunk_overlap": 256
    },
    {
      "engine_id": "lighton-ocr-2-1b-q8",
      "kind": "ocr-openai-vision",
      "endpoint": "http://127.0.0.1:8081",
      "model": "lighton-ocr-2-1b-q8",
      "weights_sha256": "<sha256>",
      "licence": "<verified licence id>",
      "permits": 1,
      "cpu_set": "0-4",
      "nice": 19,
      "max_job_secs": 900,
      "max_input_bytes": 20971520
    },
    {
      "engine_id": "gliner-guest",
      "kind": "span-gliner",
      "endpoint": "http://127.0.0.1:9085",
      "model": "gliner",
      "weights_sha256": "<sha256>",
      "licence": "<verified licence id>",
      "permits": 1,
      "max_job_secs": 300,
      "max_input_bytes": 20971520,
      "chunk_bytes": 2000,
      "chunk_overlap": 100,
      "domain_id": "projects"
    }
  ]
}
```

- `fallback` must be absent or null; there is no fallback between engines. An
  unavailable engine makes the job `deferred`.
- `cpu_set` / `nice` are **declared and reported only**. They are enforced by the
  engine's own systemd unit (`os-totebox/scripts/stage-ocr-engines.sh` writes
  `CPUAffinity=` / `Nice=`); keep the two in agreement.
- `llm-local` jobs run only while Tier A is completely idle (P2), so interactive
  OLMo traffic is never delayed by a job.
- `span-gliner` label domains: an engine serves its own `domain_id` (default
  `projects`). To let one engine serve several domains, list them in `domains`
  (for example `["projects", "corporate", "documentation"]`; each a label of
  `a-z`, `0-9`, `-`, `_`; the engine's own `domain_id` must be in the list) and
  have the caller send `params.domain_id` on each `extract-spans` job. A job
  naming a domain the entry does not list is refused (400); a job with no
  `domain_id` uses the engine's own. One engine has one permit, so the
  domains queue behind each other instead of reaching a single-inference
  service together.
- Unknown fields are rejected.

## Credential table (`SLM_JOBS_CREDENTIALS_FILE`)

Make a token and its hash (store the token with the caller, only the hash here):

```bash
tok=$(openssl rand -hex 32); echo "token: $tok"; printf %s "$tok" | sha256sum | cut -d' ' -f1
```

```json
{
  "credentials": [
    {
      "credential_id": "caller-a-worker",
      "token_sha256": "<64 lowercase hex>",
      "module_id": "caller-a",
      "priority_class": "idle",
      "engines": ["olmo-3-7b-q4", "lighton-ocr-2-1b-q8", "gliner-guest"]
    },
    {
      "credential_id": "caller-b-run",
      "token_sha256": "<64 lowercase hex>",
      "module_id": "caller-b",
      "priority_class": "idle",
      "engines": ["lighton-ocr-2-1b-q8"],
      "input_dir": "/var/lib/os-totebox-jobs/caller-b-inputs"
    }
  ]
}
```

`module_id`, the engine allow-list and the priority class come only from the
matched entry. `X-Foundry-Module-ID` and `X-Foundry-Priority` are ignored on
these routes. `input_dir` (optional) enables `input.local_name` for files that
already sit on the host (single file name, symlinks that leave the directory
are refused); the Doorman never deletes such files.

## Caller flow

1. `POST /v1/jobs/inputs` (raw body, `Content-Type` = media type) → `{staged_ref, sha256, size}`
2. `POST /v1/jobs` `{engine_id, kind, input:{sha256, media_type, staged_ref | local_name}, params:{prompt_id?, schema_id?, temperature?, seed?, max_tokens?, page_ref?}}` → `202 {job_id}`
   - `kind`: `extract-llm` (prompts `entities-v1` raw JSON, `entities-v2` witness-validated), `ocr-page` (png/jpeg/webp, byte-for-byte), `extract-spans` (prompt `spans-v1`)
3. `GET /v1/jobs/{id}` → `status` in `queued|running|done|empty|not-applicable|unreadable|deferred|failed`, plus a hash/parameter `record`
4. `GET /v1/jobs/{id}/result` → `{job, text, entities?, rejected?}` (entities carry UTF-8 byte `span`s into the full input)
5. `POST /v1/jobs/{id}/ack` → deletes the staged input and result (idempotent; audit row written)
6. `POST /v1/jobs/{id}/replay` → new job; its `replay_match` says whether `output_sha256` equals the original's (needs the input, i.e. before ack)
7. `GET /v1/jobs/compare?a=&b=` → agreement between two engines' entity results
8. `GET /v1/engines` → the engines this credential may use

The audit ledger gets a `job` entry per finished job (and one on ack/TTL): ids,
hashes, counts and timings only — never input or output content.
