use code_reload::{hotreload, is_inside_dynamic_library};

// #[hotreload]
// pub fn foo(v: i32) -> i32 {
//     v + 10
// }

#[unsafe(export_name = "__code_reload_UNKNOWN_1_1_foo")]
pub fn foo(v: i32) -> i32 {
    if !is_inside_dynamic_library(foo as *const _) {
        return ::code_reload::provide_fn::<fn(i32) -> i32>(
            "simple.dll",
            b"__code_reload_UNKNOWN_1_1_foo",
        )(v);
    }
    v + v
}
