#!/bin/bash

set -e

TOP=$(git rev-parse --show-toplevel)
FAT=$TOP/ext/fat32-imgs

cd $FAT

tar -xvzf mock1.fat32.tar.gz
tar -xvzf mock2.fat32.tar.gz
tar -xvzf mock3.fat32.tar.gz
tar -xvzf mock4.fat32.tar.gz
