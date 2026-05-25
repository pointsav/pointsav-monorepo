#!/bin/bash
set -euo pipefail

echo "========================================================"
echo " 🟢 IGNITING SOVEREIGN SLM SERVER (LOCALHOST:8080)"
echo "========================================================"
echo "[SYSTEM] The server will lock the terminal. Press Ctrl+C to terminate."
echo "--------------------------------------------------------"

./llamafile -m weights/phi-3-mini-4k-instruct-q4.gguf --server --host 127.0.0.1 --port 8080 --nobrowser -c 4096
