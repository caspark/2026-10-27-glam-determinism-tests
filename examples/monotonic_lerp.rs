//! The docs explicitly permit hardware-dependent fusion for lerp_monotonic
//! Workaround: rebuild with --features scalar-math, or choose fused or separate arithmetic

use glam::{Vec3A, Vec4};
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let a = black_box(Vec3A::splat(-1.0));
    let b = black_box(Vec3A::splat(0.00000011920929));
    let t = black_box(0.9999999);
    show("vec3a", a.lerp_monotonic(b, t).x);
    show("vec4", Vec4::splat(a.x).lerp_monotonic(Vec4::splat(b.x), t).x);

    show("separate", a.x + (b.x - a.x) * t);
    show("fused", (b.x - a.x).mul_add(t, a.x));
}
