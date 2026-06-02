# Migrate Services to vm-mediakit

This guide covers migrating the MediaKit service surface from the GCP host into the
`vm-mediakit` Ubuntu 24.04 guest VM using `infrastructure/virt/migrate-service-to-vm.sh`.

**Safety guarantee:** all original services remain running on the host throughout. No DNS
records change. Each service is verified in the VM via a host port-forward before proceeding
to the next. The host and VM run in parallel until the operator decides to cut over DNS.

Before running this guide, the VM must be running and passing all four verification checks
in `guide-vm-mediakit-provision.md`.

---

## Migration sequence

Migrate services in this order. Each service listed must be verified before starting the next.

| Order | Service | Port | Blocker |
|---|---|---|---|
| 1 | service-fs | 9100 | Command Session must promote project-data 23 commits first |
| 2 | proofreader | 9092 | None — internal API only |
| 3 | knowledge-documentation | 9090 | None |
| 4 | knowledge-corporate | 9095 | None |
| 5 | knowledge-projects | 9093 | None |
| 6 | marketing-pointsav | 9101 | None |
| 7 | marketing | 9102 | None |
| 8 | bim-orchestration | 9096 | service-fs must be running in VM first |

As of 2026-05-29: services 2–7 are running in the VM. Services 1 and 8 are blocked.

---

## Running a migration

```bash
cd /srv/foundry/clones/project-infrastructure
./infrastructure/virt/migrate-service-to-vm.sh <service-name> <port>
```

The script performs five steps:

1. **SSH verify** — confirms the VM is reachable
2. **Binary copy** — `scp` the release binary to `/opt/mediakit/bin/` via a port-suffixed
   staging path (prevents collision if running two migrations simultaneously)
3. **Data copy** — transfers content directories using `tar` piped over SSH (no rsync needed)
4. **Systemd unit** — adapts the unit file from `infrastructure/systemd/local-<name>.service`
   (replaces `/usr/local/bin/` with `/opt/mediakit/bin/`), creates the WorkingDirectory,
   uploads to `/etc/systemd/system/`, and runs `systemctl enable --now`
5. **Smoke test** — `curl` to `http://localhost:1<port>/` with a 60-second timeout and
   non-fatal error handling (first request on TCG can take 30–60 s)

---

## Service-by-service instructions

### service-fs (port 9100) — BLOCKED

service-fs is the WORM ledger that bim-orchestration depends on. Its binary must be built
from project-data source, which requires the Command Session to promote project-data's
23 pending commits before the build can run.

Once the binary is available at `/usr/local/bin/service-fs`:

```bash
./infrastructure/virt/migrate-service-to-vm.sh service-fs 9100
```

Verify:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 30 http://localhost:19100/healthz
# Expected: 200
```

### proofreader (port 9092)

```bash
./infrastructure/virt/migrate-service-to-vm.sh proofreader 9092
```

Verify: the proofreader is an internal API, not a web UI. It returns HTTP 404 at `/`
when correctly running:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 30 http://localhost:19092/
# Expected: 404 (service is responding; root path is not defined)
```

Check `journalctl -u local-proofreader.service` inside the VM if the response is `000`
(no connection). Note: Doorman (`localhost:9080`) and LanguageTool (`localhost:8010`) are
listed in the unit file as environment variables; the proofreader starts and binds without
requiring them to be reachable.

### knowledge-documentation (port 9090)

```bash
./infrastructure/virt/migrate-service-to-vm.sh knowledge-documentation 9090
```

This service transfers approximately 20 MB of wiki content via tar pipe. The tar step
may take 1–2 minutes.

Verify:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 90 http://localhost:19090/
# Expected: 200
```

Note the 90-second timeout. On TCG, the first request triggers Tantivy search index
initialisation and git repository setup — this takes 30–60 seconds. Subsequent requests
are faster.

### knowledge-corporate (port 9095) and knowledge-projects (port 9093)

These use the same binary as knowledge-documentation (`app-mediakit-knowledge`). The binary
was already installed during the knowledge-documentation migration; the script overwrites it
(no harm done). Each transfers approximately 4 MB of content.

```bash
./infrastructure/virt/migrate-service-to-vm.sh knowledge-corporate 9095
./infrastructure/virt/migrate-service-to-vm.sh knowledge-projects 9093
```

Do not run these simultaneously — both copy the binary to the same destination and the
second `mv` would race the first. Run sequentially.

Verify each:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 90 http://localhost:19095/
curl -s -o /dev/null -w "%{http_code}" --max-time 90 http://localhost:19093/
# Expected: 200 for both
```

