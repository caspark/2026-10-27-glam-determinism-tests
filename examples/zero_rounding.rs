//! SSE2 rounding loses negative-zero signs in these lanes
//! Workaround: canonicalize zero results, or rebuild with --features scalar-math

use glam::{Vec3A, Vec4};
use glam_determinism_repros::{show, workarounds::canonicalize_f32_zero};
use std::hint::black_box;

fn main() {
    let v = black_box(Vec3A::new(-0.1, -0.0, -0.5));
    let v4 = black_box(Vec4::new(-0.1, -0.0, -0.5, -0.25));
    for (name, value) in [
        ("vec3a_floor", v.floor().y),
        ("vec3a_ceil", v.ceil().x),
        ("vec3a_trunc", v.trunc().x),
        ("vec3a_round", v.round().x),
        ("vec4_floor", v4.floor().y),
        ("vec4_ceil", v4.ceil().x),
        ("vec4_trunc", v4.trunc().x),
        ("vec4_round", v4.round().x),
    ] {
        show(name, value);
        show(&format!("canonical_{name}"), canonicalize_f32_zero(value));
    }
}
