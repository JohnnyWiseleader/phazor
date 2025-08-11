// phazor_core/src/outbox/store_mem.rs
use super::store::OutboxStore;
use super::types::{Envelope, DeliveryState};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use std::sync::RwLock;

#[derive(Default, Clone)]
pub struct MemStore(Arc<RwLock<HashMap<uuid::Uuid, Envelope>>>);

#[async_trait]
impl OutboxStore for MemStore {
    async fn put(&self, env: Envelope) -> anyhow::Result<()> {
        self.0.write().await.insert(env.msg.id, env);
        Ok(())
    }
    async fn get(&self, id: uuid::Uuid) -> anyhow::Result<Option<Envelope>> {
        Ok(self.0.read().await.get(&id).cloned())
    }
    async fn update(&self, env: Envelope) -> anyhow::Result<()> {
        self.0.write().await.insert(env.msg.id, env);
        Ok(())
    }
    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        self.0.write().await.remove(&id);
        Ok(())
    }
    async fn due(&self, limit: usize) -> anyhow::Result<Vec<Envelope>> {
        let now = SystemTime::now();
        let list = self.0.read().await
            .values()
            .filter(|e| matches!(e.state, DeliveryState::Pending | DeliveryState::InFlight)
                && e.next_attempt_after <= now)
            .take(limit)
            .cloned()
            .collect();
        Ok(list)
    }
    async fn counts(&self) -> anyhow::Result<(usize, usize, usize)> {
        let g = self.0.read().await;
        let mut p=0; let mut i=0; let mut f=0;
        for e in g.values() {
            match e.state {
                DeliveryState::Pending => p+=1,
                DeliveryState::InFlight => i+=1,
                DeliveryState::Failed => f+=1,
                DeliveryState::Succeeded => {},
            }
        }
        Ok((p,i,f))
    }
}
