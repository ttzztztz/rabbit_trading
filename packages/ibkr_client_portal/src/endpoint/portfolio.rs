// https://www.interactivebrokers.com/api/doc.html#tag/Portfolio

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::position::{GetPositionsRequest, GetPositionsResponse},
};

impl IBClientPortal {
    pub async fn get_positions(
        &self,
        request: GetPositionsRequest,
    ) -> Result<GetPositionsResponse, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, request.page);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    // todo
    // /portfolio/allocation
    // /portfolio/{accountId}/position/{conid}
    // /portfolio/{accountId}/positions/invalidate
    // /portfolio/positions/{conid}
}
