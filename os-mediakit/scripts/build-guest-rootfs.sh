#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# build-guest-rootfs.sh — G2.5: os-mediakit's seL4/libvmm guest rootfs.
#
# Produces: build/guest-rootfs/rootfs.cpio.gz (aarch64, glibc — Ubuntu 24.04
#           "noble" minimal base via debootstrap, overlaid with
#           app-mediakit-knowledge, running all 3 wiki tenants)
#
# Ported from app-orchestration-command/scripts/build-guest-rootfs.sh
# (2026-08-26), itself mirroring os-totebox's/app-orchestration-slm's own
# copies — same base OS -> install binary -> install init -> package shape.
# The one real technical change from that template, not cosmetic renaming:
# this guest runs ONE binary as THREE independent processes (documentation/
# projects/corporate tenants, ports 9090/9093/9095 — matching the ratified
# 1-VM topology, BRIEF-os-mediakit-product-family.md Decisions-open #1),
# not two different binaries.
#
# G-TLS (2026-09-01): nginx + certbot core are now installed and nginx
# reverse-proxies each of the 3 backends by server_name (real public domains
# — documentation.pointsav.com, projects.woodfinegroup.com,
# corporate.woodfinegroup.com), with an ACME HTTP-01 webroot wired and
# /etc/letsencrypt bind-mounted to /data for future cert persistence.
# Structure only — no real cert exists and no real `certbot certonly` request
# is made anywhere in this build: there is still no real public DNS/
# reachability in this test environment to issue against (operator
# direction, 2026-08-25, still true). What changed 2026-09-01 is scoped to
# what doesn't need a real target: the reverse-proxy scaffolding, verified
# live (M-SMOKE's nginx routing checks below), not the actual TLS/ACME parts,
# which stay genuinely blocked on Phase T2A providing a real target.
#
# /data persistence (2026-08-26): /init bind-mounts a product-agnostic
# /data/app-mediakit-knowledge/<instance>/{content,state} layout onto each
# tenant's /var/lib/wiki{,-state}/<instance>, seeding content from the
# baked-in sample on first boot. Per BRIEF-os-mediakit-product-family.md
# Decisions-open #9 — keyed by product+instance so a later app-mediakit-
# marketing/-distributions port onto this same disk doesn't collide with
# or require redoing this layout.
#
# Requires: debootstrap, qemu-user-static (for the aarch64 chroot second
#           stage), cpio.
#
# Usage:
#   BINARIES_DIR=/path/to/aarch64-unknown-linux-gnu/release \
#   bash scripts/build-guest-rootfs.sh
set -euo pipefail

ARCH="arm64"
RUST_TARGET="aarch64-unknown-linux-gnu"
UBUNTU_RELEASE="noble"  # 24.04 LTS
UBUNTU_MIRROR="http://ports.ubuntu.com/ubuntu-ports"
BUILD_DIR="build"
BASE_DIR="${BUILD_DIR}/guest-rootfs-base"
OVERLAY="${BUILD_DIR}/guest-rootfs-overlay"
_CARGO_RELEASE="${CARGO_TARGET_DIR:-../../target}/${RUST_TARGET}/release"
BINARIES_DIR="${BINARIES_DIR:-${_CARGO_RELEASE}}"
OUTPUT_ROOTFS="${BUILD_DIR}/guest-rootfs/rootfs.cpio.gz"
LIBVMM_TOOLS="../vendor-libvmm/tools"

WIKI_UID=990
WIKI_GID=990

# ── 1. Preflight ─────────────────────────────────────────────────────────────
# Same fresh-checkout fix app-orchestration-command's own port already
# applied (real bug found there 2026-08-06, latent in os-totebox's/SLM's
# copies too): debootstrap fails with "cd: can't cd to build" without this.
mkdir -p "${BUILD_DIR}"
for CMD in debootstrap cpio; do
    command -v "${CMD}" >/dev/null || { echo "error: ${CMD} not found on PATH"; exit 1; }
done
[ -x "${LIBVMM_TOOLS}/packrootfs" ] || {
    echo "error: ${LIBVMM_TOOLS}/packrootfs not found — run from os-mediakit/ with vendor-libvmm as a sibling (via project-knowledge's clone)"
    exit 1
}
[ -f "${BINARIES_DIR}/app-mediakit-knowledge" ] || {
    echo "error: app-mediakit-knowledge not found in BINARIES_DIR=${BINARIES_DIR}" >&2
    echo "  cross-compile first: cargo build --release --target ${RUST_TARGET} -p app-mediakit-knowledge" >&2
    exit 1
}

