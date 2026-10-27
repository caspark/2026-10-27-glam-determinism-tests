use glam::{Vec2, Vec3, Vec3A, Vec4};
use std::hint::black_box;

#[inline(never)]
fn choose(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn main() {
    let a = black_box(-0.0f32);
    let b = black_box(0.0f32);
    for (name, value) in [
        ("raw.select", if a < b { a } else { b }),
        ("raw.select_constant", if a < 0.0 { a } else { 0.0 }),
        ("raw.choose", choose(a, b)),
        ("raw.min", a.min(b)),
        ("raw.canonical_zero", if a == 0.0 { 0.0 } else { a }),
        ("raw.canonical_min", { let v = a.min(0.0); if v == 0.0 { 0.0 } else { v } }),
        ("Vec2.min.dynamic", Vec2::splat(a).min(Vec2::splat(b)).x),
        ("Vec2.min.constant", Vec2::splat(a).min(Vec2::ZERO).x),
        ("Vec3.min.dynamic", Vec3::splat(a).min(Vec3::splat(b)).x),
        ("Vec3.min.constant", Vec3::splat(a).min(Vec3::ZERO).x),
        ("Vec3A.min.dynamic", Vec3A::splat(a).min(Vec3A::splat(b)).x),
        ("Vec3A.min.constant", Vec3A::splat(a).min(Vec3A::ZERO).x),
        ("Vec4.min.dynamic", Vec4::splat(a).min(Vec4::splat(b)).x),
        ("Vec4.min.constant", Vec4::splat(a).min(Vec4::ZERO).x),
    ] {
        println!("{name}: {value:?} {:08x}", value.to_bits());
    }
}
