#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# qmp-shutdown.py — send a QEMU Machine Protocol `system_powerdown` request
# over a QMP unix socket.
#
# Ported verbatim from project-totebox's os-totebox/scripts/qmp-shutdown.py
# (2026-08-27) — same shared vendor-libvmm/examples/virtio guest DTS, same
# limitation applies here unchanged, not re-derived independently.
#
# CONFIRMED NOT SUFFICIENT for graceful guest shutdown — inherited, not
# re-tested independently here, but there is no reason to expect a different
# result: this guest's DTS (client_vm/linux.dts, the same shared example
# os-totebox uses) declares no ACPI/power-button/GPIO device, so QEMU has
# nothing to deliver `system_powerdown` to inside the guest. The command
# below succeeds at the QMP protocol level ("system_powerdown accepted") but
# the guest's own SIGTERM handler never fires. Kept as a real, working QMP
# client (useful building block for a future real fix — e.g. a
# virtio-console-based shutdown-request channel) — not as a claim that
# running this achieves graceful shutdown today. See BRIEF-os-mediakit-
# product-family.md's Phase Deploy section for the full context, including
# the /data persistence investigation this same limitation was cross-checked
# against.
#
# Usage: qmp-shutdown.py /run/os-mediakit/qmp.sock [timeout_seconds]
#
# Exit 0 once the QMP handshake + system_powerdown command both succeed.
# This does NOT wait for the guest to actually finish shutting down — the
# caller (systemd's ExecStop, via TimeoutStopSec) owns that wait, matching
# systemd's own stop-then-verify convention.

import json
import socket
import sys

sock_path = sys.argv[1]
timeout = float(sys.argv[2]) if len(sys.argv) > 2 else 5.0

s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
s.settimeout(timeout)
s.connect(sock_path)

def read_json(sock):
    buf = b""
    while not buf.endswith(b"\r\n") and b"}\n" not in buf and b"}\r\n" not in buf:
        chunk = sock.recv(4096)
        if not chunk:
            break
        buf += chunk
    return buf

# QMP handshake: server sends a greeting first, then expects capabilities negotiation.
greeting = read_json(s)
if b"QMP" not in greeting:
    sys.stderr.write(f"qmp-shutdown: unexpected greeting: {greeting!r}\n")
    sys.exit(1)

s.sendall(json.dumps({"execute": "qmp_capabilities"}) .encode() + b"\n")
cap_reply = read_json(s)
if b"return" not in cap_reply:
    sys.stderr.write(f"qmp-shutdown: capabilities negotiation failed: {cap_reply!r}\n")
    sys.exit(1)

s.sendall(json.dumps({"execute": "system_powerdown"}).encode() + b"\n")
powerdown_reply = read_json(s)
if b"return" not in powerdown_reply:
    sys.stderr.write(f"qmp-shutdown: system_powerdown rejected: {powerdown_reply!r}\n")
    sys.exit(1)

s.close()
print("qmp-shutdown: system_powerdown accepted")
