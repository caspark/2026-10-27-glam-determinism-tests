pub mod workarounds;

pub fn show(name: &str, value: f32) {
    println!("{name} {:08x}", value.to_bits());
}

pub fn show_f64(name: &str, value: f64) {
    println!("{name} {:016x}", value.to_bits());
}
