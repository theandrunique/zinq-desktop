use std::{sync::Arc, time::Duration};

use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::ApiError;

#[derive(Debug)]
pub enum ClientError {
    Network(reqwest::Error),
    Api(ApiError),
    Serialization(reqwest::Error),
    UnexpectedStatus(StatusCode, String),
}

type TokenProvider = Arc<dyn Fn() -> Option<String> + Send + Sync>;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token_provider: Option<TokenProvider>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            base_url,
            token_provider: None,
        }
    }

    pub fn set_token_provider<F>(&mut self, provider: F)
    where
        F: Fn() -> Option<String> + Send + Sync + 'static,
    {
        self.token_provider = Some(Arc::new(provider));
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

    fn get_token(&self) -> Option<String> {
        self.token_provider.as_ref().and_then(|f| f())
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let token = self.get_token();
        let request = self.client.get(self.build_url(endpoint));

        if token.is_some() {
            self.execute(request.bearer_auth(token.unwrap())).await
        } else {
            self.execute(request).await
        }
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let token = self.get_token();
        let request = self.client.post(self.build_url(endpoint)).json(body);

        if token.is_some() {
            self.execute(request.bearer_auth(token.unwrap())).await
        } else {
            self.execute(request).await
        }
    }

    async fn execute<T>(&self, request: RequestBuilder) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await.map_err(ClientError::Network)?;

        if response.status().is_success() {
            match response.json::<T>().await {
                Ok(data) => Ok(data),
                Err(e) => {
                    if e.is_decode() {
                        Err(ClientError::Serialization(e))
                    } else {
                        Err(ClientError::Network(e))
                    }
                }
            }
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<empty or incorrect body response>".to_string());

            match serde_json::from_str::<ApiError>(&body) {
                Ok(api_err) => Err(ClientError::Api(api_err)),
                Err(_) => Err(ClientError::UnexpectedStatus(status, body)),
            }
        }
    }
}