# Same resource-pressure guard as build-microkit-image.sh — debootstrap is
# disk+network heavy and this host is shared across concurrent Totebox
# sessions; hard-abort, not warn-and-continue.
_mem_total_kb=$(awk '/^MemTotal:/{print $2}' /proc/meminfo)
_mem_avail_kb=$(awk '/^MemAvailable:/{print $2}' /proc/meminfo)
_mem_used_pct=$(awk "BEGIN{printf \"%.1f\", (${_mem_total_kb}-${_mem_avail_kb})/${_mem_total_kb}*100}")
_psi_mem_some=$(awk '/^some/ {for(i=1;i<=NF;i++) if($i~/^avg10=/) {sub(/avg10=/,"",$i); print $i; exit}}' \
    /sys/fs/cgroup/memory.pressure 2>/dev/null || echo "0.00")
read -r _load_1 _ _ _ < /proc/loadavg
_nproc_count=$(nproc 2>/dev/null || echo 1)
if awk "BEGIN{exit !(${_mem_used_pct} > 85 || ${_psi_mem_some} > 10 || ${_load_1} >= ${_nproc_count})}"; then
    if [ "${FOUNDRY_RESOURCE_GUARD_BYPASS:-0}" = "1" ]; then
        echo "  WARN: host under pressure (mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}) — FOUNDRY_RESOURCE_GUARD_BYPASS=1 set, proceeding anyway (operator-approved 2026-08-26)" >&2
    else
        echo "error: refusing to start debootstrap — host already under pressure" >&2
        echo "  mem_used_pct=${_mem_used_pct}% psi_mem_some_avg10=${_psi_mem_some} load_1=${_load_1} nproc=${_nproc_count}" >&2
        echo "  wait for load/memory to settle and rerun -- this is a hard gate, not a warning" >&2
        echo "  confirmed operator override: FOUNDRY_RESOURCE_GUARD_BYPASS=1 bash ${0} ..." >&2
        exit 1
    fi
else
    echo "  preflight OK: mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}"
fi

# ── 2. Debootstrap base (two-stage: foreign-arch extract, then qemu-user chroot) ──
if [ ! -f "${BASE_DIR}/.debootstrap-complete" ]; then
    echo "  debootstrapping ${UBUNTU_RELEASE} ${ARCH} base (stage 1: foreign)..."
    sudo debootstrap --arch="${ARCH}" --foreign "${UBUNTU_RELEASE}" "${BASE_DIR}" "${UBUNTU_MIRROR}"
    echo "  debootstrap stage 2 (qemu-user chroot)..."
    sudo cp "$(command -v qemu-aarch64-static)" "${BASE_DIR}/usr/bin/"
    sudo chroot "${BASE_DIR}" /debootstrap/debootstrap --second-stage
    sudo touch "${BASE_DIR}/.debootstrap-complete"
else
    echo "  cached: ${BASE_DIR} (debootstrap already complete)"
fi

# ── 2b. G-TLS scaffolding — nginx only, cached into BASE_DIR alongside the
# debootstrap base (2026-09-01, revised same day — see real bug below).
# `nginx-light` was the original intent (smaller — this is a reverse proxy
# in front of 3 backends, no extra modules needed) but it no longer exists
# as a separate package in noble (apt names `nginx` as its direct
# replacement), so plain `nginx` is installed instead.
#
# REAL BUG, same-day revision: the first version of this step also
# installed `certbot` (`apt-get install nginx certbot`, after adding the
# `universe` component certbot lives in — `main` alone gives "no
# installation candidate"). That build succeeded, but the guest **panicked
# on boot**: certbot's Python dependency chain (cryptography, requests,
# acme, josepy, ~10 packages) grew the compressed rootfs from 89M to 127M
# (392MB uncompressed) — enough to blow past this guest's actual RAM
# ceiling. `-m size=2G` on the QEMU/Microkit VMM invocation is NOT the
# guest kernel's own RAM: `vendor-libvmm/examples/virtio/client_vm/
# linux.dts`'s `memory@40000000` node declares exactly 0x40000000 bytes —
# 1GiB — as what the guest Linux kernel is told it has, a hard ceiling
# separate from the VMM-level pool. Panic was `Initramfs unpacking failed:
# write error` → `VFS: Unable to mount root fs`. certbot cannot actually
# issue anything until a real target with real public DNS exists anyway
# (Phase T2A, still deferred) — so the fix is scope, not a bigger VM:
# certbot is NOT installed in this build. nginx alone (no Python
# dependency chain) stays well within the 1GiB ceiling. Re-add certbot at
# whatever point Phase T2A actually needs it — either a rebuild at that
# time, or installed directly on the real target rather than baked into
# every dev-loop guest-rootfs rebuild.
if [ ! -f "${BASE_DIR}/.nginx-installed" ]; then
    echo "  installing nginx (G-TLS reverse-proxy scaffolding only — certbot deferred to Phase T2A, see comment above)..."
    sudo cp "$(command -v qemu-aarch64-static)" "${BASE_DIR}/usr/bin/"
    sudo chroot "${BASE_DIR}" /bin/sh -c "export DEBIAN_FRONTEND=noninteractive; apt-get update && apt-get install -y --no-install-recommends nginx"
    sudo rm -f "${BASE_DIR}/usr/bin/qemu-aarch64-static"
    sudo touch "${BASE_DIR}/.nginx-installed"
