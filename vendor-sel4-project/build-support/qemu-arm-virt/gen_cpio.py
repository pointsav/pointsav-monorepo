#!/usr/bin/env python3
"""
Generate a CPIO "newc" (070701) archive for the seL4 elfloader.

Usage:
  python3 gen_cpio.py kernel.elf kernel.dtb rootserver output.cpio

The archive contains three entries named exactly "kernel.elf", "kernel.dtb",
"rootserver" — the names the seL4 elfloader searches for at boot time.

Padding: header+name is padded to 4 bytes; file data is padded to 4 bytes.
This matches the cpio.c parser used in the manual elfloader build.
"""
import sys
import os


def align4(n):
    return (n + 3) & ~3


def make_entry(name, data):
    name_bytes = name.encode() + b'\x00'
    namesize = len(name_bytes)
    filesize = len(data)

    hdr = (
        b'070701'
        + b'00000000'             # ino
        + b'000081a4'             # mode (regular file, 0644)
        + b'00000000'             # uid
        + b'00000000'             # gid
        + b'00000001'             # nlink
        + b'00000000'             # mtime
        + f'{filesize:08x}'.encode()
        + b'00000008'             # devmajor
        + b'00000001'             # devminor
        + b'00000000'             # rdevmajor
        + b'00000000'             # rdevminor
        + f'{namesize:08x}'.encode()
        + b'00000000'             # check
    )
    assert len(hdr) == 110

    name_pad = b'\x00' * (align4(110 + namesize) - 110 - namesize)
    data_pad = b'\x00' * (align4(filesize) - filesize)
    return hdr + name_bytes + name_pad + data + data_pad


def main():
    if len(sys.argv) != 5:
        print(f'Usage: {sys.argv[0]} kernel.elf kernel.dtb rootserver output.cpio')
        sys.exit(1)

    kernel_elf, kernel_dtb, rootserver, output = sys.argv[1:]

    with open(kernel_elf, 'rb') as f:
        elf_data = f.read()
    with open(kernel_dtb, 'rb') as f:
        dtb_data = f.read()
    with open(rootserver, 'rb') as f:
        rs_data = f.read()

    archive = (
        make_entry('kernel.elf', elf_data)
        + make_entry('kernel.dtb', dtb_data)
        + make_entry('rootserver', rs_data)
        + make_entry('TRAILER!!!', b'')
    )

    with open(output, 'wb') as f:
        f.write(archive)

    print(f'Archive: {output} ({len(archive)} bytes)')
    print(f'  kernel.elf: {len(elf_data)} bytes')
    print(f'  kernel.dtb: {len(dtb_data)} bytes')
    print(f'  rootserver: {len(rs_data)} bytes')


if __name__ == '__main__':
    main()
