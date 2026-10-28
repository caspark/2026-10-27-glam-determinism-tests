use glam::Vec4;

fn main() {
    let v = std::hint::black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
    println!("element_sum: {}", v.element_sum());
    println!("dot(ONE): {}", v.dot(Vec4::ONE));
}
