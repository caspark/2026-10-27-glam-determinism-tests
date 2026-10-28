use glam::{Vec2, Vec3, Vec3A, Vec4};
use std::hint::black_box;

pub mod prelude {
    pub use glam::{Vec2, Vec3, Vec4};
}

include!("canonical_zero.rs");
mod safe_float;
use safe_float::{FloatSafeExt, VectorFloatSafeExt};

macro_rules! verify_scalar {
    ($float:ty) => {{
        let mut checks = 0;
        for a in [0.0 as $float, -0.0 as $float] {
            for b in [0.0 as $float, -0.0 as $float] {
                let a = black_box(a);
                let b = black_box(b);
                for result in [a.min_safe(b), a.max_safe(b), a.clamp_safe(b, 1.0), a.clamp_safe(-1.0, b), a.clamp_safe(b, b)] {
                    assert_eq!(result.to_bits(), 0, "{} zero guard", stringify!($float));
                    checks += 1;
                }
            }
        }
        for a in [-3.5 as $float, -0.0 as $float, 2.25 as $float, 9.0 as $float] {
            let a = black_box(a);
            for (got, expected) in [(a.min_safe(1.0), a.min(1.0)), (a.max_safe(-1.0), a.max(-1.0)), (a.clamp_safe(-1.0, 1.0), a.clamp(-1.0, 1.0))] {
                let expected = if expected == 0.0 { 0.0 } else { expected };
                assert_eq!(got.to_bits(), expected.to_bits(), "{} finite control", stringify!($float));
                checks += 1;
            }
        }
        println!("{}: {checks} scalar checks passed", stringify!($float));
    }};
}

macro_rules! verify_vector {
    ($vector:ty, $len:expr) => {{
        let mut checks = 0;
        for a in [0.0f32, -0.0] {
            for b in [0.0f32, -0.0] {
                let a = black_box(<$vector>::splat(a));
                let b = black_box(<$vector>::splat(b));
                for result in [a.min_safe(b), a.max_safe(b), a.clamp_safe(b, <$vector>::ONE), a.clamp_safe(-<$vector>::ONE, b), a.clamp_safe(b, b)] {
                    for value in result.to_array() {
                        assert_eq!(value.to_bits(), 0, "{} zero guard", stringify!($vector));
                        checks += 1;
                    }
                }
                for result in [a.min_element_safe(), a.max_element_safe()] {
                    assert_eq!(result.to_bits(), 0, "{} reduction", stringify!($vector));
                    checks += 1;
                }
            }
        }
        let a = black_box(<$vector>::splat(-0.0));
        for result in [a.min_safe(<$vector>::ZERO), a.max_safe(<$vector>::ZERO), <$vector>::ZERO.min_safe(a), <$vector>::ZERO.max_safe(a), a.clamp_safe(<$vector>::ZERO, <$vector>::ONE), a.clamp_safe(-<$vector>::ONE, <$vector>::ZERO), a.min_safe(<$vector>::splat(f32::INFINITY)), a.max_safe(<$vector>::splat(f32::NEG_INFINITY))] {
            for value in result.to_array() {
                assert_eq!(value.to_bits(), 0, "{} constant-bound guard", stringify!($vector));
                checks += 1;
            }
        }
        let lhs: [f32; $len] = std::array::from_fn(|i| [-3.5, -0.0, 2.25, 9.0][i % 4]);
        let rhs: [f32; $len] = std::array::from_fn(|i| [-2.0, 0.0, 4.0, 8.0][i % 4]);
        let lhs = black_box(<$vector>::from_array(lhs));
        let rhs = black_box(<$vector>::from_array(rhs));
        for (got, expected) in [(lhs.min_safe(rhs), lhs.min(rhs)), (lhs.max_safe(rhs), lhs.max(rhs)), (lhs.clamp_safe(-<$vector>::ONE, <$vector>::ONE), lhs.clamp(-<$vector>::ONE, <$vector>::ONE))] {
            for (got, expected) in got.to_array().into_iter().zip(expected.to_array()) {
                assert_eq!(got.to_bits(), canonicalize_f32_zero(expected).to_bits(), "{} finite control", stringify!($vector));
                checks += 1;
            }
        }
        for (got, expected) in [(lhs.min_element_safe(), lhs.min_element()), (lhs.max_element_safe(), lhs.max_element())] {
            assert_eq!(got.to_bits(), canonicalize_f32_zero(expected).to_bits(), "{} finite reduction", stringify!($vector));
            checks += 1;
        }
        println!("{}: {checks} vector checks passed", stringify!($vector));
    }};
}

fn main() {
    verify_scalar!(f32);
    verify_scalar!(f64);
    verify_vector!(Vec2, 2);
    verify_vector!(Vec3, 3);
    verify_vector!(Vec3A, 3);
    verify_vector!(Vec4, 4);
}
