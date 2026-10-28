#[inline]
pub fn canonicalize_f32_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

/// Applies [`canonicalize_f32_zero`] to both components of a vector.
#[inline]
pub fn canonicalize_vec2_zeros(value: Vec2) -> Vec2 {
    Vec2::new(canonicalize_f32_zero(value.x), canonicalize_f32_zero(value.y))
}
