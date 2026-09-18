use code_reload::is_inside_dynamic_library;
use std::time::Duration;

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
    println!("\tStart...");
    std::thread::sleep(Duration::from_secs(5));
    println!("\t...done");
    std::thread::sleep(Duration::from_millis(100));
    v + v + 100
}

pub struct Struct;

impl Struct {
    #[unsafe(export_name = "__code_reload_UNKNOWN_1_1_assoc")]
    pub fn assoc(self, v: i32) -> i32 {
        if !is_inside_dynamic_library(Self::assoc as *const _) {
            return ::code_reload::provide_fn::<fn(Self, i32) -> i32>(
                "simple.dll",
                b"__code_reload_UNKNOWN_1_1_assoc",
            )(self, v);
        }
        v + 122
    }
}

pub trait Trait {
    fn trait_assoc(&self, v: i32) -> i32;

    #[unsafe(export_name = "__code_reload_UNKNOWN_1_1_statat")]
    fn statat(v: i32) -> i32 {
        v + 1
    }
}

impl Trait for Struct {
    #[unsafe(export_name = "__code_reload_UNKNOWN_1_1_trait_assoc")]
    fn trait_assoc(&self, v: i32) -> i32 {
        if !is_inside_dynamic_library(Self::trait_assoc as *const _) {
            return ::code_reload::provide_fn::<fn(&Self, i32) -> i32>(
                "simple.dll",
                b"__code_reload_UNKNOWN_1_1_trait_assoc",
            )(self, v);
        }
        v + 1535
    }
}
