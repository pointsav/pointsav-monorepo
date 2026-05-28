#pragma once
#include <stddef.h>
/*
 * Minimal libcpio compatibility header for seL4 elfloader.
 * Implements CPIO "newc" (070701) format parsing.
 */

typedef unsigned long cpio_file_size_t;

/* Return a pointer to the data of the named file, or NULL if not found. */
void const *cpio_get_file(void const *archive, size_t len,
                          char const *name, cpio_file_size_t *size_out);

/* Return a pointer to the data of the Nth file (0-indexed), setting *name_out
 * and *size_out if non-NULL. Returns NULL if index is out of range. */
void const *cpio_get_entry(void const *archive, size_t len,
                           unsigned int index,
                           char const **name_out,
                           cpio_file_size_t *size_out);
