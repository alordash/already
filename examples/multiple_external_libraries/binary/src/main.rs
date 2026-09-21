fn main() {
    let (a, b, c) = (3.0f32, 4.0f32, 5.0f32);
    loop {
        let add = first_library::add(a, b);
        dbg!(add);
        let mul_add = second_library::mul_add(a, b, c);
        dbg!(mul_add);
        std::thread::sleep(core::time::Duration::from_millis(2000));
    }
}
