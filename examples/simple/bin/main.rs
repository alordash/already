use simple::foo;

fn main() {
    loop {
        let result = foo(10);
        dbg!(result);
        std::thread::sleep(core::time::Duration::from_millis(1700));
    }
}
