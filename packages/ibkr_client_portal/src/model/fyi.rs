use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYIUnreadNumberResponse {
    /// unread number
    #[serde(rename = "BN")]
    pub bn: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FYISetting {
    /// optional, if A doesn't exist, it means user can't toggle this option. 0-off, 1-on.
    #[serde(rename = "A")]
    pub A: Option<i32>,
    /// fyi code
    #[serde(rename = "FC")]
    pub fyi_code: Option<String>,
    /// disclaimer read, 1 = yes, = 0 no.
    #[serde(rename = "H")]
    pub disclaimer_read: Option<i32>,
    /// detailed description
    #[serde(rename = "FD")]
    pub detailed_description: Option<String>,
    /// title
    #[serde(rename = "FN")]
    pub title: Option<String>,
}

pub type GetFYISettingsResponse = Vec<FYISetting>;

pub struct GetFYIDisclaimerRequest {
    pub type_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYIDisclaimerResponse {
    /// disclaimer message
    #[serde(rename = "DT")]
    pub disclaimer_message: Option<String>,
    /// fyi code
    #[serde(rename = "FC")]
    pub fyi_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFYISettingsRequest {
    #[serde(skip)]
    pub type_code: String,
    pub enabled: bool,
}

pub struct ReadFYIDisclaimerRequest {
    pub type_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadFYIDisclaimerResponse {
    #[serde(rename = "T")]
    pub T: Option<i32>,
    #[serde(rename = "V")]
    pub V: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    /// notification date
    #[serde(rename = "D")]
    pub date: Option<String>,
    /// unique way to reference this notification
    #[serde(rename = "ID")]
    pub id: String,
    /// FYI code, we can use it to find whether the disclaimer is accepted or not in settings
    #[serde(rename = "FC")]
    pub fyi_code: Option<String>,
    /// content of notification
    #[serde(rename = "MD")]
    pub content: Option<String>,
    /// title of notification
    #[serde(rename = "MS")]
    pub title: Option<String>,
    /// 0-unread, 1-read
    #[serde(rename = "R")]
    pub is_read: Option<String>,
}

pub type GetNotificationListResponse = Vec<Notification>;
pub type GetMoreNotificationListResponse = Vec<Notification>;

pub struct ReadNotificationRequest {
    pub id: String,
}

pub struct GetMoreNotificationListRequest {
    pub id: String,
}

pub struct GetNotificationListRequest {
    /// max number of fyis in response, max <= 10
    pub max: i64,
    /// if set, don't set include
    pub exclude: Option<String>,
    /// if set, don't set exclude
    pub include: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryOptionDevice {
    /// device name
    #[serde(rename = "NM")]
    pub device_name: Option<String>,
    /// device id
    #[serde(rename = "I")]
    pub device_id: Option<String>,
    /// unique device id
    #[serde(rename = "UI")]
    pub unique_device_id: Option<String>,
    /// device is enabled or not 0-true, 1-false.
    #[serde(rename = "A")]
    pub is_enabled: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYIDeliveryOptionsResponse {
    /// Email option is enabled or not 0-off, 1-on.
    #[serde(rename = "M")]
    pub email_option: Option<i32>,
    #[serde(rename = "E")]
    pub device_list: Option<Vec<DeliveryOptionDevice>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFYIDeliveryOptionsForEmailResponse {
    #[serde(rename = "T")]
    pub T: Option<i32>,
    #[serde(rename = "V")]
    pub V: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFYIDeliveryOptionsForDeviceResponse {
    #[serde(rename = "T")]
    pub T: Option<i32>,
    #[serde(rename = "V")]
    pub V: Option<i32>,
}

pub struct DeleteFYIDeliveryOptionsForDeviceRequest {
    pub device_id: String,
}

pub struct ToggleFYIDeliveryOptionsForEmailRequest {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleFYIDeliveryOptionsForDeviceRequest {
    #[serde(rename = "devicename", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "uiName", skip_serializing_if = "Option::is_none")]
    pub ui_name: Option<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
