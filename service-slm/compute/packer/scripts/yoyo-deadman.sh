#!/usr/bin/env bash
# Yo-Yo dead-man's-switch (Phase 0 G3 hardening).
#
# On every boot this schedules an unconditional VM self-shutdown at max-lifetime.
# It is the GUARANTEED teardown path: unlike the Doorman idle monitor and the
# start-yoyo.sh --runtime watchdog (both of which die with their host process),
# this runs on the VM itself and survives Doorman crashes and network partitions.
#
# The normal stop paths (idle monitor, stop-yoyo.sh) fire earlier; whichever
# fires first wins. If the VM is stopped before this fires, no harm — on the next
# boot the service runs again and re-arms.
set -uo pipefail

META="http://metadata.google.internal/computeMetadata/v1/instance"

# max-lifetime-seconds is set in instance metadata by start-yoyo.sh / OpenTofu.
MAX="$(curl -sf -H 'Metadata-Flavor: Google' "${META}/attributes/max-lifetime-seconds" || true)"
[[ "${MAX}" =~ ^[0-9]+$ ]] || MAX=14400   # default 4 h

# Round up to whole minutes; `shutdown` takes minutes.
MINUTES=$(( (MAX + 59) / 60 ))
[[ "${MINUTES}" -lt 1 ]] && MINUTES=1

# `shutdown -h +M` powers the guest off after M minutes; GCE then reports the
# instance TERMINATED (stopped, not deleted — the persistent weights disk
# survives). systemd holds the schedule; a reboot loses it but this service
# re-arms on the next boot.
shutdown -h "+${MINUTES}" "yoyo-deadman: max lifetime ${MAX}s reached" || true
echo "yoyo-deadman: armed — VM will self-stop in ${MINUTES} min (max-lifetime ${MAX}s)."
