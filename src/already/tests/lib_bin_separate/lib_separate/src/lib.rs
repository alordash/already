#[already::hotreload]
pub fn standalone_get() -> i32 {
    1
}

pub struct Struct;

impl Struct {
    #[already::hotreload]
    pub fn get(&self) -> i32 {
        2
    }
    #[already::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        3
    }
    #[already::hotreload]
    pub fn consume_get(self) -> i32 {
        4
    }
    #[already::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        5
    }
    #[already::hotreload]
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
    #[already::hotreload]
    fn trait_get(&self) -> i32 {
        7
    }
    #[already::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        8
    }
    #[already::hotreload]
    fn trait_consume_get(self) -> i32 {
        9
    }
    #[already::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        10
    }
    #[already::hotreload]
    fn trait_static_get() -> i32 {
        11
    }
}
