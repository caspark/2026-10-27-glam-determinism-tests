//! A SIMD padding lane changes zero signs without changing the numerical value
//! Workaround: canonicalize zero results, or rebuild with --features scalar-math

use glam::Vec3A;
use glam_determinism_repros::{show, workarounds::canonicalize_f32_zero};
use std::hint::black_box;

fn main() {
    let v = black_box(Vec3A::splat(-0.0));
    let sum = v.element_sum();
    let dot = v.dot_into_vec(Vec3A::ONE).x;
    show("sum", sum);
    show("dot_into_vec_x", dot);
    show("canonical_sum", canonicalize_f32_zero(sum));
    show("canonical_dot", canonicalize_f32_zero(dot));
}
