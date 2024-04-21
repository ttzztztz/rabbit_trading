// https://www.interactivebrokers.com/api/doc.html#tag/Session

use reqwest_middleware::Error;

use crate::{
    client::IBClientPortal,
    model::session::{AuthStatus, LogoutResponse, SSOValidateResponse, Tickle},
};

impl IBClientPortal {
    /// If the gateway has not received any requests for several minutes an open session will automatically timeout. The tickle endpoint pings the server to prevent the session from ending. It is expected to call this endpoint approximately every 60 seconds to maintain the connection to the brokerage session.
    pub async fn tickle(&self) -> Result<Tickle, Error> {
        let response = self
            .client
            .post(self.get_url("/tickle"))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Current Authentication status to the Brokerage system. Market Data and Trading is not possible if not authenticated, e.g. authenticated shows false
    pub async fn get_auth_status(&self) -> Result<AuthStatus, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/auth/status"))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    ///  After retrieving the access token and subsequent Live Session Token, customers can initialize their brokerage session with the ssodh/init endpoint.
    /// NOTE: This is essential for using all /iserver endpoints, including access to trading and market data,
    pub async fn init_brokerage_session(&self) -> Result<AuthStatus, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/auth/ssodh/init"))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Logs the user out of the gateway session. Any further activity requires re-authentication.
    pub async fn logout(&self) -> Result<LogoutResponse, Error> {
        let response = self
            .client
            .post(self.get_url("/logout"))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Validates the current session for the SSO user
    pub async fn sso_validate(&self) -> Result<SSOValidateResponse, Error> {
        let path = "/sso/validate";
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// When using the CP Gateway, this endpoint provides a way to reauthenticate to the Brokerage system as long as there is a valid SSO session, see /sso/validate.
    pub async fn reauthenticate(&self) -> Result<AuthStatus, Error> {
        let path = "/iserver/reauthenticate";
        let response = self
            .client
            .post(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }
}