else
    echo "  cached: ${BASE_DIR} (nginx already installed)"
fi

# ── 3. Assemble overlay (copy base, don't mutate it — keep it re-usable/cached) ──
sudo rm -rf "${OVERLAY}"
sudo cp -a "${BASE_DIR}" "${OVERLAY}"
sudo rm -f "${OVERLAY}/.debootstrap-complete" "${OVERLAY}/usr/bin/qemu-aarch64-static"

# ── 4. Install the binary + 'wiki' system user (matches Format B's convention) ──
echo "  installing app-mediakit-knowledge..."
sudo install -D -m 0755 "${BINARIES_DIR}/app-mediakit-knowledge" "${OVERLAY}/usr/local/bin/app-mediakit-knowledge"

echo "  wiki system user (uid=${WIKI_UID})..."
echo "wiki:x:${WIKI_UID}:${WIKI_GID}:Wiki Service:/var/lib/wiki:/usr/sbin/nologin" | sudo tee -a "${OVERLAY}/etc/passwd" > /dev/null
echo "wiki:x:${WIKI_GID}:" | sudo tee -a "${OVERLAY}/etc/group" > /dev/null

# ── 4b. Content directories + sample article + TOML configs (baked in at
# build time, same convention as Format B's build-image.sh — not generated
# dynamically by /init) ──
echo "  wiki content + config..."
declare -A PORTS=( [documentation]=9090 [projects]=9093 [corporate]=9095 )
declare -A BRANDS=( [documentation]=pointsav [projects]=woodfine [corporate]=woodfine )
for INSTANCE in documentation projects corporate; do
    sudo mkdir -p "${OVERLAY}/var/lib/wiki/${INSTANCE}" "${OVERLAY}/var/lib/wiki-state/${INSTANCE}"
done
# All 3 instances get a sample article, not just documentation — real bug
# found 2026-08-26 chasing an unrelated /data-persistence investigation:
# projects/corporate had no baked-in content at all (mkdir-only), so every
# boot's /init correctly saw their /data seed source as empty and correctly
# copied zero files — three separate "durability fix" attempts (explicit
# sync, QEMU cache=writethrough, 65s elapsed time before kill) all failed to
# change this because none of them addressed the real cause: there was
# never anything to persist. The evidence was already visible in every
# prior boot's own log (`indexed 0 article(s)` x2, `indexed 1 article(s)`
# x1) and went unnoticed until directly instrumented with df/find counts.
for INSTANCE in documentation projects corporate; do
    sudo tee "${OVERLAY}/var/lib/wiki/${INSTANCE}/getting-started.md" > /dev/null << EOF
---
title: Getting Started
description: Welcome to your ${BRANDS[${INSTANCE}]^} ${INSTANCE^} Wiki
date: 2026-01-01
quality: stub
---

# Getting Started

This is your ${BRANDS[${INSTANCE}]^} ${INSTANCE^} Wiki. Add Markdown files to this directory to create articles.
EOF
done
sudo chown -R "${WIKI_UID}:${WIKI_GID}" "${OVERLAY}/var/lib/wiki" "${OVERLAY}/var/lib/wiki-state"
sudo chmod 0750 "${OVERLAY}/var/lib/wiki" "${OVERLAY}/var/lib/wiki-state"

declare -A DOMAINS=( [documentation]=documentation.pointsav.com [projects]=projects.woodfinegroup.com [corporate]=corporate.woodfinegroup.com )

sudo mkdir -p "${OVERLAY}/etc/wiki"
for INSTANCE in documentation projects corporate; do
    sudo tee "${OVERLAY}/etc/wiki/${INSTANCE}.toml" > /dev/null << TOML
