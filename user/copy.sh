#!/bin/bash -e

IMG=fs.img
MNT=mnt

PROGS=(sleep fib echo)

if [ -z "$CS3210_COPY" ]; then
    echo "[!] please set CS3210_COPY environment variable"
    exit 1
fi

for d in ${PROGS[@]}; do
    (cd $d; make build)
done

(cd ../kern5; make)

for d in ${PROGS[@]}; do
    cp $d/build/$d.bin $CS3210_COPY/$d
done

cp ../kern5/build/kernel.bin $CS3210_COPY/kernel.bin 
