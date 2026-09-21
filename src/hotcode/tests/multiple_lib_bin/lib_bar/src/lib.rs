#[hotcode::hotreload]
pub fn get_both() -> (i32, i32) {
    (1, lib_foo::get())
}
