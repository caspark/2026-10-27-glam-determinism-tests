//! Paired multiplication differs from left-to-right multiplication
//! Workaround: rebuild with --features scalar-math, or choose one multiplication order

use glam::Vec4;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let v = black_box(Vec4::new(1.1, 1.1, 1.1, 1.3));
    show("product", v.element_product());
    show("fixed_order", ((v.x * v.y) * v.z) * v.w);
}
