use chrono::{DateTime, Local};

use crate::{Order};

pub struct Position {
    pub id: String,
    pub bought: DateTime<Local>,
    pub order: Order,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionType {
    LongCall,
    LongPut,
    ShortCall,
    ShortPut,
}
