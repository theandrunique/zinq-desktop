use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::{ApiError, ErrorCode};

#[derive(Debug)]
pub enum ClientError {
    Network(reqwest::Error),
    Api(ApiError),
    Serialization(reqwest::Error),
    UnexpectedStatus(StatusCode, String),
}

pub type TokenProvider = Arc<dyn Fn() -> Option<String> + Send + Sync>;
pub type RefreshProvider =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync>;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token_provider: Option<TokenProvider>,
    refresh_provider: Option<RefreshProvider>,
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
            refresh_provider: None,
        }
    }

    pub fn set_token_provider<F>(&mut self, provider: F)
    where
        F: Fn() -> Option<String> + Send + Sync + 'static,
    {
        self.token_provider = Some(Arc::new(provider));
    }

    pub fn set_refresh_provider<F>(&mut self, provider: F)
    where
        F: Fn() -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync + 'static,
    {
        self.refresh_provider = Some(Arc::new(provider));
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

    fn get_token(&self) -> Option<String> {
        self.token_provider.as_ref().and_then(|f| f())
    }

    fn should_refresh(&self, err: &ClientError) -> bool {
        if self.refresh_provider.is_none() {
            return false;
        }
        match err {
            ClientError::Api(api_err) => matches!(api_err.code, ErrorCode::AuthInvalidToken),
            ClientError::UnexpectedStatus(status, _) => *status == StatusCode::UNAUTHORIZED,
            _ => false,
        }
    }

    fn build_request(&self, method: &str, url: &str) -> RequestBuilder {
        match method {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "PATCH" => self.client.patch(url),
            "DELETE" => self.client.delete(url),
            _ => unreachable!("unsupported HTTP method: {method}"),
        }
    }

    async fn send_inner<T, B>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<&B>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        tracing::debug!(method, endpoint, "Sending request");

        let try_send = |token: Option<String>| {
            let mut req = self.build_request(method, &self.build_url(endpoint));
            if let Some(b) = body {
                req = req.json(b);
            }
            if let Some(t) = token {
                req = req.bearer_auth(t);
            }
            req
        };

        let token = self.get_token();
        let result = self.execute::<T>(try_send(token)).await;

        if let Err(ref err) = result {
            if self.should_refresh(err) {
                tracing::warn!(endpoint, "Token expired, attempting refresh");
                let refresh = self.refresh_provider.as_ref().unwrap();
                if refresh().await {
                    tracing::info!(endpoint, "Token refreshed, retrying request");
                    let new_token = self.get_token();
                    return self.execute::<T>(try_send(new_token)).await;
                }
                tracing::error!(endpoint, "Token refresh failed");
            }
        }

        match &result {
            Ok(_) => tracing::debug!(method, endpoint, "Request succeeded"),
            Err(e) => tracing::warn!(method, endpoint, error = ?e, "Request failed"),
        }

        result
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_inner("GET", endpoint, None::<&()>).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner("POST", endpoint, Some(body)).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner("PUT", endpoint, Some(body)).await
    }

    pub async fn patch<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner("PATCH", endpoint, Some(body)).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_inner("DELETE", endpoint, None::<&()>).await
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
