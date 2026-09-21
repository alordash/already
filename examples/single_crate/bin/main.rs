use single_target::sum;

fn main() {
    loop {
        let result = sum(1, 1);
        dbg!(result);
        std::thread::sleep(core::time::Duration::from_millis(2000));
    }
}
