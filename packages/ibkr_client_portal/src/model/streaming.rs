use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use super::{account::GetAccountsResponse, market_data::MarketHistoryBarData};

pub struct StreamingDataStructuredRequest {
    pub topic: String,
    pub arguments: Option<Vec<String>>,
    pub body: Option<String>,
}

impl StreamingDataStructuredRequest {
    pub fn to_message(&self) -> Message {
        let mut components = vec![self.topic.clone()];
        if let Some(arguments) = &self.arguments {
            components.append(&mut arguments.clone());
        }
        if let Some(body) = &self.body {
            components.push(body.clone());
        }
        Message::Text(components.join("+").to_owned())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum StreamingDataResponse {
    /// (blt) If there are urgent messages concerning exchange issues, system problems, and other trading information, the topic blt is sent along with the message argument and a unique identifier for the bulletin.
    Bulletins(TopicArgsResponse<BulletinsArgs>),
    /// (ntf) If there is a brief message regarding trading activity the topic ntf will be sent.
    Notifications(TopicArgsResponse<NotificationsArgs>),
    /// (sts) When initially connecting to the websocket endpoint, the topic sts will relay back the current authentication status of the user. Authentication status updates, for example those resulting from competing sessions, are also relayed back to the websocket client via this topic.
    AuthenticationStatus(TopicArgsResponse<AuthenticationStatusArgs>),
    /// (system) When initially connecting to websocket the topic system relays back a confirmation with the corresponding username. While the websocket is connecting every 10 seconds there after a heartbeat with corresponding unix time (in millisecond format) is relayed back.
    SystemConnection(SystemConnectionMessage),
    /// (sbd)
    BookTraderPriceLadder(BookTraderPriceLadderResponse),
    /// (ssd)
    AccountSummary(AccountSummaryResponse),
    /// (sld)
    AccountLedger(AccountLedgerResponse),
    /// (sor)
    OrderUpdate(OrderUpdateResponse),
    /// (spl)
    ProfitAndLossUpdate(ProfitAndLossUpdateResponse),
    /// (str)
    TradeData(TradeDataResponse),
    /// (smh)
    HistoricalData(HistoricalDataResponse),
    /// (act)
    AccountUpdate(AccountUpdateMessage),

    // TODO: smd
    ResultMessage(ResultMessageResponse),
    #[serde(skip_serializing)]
    Unknown(String),
}

impl StreamingDataResponse {
    pub fn from_str(str: &str) -> StreamingDataResponse {
        serde_json::from_str::<StreamingDataResponse>(&str)
            .unwrap_or(StreamingDataResponse::Unknown(str.to_string()))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TopicArgsResponse<T> {
    pub topic: String,
    pub args: T,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AuthenticationStatusArgs {
    ///  Returns whether the user is authenticated to the brokerage session.
    pub authenticated: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BulletinsArgs {
    /// The ID for the specific bulletin.
    pub id: String,
    /// The bulletin information.
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NotificationsArgs {
    /// The identifier for the specific notification.
    pub id: String,
    /// The body text for the affiliated notification.
    pub text: String,
    /// The title or headline for the notification.
    pub title: Option<String>,
    /// If relevant, provides a url where a user can go to read more about the notification.
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ResultMessageResponse {
    pub result: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AccountSummaryResponse {
    /// Array of JSON objects, each corresponding to an account summary value for the account.
    pub result: Vec<AccountSummaryResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SystemConnectionMessage {
    /// Equals to "system"
    pub topic: String,
    ///  Returns the username logged in with that has built the websocket.
    pub success: String,
}

pub trait ToStructuredRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest;
}

/// Subscribes to a stream of account summary messages for the specified account.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeAccountSummaryRequest {
    /// Must pass the account ID whose account summary data will be subscribed.
    pub account_id: String,
    /// Pass specific account summary data keys to receive messages concerning only those keys. Passing no named keys when opening the subscription will deliver account summary messages containing values for the selected account.
    /// Example Values: "AccruedCash-S", "ExcessLiquidity-S"
    pub keys: Vec<String>,
    /// Pass specific account summary field names to filter responses to include only these fields for the requested keys. Passing no named fields when opening the subscription will deliver all available data points for the specified account summary keys.
    /// Example Values: "currency", "monetaryValue"
    pub fields: Vec<String>,
}

impl ToStructuredRequest for SubscribeAccountSummaryRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "ssd".to_owned(),
            arguments: Option::Some(vec![self.account_id.to_owned()]),
            body: Option::Some(
                json!({
                    "keys": self.keys,
                    "fields": self.fields,
                })
                .to_string(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AccountSummaryResult {
    /// The name of the account summary value.
    pub key: String,
    /// The currency reflected by monetaryValue.
    /// Example Value: “USD”, “EUR”, “HKD”
    pub currency: String,
    /// A non-monetary value associated with the key. This may include dates, account titles, or other relevant information.
    pub value: Option<Decimal>,
    /// A monetary value associated with the key. Returned when the key pertains to pricing or balance details.
    #[serde(rename = "monetaryValue")]
    pub monetary_value: Option<Decimal>,
    /// Internal use only.
    pub severity: Option<i64>,
    /// The timestamp reflecting when the value was retrieved.
    pub timestamp: i64,
}

/// Unsubscribes the user from account summary information for the specified account.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeAccountSummaryRequest {
    /// Must pass the account ID whose account summary messages will be unsubscribed.
    pub account_id: String,
}

impl ToStructuredRequest for UnsubscribeAccountSummaryRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "usd".to_owned(),
            arguments: Option::Some(vec![self.account_id.clone()]),
            body: Option::Some("{}".to_owned()),
        }
    }
}

///  Subscribes to a stream of account ledger messages for the specified account, with contents sorted by currency.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeAccountLedgerRequest {
    /// Must pass the account ID whose ledger data will be subscribed.
    pub account_id: String,
    /// Pass specific ledger currency keys to receive messages with data only for those currencies. Passing no named keys when opening the subscription will deliver ledger messages containing values for all currencies in the selected account.
    /// Example Values: "LedgerListEUR", "LedgerListUSD", "LedgerListBASE" (for the account's base currency)
    pub keys: Vec<String>,
    /// Pass specific ledger field names to receive messages only those data points for the currencies specified in the keys argument. Passing no named fields when opening the subscription will deliver all available data points for the specified currencies.
    /// Example Values: "cashBalance", "exchangeRate"
    pub fields: Vec<String>,
}

/// A new message is published every 10 seconds until the sld topic is unsubscribed. A given message will only deliver a currency’s field data when a change occurred for that currency in the preceding interval. If no change occurred, the currency’s entry in the sld message will be “blank”, containing only the currency key and a timestamp.
///
/// Note that all currency values of JSON number type will be presented with a fractional component following a decimal point, and may also include an exponential component following an E if sufficiently large.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AccountLedgerResponse {
    /// sld+DU1234567
    pub topic: String,
    /// Array of JSON objects, with each object containing the set of key-value pairs for one currency in the account.
    pub result: Vec<AccountLedgerResult>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AccountLedgerResult {
    /// Currency identifier string in the form “LedgerListXXX”, where XXX is the three-character currency code of a currency in the requested account, or “LedgerListBASE”, corresponding to the account’s base currency.
    /// This is always returned.
    pub key: String,
    /// The timestamp reflecting when the currency’s set of values was retrieved.
    /// This is always returned.
    pub timestamp: i64,
    /// The account containing the currency position described by the accompanying data.
    #[serde(rename = "acctCode")]
    pub account_id: Option<String>,
    #[serde(rename = "cashbalance")]
    pub cash_balance: Option<Decimal>,
    #[serde(rename = "cashBalanceFXSegment")]
    pub cash_balance_fx_segment: Option<Decimal>,
    #[serde(rename = "commodityMarketValue")]
    pub commodity_market_value: Option<Decimal>,
    #[serde(rename = "corporateBondsMarketValue")]
    pub corporate_bonds_market_value: Option<Decimal>,
    pub dividends: Option<Decimal>,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: Option<Decimal>,
    pub funds: Option<Decimal>,
    #[serde(rename = "marketValue")]
    pub market_value: Option<Decimal>,
    #[serde(rename = "optionMarketValue")]
    pub option_market_value: Option<Decimal>,
    pub interest: Option<Decimal>,
    #[serde(rename = "issueOptionsMarketValue")]
    pub issue_options_market_value: Option<Decimal>,
    #[serde(rename = "moneyFunds")]
    pub money_funds: Option<Decimal>,
    #[serde(rename = "netLiquidationValue")]
    pub net_liquidation_value: Option<Decimal>,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Option<Decimal>,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: Option<Decimal>,
    #[serde(rename = "secondKey")]
    pub second_key: Option<String>,
    #[serde(rename = "settledCash")]
    pub settled_cash: Option<Decimal>,
    #[serde(rename = "stockMarketValue")]
    pub stock_market_value: Option<Decimal>,
    #[serde(rename = "tBillsMarketValue")]
    pub t_bills_market_value: Option<Decimal>,
    #[serde(rename = "tBondsMarketValue")]
    pub t_bonds_market_value: Option<Decimal>,
    #[serde(rename = "warrantsMarketValue")]
    pub warrants_market_value: Option<Decimal>,
    pub severity: Option<i64>,
}

impl ToStructuredRequest for SubscribeAccountLedgerRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "sld".to_owned(),
            arguments: Option::Some(vec![self.account_id.clone()]),
            body: Option::Some(
                json!({
                    "keys": self.keys,
                    "fields": self.fields,
                })
                .to_string(),
            ),
        }
    }
}

/// Unsubscribes from account ledger messages for the specified account.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeAccountLedgerRequest {
    /// Must pass the account ID whose ledger messages will be unsubscribed.
    pub account_id: String,
}

impl ToStructuredRequest for UnsubscribeAccountLedgerRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "uld".to_owned(),
            arguments: Option::Some(vec![self.account_id.clone()]),
            body: Option::Some("{}".to_owned()),
        }
    }
}

