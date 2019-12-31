#!/bin/bash

set -e

TOP=$(git rev-parse --show-toplevel)
BIN=$TOP/bin
EXT=$TOP/ext
VER=9b4efa2ede5db24377405a21b218066b90fe2f0e

cd $EXT

if [[ ! -e qemu-system-aarch64 ]]; then
    git clone https://github.com/qemu/qemu

    cd qemu
    git checkout $VER -b cs3210 
    git submodule init
    git submodule update
    
    mkdir -p build
    cd build
    ../configure --disable-capstone --target-list=aarch64-softmmu
    make -j$($BIN/ncpus)
fi
