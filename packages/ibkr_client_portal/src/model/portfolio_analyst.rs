use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceCpsData {
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// for example-- acctid
    #[serde(rename = "idType")]
    pub id_type: Option<String>,
    /// start date-- yyyyMMdd
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(rename = "baseCurrency")]
    pub base_currency: Option<String>,
    /// each value stands for price change percent of corresponding date in dates array
    #[serde(rename = "returns")]
    pub returns: Option<Vec<Decimal>>,
    /// end date-- yyyyMMdd
    #[serde(rename = "end")]
    pub end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceCps {
    /// array of dates, the length should be same as the length of returns inside data.
    #[serde(rename = "dates")]
    pub dates: Option<Vec<String>>,
    /// D means Day
    #[serde(rename = "freq")]
    pub freq: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<PerformanceCpsData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceTpps {
    /// array of dates, the length should be same as the length of returns inside data.
    #[serde(rename = "dates")]
    pub dates: Option<Vec<String>>,
    /// M means Month
    #[serde(rename = "freq")]
    pub freq: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<PerformanceCpsData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceNav {
    /// array of dates, the length should be same as the length of returns inside data.
    #[serde(rename = "dates")]
    pub dates: Option<Vec<String>>,
    /// D means Day
    #[serde(rename = "freq")]
    pub freq: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<Vec<PerformanceCpsData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPortfolioPerformanceResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "cps")]
    pub cps: Option<PerformanceCps>,
    #[serde(rename = "tpps")]
    pub tpps: Option<PerformanceTpps>,
    #[serde(rename = "nav")]
    pub nav: Option<PerformanceNav>,
    #[serde(rename = "pm")]
    pub pm: Option<String>,
    #[serde(rename = "included")]
    pub included: Option<Vec<String>>,
    #[serde(rename = "currencyType")]
    pub currency_type: Option<String>,
    #[serde(rename = "rc")]
    pub rc: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPortfolioPerformanceRequest {
    #[serde(rename = "acctIds")]
    pub account_id_list: Option<Vec<String>>,
    /// Frequency of cumulative performance data points: 'D'aily, 'M'onthly,'Q'uarterly.
    #[serde(rename = "freq")]
    pub freq: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPortfolioTransactionsRequest {
    #[serde(rename = "acctIds")]
    pub account_id_list: Vec<String>,
    #[serde(rename = "conids")]
    pub conid_list: Vec<i64>,
    /// optional defaults to USD.
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// optional, default value is 90
    #[serde(rename = "days")]
    pub days: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioTransactions {
    #[serde(rename = "acctid")]
    pub account_id: Option<String>,
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// currency code
    #[serde(rename = "cur")]
    pub cur: Option<String>,
    /// Conversion rate from asset currency to response currency
    #[serde(rename = "fxRate")]
    pub fx_rate: Option<Decimal>,
    /// Transaction description
    #[serde(rename = "desc")]
    pub desc: Option<String>,
    /// Date of transaction.  Epoch time, GMT
    #[serde(rename = "date")]
    pub date: Option<String>,
    /// Transaction Type Name: Examples: \"Sell\", \"Buy\", \"Corporate Action\", \"Dividend Payment\", \"Transfer\", \"Payment in Lieu\" Dividends and Transfers do not have price and quantity in response
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Not applicable for all transaction types
    #[serde(rename = "qty")]
    pub quantity: Option<Decimal>,
    /// In asset currency. Not be applicable for all transaction types.
    #[serde(rename = "pr")]
    pub pr: Option<Decimal>,
    /// Raw value, no formatting. Net transaction amount (may include commission, tax). In asset currency
    #[serde(rename = "amt")]
    pub amount: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPortfolioTransactionsResponse {
    /// will always be getTransactions
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// same as request
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Indicates whether current day and realtime data is included in the result
    #[serde(rename = "includesRealTime")]
    pub includes_real_time: Option<bool>,
    /// Period start date. Epoch time, GMT
    #[serde(rename = "from")]
    pub from: Option<i64>,
    /// Period end date. Epoch time, GMT
    #[serde(rename = "to")]
    pub to: Option<i64>,
    /// Sorted by date descending
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<PortfolioTransactions>>,
}
