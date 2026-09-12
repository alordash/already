use code_reload::hotreload;

fn main() {
    let result = foo(10);
    dbg!(result);
    println!("well hello there");
}

#[hotreload]
fn foo(v: i32) -> i32 {
    v + 1
}