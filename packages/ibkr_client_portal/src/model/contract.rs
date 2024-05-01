use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    definition::{AssetClass, OptionRight},
    portfolio::DisplayRule,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDefinitions {
    pub secdef: Vec<Contract>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// List of exchanges and venues contract trades.
    #[serde(with = "unpack_exchanges", alias = "exchange")]
    pub all_exchanges: Vec<String>,
    /// Group of financial instruments which have similar financial characteristics and behave similar in the marketplace.
    pub asset_class: Option<AssetClass>,
    /// HTML encoded company description in Chinese.
    pub chinese_name: Option<String>,
    /// IBKR contract identifier.
    pub conid: i64,
    pub country_code: Option<String>,
    /// Currency contract trades in.
    pub currency: Option<String>,
    pub display_rule: Option<DisplayRule>,
    /// Specific data contract expires.
    pub expiry: Option<String>,
    /// Formatted company name with underlying symbol, expiration, strike, right.
    pub full_name: Option<String>,
    /// Potential characteristic of each product.
    pub group: Option<String>,
    /// If contract has an option.
    pub has_options: Option<bool>,
    /// Price increment value contract trades.
    pub increment_rules: Option<Vec<IncrementRule>>,
    pub is_event_contract: Option<bool>,
    /// If contract is a US contract. Currently supported for stocks, options and warrants.
    #[serde(rename = "isUS")]
    pub is_us: Option<bool>,
    /// Final day derivative contract can be traded before delivery of the underlying asset or cash settlement.
    pub last_trading_day: Option<String>,
    /// Main trading venue.
    pub listing_exchange: Option<String>,
    /// Multiplier for total premium paid or received for derivative contract.
    pub multiplier: Option<Decimal>,
    /// Company Name.
    pub name: Option<String>,
    pub page_size: Option<i64>,
    /// Defines the right to buy or sell of the underlying security.
    pub put_or_call: Option<OptionRight>,
    /// The category of the economy.
    pub sector: Option<String>,
    /// Stock Group contract belongs too.
    pub sector_group: Option<String>,
    /// Set price at which a derivative contract can be bought or sold.
    pub strike: Option<String>,
    /// Contract symbol.
    pub ticker: Option<String>,
    pub time: Option<i64>,
    /// Multiplier for total premium paid or received for derivative contract.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Underlying contract identifier.
    #[serde(rename = "undConid")]
    pub underlying_conid: Option<i64>,
    /// Defines if a derivative contract has a different currency.
    pub cross_currency: Option<bool>,
    /// Company name for underlying contract.
    #[serde(rename = "undComp")]
    pub underlying_company_name: Option<String>,
    /// IBKR Symbol for underlying contract.
    #[serde(rename = "undSym")]
    pub underlying_symbol: Option<String>,
}

