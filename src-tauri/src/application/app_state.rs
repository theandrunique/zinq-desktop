use std::sync::atomic::{AtomicBool, AtomicU64};

use serde::Serialize;

pub struct NetworkState {
    pub is_online: AtomicBool,
    pub last_ping_ms: AtomicU64,
}

pub struct AppState {
    pub network_state: NetworkState,
}

pub fn init_app_state() -> AppState {
    AppState {
        network_state: NetworkState {
            is_online: AtomicBool::new(false),
            last_ping_ms: AtomicU64::new(0),
        },
    }
}
