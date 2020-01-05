#!/bin/sh

brew install qemu

rustup target add riscv64gc-unknown-none-elf

dd if=/dev/zero of=hdd.dsk count=32 bs=1m

brew tap riscv/riscv
brew install riscv-tools

install_name_tool -change '/usr/local/opt/isl/lib/libisl.21.dylib' /usr/local/opt/isl/lib/libisl.22.dylib /usr/local/Cellar/riscv-gnu-toolchain/gnu/libexec/gcc/riscv64-unknown-elf/9.2.0/cc1
