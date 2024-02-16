// https://www.interactivebrokers.com/api/doc.html#tag/Alert

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::alert::{
        DeleteAlertRequest, DeleteAlertResponse, GetAlertDetailsRequest, GetAlertDetailsResponse,
        GetListOfAvailableAlertsRequest, GetListOfAvailableAlertsResponse, GetMTAAlertResponse,
        TogglerAlertActivationRequest, TogglerAlertActivationResponse,
    },
};

impl IBClientPortal {
    /// The response will contain both active and inactive alerts, but it won't have MTA alert
    pub async fn get_list_of_available_alerts(
        &self,
        request: GetListOfAvailableAlertsRequest,
    ) -> Result<GetListOfAvailableAlertsResponse, Error> {
        let path = format!("/iserver/account/{}/alerts", request.account_id);
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Each login user only has one mobile trading assistant (MTA) alert with it's own unique tool id. The tool id cannot be changed. When modified a new order Id is generated. MTA alerts can not be created or deleted. If you call delete /iserver/account/:accountId/alert/:alertId, it will reset MTA to default. See here for more information on MTA alerts.
    pub async fn get_mobile_trading_assistant_alert(&self) -> Result<GetMTAAlertResponse, Error> {
        let path = "/iserver/account/mta";
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Use the endpoint /iserver/account/:accountId/alerts to receive the alert id. The order_id in the response is the alert id.
    pub async fn get_alert_details(
        &self,
        request: GetAlertDetailsRequest,
    ) -> Result<GetAlertDetailsResponse, Error> {
        let path = format!("/iserver/account/alert/{}", request.alert_id);
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Please be careful, if alertId is 0, it will delete all alerts
    pub async fn delete_alert(
        &self,
        request: DeleteAlertRequest,
    ) -> Result<DeleteAlertResponse, Error> {
        let path = format!(
            "/iserver/account/{}/alert/{}",
            request.account_id, request.alert_id
        );
        let response = self
            .client
            .delete(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Please note, if alertId is 0, it will activate/deactivate all alerts
    pub async fn toggle_alert_activation(
        &self,
        request: TogglerAlertActivationRequest,
    ) -> Result<TogglerAlertActivationResponse, Error> {
        let path = format!("/iserver/account/{}/alert/activate", request.account_id);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Please note here, DO NOT pass orderId when creating a new alert, toolId is only required for MTA alert
    pub async fn upsert_alert(
        &self,
        request: TogglerAlertActivationRequest,
    ) -> Result<TogglerAlertActivationResponse, Error> {
        let path = format!("/iserver/account/{}/alert", request.account_id);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
