// https://www.interactivebrokers.com/api/doc.html#tag/Session

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::session::{AuthStatus, LogoutResponse, SSOValidateResponse, Tickle},
};

impl IBClientPortal {
    pub async fn tickle(&self) -> Result<Tickle, Error> {
        let response = self
            .client
            .post(self.get_url("/tickle"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn auth_status(&self) -> Result<AuthStatus, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/auth/status"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn logout(&self) -> Result<LogoutResponse, Error> {
        let response = self
            .client
            .post(self.get_url("/logout"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn sso_validate(&self) -> Result<SSOValidateResponse, Error> {
        let path = "/sso/validate";
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn reauthenticate(&self) -> Result<AuthStatus, Error> {
        let path = "/iserver/reauthenticate";
        let response = self
            .client
            .post(self.get_url(&path))
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
