use crate::github::Pat;
use axum::response;
use graphql_client::{GraphQLQuery, Response};
use reqwest::header::{self, HeaderMap, HeaderValue};
use std::time::Duration;

use crate::github::gql::query;

use super::gql::query::authenticate::Variables;

pub struct GithubClient {
    client: reqwest::Client,
}

impl GithubClient {
    const ENDPOINT: &'static str = "https://api.github.com/graphql";

    /// Constructs a new `GithubClient`.
    pub fn new(pat: Option<Pat>) -> anyhow::Result<Self> {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let mut headers = HeaderMap::new();
        if let Some(pat) = pat {
            let mut token = HeaderValue::from_str(&format!("bearer {}", pat.token))?;
            token.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, token);
        }
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    /// Authenticates the client and returns the login name of the authenticated user.
    pub async fn authenticate(&self) -> anyhow::Result<String> {
        let variables = query::authenticate::Variables {};
        let req = query::Authenticate::build_query(variables);
        let response: query::authenticate::ResponseData = self.request(req).await?;
        Ok(response.viewer.login)
    }

    /// Executes a GraphQL query.
    async fn request<Body, ResponseData>(&self, body: Body) -> anyhow::Result<ResponseData>
    where
        Body: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned,
    {
        let response: Response<ResponseData> = self
            .client
            .post(Self::ENDPOINT)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        match (response.data, response.errors) {
            (Some(data), None) => Ok(data),
            (_, Some(errors)) => Err(anyhow::anyhow!("failed to execute query: {:?}", errors)),
            _ => Err(anyhow::anyhow!("unexpected response format")),
        }
    }
}