/// Subscribes the user to watch list market data.
/// Streaming, top-of-the-book, level one, market data is available for all instruments using Client Portal API’s websocket endpoint.
///
/// NOTE: The maximum number of market data subscriptions is based on your account’s Market Data Lines.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeMarketDataRequest {
    /// Must pass a single contract identifier.
    /// Contracts requested use SMART routing by default. To specify the exchange, the contract identifier should be modified to: conId@EXCHANGE, where EXCHANGE is the requested data source.
    /// Combos or Spreads market data can be retrieved using identical formatting to Combo or Spread Orders. The only difference is that a spread_conid of 0 must be passed.
    pub conid: String,
    /// Pass an array of field IDs. Each ID should be passed as a string.
    /// You can find a list of fields in the Market Data Fields section.
    pub fields: Vec<String>,
}

impl ToStructuredRequest for SubscribeMarketDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "smd".to_owned(),
            arguments: Option::Some(vec![self.conid.clone()]),
            body: Option::Some(json!({"fields": self.fields}).to_string()),
        }
    }
}

///  Unubscribes the user from watchlist market data.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeMarketDataRequest {
    /// Must pass a single contract identifier.
    pub conid: String,
}

impl ToStructuredRequest for UnsubscribeMarketDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "umd".to_owned(),
            arguments: Option::Some(vec![self.conid.clone()]),
            body: Option::Some("{}".to_owned()),
        }
    }
}