# /etc/wiki/${INSTANCE}.toml — G2.5 guest build; canonical_url points at the
# real public domain ahead of G-TLS actually being live (2026-09-01) — nginx
# (below) proxies to this backend by Host header, but nginx itself has no
# real cert/DNS reachability yet in this test environment (see BRIEF's G-TLS
# section). Direct curl against 127.0.0.1:${PORTS[${INSTANCE}]}/healthz (M-SMOKE) is
# unaffected by this value either way.
[site]
title         = "${INSTANCE^} Wiki"
brand         = "${BRANDS[${INSTANCE}]}"
bind          = "0.0.0.0:${PORTS[${INSTANCE}]}"
state_dir     = "/var/lib/wiki-state/${INSTANCE}"
instance      = "${INSTANCE}"
canonical_url = "https://${DOMAINS[${INSTANCE}]}"

[[mount]]
path          = "/var/lib/wiki/${INSTANCE}"
role          = "primary"
blueprint_set = ["TOPIC", "GUIDE"]
TOML
done
sudo chown -R "root:${WIKI_GID}" "${OVERLAY}/etc/wiki"
# File-level chmod BEFORE the directory lockdown below — the glob
# ("${OVERLAY}/etc/wiki/"*.toml) expands in this unprivileged shell, not
# inside sudo's privileged one; once the directory itself is chmod 0750
# (root:wiki, unreadable to the invoking user), the glob can no longer
# match anything and bash passes the literal "*.toml" through, which then
# 404s against a file that doesn't exist. Real bug, found running this
# script for real (2026-08-26) — not caught by bash -n syntax checking.
sudo chmod 0640 "${OVERLAY}/etc/wiki/"*.toml
sudo chmod 0750 "${OVERLAY}/etc/wiki"

