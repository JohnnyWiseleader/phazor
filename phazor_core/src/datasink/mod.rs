#[cfg(any(test, feature = "fake"))]
pub mod fake;

use async_trait::async_trait;
use crate::outbox::types::Message;

#[async_trait]
pub trait DataSink: Send + Sync {
    /// Send a single message to the remote/local backend.
    async fn send(&self, msg: &Message) -> anyhow::Result<()>;
}