/// For streaming historical data, the topic smh+Id is used. There are also optional parameters available in JSON format. If no parameters are specified, the empty parameters array {} can be passed. Incorrectly specified parameters are ignored and the default (empty) response is returned.
/// Subscribes the user to historical bar data.
/// Streaming, top-of-the-book, level one, historical data is available for all instruments using Client Portal API’s websocket endpoint.
///
/// NOTE: Only a max of 5 concurrent historical data request available at a time.
///
/// NOTE: Historical data will only respond once, though customers will still need to unsubscribe from the endpoint.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeHistoricalDataRequest {
    /// Contracts requested use SMART routing by default. To specify the exchange, the contract identifier should be modified to: conId@EXCHANGE, where EXCHANGE is the requested data source.
    pub conid: String,
    /// Requested exchange to receive data.
    pub exchange: String,
    /// Total duration for which bars will be requested.
    pub period: String,
    /// Interval of time to receive data.
    pub bar: String,
    /// Determines if you want data outside regular trading hours (true) or only during market hours (false).
    pub outside_regular_trading_hours: bool,
    /// The value determining what type of data to show.
    pub source: String,
    /// The format in which bars are returned.
    pub format: String,
}

impl ToStructuredRequest for SubscribeHistoricalDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "smh".to_owned(),
            arguments: Option::Some(vec![self.conid.clone()]),
            body: Option::Some(
                json!({
                    "exchange": self.exchange,
                    "period": self.period,
                    "bar": self.bar,
                    "outsideRth": self.outside_regular_trading_hours,
                    "source": self.source,
                    "format": self.format,
                })
                .to_string(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HistoricalDataResponse {
    /// Represents the request sent.
    pub topic: String,
    /// Request identifier for the specific historical data request. Used for cancelling the data stream.
    #[serde(rename = "serverId")]
    pub server_id: Option<String>,
    /// Returns the symbol for the requested conid.
    pub symbol: String,
    /// Company long name.
    pub text: Option<String>,
    /// Price multiplier (based on $0.01)
    #[serde(rename = "priceFactor")]
    pub price_factor: Option<Decimal>,
    /// Returns the starting time (in epoch time) of the response.
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// Returns the highest “high value/Volume value/Outside RTH volume” of the period.
    pub high: Option<String>,
    /// Returns the lowest “Low value/Volume value/Outside RTH volume” of the period.
    pub low: Option<String>,
    /// Returns the period covered by the request.
    #[serde(rename = "timePeriod")]
    pub time_period: Option<String>,
    /// Returns the string length of the bar response.
    #[serde(rename = "barLength")]
    pub bar_length: Option<i64>,
    /// Internal IBKR message.
    #[serde(rename = "mdAvailability")]
    pub market_data_availability: Option<String>,
    /// Returns if there is any delay in the market data.
    #[serde(rename = "mktDataDelay")]
    pub market_data_delay: Option<i64>,
    /// Returns if the data contains information outside regular trading hours.
    #[serde(rename = "outsideRth")]
    pub outside_regular_trading_hours: Option<bool>,
    /// Determines if the volume is returned as lots, multipliers, or as-is.
    #[serde(rename = "volumeFactor")]
    pub volume_factor: Option<Decimal>,
    /// Internal IBKR message.
    #[serde(rename = "priceDisplayRule")]
    pub price_display_rule: Option<String>,
    /// Internal IBKR message.
    #[serde(rename = "priceDisplayValue")]
    pub price_display_value: Option<String>,
    /// Returns contract rule whether the contract supports negative values or not.
    #[serde(rename = "negativeCapable")]
    pub negative_capable: Option<bool>,
    /// Internal IBKR message.
    #[serde(rename = "messageVersion")]
    pub message_version: Option<i64>,
    /// Returns all bars related that fall within the period.
    pub data: Vec<MarketHistoryBarData>,
    /// Displays the total number of bars returned within ‘data’.
    pub points: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AccountUpdateMessage {
    /// (act) Returns the topic of the given request.
    pub topic: String,
    /// Returns the object containing the pnl data.
    pub args: GetAccountsResponse,
}

/// Unsubscribes the user from historical bar data.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeHistoricalDataRequest {
    /// serverId is passe initially from the historical data request.
    pub server_id: String,
}

impl ToStructuredRequest for UnsubscribeHistoricalDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "umh".to_owned(),
            arguments: Option::Some(vec![self.server_id.clone()]),
            body: Option::None,
        }
    }
}

