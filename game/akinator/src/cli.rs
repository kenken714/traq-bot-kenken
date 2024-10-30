use super::{Akinator, AkinatorError, AkinatorResponse};
use axum::extract::path;
use reqwest::Response;
use serde::Serialize;

impl Akinator {
    //Akinatorにリクエストを送信
    pub async fn send_request(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<Response, AkinatorError> {
        let response = reqwest::Client::new()
            .post(self.params.url.join(path).unwrap())
            .json(body)
            .send()
            .await
            .map_err(|_| AkinatorError::ConnectionError)?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(AkinatorError::InvalidResponse)
        }
    }

    pub async fn send_ingame_request(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<AkinatorResponse, AkinatorError> {
        let response = self.send_request(path, body).await?;
        let response = response.json::<AkinatorResponse>().await.unwrap();

        Ok(response)
    }
}
