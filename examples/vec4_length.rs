//! The reduction order changes both squared length and length
//! Workaround: rebuild with --features scalar-math, or use a fixed-order reduction

use glam::Vec4;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let v = black_box(Vec4::new(1.0, 1.0, 1.0, 4096.0));
    show("squared", v.length_squared());
    show("length", v.length());

    let squared = ((v.x * v.x + v.y * v.y) + v.z * v.z) + v.w * v.w;
    show("fixed_squared", squared);
    show("fixed_length", squared.sqrt());
}
