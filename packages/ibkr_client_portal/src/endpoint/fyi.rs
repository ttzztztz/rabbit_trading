// https://www.interactivebrokers.com/api/doc.html#tag/FYI

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::fyi::{
        DeleteFYIDeliveryOptionsForDeviceRequest, GetFYIDisclaimerRequest,
        GetFYIDisclaimerResponse, GetFYISettingsResponse, GetFYIUnreadNumberResponse,
        GetMoreNotificationListRequest, GetMoreNotificationListResponse,
        GetNotificationListRequest, GetNotificationListResponse, ReadFYIDisclaimerRequest,
        ReadFYIDisclaimerResponse, ReadNotificationRequest,
        ToggleFYIDeliveryOptionsForDeviceRequest, ToggleFYIDeliveryOptionsForDeviceResponse,
        ToggleFYIDeliveryOptionsForEmailRequest, ToggleFYIDeliveryOptionsForEmailResponse,
        ToggleFYISettingsRequest,
    },
};

impl IBClientPortal {
    /// Returns the total number of unread FYIs
    pub async fn get_fyi_unread_number(&self) -> Result<GetFYIUnreadNumberResponse, Error> {
        let path = "/fyi/unreadnumber";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Return the current choices of subscriptions, we can toggle the option
    pub async fn get_fyi_settings(&self) -> Result<GetFYISettingsResponse, Error> {
        let path = "/fyi/settings";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Configure which typecode you would like to enable/disable.
    pub async fn toggle_fyi_setting(&self, request: ToggleFYISettingsRequest) -> Result<(), Error> {
        let path = format!("/fyi/settings/{}", request.type_code);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        Result::Ok(())
    }

    /// Get disclaimer for a certain kind of fyi
    pub async fn get_fyi_disclaimer(
        &self,
        request: GetFYIDisclaimerRequest,
    ) -> Result<GetFYIDisclaimerResponse, Error> {
        let path = format!("/fyi/disclaimer/{}", request.type_code);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Get disclaimer for a certain kind of fyi
    pub async fn read_fyi_disclaimer(
        &self,
        request: ReadFYIDisclaimerRequest,
    ) -> Result<ReadFYIDisclaimerResponse, Error> {
        let path = format!("/fyi/disclaimer/{}", request.type_code);
        let response = self.client.put(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Get a list of notifications
    pub async fn get_notification_list(
        &self,
        request: GetNotificationListRequest,
    ) -> Result<GetNotificationListResponse, Error> {
        let path = "/fyi/notifications";

        let mut query = vec![("max", request.max.to_string())];
        if let Some(exclude) = request.exclude {
            query.push(("exclude", exclude));
        }
        if let Some(include) = request.include {
            query.push(("include", include));
        }
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Returns the total number of unread FYIs
    pub async fn get_more_notification_list(
        &self,
        request: GetMoreNotificationListRequest,
    ) -> Result<GetMoreNotificationListResponse, Error> {
        let path = "/fyi/notifications/more";
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&[("id", request.id)])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Options for sending FYIe to email and other devices
    pub async fn get_fyi_delivery_options(&self) -> Result<GetFYIDisclaimerResponse, Error> {
        let path = "/fyi/deliveryoptions";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Enable/Disable email option
    pub async fn toggle_fyi_delivery_options_for_email(
        &self,
        request: ToggleFYIDeliveryOptionsForEmailRequest,
    ) -> Result<ToggleFYIDeliveryOptionsForEmailResponse, Error> {
        let path = "/fyi/deliveryoptions/email";
        let response = self
            .client
            .put(self.get_url(&path))
            .query(&["enabled", request.enabled.to_string().as_str()])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Enable/Disable device option
    pub async fn toggle_fyi_delivery_options_for_device(
        &self,
        request: ToggleFYIDeliveryOptionsForDeviceRequest,
    ) -> Result<ToggleFYIDeliveryOptionsForDeviceResponse, Error> {
        let path = "/fyi/deliveryoptions/device";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Delete a device
    pub async fn delete_fyi_delivery_options_for_device(
        &self,
        request: DeleteFYIDeliveryOptionsForDeviceRequest,
    ) -> Result<(), Error> {
        let path = format!("/fyi/deliveryoptions/{}", request.device_id);
        let response = self.client.delete(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        Result::Ok(())
    }

    /// mark a notification read
    pub async fn read_notification(&self, request: ReadNotificationRequest) -> Result<(), Error> {
        let path = format!("/fyi/notifications/{}", request.id);
        let response = self.client.put(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
