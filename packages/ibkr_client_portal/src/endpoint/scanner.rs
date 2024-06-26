// https://www.interactivebrokers.com/api/doc.html#tag/Scanner/paths/~1hmds~1scanner/post

use reqwest_middleware::Error;

use crate::{
    client::IBClientPortal,
    model::scanner::{
        GetScannerParametersResponse, RunScannerBetaRequest, RunScannerBetaResponse,
        ScannerRunRequest, ScannerRunResponse,
    },
};

impl IBClientPortal {
    /// Returns an object contains four lists contain all parameters for scanners
    pub async fn get_scanner_parameters(&self) -> Result<GetScannerParametersResponse, Error> {
        let path = "/iserver/scanner/params";
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

    /// (Beta) Using a direct connection to the market data farm, will provide results to the requested scanner.
    pub async fn run_scanner_beta(
        &self,
        request: RunScannerBetaRequest,
    ) -> Result<RunScannerBetaResponse, Error> {
        let path = "/hmds/scanner";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Searches for contracts according to the filters specified in scanner/params endpoint
    pub async fn scanner_run(
        &self,
        request: ScannerRunRequest,
    ) -> Result<ScannerRunResponse, Error> {
        let path = "/iserver/scanner/run";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }
}
