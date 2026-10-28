//! Different addition orders for the same sum and dot product
//! Workaround: rebuild with --features scalar-math, or choose one reduction order

use glam::Vec4;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let v = black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
    show("sum", v.element_sum());
    show("dot", v.dot(Vec4::ONE));

    let fixed_order = ((v.x + v.y) + v.z) + v.w;
    show("fixed_order", fixed_order);
}
