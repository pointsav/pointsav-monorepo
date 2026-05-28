#!/bin/bash
# Build seL4 elfloader for qemu-arm-virt AArch64.
# Phase 1C.c boot procedure — see projects/hello-rootserver/main.c.
#
# Prerequisites:
#   aarch64-linux-gnu-gcc, qemu-system-aarch64
#   vendor-sel4-tools/  (seL4_tools clone: github.com/seL4/seL4_tools)
#   vendor-sel4-kernel/ (seL4 kernel, built with KernelPrinting=ON)
#
# Usage:
#   KERNEL_BUILD=<kernel-build-dir> bash build-elfloader.sh
#   Default KERNEL_BUILD: ../../vendor-sel4-kernel/build/aarch64-qemu
set -e

ARCHIVE_ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
ELFLOADER="$ARCHIVE_ROOT/vendor-sel4-tools/elfloader-tool"
KERNEL_BUILD="${KERNEL_BUILD:-$ARCHIVE_ROOT/vendor-sel4-kernel/build/aarch64-qemu}"
BUILD=/tmp/elfloader-build2
SUPPORT="$(dirname "$0")"

mkdir -p "$BUILD/obj"

INCLUDES="-I$ELFLOADER/include \
  -I$ELFLOADER/include/arch-arm \
  -I$ELFLOADER/include/arch-arm/64 \
  -I$ELFLOADER/include/arch-arm/armv/armv8-a \
  -I$ELFLOADER/include/arch-arm/armv/armv8-a/64 \
  -I$SUPPORT/gen_config \
  -I$SUPPORT/gen_headers \
  -I/tmp/elfloader-build/libcpio \
  -I$KERNEL_BUILD/autoconf \
  -I$KERNEL_BUILD/gen_config"

CFLAGS="-march=armv8-a -D__KERNEL_64__ -ffreestanding -fno-common -fno-pic -fno-pie \
  -mgeneral-regs-only -mstrict-align -D_XOPEN_SOURCE=700"
CC=aarch64-linux-gnu-gcc
N=0

cc_src() {
  N=$((N+1))
  local src="$1"
  local out="$BUILD/obj/$(printf '%03d' $N).o"
  $CC $CFLAGS $INCLUDES -c -o "$out" "$src" 2>&1 && echo "  OK [$N]: $(basename $src)" || { echo "FAIL: $src"; exit 1; }
}

# Build rootserver
aarch64-linux-gnu-gcc \
  -nostdlib -nostartfiles -ffreestanding -static -no-pie \
  -march=armv8-a -O2 \
  -o /tmp/elfloader-build2/rootserver \
  "$ARCHIVE_ROOT/vendor-sel4-project/projects/hello-rootserver/main.c"
echo "  OK: rootserver"

# Generate CPIO archive
python3 "$SUPPORT/gen_cpio.py" \
  "$KERNEL_BUILD/kernel.elf" \
  "$KERNEL_BUILD/kernel.dtb" \
  /tmp/elfloader-build2/rootserver \
  /tmp/elfloader-build/archive.cpio
echo "  OK: archive.cpio"

# Generate archive.S
cat > /tmp/elfloader-build/archive.S << 'ASM'
.section ._archive_cpio,"a"
.globl _archive_start_end
_archive_start_end_minus_archive:
.incbin "/tmp/elfloader-build/archive.cpio"
_archive_start_end:
ASM

