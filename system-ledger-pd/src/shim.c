/*
 * shim.c — expose Microkit inline/macro functions as linkable C symbols.
 *
 * microkit.h defines microkit_dbg_puts as an inline function that Rust's
 * `extern "C"` declarations cannot reach directly. This file wraps it so
 * the linker can resolve the symbol from the Rust staticlib.
 *
 * Only the functions used by system-ledger-pd are exposed here.
 * system-ledger-pd is the server side of the PPC protocol; it does not
 * call microkit_ppcall or microkit_notify — those are client-side only.
 */

#include <microkit.h>

void microkit_dbg_puts(const char *s) {
    microkit_dbg_puts(s);
}
