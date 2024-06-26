use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::definition::TickType;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetMarketDataRequest {
    /// list of conids
    pub conid_list: Vec<i64>,
    /// time period since which updates are required. uses epoch time with milliseconds.
    pub since: Option<i64>,
    /// list of fields
    pub fields: Option<Vec<TickType>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarketHistoryBarData {
    /// Time - Formatted in unix time in ms.
    #[serde(rename = "t")]
    pub time: Option<i64>,
    /// Open - First price returned for bar value.
    #[serde(rename = "o")]
    pub open: Option<Decimal>,
    /// Close - Last price returned for bar value.
    #[serde(rename = "c")]
    pub close: Option<Decimal>,
    /// High - High price returned for bar value.
    #[serde(rename = "h")]
    pub high: Option<Decimal>,
    /// Low - Last price returned for bar value.
    #[serde(rename = "l")]
    pub low: Option<Decimal>,
    /// Volume - Traded volume for bar value.
    #[serde(rename = "v")]
    pub volume: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataHistory {
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// companyName
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// priceFactor is price increment obtained from display rule
    #[serde(rename = "priceFactor")]
    pub price_factor: Option<Decimal>,
    /// start date time in the format YYYYMMDD-HH:mm:ss
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// High value during this time series with format %h/%v/%t.
    /// %h is the high price (scaled by priceFactor),
    /// %v is volume (volume factor will always be 100 (reported volume = actual volume/100))
    /// %t is minutes from start time of the chart
    #[serde(rename = "high")]
    pub high: Option<String>,
    /// Low value during this time series with format %l/%v/%t.
    /// %l is the low price (scaled by priceFactor),
    /// %v is volume (volume factor will always be 100 (reported volume = actual volume/100))
    /// %t is minutes from start time of the chart
    #[serde(rename = "low")]
    pub low: Option<String>,
    /// The duration for the historical data request
    #[serde(rename = "timePeriod")]
    pub time_period: Option<String>,
    /// The number of seconds in a bar
    #[serde(rename = "barLength")]
    pub bar_length: Option<Decimal>,
    /// Market Data Availability. The field may contain two chars.
    /// The first char is the primary code: S = Streaming, R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed.
    /// The second char is the secondary code: P = Snapshot Available, p = Consolidated.
    #[serde(rename = "mdAvailability")]
    pub market_data_availability: Option<String>,
    /// The time it takes, in milliseconds, to process the historical data request
    #[serde(rename = "mktDataDelay")]
    pub market_data_delay: Option<i32>,
    /// The historical data returned includes outside of regular trading hours
    #[serde(rename = "outsideRth")]
    pub outside_regular_trading_hours: Option<bool>,
    /// The number of seconds in the trading day
    #[serde(rename = "tradingDayDuration")]
    pub trading_day_duration: Option<i32>,
    #[serde(rename = "volumeFactor")]
    pub volume_factor: Option<Decimal>,
    #[serde(rename = "priceDisplayRule")]
    pub price_display_rule: Option<i32>,
    #[serde(rename = "priceDisplayValue")]
    pub price_display_value: Option<String>,
    #[serde(rename = "negativeCapable")]
    pub negative_capable: Option<bool>,
    #[serde(rename = "messageVersion")]
    pub message_version: Option<i32>,
    #[serde(rename = "data")]
    pub data: Option<Vec<MarketHistoryBarData>>,
    /// total number of points
    #[serde(rename = "points")]
    pub points: Option<i32>,
    #[serde(rename = "travelTime")]
    pub travel_time: Option<i32>,
}

pub struct GetMarketDataHistoryRequest {
    /// contract id
    pub conid: i64,
    /// Exchange of the conid (e.g. ISLAND, NYSE, etc.). Default value is empty which corresponds to primary exchange of the conid.
    pub exchange: Option<String>,
    /// available time period-- {1-30}min, {1-8}h, {1-1000}d, {1-792}w, {1-182}m, {1-15}y
    pub period: String,
    /// possible value-- 1min, 2min, 3min, 5min, 10min, 15min, 30min, 1h, 2h, 3h, 4h, 8h, 1d, 1w, 1m
    pub bar: Option<String>,
    /// For contracts that support it, will determine if historical data includes outside of regular trading hours.
    pub outside_regular_trading_hours: Option<bool>,
    pub start_time: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnsubscribeAllMarketDataResponse {
    /// true means market data is cancelled, false means it is not.
    #[serde(rename = "confirmed")]
    pub confirmed: Option<bool>,
}

pub struct UnsubscribeMarketDataRequest {
    pub conid: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnsubscribeMarketDataResponse {
    /// true means market data is cancelled, false means it is not.
    #[serde(rename = "confirmed")]
    pub confirmed: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarketHistoryDataBars {
    /// First price returned for bar value.
    #[serde(rename = "open")]
    pub open: Option<Decimal>,
    /// Start Time in the format YYYYMMDD.
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// Start Time Value - Formatted in unix time in ms.
    #[serde(rename = "startTimeVal")]
    pub start_time_val: Option<i32>,
    /// End Time in the format YYYYMMDD.
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// End Time Value - Formatted in unix time in ms.
    #[serde(rename = "endTimeVal")]
    pub end_time_val: Option<i32>,
    /// total number of data points.
    #[serde(rename = "points")]
    pub points: Option<i32>,
    #[serde(rename = "data")]
    pub data: Option<Vec<MarketHistoryBarData>>,
    /// If 0 then data is returned in real time. Otherwise will return the number of seconds history data is delayed.
    #[serde(rename = "mktDataDelay")]
    pub market_data_delay: Option<i32>,
}

pub struct GetMarketDataHistoryBetaRequest {
    /// contract id
    pub conid: i64,
    /// Enum: "min" "h" "d" "w" "m" "y"
    /// Time period for history request.
    ///
    /// min: Minutes
    /// h: Hours
    /// d: Days
    /// w: Weeks
    /// m: Months
    /// y: Years
    pub period: String,
    /// Enum: "min" "h" "d" "w" "m"
    ///
    /// Duration of time for each candlestick bar.
    ///
    /// min: Minutes
    /// h: Hours
    /// d: Days
    /// w: Weeks
    /// m: Months
    pub bar: Option<String>,
    /// For contracts that support it, will determine if history data includes outside of regular trading hours.
    pub outside_regular_trading_hours: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetMarketDataHistoryBetaResponse {
    #[serde(rename = "bars")]
    pub bars: MarketHistoryDataBars,
}

#[mixin::declare]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarketDataMixin {
    /// Last Price - The last price at which the contract traded. May contain one of the following prefixes:
    /// * C - Previous day's closing price.
    /// * H - Trading has halted.
    #[serde(rename = "31")]
    pub last_price: Option<String>,
    ///Symbol
    #[serde(rename = "55")]
    pub symbol: Option<String>,
    ///Text
    #[serde(rename = "58")]
    pub text: Option<String>,
    /// High - Current day high price
    #[serde(rename = "70")]
    pub high_price: Option<Decimal>,
    /// Low - Current day low price
    #[serde(rename = "71")]
    pub low_price: Option<Decimal>,
    /// Market Value - The current market value of  your position in the security. Market Value is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "73")]
    pub market_value: Option<Decimal>,
    /// Avg Price - The average price of the position.
    #[serde(rename = "74")]
    pub avg_price: Option<Decimal>,
    ///Unrealized PnL - Unrealized profit or loss. Unrealized PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "75")]
    pub unrealized_pnl: Option<String>,
    ///Formatted position
    #[serde(rename = "76")]
    pub formatted_position: Option<String>,
    ///Formatted Unrealized PnL
    #[serde(rename = "77")]
    pub formatted_unrealized_pnl: Option<String>,
    ///Daily PnL - Your profit or loss of the day since prior close. Daily PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "78")]
    pub daily_pnl: Option<String>,
    ///Realized PnL - Realized profit or loss. Realized PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "79")]
    pub realized_pnl: Option<String>,
    ///Unrealized PnL % - Unrealized profit or loss expressed in percentage.
    #[serde(rename = "80")]
    pub unrealized_pnl_pct: Option<String>,
    /// Change - The difference between the last price and the close on the previous trading day
    #[serde(rename = "82")]
    pub change: Option<Decimal>,
    /// Change % - The difference between the last price and the close on the previous trading day in percentage.
    #[serde(rename = "83")]
    pub change_percent: Option<Decimal>,
    /// Bid Price - The highest-priced bid on the contract.
    #[serde(rename = "84")]
    pub bid_price: Option<Decimal>,
    /// Ask Size - The number of contracts or shares offered at the ask price. For US stocks, the number displayed is divided by 100.
    #[serde(rename = "85")]
    pub ask_size: Option<String>,
    /// Ask Price - The lowest-priced offer on the contract.
    #[serde(rename = "86")]
    pub ask_price: Option<Decimal>,
    /// Volume - Volume for the day, formatted with 'K' for thousands or 'M' for millions. For higher precision volume refer to field 7762.
    #[serde(rename = "87")]
    pub volume: Option<String>,
    /// Bid Size - The number of contracts or shares bid for at the bid price. For US stocks, the number displayed is divided by 100.
    #[serde(rename = "88")]
    pub bid_size: Option<String>,
    /// Exchange
    #[serde(rename = "6004")]
    pub exchange: Option<String>,
    /// Conid - Contract identifier from IBKR's database.
    #[serde(rename = "6008")]
    pub conid_6008: Option<String>,
    /// SecType - The asset class of the instrument.
    #[serde(rename = "6070")]
    pub sec_type: Option<String>,
    /// Months
    #[serde(rename = "6072")]
    pub months: Option<String>,
    /// Regular Expiry
    #[serde(rename = "6073")]
    pub regular_expiry: Option<String>,
    /// Field value of the server_id. Returns the request’s identifier.
    #[serde(rename = "6119")]
    pub server_id_6119: Option<String>,
    ///Underlying Conid. Use /trsrv/secdef to get more information about the security
    #[serde(rename = "6457")]
    pub underlying_conid: Option<String>,
    ///Service Params.
    #[serde(rename = "6508")]
    pub service_params: Option<String>,
    /// Market Data Availability. The field may contain three chars.
    /// First char defines: R = RealTime, D = Delayed, Z = Frozen, Y = Frozen Delayed, N = Not Subscribed.
    /// Second char defines: P = Snapshot, p = Consolidated.
    /// Third char defines: B = Book
    /// * RealTime - Data is relayed back in real time without delay, market data subscription(s) are required.
    /// * Delayed - Data is relayed back 15-20 min delayed.   * Frozen - Last recorded data at market close, relayed back in real time.
    /// * Frozen Delayed - Last recorded data at market close, relayed back delayed.
    /// * Not Subscribed - User does not have the required market data subscription(s) to relay back either real time or delayed data.
    /// * Snapshot - Snapshot request is available for contract.   * Consolidated - Market data is aggregated across multiple exchanges or venues.
    /// * Book - Top of the book data is available for contract.
    #[serde(rename = "6509")]
    pub market_data_availability: Option<String>,
    ///Company name
    #[serde(rename = "7051")]
    pub company_name: Option<String>,
    /// Ask Exch - Displays the exchange(s) offering the SMART price.
    /// A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    #[serde(rename = "7057")]
    pub ask_exchange: Option<String>,
    /// Last Exch - Displays the exchange(s) offering the SMART price.
    /// A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    #[serde(rename = "7058")]
    pub last_exchange_7058: Option<String>,
    /// Last Size - The number of unites traded at the last price
    #[serde(rename = "7059")]
    pub last_size: Option<Decimal>,
    /// Bid Exch - Displays the exchange(s) offering the SMART price.
    /// A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    #[serde(rename = "7068")]
    pub bid_exchange: Option<String>,
    ///Implied Vol./Hist. Vol % - The ratio of the implied volatility over the historical volatility, expressed as a percentage.
    #[serde(rename = "7084")]
    pub implied_vol_hist_vol: Option<String>,
    ///Put/Call Interest - Put option open interest/call option open interest for the trading day.
    #[serde(rename = "7085")]
    pub put_call_interest: Option<String>,
    ///Put/Call Volume - Put option volume/call option volume for the trading day.
    #[serde(rename = "7086")]
    pub put_call_volume: Option<String>,
    ///Hist. Vol. % - 30-day real-time historical volatility.
    #[serde(rename = "7087")]
    pub hist_vol: Option<String>,
    ///Hist. Vol. Close % - Shows the historical volatility based on previous close price.
    #[serde(rename = "7088")]
    pub hist_vol_close: Option<String>,
    ///Opt. Volume - Option Volume
    #[serde(rename = "7089")]
    pub opt_volume: Option<String>,
    ///Conid + Exchange
    #[serde(rename = "7094")]
    pub con_id_exchange: Option<String>,
    ///canBeTraded - If contract is a trade-able instrument. Returns 1(true) or 0(false).
    #[serde(rename = "7184")]
    pub can_be_traded: Option<String>,
    /// IV Rank
    #[serde(rename = "7195")]
    pub iv_rank_7195: Option<String>,
    /// IV Rank
    #[serde(rename = "7196")]
    pub iv_rank_7196: Option<String>,
    /// IV Rank
    #[serde(rename = "7197")]
    pub iv_rank_7197: Option<String>,
    /// IV Percentile
    #[serde(rename = "7198")]
    pub iv_percentile_7198: Option<String>,
    /// IV Percentile
    #[serde(rename = "7199")]
    pub iv_percentile_7199: Option<String>,
    /// IV Percentile
    #[serde(rename = "7200")]
    pub iv_percentile_7200: Option<String>,
    /// IV High Low
    #[serde(rename = "7201")]
    pub iv_high_low_7201: Option<String>,
    /// IV High Low
    #[serde(rename = "7202")]
    pub iv_high_low_7202: Option<String>,
    /// IV High Low
    #[serde(rename = "7203")]
    pub iv_high_low_7203: Option<String>,
    /// IV High Low
    #[serde(rename = "7204")]
    pub iv_high_low_7204: Option<String>,
    /// IV High Low
    #[serde(rename = "7205")]
    pub iv_high_low_7205: Option<String>,
    /// IV High Low
    #[serde(rename = "7206")]
    pub iv_high_low_7206: Option<String>,
    /// HV Rank
    #[serde(rename = "7207")]
    pub hv_rank_7207: Option<String>,
    /// HV Rank
    #[serde(rename = "7208")]
    pub hv_rank_7208: Option<String>,
    /// HV Rank
    #[serde(rename = "7209")]
    pub hv_rank_7209: Option<String>,
    /// HV Percentile
    #[serde(rename = "7210")]
    pub hv_percentile_7210: Option<String>,
    /// HV Percentile
    #[serde(rename = "7211")]
    pub hv_percentile_7211: Option<String>,
    /// HV Percentile
    #[serde(rename = "7212")]
    pub hv_percentile_7212: Option<String>,
    ///Contract Description
    #[serde(rename = "7219")]
    pub contract_description_7219: Option<String>,
    ///Contract Description
    #[serde(rename = "7220")]
    pub contract_description_7220: Option<String>,
    ///Listing Exchange
    #[serde(rename = "7221")]
    pub listing_exchange: Option<String>,
    /// HV High Low
    #[serde(rename = "7245")]
    pub hv_high_low_7245: Option<String>,
    /// HV High Low
    #[serde(rename = "7246")]
    pub hv_high_low_7246: Option<String>,
    /// HV High Low
    #[serde(rename = "7247")]
    pub hv_high_low_7247: Option<String>,
    /// HV High Low
    #[serde(rename = "7248")]
    pub hv_high_low_7248: Option<String>,
    /// HV High Low
    #[serde(rename = "7249")]
    pub hv_high_low_7249: Option<String>,
    /// HV High Low
    #[serde(rename = "7263")]
    pub hv_high_low_7263: Option<String>,
    /// ESG
    #[serde(rename = "7264")]
    pub esg_7264: Option<String>,
    /// ESG
    #[serde(rename = "7265")]
    pub esg_7265: Option<String>,
    /// ESG
    #[serde(rename = "7266")]
    pub esg_7266: Option<String>,
    /// ESG
    #[serde(rename = "7267")]
    pub esg_7267: Option<String>,
    /// ESG
    #[serde(rename = "7268")]
    pub esg_7268: Option<String>,
    /// ESG
    #[serde(rename = "7269")]
    pub esg_7269: Option<String>,
    /// ESG
    #[serde(rename = "7271")]
    pub esg_7271: Option<String>,
    /// ESG
    #[serde(rename = "7272")]
    pub esg_7272: Option<String>,
    /// ESG
    #[serde(rename = "7273")]
    pub esg_7273: Option<String>,
    /// ESG
    #[serde(rename = "7274")]
    pub esg_7274: Option<String>,
    /// ESG
    #[serde(rename = "7275")]
    pub esg_7275: Option<String>,
    /// ESG
    #[serde(rename = "7276")]
    pub esg_7276: Option<String>,
    /// ESG
    #[serde(rename = "7277")]
    pub esg_7277: Option<String>,
    /// Industry - Displays the type of industry under which the underlying company can be categorized.
    #[serde(rename = "7280")]
    pub industry: Option<String>,
    /// Category - Displays a more detailed level of description within the industry under which the underlying company can be categorized.
    #[serde(rename = "7281")]
    pub category: Option<String>,
    /// Average Volume - The average daily trading volume over 90 days.
    #[serde(rename = "7282")]
    pub average_volume: Option<String>,
    /// Option Implied Vol. % - A prediction of how volatile an underlying will be in the future.
    /// At the market volatility estimated for a maturity thirty calendar days forward of the current trading day, and based on option prices from two consecutive expiration months.      
    #[serde(rename = "7283")]
    pub option_implied_volume: Option<String>,
    /// Historic Volume (30d)
    #[serde(rename = "7284")]
    pub historic_volume: Option<String>,
    /// Put/Call Ratio
    #[serde(rename = "7285")]
    pub put_call_ratio: Option<String>,
    /// Dividend Amount - Displays the amount of the next dividend.
    #[serde(rename = "7286")]
    pub dividend_amount: Option<Decimal>,
    /// Dividend Yield % - This value is the toal of the expected dividend payments over the next twelve months per share divided by the Current Price and is expressed as a percentage.
    /// For derivatives, this displays the total of the expected dividend payments over the expiry date.
    #[serde(rename = "7287")]
    pub dividend_yield: Option<String>,
    /// Ex-date of the dividend
    #[serde(rename = "7288")]
    pub ex_date_of_dividend: Option<String>,
    /// Market Cap
    #[serde(rename = "7289")]
    pub market_cap: Option<String>,
    /// P/E
    #[serde(rename = "7290")]
    pub pe: Option<String>,
    /// EPS
    #[serde(rename = "7291")]
    pub eps: Option<String>,
    /// Cost Basis - Your current position in this security multiplied by the average price and multiplier.
    #[serde(rename = "7292")]
    pub cost_basis: Option<String>,
    /// 52 Week High - The highest price for the past 52 weeks.
    #[serde(rename = "7293")]
    pub _52_week_high: Option<String>,
    /// 52 Week Low - The lowest price for the past 52 weeks.
    #[serde(rename = "7294")]
    pub _52_week_low: Option<String>,
    /// Open - Today's opening price.
    #[serde(rename = "7295")]
    pub open: Option<Decimal>,
    /// Close - Today's closing price.
    #[serde(rename = "7296")]
    pub close: Option<Decimal>,
    ///Delta - The ratio of the change in the price of the option to the corresponding change in the price of the underlying.
    #[serde(rename = "7308")]
    pub delta: Option<String>,
    ///Gamma - The rate of change for the delta with respect to the underlying asset's price.
    #[serde(rename = "7309")]
    pub gamma: Option<String>,
    ///Theta - A measure of the rate of decline the value of an option due to the passage of time.
    #[serde(rename = "7310")]
    pub theta: Option<String>,
    ///Vega - The amount that the price of an option changes compared to a 1% change in the volatility.
    #[serde(rename = "7311")]
    pub vega: Option<String>,
    /// Reuters Fundamentals
    #[serde(rename = "7331")]
    pub reuters_fundamentals_7331: Option<String>,
    /// ESG
    #[serde(rename = "7370")]
    pub esg_7370: Option<String>,
    /// ESG
    #[serde(rename = "7371")]
    pub esg_7371: Option<String>,
    /// ESG
    #[serde(rename = "7372")]
    pub esg_7372: Option<String>,
    ///Opt. Volume Change % - Today's option volume as a percentage of the average option volume.
    #[serde(rename = "7607")]
    pub opt_volume_change: Option<String>,
    ///Implied Vol. % - The implied volatility for the specific strike of the option in percentage. To query the Option Implied Vol. % from the underlying refer to field 7283.
    #[serde(rename = "7633")]
    pub implied_vol: Option<String>,
    /// Mark - The mark price is, the ask price if ask is less than last price, the bid price if bid is more than the last price, otherwise it's equal to last price
    #[serde(rename = "7635")]
    pub mark: Option<String>,
    /// shortable inventory
    #[serde(rename = "7636")]
    pub shortable_inventory: Option<String>,
    /// Fee rebate rate
    #[serde(rename = "7637")]
    pub fee_rebate_rate: Option<String>,
    ///Option Open Interest
    #[serde(rename = "7638")]
    pub option_open_interest: Option<String>,
    ///% of Mark Value - Displays the market value of the contract as a percentage of the total market value of the account.
    //Mark Value is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "7639")]
    pub pct_of_mark_value: Option<String>,
    /// Shortable - Describes the level of difficulty with which the security can be sold short.
    #[serde(rename = "7644")]
    pub shortable: Option<String>,
    ///Morningstar Rating - Displays Morningstar Rating provided value. Requires [Morningstar](https://www.interactivebrokers.com/en/index.php?f=14262) subscription.
    #[serde(rename = "7655")]
    pub morningstar_rating: Option<String>,
    ///Dividends - This value is the total of the expected dividend payments over the next twelve months per share.
    #[serde(rename = "7671")]
    pub dividends: Option<String>,
    ///Dividends TTM - This value is the total of the expected dividend payments over the last twelve months per share.
    #[serde(rename = "7672")]
    pub dividends_ttm: Option<String>,
    /// EMA(200) - Exponential moving average (N=200).
    #[serde(rename = "7674")]
    pub ema_200: Option<String>,
    /// EMA(100) - Exponential moving average (N=100).
    #[serde(rename = "7675")]
    pub ema_100: Option<String>,
    /// EMA(50) - Exponential moving average (N=50).
    #[serde(rename = "7676")]
    pub ema_50: Option<String>,
    /// EMA(20) - Exponential moving average (N=20).
    #[serde(rename = "7677")]
    pub ema_20: Option<String>,
    ///Price/EMA(200) - Price to Exponential moving average (N=200) ratio -1, displayed in percents.
    #[serde(rename = "7677")]
    pub price_ema_200: Option<String>,
    ///Price/EMA(100) - Price to Exponential moving average (N=100) ratio -1, displayed in percents.
    #[serde(rename = "7677")]
    pub price_ema_100: Option<String>,
    ///Price/EMA(50) - Price to Exponential moving average (N=50) ratio -1, displayed in percents.
    #[serde(rename = "7677")]
    pub price_ema_50: Option<String>,
    /// Price/EMA(20) - Price to Exponential moving average (N=20) ratio -1, displayed in percents.
    #[serde(rename = "7681")]
    pub price_ema_20: Option<String>,
    ///Change Since Open - The difference between the last price and the open price.
    #[serde(rename = "7682")]
    pub change_since_open: Option<String>,
    ///Upcoming Event - Shows the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7683")]
    pub upcoming_event: Option<String>,
    ///Upcoming Event Date - The date of the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7684")]
    pub upcoming_event_date: Option<String>,
    ///Upcoming Analyst Meeting - The date and time of the next scheduled analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7685")]
    pub upcoming_analyst_meting: Option<String>,
    ///Upcoming Earnings - The date and time of the next scheduled earnings/earnings call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7686")]
    pub upcoming_earnings: Option<String>,
    ///Upcoming Misc Event - The date and time of the next shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7687")]
    pub upcoming_misc_event: Option<String>,
    ///Recent Analyst Meeting - The date and time of the most recent analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7688")]
    pub recent_analyst_meeting: Option<String>,
    ///Recent Earnings - The date and time of the most recent earnings/earning call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7689")]
    pub recent_earnings: Option<String>,
    ///Recent Misc Event - The date and time of the most recent shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7690")]
    pub recent_misc_event: Option<String>,
    ///Probability of Max Return - Customer implied probability of maximum potential gain.
    #[serde(rename = "7694")]
    pub probability_of_max_return: Option<String>,
    ///Break Even - Break even points
    #[serde(rename = "7695")]
    pub break_even: Option<String>,
    ///SPX Delta - Beta Weighted Delta is calculated using the formula; Delta x dollar adjusted beta, where adjusted beta is adjusted by the ratio of the close price.
    #[serde(rename = "7696")]
    pub spx_delta: Option<String>,
    ///Futures Open Interest - Total number of outstanding futures contracts
    #[serde(rename = "7697")]
    pub futures_open_interest: Option<String>,
    /// Last Yield - Implied yield of the bond if it is purchased at the current last price. Last yield is calculated using the Last price on all possible call dates. It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates. The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher.
    #[serde(rename = "7698")]
    pub last_yield: Option<String>,
    /// Bid Yield - Implied yield of the bond if it is purchased at the current bid price. Bid yield is calculated using the Ask on all possible call dates.
    /// It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates.
    /// The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions).
    /// Yield to worse may be the same as yield to maturity but never higher.
    #[serde(rename = "7699")]
    pub bid_yield: Option<String>,
    ///Probability of Max Return - Customer implied probability of maximum potential gain.
    #[serde(rename = "7700")]
    pub probability_of_max_return7700: Option<String>,
    ///Probability of Max Loss - Customer implied probability of maximum potential loss.
    #[serde(rename = "7702")]
    pub probability_of_max_loss: Option<String>,
    ///Profit Probability - Customer implied probability of any gain.
    #[serde(rename = "7703")]
    pub profit_probability: Option<String>,
    ///Organization Type
    #[serde(rename = "7704")]
    pub organization_type: Option<String>,
    ///Debt Class
    #[serde(rename = "7705")]
    pub debt_class: Option<String>,
    ///Ratings - Ratings issued for bond contract.
    #[serde(rename = "7706")]
    pub ratings: Option<String>,
    ///Bond State Code
    #[serde(rename = "7707")]
    pub bond_state_code: Option<String>,
    ///Bond Type
    #[serde(rename = "7708")]
    pub bond_type: Option<String>,
    ///Last Trading Date
    #[serde(rename = "7714")]
    pub last_trading_date: Option<String>,
    ///Issue Date
    #[serde(rename = "7715")]
    pub issue_date: Option<String>,
    /// Beta - Beta is against standard index.
    #[serde(rename = "7718")]
    pub beta: Option<Decimal>,
    /// Ask Yield - Implied yield of the bond if it is purchased at the current offer. Ask yield is calculated using the Bid on all possible call dates.
    /// It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates.
    /// The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions).
    /// Yield to worse may be the same as yield to maturity but never higher.
    #[serde(rename = "7720")]
    pub ask_yield: Option<String>,
    /// Prior Close
    #[serde(rename = "7741")]
    pub prior_close: Option<Decimal>,
    /// Reuters Fundamentals
    #[serde(rename = "7743")]
    pub reuters_fundamentals_7743: Option<String>,
    /// ESG
    #[serde(rename = "7761")]
    pub esg: Option<String>,
    /// Volume - Volume for the day, formatted with 'K' for thousands or 'M' for millions.
    #[serde(rename = "7762")]
    pub volume_long: Option<Decimal>,
    ///hasTradingPermissions - if user has trading permissions for specified contract. Returns 1(true) or 0(false).
    #[serde(rename = "7768")]
    pub has_trading_permissions: Option<String>,
    /// 26 Week High - The highest price for the past 26 weeks.
    #[serde(rename = "7992")]
    pub _26_week_high: Option<String>,
    /// 26 Week Low - The lowest price for the past 26 weeks.
    #[serde(rename = "7993")]
    pub _26_week_low: Option<String>,
    /// 13 Week High - The highest price for the past 13 weeks.
    #[serde(rename = "7994")]
    pub _13_week_high: Option<String>,
    /// 13 Week Low - The lowest price for the past 13 weeks.
    #[serde(rename = "7995")]
    pub _13_week_low: Option<String>,
}

#[mixin::insert(MarketDataMixin)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarketDataSnapshot {
    /// IBKR Contract identifier
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// minimum price increment
    #[serde(rename = "minTick")]
    pub min_tick: Option<Decimal>,
    /// Color for Best Bid/Offer Exchange in hex code
    #[serde(rename = "BboExchange")]
    pub bbo_exchange: Option<String>,
    /// If market data field values return delayed
    #[serde(rename = "HasDelayed")]
    pub has_delayed: Option<bool>,
    /// minimum size increment
    #[serde(rename = "sizeMinTick")]
    pub size_min_tick: Option<Decimal>,
    #[serde(rename = "BestEligible")]
    pub best_eligible: Option<Decimal>,
    #[serde(rename = "BestBidExch")]
    pub best_bid_exchange: Option<Decimal>,
    #[serde(rename = "BestAskExch")]
    pub best_ask_exchange: Option<Decimal>,
    #[serde(rename = "PreOpenBid")]
    pub pre_open_bid: Option<Decimal>,
    #[serde(rename = "LastAttribs")]
    pub last_attributes: Option<Decimal>,
    /// Base time stamp for last update in format YYYYMMDD
    #[serde(rename = "TimestampBase")]
    pub timestamp_base: Option<String>,
    #[serde(rename = "TimestampDelta")]
    pub timestamp_delta: Option<Decimal>,
    #[serde(rename = "LastExch")]
    pub last_exchange: Option<String>,
    #[serde(rename = "CloseAttribs")]
    pub close_attributes: Option<Decimal>,
}

pub type GetMarketDataSnapshotResponse = Vec<MarketDataSnapshot>;

pub struct GetMarketDataSnapshotRequest {
    /// Value: "conid@exchange:instrType"
    /// List of conids comma separated. Optional exchange and instrument type can be specified.
    ///
    /// conid: IBKR Contract Identifier
    /// exchange: Exchange or venue
    /// instrType: Instrument Type supported values: CS (Stocks), OPT (Options), FUT (Futures), FOP (Future Options), WAR (Warrants), BOND (Bonds), FUND (Mutual Funds), CASH (Forex), CFD (Contract for difference), IND (Index)
    pub conid_list: Vec<i64>,
    /// Example: fields=31&fields=84&fields=85&fields=86&fields=88
    /// list of fields separated by comma
    pub field_list: Vec<TickType>,
}

#[mixin::insert(MarketDataMixin)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarketData {
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    #[serde(rename = "conidEx")]
    pub conidex: Option<String>,
    /// Returns the epoch time of the update in a 13 character integer
    #[serde(rename = "_updated")]
    pub updated: Option<i64>,
}

pub type GetMarketDataResponse = Vec<MarketData>;
