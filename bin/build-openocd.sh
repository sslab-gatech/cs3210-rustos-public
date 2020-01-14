#!/bin/bash

set -e

TOP=$(git rev-parse --show-toplevel)
EXT=$TOP/ext

cd $EXT

sudo apt install -y libtool autotools-dev automake autoconf libusb-1.0-0-dev
git clone https://git.code.sf.net/p/openocd/code openocd
cd openocd
./bootstrap
./configure
make
sudo make install 