### marketing-pointsav (port 9101) and marketing (port 9102)

```bash
./infrastructure/virt/migrate-service-to-vm.sh marketing-pointsav 9101
./infrastructure/virt/migrate-service-to-vm.sh marketing 9102
```

The marketing binary (`app-mediakit-marketing`) is a static-file server. Responses are
fast — the smoke test typically completes within the default 60-second window.

Verify:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 15 http://localhost:19101/
curl -s -o /dev/null -w "%{http_code}" --max-time 15 http://localhost:19102/
# Expected: 200 for both
```

### bim-orchestration (port 9096) — BLOCKED

bim-orchestration depends on `service-fs` at `http://127.0.0.1:9100` inside the VM.
Migrate service-fs first and confirm it is running before attempting this step.

```bash
./infrastructure/virt/migrate-service-to-vm.sh bim-orchestration 9096
```

Verify:

```bash
curl -s -o /dev/null -w "%{http_code}" --max-time 60 http://localhost:19096/
```

---

## Verifying all services at once

Once all services are migrated, this command checks the full service roster inside the VM:

```bash
KEY="infrastructure/virt/work/foundry-vm-key"
ssh -p 10022 -i ${KEY} -o StrictHostKeyChecking=no foundry@localhost \
  "systemctl is-active local-proofreader.service local-knowledge-documentation.service \
   local-knowledge-corporate.service local-knowledge-projects.service \
   local-marketing-pointsav.service local-marketing.service \
   local-bim-orchestration.service local-fs.service"
```

Each line of output should be `active`. A line reading `inactive` or `failed` means that
service did not migrate successfully.

---

## Checking service logs

Inside the VM, all services log to the systemd journal:

```bash
KEY="infrastructure/virt/work/foundry-vm-key"
ssh -p 10022 -i ${KEY} -o StrictHostKeyChecking=no foundry@localhost \
  "journalctl -u local-knowledge-documentation.service -n 30 --no-pager"
```

Replace `local-knowledge-documentation.service` with any other unit name.

---

## What the smoke test result means

| `curl` output | Meaning |
|---|---|
| `200` | Service is running and responding normally |
| `301` / `302` | Service is redirecting — likely still healthy; check the redirect target |
| `404` | Service running; root path not defined (expected for proofreader) |
| `000` (no response, 60s timeout) | Service started but first request is slow on TCG; wait 30s and retry |
| `000` (immediate) | Service not running; check `systemctl status` and `journalctl` |
| Script exits with error | Binary copy or systemd step failed; read the script output for the failed step |

---

## TCG performance

QEMU/TCG runs at approximately one-tenth the speed of a native KVM guest. Expectations:

- Binary copy (19 MB binary): 15–30 seconds
- Tar pipe for 20 MB content: 30–60 seconds
- Service startup (first request): 30–60 seconds for wiki services (search index + git init)
- Service startup (first request): 2–5 seconds for static-file services (marketing)

These are normal. Slow responses do not indicate a service defect.

---

## After migration — pre-DNS checklist

Before changing any DNS record to point at the VM (via a WireGuard overlay or direct
address), confirm each of the following:

1. All target services return the expected HTTP status from the host port-forward
2. A sample page from each wiki renders the same content as the host-side original
3. `systemctl is-active local-*.service` shows `active` for all migrated services
4. `journalctl -u <unit>` shows no crash/restart loops in the last 30 minutes
5. The host-side originals are still running (`systemctl is-active <original-unit>`)

Do not delete or stop host-side originals until the VM-side services have been validated
under real traffic for at least one session.

---

## See also

- `guide-vm-mediakit-provision.md` — provision the Ubuntu 24.04 VM (prerequisite)
- `guide-vm-prove-balloon-demo.md` — verify the virtio-balloon resource pool mechanism
