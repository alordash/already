#[already::hotreload]
pub fn standalone_get() -> i32 {
    10
}

pub struct Struct;

impl Struct {
    #[already::hotreload]
    pub fn get(&self) -> i32 {
        20
    }
    #[already::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        30
    }
    #[already::hotreload]
    pub fn consume_get(self) -> i32 {
        40
    }
    #[already::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        50
    }
    #[already::hotreload]
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
    #[already::hotreload]
    fn trait_get(&self) -> i32 {
        70
    }
    #[already::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        80
    }
    #[already::hotreload]
    fn trait_consume_get(self) -> i32 {
        90
    }
    #[already::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        100
    }
    #[already::hotreload]
    fn trait_static_get() -> i32 {
        110
    }
}
