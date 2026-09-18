use simple::{Struct, Trait};

fn main() {
    let s = Struct;
    loop {
        // let result = foo(10);
        // let result = Struct.assoc(10);
        let result = s.trait_assoc(10);
        dbg!(result);
        std::thread::sleep(core::time::Duration::from_millis(1700));
    }
    println!("well hello there");
}
