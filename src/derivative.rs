#![allow(unused)]

// todo
pub struct Derivative {
    pub identifier: Identifier
}

impl Derivative {
    pub fn new(identifier: Identifier) -> Self {
        Self {
            identifier
        }
    }
}

pub enum Identifier {
    ISIN(String),
    SYMBOL(String),
}
