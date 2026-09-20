#[hotcode::hotreload]
pub fn standalone_get() -> i32 {
    10
}

pub struct Struct;

impl Struct {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        20
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        30
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        40
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        50
    }
    #[hotcode::hotreload]
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
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        70
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        80
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        90
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        100
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        110
    }
}
