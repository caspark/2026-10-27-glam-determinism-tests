//! Fused multiply-subtract differs from separate products and subtraction
//! Workaround: rebuild with --features scalar-math, or choose one arithmetic policy

use glam::Vec3A;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let a = black_box(Vec3A::new(0.0, 16785408.0, 4097.0));
    let b = black_box(Vec3A::new(0.0, 4097.0, 1.0));
    show("cross_x", a.cross(b).x);

    show("separate_x", a.y * b.z - a.z * b.y);
    show("fused_x", (-a.z).mul_add(b.y, a.y * b.z));
}
