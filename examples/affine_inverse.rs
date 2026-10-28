//! Affine inversion inherits the Mat3A cross-product arithmetic
//! Workaround: rebuild with --features scalar-math

use glam::{Affine3A, Mat3A, Vec3A};
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let affine = black_box(Affine3A {
        matrix3: Mat3A::from_cols(
            Vec3A::new(2.0, 0.3, 0.4),
            Vec3A::new(0.5, 3.0, 0.6),
            Vec3A::new(0.7, 0.8, 4.0),
        ),
        translation: Vec3A::new(1.0, 2.0, 3.0),
    });
    let inverse = affine.inverse();
    show("inverse_xy", inverse.matrix3.x_axis.y);
    show("translation_x", inverse.translation.x);
}
