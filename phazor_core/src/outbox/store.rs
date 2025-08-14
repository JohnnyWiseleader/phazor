use super::types:: Envelope;
use async_trait::async_trait;

#[async_trait]
pub trait OutboxStore: Send + Sync {
    async fn put(&self, env: Envelope) -> anyhow::Result<()>;
    async fn get(&self, id: uuid::Uuid) -> anyhow::Result<Option<Envelope>>;
    async fn update(&self, env: Envelope) -> anyhow::Result<()>;
    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;

    /// Pull a batch that is due for send (Pending or InFlight past due).
    async fn due(&self, limit: usize) -> anyhow::Result<Vec<Envelope>>;

    /// For metrics / UI.
    async fn counts(&self) -> anyhow::Result<(usize, usize, usize)>; // (pending, inflight, failed)
}
