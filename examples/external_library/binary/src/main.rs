use library::add;

fn main() {
    loop {
        let result = add(1, 1);
        dbg!(result);
        std::thread::sleep(core::time::Duration::from_millis(2000));
    }
}
