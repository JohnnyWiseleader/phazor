#[cfg(any(test, feature = "fake"))]
pub mod fake;

#[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm"))]
pub mod rest_http;

#[cfg(all(not(target_arch = "wasm32"), feature = "rest-http-native"))]
pub mod rest_http;

use async_trait::async_trait;
use crate::outbox::types::Message;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DataSink: Send + Sync {
    async fn send(&self, msg: &Message) -> anyhow::Result<()>;
}
