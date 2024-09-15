#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

target_tuple="$(uname -m)-unknown-linux-musl"

cargo build --target "$target_tuple" \
    -p quarterdeck -p cog -p coreutils -p tcpecho

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
install_bin user
install_bin tcpecho

mkdir -p rootfs/etc
cp /etc/resolv.conf rootfs/etc/resolv.conf

mkdir -p rootfs/etc/services

cat > rootfs/etc/services/tcpecho2000.toml <<EOF
name = "tcpecho2000"
exec = [ "/bin/tcpecho", "0.0.0.0:2000" ]
EOF
