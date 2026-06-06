use std::sync::Arc;

use tokio::sync::RwLock;

use crate::auth::types::{TokenPair, User};

pub struct AuthState {
    user: Arc<RwLock<Option<User>>>,
    tokens: Arc<RwLock<Option<TokenPair>>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            user: Arc::new(RwLock::new(None)),
            tokens: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_tokens(&self, tokens: TokenPair) {
        *self.tokens.write().await = Some(tokens);
    }

    pub async fn get_tokens(&self) -> Option<TokenPair> {
        self.tokens.read().await.clone()
    }
}
