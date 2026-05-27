/*
 * hello.c — Phase 1C minimal bare-metal AArch64 seL4 PD stub.
 *
 * Compiled with:
 *   aarch64-linux-gnu-gcc -nostdlib -nostartfiles -ffreestanding
 *     -march=armv8-a -mgeneral-regs-only hello.c -o build/hello.elf
 *
 * No libc, no crt0. _start is the ELF entry point.
 *
 * Phase 1C.d (follow-on): wire to Microkit libmicrokit (void init(void)
 * + void notified(microkit_channel)) once the Rust image assembler or
 * Microkit SDK is available.
 */

void _start(void) {
    /* Halt. A real Microkit PD would call init() then enter the
     * event loop. The UART write + seL4_DebugPutChar path lands
     * in Phase 1C.d once the image assembler is in place. */
    for (;;) {}
}
