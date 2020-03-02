#!/bin/bash -e

IMG=fs.img
MNT=mnt

PROGS=(sleep fib)

for d in ${PROGS[@]}; do
    (cd $d; make build)
done

dd if=/dev/zero of=$IMG bs=1MB count=128
echo -e "n\np\n1\n\n\nt\nc\nw\n" | fdisk $IMG

LO=$(sudo losetup --show -f -P $IMG)
LOP1=${LO}p1

if [ ! -e $LOP1 ]; then
    echo "[!] can't find the partition in $IMG"
    sudo losetup -d $LO
fi

sudo mkfs.vfat -F32 $LOP1

mkdir -p $MNT
sudo mount $LOP1 $MNT

trap "sudo umount $MNT; rmdir $MNT; sudo losetup -d $LO" EXIT

for d in ${PROGS[@]}; do
    sudo cp $d/build/$d.bin $MNT/$d
done
