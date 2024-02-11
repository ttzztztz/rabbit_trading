use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertCondition {
    /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN&
    #[serde(rename = "condition_type")]
    pub condition_type: Option<i32>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    /// Format contract name
    #[serde(rename = "contract_description_1")]
    pub contract_description_1: Option<String>,
    /// optional, operator for the current condition   * >= Greater than or equal to   * <= Less than or equal to
    #[serde(rename = "condition_operator")]
    pub condition_operator: Option<String>,
    /// optional, only some type of conditions have triggerMethod
    #[serde(rename = "condition_trigger_method")]
    pub condition_trigger_method: Option<String>,
    /// can not be empty, can pass default value \"*\"
    #[serde(rename = "condition_value")]
    pub condition_value: Option<String>,
    /// Condition array should end with \"n\"   * a - AND   * o - OR   * n - END
    #[serde(rename = "condition_logic_bind")]
    pub condition_logic_bind: Option<String>,
    /// only needed for some MTA alert condition
    #[serde(rename = "condition_time_zone")]
    pub condition_time_zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    /// account id
    #[serde(rename = "account")]
    pub account: Option<String>,
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    /// name of alert
    #[serde(rename = "alert_name")]
    pub alert_name: Option<String>,
    /// The message you want to receive via email or text message
    #[serde(rename = "alert_message")]
    pub alert_message: Option<String>,
    /// whether alert is active or not, so value can only be 0 or 1
    #[serde(rename = "alert_active")]
    pub alert_active: Option<i32>,
    /// whether alert is repeatable or not, so value can only be 0 or 1
    #[serde(rename = "alert_repeatable")]
    pub alert_repeatable: Option<i32>,
    /// email address to receive alert
    #[serde(rename = "alert_email")]
    pub alert_email: Option<String>,
    /// whether allowing to send email or not, so value can only be 0 or 1,
    #[serde(rename = "alert_send_message")]
    pub alert_send_message: Option<i32>,
    /// time in force, can only be GTC or GTD
    #[serde(rename = "tif")]
    pub tif: Option<String>,
    /// format, YYYYMMDD-HH:mm:ss
    #[serde(rename = "expire_time")]
    pub expire_time: Option<String>,
    /// status of alert
    #[serde(rename = "order_status")]
    pub order_status: Option<String>,
    /// value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.
    #[serde(rename = "outsideRth")]
    pub outside_rth: Option<i32>,
    /// value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile
    #[serde(rename = "itws_orders_only")]
    pub itws_orders_only: Option<i32>,
    /// value can only be 0 or 1, set to 1 to allow to show alert in pop-ups
    #[serde(rename = "alert_show_popup")]
    pub alert_show_popup: Option<i32>,
    /// whether the alert has been triggered
    #[serde(rename = "alert_triggered")]
    pub alert_triggered: Option<bool>,
    /// whether the alert can be edited
    #[serde(rename = "order_not_editable")]
    pub order_not_editable: Option<bool>,
    /// for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert
    #[serde(rename = "tool_id")]
    pub tool_id: Option<i64>,
    /// audio message to play when alert is triggered
    #[serde(rename = "alert_play_audio")]
    pub alert_play_audio: Option<String>,
    /// MTA alert only
    #[serde(rename = "alert_mta_currency")]
    pub alert_mta_currency: Option<String>,
    /// MTA alert only
    #[serde(rename = "alert_mta_defaults")]
    pub alert_mta_defaults: Option<String>,
    /// MTA alert only
    #[serde(rename = "time_zone")]
    pub time_zone: Option<String>,
    /// MTA alert only
    #[serde(rename = "alert_default_type")]
    pub alert_default_type: Option<String>,
    /// size of conditions array
    #[serde(rename = "condition_size")]
    pub condition_size: Option<i32>,
    /// whether allowing the condition can be triggered outside of regular trading hours, 1 means allow
    #[serde(rename = "condition_outside_rth")]
    pub condition_outside_rth: Option<i32>,
    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<AlertCondition>>,
}

pub type GetMTAAlertResponse = Alert;
pub type GetAlertDetailsResponse = Alert;

