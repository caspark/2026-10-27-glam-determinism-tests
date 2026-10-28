# glam 0.34.1 determinism audit

## Scope

The audited release is glam 0.34.1, with `libm` enabled. No game dependencies changed.

Source: [glam tag 0.34.1](https://github.com/bitshifter/glam-rs/tree/0.34.1), commit `f5c5a6b635c5be1b7e904808175470ed0f29f4ff`. The audited `src/` tree matches the published crate archive.

The source review covered backend selection, floating-point vector primitives, matrix/quaternion arithmetic, affine call chains, and shared math helpers. Camera and Euler code were inspected for their arithmetic dependencies. This is not a proof for every API and input.

The executable probes cover 14 build configurations:

| Target | Rust | Configurations |
| --- | --- | --- |
| ARM64 macOS | 1.96.1 | Default NEON, scalar-math |
| x86-64 Windows MSVC | 1.95.0 | Default SSE2, scalar-math, +fma, +fma with scalar-math |
| x86-64 Linux | 1.94.0 | Default SSE2, scalar-math, +fma, +fma with scalar-math |
| wasm32-wasip1 / Wasmtime 44.0.1 | 1.96.1 | Default scalar, scalar-math, +simd128, +simd128 with scalar-math |

Windows and Linux compiled and ran the code on their respective machines. Each build records its selected glam backend and compile-time FMA flag. Both x86 CPUs support FMA.

Each configuration produced 444 records: 60 fixed records, plus inputs and results for 64 matrix/quaternion cases. The generated inputs are identical across configurations. Comparisons use result bits, not formatted decimal equality.

## Measured numerical differences

The scalar-math configurations agree on the numerical examples in this table. Signed-zero exceptions follow separately.

| Operation and input | NEON | SSE2 without FMA | Wasm scalar | Wasm SIMD | scalar-math |
| --- | --- | --- | --- | --- | --- |
| Vec4 `(1e20, 1, -1e20, 1)`, element_sum | 0 | 0 | 1 | 0 | 1 |
| Same Vec4, dot with ONE | 0 | 2 | 1 | 2 | 1 |
| Vec4 `(1.1, 1.1, 1.1, 1.3)`, element_product | 1.7303001 | 1.7303 | 1.7303001 | 1.7303 | 1.7303001 |
| Vec4 `(1, 1, 1, 4096)`, length_squared | 16777218 | 16777218 | 16777220 | 16777218 | 16777220 |
| Same Vec4, length | 4096 | 4096 | 4096.0005 | 4096 | 4096.0005 |
| Vec3A `(1, 1, 3)`, normalize().z | 0.90453404 | 0.904534 | 0.90453404 | 0.904534 | 0.90453404 |
| Vec3A `(0, 16785408, 4097)` cross `(0, 4097, 1)`, x | -1 | 0 | 0 | 0 | 0 |
| Mat3A with columns X and the two cross-product inputs, determinant | -1 | 0 | 0 | 0 | 0 |
| Same Mat3A, try_inverse | Some | None | None | None | None |

Enabling `+fma` on either x86 host changes the cross-product result to -1 and makes `try_inverse` return Some. It does not fix the reduction or normalization differences.

`Vec4::normalize()` also differs for `(1, 1, 3, 0)`. Its z component has the same one-ULP difference as the Vec3A example.

### Source causes

All source paths in this section are relative to the pinned source root.

1. **Four-term reductions.** Scalar Vec4 sums left to right. NEON sums adjacent pairs. SSE2 and Wasm SIMD dot products sum `(x + z) + (y + w)`. Their element sums instead use `(x + y) + (z + w)`. Sources: `src/f32/scalar/vec4.rs:243,379`, `src/f32/neon/vec4.rs:365`, `src/neon.rs:67`, `src/sse2.rs:80`, `src/wasm.rs:65`.

2. **Products also regroup.** NEON/scalar Vec4 element products associate left to right. SSE2/Wasm SIMD multiply two pairs before the final product. Sources: `src/f32/{neon,sse2,wasm,scalar}/vec4.rs`, methods `element_product`.

3. **Normalization uses different operations.** SSE2 and Wasm SIMD divide each component by the length. NEON/scalar multiply each component by the reciprocal length. The length calculation is identical for the Vec3A witness. The final division versus reciprocal/multiply produces the difference. Sources: `src/f32/sse2/vec3a.rs:674`, `src/f32/wasm/vec3a.rs:643`, `src/f32/neon/vec3a.rs:674`, `src/f32/scalar/vec3a.rs:647`. Vec4 has the same split.

4. **Cross products conditionally fuse.** NEON uses `vfmsq_f32`. SSE2 calls `m128_neg_mul_sub`, which fuses only with `target_feature="fma"`. Scalar and Wasm SIMD use separate products and subtraction. Sources: `src/f32/neon/vec3a.rs:270`, `src/f32/sse2/vec3a.rs:277`, `src/sse2.rs:149`, `src/f32/scalar/vec3a.rs:271`, `src/f32/wasm/vec3a.rs:263`.

5. **Mat3A inherits cross-product differences.** Determinant uses `x.dot(y.cross(z))`. Inversion constructs three cross products, computes a reciprocal determinant, then transposes scaled cofactors. Affine3A inversion calls Mat3A inversion. Sources: `src/f32/neon/mat3a.rs:707,724`, `src/f32/scalar/mat3a.rs:689,707`, `src/f32/affine3a.rs:424`. The general Affine3A inverse probe also differs, without a zero determinant.

The matrix example concerns consistency, not accuracy. Scalar-math makes every client reject the same particular inverse. It does not improve the numerical result.

## Other reproduced differences

### Monotonic interpolation

For Vec3A and Vec4:

```rust
let a = Vec3A::splat(-1.0);
let b = Vec3A::splat(0.00000011920929);
let result = a.lerp_monotonic(b, 0.9999999);
```

NEON and x86 with `+fma` return `-1.4210855e-14` per component. Other tested configurations return zero.

This is documented behavior: the SIMD method documentation explicitly permits hardware-dependent fusion. It is a determinism limitation, not evidence that this method violates its documented contract.

Sources: `src/f32/neon/vec3a.rs:1032`, `src/f32/sse2/vec3a.rs:1035`, `src/f32/scalar/vec3a.rs:1017`, and corresponding Vec4 methods.

**The public `mul_add` methods are not the same case.** Their SSE2 fallback calls `math::mul_add`, which calls `libm::fmaf`. Vec3A and Vec4 public `mul_add` return 1 for `4097 * 4097 - 16785408` in every tested configuration. Do not confuse those methods with the internal conditional-fusion helpers.

### Mat4

The 64 matrix cases produced these differences against the scalar reference build:

| Backend | Determinants that differ | Inverse matrices that differ |
| --- | --- | --- |
| NEON | 0/64 | 19/64 |
| SSE2, with or without FMA | 12/64 | 12/64 |
| Wasm SIMD | 12/64 | 12/64 |
| All scalar-math builds | 0/64 | 0/64 |

NEON and scalar `Mat4::determinant` use the same explicit cofactor expression. SSE2/Wasm SIMD use vector cofactors and a four-term dot reduction. All inverse implementations construct cofactors, but their final determinant reductions differ in association.

Sources: `src/f32/neon/mat4.rs:761,948`, `src/f32/scalar/mat4.rs:763,860`, `src/f32/sse2/mat4.rs:772,952`, `src/f32/wasm/mat4.rs:770,947`.

`Mat4::mul_transpose_vec4` also reproduces the Vec4 dot discrepancy. It returns four zeros on NEON, four twos on SSE2/Wasm SIMD, and four ones on scalar.

Ordinary matrix-vector multiplication is a different path. The inspected Mat3A/Mat4 implementations add axis products sequentially, without the conditional-fusion helpers. Do not label every matrix multiplication as a four-term dot reduction.

### Quaternions

Quaternion multiplication differs in 44 of the 64 unit-quaternion pairs. Each pair has identical input bits across builds. SIMD implementations group products into pairs, while scalar code spells sequential component expressions.

The fused instructions in quaternion multiplication multiply already-rounded products by sign constants. Their presence alone does not establish an original-product fusion difference. The source-level association differs independently.

Sources: `src/f32/scalar/quat.rs:780`, `src/f32/sse2/quat.rs:794`, `src/f32/neon/quat.rs:794`, `src/f32/wasm/quat.rs:774`.

For this normalized-input probe:

```rust
Quat::IDENTITY.slerp(Quat::from_xyzw(0.5, 0.5, 0.5, 0.5), 0.37)
```

NEON/SSE2 return x = `0.21814649`, w = `0.9258706`. Scalar/Wasm return x = `0.21814646`, w = `0.9258705`.

SSE2/NEON slerp use private polynomial sine implementations even with `libm` enabled. Scalar and Wasm slerp use the shared math facade. Their final arithmetic also differs.

Sources: `src/f32/neon/quat.rs:686`, `src/f32/sse2/quat.rs:683`, `src/f32/scalar/quat.rs:677`, `src/f32/wasm/quat.rs:676`.

## Signed-zero findings

These are separate from the numerical-value examples.

- Vec3A element_sum adds a positive-zero padding lane in SIMD implementations. Three negative zeros sum to positive zero there, but to negative zero on scalar.
- NEON Vec3A dot_into_vec also adds a zero lane. It returns positive zero where its scalar-valued dot method returns negative zero.
- SSE2 Vec3A/Vec4 floor, ceil, trunc, and round lose negative-zero signs in the tested cases. The helpers convert through integers without restoring those signs. Sources: `src/sse2.rs:88,108,181,193`.
- SIMD min/max and min/max reductions have target-specific zero/NaN selection. The library documents differing NaN behavior. This audit does not treat NaN payload differences as a new defect.

### A mismatch that survives scalar-math

On ARM64 macOS with scalar-math:

```rust
let x = std::hint::black_box(-0.0f32);
let result = if x < 0.0 { x } else { 0.0 };
```

The result is negative zero. The same compiler version produces positive zero on Wasm.

Vec2, Vec3, Vec3A, and Vec4 min calls with a constant positive-zero operand reproduce that difference. A dynamic zero operand and a non-inlined comparison control return positive zero on both targets. Explicit zero-canonicalization controls also return positive zero on both.

This is not a demonstrated glam-only defect: plain Rust reproduces it. Comparison-based source code alone does not establish a signed-zero guarantee after optimization. No compiler root cause or fix is claimed here.

The game's Vec2/Vec3 exemption in AGENTS.md was too strong. That factual claim was corrected during the audit. Game code and lint policy remained unchanged at that point.

## What matched and what remains unproven

- Vec3A's inspected three-term sum/dot trees do not have the four-term regrouping of Vec4. The finite cancellation probe matched. The zero-lane exceptions remain relevant.
- Vec3A try_normalize, normalize_or_zero, and normalize_and_length matched for the normalization witness. Unlike SSE2 normalize, these paths use reciprocal/multiply.
- Public Vec3A/Vec4 mul_add matched, as did the vector sin/exp probes with libm enabled.
- Mat2 determinant, inverse, and multiplication probes matched. Affine2 transform/inverse probes matched. The inspected 2D expressions use common scalar math or two-product sums, not the divergent four-term reductions.
- Vec2, Vec3, Mat3, and f64 types use shared arithmetic source, not the f32 SIMD backend families. The Vec3 and DVec3 normalization controls matched. This does not exempt their min/max zero ties.
- Camera construction uses shared Vec3/math operations. Euler construction/extraction uses shared sin_cos, sqrt, and atan2 helpers. Their returned matrix/quaternion types still have the backend-dependent operations described above.

Core-simd source was inspected but not compiled. Nightly, SPIR-V, CUDA, relaxed Wasm SIMD, non-default floating-point environments, and libm's own implementation correctness are outside this audit. Integer APIs and serialization layout are also outside scope.

With scalar-math, every measured numerical value matched. The two constant-zero vector min probes still differed in zero signs. Scalar-math is not a complete substitute for zero-sign policy.

## Reproduction files

The original reproduction files and captures are in [`../archive/`](../archive/). Run the archived commands from that directory. The primary examples and runner are in the repository root.

- `audit.rs`, `v0.34.1/Cargo.toml`, `v0.34.1/Cargo.lock`: numerical probes and fixed input generator
- `windows-0341.ps1`, `linux-0341.sh`: native runners
- `macos-0341-*.txt`, `wasm-0341-*.txt`, `windows-0341-results.txt`, `linux-0341-results.txt`: raw output
- `results-0341.json`: 444 records for each of the 14 configurations
- `zero.rs`, `macos-0341-zero-isolation.txt`, `wasm-0341-zero-isolation.txt`: separate comparison controls

Disposable remote code remains in `C:\temp\glam-reduction-repro-remote` and `/tmp/glam-reduction-repro-remote`.
