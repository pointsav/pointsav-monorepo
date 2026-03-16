#!/bin/bash
set -euo pipefail

echo "========================================================"
echo " 🧠 VENDOR ACQUISITION: PHI-3-MINI-4K-INSTRUCT"
echo "========================================================"

WEIGHTS_DIR="$(pwd)/weights"
mkdir -p "$WEIGHTS_DIR"

echo "[SYSTEM] Acquiring Immutable Execution Engine (Mozilla Llamafile v0.8.14)..."
if [ ! -f "llamafile" ]; then
    wget -q --show-progress https://github.com/Mozilla-Ocho/llamafile/releases/download/0.8.14/llamafile-0.8.14 -O llamafile
    chmod +x llamafile
else
    echo "  -> Engine already exists."
fi

echo "[SYSTEM] Acquiring Mathematical Weights (Microsoft Phi-3-Mini 4K Instruct GGUF)..."
if [ ! -f "$WEIGHTS_DIR/phi-3-mini-4k-instruct-q4.gguf" ]; then
    wget -q --show-progress https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf -O "$WEIGHTS_DIR/phi-3-mini-4k-instruct-q4.gguf"
else
    echo "  -> Weights already exist."
fi

echo "[SUCCESS] Vendor payloads successfully acquired to local SSD."
