//! Different grouping of the four Hamilton-product terms changes the result
//! Workaround: rebuild with --features scalar-math, or use a common grouping
//! These normalized inputs come from case 5 of the archived quaternion scan

use glam::Quat;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let a = black_box(Quat::from_xyzw(0.31863627, 0.6506888, -0.3602267, 0.58763236));
    let b = black_box(Quat::from_xyzw(-0.39685926, -0.14977257, 0.6044121, -0.67435664));
    show("product_x", (a * b).x);
    show("fixed_order_x", ((a.w * b.x + a.x * b.w) + a.y * b.z) - a.z * b.y);
}
