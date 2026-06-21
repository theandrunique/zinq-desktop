use anyhow::Context;
use futures_util::FutureExt;
use tokio::sync::mpsc;

use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};

use crate::schemas::EventLog;

pub struct SocketClient {
    _client: Client,
}

impl SocketClient {
    pub async fn connect(
        base_url: &str,
        access_token: &str,
    ) -> anyhow::Result<(Self, mpsc::UnboundedReceiver<EventLog>)> {
        let (tx, rx) = mpsc::unbounded_channel();
        let url = format!("{}?access_token={}", base_url, access_token);

        let client = ClientBuilder::new(&url)
            .on("event", move |payload: Payload, _socket: Client| {
                let tx = tx.clone();
                async move {
                    if let Payload::Text(values) = payload {
                        for value in values {
                            if let Ok(event) = serde_json::from_value::<EventLog>(value) {
                                tx.send(event).ok();
                            }
                        }
                    }
                }
                .boxed()
            })
            .connect()
            .await
            .context("Socket connection failed")?;

        Ok((Self { _client: client }, rx))
    }
}
