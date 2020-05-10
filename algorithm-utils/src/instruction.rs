use crate::{OrderType, Position, PositionType};

pub enum Instruction<'p> {
    Buy {
        pieces: u64,
        order_type: OrderType,
        position_type: PositionType,
    },
    Sell {
        position: &'p Position
    },
    None,
}
