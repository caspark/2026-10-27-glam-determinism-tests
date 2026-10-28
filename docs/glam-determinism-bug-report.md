# Vec4 reductions and Vec3A operations give different results across ARM64, x86-64, and Wasm

I am not sure if cross-platform determinism is an intended guarantee, but `Vec4` reductions and `Vec3A` normalization give different results across ARM64, x86-64, and Wasm. This happens with identical finite input bits, with `libm` enabled.

Minimal repro:

1. Create a separate Cargo project with this `Cargo.toml`:

   ```toml
   [package]
   name = "glam-reduction-repro"
   version = "0.0.0"
   edition = "2021"

   [workspace]

   [dependencies]
   glam = { version = "=0.34.1", features = ["libm"] }
   ```

2. Put this in `src/main.rs`:

   ```rust
   use glam::{Vec3A, Vec4};

   fn main() {
       let v = std::hint::black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
       println!("element_sum: {}", v.element_sum());
       println!("dot(ONE): {}", v.dot(Vec4::ONE));

       let v3 = std::hint::black_box(Vec3A::new(1.0, 1.0, 3.0));
       println!("normalize.z: {}", v3.normalize().z);

       let a = std::hint::black_box(Vec3A::new(0.0, 16785408.0, 4097.0));
       let b = std::hint::black_box(Vec3A::new(0.0, 4097.0, 1.0));
       println!("cross.x: {}", a.cross(b).x);
   }
   ```

3. Run these commands on each native platform:

   ```sh
   cargo run --release
   cargo run --release --features glam/scalar-math
   ```

4. With Wasmtime and the `wasm32-wasip1` target installed, run:

   ```sh
   cargo build --release --target wasm32-wasip1
   wasmtime target/wasm32-wasip1/release/glam-reduction-repro.wasm
   ```

**Observed on glam 0.34.1:**

| Build | `element_sum()` | `dot(Vec4::ONE)` | `normalize.z` | `cross.x` |
| --- | --- | --- | --- | --- |
| ARM64 macOS, NEON backend | 0 | 0 | 0.90453404 | -1 |
| x86-64 Windows, SSE2 backend | 0 | 2 | 0.904534 | 0 |
| x86-64 Linux, SSE2 backend | 0 | 2 | 0.904534 | 0 |
| `wasm32-wasip1`, scalar backend | 1 | 1 | 0.90453404 | 0 |
| `wasm32-wasip1`, `+simd128` | 0 | 2 | 0.904534 | 0 |
| All four targets, `scalar-math` | 1 | 1 | 0.90453404 | 0 |

On x86-64, `v.dot(Vec4::ONE)` also differs from `v.element_sum()`.

**Expected (for deterministic simulations):** the same result bits across backends for a given method and input. The choice of reduction order does not matter.

**Cause:** the input is `[A, 1, -A, 1]`, where `A` is approximately `1e20`. At this magnitude, `A + 1` rounds back to `A`.

The NEON backend uses `vaddvq_f32`, which adds pairs first. SSE2 uses different pairings for the sum and dot product. The scalar backend adds from left to right:

```text
NEON sum/dot, SSE2 sum: (A + 1) + (-A + 1) = 0
SSE2 dot:               (A + -A) + (1 + 1) = 2
Scalar sum/dot:         ((A + 1) + -A) + 1 = 1
```

This is a difference in the operation order, not a disagreement between platforms about individual additions.

`Vec3A::normalize()` has a separate difference: SSE2/Wasm SIMD divide by the length, while NEON/scalar multiply by its reciprocal.

`Vec3A::cross()` fuses one multiply-subtract on NEON. The baseline x86 and Wasm backends use separate multiply and subtract operations.

**Workaround:** the `scalar-math` feature makes the results in these examples match across targets, but disables glam's explicit SIMD implementations.

Notes:
* All examples reproduced on 0.34.1. The Vec4 sum/dot repro also fails on 0.30.10 and 0.33.12.
* The `fast-math` feature no longer exists in 0.34.1.
* Further probes found differences in element products, monotonic interpolation, matrix inverses, and quaternion multiplication/interpolation.
* The builds used Rust 1.96.1 on macOS/Wasm, 1.95.0 on Windows, and 1.94.0 on Linux.
* Wasmtime 44.0.1 ran the Wasm builds.
* Found during a determinism audit for my game [Signs of Danger](https://www.slowrush.dev/), which uses rollback netcode.

Is `scalar-math` the intended requirement for cross-platform determinism, or can SIMD backends use the same operations and reduction order?
