#[code_reload::hotreload]
pub fn standalone_get() -> i32 {
    1
}

pub struct Struct;

impl Struct {
    #[code_reload::hotreload_assoc]
    pub fn get(&self) -> i32 {
        2
    }
    #[code_reload::hotreload_assoc]
    pub fn mut_get(&mut self) -> i32 {
        3
    }
    #[code_reload::hotreload_assoc]
    pub fn consume_get(self) -> i32 {
        4
    }
    #[code_reload::hotreload_assoc]
    pub fn box_get(self: Box<Self>) -> i32 {
        5
    }
    #[code_reload::hotreload_assoc]
    pub fn static_get() -> i32 {
        6
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
    #[code_reload::hotreload_assoc]
    fn trait_get(&self) -> i32 {
        7
    }
    #[code_reload::hotreload_assoc]
    fn trait_mut_get(&mut self) -> i32 {
        8
    }
    #[code_reload::hotreload_assoc]
    fn trait_consume_get(self) -> i32 {
        9
    }
    #[code_reload::hotreload_assoc]
    fn trait_box_get(self: Box<Self>) -> i32 {
        10
    }
    #[code_reload::hotreload_assoc]
    fn trait_static_get() -> i32 {
        11
    }
}
