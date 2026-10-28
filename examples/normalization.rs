//! Division differs from multiplication by the reciprocal length
//! Workaround: rebuild with --features scalar-math
//! normalize_or_zero matches the reciprocal path for this Vec3A input, not every vector input

use glam::{Vec3A, Vec4};
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let v = black_box(Vec3A::new(1.0, 1.0, 3.0));
    show("vec3a_z", v.normalize().z);
    show("reciprocal_vec3a_z", v.normalize_or_zero().z);

    let v = black_box(Vec4::new(1.0, 1.0, 3.0, 0.0));
    show("vec4_z", v.normalize().z);
    let squared = ((v.x * v.x + v.y * v.y) + v.z * v.z) + v.w * v.w;
    show("reciprocal_vec4_z", (v * (1.0 / squared.sqrt())).z);
}
