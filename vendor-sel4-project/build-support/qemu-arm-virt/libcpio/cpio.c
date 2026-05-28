/*
 * Minimal CPIO "newc" (070701) parser for seL4 elfloader.
 */
#include <cpio/cpio.h>

#define CPIO_HDR_SIZE 110

static unsigned long parse_hex8(const char *s)
{
    unsigned long v = 0;
    for (int i = 0; i < 8; i++) {
        char c = s[i];
        v <<= 4;
        if (c >= '0' && c <= '9') v |= (unsigned long)(c - '0');
        else if (c >= 'a' && c <= 'f') v |= (unsigned long)(c - 'a' + 10);
        else if (c >= 'A' && c <= 'F') v |= (unsigned long)(c - 'A' + 10);
    }
    return v;
}

static int str_eq(const char *a, const char *b)
{
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}

#define ALIGN4(x) (((size_t)(x) + 3u) & ~3u)

static void const *cpio_next(const char *p, const char *end,
                              const char **name_out,
                              unsigned long *filesize_out)
{
    if ((size_t)(end - p) < CPIO_HDR_SIZE) return (void *)0;
    if (p[0]!='0'||p[1]!='7'||p[2]!='0'||p[3]!='7'||p[4]!='0'||p[5]!='1')
        return (void *)0;
    unsigned long namesize = parse_hex8(p + 94);
    unsigned long filesize = parse_hex8(p + 54);
    const char *name = p + CPIO_HDR_SIZE;
    if ((size_t)(end - name) < namesize) return (void *)0;
    if (name_out) *name_out = name;
    if (filesize_out) *filesize_out = filesize;
    return (void const *)(p + ALIGN4(CPIO_HDR_SIZE + namesize));
}

static const char *cpio_advance(const char *p, const char *end)
{
    const char *name;
    unsigned long filesize;
    void const *data = cpio_next(p, end, &name, &filesize);
    if (!data) return end;
    return (const char *)data + ALIGN4(filesize);
}

void const *cpio_get_file(void const *archive, size_t len,
                          char const *fname, cpio_file_size_t *size_out)
{
    const char *p = (const char *)archive;
    const char *end = p + len;
    while (p < end) {
        const char *name;
        unsigned long filesize;
        void const *data = cpio_next(p, end, &name, &filesize);
        if (!data) return (void *)0;
        if (str_eq(name, "TRAILER!!!")) return (void *)0;
        if (str_eq(name, fname)) {
            if (size_out) *size_out = (cpio_file_size_t)filesize;
            return data;
        }
        p = cpio_advance(p, end);
    }
    return (void *)0;
}

void const *cpio_get_entry(void const *archive, size_t len,
                           unsigned int index,
                           char const **name_out,
                           cpio_file_size_t *size_out)
{
    const char *p = (const char *)archive;
    const char *end = p + len;
    unsigned int i = 0;
    while (p < end) {
        const char *name;
        unsigned long filesize;
        void const *data = cpio_next(p, end, &name, &filesize);
        if (!data) return (void *)0;
        if (str_eq(name, "TRAILER!!!")) return (void *)0;
        if (i == index) {
            if (name_out) *name_out = name;
            if (size_out) *size_out = (cpio_file_size_t)filesize;
            return data;
        }
        p = cpio_advance(p, end);
        i++;
    }
    return (void *)0;
}
