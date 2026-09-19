#[code_reload::hotreload]
pub fn standalone_get() -> i32 {
    10
}

pub struct Struct;

impl Struct {
    #[code_reload::hotreload]
    pub fn get(&self) -> i32 {
        20
    }
    #[code_reload::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        30
    }
    #[code_reload::hotreload]
    pub fn consume_get(self) -> i32 {
        40
    }
    #[code_reload::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        50
    }
    #[code_reload::hotreload]
    pub fn static_get() -> i32 {
        60
    }
}

pub trait Trait {
    fn trait_get(&self) -> i32;
    fn trait_mut_get(&mut self) -> i32;
    fn trait_consume_get(self) -> i32;
    fn trait_box_get(self: Box<Self>) -> i32;
    fn trait_static_get() -> i32;
}

impl Trait for Struct {
    #[code_reload::hotreload]
    fn trait_get(&self) -> i32 {
        70
    }
    #[code_reload::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        80
    }
    #[code_reload::hotreload]
    fn trait_consume_get(self) -> i32 {
        90
    }
    #[code_reload::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        100
    }
    #[code_reload::hotreload]
    fn trait_static_get() -> i32 {
        110
    }
}
