use rust_decimal::Decimal;

use super::symbol::Symbol;

pub enum Direction {
    Buy,
    Sell,
}

pub enum RegularTradingTime {
    AllTime,
    OnlyRegularTradingTime,
}

pub enum Expire {
    Day,
    GoodTillDate,
    GoodTillCancelled { year: i32, month: i32, day: i32 },
}

#[derive(Clone, Copy)]
pub enum TrailingLimitPrice {
    Amount {
        limit_offset: Decimal,
        trailing_amount: Decimal,
    },
    Percent {
        limit_offset: Decimal,
        trailing_percent: Decimal,
    },
}

#[derive(Clone, Copy)]
pub enum TrailingMarketPrice {
    Amount { trailing_amount: Decimal },
    Percent { trailing_percent: Decimal },
}

pub enum Price {
    LimitOrder {
        price: Decimal,
    },
    MarketOrder,
    LimitIfTouched {
        submit_price: Decimal,
        trigger_price: Decimal,
    },
    MarketIfTouched {
        trigger_price: Decimal,
    },
    TrailingLimitIfTouched {
        trailing: TrailingLimitPrice,
    },
    TrailingMarketIfTouched {
        trailing: TrailingMarketPrice,
    },
}

pub struct SubmitOrderRequest {
    pub symbol: Symbol,
    pub quantity: i64,
    pub direction: Direction,
    pub regular_trading_time: RegularTradingTime,
    pub expire: Expire,
    pub price: Price,
}

pub struct SubmitOrderResponse {
    pub order_id: String,
}
