#[hotcode::hotreload]
pub fn get_both() -> (i32, i32) {
    (2, lib_foo::get())
}
