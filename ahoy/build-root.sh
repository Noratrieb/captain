#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

target_tuple="$(uname -m)-unknown-linux-musl"

cargo build -p quarterdeck --target "$target_tuple"
cargo build -p cog --target "$target_tuple"
cargo build -p coreutils --target "$target_tuple"

target_dir="../target/$target_tuple/debug"

rm -rf rootfs

mkdir -p rootfs
mkdir -p rootfs/bin

install_bin() {
    cp "$target_dir/$1" rootfs/bin
}

install_bin quarterdeck
install_bin cog
install_bin net
install_bin ls

mkdir -p rootfs/etc
cp /etc/resolv.conf rootfs/etc/resolv.conf
