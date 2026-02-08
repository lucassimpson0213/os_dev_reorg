#!/usr/bin/env bash
set -euo pipefail

TARGET="$HOME/code/c/os_dev/rust/crates/kernel/i686-os.json"
ZFLAGS="-Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem"

# -----------------------------
# Host (std) crates
# -----------------------------
cargo build -p tools --message-format=short
# If `elf` is a host parser/tooling crate (std), keep it here:
cargo build -p elf --message-format=short

# -----------------------------
# i686 kernel (no_std) crates
# -----------------------------
cargo +nightly build -p kernel --target i686-os $ZFLAGS





