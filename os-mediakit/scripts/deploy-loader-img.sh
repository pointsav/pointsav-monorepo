#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# deploy-loader-img.sh — transfer a built os-mediakit loader.img to a
# dedicated appliance VM and (re)start it as a managed systemd service.
#
# Ported from project-totebox's os-totebox/scripts/deploy-loader-img.sh
# (2026-08-27), adapted for os-mediakit's 3 wiki tenants (documentation,
# projects, corporate on 9090/9093/9095) instead of a single API port —
# same overall shape and the same real bugs precedent already found and
# fixed (pgrep -x truncated-comm matching, enable --now no-op on an
# already-active unit, self-matching pkill -f against the deploy command's
# own argv) ported as-is rather than re-derived.
#
# NOT YET RUN AGAINST A REAL TARGET as of this writing — no os-mediakit
# appliance VM exists yet (BRIEF-os-mediakit-product-family.md Decisions-open
# #10, "Phase T2A": production VM provisioning is deliberately deferred
# until multi-product validation is done). This script is the Phase Deploy
# deliverable; actually invoking it against a real GCE instance is Phase
# T2A's job, later — do not run this against a production target without
# re-confirming that gate has actually been cleared.
#
# Uses `gcloud compute scp`/`gcloud compute ssh` rather than plain scp/rsync
# — this workspace's VMs are reached via GCP IAP, not directly-routable SSH
# (same finding precedent already confirmed for os-totebox-1).
#
# Deliberately never touches blk_storage — that's the guest's persistent
# /data block device (BRIEF-os-mediakit-product-family.md's own persistence
# section — DONE and live-verified 2026-08-26, see that entry for the full
# finding). This script fails loudly if blk_storage is missing at the
# target rather than silently fabricating one in the wrong format.
#
# Usage:
#   bash scripts/deploy-loader-img.sh <instance-name> [zone] [loader.img path]
#
# Example:
#   bash scripts/deploy-loader-img.sh os-mediakit-1
#   bash scripts/deploy-loader-img.sh os-mediakit-1 us-west1-a vendor-libvmm/examples/virtio/build-os-mediakit/loader.img

set -euo pipefail

TARGET="${1:?usage: deploy-loader-img.sh <instance-name> [zone] [loader.img path]}"
ZONE="${2:-us-west1-a}"
ARCHIVE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# build-os-mediakit — this product's own isolated BUILD_DIR (Phase 0
# decision), never the shared vendor-libvmm/examples/virtio/build/ default.
LOADER_IMG="${3:-${ARCHIVE_ROOT}/vendor-libvmm/examples/virtio/build-os-mediakit/loader.img}"
QMP_SCRIPT="${ARCHIVE_ROOT}/os-mediakit/scripts/qmp-shutdown.py"
UNIT_FILE="${ARCHIVE_ROOT}/os-mediakit/systemd/os-mediakit-guest.service"
REMOTE_DIR="/var/lib/os-mediakit"

gssh() { gcloud compute ssh "${TARGET}" --zone="${ZONE}" --internal-ip --command "$1"; }
gscp() { gcloud compute scp "$1" "${TARGET}:$2" --zone="${ZONE}" --internal-ip; }

[ -f "${LOADER_IMG}" ] || { echo "error: ${LOADER_IMG} not found — build it first (os-mediakit/scripts/build-microkit-image.sh with INITRD= the real guest rootfs)"; exit 1; }
[ -f "${QMP_SCRIPT}" ] || { echo "error: ${QMP_SCRIPT} not found"; exit 1; }
[ -f "${UNIT_FILE}" ] || { echo "error: ${UNIT_FILE} not found"; exit 1; }

LOCAL_SHA="$(sha256sum "${LOADER_IMG}" | cut -d' ' -f1)"
echo "== local loader.img: ${LOADER_IMG} (${LOCAL_SHA}) =="

echo "== ensuring ${REMOTE_DIR} exists on ${TARGET} =="
gssh "sudo mkdir -p ${REMOTE_DIR} && sudo chown \$(whoami):\$(whoami) ${REMOTE_DIR}"

