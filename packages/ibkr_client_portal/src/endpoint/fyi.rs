// https://www.interactivebrokers.com/api/doc.html#tag/FYI

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::fyi::{
        GetFYIDisclaimerInfoRequest, GetFYIDisclaimerInfoResponse, GetFYISettingsResponse,
        GetFYIUnreadNumberResponse,
    },
};

impl IBClientPortal {
    // todo
    // /fyi/settings/{typecode}
    // PUT /fyi/disclaimer/{typecode}
    // /fyi/deliveryoptions
    // PUT /fyi/deliveryoptions/email
    // /fyi/deliveryoptions/device
    // DELETE /fyi/deliveryoptions/{deviceId}
    // /fyi/notifications
    // /fyi/notifications/more
    // /fyi/notifications/{notificationId}

    pub async fn fyi_unread_number(&self) -> Result<GetFYIUnreadNumberResponse, Error> {
        let path = "/fyi/unreadnumber";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn fyi_settings(&self) -> Result<GetFYISettingsResponse, Error> {
        let path = "/fyi/settings";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn fyi_disclaimer_info(
        &self,
        request: GetFYIDisclaimerInfoRequest,
    ) -> Result<GetFYIDisclaimerInfoResponse, Error> {
        let path = format!("/fyi/disclaimer/{}", request.type_code);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
