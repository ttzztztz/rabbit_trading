use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{currency::Currency, symbol::Symbol};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RegularTradingTime {
    AllTime,
    OnlyRegularTradingTime,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Expire {
    Day,
    GoodTillDate { year: i32, month: i32, day: i32 },
    GoodTillCancelled,
    OpenPriceGuarantee,
    ImmediateOrCancel,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TrailingMarketPrice {
    Amount { trailing_amount: Decimal },
    Percent { trailing_percent: Decimal },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubmitOrderRequest {
    pub symbol: Symbol,
    pub quantity: Decimal,
    pub direction: Direction,
    pub regular_trading_time: RegularTradingTime,
    pub expire: Expire,
    pub price: Price,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubmitOrderResponse {
    pub order_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EstimateMaxBuyingPowerRequest {
    pub symbol: Symbol,
    pub direction: Direction,
    pub price: Price,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuyingPower {
    pub cash_max_quantity: Decimal,
    pub margin_max_quantity: Decimal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CancelOrderRequest {
    pub order_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CancelOrderResponse {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EditOrderRequest {
    pub order_id: String,
    pub quantity: Decimal,
    pub price: Price,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EditOrderResponse {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderDetailRequest {
    pub order_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderDetail {
    pub order_id: String,
    pub symbol: Symbol,
    pub currency: Currency,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub price: Price,
    pub executed_price: Option<Decimal>,
    pub direction: Direction,
    pub regular_trading_time: RegularTradingTime,
    pub expire: Expire,
    pub created_timestamp: u64,
    pub updated_timestamp: Option<u64>,
    pub triggered_timestamp: Option<u64>,
}