/// Subscribes the user to BookTrader price ladder data.
/// Streaming BookTrader data requires users to maintain a L2, Depth of Book, market data subscription. See the Market Data Subscriptions page for more details.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeBookTraderPriceLadderRequest {
    /// Must pass a single AccountId.
    pub account_id: String,
    /// Must pass a single contract identifier.
    pub conid: String,
    /// Must pass a single contract identifier.
    /// If no exchange is specified, all available deep exchanges are assumed.
    pub exchange: Option<String>,
}

impl ToStructuredRequest for SubscribeBookTraderPriceLadderRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        let mut arguments: Vec<String> = vec![self.account_id.clone(), self.conid.clone()];
        if let Some(exchange) = &self.exchange {
            arguments.push(exchange.clone());
        }

        StreamingDataStructuredRequest {
            topic: "sbd".to_owned(),
            arguments: Option::Some(arguments),
            body: Option::None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BookTraderPriceLadderResponse {
    /// sbd+acctId+conid
    pub topic: String,
    /// Returns an array of objects to indicate ladder depth.
    pub data: Vec<BookTraderPriceLadderData>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BookTraderPriceLadderData {
    /// Returns the row identifier of the ladder data.
    pub row: i32,
    /// Indicates if the value was marked as the last price for the contract.
    pub focus: i32,
    /// Returns the Last, or last executed trade, price.
    /// In some instances, price and size will be returned in the structure '"price":"size @ price"'
    pub price: String,
    /// Returns the corresponding ask size.
    pub bid: String,
    /// Returns the corresponding bid size.
    pub ask: String,
}

/// Unsubscribes the user from price ladder data.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeBookTraderPriceLadderRequest {
    /// Must pass the account ID of the account that made the request.
    pub account_id: String,
}

impl ToStructuredRequest for UnsubscribeBookTraderPriceLadderRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "ubd".to_owned(),
            arguments: Option::Some(vec![self.account_id.clone()]),
            body: Option::None,
        }
    }
}

