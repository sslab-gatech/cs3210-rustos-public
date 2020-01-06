#!/usr/bin/env python3

import os
import sys
import subprocess

ROOT = os.path.abspath(os.path.dirname(__file__))
READELF = os.path.join(ROOT, "aarch64-readelf")
CONFIG = """\
arm_control=0x200
kernel_address=0x%x
"""

assert(len(sys.argv) == 2)

def get_entry_point(elf):
    header = subprocess.check_output([READELF, "-h", elf],
                                     universal_newlines=True)
    for l in header.splitlines():
        if "Entry point address:" in l:
            return int(l.strip().split(":")[1], 16)

    raise Exception("Failed to find entry point")

txt = CONFIG % get_entry_point(sys.argv[1])
config = os.path.join(os.path.dirname(sys.argv[1]), "config.txt")

with open(config, "w") as fd:
    fd.write(txt)
