#!/usr/bin/env bash
# Packer provisioner for the slm-yoyo GCE image.
# Installs: CUDA 12, Python 3.12, vLLM ≥ 0.12, Nginx TLS reverse proxy.
# Idempotent — safe to re-run.
set -euo pipefail

VLLM_PORT=${VLLM_PORT:-8000}

echo "==> Installing system packages"
apt-get update -qq
apt-get install -y --no-install-recommends \
    curl gnupg ca-certificates \
    nginx openssl \
    python3 python3-pip python3-venv

# ── CUDA ──────────────────────────────────────────────────────────────────────
echo "==> Installing CUDA keyring"
CUDA_KEYRING_URL="https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2404/x86_64/cuda-keyring_1.1-1_all.deb"
curl -fsSL "${CUDA_KEYRING_URL}" -o /tmp/cuda-keyring.deb
dpkg -i /tmp/cuda-keyring.deb
rm /tmp/cuda-keyring.deb

echo "==> Installing CUDA drivers (L4 / Ada Lovelace)"
apt-get update -qq
apt-get install -y --no-install-recommends cuda-drivers

# ── vLLM ──────────────────────────────────────────────────────────────────────
echo "==> Installing vLLM >= 0.12"
pip3 install --upgrade pip
pip3 install "vllm>=0.12"

# ── systemd units ─────────────────────────────────────────────────────────────
echo "==> Installing vllm.service"
install -m 644 /tmp/vllm.service /etc/systemd/system/vllm.service
systemctl enable vllm

# ── Nginx TLS ─────────────────────────────────────────────────────────────────
echo "==> Generating self-signed TLS certificate for Nginx"
mkdir -p /etc/nginx/ssl
openssl req -x509 -nodes -newkey rsa:4096 -days 3650 \
    -keyout /etc/nginx/ssl/yoyo.key \
    -out    /etc/nginx/ssl/yoyo.crt \
    -subj   "/CN=yoyo-tier-b.internal"

echo "==> Installing nginx-yoyo.conf"
install -m 644 /tmp/nginx-yoyo.conf /etc/nginx/conf.d/yoyo.conf
rm -f /etc/nginx/sites-enabled/default
systemctl enable nginx

# ── Data disk mount point ─────────────────────────────────────────────────────
echo "==> Creating /data/weights mount point"
mkdir -p /data/weights

# Startup script: mount the persistent weights disk (device yoyo-weights) on boot.
# The disk is attached by OpenTofu with device_name = "yoyo-weights".
cat > /etc/rc.local << 'EOF'
#!/usr/bin/env bash
# Mount persistent weights disk on boot (attached by OpenTofu).
DEVICE=/dev/disk/by-id/google-yoyo-weights
MOUNTPOINT=/data/weights
if [ -b "${DEVICE}" ] && ! mountpoint -q "${MOUNTPOINT}"; then
    # Format on first boot if no filesystem present.
    if ! blkid "${DEVICE}" > /dev/null 2>&1; then
        mkfs.ext4 -F "${DEVICE}"
    fi
    mount "${DEVICE}" "${MOUNTPOINT}"
fi

# Retrieve bearer token from GCP instance metadata and write it to the Nginx
# map file so the proxy auth is set without baking the secret into the image.
TOKEN=$(curl -sf -H "Metadata-Flavor: Google" \
    "http://metadata.google.internal/computeMetadata/v1/instance/attributes/bearer-token" || true)
if [ -n "${TOKEN}" ]; then
    printf 'map $http_authorization $auth_ok {\n  default 0;\n  "Bearer %s" 1;\n}\n' "${TOKEN}" \
        > /etc/nginx/conf.d/yoyo-auth-map.conf
    systemctl reload nginx || true
fi

exit 0
EOF
chmod +x /etc/rc.local

echo "==> provision.sh complete"
