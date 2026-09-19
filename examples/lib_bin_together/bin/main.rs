use lib_bin_together::foo;

fn main() {
    loop {
        let result = foo(10);
        dbg!(result);
        std::thread::sleep(core::time::Duration::from_millis(1700));
    }
    println!("well hello there");
}
