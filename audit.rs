use glam::{Affine2, Affine3A, DVec3, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
use std::hint::black_box;

fn show<const N: usize>(name: &str, values: [f32; N]) {
    println!("{name}: {values:?} | {:08x?}", values.map(f32::to_bits));
}

fn next_float(state: &mut u32) -> f32 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    ((*state >> 8) % 2001) as f32 / 1000.0 - 1.0
}

fn unit_quat(values: [f32; 4]) -> Quat {
    let length = (values[0] * values[0] + values[1] * values[1]
        + values[2] * values[2] + values[3] * values[3]).sqrt();
    Quat::from_array(values.map(|value| value / length))
}

fn main() {
    println!("backend: scalar={} sse2={} neon={} wasm={}",
        Vec3A::USES_SCALAR_MATH, Vec3A::USES_SSE2, Vec3A::USES_NEON, Vec3A::USES_WASM_SIMD);
    println!("compiled_fma={}", cfg!(target_feature = "fma"));
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    println!("runtime_fma={}", std::is_x86_feature_detected!("fma"));
    let v4 = black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
    show("Vec4.element_sum", [v4.element_sum()]);
    show("Vec4.dot", [v4.dot(Vec4::ONE)]);
    show("Vec4.element_product", [black_box(Vec4::new(1.1, 1.1, 1.1, 1.3)).element_product()]);
    let length4 = black_box(Vec4::new(1.0, 1.0, 1.0, 4096.0));
    show("Vec4.length_squared", [length4.length_squared()]);
    show("Vec4.length", [length4.length()]);
    let v3 = black_box(Vec3A::new(1.0, 1.0, 3.0));
    show("Vec3A.normalize", v3.normalize().to_array());
    show("Vec3A.try_normalize", v3.try_normalize().unwrap().to_array());
    show("Vec3A.normalize_or_zero", v3.normalize_or_zero().to_array());
    show("Vec3A.normalize_and_length", v3.normalize_and_length().0.to_array());
    show("Vec3A.dot", [v3.dot(Vec3A::ONE)]);
    show("Vec3A.element_sum", [v3.element_sum()]);
    let cancel3 = black_box(Vec3A::new(1e20, 1.0, -1e20));
    show("Vec3A.cancellation.dot", [cancel3.dot(Vec3A::ONE)]);
    show("Vec3A.cancellation.element_sum", [cancel3.element_sum()]);
    let a = black_box(Vec3A::new(0.0, 16785408.0, 4097.0));
    let b = black_box(Vec3A::new(0.0, 4097.0, 1.0));
    show("Vec3A.cross", a.cross(b).to_array());
    let s = black_box(0.9999999);
    show("Vec3A.lerp_monotonic", black_box(Vec3A::splat(-1.0)).lerp_monotonic(Vec3A::splat(0.00000011920929), s).to_array());
    show("Vec4.lerp_monotonic", black_box(Vec4::splat(-1.0)).lerp_monotonic(Vec4::splat(0.00000011920929), s).to_array());
    show("Vec3A.mul_add", black_box(Vec3A::splat(4097.0)).mul_add(Vec3A::splat(4097.0), Vec3A::splat(-16785408.0)).to_array());
    show("Vec4.mul_add", black_box(Vec4::splat(4097.0)).mul_add(Vec4::splat(4097.0), Vec4::splat(-16785408.0)).to_array());
    show("Vec4.normalize", black_box(Vec4::new(1.0, 1.0, 3.0, 0.0)).normalize().to_array());
    let zeros = black_box(Vec3A::splat(-0.0));
    show("Vec3A.zero.element_sum", [zeros.element_sum()]);
    show("Vec3A.zero.dot", [zeros.dot(Vec3A::ONE)]);
    show("Vec3A.zero.dot_into_vec", zeros.dot_into_vec(Vec3A::ONE).to_array());
    show("Vec3A.zero.min", zeros.min(Vec3A::ZERO).to_array());
    show("Vec3A.zero.max", Vec3A::ZERO.max(zeros).to_array());
    let zeros4 = black_box(Vec4::splat(-0.0));
    show("Vec4.zero.min", zeros4.min(Vec4::ZERO).to_array());
    show("Vec4.zero.max", Vec4::ZERO.max(zeros4).to_array());
    show("Vec3A.zero.min_element", [black_box(Vec3A::new(0.0, -0.0, 0.0)).min_element()]);
    show("Vec4.zero.min_element", [black_box(Vec4::new(0.0, -0.0, 0.0, -0.0)).min_element()]);
    let rounding = black_box(Vec3A::new(-0.1, -0.0, -0.5));
    show("Vec3A.floor", rounding.floor().to_array());
    show("Vec3A.ceil", rounding.ceil().to_array());
    show("Vec3A.trunc", rounding.trunc().to_array());
    show("Vec3A.round", rounding.round().to_array());
    let rounding4 = black_box(Vec4::new(-0.1, -0.0, -0.5, -0.25));
    show("Vec4.floor", rounding4.floor().to_array());
    show("Vec4.ceil", rounding4.ceil().to_array());
    show("Vec4.trunc", rounding4.trunc().to_array());
    show("Vec4.round", rounding4.round().to_array());
    let m3a = black_box(Mat3A::from_cols(Vec3A::X, a, b));
    show("Mat3A.determinant", [m3a.determinant()]);
    println!("Mat3A.try_inverse: {}", m3a.try_inverse().is_some());
    let affine3a = black_box(Affine3A {
        matrix3: Mat3A::from_cols(Vec3A::new(2.0, 0.3, 0.4), Vec3A::new(0.5, 3.0, 0.6), Vec3A::new(0.7, 0.8, 4.0)),
        translation: Vec3A::new(1.0, 2.0, 3.0),
    });
    show("Affine3A.inverse.matrix", affine3a.inverse().matrix3.to_cols_array());
    show("Affine3A.inverse.translation", affine3a.inverse().translation.to_array());
    let m3 = black_box(Mat3::from_cols(Vec3::X, a.to_vec3(), b.to_vec3()));
    show("Mat3.determinant", [m3.determinant()]);
    let mt = black_box(Mat4::from_cols(v4, v4, v4, v4));
    show("Mat4.mul_transpose_vec4", mt.mul_transpose_vec4(Vec4::ONE).to_array());
    let general4 = black_box(Mat4::from_cols_array(&[
        2.0, 0.3, 0.4, 0.5, 0.6, 3.0, 0.8, 0.9,
        1.0, 1.1, 4.0, 1.3, 1.4, 1.5, 1.6, 5.0,
    ]));
    show("Mat4.inverse", general4.inverse().to_cols_array());
    show("Mat4.determinant", [general4.determinant()]);
    let dense4 = black_box(Mat4::from_cols_array(&[
        1.2, 0.7, -0.3, 2.1, -0.4, 2.3, 1.1, 0.8,
        0.5, -0.9, 3.4, 1.7, 2.2, 0.6, -1.4, 4.5,
    ]));
    show("Mat4.dense.determinant", [dense4.determinant()]);
    show("Mat4.dense.inverse", dense4.inverse().to_cols_array());
    let q = black_box(Quat::from_xyzw(0.18257418, 0.36514837, 0.5477226, 0.73029673));
    let r = black_box(Quat::from_xyzw(0.73029673, 0.5477226, 0.36514837, 0.18257418));
    show("Quat.mul", (q * r).to_array());
    let r2 = black_box(Quat::from_xyzw(0.10259783, 0.20519567, 0.3077935, 0.9233805));
    show("Quat.mul_mixed", (q * r2).to_array());
    show("Quat.slerp", black_box(Quat::IDENTITY).slerp(Quat::from_xyzw(0.5, 0.5, 0.5, 0.5), 0.37).to_array());
    let m2 = black_box(Mat2::from_cols(Vec2::new(1.1, 2.2), Vec2::new(3.3, 4.4)));
    show("Mat2.determinant", [m2.determinant()]);
    show("Mat2.inverse", m2.inverse().to_cols_array());
    show("Mat2.mul_vec2", (m2 * Vec2::new(5.5, 6.6)).to_array());
    let affine2 = black_box(Affine2::from_cols(m2.x_axis, m2.y_axis, Vec2::new(5.5, 6.6)));
    show("Affine2.transform_point2", affine2.transform_point2(Vec2::new(7.7, 8.8)).to_array());
    show("Affine2.inverse", affine2.inverse().to_cols_array());
    show("Vec3.normalize", black_box(Vec3::new(1.0, 1.0, 3.0)).normalize().to_array());
    show("Vec3.cross", a.to_vec3().cross(b.to_vec3()).to_array());
    let d = black_box(DVec3::new(1.0, 1.0, 3.0)).normalize().to_array();
    println!("DVec3.normalize: {d:?} | {:016x?}", d.map(f64::to_bits));
    show("Vec3A.sin", black_box(Vec3A::new(0.3, 0.7, 1.1)).sin().to_array());
    show("Vec3A.exp", black_box(Vec3A::new(0.3, 0.7, 1.1)).exp().to_array());
    show("Vec4.sin", black_box(Vec4::new(0.3, 0.7, 1.1, 1.3)).sin().to_array());
    let mut state = 12345;
    for i in 0..64 {
        let mut values = std::array::from_fn(|_| next_float(&mut state));
        for j in [0, 5, 10, 15] {
            values[j] += 3.0;
        }
        let matrix = black_box(Mat4::from_cols_array(&values));
        print!("Mat4.scan.{i}."); show("input", values);
        print!("Mat4.scan.{i}."); show("det", [matrix.determinant()]);
        print!("Mat4.scan.{i}."); show("inverse", matrix.try_inverse().unwrap().to_cols_array());
        let q = black_box(unit_quat(std::array::from_fn(|_| next_float(&mut state))));
        let r = black_box(unit_quat(std::array::from_fn(|_| next_float(&mut state))));
        print!("Quat.scan.{i}."); show("lhs", q.to_array());
        print!("Quat.scan.{i}."); show("rhs", r.to_array());
        print!("Quat.scan.{i}."); show("mul", (q * r).to_array());
    }
}
