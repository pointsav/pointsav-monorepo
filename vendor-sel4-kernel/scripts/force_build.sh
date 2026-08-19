#!/bin/bash
echo "[*] Initializing strict script execution to bypass terminal limits..."

cd "${SEL4_KERNEL_DIR:?SEL4_KERNEL_DIR must be set (2026-08-19 GitHub-exposure remediation - no real-value default)}"
rm -rf build
mkdir -p build
cd build

echo "[*] Configuring seL4 for Legacy Emulation (No PCID/FSGSBASE)..."
cmake -G Ninja \
    -DCROSS_COMPILER_PREFIX=x86_64-linux-gnu- \
    -DKernelPlatform=pc99 \
    -DKernelX86PCID=OFF \
    -DKernelFSGSBase=OFF \
    -DKernelOptimisation=-O2 \
    -DKernelVerificationBuild=OFF \
    ../

echo "[*] Compiling the kernel..."
ninja

echo "[+] Kernel compilation complete."
