#!/bin/bash
echo "[*] Initializing strict script execution to bypass terminal limits..."

cd /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/vendor-sel4-kernel/
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