echo "== checking remote blk_storage — never overwritten by this script (persistent /data) =="
if ! gssh "test -f ${REMOTE_DIR}/blk_storage"; then
    echo "warning: ${REMOTE_DIR}/blk_storage does not exist yet on ${TARGET}."
    echo "  This script does not create it — /data provisioning is a separate"
    echo "  concern (BRIEF-os-mediakit-product-family.md's persistence section)."
    echo "  If migrating an existing appliance's data disk, move it into place first, e.g.:"
    echo "    gcloud compute ssh ${TARGET} --zone=${ZONE} --internal-ip --command 'sudo mv /tmp/blk_storage ${REMOTE_DIR}/blk_storage'"
    exit 1
fi

echo "== transferring loader.img (sha256-verified) =="
gscp "${LOADER_IMG}" "${REMOTE_DIR}/loader.img"
REMOTE_SHA="$(gssh "sha256sum ${REMOTE_DIR}/loader.img | cut -d' ' -f1")"
if [ "${LOCAL_SHA}" != "${REMOTE_SHA}" ]; then
    echo "error: sha256 mismatch after transfer (local ${LOCAL_SHA} != remote ${REMOTE_SHA})"
    exit 1
fi
echo "   verified: ${REMOTE_SHA}"

echo "== transferring qmp-shutdown.py + systemd unit =="
gscp "${QMP_SCRIPT}" "${REMOTE_DIR}/qmp-shutdown.py"
gscp "${UNIT_FILE}" "/tmp/os-mediakit-guest.service.new"
gssh "sudo mv /tmp/os-mediakit-guest.service.new /etc/systemd/system/os-mediakit-guest.service && sudo systemctl daemon-reload"

echo "== stopping any prior hand-launched instance (best-effort, not the new managed one) =="
# Same two real bugs precedent found and fixed, ported as-is:
# 1. `pgrep -f`/`pkill -f` against any pattern containing "qemu-system-aarch64"
#    self-matches this very ssh command's own argv over gcloud's
#    non-interactive --command invocation — use `-x` by process NAME only.
# 2. The kernel truncates /proc/[pid]/comm to 15 characters, so the real
#    process name is "qemu-system-aar", not the full "qemu-system-aarch64" —
#    plain `pgrep qemu-system-aarch64` (no -x, wrong length) never matches
#    anything, silently.
# Excludes the managed unit's own MainPID so a second run of this script
# never kills the instance it just started.
gssh "MANAGED_PID=\$(systemctl show -p MainPID --value os-mediakit-guest.service 2>/dev/null || echo 0); for PID in \$(pgrep -x qemu-system-aar 2>/dev/null || true); do if [ \"\$PID\" != \"\$MANAGED_PID\" ]; then echo \"killing orphaned instance PID=\$PID\"; sudo kill -TERM \$PID; sleep 2; sudo kill -0 \$PID 2>/dev/null && sudo kill -KILL \$PID; fi; done; echo done"

echo "== enabling + (re)starting os-mediakit-guest.service =="
# `enable --now` is a no-op on an already-active unit (exits 0, leaves the
# running process untouched) — same real bug precedent found: a redeploy
# with a genuinely new image never actually took effect. Explicit is-active
# check + restart, not reliance on enable --now's semantics.
gssh "sudo systemctl enable os-mediakit-guest.service; if sudo systemctl is-active --quiet os-mediakit-guest.service; then sudo systemctl restart os-mediakit-guest.service; else sudo systemctl start os-mediakit-guest.service; fi"

echo "== waiting for all 3 wiki tenants to answer /healthz =="
for PORT_NAME in "9090:documentation" "9093:projects" "9095:corporate"; do
    PORT="${PORT_NAME%%:*}"
    NAME="${PORT_NAME##*:}"
    for i in $(seq 1 30); do
        if gssh "curl -sf -m 3 http://127.0.0.1:${PORT}/healthz >/dev/null 2>&1"; then
            echo "   ${NAME} (:${PORT}) healthy after ${i}s"
            break
        fi
        sleep 1
        if [ "${i}" -eq 30 ]; then
            echo "error: ${NAME} (:${PORT}) did not answer /healthz within 30s — check: gcloud compute ssh ${TARGET} --zone=${ZONE} --internal-ip --command 'sudo journalctl -u os-mediakit-guest.service'"
            exit 1
        fi
    done
done

echo "== done. Managed via: gcloud compute ssh ${TARGET} --zone=${ZONE} --internal-ip --command 'sudo systemctl {status,stop,restart} os-mediakit-guest.service' =="