# ── 4c. G-TLS scaffolding — nginx reverse proxy + ACME HTTP-01 webroot
# (2026-09-01). Structure only: no real cert exists, no real ACME request is
# made anywhere in this build. Once a real target has real public DNS
# pointing at it (Phase T2A), the commented `certbot certonly` command in
# /init's startup comment below is the documented next real step — not run
# here. Single conf.d file, not systemd — this appliance has no systemctl,
# so nginx's default sites-enabled/default is removed rather than
# relying on any systemd-triggered reload path.
sudo rm -f "${OVERLAY}/etc/nginx/sites-enabled/default"
sudo mkdir -p "${OVERLAY}/var/www/acme-challenge/.well-known/acme-challenge"
sudo mkdir -p "${OVERLAY}/etc/nginx/conf.d"
{
    for INSTANCE in documentation projects corporate; do
        cat << NGINX
server {
    listen 80;
    server_name ${DOMAINS[${INSTANCE}]};

    # ACME HTTP-01 challenge path — shared webroot across all 3 server
    # blocks, matches the --webroot argument the future real certbot
    # invocation (documented in /init below) will use.
    location /.well-known/acme-challenge/ {
        root /var/www/acme-challenge;
    }

    # TODO (Phase T2A, once real certs exist under
    # /etc/letsencrypt/live/${DOMAINS[${INSTANCE}]}/): add a matching
    # \`listen 443 ssl;\` server block referencing fullchain.pem/privkey.pem,
    # and redirect this plain :80 block to https instead of proxying
    # directly. Not added yet — no cert to reference would just be a
    # broken config, not real scaffolding.
    location / {
        proxy_pass http://127.0.0.1:${PORTS[${INSTANCE}]};
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
NGINX
    done
} | sudo tee "${OVERLAY}/etc/nginx/conf.d/os-mediakit.conf" > /dev/null

# ── 5. Install /init — a direct appliance-style PID 1, not full systemd ────────
# Supervises all 3 tenants as background children — the one genuinely new
# piece vs. the 1-or-2-binary precedent scripts (os-totebox/SLM/Command all
# supervise at most 2 processes; this supervises 3 of the SAME binary).
INIT_TMP="$(mktemp)"
cat > "${INIT_TMP}" << 'INIT_EOF'
#!/bin/sh
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev 2>/dev/null || true
mkdir -p /dev/pts && mount -t devpts devpts /dev/pts 2>/dev/null || true
echo "os-mediakit appliance init starting..."
ip link set lo up 2>/dev/null || ifconfig lo up 2>/dev/null || true
ip link set eth0 up 2>/dev/null || ifconfig eth0 up 2>/dev/null || true
udhcpc -i eth0 -q -t 3 -T 1 2>/dev/null || dhclient -1 eth0 2>/dev/null || true
# Same DHCP-fallback fix as the precedent scripts' own /init (this minimal
# debootstrap base ships neither udhcpc nor dhclient by default).
if ! ip addr show eth0 2>/dev/null | grep -q "inet "; then
    echo "no DHCP client / no lease obtained — falling back to static QEMU usermode-networking address"
    ip addr add 10.0.2.15/24 dev eth0 2>/dev/null || true
    ip route add default via 10.0.2.2 2>/dev/null || true
fi

CMDLINE="$(cat /proc/cmdline 2>/dev/null || true)"
cmdline_param() {
    printf '%s\n' "${CMDLINE}" | tr ' ' '\n' | grep "^foundry\\.$1=" | tail -1 | cut -d= -f2-
}
FOUNDRY_MODE="$(cmdline_param mode)"  # "smoketest" or empty (default: production appliance)

# ── Persistent data disk — same mount/mkfs/fallback logic as the precedent
# scripts' /init (identical real bugs already found and fixed there: no
# /dev/vda1 partition device ever appears in this guest, only the whole-disk
# /dev/vda; mount needs an explicit -t ext4; mke2fs's destructive-
# confirmation prompt reads /dev/tty directly so it must be pre-wiped, not
# answered). Wired into the wiki content/state paths below (2026-08-26) via
# bind-mount, product-agnostic layout — see that block's own comment.
DATA_DEV=""
if [ -b /dev/vda1 ]; then
    DATA_DEV=/dev/vda1
elif [ -b /dev/vda ]; then
    DATA_DEV=/dev/vda
fi
if [ -n "${DATA_DEV}" ]; then
    mkdir -p /data
    FIRST_MOUNT_OUT="$(mount -t ext4 "${DATA_DEV}" /data 2>&1)"
    if [ $? -ne 0 ]; then
        echo "first mount attempt failed: ${FIRST_MOUNT_OUT}"
        if command -v mkfs.ext4 >/dev/null 2>&1; then
            echo "${DATA_DEV} has no usable filesystem — formatting (first boot)"
            dd if=/dev/zero of="${DATA_DEV}" bs=1M count=4 conv=fsync 2>&1
            DEV_SIZE_MB="$(($(blockdev --getsize64 "${DATA_DEV}" 2>/dev/null || echo 0) / 1048576))"
            if [ "${DEV_SIZE_MB}" -gt 4 ]; then
                dd if=/dev/zero of="${DATA_DEV}" bs=1M count=4 seek=$((DEV_SIZE_MB - 4)) conv=fsync 2>&1
            fi
            MKFS_OUT="$(mkfs.ext4 -F "${DATA_DEV}" 2>&1)"
            MKFS_RC=$?
            echo "mkfs.ext4 exit=${MKFS_RC}: ${MKFS_OUT}"
            if [ "${MKFS_RC}" -eq 0 ]; then
                MOUNT_OUT="$(mount -t ext4 "${DATA_DEV}" /data 2>&1)"
                MOUNT_RC=$?
                echo "post-format mount exit=${MOUNT_RC}: ${MOUNT_OUT}"
            fi
        fi
    fi
    if mountpoint -q /data 2>/dev/null; then
        echo "persistent data disk mounted at /data (${DATA_DEV})"
        # ── Wire /data into the wiki content/state paths — product-agnostic
        # layout (/data/<product>/<instance>/...), per BRIEF-os-mediakit-
        # product-family.md Decisions-open #9: this appliance is not assumed
        # to be the only product that will ever use this disk. Bind-mount,
        # not a path rewrite in /etc/wiki/*.toml — keeps the baked-in config
        # paths stable and means the appliance still works (non-persistently,
        # falling back to the baked-in sample content) on a boot with no data
        # disk attached at all, e.g. a plain G1-style dev boot with no
        # blk_storage passed.
        for INSTANCE in documentation projects corporate; do
            DATA_CONTENT="/data/app-mediakit-knowledge/${INSTANCE}/content"
            DATA_STATE="/data/app-mediakit-knowledge/${INSTANCE}/state"
            mkdir -p "${DATA_CONTENT}" "${DATA_STATE}"
            if [ -z "$(ls -A "${DATA_CONTENT}" 2>/dev/null)" ]; then
                echo "  ${INSTANCE}: /data content empty (first boot on this disk) — seeding from baked-in sample content"
                cp -a /var/lib/wiki/"${INSTANCE}"/. "${DATA_CONTENT}"/
            fi
            chown -R 990:990 "${DATA_CONTENT}" "${DATA_STATE}"
            if mount --bind "${DATA_CONTENT}" /var/lib/wiki/"${INSTANCE}" \
                && mount --bind "${DATA_STATE}" /var/lib/wiki-state/"${INSTANCE}"; then
                echo "  ${INSTANCE}: /var/lib/wiki/${INSTANCE} and /var/lib/wiki-state/${INSTANCE} now backed by ${DATA_CONTENT} / ${DATA_STATE} — live edits persist across reboots"
            else
                echo "  WARNING: ${INSTANCE}: bind-mount of ${DATA_CONTENT}/${DATA_STATE} failed — falling back to non-persistent baked-in content this boot"
            fi
        done
        # Cert persistence (2026-09-01, G-TLS scaffolding) — shared, not
        # per-instance: certbot organizes all domains' certs under one
        # /etc/letsencrypt tree regardless of how many domains are issued
        # for, so this uses a single "_shared" slot rather than the
        # per-instance layout above. No real certs exist yet (nothing has
        # run `certbot certonly` anywhere in this build), so this mount is
        # empty scaffolding today — it exists so cert persistence doesn't
        # need a second round of /init changes once real issuance happens.
        DATA_TLS="/data/app-mediakit-knowledge/_shared/letsencrypt"
        mkdir -p "${DATA_TLS}" /etc/letsencrypt
        if mount --bind "${DATA_TLS}" /etc/letsencrypt; then
            echo "  /etc/letsencrypt now backed by ${DATA_TLS} — future real certs persist across reboots"
        else
            echo "  WARNING: bind-mount of ${DATA_TLS} onto /etc/letsencrypt failed — certs (once issued) would not survive a reboot"
        fi
        # Explicit sync so a first-boot seed write is durable before any
        # abrupt guest kill (this harness's smoke-test path never cleanly
        # unmounts /data, and neither does a real production shutdown yet —
        # the guest DTS has no ACPI/power-button device, see the BRIEF's
        # Phase Deploy note). NOTE 2026-08-26: an apparent durability bug
        # chased at length here (documentation's seed persisting,
        # projects'/corporate's not) turned out to have nothing to do with
        # sync/flush/cache timing at all — the real cause was upstream, in
        # 4b above (projects/corporate had no baked-in sample content to
        # seed from in the first place, so `cp -a` of an empty source
        # correctly copied zero files every time). Fixed there. `sync` here
        # is still correct defensive practice, just wasn't the fix.
        sync
    else
        echo "WARNING: ${DATA_DEV} present but could not be mounted — /data unavailable this boot, all 3 tenants running on non-persistent baked-in content"
    fi
else
    echo "no attached data disk (/dev/vda or /dev/vda1) — /data unavailable this boot (expected for a G2.5 dev boot with no -drive passed)"
fi

# ── Start all 3 wiki tenants as background children ─────────────────────────
echo "starting app-mediakit-knowledge (3 tenants: documentation:9090, projects:9093, corporate:9095)..."
PIDS=""
for INSTANCE in documentation projects corporate; do
    WIKI_KNOWLEDGE_TOML="/etc/wiki/${INSTANCE}.toml" /usr/local/bin/app-mediakit-knowledge serve &
    PID=$!
    PIDS="${PIDS} ${PID}"
    echo "  ${INSTANCE} started (pid ${PID})"
done

# ── Start nginx (G-TLS scaffolding, 2026-09-01) — reverse proxy in front of
# the 3 backends above, per-domain server_name matching (see
# /etc/nginx/conf.d/os-mediakit.conf, templated at build time). Plain HTTP
# only; no real cert exists. `-t` validates config before starting so a bad
# template fails loudly here instead of nginx silently not listening.
# `-g "daemon off;"` keeps it as a normal tracked child, same as the 3 wiki
# processes above, rather than nginx's own default self-daemonizing behavior.
echo "validating + starting nginx (G-TLS reverse-proxy scaffolding, no real certs)..."
if nginx -t 2>&1; then
    nginx -g "daemon off;" &
    NGINX_PID=$!
    PIDS="${PIDS} ${NGINX_PID}"
    echo "  nginx started (pid ${NGINX_PID})"
else
    echo "  WARNING: nginx config validation failed — nginx NOT started this boot, direct backend ports still reachable"
    NGINX_PID=""
fi

if [ "${FOUNDRY_MODE}" != "smoketest" ]; then
    echo "production appliance mode — service will stay running (pass foundry.mode=smoketest on the kernel cmdline for the dev smoke-test+self-shutdown path)"
    wait
    echo "all wiki processes exited (unexpected in production mode) — dropping to a shell"
    exec /bin/sh
fi

echo "foundry.mode=smoketest — running dev verification path (smoke test + SIGTERM self-test for ALL 3 processes, then shell)"
echo "waiting for the chassis to come up before smoke test..."
python3 - << 'SMOKETEST_EOF'
import time
import urllib.error
import urllib.request

def get(url, timeout=3, host_header=None):
    req = urllib.request.Request(url, headers={"Host": host_header} if host_header else {})
    try:
        resp = urllib.request.urlopen(req, timeout=timeout)
        return resp.status, resp.read().decode("utf-8", errors="replace")
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode("utf-8", errors="replace")

def check(name, url, expect_status=None, retries=20, delay=1.5, host_header=None):
    last_exc = None
    for attempt in range(1, retries + 1):
        try:
            status, body = get(url, host_header=host_header)
            ok = expect_status is None or status == expect_status
            marker = "PASS" if ok else "FAIL"
            print(f"[M-SMOKE] {marker} {name}: HTTP {status} (attempt {attempt}) — {body[:200]}")
            return ok
        except Exception as e:
            last_exc = e
            time.sleep(delay)
    print(f"[M-SMOKE] FAIL {name}: unreachable after {retries} attempts — {type(last_exc).__name__}: {last_exc}")
    return False

results = [
    check("documentation /healthz", "http://127.0.0.1:9090/healthz", expect_status=200),
    check("projects /healthz",      "http://127.0.0.1:9093/healthz", expect_status=200),
    check("corporate /healthz",     "http://127.0.0.1:9095/healthz", expect_status=200),
]
print(f"[M-SMOKE] SUMMARY: {sum(results)}/{len(results)} reachable and returned an HTTP response")

# G-TLS scaffolding check (2026-09-01) — proves nginx is actually routing by
# server_name to the right backend, not just that it started. Plain HTTP,
# port 80, correct Host header per domain — this is the real thing nginx
# config templating could get wrong (wrong proxy_pass port, wrong
# server_name) that "nginx -t" validation alone would never catch.
nginx_results = [
    check("nginx→documentation (Host: documentation.pointsav.com)", "http://127.0.0.1:80/healthz",
          expect_status=200, host_header="documentation.pointsav.com"),
    check("nginx→projects (Host: projects.woodfinegroup.com)", "http://127.0.0.1:80/healthz",
          expect_status=200, host_header="projects.woodfinegroup.com"),
    check("nginx→corporate (Host: corporate.woodfinegroup.com)", "http://127.0.0.1:80/healthz",
          expect_status=200, host_header="corporate.woodfinegroup.com"),
]
print(f"[M-SMOKE] NGINX SUMMARY: {sum(nginx_results)}/{len(nginx_results)} routed correctly by server_name")
SMOKETEST_EOF

# ── SIGTERM graceful-shutdown self-test — ALL 3 processes, not just one.
# Deliberately avoids the real gap the BRIEF's own gate ladder flags:
# app-orchestration-command's own /init has zero SIGTERM-handling code and
# only checks its own port closes, never a spawned child's.
echo "[SIGTERM-TEST] sending SIGTERM to all 3 wiki processes (pids:${PIDS})..."
for PID in ${PIDS}; do
    kill -TERM "${PID}" 2>/dev/null || true
done
for PID in ${PIDS}; do
    for i in $(seq 1 20); do
        if ! kill -0 "${PID}" 2>/dev/null; then
            echo "[SIGTERM-TEST] PASS pid ${PID} exited (confirmed via kill -0) after ${i}s"
            break
        fi
        sleep 1
    done
    if kill -0 "${PID}" 2>/dev/null; then
        echo "[SIGTERM-TEST] FAIL pid ${PID} still running 20s after SIGTERM"
    fi
done
python3 - << 'PORTCHECK_EOF'
import urllib.error
import urllib.request

def port_closed(name, url):
    try:
        urllib.request.urlopen(url, timeout=3)
        print(f"[SIGTERM-TEST] FAIL {name}: still answering after shutdown")
        return False
    except urllib.error.URLError:
        print(f"[SIGTERM-TEST] PASS {name}: no longer answering (connection refused/reset)")
        return True
    except Exception as e:
        print(f"[SIGTERM-TEST] FAIL {name}: unexpected {type(e).__name__}: {e}")
        return False

results = [
    port_closed("documentation :9090", "http://127.0.0.1:9090/healthz"),
    port_closed("projects :9093",      "http://127.0.0.1:9093/healthz"),
    port_closed("corporate :9095",     "http://127.0.0.1:9095/healthz"),
    port_closed("nginx :80",           "http://127.0.0.1:80/healthz"),
]
print(f"[SIGTERM-TEST] SUMMARY: {sum(results)}/{len(results)} ports confirmed closed after shutdown")
PORTCHECK_EOF

echo "smoke test complete — dropping to a shell"
exec /bin/sh
INIT_EOF
sudo cp "${INIT_TMP}" "${OVERLAY}/init"
rm -f "${INIT_TMP}"
sudo chmod +x "${OVERLAY}/init"

# ── 5b. Prune non-server content ────────────────────────────────────────────
# Real bug found 2026-09-01 chasing the G-TLS kernel panic (see 2b's comment
# for the full finding): this prune step already removed apt's downloaded
# .deb archives (var/cache/apt/archives) and package index lists (var/lib/
# apt/lists), but NOT apt's own internal binary metadata caches
# (var/cache/apt/pkgcache.bin + srcpkgcache.bin) — these two files alone
# were 86MB, dwarfing nginx's actual installed footprint, and were the real
# reason adding nginx (a genuinely small package) blew the rootfs from
# 89MB to 123-127MB compressed. Both are pure apt-internal caches, safe to
# delete (apt regenerates them on demand; nothing at runtime reads them —
# this appliance never runs apt again after build time). Also pruning
# udev's hwdb (its only consumer, systemd-udevd, never runs here — this
# appliance's own /init brings up networking directly via `ip`/`udhcpc`,
# not udev) and Python's __pycache__ (bytecode cache, regenerated on
# demand, not needed to ship).
echo "  pruning non-server content..."
sudo rm -rf "${OVERLAY}/var/cache/apt/archives" \
    "${OVERLAY}/var/cache/apt/pkgcache.bin" \
    "${OVERLAY}/var/cache/apt/srcpkgcache.bin" \
    "${OVERLAY}/var/cache/debconf/templates.dat-old" \
    "${OVERLAY}/var/cache/debconf/config.dat-old" \
    "${OVERLAY}/var/lib/apt/lists" \
    "${OVERLAY}/usr/lib/udev/hwdb.bin" \
    "${OVERLAY}/usr/lib/udev/hwdb.d" \
    "${OVERLAY}/usr/share/doc" \
    "${OVERLAY}/usr/share/man" \
    "${OVERLAY}/usr/share/locale" \
    "${OVERLAY}/usr/share/i18n" \
    "${OVERLAY}/usr/share/lintian" \
    "${OVERLAY}/usr/share/zoneinfo"
sudo find "${OVERLAY}/usr/share/locale-langpack" -mindepth 1 -maxdepth 1 ! -name "en*" -exec rm -rf {} \; 2>/dev/null || true
sudo find "${OVERLAY}" -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true

# ── 6. Package via packrootfs ────────────────────────────────────────────────
echo "  converting overlay directory to cpio.gz..."
mkdir -p "$(dirname "${OUTPUT_ROOTFS}")"
BASE_CPIO="${BUILD_DIR}/guest-rootfs-overlay.cpio.gz"
( cd "${OVERLAY}" && sudo find . | sudo cpio -o -H newc 2>/dev/null | gzip -9 ) > "${BASE_CPIO}"

echo "  packing final rootfs..."
mkdir -p "${BUILD_DIR}/packrootfs-tmp"
"${LIBVMM_TOOLS}/packrootfs" "${BASE_CPIO}" "${BUILD_DIR}/packrootfs-tmp" \
    -o "${OUTPUT_ROOTFS}"

echo ""
echo "  done: ${OUTPUT_ROOTFS}"
echo "  $(du -sh "${OUTPUT_ROOTFS}" | cut -f1)"
echo ""
echo "  boots as a real, running production appliance by default (no self-test,"
echo "  no self-shutdown), running all 3 wiki tenants (documentation:9090,"
echo "  projects:9093, corporate:9095)."
echo "  Pass foundry.mode=smoketest to run the dev/CI smoke test + SIGTERM"
echo "  self-test instead (drops to a shell after, does not stay up)."
echo ""
echo "  use with vendor-libvmm's examples/virtio build via:"
echo "    make MICROKIT_BOARD=qemu_virt_aarch64 MICROKIT_SDK=/opt/microkit-sdk-2.2.0 \\"
echo "      BUILD_DIR=build-os-mediakit INITRD=$(pwd)/${OUTPUT_ROOTFS} qemu"
