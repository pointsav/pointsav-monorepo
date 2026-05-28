/*
 * Minimal seL4 rootserver — Phase 1C.c hello-world.
 *
 * Uses SysDebugPutChar (-9) via direct inline assembly on AArch64.
 * seL4 AArch64 syscall convention: svc #0 with x7 = syscall number.
 *
 * SysDebugPutChar requires KernelPrinting=ON (set in Phase 1C.b build).
 * Entry point is _start (the seL4 kernel calls _start for the rootserver).
 */

typedef unsigned long word_t;

static inline void sel4_debug_putchar(char c)
{
    register word_t scno asm("x7") = (word_t)-9; /* SysDebugPutChar */
    register word_t arg0 asm("x0") = (word_t)(unsigned char)c;
    asm volatile(
        "svc #0"
        : "+r"(arg0)
        : "r"(scno)
        : "x1", "x2", "x3", "x4", "x5", "x6", "memory"
    );
}

static inline void sel4_yield(void)
{
    register word_t scno asm("x7") = (word_t)-7; /* SysYield */
    asm volatile(
        "svc #0"
        :
        : "r"(scno)
        : "x0", "x1", "x2", "x3", "x4", "x5", "x6", "memory"
    );
}

static const char message[] = "hello from seL4 rootserver\r\n";

void __attribute__((noreturn)) _start(void)
{
    for (int i = 0; message[i] != '\0'; i++) {
        sel4_debug_putchar(message[i]);
    }
    for (;;) {
        sel4_yield();
    }
}
