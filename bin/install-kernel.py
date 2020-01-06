#!/usr/bin/env python3

import os
import sys
import glob
import hashlib
import subprocess
import shutil
import time

ROOT = os.path.abspath(os.path.dirname(__file__))
CACHE = "/tmp/.cs3210-sdcard-dir.txt"

def load_target_dir():
    global CACHE
    
    dst = None
    if os.path.exists(CACHE):
        dst = open(CACHE).read().strip()
    if len(sys.argv) == 3:
        dst = sys.argv[2]

    if dst is None:
        os.system("lsblk")
        print("[!] Please provide a installation directory")
        dst = input("(input) > ").strip()

    if not os.path.isdir(dst):
        print("[!] Please inesrt your sdcard (mounting point: %s)" % dst)
        print("    waiting", end="", flush=True)

        while not os.path.isdir(dst):
            print(".", end="", flush=True)
            time.sleep(1)
        print()

    with open(CACHE, "w") as fd:
        fd.write(dst)

    return dst

def load_firmware():
    rtn = []
    for pn in glob.glob("%s/../ext/firmware/*" % ROOT):
        rtn.append((os.path.abspath(pn), os.path.basename(pn)))
    return rtn

def load_kernel():
    assert(len(sys.argv) >= 2)

    pn = os.path.abspath(sys.argv[1])
    if not os.path.exists(pn):
        print("[!] %s doesn't exist" % pn)
        exit(1)

    (name, ext) = os.path.splitext(pn)
    assert(ext in [".bin", ".elf", ".img"])

    for ext in [".bin", ".img"]:
        pn = name + ext
        if os.path.exists(pn):
            return pn

    raise Exception("Please dump the code from elf first!")

def build_config(kernel):
    (name, ext) = os.path.splitext(kernel)
    assert(ext in [".bin", ".elf", ".img"])

    config = "%s/config.txt" % os.path.dirname(kernel)

    # already have config.txt, like ext/rpi*
    if ext == ".img" and os.path.exists(config):
        return config

    kernel = name + ".elf"
    subprocess.check_call(["%s/gen-rpi3-config.py" % ROOT,
                           kernel],
                          universal_newlines=True)
    return config

def md5sum(pn):
    if not os.path.exists(pn):
        return ""
    md5 = hashlib.new("md5")
    md5.update(open(pn, "rb").read())
    return md5.hexdigest()

def copy_to(src, name, dst):
    assert(os.path.isfile(src))
    assert(os.path.isdir(dst))

    dst = os.path.join(dst, name)

    hash_dst = md5sum(dst)
    hash_src = md5sum(src)

    if hash_dst != hash_src:
        bak = "%s~" % dst
        if hash_dst != "":
            if os.path.exists(bak):
                os.unlink(bak)
            shutil.move(dst, bak)
        print("[!] %s is updated" % dst)
        shutil.copy2(src, dst)
    else:
        print("[!] %s is up-to-date" % dst)

if __name__ == '__main__':
    if len(sys.argv) == 1:
        print("[!] %s [kernel.{bin|elf}] [sdcard directory]?")
        print(" NOTE. if the sdcard directory is not provided,")
        print("       we will select the directory previously used")
        exit(1)
    
    assert(len(sys.argv) <= 3)

    kernel = load_kernel()
    sdcard = load_target_dir()
    config = build_config(kernel)
    
    for f in load_firmware() \
        + [(kernel, "kernel8.img"),
           (config, "config.txt")]:
        copy_to(*f, sdcard)

    print("[!] unmounting %s" % sdcard)
    os.system("sudo umount '%s'" % sdcard)