use code_reload::hotreload;

#[hotreload]
pub fn foo(v: i32) -> i32 {
    v + 10
}
