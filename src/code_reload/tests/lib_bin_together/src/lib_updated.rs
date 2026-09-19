#[code_reload::hotreload]
pub fn standalone_get() -> i32 {
    10
}

pub struct Struct;

impl Struct {
    pub fn get(&self) -> i32 {
        20
    }
    pub fn mut_get(&mut self) -> i32 {
        30
    }
    pub fn consume_get(self) -> i32 {
        40
    }
    pub fn static_get() -> i32 {
        50
    }
}

pub trait Trait {
    fn trait_get(&self) -> i32;
    fn trait_mut_get(&mut self) -> i32;
    fn trait_consume_get(self) -> i32;
    fn trait_static_get() -> i32;
}

impl Trait for Struct {
    fn trait_get(&self) -> i32 {
        60
    }
    fn trait_mut_get(&mut self) -> i32 {
        70
    }
    fn trait_consume_get(self) -> i32 {
        80
    }
    fn trait_static_get() -> i32 {
        90
    }
}