pub struct GetAlertDetailsRequest {
    pub alert_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertSummary {
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    /// account id
    #[serde(rename = "account")]
    pub account: Option<i64>,
    #[serde(rename = "alert_name")]
    pub alert_name: Option<String>,
    /// Value can only be 0 or 1, 1 means active
    #[serde(rename = "alert_active")]
    pub alert_active: Option<i32>,
    /// format, YYYYMMDD-HH:mm:ss, the time when you created the alert
    #[serde(rename = "order_time")]
    pub order_time: Option<String>,
    /// whether the alert has been triggered or not
    #[serde(rename = "alert_triggered")]
    pub alert_triggered: Option<bool>,
    /// whether the alert can be repeatable or not, value can be 1 or 0. 1 means true
    #[serde(rename = "alert_repeatable")]
    pub alert_repeatable: Option<i32>,
}

pub struct GetListOfAvailableAlertsRequest {
    pub account_id: i64,
}

pub type GetListOfAvailableAlertsResponse = Vec<AlertSummary>;

pub struct DeleteAlertRequest {
    pub account_id: i64,
    pub alert_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAlertResponse {
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    #[serde(rename = "msg")]
    pub msg: Option<String>,
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    #[serde(rename = "account")]
    pub account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TogglerAlertActivationRequest {
    #[serde(skip)]
    pub account_id: i64,
    /// alert id(order id)
    #[serde(rename = "alertId")]
    pub alert_id: i64,
    /// 1 to activate, 0 to deactivate
    #[serde(rename = "alertActive")]
    pub alert_active: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TogglerAlertActivationResponse {
    #[serde(rename = "request_id")]
    pub request_id: i64,
    #[serde(rename = "order_id")]
    pub order_id: i64,
    #[serde(rename = "success")]
    pub success: Option<bool>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "order_status")]
    pub order_status: Option<String>,
    #[serde(rename = "failure_list")]
    pub failure_list: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertAlertCondition {
    /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN&
    #[serde(rename = "type")]
    pub _type: Option<i32>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    /// optional, operator for the current condition, can be >= or <=
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// optional, only some type of conditions have triggerMethod
    #[serde(rename = "triggerMethod")]
    pub trigger_method: Option<String>,
    /// can not be empty, can pass default value \"*\"
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// \"a\" means \"AND\", \"o\" means \"OR\", \"n\" means \"END\", the last one condition in the condition array should \"n\"
    #[serde(rename = "logicBind")]
    pub logic_bind: Option<String>,
    /// only needed for some MTA alert condition
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertAlertRequest {
    #[serde(skip)]
    pub account_id: i64,
    /// orderId is required when modifying alert. You can get it from /iserver/account/:accountId/alerts
    #[serde(rename = "orderId")]
    pub order_id: Option<i64>,
    /// name of alert
    #[serde(rename = "alertName")]
    pub alert_name: Option<String>,
    /// The message you want to receive via email or text message
    #[serde(rename = "alertMessage")]
    pub alert_message: Option<String>,
    /// whether alert is repeatable or not, so value can only be 0 or 1, this has to be 1 for MTA alert
    #[serde(rename = "alertRepeatable")]
    pub alert_repeatable: Option<i32>,
    /// email address to receive alert
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// whether allowing to send email or not, so value can only be 0 or 1,
    #[serde(rename = "sendMessage")]
    pub send_message: Option<i32>,
    /// time in force, can only be GTC or GTD
    #[serde(rename = "tif")]
    pub tif: Option<String>,
    /// format, YYYYMMDD-HH:mm:ss, please NOTE this will only work when tif is GTD
    #[serde(rename = "expireTime")]
    pub expire_time: Option<String>,
    /// value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.
    #[serde(rename = "outsideRth")]
    pub outside_rth: Option<i32>,
    /// value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile
    #[serde(rename = "iTWSOrdersOnly")]
    pub i_tws_orders_only: Option<i32>,
    /// value can only be 0 or 1, set to 1 to allow to show alert in pop-ups
    #[serde(rename = "showPopup")]
    pub show_popup: Option<i32>,
    /// for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert
    #[serde(rename = "toolId")]
    pub tool_id: Option<i64>,
    /// audio message to play when alert is triggered
    #[serde(rename = "playAudio")]
    pub play_audio: Option<String>,
    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<UpsertAlertCondition>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertAlertResponse {
    #[serde(rename = "request_id")]
    pub request_id: Option<i64>,
    #[serde(rename = "order_id")]
    pub order_id: Option<i64>,
    #[serde(rename = "success")]
    pub success: Option<bool>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "order_status")]
    pub order_status: Option<String>,
    #[serde(rename = "warning_message")]
    pub warning_message: Option<String>,
}
