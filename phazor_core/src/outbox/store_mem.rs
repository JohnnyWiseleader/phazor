// phazor_core/src/outbox/store_mem.rs
use super::store::OutboxStore;
use super::types::{Envelope, DeliveryState};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use tokio::sync::RwLock; 

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

#[cfg(test)]
mod tests {
    use super::*; // MemStore + trait impls
    use crate::outbox::store::OutboxStore;
    use crate::outbox::types::*;
    use std::time::{Duration, SystemTime};

    fn env_with(
        state: DeliveryState,
        next_after: SystemTime,
    ) -> Envelope {
        let msg = Message::new(
            MessageKind::Create { collection: "todos".into() },
            serde_json::json!({"title": "Buy hay bales", "done": false}),
        );
        let mut e = Envelope::new(msg);
        e.state = state;
        e.next_attempt_after = next_after;
        e
    }

    #[tokio::test]
    async fn put_and_get_roundtrip() {
        let store = MemStore::default();
        let env = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);

        store.put(env.clone()).await.unwrap();
        let got = store.get(env.msg.id).await.unwrap().expect("exists");
        assert_eq!(got.msg.id, env.msg.id);
        assert!(matches!(got.state, DeliveryState::Pending));
    }

    #[tokio::test]
    async fn update_replaces_existing() {
        let store = MemStore::default();
        let mut env = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);
        store.put(env.clone()).await.unwrap();

        // mutate and update
        env.state = DeliveryState::InFlight;
        env.attempts = 3;
        env.last_error = Some("network down".into());
        store.update(env.clone()).await.unwrap();

        let got = store.get(env.msg.id).await.unwrap().unwrap();
        assert!(matches!(got.state, DeliveryState::InFlight));
        assert_eq!(got.attempts, 3);
        assert_eq!(got.last_error.as_deref(), Some("network down"));
    }

    #[tokio::test]
    async fn delete_removes() {
        let store = MemStore::default();
        let env = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);
        let id = env.msg.id;

        store.put(env).await.unwrap();
        store.delete(id).await.unwrap();

        let got = store.get(id).await.unwrap();
        assert!(got.is_none());
    }

    #[tokio::test]
    async fn due_filters_by_time_and_state() {
        let store = MemStore::default();

        // Due now
        let a = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);
        let c = env_with(DeliveryState::InFlight, SystemTime::UNIX_EPOCH);

        // Not due yet
        let future = SystemTime::now() + Duration::from_secs(60);
        let b = env_with(DeliveryState::Pending, future);
        let d = env_with(DeliveryState::InFlight, future);

        // Never due: terminal states
        let e = env_with(DeliveryState::Succeeded, SystemTime::UNIX_EPOCH);
        let f = env_with(DeliveryState::Failed, SystemTime::UNIX_EPOCH);

        for env in [a.clone(), b, c.clone(), d, e, f] {
            store.put(env).await.unwrap();
        }

        let due = store.due(10).await.unwrap();
        let ids: std::collections::HashSet<_> = due.into_iter().map(|e| e.msg.id).collect();

        // We only assert membership; MemStore (HashMap) has no stable order.
        assert!(ids.contains(&a.msg.id));
        assert!(ids.contains(&c.msg.id));
        assert_eq!(ids.len(), 2, "only the two 'due now' (Pending/Inflight) items");
    }

    #[tokio::test]
    async fn due_respects_limit() {
        let store = MemStore::default();
        for _ in 0..5 {
            store
                .put(env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH))
                .await
                .unwrap();
        }
        let due3 = store.due(3).await.unwrap();
        assert_eq!(due3.len(), 3);
    }

    #[tokio::test]
    async fn counts_matches_states() {
        let store = MemStore::default();

        // 2 pending
        store.put(env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH)).await.unwrap();
        store.put(env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH)).await.unwrap();

        // 1 inflight
        store.put(env_with(DeliveryState::InFlight, SystemTime::UNIX_EPOCH)).await.unwrap();

        // 1 failed
        store.put(env_with(DeliveryState::Failed, SystemTime::UNIX_EPOCH)).await.unwrap();

        // 1 succeeded (not counted in (pending, inflight, failed))
        store.put(env_with(DeliveryState::Succeeded, SystemTime::UNIX_EPOCH)).await.unwrap();

        let (p, i, f) = store.counts().await.unwrap();
        assert_eq!((p, i, f), (2, 1, 1));
    }
}

