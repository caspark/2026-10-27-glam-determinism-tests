//! Transposed matrix-vector multiplication inherits the four-term dot reduction
//! Workaround: rebuild with --features scalar-math, or use fixed-order dot products

use glam::{Mat4, Vec4};
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let v = black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
    let matrix = black_box(Mat4::from_cols(v, v, v, v));
    show("transpose_x", matrix.mul_transpose_vec4(Vec4::ONE).x);
    show("fixed_order_x", ((v.x + v.y) + v.z) + v.w);
}
