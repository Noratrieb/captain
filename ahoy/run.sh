#!/usr/bin/env bash

exec bwrap \
    --bind ./ahoy/rootfs / \
    --proc /proc \
    --dev /dev \
    --unshare-user \
    --unshare-ipc \
    --unshare-pid \
    --unshare-uts \
    --unshare-cgroup \
    --as-pid-1 \
    /bin/quarterdeck
