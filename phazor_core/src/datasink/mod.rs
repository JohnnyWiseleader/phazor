#[cfg(any(test, feature = "fake"))]
pub mod fake;

// only available on wasm targets when feature set
#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
pub mod rest_rexie;

use async_trait::async_trait;
use crate::outbox::types::Message;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DataSink: Send + Sync {
    async fn send(&self, msg: &Message) -> anyhow::Result<()>;
}