/// Ping the websocket in order to keep the websocket session alive.
/// To maintain a session for accessing /iserver or /ccp endpoints, use the topic tic. It is advised to ping the session at least once per minute.
///
/// Note: It is still required to send a request to the /tickle endpoint every few minutes or when the session expires (/sso/validate is returning a 0).
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TickleRequest {}

impl ToStructuredRequest for TickleRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "tic".to_owned(),
            arguments: Option::None,
            body: Option::None,
        }
    }
}

/// As long as an order is active, it is possible to retrieve it using Client Portal API. Live streaming orders can be requested by subscribing to the sor topic. Once live orders are requested we will start to relay back when there is an update. To receive all orders for the current day the endpoint /iserver/account/orders can be used. It is advised to query all orders for the current day first before subscribing to live orders.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeLiveOrderUpdateRequest {}

impl ToStructuredRequest for SubscribeLiveOrderUpdateRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "sor".to_owned(),
            arguments: Option::None,
            body: Option::Some("{}".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrderUpdateResponse {
    /// sor
    pub topic: String,
    pub args: Vec<OrderUpdateArgument>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrderUpdateArgument {
    /// Returns the account Id of which account made the request.
    #[serde(rename = "acct")]
    pub account_id: String,
    /// Contract Identifier for the given order.
    pub conid: String,
    /// Order identifier affiliated with the given order.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Base currency used for the transaction.
    #[serde(rename = "cashCcy")]
    pub cash_currency: Option<String>,
    /// Total quantity filled in the order.
    #[serde(rename = "sizeAndFills")]
    pub size_and_fills: Option<String>,
    /// Order description of the given order.
    /// Describes the side, size, orderType, price, and tif of the order.
    #[serde(rename = "orderDesc")]
    pub order_description: Option<String>,
    /// Ticker symbol of the request.
    pub description1: Option<String>,
    /// Ticker symbol of the request.
    pub ticker: Option<String>,
    /// Security type of the request.
    #[serde(rename = "secType")]
    pub security_type: Option<String>,
    /// Primary exchange where the contract is held.
    #[serde(rename = "listingExchange")]
    pub listing_exchange: Option<String>,
    /// Percentage of the order quantity remaining.
    #[serde(rename = "remainingQuantity")]
    pub remaining_quantity: Option<Decimal>,
    /// Percentage of the ordr quantity filled.
    #[serde(rename = "filledQuantity")]
    pub filled_quantity: Option<Decimal>,
    /// Long name of the contract’s company.
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    /// Current order status.
    /// Value Format: Presubmitted, Submitted, Filled, Cancelled.
    pub status: Option<String>,
    /// Returns the original order type of the given order.
    #[serde(rename = "origOrderType")]
    pub origin_order_type: Option<String>,
    /// Determines if the order supports Tax Optimizer.
    #[serde(rename = "supportsTaxOpt")]
    pub supports_tax_opt: Option<String>,
    /// Returns the datetime object of the most recent execution.
    #[serde(rename = "lastExecutionTime")]
    pub last_execution_time: Option<String>,
    /// Returns the epoch timestamp of the most recent execution.
    #[serde(rename = "lastExecutionTime_r")]
    pub last_execution_time_r: Option<i64>,
    /// Returns the current order type of the order.
    /// Value Format: MARKET, LIMIT, STOP
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    /// Returns the side of the trade.
    /// Value Format: BUY, SELL
    pub side: Option<String>,
    /// Returns the time in force for the given order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,
    pub price: Option<Decimal>,
    /// Background color. Used for Client Portal only.
    #[serde(rename = "bgColor")]
    pub background_color: Option<String>,
    /// Foreground color. Used for Client Portal only.
    #[serde(rename = "fgColor")]
    pub foreground_color: Option<String>,
}

/// Cancels the live order updates subscription.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeLiveOrderUpdatesRequest {}

impl ToStructuredRequest for UnsubscribeLiveOrderUpdatesRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "uor".to_owned(),
            arguments: Option::None,
            body: Option::Some("{}".to_owned()),
        }
    }
}

/// Subscribes the user to live profit and loss information.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeProfitAndLossRequest {}

impl ToStructuredRequest for SubscribeProfitAndLossRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "spl".to_owned(),
            arguments: Option::None,
            body: Option::Some("{}".to_owned()),
        }
    }
}

