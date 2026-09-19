#[code_reload::hotreload]
pub fn standalone_get() -> i32 {
    1
}

pub struct Struct;

impl Struct {
    pub fn get(&self) -> i32 {
        2
    }
    pub fn mut_get(&mut self) -> i32 {
        3
    }
    pub fn consume_get(self) -> i32 {
        4
    }
    pub fn static_get() -> i32 {
        5
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
        6
    }
    fn trait_mut_get(&mut self) -> i32 {
        7
    }
    fn trait_consume_get(self) -> i32 {
        8
    }
    fn trait_static_get() -> i32 {
        9
    }
}
