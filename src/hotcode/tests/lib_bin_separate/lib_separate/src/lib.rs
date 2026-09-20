#[hotcode::hotreload]
pub fn standalone_get() -> i32 {
    1
}

pub struct Struct;

impl Struct {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        2
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        3
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        4
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        5
    }
    #[hotcode::hotreload]
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
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        7
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        8
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        9
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        10
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        11
    }
}