/// Subscribes the user to live profit and loss information.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ProfitAndLossUpdateResponse {
    /// Returns the topic of the given request.
    pub topic: String,
    /// Returns the object containing the pnl data.
    /// key: Specifies the account for which data was requested.
    pub args: HashMap<String, ProfitAndLossUpdateArgument>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ProfitAndLossUpdateArgument {
    /// The row value of the request. Will increment with additional accounts.
    #[serde(rename = "rowType")]
    pub row_type: i64,
    /// Daily Profit and Loss value.
    pub daily_profit_and_loss: Option<Decimal>,
    /// Net Liquidity in the account.
    pub net_liquidity: Option<Decimal>,
    /// Unrealized Profit and Loss for the day.
    pub unrealized_profit_and_loss: Option<Decimal>,
    /// Unrounded Excess Liquidity in the account.
    pub unrounded_excess_liquidity: Option<Decimal>,
    /// Market value of held stocks in the account.
    pub market_value: Option<Decimal>,
}

/// Cancels the subscriptions to profit and loss information.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeProfitAndLossRequest {}

impl ToStructuredRequest for UnsubscribeProfitAndLossRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "upl".to_owned(),
            arguments: Option::None,
            body: Option::Some("{}".to_owned()),
        }
    }
}

/// Subscribes the user to trades data. This will return all executions data while streamed.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SubscribeTradeDataRequest {
    /// Decide whether you want to display any historical executions, or only the executions available in real time.
    /// Set to false by default.
    #[serde(
        rename = "realtimeUpdatesOnly",
        skip_serializing_if = "Option::is_none"
    )]
    pub real_time_updates_only: Option<bool>,
    /// Returns the number of days of executions for data to be returned.
    /// Set to 1 by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

impl ToStructuredRequest for SubscribeTradeDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "str".to_owned(),
            arguments: Option::None,
            body: Option::Some(serde_json::to_string(self).unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TradeDataResponse {
    /// Returns the topic of the given request.
    pub topic: String,
    /// Returns the object containing the pnl data.
    pub args: Vec<TradeDataArgument>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TradeDataArgument {
    /// Execution identifier of the specific trade.
    pub execution_id: String,
    /// Contract identifier for the traded contract.
    pub conid: i64,
    /// Ticker symbol of the traded contract.
    pub symbol: Option<String>,
    /// Determines if the contract supports the tax optimizer. Client Portal only.
    pub supports_tax_opt: Option<String>,
    /// Determines if the order was a buy or sell side.
    pub side: Option<String>,
    /// Describes the full content of the order.
    /// Value format: "{SIDE} {SIZE} @ {PRICE} on {EXCHANGE}"
    pub order_description: Option<String>,
    /// Traded date time in UTC.
    /// Value format: “YYYYMMDD-HH:mm:ss”
    pub trade_time: Option<String>,
    /// Traded datetime of the execution in epoch time.
    pub trade_time_r: Option<i64>,
    /// Returns the quantity of shares traded.
    pub size: Option<Decimal>,
    /// Returns the price used for the given trade.
    pub price: Option<Decimal>,
    /// Returns the exchange the order executed at.
    pub exchange: Option<String>,
    /// Returns the total amount traded after calculating multiplier.
    pub net_amount: Option<Decimal>,
    /// Returns the account the order was traded on.
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Returns the account the order was traded on.
    #[serde(rename = "accountCode")]
    pub account_code: Option<String>,
    /// Returns the title of the company for the contract.
    pub company_name: Option<String>,
    /// Returns the underlying symbol of the contract.
    pub contract_description_1: Option<String>,
    /// Returns a full description of the derivative.
    pub contract_description_2: Option<String>,
    /// Returns the security type traded.
    pub sec_type: Option<String>,
    /// Returns the conidEx of the order if specified. Otherwise returns conid.
    #[serde(rename = "conidEx")]
    pub conid_exchange: Option<String>,
    /// Returns if the execution was a closing trade.
    /// Returns "???" if the position was already open, but not a closing order.
    pub open_close: Option<String>,
    /// Returns if the trade was a result of liquidation.
    pub liquidation_trade: Option<String>,
    /// Determines if the order can be used with EventTrader.
    pub is_event_trading: Option<String>,
}

/// Cancels the trades data subscription
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UnsubscribeTradeDataRequest {}

impl ToStructuredRequest for UnsubscribeTradeDataRequest {
    fn to_structured_request(&self) -> StreamingDataStructuredRequest {
        StreamingDataStructuredRequest {
            topic: "utr".to_owned(),
            arguments: Option::None,
            body: Option::None,
        }
    }
}
