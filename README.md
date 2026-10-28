# Glam determinism reproducers

Small examples of different result bits across glam's backends. The main suite uses **glam 0.34.1 with `libm` enabled**.

Each example has fixed decimal inputs and named hexadecimal results. Short arithmetic controls sit beside the failing call. The runner also rebuilds each example with `scalar-math`.

## Run

Install [Rust with rustup](https://rustup.rs/) and the [Wasmtime CLI](https://docs.wasmtime.dev/cli-install.html). The toolchain file selects Rust 1.96 and `wasm32-wasip1`.

```sh
cargo run --release                                      # All examples, both feature configurations
cargo run --release -- compare zero_selection            # One example, both configurations
cargo run --release -- compare zero_selection --scalar-math # Only the scalar-math configuration
cargo run --release -- native quaternion_slerp           # Native results only
cargo run --release -- wasm quaternion_slerp             # Wasm scalar results only
cargo run --release -- wasm-simd quaternion_slerp        # Wasm SIMD results only

# Standalone examples do not require Wasmtime:
cargo run --release --example zero_selection
cargo run --release --example zero_selection --features scalar-math
```

`compare` prints native, Wasm scalar, and Wasm SIMD results in adjacent columns. `DIFF` marks unequal bits. The first configuration uses `libm`; the second adds `scalar-math`.

Exit codes: **0** means equal results or a successful single-target run, **1** means compared results differ, and **2** means a build/runtime error. Differences are expected. Some cases differ only between native architectures, so one host cannot expose every difference.

The runner controls target flags and clears inherited `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, and `CARGO_BUILD_TARGET` for child builds. Standalone Cargo commands retain your flags.

## Divergence with both `libm` and `scalar-math`

The results here use both features. The builds use Rust 1.96.1 on macOS ARM64, Linux x86-64, and Windows x86-64 (MSVC). Wasmtime 44.0.1 runs the Wasm builds. Both Wasm configurations produce identical results across all three hosts.

Results are hexadecimal `f32` bits unless stated otherwise. `80000000` is -0; `00000000` is +0. Equal bits do not establish numerical accuracy.

Results are grouped by identical output. Each group lists the builds that produce that output. Groups do not imply numerical correctness.

### [Signed-zero selection: Rust, glam, and guarded results](examples/zero_selection.rs)

`if x < 0.0 { x } else { 0.0 }` differs across targets for `x = -0.0`. Glam's comparison-based selection can produce the same symptom. This is not a glam-only defect, and the example does not establish one compiler root cause.

The program starts with plain Rust, then exercises Vec2, Vec3, Vec3A, Vec4, DVec2, DVec3, and DVec4. Every operation prints adjacent `_raw` and `_safe` results. It asserts that the guarded results are +0 and that selected nonzero results stay unchanged.

The measured cases include:

- `min`: negative zero against a constant positive zero.
- `max`: positive zero against a constant negative zero; the Vec4 and DVec4 forms diverge.
- `clamp`: zero-valued upper and lower bounds.
- `min_element`: one opaque negative-zero component and constant positive-zero components.
- `select`: the vector equivalent of the plain Rust comparison.

Reported results: `Vec4_min_raw / Vec4_max_raw`.

**DIVERGES: 2 result groups**

- `80000000 / 00000000`: macOS ARM64
- `00000000 / 80000000`: Linux x86-64, Windows x86-64, Wasm scalar, Wasm SIMD target with `scalar-math`

The plain Rust selection, `clamp_upper`, `min_element`, and comparison-driven `select` witnesses follow the `min` split. Not every type or operand arrangement diverges. The DVec results use the corresponding 64-bit zero patterns.

With both features, the selected `max_element`, `saturate`, and `smoothstep` controls match. The same example also includes their raw and guarded results. The [SIMD-only selection results](#additional-simd-zero-selection) differ without `scalar-math`.

**Workaround: canonicalize after selection.** [`src/workarounds.rs`](src/workarounds.rs) provides scalar and vector min/max/clamp and extrema-reduction wrappers. The scalar and f32-vector helpers adapt the game's API. DVec support extends the same policy for this repo; it does not change the game.

```rust
use glam::Vec2;
use glam_determinism_repros::workarounds::VectorFloatSafeExt;

let v = std::hint::black_box(Vec2::splat(-0.0));
assert_eq!(v.min_safe(Vec2::ZERO).x.to_bits(), 0);
```

For `select`, the guarded result uses zero canonicalization directly. For `saturate`, it uses `clamp_safe(ZERO, ONE)`. All guarded results are +0 on every tested build, including the f64 cases. This policy deliberately erases zero signs; it does not normalize NaN payloads or fix other arithmetic differences.

## Divergence with `libm`, without `scalar-math`

These results use the default NEON/SSE2 backends and both Wasm backends. Adding `scalar-math` makes the reported results match across the tested builds. It does not guarantee agreement for every API or input.

The fixed-order controls select one evaluation order. They demonstrate possible fixes, not complete replacements for the library methods.

### [Vec4 sum and dot](examples/vec4_reductions.rs)

Different addition orders produce three answers for `(1e20, 1, -1e20, 1)`. Workaround: `scalar-math`, or a common reduction order.

Reported results: `sum / dot`.

**DIVERGES: 3 result groups**

- `00000000 / 00000000`: macOS ARM64
- `00000000 / 40000000`: Linux x86-64, Windows x86-64, Wasm SIMD
- `3f800000 / 3f800000`: Wasm scalar

The fixed-order control returns `3f800000` on every build.

### [Vec4 element product](examples/vec4_product.rs)

Paired multiplication differs from left-to-right multiplication for `(1.1, 1.1, 1.1, 1.3)`. Workaround: `scalar-math`, or a common multiplication order.

Reported result: `product`.

**DIVERGES: 2 result groups**

- `3fdd7a79`: macOS ARM64, Wasm scalar
- `3fdd7a78`: Linux x86-64, Windows x86-64, Wasm SIMD

### [Vec4 length](examples/vec4_length.rs)

The reduction order changes squared length and length for `(1, 1, 1, 4096)`. Workaround: `scalar-math`, or a fixed-order squared-length reduction.

Reported result: `length`.

**DIVERGES: 2 result groups**

- `45800000`: macOS ARM64, Linux x86-64, Windows x86-64, Wasm SIMD
- `45800001`: Wasm scalar

### [Normalization](examples/normalization.rs)

Division differs from reciprocal/multiply for Vec3A `(1, 1, 3)` and Vec4 `(1, 1, 3, 0)`. Workaround: `scalar-math`.

Reported result: the z component for either vector.

**DIVERGES: 2 result groups**

- `3f678f8b`: macOS ARM64, Wasm scalar
- `3f678f8a`: Linux x86-64, Windows x86-64, Wasm SIMD

The reciprocal controls return `3f678f8b`. `normalize_or_zero` is only a control for the selected Vec3A input; it also changes zero-length behavior.

### [Cross product](examples/cross_product.rs)

Fused multiply-subtract differs from separate products and subtraction. Workaround: `scalar-math`, or one arithmetic policy.

Reported result: `cross_x`.

**DIVERGES: 2 result groups**

- `bf800000`: macOS ARM64
- `00000000`: Linux x86-64, Windows x86-64, Wasm scalar, Wasm SIMD

The separate and explicit fused controls each agree across builds. They intentionally give different answers.

### [Monotonic interpolation](examples/monotonic_lerp.rs)

`lerp_monotonic` explicitly permits hardware-dependent fusion. Workaround: `scalar-math`, or explicit fused/separate arithmetic. This is documented behavior, not a demonstrated contract violation.

Reported result: `vec3a`; the Vec4 result matches it.

**DIVERGES: 2 result groups**

- `a8800000`: macOS ARM64
- `00000000`: Linux x86-64, Windows x86-64, Wasm scalar, Wasm SIMD

### [Mat3A inverse availability](examples/mat3a_inverse.rs)

Cross-product rounding changes the determinant of this ill-conditioned matrix. Workaround: `scalar-math`, which makes every build return `None`. That result is consistent, not more accurate.

Reported result: `try_inverse()` availability, not float bits.

**DIVERGES: 2 result groups**

- `Some`: macOS ARM64
- `None`: Linux x86-64, Windows x86-64, Wasm scalar, Wasm SIMD

The program encodes this result as `00000001` for `Some` and `00000000` for `None`.

### [Mat4 inverse](examples/mat4_inverse.rs)

The final determinant reduction changes inverse bits. Workaround: `scalar-math`.

Reported result: `inverse_xx`.

**DIVERGES: 2 result groups**

- `3e9d0c09`: macOS ARM64, Linux x86-64, Windows x86-64, Wasm SIMD
- `3e9d0c0a`: Wasm scalar

### [Transposed matrix-vector product](examples/mat4_transpose.rs)

`mul_transpose_vec4` inherits the four-term dot reduction. Workaround: `scalar-math`, or fixed-order dot products.

Reported result: `transpose_x`.

**DIVERGES: 3 result groups**

- `00000000`: macOS ARM64
- `40000000`: Linux x86-64, Windows x86-64, Wasm SIMD
- `3f800000`: Wasm scalar

### [Affine inverse](examples/affine_inverse.rs)

Mat3A arithmetic changes the inverse matrix and translation. Workaround: `scalar-math`.

Reported result: `translation_x`.

**DIVERGES: 2 result groups**

- `be18874e`: macOS ARM64
- `be188752`: Linux x86-64, Windows x86-64, Wasm scalar, Wasm SIMD

### [Quaternion multiplication](examples/quaternion_mul.rs)

Different grouping of the Hamilton-product terms changes the x component. Workaround: `scalar-math`, or a common grouping.

Reported result: `product_x`.

**DIVERGES: 2 result groups**

- `bddeb82c`: macOS ARM64, Linux x86-64, Windows x86-64, Wasm SIMD
- `bddeb82d`: Wasm scalar

### [Quaternion interpolation](examples/quaternion_slerp.rs)

Native SIMD `slerp` uses a private sine approximation despite `libm`. Its final arithmetic also differs. Workaround: `scalar-math`.

Reported result: `slerp_x`.

**DIVERGES: 2 result groups**

- `3e5f61cb`: macOS ARM64, Linux x86-64, Windows x86-64
- `3e5f61c9`: Wasm scalar, Wasm SIMD

### [Zero padding lanes](examples/zero_padding.rs)

A SIMD padding lane changes the sign of a zero reduction. Workaround: zero canonicalization, or `scalar-math`.

Reported results: `sum / dot_into_vec_x`.

**DIVERGES: 3 result groups**

- `00000000 / 00000000`: macOS ARM64
- `00000000 / 80000000`: Linux x86-64, Windows x86-64, Wasm SIMD
- `80000000 / 80000000`: Wasm scalar

### Additional SIMD zero selection

The same [signed-zero example](examples/zero_selection.rs) also exposes differences that disappear with `scalar-math`. These are additional backend choices, not proof of the same compiler issue.

**Maximum reduction**, reported result `Vec4_max_element_raw`:

**DIVERGES: 2 result groups**

- `00000000`: macOS ARM64, Wasm SIMD
- `80000000`: Linux x86-64, Windows x86-64, Wasm scalar

**Saturation**, reported result `Vec4_saturate_raw`:

**DIVERGES: 2 result groups**

- `00000000`: macOS ARM64, Linux x86-64, Windows x86-64, Wasm scalar
- `80000000`: Wasm SIMD

Workaround: `max_element_safe` for the reduction, and `clamp_safe(ZERO, ONE)` for saturation. Both guarded results are +0 on every tested build. `smoothstep` uses saturation internally, but its selected output matches across builds.

### [Rounding zero signs](examples/zero_rounding.rs)

SSE2 loses negative-zero signs in the selected floor, ceil, trunc, and round lanes. Workaround: zero canonicalization, or `scalar-math`.

Reported result: `vec3a_ceil`, for `-0.1`. The example includes both Vec3A and Vec4.

**DIVERGES: 2 result groups**

- `80000000`: macOS ARM64, Wasm scalar, Wasm SIMD
- `00000000`: Linux x86-64, Windows x86-64

## Source notes and original evidence

- [0.34.1 source audit](docs/glam-determinism-audit-0.34.1.md): arithmetic paths, measured differences, and limits.
- [Upstream report draft](docs/glam-determinism-bug-report.md): the smaller sum/dot, normalization, and cross-product repro.
- [Original files](archive/README.md): unchanged probe sources and captures, including the 0.30.10/0.33.12 reduction repros and the 0.30.10 helper verification.

The archived 0.34.1 audit has 14 configurations and 444 records per configuration, including input records. It is separate from this smaller suite. The archived helper program passed 476 checks on each of macOS, Windows, Linux, and Wasm.

These are counterexamples to bit-identical behavior, not proof for every input, compiler version, or floating-point environment. Build outputs remain untracked.
