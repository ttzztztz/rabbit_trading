use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTicket {
    /// acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.
    #[serde(rename = "acctId")]
    pub account_id: Option<String>,
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
    pub outside_regular_trading_hours: Option<bool>,
    /// optional if order is LMT, or STOP_LIMIT, this is the limit price. For STP|TRAIL this is the stop price. For MIDPRICE this is the option price cap.
    #[serde(rename = "price")]
    pub price: Option<Decimal>,
    /// optional if order is STOP_LIMIT|TRAILLMT, this is the stop price. You must specify both price and auxPrice for STOP_LIMIT|TRAILLMT orders.
    #[serde(rename = "auxPrice")]
    pub aux_price: Option<Value>,
    /// SELL or BUY
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// This is the  underlying symbol for the contract.
    #[serde(rename = "ticker")]
    pub ticker: Option<String>,
    /// The Time-In-Force determines how long the order remains active on the market.
    /// * GTC - use Good-Till-Cancel for orders to remain active until it executes or cancelled.
    /// * OPG - use Open-Price-Guarantee for Limit-On-Open (LOO) or Market-On-Open (MOO) orders.
    /// * DAY - if not executed a Day order will automatically cancel at the end of the markets regular trading hours.
    /// * IOC - any portion of an Immediate-or-Cancel order that is not filled as soon as it becomes available in the market is cancelled.
    #[serde(rename = "tif")]
    pub time_in_force: Option<String>,
    /// optional if order is TRAIL, or TRAILLMT. When trailingType is amt, this is the trailing amount, when trailingType is %, it means percentage. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingAmt")]
    pub trailing_amount: Option<Decimal>,
    /// This is the trailing type for trailing amount. We only support two types here: amt or %. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingType")]
    pub trailing_type: Option<String>,
    /// Custom order reference
    #[serde(rename = "referrer")]
    pub referrer: Option<String>,
    /// Usually integer, for some special cases such as fractional orders can specify as a float, e.g. quantity = 0.001. In some special cases quantity is not specified, such as when using 'cashQty' or 'fxQty'.
    #[serde(rename = "quantity")]
    pub quantity: Option<Decimal>,
    /// Cash Quantity - used to specify the monetary value of an order instead of the number of shares. When using 'cashQty' don't specify 'quantity' Orders that express size using a monetary value, e.g. cash quantity can result in fractional shares and are provided on a non-guaranteed basis. The system simulates the order by canceling it once the specified amount is spent (for buy orders) or collected (for sell orders). In addition to the monetary value, the order uses a maximum size that is calculated using the Cash Quantity Estimated Factor, which can be modified in Order Presets.   
    #[serde(rename = "cashQty")]
    pub cash_quantity: Option<Decimal>,
    /// double number, this is the cash quantity field which can only be used for Currency Conversion Orders. When using 'fxQty' don't specify 'quantity'.
    #[serde(rename = "fxQty")]
    pub fx_quantity: Option<Decimal>,
    /// If true, the system will use the Price Management Algo to submit the order. https://www.interactivebrokers.com/en/index.php?f=43423
    #[serde(rename = "useAdaptive")]
    pub use_adaptive: Option<bool>,
    /// set to true if the order is a FX conversion order
    #[serde(rename = "isCcyConv")]
    pub is_currency_conversion: Option<bool>,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrderRequest {
    pub orders: Vec<OrderTicket>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Please note here, if the message is a question, you have to reply to question in order to submit the order successfully. See more in the \"/iserver/reply/{replyid}\" endpoint.
    #[serde(rename = "message")]
    pub message: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveOrder {
    /// Account number
    #[serde(rename = "acct")]
    pub acct: Option<String>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    /// Contract identifier
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// Order identifier
    #[serde(rename = "orderId")]
    pub order_id: Option<i64>,
    /// Cash currency
    #[serde(rename = "cashCcy")]
    pub cash_currency: Option<String>,
    /// Quantity outstanding and total quantity concatenated with forward slash separator
    #[serde(rename = "sizeAndFills")]
    pub size_and_fills: Option<String>,
    /// Order description
    #[serde(rename = "orderDesc")]
    pub order_desc: Option<String>,
    /// Formatted ticker description
    #[serde(rename = "description1")]
    pub description1: Option<String>,
    /// Underlying symbol
    #[serde(rename = "ticker")]
    pub ticker: Option<String>,
    /// Asset class
    #[serde(rename = "secType")]
    pub sec_type: Option<String>,
    /// Listing Exchange
    #[serde(rename = "listingExchange")]
    pub listing_exchange: Option<String>,
    /// Quantity remaining
    #[serde(rename = "remainingQuantity")]
    pub remaining_quantity: Option<Decimal>,
    /// Quantity filled
    #[serde(rename = "filledQuantity")]
    pub filled_quantity: Option<Decimal>,
    /// Company Name
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    /// Status of the order
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Original order type
    #[serde(rename = "origOrderType")]
    pub orig_order_type: Option<String>,
    /// Supports Tax Optimization with 0 for no and 1 for yes
    #[serde(rename = "supportsTaxOpt")]
    pub supports_tax_opt: Option<String>,
    /// Last status update in format YYMMDDhhmms based in GMT
    #[serde(rename = "lastExecutionTime")]
    pub last_execution_time: Option<String>,
    /// Last status update unix time in ms
    #[serde(rename = "lastExecutionTime_r")]
    pub last_execution_time_r: Option<i64>,
    /// Order type
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    /// Order reference
    #[serde(rename = "order_ref")]
    pub order_ref: Option<String>,
    /// The side of the market of the order.  * BUY: Buy contract near posted ask price  * SELL: Sell contract near posted bid price  * ASSN: Option Assignment, if BUYSELL=BUY and OptionType=PUT or BUYSELL=SELL and OptionType=CALL  * EXER: Option Exercise, if BUYSELL=SELL and OptionType=PUT or BUYSELL=BUY and OptionType=CALL
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,
    /// Price of order
    #[serde(rename = "price")]
    pub price: Option<Decimal>,
    /// Background color in hex format
    #[serde(rename = "bgColor")]
    pub background_color: Option<String>,
    /// Foreground color in hex format
    #[serde(rename = "fgColor")]
    pub foreground_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetLiveOrderResponse {
    #[serde(rename = "filters")]
    filters: Option<Vec<String>>,
    #[serde(rename = "orders")]
    orders: Vec<LiveOrder>,
    /// If live order update is a snapshot
    #[serde(rename = "snapshot")]
    snapshot: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderStatus {
    /// order sub-type
    #[serde(rename = "sub_type")]
    pub sub_type: Option<String>,
    /// order request id
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// system generated order id, unique per account
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// Format contract name
    #[serde(rename = "contract_description_1")]
    pub contract_description_1: Option<String>,
    /// Trading Exchange or Venue
    #[serde(rename = "listing_exchange")]
    pub listing_exchange: Option<String>,
    #[serde(rename = "option_acct")]
    pub option_acct: Option<String>,
    /// Contracts company name
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    /// Quantity updated
    #[serde(rename = "size")]
    pub size: Option<String>,
    /// Total quantity
    #[serde(rename = "total_size")]
    pub total_size: Option<Decimal>,
    /// Contract traded currency
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// account id
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// Types of orders
    #[serde(rename = "order_type")]
    pub order_type: Option<String>,
    /// Limit price
    #[serde(rename = "limit_price")]
    pub limit_price: Option<Decimal>,
    /// Stop price
    #[serde(rename = "stop_price")]
    pub stop_price: Option<Decimal>,
    /// Cumulative fill
    #[serde(rename = "cum_fill")]
    pub cum_fill: Option<Decimal>,
    /// *  PendingSubmit - Indicates the order was sent, but confirmation has not been received that it has been received by the destination. Occurs most commonly if an exchange is closed.
    /// *  PendingCancel - Indicates that a request has been sent to cancel an order but confirmation has not been received of its cancellation.
    /// *  PreSubmitted - Indicates that a simulated order type has been accepted by the IBKR system and that this order has yet to be elected. The order is held in the IBKR system until the election criteria are met. At that time the order is transmitted to the order destination as specified.
    /// *  Submitted - Indicates that the order has been accepted at the order destination and is working.
    /// *  Cancelled - Indicates that the balance of the order has been confirmed cancelled by the IB system. This could occur unexpectedly when IB or the destination has rejected the order.
    /// *  Filled - Indicates that the order has been completely filled.
    /// *  Inactive - Indicates the order is not working, for instance if the order was invalid and triggered an error message, or if the order was to short a security and shares have not yet been located.
    #[serde(rename = "order_status")]
    pub order_status: Option<String>,
    /// Description of the order status
    #[serde(rename = "order_status_description")]
    pub order_status_description: Option<String>,
    /// Time-in-Force - length of time order will continue working before it is canceled.
    #[serde(rename = "tif")]
    pub time_in_force: Option<String>,
    /// Foreground color in hex format
    #[serde(rename = "fg_color")]
    pub foreground_color: Option<String>,
    /// Background color in hex format
    #[serde(rename = "bg_color")]
    pub background_color: Option<String>,
    /// If true not allowed to modify order
    #[serde(rename = "order_not_editable")]
    pub order_not_editable: Option<bool>,
    /// Fields that can be edited in escaped unicode characters
    #[serde(rename = "editable_fields")]
    pub editable_fields: Option<String>,
    /// If true not allowed to cancel order
    #[serde(rename = "cannot_cancel_order")]
    pub cannot_cancel_order: Option<bool>,
    /// If true order trades outside regular trading hours
    #[serde(rename = "outside_regular_trading_hours")]
    pub outside_regular_trading_hours: Option<bool>,
    /// If true order is de-activated
    #[serde(rename = "deactivate_order")]
    pub deactivate_order: Option<bool>,
    /// If true price management algo is enabled, refer to https://www.interactivebrokers.com/en/index.php?f=43423
    #[serde(rename = "use_price_mgmt_algo")]
    pub use_price_mgmt_algo: Option<bool>,
    /// Asset class
    #[serde(rename = "sec_type")]
    pub sec_type: Option<String>,
    /// List of available chart periods
    #[serde(rename = "available_chart_periods")]
    pub available_chart_periods: Option<String>,
    /// Format description of order
    #[serde(rename = "order_description")]
    pub order_description: Option<String>,
    /// order_description with the symbol
    #[serde(rename = "order_description_with_contract")]
    pub order_description_with_contract: Option<String>,
    #[serde(rename = "alert_active")]
    pub alert_active: Option<i32>,
    /// type of the child order
    #[serde(rename = "child_order_type")]
    pub child_order_type: Option<String>,
    /// Format fillQuantity\\totalQuantity
    #[serde(rename = "size_and_fills")]
    pub size_and_fills: Option<String>,
    /// Position display price
    #[serde(rename = "exit_strategy_display_price")]
    pub exit_strategy_display_price: Option<String>,
    /// Position description to display on chart
    #[serde(rename = "exit_strategy_chart_description")]
    pub exit_strategy_chart_description: Option<String>,
    /// * 1: If your account has position or order for contract * 0: If your account has no position or order for contract
    #[serde(rename = "exit_strategy_tool_availability")]
    pub exit_strategy_tool_availability: Option<String>,
    /// Returns true if contract supports duplicate/opposite side order.
    #[serde(rename = "allowed_duplicate_opposite")]
    pub allowed_duplicate_opposite: Option<bool>,
    /// Time of status update in unix time
    #[serde(rename = "order_time")]
    pub order_time: Option<String>,
    /// only exists for oca orders, oca orders in same group will have same id
    #[serde(rename = "oca_group_id")]
    pub oca_group_id: Option<String>,
    pub trailing_amount: Option<Decimal>,
    pub trailing_amount_unit: Option<String>,
    #[serde(rename = "limit_price_Offset")]
    pub limit_price_offset: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetOrderStatusRequest {
    pub order_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreviewOrderAmount {
    /// for example 23,000 USD
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    /// for example 1.1 ... 1.2 USD
    #[serde(rename = "commission")]
    pub commission: Option<String>,
    #[serde(rename = "total")]
    pub total: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreviewOrderEquity {
    #[serde(rename = "current")]
    pub current: Option<String>,
    #[serde(rename = "change")]
    pub change: Option<String>,
    #[serde(rename = "after")]
    pub after: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderRequest {
    /// acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i64>,
    /// Conid and Exchange - Can be used instead of conid when specifying the contract identifier of a security.
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// The contract-identifier (conid) and security type (type) specified as a concatenated value, conid:type
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
    /// Customer Order ID. An arbitrary string that can be used to identify the order, e.g \"my-fb-order\". The value must be unique for a 24h span. Please do not set this value for child orders when placing a bracket order.
    #[serde(rename = "cOID", skip_serializing_if = "Option::is_none")]
    pub c_oid: Option<String>,
    /// Only specify for child orders when placing bracket orders. The parentId for the child order(s) must be equal to the cOId (customer order id) of the parent.
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The order-type determines what type of order you want to send.
    /// * LMT - A limit order is an order to buy or sell at the specified price or better.
    /// * MKT - A market order is an order to buy or sell at the markets current NBBO.
    /// * STP - A stop order becomes a market order once the specified stop price is attained or penetrated.
    /// * STOP_LIMIT - A stop limit order becomes a limit order once the specified stop price is attained or penetrated.
    /// * MIDPRICE - A MidPrice order attempts to fill at the current midpoint of the NBBO or better.
    /// * TRAIL - A sell trailing stop order sets the stop price at a fixed amount below the market price with an attached \"trailing\" amount. See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=605
    /// * TRAILLMT - A trailing stop limit order is designed to allow an investor to specify a limit on the maximum possible loss, without setting a limit on the maximum possible gain.
    /// See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=606
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// listingExchange is optional. By default we use \"SMART\" routing. Possible values are available via the endpoint: /iserver/contract/{conid}/info, see **valid_exchange** e.g: SMART,AMEX,NYSE,CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// set to true if you want to place a single group orders(OCA)
    #[serde(rename = "isSingleGroup", skip_serializing_if = "Option::is_none")]
    pub is_single_group: Option<bool>,
    /// set to true if the order can be executed outside regular trading hours.
    #[serde(rename = "outsideRTH")]
    pub outside_regular_trading_hours: bool,
    /// optional if order is LMT, or STOP_LIMIT, this is the limit price. For STP|TRAIL this is the stop price. For MIDPRICE this is the option price cap.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    /// optional if order is STOP_LIMIT|TRAILLMT, this is the stop price. You must specify both price and auxPrice for STOP_LIMIT|TRAILLMT orders.
    #[serde(rename = "auxPrice", skip_serializing_if = "Option::is_none")]
    pub aux_price: Option<Decimal>,
    #[serde(rename = "lmtOffset", skip_serializing_if = "Option::is_none")]
    pub limit_offset: Option<Decimal>,
    /// SELL or BUY
    #[serde(rename = "side")]
    pub side: String,
    /// This is the  underlying symbol for the contract.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The Time-In-Force determines how long the order remains active on the market.   * GTC - use Good-Till-Cancel for orders to remain active until it executes or cancelled.   * OPG - use Open-Price-Guarantee for Limit-On-Open (LOO) or Market-On-Open (MOO) orders.   * DAY - if not executed a Day order will automatically cancel at the end of the markets regular trading hours.   * IOC - any portion of an Immediate-or-Cancel order that is not filled as soon as it becomes available in the market is cancelled.
    #[serde(rename = "tif")]
    pub time_in_force: String,
    /// optional if order is TRAIL, or TRAILLMT. When trailingType is amt, this is the trailing amount, when trailingType is %, it means percentage. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingAmt", skip_serializing_if = "Option::is_none")]
    pub trailing_amount: Option<Decimal>,
    /// This is the trailing type for trailing amount. We only support two types here: amt or %. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingType", skip_serializing_if = "Option::is_none")]
    pub trailing_type: Option<String>,
    /// Custom order reference
    #[serde(rename = "referrer", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// Usually integer, for some special cases such as fractional orders can specify as a float, e.g. quantity = 0.001. In some special cases quantity is not specified, such as when using 'cashQty' or 'fxQty'.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    /// Cash Quantity - used to specify the monetary value of an order instead of the number of shares. When using 'cashQty' don't specify 'quantity' Orders that express size using a monetary value, e.g. cash quantity can result in fractional shares and are provided on a non-guaranteed basis. The system simulates the order by canceling it once the specified amount is spent (for buy orders) or collected (for sell orders). In addition to the monetary value, the order uses a maximum size that is calculated using the Cash Quantity Estimated Factor, which can be modified in Order Presets.   
    #[serde(rename = "cashQty", skip_serializing_if = "Option::is_none")]
    pub cash_quantity: Option<Decimal>,
    /// double number, this is the cash quantity field which can only be used for Currency Conversion Orders. When using 'fxQty' don't specify 'quantity'.
    #[serde(rename = "fxQty", skip_serializing_if = "Option::is_none")]
    pub fx_quantity: Option<Decimal>,
    /// If true, the system will use the Price Management Algo to submit the order. https://www.interactivebrokers.com/en/index.php?f=43423
    #[serde(rename = "useAdaptive", skip_serializing_if = "Option::is_none")]
    pub use_adaptive: Option<bool>,
    /// set to true if the order is a FX conversion order
    #[serde(rename = "isCcyConv", skip_serializing_if = "Option::is_none")]
    pub is_currency_conv: Option<bool>,
    /// Set the allocation method when placing an order using an FA account for a group Possible allocation methods are \"NetLiquidity\", \"AvailableEquity\", \"EqualQuantity\" and \"PctChange\".
    #[serde(rename = "allocationMethod", skip_serializing_if = "Option::is_none")]
    pub allocation_method: Option<String>,
    /// Specify which IB Algo algorithm to use for this order.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// The IB Algo parameters for the specified algorithm.
    #[serde(rename = "strategyParameters", skip_serializing_if = "Option::is_none")]
    pub strategy_parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreviewOrderRequest {
    #[serde(skip)]
    pub account_id: String,

    /// Notes for bracket orders: 1. Children orders will not have its own \"cOID\", so please donot pass \"cOID\" parameter in child order.Instead, they will have a \"parentId\" which must be equal to \"cOID\" of parent. 2. When you cancel a parent order, it will cancel all bracket orders, when you cancel one child order, it will also cancel its sibling order.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderRequest>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreviewOrderResponse {
    #[serde(rename = "amount")]
    pub amount: Option<PreviewOrderAmount>,
    #[serde(rename = "equity")]
    pub equity: Option<PreviewOrderEquity>,
    #[serde(rename = "initial")]
    pub initial: Option<PreviewOrderEquity>,
    #[serde(rename = "maintenance")]
    pub maintenance: Option<PreviewOrderEquity>,
    #[serde(rename = "warn")]
    pub warn: Option<String>,
    #[serde(rename = "error")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CancelOrderRequest {
    pub account_id: String,
    pub order_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CancelOrderResponse {
    pub order_id: Option<i64>,
    pub msg: Option<String>,
    pub conid: Option<i64>,
    pub account: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrderReplyRequest {
    #[serde(skip)]
    pub reply_id: String,
    /// answer to question, true means yes, false means no
    #[serde(rename = "confirmed")]
    pub confirmed: bool,
}

pub type PlaceOrderReplyResponse = PlaceOrdersResponse;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrdersRequest {
    #[serde(skip)]
    pub account_id: String,
    /// Notes for bracket orders: 1. Children orders will not have its own \"cOID\", so please donot pass \"cOID\" parameter in child order.Instead, they will have a \"parentId\" which must be equal to \"cOID\" of parent. 2. When you cancel a parent order, it will cancel all bracket orders, when you cancel one child order, it will also cancel its sibling order.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderRequest>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlaceOrdersResponse {
    Ok(Vec<PlaceOrModifyOrderResult>),
    Error(PlaceOrModifyOrderError),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrModifyOrderError {
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PlaceOrModifyOrderResult {
    Success(PlaceOrModifyOrderInfo),
    Question(PlaceOrModifyOrderQuestion),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrModifyOrderInfo {
    pub order_id: String,
    pub local_order_id: Option<String>,
    pub order_status: Option<String>,
    pub parent_order_id: Option<String>,
    pub encrypt_message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrModifyOrderQuestion {
    #[serde(rename = "id")]
    pub id: String,
    /// Please note here, if the message is a question, you have to reply to question in order to submit the order successfully. See more in the \"/iserver/reply/{replyid}\" endpoint.
    #[serde(rename = "message")]
    pub message: Vec<String>,
    #[serde(rename = "isSuppressed")]
    pub is_suppressed: Option<bool>,
    #[serde(rename = "messageIds")]
    pub message_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlaceOrderForFinancialAdvisorsRequest {
    pub financial_advisors_group: String,
    pub order: OrderRequest,
}

pub type PlaceOrderForFinancialAdvisorsResponse = PlaceOrdersResponse;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModifyOrderRequest {
    #[serde(skip)]
    pub account_id_or_financial_advisors_group: String,
    #[serde(skip)]
    pub order_id: String,

    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i64>,
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// for example LMT
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "outsideRTH", skip_serializing_if = "Option::is_none")]
    pub outside_regular_trading_hours: Option<bool>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(rename = "auxPrice", skip_serializing_if = "Option::is_none")]
    pub aux_price: Option<Decimal>,
    /// SELL or BUY
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// optional, not required
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// The ticker symbol of the original place order
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Specify a time in force to change how long your order will continue to work in the market
    #[serde(rename = "tif", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// usually integer, for some special cases can be float numbers
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    /// Set to true if you want to pause a working order. For details refer to the [TWS Users' Guide:](https://guides.interactivebrokers.com/tws/twsguide.html#usersguidebook/getstarted/pause_execution.htm)
    #[serde(rename = "deactivated", skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
    #[serde(rename = "useAdaptive", skip_serializing_if = "Option::is_none")]
    pub use_adaptive: Option<bool>,
    #[serde(rename = "lmtOffset", skip_serializing_if = "Option::is_none")]
    pub limit_offset: Option<Decimal>,
    /// optional if order is TRAIL, or TRAILLMT. When trailingType is amt, this is the trailing amount, when trailingType is %, it means percentage. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingAmt", skip_serializing_if = "Option::is_none")]
    pub trailing_amount: Option<Decimal>,
    /// This is the trailing type for trailing amount. We only support two types here: amt or %. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order
    #[serde(rename = "trailingType", skip_serializing_if = "Option::is_none")]
    pub trailing_type: Option<String>,
}

pub type ModifyOrderResponse = PlaceOrdersResponse;
