use crate::{Derivative, Percent, Points, PositionType, Price, Instruction, StockExchange};

use chrono::{DateTime, Local, Duration};

#[derive(Clone, Debug)]
pub struct Order {
    stock_exchange: StockExchange,
    order_time: DateTime<Local>,
    derivative: Derivative,
    pieces: u64,
    order_type: OrderType,
    position_type: PositionType,
    order_moment: OrderMoment,
    order_validity: OrderValidity,
    one_cancels_the_other: Option<Box<Order>>
}

impl Order {
    #[inline]
    pub fn is_now_or_passed(&self) -> bool {
        self.order_moment.is_now_or_passed()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.order_validity.is_valid(&self.order_time)
    }

    /// cancels all orders tied to it by the one_cancels_the_other_field
    pub fn cancel_others() -> CancelResult {
        unimplemented!()
    }
}

impl From<Instruction<'_>> for Order {
    fn from(_instruction: Instruction) -> Self {
        unimplemented!()
    }
}

pub enum CancelResult {
    Canceled,
    NotCanceled,
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
