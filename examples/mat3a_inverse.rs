//! Cross-product rounding changes whether this ill-conditioned matrix has an inverse
//! Workaround: rebuild with --features scalar-math
//! Scalar-math makes every backend reject this input, not compute a more accurate inverse

use glam::{Mat3A, Vec3A};
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let matrix = black_box(Mat3A::from_cols(
        Vec3A::X,
        Vec3A::new(0.0, 16785408.0, 4097.0),
        Vec3A::new(0.0, 4097.0, 1.0),
    ));
    show("determinant", matrix.determinant());
    println!("inverse_exists {:08x}", u32::from(matrix.try_inverse().is_some()));
}
