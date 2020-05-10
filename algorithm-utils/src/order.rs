use crate::{Derivative, Percent, Points, PositionType, Price, Instruction};

use chrono::{DateTime, Local, Duration};

#[derive(Clone, Debug)]
pub struct Order {
    pub order_time: DateTime<Local>,
    pub derivative: Derivative,
    pub pieces: u64,
    pub order_type: OrderType,
    pub position_type: PositionType,
    pub order_moment: OrderMoment,
    pub order_validity: OrderValidity
}

impl Order {
    pub fn from_instruction(_instruction: Instruction) -> Self {
        unimplemented!()
    }

    pub fn is_now_or_passed(&self) -> bool {
        self.order_moment.is_now_or_passed()
    }

    pub fn is_valid(&self) -> bool {
        self.order_validity.is_valid(&self.order_time)
    }
}

#[derive(Clone, Debug)]
pub struct OneCancelsTheOther {
    pub orders: Vec<Order>,
    pub canceled: bool,
}

impl OneCancelsTheOther {
    pub fn new(orders: Vec<Order>) -> Self {
        Self {
            orders,
            canceled: false,
        }
    }
}

impl Default for OneCancelsTheOther {
    fn default() -> Self {
        Self {
            orders: Vec::new(),
            canceled: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OrderType {
    MarketOrder,
    LimitOrder(Price),
    StopOrder(Price),
    TrailingStopOrder(Price, TrailingStop),
}

#[derive(Clone, Copy, Debug)]
pub enum TrailingStop {
    Percent(Percent),
    Price(Price),
    Points(Points),
}

#[derive(Clone, Debug)]
pub enum OrderMoment {
    Instant,
    Planed(DateTime<Local>),
}

impl OrderMoment {
    pub fn as_duration(&self) -> Duration {
        use OrderMoment::*;
        match self {
            Instant => Duration::zero(),
            Planed(date_time) => *date_time - Local::now()
        }
    }

    pub fn is_now_or_passed(&self) -> bool {
        self.as_duration() <= Duration::zero()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OrderValidity {
    OneDay,
    OneWeek,
    OneMonth,
    OneYear,
    Forever,
}

impl OrderValidity {
    pub fn as_duration(&self) -> Duration {
        use OrderValidity::*;
        match self {
            OneDay => Duration::days(1),
            OneWeek => Duration::weeks(1),
            OneMonth => Duration::days(30),
            OneYear => Duration::days(365),
            Forever => Duration::max_value()
        }
    }

    pub fn is_valid(&self, start: &DateTime<Local>) -> bool {
        let duration = self.as_duration();
        *start + duration < Local::now()
    }
}
