use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerType {
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "instruments")]
    pub instruments: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Instrument {
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Filter {
    #[serde(rename = "group")]
    pub group: Option<String>,
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Location {
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocationTree {
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "locations")]
    pub locations: Option<Vec<Location>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetScannerParametersResponse {
    #[serde(rename = "scan_type_list")]
    pub scan_type_list: Vec<ScannerType>,
    #[serde(rename = "instrument_list")]
    pub instrument_list: Vec<Instrument>,
    #[serde(rename = "filter_list")]
    pub filter_list: Vec<Filter>,
    #[serde(rename = "location_tree")]
    pub location_tree: Vec<LocationTree>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerResultContract {
    #[serde(rename = "inScanTime")]
    pub in_scan_time: Option<String>,
    #[serde(rename = "distance")]
    pub distance: Option<i32>,
    #[serde(rename = "contractID")]
    pub contract_id: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HmdsScannerFilter {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Value can be either an integer, double, boolean or a string depending upon the type of filter specified in the code section
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RunScannerBetaRequest {
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<String>,
    #[serde(rename = "scanCode", skip_serializing_if = "Option::is_none")]
    pub scan_code: Option<String>,
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HmdsScannerFilter>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerResultContracts {
    #[serde(rename = "Contract")]
    pub contract: Vec<ScannerResultContract>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RunScannerBetaResponse {
    #[serde(rename = "total")]
    pub total: Option<i32>,
    #[serde(rename = "size")]
    pub size: Option<i32>,
    #[serde(rename = "offset")]
    pub offset: Option<i32>,
    #[serde(rename = "scanTime")]
    pub scan_time: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<i64>,
    #[serde(rename = "position")]
    pub position: Option<String>,
    #[serde(rename = "Contracts")]
    pub contracts: Option<ScannerResultContracts>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerRunContract {
    #[serde(rename = "server_id")]
    pub server_id: Option<String>,
    #[serde(rename = "column_name")]
    pub column_name: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    #[serde(rename = "con_id")]
    pub conid: Option<i64>,
    /// List of available chart periods
    #[serde(rename = "available_chart_periods")]
    pub available_chart_periods: Option<String>,
    /// Contracts company name
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    /// Format contract name
    #[serde(rename = "contract_description_1")]
    pub contract_description_1: Option<String>,
    #[serde(rename = "listing_exchange")]
    pub listing_exchange: Option<String>,
    #[serde(rename = "sec_type")]
    pub sec_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerRunResponse {
    pub contracts: Vec<ScannerRunContract>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerFilter {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannerRunRequest {
    /// Contains an instrument, which to scan for. For example - \"STK\"
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    /// Specify the scan type to use. For example - \"MOST_ACTIVE_USD\"
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Contains location code, where to look for specified instrument. For example - \"STK.US.MAJOR\"
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Contains list of filters supported for the scanner
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ScannerFilter>>,
}
