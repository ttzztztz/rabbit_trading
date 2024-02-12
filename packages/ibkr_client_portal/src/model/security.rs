use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::definition::AssetClass;

pub struct SecurityDefinitionsRequest {
    /// underlying contract id
    pub underlying_con_id: i64,
    /// FUT/OPT/WAR/CASH/CFD
    pub sectype: AssetClass,
    /// contract month, only required for FUT/OPT/WAR in the format MMMYY, example JAN00
    pub month: Option<String>,
    /// optional, default is SMART
    pub exchange: Option<String>,
    /// optional, only required for OPT/WAR
    pub strike: Option<Decimal>,
    /// C for call, P for put
    pub right: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityDefinitionsResponse {
    /// IBKR contract identifier
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// Security type
    #[serde(rename = "secType")]
    pub sec_type: Option<String>,
    /// Primary Exchange, Routing or Trading Venue
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    /// Main Trading Venue
    #[serde(rename = "listingExchange")]
    pub listing_exchange: Option<String>,
    /// Put or Call of the option. C = Call Option, P = Put Option
    #[serde(rename = "right")]
    pub right: Option<String>,
    /// Set price at which a derivative contract can be bought or sold. The strike price also known as exercise price.
    #[serde(rename = "strike")]
    pub strike: Option<i64>,
    /// Currency the contract trades in
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Committee on Uniform Securities Identification Procedures number
    #[serde(rename = "cusip")]
    pub cusip: Option<String>,
    /// Annual interest rate paid on a bond
    #[serde(rename = "coupon")]
    pub coupon: Option<String>,
    /// Currency pairs for Forex e.g. EUR.AUD, EUR.CAD, EUR.CHF etc.
    #[serde(rename = "desc1")]
    pub desc1: Option<String>,
    /// Formatted expiration, strike and right
    #[serde(rename = "desc2")]
    pub desc2: Option<String>,
    /// Format YYYYMMDD, the date on which the underlying transaction settles if the option is exercised
    #[serde(rename = "maturityDate")]
    pub maturity_date: Option<i64>,
    /// Multiplier for total premium paid or received for derivative contract.
    #[serde(rename = "multiplier")]
    pub multiplier: Option<String>,
    /// Designation of the contract.
    #[serde(rename = "tradingClass")]
    pub trading_class: Option<String>,
    /// Comma separated list of exchanges or trading venues.
    #[serde(rename = "validExchanges")]
    pub valid_exchanges: Option<String>,
}
