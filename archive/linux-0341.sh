#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
cd -- "$(dirname -- "$0")"
rustc -vV
CARGO_HTTP_TIMEOUT=30 CARGO_NET_RETRY=1 cargo fetch --locked --manifest-path v0.34.1/Cargo.toml
printf '\n=== Linux 0.34.1 default ===\n'
cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build | tee linux-0341-default.txt
printf '\n=== Linux 0.34.1 scalar ===\n'
cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build --features glam/scalar-math | tee linux-0341-scalar.txt
if [[ "$(<linux-0341-default.txt)" == *runtime_fma=true* ]]; then
    printf '\n=== Linux 0.34.1 +fma ===\n'
    RUSTFLAGS='-C target-feature=+fma' cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build | tee linux-0341-fma.txt
    printf '\n=== Linux 0.34.1 fma-scalar ===\n'
    RUSTFLAGS='-C target-feature=+fma' cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build --features glam/scalar-math | tee linux-0341-fma-scalar.txt
fi
