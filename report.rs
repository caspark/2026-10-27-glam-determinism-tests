use glam::{Vec3A, Vec4};

fn main() {
    let v = std::hint::black_box(Vec4::new(1e20, 1.0, -1e20, 1.0));
    println!("element_sum: {}", v.element_sum());
    println!("dot(ONE): {}", v.dot(Vec4::ONE));

    let v3 = std::hint::black_box(Vec3A::new(1.0, 1.0, 3.0));
    println!("normalize.z: {}", v3.normalize().z);

    let a = std::hint::black_box(Vec3A::new(0.0, 16785408.0, 4097.0));
    let b = std::hint::black_box(Vec3A::new(0.0, 4097.0, 1.0));
    println!("cross.x: {}", a.cross(b).x);
}
