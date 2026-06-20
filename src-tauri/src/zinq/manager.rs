use std::sync::Arc;

use crate::api_client::ApiClient;

pub struct ZinqManager {
    api_client: Arc<ApiClient>,
}

impl ZinqManager {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Self { api_client }
    }

    pub async fn init(&self) {}
}