pub mod unpack_exchanges {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(exchanges: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = exchanges
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let exchanges: Vec<String> = s.split(',').map(|e| e.trim().to_string()).collect();

        Ok(exchanges)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementRule {
    #[serde(with = "rust_decimal::serde::float")]
    pub increment: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub lower_edge: Decimal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractDetail {
    /// true means you can trade outside RTH(regular trading hours)
    #[serde(rename = "r_t_h")]
    pub regular_trading_hours: bool,
    /// same as that in request
    #[serde(rename = "con_id")]
    pub conid: i64,
    /// Contracts company name
    pub company_name: Option<String>,
    pub exchange: Option<String>,
    /// for exmple FB
    pub local_symbol: Option<String>,
    /// for example STK
    pub instrument_type: String,
    pub currency: String,
    pub category: Option<String>,
    pub industry: Option<String>,
    pub symbol: Option<String>,
    pub cusip: Option<String>,
    pub expiry_full: Option<String>,
    pub maturity_date: Option<String>,
    pub multiplier: Option<String>,
    #[serde(rename = "underlying_con_id")]
    pub underlying_conid: Option<i64>,
    pub underlying_issuer: Option<String>,
    pub trading_class: Option<String>,
    #[serde(with = "unpack_exchanges")]
    pub valid_exchanges: Vec<String>,
    pub allow_sell_long: Option<bool>,
    pub is_zero_commission_security: Option<bool>,
    pub contract_clarification_type: Option<String>,
    pub contract_month: Option<String>,
    pub classifier: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetContractDetailRequest {
    pub conid: i64,
}

pub type StockContracts = HashMap<String, Vec<StockContractInfo>>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockContractInfo {
    pub asset_class: AssetClass,
    pub chinese_name: Option<String>,
    pub contracts: Vec<Contract>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetStocksBySymbolRequest {
    pub symbols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForSecurityRequest {
    /// symbol or name to be searched
    pub symbol: String,
    #[serde(rename = "name")]
    /// should be true if the search is to be performed by name. false by default.
    pub is_name: bool,
    /// If search is done by name, only the assets provided in this field will be returned. Currently, only STK is supported.
    #[serde(rename = "secType")]
    pub sec_type: AssetClass,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchForSecurityResponseSection {
    /// Asset Class
    #[serde(rename = "secType")]
    sec_type: Option<String>,
    /// List of expiration month(s) and year(s) in MMMYY format separated by semicolon
    #[serde(rename = "months")]
    months: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    symbol: Option<String>,
    /// Listing Exchange
    #[serde(rename = "exchange")]
    exchange: Option<String>,
    /// For combo's defines the asset class for each leg
    #[serde(rename = "legSecType")]
    leg_sec_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchForSecurityItem {
    /// Contract Identifier
    #[serde(rename = "conid")]
    pub conid: Option<String>,
    /// Company Name - Exchange
    #[serde(rename = "companyHeader")]
    pub company_header: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// Exchange
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "restricted")]
    pub restricted: Option<String>,
    /// List of Future Option expirations in YYYMMDD format separated by semicolon
    #[serde(rename = "fop")]
    pub future_option_expirations: Option<String>,
    /// List of Option expirations in YYYYMMDD format separated by semicolon
    #[serde(rename = "opt")]
    pub option_expirations: Option<String>,
    /// List of Warrant expirations in YYYYMMDD format separated by semicolon
    #[serde(rename = "war")]
    pub warrant_expirations: Option<String>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<SearchForSecurityResponseSection>>,
}

pub type SearchForSecurityResponse = Vec<SearchForSecurityItem>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSecurityDefinitionByConIdRequest {
    #[serde(rename = "conids")]
    pub conid_list: Vec<i64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TradingSession {
    #[serde(rename = "openingTime")]
    pub opening_time: Option<String>,
    #[serde(rename = "closingTime")]
    pub closing_time: Option<String>,
    /// If the whole trading day is considered LIQUID then the value 'LIQUID' is returned.
    #[serde(rename = "prop")]
    pub prop: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TradingTime {
    #[serde(rename = "openingTime")]
    pub opening_time: Option<String>,
    #[serde(rename = "closingTime")]
    pub closing_time: Option<String>,
    #[serde(rename = "cancelDayOrders")]
    pub cancel_day_orders: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TradingSchedule {
    #[serde(rename = "clearingCycleEndTime")]
    pub clearing_cycle_end_time: Option<String>,
    /// 20000101 stands for any Sat, 20000102 stands for any Sun, ... 20000107 stands for any Fri. Any other date stands for itself.
    #[serde(rename = "tradingScheduleDate")]
    pub trading_schedule_date: Option<String>,
    #[serde(rename = "sessions")]
    pub sessions: Option<Vec<TradingSession>>,
    #[serde(rename = "tradingTimes")]
    pub trading_times: Option<TradingTime>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSecurityTradingScheduleRequest {
    pub asset_class: AssetClass,
    pub symbol: String,
    pub exchange: Option<String>,
    pub exchange_filter: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityTradingSchedule {
    /// Exchange parameter id
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Reference on a trade venue of given exchange parameter
    #[serde(rename = "tradeVenueId")]
    pub trade_venue_id: Option<String>,
    /// Always contains at least one 'tradingTime'  and zero or more 'sessionTime' tags
    #[serde(rename = "schedules")]
    pub schedules: Option<Vec<TradingSchedule>>,
}

pub type GetSecurityTradingScheduleResponse = Vec<SecurityTradingSchedule>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSecurityStrikesRequest {
    pub conid: i64,
    pub sectype: AssetClass,
    pub month: String,
    pub exchange: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSecurityStrikesResponse {
    #[serde(rename = "call")]
    pub call: Option<Vec<Decimal>>,
    #[serde(rename = "put")]
    pub put: Option<Vec<Decimal>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderDefault {
    /// Outside of Regular Trading Hours
    #[serde(rename = "ORTH")]
    pub outside_of_regular_trading_hours: Option<bool>,
    /// Stop Price value
    #[serde(rename = "SP")]
    pub stop_price_value: Option<String>,
    /// Limit Price value
    #[serde(rename = "LP")]
    pub limit_price_value: Option<String>,
    /// Price Cap value
    #[serde(rename = "PC")]
    pub price_cap_value: Option<String>,
    /// Trailing amount value
    #[serde(rename = "TA")]
    pub trailing_amount_value: Option<String>,
    /// Trailing unit
    #[serde(rename = "TU")]
    pub trailing_unit: Option<String>,
    /// Relative offset amount
    #[serde(rename = "ROA")]
    pub relative_offset_amount: Option<String>,
    /// Relative offset percent
    #[serde(rename = "ROP")]
    pub relative_offset_percent: Option<String>,
    /// Touch trigger price
    #[serde(rename = "TT")]
    pub touch_trigger_price: Option<String>,
    /// Use Net Price for Bonds
    #[serde(rename = "UNP")]
    pub use_net_price_for_bonds: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TradingRule {
    /// Contract supports algo orders
    #[serde(rename = "algoEligible")]
    pub algo_eligible: Option<bool>,
    /// List of Accounts that can be traded
    #[serde(rename = "canTradeAcctIds")]
    pub can_trade_acct_ids: Option<Vec<String>>,
    /// Returns a description on any errors with order presets
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// list of available order types
    #[serde(rename = "orderTypes")]
    pub order_types: Option<Vec<String>>,
    /// order types that support IB Algos
    #[serde(rename = "ibalgoTypes")]
    pub ibalgo_types: Option<Vec<String>>,
    /// order types that support fractional trades - NOTE: Fractional share orders cannot be placed using the IBKR API solutions
    #[serde(rename = "fraqTypes")]
    pub fractional_types: Option<Vec<String>>,
    /// order types that support cash quantity trades
    #[serde(rename = "cqtTypes")]
    pub cqt_types: Option<Vec<String>>,
    /// If object returned will provide the defaults based on user settings
    #[serde(rename = "orderDefaults")]
    pub order_defaults: Option<HashMap<String, OrderDefault>>,
    /// order types that support outside of regular trading hours
    #[serde(rename = "orderTypesOutside")]
    pub order_types_outside: Option<Vec<String>>,
    /// Default quantity
    #[serde(rename = "defaultSize")]
    pub default_size: Option<Decimal>,
    /// cash value
    #[serde(rename = "cashSize")]
    pub cash_size: Option<Decimal>,
    /// increment quantity value
    #[serde(rename = "sizeIncrement")]
    pub size_increment: Option<Decimal>,
    /// Time in Force values, formatted with o for supporting Outside regular trading hours and a for Algo trading
    #[serde(rename = "tifTypes")]
    pub tif_types: Option<Vec<String>>,
    /// Default time in force value
    #[serde(rename = "defaultTIF")]
    pub default_time_in_force: Option<String>,
    /// Limit price
    #[serde(rename = "limitPrice")]
    pub limit_price: Option<Decimal>,
    /// Stop price
    #[serde(rename = "stopprice")]
    pub stop_price: Option<Decimal>,
    /// Order origin designation for US securities options and Options Clearing Corporation
    #[serde(rename = "orderOrigination")]
    pub order_origination: Option<Decimal>,
    /// order preview required
    #[serde(rename = "preview")]
    pub preview: Option<bool>,
    #[serde(rename = "displaySize")]
    pub display_size: Option<Decimal>,
    /// decimal places for fractional order size
    #[serde(rename = "fraqInt")]
    pub fractional_order_size: Option<Decimal>,
    /// Cash currency for the contract
    #[serde(rename = "cashCcy")]
    pub cash_currency: Option<String>,
    /// Increment value for cash quantity
    #[serde(rename = "cashQtyIncr")]
    pub cash_quantity_increment: Option<Decimal>,
    /// Price Magnifier
    #[serde(rename = "priceMagnifier")]
    pub price_magnifier: Option<Decimal>,
    /// trading negative price support
    #[serde(rename = "negativeCapable")]
    pub negative_capable: Option<bool>,
    /// Price increment value
    #[serde(rename = "increment")]
    pub increment: Option<Decimal>,
    /// Number of digits for price increment
    #[serde(rename = "incrementDigits")]
    pub increment_digits: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityTradingRuleAndInfo {
    /// Classification of Financial Instrument codes
    #[serde(rename = "cfi_code")]
    pub cfi_code: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "cusip")]
    pub cusip: Option<String>,
    /// Expiration Date in the format YYYYMMDD
    #[serde(rename = "expiry_full")]
    pub expiry_full: Option<i64>,
    /// IBKRs contract identifier
    #[serde(rename = "con_id")]
    pub conid: Option<i64>,
    /// Date on which the underlying transaction settles if the option is exercised
    #[serde(rename = "maturity_date")]
    pub maturity_date: Option<i64>,
    /// Specific group of companies or businesses.
    #[serde(rename = "industry")]
    pub industry: Option<String>,
    /// Asset Class of the contract
    #[serde(rename = "instrument_type")]
    pub instrument_type: Option<String>,
    /// Designation of the contract
    #[serde(rename = "trading_class")]
    pub trading_class: Option<String>,
    /// Comma separated list of exchanges or trading venues
    #[serde(rename = "valid_exchanges")]
    pub valid_exchanges: Option<String>,
    /// Allowed to sell shares that you own
    #[serde(rename = "allow_sell_long")]
    pub allow_sell_long: Option<bool>,
    /// Supports zero commission trades
    #[serde(rename = "is_zero_commission_security")]
    pub is_zero_commission_security: Option<bool>,
    /// Contracts symbol from primary exchange. For options it is the OCC symbol.
    #[serde(rename = "local_symbol")]
    pub local_symbol: Option<String>,
    #[serde(rename = "classifier")]
    pub classifier: Option<String>,
    /// Currency contract trades in
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Formatted contract parameters
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// IBKRs contract identifier for the underlying instrument
    #[serde(rename = "underlying_con_id")]
    pub underlying_conid: Option<i64>,
    /// Provides trading outside of Regular Trading Hours
    #[serde(rename = "r_t_h")]
    pub regular_trading_hours: Option<bool>,
    /// numerical value of each point of price movement
    #[serde(rename = "multiplier")]
    pub multiplier: Option<String>,
    /// fixed price at which the owner of the option buys or sells the underlying
    #[serde(rename = "strike")]
    pub strike: Option<String>,
    /// Put or Call of the option
    #[serde(rename = "right")]
    pub right: Option<String>,
    /// Legal entity for underlying contract
    #[serde(rename = "underlying_issuer")]
    pub underlying_issuer: Option<String>,
    /// Month the contract must be satisfied by making or accepting delivery
    #[serde(rename = "contract_month")]
    pub contract_month: Option<String>,
    /// Contracts company name
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    /// Support IBKRs SMART routing
    #[serde(rename = "smart_available")]
    pub smart_available: Option<bool>,
    /// Primary Exchange, Routing or Trading Venue
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    #[serde(rename = "rules")]
    pub rules: Option<TradingRule>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityTradingRule {
    #[serde(rename = "algoEligible")]
    pub algo_eligible: Option<bool>,
    /// List of Accounts that can be traded
    #[serde(rename = "canTradeAcctIds")]
    pub can_trade_account_ids: Option<Vec<String>>,
    /// Returns a description on any errors with order presets
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// list of available order types
    #[serde(rename = "orderTypes")]
    pub order_types: Option<Vec<String>>,
    /// order types that support IB Algos
    #[serde(rename = "ibalgoTypes")]
    pub ib_algorithm_types: Option<Vec<String>>,
    // order types that support fractional trades
    #[serde(rename = "fraqTypes")]
    pub fractional_types: Option<Vec<String>>,
    /// order types that support cash quantity trades
    #[serde(rename = "cqtTypes")]
    pub cqt_types: Option<Vec<String>>,
    /// If object returned will provide the defaults based on user settings
    #[serde(rename = "orderDefaults")]
    pub order_defaults: Option<HashMap<String, OrderDefault>>,
    /// order types that support outside of regular trading hours
    #[serde(rename = "orderTypesOutside")]
    pub order_types_outside: Option<Vec<String>>,
    /// Default quantity
    #[serde(rename = "defaultSize")]
    pub default_size: Option<Decimal>,
    /// cash value
    #[serde(rename = "cashSize")]
    pub cash_size: Option<Decimal>,
    /// increment quantity value
    #[serde(rename = "sizeIncrement")]
    pub size_increment: Option<Decimal>,
    /// Time in Force values, formatted with o for supporting Outside regular trading hours and a for Algo trading
    #[serde(rename = "tifTypes")]
    pub tif_types: Option<Vec<String>>,
    /// Default time in force value
    #[serde(rename = "defaultTIF")]
    pub default_time_in_force: Option<String>,
    /// Limit price
    #[serde(rename = "limitPrice")]
    pub limit_price: Option<Decimal>,
    /// Stop price
    #[serde(rename = "stopprice")]
    pub stopprice: Option<Decimal>,
    /// Order origin designation for US securities options and Options Clearing Corporation
    #[serde(rename = "orderOrigination")]
    pub order_origination: Option<Decimal>,
    /// order preview required
    #[serde(rename = "preview")]
    pub preview: Option<bool>,
    #[serde(rename = "displaySize")]
    pub display_size: Option<Decimal>,
    /// decimal places for fractional order size
    #[serde(rename = "fraqInt")]
    pub fractional_order_size: Option<Decimal>,
    /// Cash currency for the contract
    #[serde(rename = "cashCcy")]
    pub cash_currency: Option<String>,
    /// Increment value for cash quantity
    #[serde(rename = "cashQtyIncr")]
    pub cash_quantity_increment: Option<Decimal>,
    /// Price Magnifier
    #[serde(rename = "priceMagnifier")]
    pub price_magnifier: Option<Decimal>,
    /// trading negative price support
    #[serde(rename = "negativeCapable")]
    pub negative_capable: Option<bool>,
    /// Price increment value
    #[serde(rename = "increment")]
    pub increment: Option<Decimal>,
    /// Number of digits for price increment
    #[serde(rename = "incrementDigits")]
    pub increment_digits: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetInfoAndRulesByConIdRequest {
    pub conid: i64,
    pub is_buy: bool,
}

pub type GetInfoAndRulesByConIdResponse = SecurityTradingRuleAndInfo;
pub type GetContractRulesResponse = SecurityTradingRule;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetContractRulesRequest {
    /// IBKR contract identifier
    #[serde(rename = "conid")]
    pub conid: i64,
    /// Side of the market rules apply too. Set to **true** for Buy Orders, set to **false** for Sell Orders
    #[serde(rename = "isBuy")]
    pub is_buy: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContractAlgorithmsParameter {
    /// The algo parameter
    #[serde(rename = "id")]
    pub id: String,
    /// If true a value must be entered.
    #[serde(rename = "required")]
    pub required: Option<bool>,
    /// Descriptive name of the parameter.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Format of the parameter.
    #[serde(rename = "valueClassName")]
    pub value_class_name: String,
    /// Smallest value, only applies to parameters with valueClassName=Double.
    #[serde(rename = "minValue")]
    pub min_value: Option<Decimal>,
    /// Largest value, only applies to parameters with valueClassName=Double.
    #[serde(rename = "maxValue")]
    pub max_value: Option<Decimal>,
    /// User configured preset for this parameter.
    #[serde(rename = "defaultValue")]
    pub default_value: Option<bool>,
    /// The list of choices
    #[serde(rename = "legalStrings")]
    pub legal_strings: Option<String>,
    /// Detailed description of the parameter.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The order in UI, used when building dynamic UI so that more important parameters are presented first.
    #[serde(rename = "guiRank")]
    pub gui_rank: Option<i64>,
    /// If true, must specify parameter using market rule format. Only applies to parameters with valueClassName=Double.
    #[serde(rename = "priceMarketRule")]
    pub price_market_rule: Option<bool>,
    /// The rules that UI should apply to algo parameters depending on chosen order type:  * MKT:speedUp:=:no - hide SpeedUp param when MKT is chosen for order type.  * LMT:strategyType:<>:empty - strategyType param cannot be empty when LMT is chosen for order type.  * MKT:strategyType:=:Marketable - set strategyType param to Marketable and disable (no other choice) when MKT is chosen for order type.
    #[serde(rename = "enabledConditions")]
    pub enabled_conditions: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AlgorithmParametersObject {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<ContractAlgorithmsParameter>>,
}

pub type GetIBAlgorithmParametersResponse = AlgorithmParametersObject;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetIBAlgorithmParametersRequest {
    pub conid: i64,
    pub algos: Option<String>,
    pub add_description: Option<String>,
    pub add_params: Option<String>,
}

pub type FuturesContracts = HashMap<String, Vec<FuturesContract>>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesContract {
    pub conid: i64,
    pub expiration_date: String,
    #[serde(rename = "ltd")]
    pub last_trading_day: String,
    pub symbol: String,
    pub underlying_conid: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetFuturesBySymbolRequest {
    pub symbols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityDefinitionsRequest {
    /// underlying contract id
    pub underlying_conid: i64,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecurityDefinition {
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
    pub strike: Option<Decimal>,
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
    pub maturity_date: Option<String>,
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

pub type SecurityDefinitionsResponse = Vec<SecurityDefinition>;
