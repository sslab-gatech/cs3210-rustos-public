#!/bin/bash -e

IMG=fs.img
MNT=mnt

PROGS=(sleep fib echo)

for d in ${PROGS[@]}; do
    (cd $d; make build)
done

dd if=/dev/zero of=$IMG bs=1MB count=128
echo -e "n\np\n1\n\n\nt\nc\nw\n" | fdisk $IMG

LO=$(sudo losetup --show -f -P $IMG)
LOP1=${LO}p1

if [ ! -e $LOP1 ]; then
    PARTITIONS=$(lsblk --raw --output "MAJ:MIN" --noheadings ${LO} | tail -n +2)COUNTER=1
    COUNTER=1
    for i in $PARTITIONS; do
        MAJ=$(echo $i | cut -d: -f1)
        MIN=$(echo $i | cut -d: -f2)
        if [ ! -e "${LO}p${COUNTER}" ]; then sudo mknod ${LO}p${COUNTER} b $MAJ $MIN; fi
        COUNTER=$((COUNTER + 1))
    done
fi

sudo mkfs.vfat -F32 $LOP1

mkdir -p $MNT
sudo mount $LOP1 $MNT

trap "sudo umount $MNT; rmdir $MNT; sudo losetup -d $LO" EXIT

for d in ${PROGS[@]}; do
    sudo cp $d/build/$d.bin $MNT/$d
done
