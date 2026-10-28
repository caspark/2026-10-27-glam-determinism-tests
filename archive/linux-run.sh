#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
cd -- "$(dirname -- "$0")"
rustc -vV
cargo --version
printf 'RUSTFLAGS=%s\nCARGO_ENCODED_RUSTFLAGS=%s\n' "${RUSTFLAGS-}" "${CARGO_ENCODED_RUSTFLAGS-}"
for version in 0.30.10 0.33.12; do
    manifest="v$version/Cargo.toml"
    echo "=== glam $version / Linux default backend ==="
    cargo run --locked --quiet --release --manifest-path "$manifest" --target-dir build
    echo "=== glam $version / Linux scalar-math ==="
    cargo run --locked --quiet --release --manifest-path "$manifest" --target-dir build --features glam/scalar-math
done
