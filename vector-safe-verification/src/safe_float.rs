//! Deterministic floating point maths helpers

use glam::Vec3A;

use super::*;
use crate::prelude::*;

/// Scalar min/max/clamp with positive-zero results and standard NaN/bounds behavior
pub trait FloatSafeExt: Sized {
    #[must_use]
    fn min_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn max_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn clamp_safe(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_scalar {
    ($float:ty, $canonicalize:path) => {
        #[allow(
            clippy::disallowed_methods,
            reason = "These implementations canonicalize the raw scalar results"
        )]
        impl FloatSafeExt for $float {
            #[inline]
            fn min_safe(self, rhs: Self) -> Self {
                $canonicalize(self.min(rhs))
            }

            #[inline]
            fn max_safe(self, rhs: Self) -> Self {
                $canonicalize(self.max(rhs))
            }

            #[inline]
            fn clamp_safe(self, min: Self, max: Self) -> Self {
                $canonicalize(self.clamp(min, max))
            }
        }
    };
}

#[inline]
fn canonicalize_f64_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

impl_scalar!(f32, canonicalize_f32_zero);
impl_scalar!(f64, canonicalize_f64_zero);

/// Vector min/max/clamp and reductions with positive-zero results
///
/// NaN behavior follows the underlying vector operations
pub trait VectorFloatSafeExt: Sized {
    #[must_use]
    fn min_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn max_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn clamp_safe(self, min: Self, max: Self) -> Self;
    #[must_use]
    fn min_element_safe(self) -> f32;
    #[must_use]
    fn max_element_safe(self) -> f32;
}

macro_rules! impl_vector {
    ($vector:ty, $canonicalize:path) => {
        #[allow(
            clippy::disallowed_methods,
            reason = "These implementations canonicalize the raw vector results"
        )]
        impl VectorFloatSafeExt for $vector {
            #[inline]
            fn min_safe(self, rhs: Self) -> Self {
                $canonicalize(self.min(rhs))
            }

            #[inline]
            fn max_safe(self, rhs: Self) -> Self {
                $canonicalize(self.max(rhs))
            }

            #[inline]
            fn clamp_safe(self, min: Self, max: Self) -> Self {
                $canonicalize(self.clamp(min, max))
            }

            #[inline]
            fn min_element_safe(self) -> f32 {
                canonicalize_f32_zero(self.min_element())
            }

            #[inline]
            fn max_element_safe(self) -> f32 {
                canonicalize_f32_zero(self.max_element())
            }
        }
    };
}

#[inline]
fn canonicalize_vec3_zeros(value: Vec3) -> Vec3 {
    Vec3::select(value.cmpeq(Vec3::ZERO), Vec3::ZERO, value)
}

#[inline]
fn canonicalize_vec3a_zeros(value: Vec3A) -> Vec3A {
    Vec3A::select(value.cmpeq(Vec3A::ZERO), Vec3A::ZERO, value)
}

#[inline]
fn canonicalize_vec4_zeros(value: Vec4) -> Vec4 {
    Vec4::select(value.cmpeq(Vec4::ZERO), Vec4::ZERO, value)
}

impl_vector!(Vec2, canonicalize_vec2_zeros);
impl_vector!(Vec3, canonicalize_vec3_zeros);
impl_vector!(Vec3A, canonicalize_vec3a_zeros);
impl_vector!(Vec4, canonicalize_vec4_zeros);
