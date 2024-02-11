use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTicket {
    /// acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.
    #[serde(rename = "acctId")]
    pub account_id: Option<i64>,
    /// conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search.
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// Conid and Exchange - Can be used instead of conid when specifying the contract identifier of a security.
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    /// The contract-identifier (conid) and security type (type) specified as a concatenated value, conid:type
    #[serde(rename = "secType")]
    pub sec_type: Option<String>,
    /// Customer Order ID. An arbitrary string that can be used to identify the order, e.g \"my-fb-order\". The value must be unique for a 24h span. Please do not set this value for child orders when placing a bracket order.
    #[serde(rename = "cOID")]
    pub c_oid: Option<String>,
    /// Only specify for child orders when placing bracket orders. The parentId for the child order(s) must be equal to the cOId (customer order id) of the parent.
    #[serde(rename = "parentId")]
    pub parent_id: Option<i64>,
    /// The order-type determines what type of order you want to send.   * LMT - A limit order is an order to buy or sell at the specified price or better.   * MKT - A market order is an order to buy or sell at the markets current NBBO.   * STP - A stop order becomes a market order once the specified stop price is attained or penetrated.   * STOP_LIMIT - A stop limit order becomes a limit order once the specified stop price is attained or penetrated.   * MIDPRICE - A MidPrice order attempts to fill at the current midpoint of the NBBO or better.   * TRAIL - A sell trailing stop order sets the stop price at a fixed amount below the market price with an attached \"trailing\" amount. See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=605   * TRAILLMT - A trailing stop limit order is designed to allow an investor to specify a limit on the maximum possible loss, without setting a limit on the maximum possible gain.     See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=606
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    /// listingExchange is optional. By default we use \"SMART\" routing. Possible values are available via the endpoint: /iserver/contract/{conid}/info, see **valid_exchange** e.g: SMART,AMEX,NYSE,CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX
    #[serde(rename = "listingExchange")]
    pub listing_exchange: Option<String>,
    /// set to true if you want to place a single group orders(OCA)
    #[serde(rename = "isSingleGroup")]
    pub is_single_group: Option<bool>,
    /// set to true if the order can be executed outside regular trading hours.
    #[serde(rename = "outsideRTH")]
    pub outside_rth: Option<bool>,
    /// optional if order is LMT, or STOP_LIMIT, this is the limit price. For STP|TRAIL this is the stop price. For MIDPRICE this is the option price cap.
    #[serde(rename = "price")]
    pub price: Option<i64>,
    /// optional if order is STOP_LIMIT|TRAILLMT, this is the stop price. You must specify both price and auxPrice for STOP_LIMIT|TRAILLMT orders.
    #[serde(rename = "auxPrice")]
    pub aux_price: Option<Value>,
    /// SELL or BUY
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// This is the  underlying symbol for the contract.
    #[serde(rename = "ticker")]
    pub ticker: Option<String>,
    /// The Time-In-Force determines how long the order remains active on the market.   * GTC - use Good-Till-Cancel for orders to remain active until it executes or cancelled.   * OPG - use Open-Price-Guarantee for Limit-On-Open (LOO) or Market-On-Open (MOO) orders.   * DAY - if not executed a Day order will automatically cancel at the end of the markets regular trading hours.   * IOC - any portion of an Immediate-or-Cancel order that is not filled as soon as it becomes available in the market is cancelled.
    #[serde(rename = "tif")]
    pub time_in_force: Option<String>,
    /// optional if order is TRAIL, or TRAILLMT. When trailingType is amt, this is the trailing amount, when trailingType is %, it means percentage. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingAmt")]
    pub trailing_amount: Option<i64>,
    /// This is the trailing type for trailing amount. We only support two types here: amt or %. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingType")]
    pub trailing_type: Option<String>,
    /// Custom order reference
    #[serde(rename = "referrer")]
    pub referrer: Option<String>,
    /// Usually integer, for some special cases such as fractional orders can specify as a float, e.g. quantity = 0.001. In some special cases quantity is not specified, such as when using 'cashQty' or 'fxQty'.
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    /// Cash Quantity - used to specify the monetary value of an order instead of the number of shares. When using 'cashQty' don't specify 'quantity' Orders that express size using a monetary value, e.g. cash quantity can result in fractional shares and are provided on a non-guaranteed basis. The system simulates the order by canceling it once the specified amount is spent (for buy orders) or collected (for sell orders). In addition to the monetary value, the order uses a maximum size that is calculated using the Cash Quantity Estimated Factor, which can be modified in Order Presets.   
    #[serde(rename = "cashQty")]
    pub cash_quantity: Option<i64>,
    /// double number, this is the cash quantity field which can only be used for Currency Conversion Orders. When using 'fxQty' don't specify 'quantity'.
    #[serde(rename = "fxQty")]
    pub fx_quantity: Option<i64>,
    /// If true, the system will use the Price Management Algo to submit the order. https://www.interactivebrokers.com/en/index.php?f=43423
    #[serde(rename = "useAdaptive")]
    pub use_adaptive: Option<bool>,
    /// set to true if the order is a FX conversion order
    #[serde(rename = "isCcyConv")]
    pub is_ccy_conversion: Option<bool>,
    /// Set the allocation method when placing an order using an FA account for a group Possible allocation methods are \"NetLiquidity\", \"AvailableEquity\", \"EqualQuantity\" and \"PctChange\".
    #[serde(rename = "allocationMethod")]
    pub allocation_method: Option<String>,
    /// Specify which IB Algo algorithm to use for this order.
    #[serde(rename = "strategy")]
    pub strategy: Option<String>,
    /// The IB Algo parameters for the specified algorithm.
    #[serde(rename = "strategyParameters")]
    pub strategy_parameters: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    pub orders: Vec<OrderTicket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Please note here, if the message is a question, you have to reply to question in order to submit the order successfully. See more in the \"/iserver/reply/{replyid}\" endpoint.
    #[serde(rename = "message")]
    pub message: Option<Vec<String>>,
}
