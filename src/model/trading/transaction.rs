use rust_decimal::Decimal;

use super::{currency::Currency, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegularTradingTime {
    AllTime,
    OnlyRegularTradingTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expire {
    Day,
    GoodTillDate { year: i32, month: i32, day: i32 },
    GoodTillCancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrailingMarketPrice {
    Amount { trailing_amount: Decimal },
    Percent { trailing_percent: Decimal },
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitOrderRequest {
    pub symbol: Symbol,
    pub quantity: i64,
    pub direction: Direction,
    pub regular_trading_time: RegularTradingTime,
    pub expire: Expire,
    pub price: Price,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitOrderResponse {
    pub order_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EstimateMaxBuyingPowerRequest {
    pub symbol: Symbol,
    pub direction: Direction,
    pub price: Price,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuyingPower {
    pub cash_max_quantity: i64,
    pub margin_max_quantity: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CancelOrderRequest {
    pub order_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CancelOrderResponse {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditOrderRequest {
    pub order_id: String,
    pub quantity: i64,
    pub price: Price,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditOrderResponse {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderDetailRequest {
    pub order_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderDetail {
    pub order_id: String,
    pub symbol: Symbol,
    pub currency: Currency,
    pub quantity: i64,
    pub executed_quantity: i64,
    pub price: Price,
    pub executed_price: Option<Decimal>,
    pub direction: Direction,
    pub regular_trading_time: RegularTradingTime,
    pub expire: Expire,
    pub created_timestamp: u64,
    pub updated_timestamp: Option<u64>,
    pub triggered_timestamp: Option<u64>,
}