cc_src $ELFLOADER/src/common.c
cc_src $ELFLOADER/src/defaults.c
cc_src $ELFLOADER/src/printf.c
cc_src $ELFLOADER/src/string.c
cc_src $ELFLOADER/src/fdt.c
cc_src $ELFLOADER/src/drivers/driver.c
cc_src $ELFLOADER/src/drivers/smp/common.c
cc_src $ELFLOADER/src/drivers/uart/8250-uart.c
cc_src $ELFLOADER/src/drivers/uart/bcm-uart.c
cc_src $ELFLOADER/src/drivers/uart/common.c
cc_src $ELFLOADER/src/drivers/uart/exynos-uart.c
cc_src $ELFLOADER/src/drivers/uart/imx-lpuart.c
cc_src $ELFLOADER/src/drivers/uart/imx-uart.c
cc_src $ELFLOADER/src/drivers/uart/meson-uart.c
cc_src $ELFLOADER/src/drivers/uart/msm-uart.c
cc_src $ELFLOADER/src/drivers/uart/pl011-uart.c
cc_src $ELFLOADER/src/drivers/uart/stm32mp2-uart.c
cc_src $ELFLOADER/src/drivers/uart/xilinx-uart.c
cc_src $ELFLOADER/src/drivers/timer/arm_generic_timer.c
cc_src $ELFLOADER/src/utils/crypt_md5.c
cc_src $ELFLOADER/src/utils/crypt_sha256.c
cc_src $ELFLOADER/src/utils/hash.c
cc_src $ELFLOADER/src/arch-arm/cpuid.c
cc_src $ELFLOADER/src/arch-arm/psci.c
cc_src $ELFLOADER/src/arch-arm/scu.c
cc_src $ELFLOADER/src/arch-arm/smp_boot.c
cc_src $ELFLOADER/src/arch-arm/sys_boot.c
cc_src $ELFLOADER/src/arch-arm/drivers/smp-imx6.c
cc_src $ELFLOADER/src/arch-arm/drivers/smp-psci.c
cc_src $ELFLOADER/src/arch-arm/drivers/smp-spin-table.c
cc_src $ELFLOADER/src/arch-arm/drivers/smp-zynq7000.c
cc_src $ELFLOADER/src/binaries/elf/elf.c
cc_src $ELFLOADER/src/binaries/elf/elf32.c
cc_src $ELFLOADER/src/binaries/elf/elf64.c
cc_src $ELFLOADER/src/arch-arm/64/cpuid.c
cc_src $ELFLOADER/src/arch-arm/64/debug.c
cc_src $ELFLOADER/src/arch-arm/64/mmu.c
cc_src $ELFLOADER/src/arch-arm/64/structures.c
cc_src $ELFLOADER/src/arch-arm/armv/armv8-a/64/smp.c
cc_src $ELFLOADER/src/arch-arm/64/crt0.S
cc_src $ELFLOADER/src/arch-arm/64/traps.S
cc_src $ELFLOADER/src/arch-arm/armv/armv8-a/64/mmu-hyp.S
cc_src $ELFLOADER/src/arch-arm/armv/armv8-a/64/mmu.S
cc_src $ELFLOADER/src/arch-arm/armv/armv8-a/64/psci_asm.S

$CC $CFLAGS $INCLUDES -c -o "$BUILD/obj/$(printf '%03d' $((N+1))).o" /tmp/elfloader-build/libcpio/cpio.c
echo "  OK: libcpio"

$CC -march=armv8-a -c -o "$BUILD/obj/000_archive.o" /tmp/elfloader-build/archive.S
echo "  OK: archive"

echo "=== All compiled ==="

$CC -march=armv8-a -I$SUPPORT/gen_config -I$SUPPORT/gen_headers \
  -I$KERNEL_BUILD/autoconf -I$KERNEL_BUILD/gen_config \
  -P -E -x c $ELFLOADER/src/linker.lds -o "$BUILD/linker.lds_pp"

$CC -nostdlib -static -Wl,--build-id=none \
  -Wl,-T,"$BUILD/linker.lds_pp" \
  -march=armv8-a \
  "$BUILD/obj/"*.o \
  -lgcc \
  -o "$BUILD/elfloader.elf"

echo "=== Linked: $BUILD/elfloader.elf ==="
aarch64-linux-gnu-readelf -h "$BUILD/elfloader.elf" | grep -E "Entry|Type|Machine"

echo ""
echo "To boot:"
echo "  qemu-system-aarch64 -machine virt,secure=off -cpu cortex-a53 -m 1G -nographic -kernel $BUILD/elfloader.elf"
