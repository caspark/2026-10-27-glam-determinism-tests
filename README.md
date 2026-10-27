# Glam determinism reproductions

Standalone Rust probes for differences between glam's ARM64 NEON, x86-64 SSE2, Wasm SIMD, and scalar implementations.

The manifests pin glam 0.30.10, 0.33.12, and 0.34.1. Each manifest has its own workspace to prevent feature unification between experiments.

The repository selects Rust 1.96 and the WASI target through `rust-toolchain.toml`. Use `cargo +<version>` to compare other toolchains. The recorded captures retain their original toolchain versions.

## Contents

- [Bug report](glam-determinism-bug-report.md): decimal-input examples for Vec4 reductions and Vec3A operations
- [Source audit](glam-determinism-audit-0.34.1.md): source paths, observed differences, and limits
- `main.rs`: original Vec4 sum/dot repro for 0.30.10 and 0.33.12
- `report.rs`: minimal 0.34.1 repro from the bug report
- `audit.rs`: fixed probes and 64 generated matrix/quaternion cases for 0.34.1
- `zero.rs`: plain Rust and vector controls for signed-zero selection
- `vector-safe-verification/`: standalone snapshot of zero-safe helpers and their verification program, using glam 0.30.10
- `*.txt`, `results-0341.json`: recorded output from the original runs

The verification snapshot contains the helper source, its canonicalization functions, and the game's Clippy configuration. It does not depend on the game repository.

## Minimal repro

From this directory, run:

```sh
cargo run --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build --bin glam-report-repro
cargo run --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build --bin glam-report-repro --features glam/scalar-math
```

The baseline assumes that `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` are unset.

## Full audit and zero controls

```sh
cargo run --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build
cargo run --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build --features glam/scalar-math
cargo run --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build --bin glam-zero-probe --features glam/scalar-math
```

For the older Vec4 repro, use `v0.30.10/Cargo.toml` or `v0.33.12/Cargo.toml`.

## Wasm

Install the Rust target:

```sh
rustup target add wasm32-wasip1
```

With Wasmtime installed, run:

```sh
cargo build --locked --release --manifest-path v0.34.1/Cargo.toml --target-dir build --bin glam-report-repro --target wasm32-wasip1
wasmtime build/wasm32-wasip1/release/glam-report-repro.wasm
```

The captured audit also includes Wasm builds with `-C target-feature=+simd128`.

## Native platform runners

On Linux, run:

```sh
bash linux-0341.sh
```

On Windows, run:

```powershell
powershell.exe -NoProfile -NonInteractive -File .\windows-0341.ps1
```

These scripts run the full 0.34.1 audit. They also run explicit x86 FMA experiments when the CPU supports FMA. The bug report excludes those experiments.

`linux-run.sh` and `windows-run.ps1` run the older Vec4 repros.

## Zero-safe helper verification

```sh
cargo run --locked --release --manifest-path vector-safe-verification/Cargo.toml --target-dir build
cargo run --locked --release --manifest-path vector-safe-verification/Cargo.toml --target-dir build --bin zero-selection
```

The first command performs 476 checks for positive-zero results and unchanged finite values. It passed on macOS, Windows, Linux, and Wasm.

The second command reproduces the unguarded constant-zero selection problem.

This lint command is expected to fail on the intentionally raw reference calls:

```sh
cargo clippy --manifest-path vector-safe-verification/Cargo.toml --target-dir build --bin vector-safe-verification --no-deps -- -A clippy::all -D clippy::disallowed_methods
```

The original lint run confirmed that all ten Vec2/Vec3 method bans reject raw calls.

## Recorded results

The original 0.34.1 audit covers 14 build configurations and 444 records per configuration. The source audit identifies the targets and toolchain versions.

The recorded files are historical captures. A local run does not update `results-0341.json` automatically.

Build outputs, downloaded crate archives, and extracted upstream source are excluded from Git.
