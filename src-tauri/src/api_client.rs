use std::{
    future::Future, pin::Pin, sync::{Arc, Mutex, OnceLock}, time::{Duration, Instant},
};

use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex as TokioMutex;
use tracing_subscriber::field::MakeExt;

use crate::errors::{ApiError, ErrorCode};

#[derive(Debug)]
pub enum ClientError {
    Network(reqwest::Error),
    Api(ApiError),
    Serialization(reqwest::Error),
    UnexpectedStatus(StatusCode, String),
}

pub type TokenProvider = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Option<String>> + Send>> + Send + Sync>;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token_provider: OnceLock<TokenProvider>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            base_url,
            token_provider: OnceLock::new(),
        }
    }

    pub fn set_token_provider<F>(&self, provider: F)
    where
        F: Fn() -> Pin<Box<dyn Future<Output = Option<String>> + Send>> + Send + Sync + 'static,
    {
        self.token_provider
            .set(Arc::new(provider))
            .unwrap_or_else(|_| panic!("Token provider already set."));
    }

    pub async fn raw_get<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_inner(Method::GET, endpoint, None::<&()>, false).await
    }

    pub async fn raw_post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner(Method::POST, endpoint, Some(body), false).await
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_inner(Method::GET, endpoint, None::<&()>, true).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner(Method::POST, endpoint, Some(body), true).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner(Method::PUT, endpoint, Some(body), true).await
    }

    pub async fn patch<T, B>(&self, endpoint: &str, body: &B) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.send_inner(Method::PATCH, endpoint, Some(body), true).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_inner(Method::DELETE, endpoint, None::<&()>, true).await
    }

    async fn send_inner<T, B>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&B>,
        include_auth: bool,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let start = Instant::now();
        tracing::trace!(%method, endpoint, "Sending request");

        let mut req = self.client.request(method.clone(), &self.build_url(endpoint));
        if let Some(b) = body {
            req = req.json(b);
        }
        if include_auth {
            let token = self.token_provider
                .get()
                .expect("Token provider not set")
                ()
                .await;

            if let Some(t) = token {
                req = req.bearer_auth(t);
            }
        }

        let result = self.execute::<T>(req).await;

        let duration = start.elapsed();
        match &result {
            Ok(_) => tracing::trace!(%method, endpoint, ?duration, "Request succeeded"),
            Err(e) => tracing::warn!(%method, endpoint, ?duration, error = ?e, "Request failed"),
        }

        result
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

            let bytes = response
                .bytes()
                .await
                .unwrap_or_default();

            match serde_json::from_slice::<ApiError>(&bytes) {
                Ok(api_err) => Err(ClientError::Api(api_err)),
                Err(_) => {
                    let body = String::from_utf8_lossy(&bytes).into_owned();
                    Err(ClientError::UnexpectedStatus(status, body))
                }
            }
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }
}
