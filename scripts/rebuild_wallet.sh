#!/usr/bin/env bash
set -euo pipefail
export RUST_BACKTRACE=full
export RUST_LOG="info,nockchain=debug,nockchain_libp2p_io=info,libp2p=info,libp2p_quic=info"
export MINIMAL_LOG_FORMAT=true
cd vendor/nockchain
export MINING_PUBKEY="EHmKL2U3vXfS5GYAY5aVnGdukfDWwvkQPCZXnjvZVShsSQi3UAuA4tQQpVwGJMzc9FfpTY8pLDkqhBGfWutiF4prrCktUH9oAWJxkXQBzAavKDc95NR3DjmYwnnw8GuugnK"
die() { echo "❌  $*" >&2; exit 1; }

need_cmd() { command -v "$1" >/dev/null 2>&1 || die "Missing tool: $1"; }
need_cmd cargo
need_cmd git
mkdir -p hoon assets
echo "🔧  Installing/Updating choo…"
cargo install --locked --force --path crates/nockapp/apps/choo --bin choo
echo "🦉  Compiling Hoon wallet kernel (wal.jam)…"
rm -f out.jam assets/wal.jam
RUST_LOG=trace choo hoon/apps/wallet/wallet.hoon hoon
mv out.jam assets/wal.jam
echo "✅  Hoon kernel written to assets/wal.jam"
BIN_NAME="wallet"
echo "🔨  Building Rust binary (${BIN_NAME})…"
cargo build --release --bin "${BIN_NAME}"
echo "✅  Rust binary built: target/release/${BIN_NAME}"
echo -e "\n🎉  Wallet build complete."
echo    "    • Kernel: assets/wal.jam"
echo    "    • Binary: target/release/${BIN_NAME}"
