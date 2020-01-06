#!/bin/sh
rustup show active-toolchain | cut -d- -f5- | cut -d' ' -f1