//! Final determinant reductions change a component of the inverse
//! Workaround: rebuild with --features scalar-math
//! This fixed input comes from case 4 of the archived matrix scan

use glam::Mat4;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let matrix = black_box(Mat4::from_cols_array(&[
        3.579, -0.959, -0.55700004, 0.65400004,
        -0.993, 2.237, 0.78, 0.16799998,
        0.15999997, -0.143, 3.7610002, 0.13300002,
        -0.352, -0.12599999, -0.884, 2.764,
    ]));
    show("determinant", matrix.determinant());
    show("inverse_xx", matrix.try_inverse().unwrap().x_axis.x);
}
