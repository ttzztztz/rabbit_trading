use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CCPOrderDataWarning {
    #[serde(rename = "PRICECAP")]
    price_cap: Option<String>,
    #[serde(rename = "TIME")]
    time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CCPOrderData {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(rename = "execId")]
    pub exec_id: Option<String>,
    #[serde(rename = "execType")]
    pub exec_type: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    #[serde(rename = "orderStatus")]
    pub order_status: Option<String>,
    /// Underlying symbol for contract
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// Quantity of active order
    #[serde(rename = "orderQty")]
    pub order_qty: Option<String>,
    /// Price of active order
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// Quantity of the last partial fill
    #[serde(rename = "lastShares")]
    pub last_shares: Option<String>,
    /// Price of the last partial fill
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// Cumulative fill quantity
    #[serde(rename = "cumQty")]
    pub cum_qty: Option<String>,
    /// Remaining quantity to be filled
    #[serde(rename = "leavesQty")]
    pub leaves_qty: Option<String>,
    /// Average fill price
    #[serde(rename = "avgPrice")]
    pub avg_price: Option<String>,
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// Order identifier
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    /// Account number
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// Contracts asset class
    #[serde(rename = "secType")]
    pub sec_type: Option<String>,
    /// Time of transaction in GMT, format YYYYMMDD-hh:m:ss
    #[serde(rename = "txTime")]
    pub tx_time: Option<String>,
    /// Time of receipt in GMT, format YYYYMMDD-hh:mm:ss
    #[serde(rename = "rcptTime")]
    pub rcpt_time: Option<String>,
    /// Time in Force
    #[serde(rename = "tif")]
    pub time_in_force: Option<String>,
    /// Contract identifier from IBKR's database.
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// Trading currency
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Exchange or venue
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    /// Listing Exchange
    #[serde(rename = "listingExchange")]
    pub listing_exchange: Option<String>,
    /// error message
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "warnings")]
    pub warnings: Option<CCPOrderDataWarning>,
    /// Commission currency
    #[serde(rename = "commCurr")]
    pub comm_curr: Option<String>,
    /// Commissions
    #[serde(rename = "comms")]
    pub comms: Option<String>,
    /// Realized PnL
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Option<String>,
}

pub struct GetCCPTradesRequest {
    /// From Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..)
    pub from: Option<String>,
    /// To Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..). To value should be bigger than from value.
    pub to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCCPTradesResponse {
    #[serde(rename = "orders")]
    pub orders: Option<Vec<CCPOrderData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCCPStatusResponse {
    /// Login session is authenticated to the CCP.
    #[serde(rename = "authenticated")]
    pub authenticated: Option<bool>,
    /// Login session is connected
    #[serde(rename = "connected")]
    pub connected: Option<bool>,
    /// server name
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCCPAccountListResponse {
    /// The primary or parent account.
    #[serde(rename = "mainAcct")]
    pub main_account: Option<String>,
    /// List of tradeable or Sub Accounts
    #[serde(rename = "acctList")]
    pub account_list: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCCPPositionResponse {
    /// Contract identifier from IBKR's database.
    #[serde(rename = "conid")]
    pub conid: i64,
    /// Number of shares or quantity of the position.
    #[serde(rename = "position")]
    pub position: Option<Decimal>,
    /// Average cost of the position.
    #[serde(rename = "avgCost")]
    pub avg_cost: Option<Decimal>,
}

pub struct GetCCPOrderStatusRequest {
    /// User Account
    pub account: String,
    /// Return only Rejected or Cancelled orders since today midnight
    pub cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCCPOrderStatusResponse {
    #[serde(rename = "orders")]
    pub orders: Option<Vec<CCPOrderData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartCCPSessionRequest {
    /// Allow competing CCP session to run
    pub compete: bool,
    /// Concatenate value for language and region, set to "en_US"
    pub locale: String,
    /// Local MAC Address
    pub mac: String,
    /// Local machine ID
    #[serde(rename = "machineId")]
    pub machine_id: String,
    /// Login user, set to dash "-"
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartCCPSessionResponse {
    /// Challenge in hex format
    #[serde(rename = "challenge")]
    pub challenge: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteCCPSessionRequest {
    #[serde(rename = "response")]
    pub response: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteCCPSessionResponse {
    /// If sso authentication completed
    #[serde(rename = "passed")]
    pub passed: Option<bool>,
    /// If connection is authenticated
    #[serde(rename = "authenticated")]
    pub authenticated: Option<bool>,
    /// Connected to CCP session
    #[serde(rename = "connected")]
    pub connected: Option<bool>,
    /// If user already has an existing brokerage session running.
    #[serde(rename = "competing")]
    pub competing: Option<bool>,
}

pub struct SubmitCCPOrderRequest {
    /// User Account
    pub account: String,
    /// Contract identifier from IBKR's database.
    pub conid: i64,
    /// Enum: "USD" "GBP" "EUR"
    /// Contract Currency
    pub contract_currency: String,
    /// Enum: "NYSE" "CBOE" "NYMEX"
    /// Exchange
    pub exchange: String,
    /// Order Quantity
    pub quantity: Decimal,
    /// Enum: "limit" "market"
    /// Order Price; required if order type is limit
    pub _type: Option<String>,
    /// Enum: "sell" "buy"
    /// Side
    pub side: Option<String>,
    /// Order Price; required if order type is limit
    pub price: Option<Decimal>,
    /// Enum: "IOC" "GTC"
    /// Time in Force
    pub time_in_force: Option<String>,
}
pub type SubmitCCPOrderResponse = CCPOrderData;

pub struct DeleteCCPOrderRequest {
    /// Account Number
    pub account: String,
    /// Order Identifier of original submit order
    pub id: i64,
}
pub type DeleteCCPOrderResponse = CCPOrderData;

pub struct UpdateCCPOrderRequest {
    /// User Account
    pub account: String,
    /// Order ID to be modified
    pub id: i64,

    // todo: validate the parameters below
    /// Order Quantity
    pub quantity: Option<Decimal>,
    /// Order Price; required if order type is limit
    pub price: Option<Decimal>,
}
pub type UpdateCCPOrderResponse = CCPOrderData;
