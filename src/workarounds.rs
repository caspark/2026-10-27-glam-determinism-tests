//! Zero-sign policy used by Signs of Danger, adapted to a standalone module
//!
//! The DVec implementations extend the same policy for these reproducers.
//! Zero results become +0. Nonzero values and NaN behavior follow the underlying operations.
//! These helpers do not fix reduction order, normalization, or other arithmetic differences.

use glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec3A, Vec4};

#[inline]
pub fn canonicalize_f32_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

#[inline]
pub fn canonicalize_f64_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

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

impl_scalar!(f32, canonicalize_f32_zero);
impl_scalar!(f64, canonicalize_f64_zero);

pub trait VectorFloatSafeExt: Sized {
    type Scalar;

    #[must_use]
    fn min_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn max_safe(self, rhs: Self) -> Self;
    #[must_use]
    fn clamp_safe(self, min: Self, max: Self) -> Self;
    #[must_use]
    fn min_element_safe(self) -> Self::Scalar;
    #[must_use]
    fn max_element_safe(self) -> Self::Scalar;
}

macro_rules! impl_vector {
    ($vector:ty, $scalar:ty, $canonicalize:path, $canonicalize_scalar:path) => {
        impl VectorFloatSafeExt for $vector {
            type Scalar = $scalar;

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
            fn min_element_safe(self) -> Self::Scalar {
                $canonicalize_scalar(self.min_element())
            }

            #[inline]
            fn max_element_safe(self) -> Self::Scalar {
                $canonicalize_scalar(self.max_element())
            }
        }
    };
}

#[inline]
fn canonicalize_vec2_zeros(value: Vec2) -> Vec2 {
    Vec2::new(canonicalize_f32_zero(value.x), canonicalize_f32_zero(value.y))
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

#[inline]
fn canonicalize_dvec2_zeros(value: DVec2) -> DVec2 {
    DVec2::new(canonicalize_f64_zero(value.x), canonicalize_f64_zero(value.y))
}

#[inline]
fn canonicalize_dvec3_zeros(value: DVec3) -> DVec3 {
    DVec3::select(value.cmpeq(DVec3::ZERO), DVec3::ZERO, value)
}

#[inline]
fn canonicalize_dvec4_zeros(value: DVec4) -> DVec4 {
    DVec4::select(value.cmpeq(DVec4::ZERO), DVec4::ZERO, value)
}

impl_vector!(Vec2, f32, canonicalize_vec2_zeros, canonicalize_f32_zero);
impl_vector!(Vec3, f32, canonicalize_vec3_zeros, canonicalize_f32_zero);
impl_vector!(Vec3A, f32, canonicalize_vec3a_zeros, canonicalize_f32_zero);
impl_vector!(Vec4, f32, canonicalize_vec4_zeros, canonicalize_f32_zero);
impl_vector!(DVec2, f64, canonicalize_dvec2_zeros, canonicalize_f64_zero);
impl_vector!(DVec3, f64, canonicalize_dvec3_zeros, canonicalize_f64_zero);
impl_vector!(DVec4, f64, canonicalize_dvec4_zeros, canonicalize_f64_zero);
