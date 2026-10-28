//! Native SIMD slerp uses a private sine approximation even with libm enabled
//! Workaround: rebuild with --features scalar-math to use the common scalar path

use glam::Quat;
use glam_determinism_repros::show;
use std::hint::black_box;

fn main() {
    let start = black_box(Quat::IDENTITY);
    let end = black_box(Quat::from_xyzw(0.5, 0.5, 0.5, 0.5));
    let result = start.slerp(end, black_box(0.37));
    show("slerp_x", result.x);
    show("slerp_w", result.w);
}
