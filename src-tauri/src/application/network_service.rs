use std::{sync::atomic::Ordering, time::Duration};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{Instant, sleep};

use crate::{api_client::ApiClient, application::app_state::{AppState, NetworkState}};

#[derive(Deserialize)]
pub struct PingResponse {
    pub status: String,
}

pub struct NetworkService {
    app_handle: AppHandle,
    http_client: reqwest::Client,
    ping_url: String,
}

impl NetworkService {
    pub fn new(app_handle: AppHandle, ping_url: &str) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        Self { app_handle, http_client, ping_url: ping_url.to_string() }
    }

    pub fn start(&self) {
        let app_handle = self.app_handle.clone();
        let http_client = self.http_client.clone();
        let ping_url = self.ping_url.clone();

        tauri::async_runtime::spawn(async move {
            tracing::info!("Starting network service");
            let app_state = app_handle.state::<AppState>().clone();
            let app_handle_clone = app_handle.clone();

            let emit = |is_online: bool, last_ping_ms: u64| {
                let _ = app_handle_clone
                    .emit("network:status-changed", NetworkStatus {
                        is_online,
                        last_ping_ms,
                    });
            };

            loop {
                let start = Instant::now();
                let result = http_client.get(&ping_url).send().await;

                match result {
                    Ok(response) if response.status().is_success() => {
                        let ping_ms = start.elapsed().as_millis() as u64;
                        tracing::info!(ping_ms, "Ping successful");
                        app_state.network_state.is_online.store(true, Ordering::Relaxed);
                        app_state.network_state.last_ping_ms.store(ping_ms, Ordering::Relaxed);
                        emit(true, ping_ms);
                    }
                    Ok(response) => {
                        tracing::warn!(error = %&response.status(), "Ping returned error status");
                        app_state.network_state.is_online.store(false, Ordering::Relaxed);
                        app_state.network_state.last_ping_ms.store(0, Ordering::Relaxed);
                        emit(false, 0);
                    }
                    Err(e) => {
                        tracing::warn!("Error during server ping");
                        app_state.network_state.is_online.store(false, Ordering::Relaxed);
                        app_state.network_state.last_ping_ms.store(0, Ordering::Relaxed);
                        emit(false, 0);
                    }
                }

                sleep(Duration::from_secs(10)).await;
            }
        });
    }

}

#[derive(Clone, Serialize)]
pub struct NetworkStatus {
    pub is_online: bool,
    pub last_ping_ms: u64,
}
