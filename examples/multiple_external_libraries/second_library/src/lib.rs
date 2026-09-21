#[hotcode::hotreload]
pub fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    first_library::add(a * b, c)
}