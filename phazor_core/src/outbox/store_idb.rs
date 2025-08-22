#![cfg(target_arch = "wasm32")]
use async_trait::async_trait;
use rexie::{ObjectStore, Rexie, TransactionMode};
use serde_wasm_bindgen as swb;

use super::store::OutboxStore;
use super::types::{DeliveryState, Envelope};
use crate::util::now::system_time_now;

#[derive(Clone)]
pub struct IdbStore {
    db_name: String,
    store_name: String,
}

impl IdbStore {
    pub fn new(db_name: impl Into<String>) -> Self {
        Self {
            db_name: db_name.into(),
            store_name: "outbox".to_string(),
        }
    }

    async fn open(&self) -> anyhow::Result<Rexie> {
        // No key_path: we’ll pass the key explicitly (the Envelope’s msg.id)
        let db = Rexie::builder(&self.db_name)
            .version(1)
            .add_object_store(
                ObjectStore::new(&self.store_name)
                    .key_path("msg.id")
                    .auto_increment(false))
            .build()
            .await
            .map_err(|e| anyhow::anyhow!("open IndexedDB: {e:?}"))?;
        Ok(db)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OutboxStore for IdbStore {
    async fn put(&self, env: Envelope) -> anyhow::Result<()> {
        let db = self.open().await?;
        let tx = db.transaction(&[&self.store_name], TransactionMode::ReadWrite)
            .map_err(|e| anyhow::anyhow!("tx RW: {e:?}"))?;
        let store = tx.store(&self.store_name)
            .map_err(|e| anyhow::anyhow!("store: {e:?}"))?;

        // rely on key_path, so no explicit key
        let js = swb::to_value(&env).map_err(|e| anyhow::anyhow!("env -> JsValue: {e:?}"))?;
        store.put(&js, None).await.map_err(|e| anyhow::anyhow!("idb put: {e:?}"))?;

        tx.done().await.map_err(|e| anyhow::anyhow!("tx commit: {e:?}"))?;
        Ok(())
    }

    async fn get(&self, id: uuid::Uuid) -> anyhow::Result<Option<Envelope>> {
        let db = self.open().await?;
        let tx = db.transaction(&[&self.store_name], TransactionMode::ReadOnly)
            .map_err(|e| anyhow::anyhow!("tx RO: {e:?}"))?;
        let store = tx.store(&self.store_name)
            .map_err(|e| anyhow::anyhow!("store: {e:?}"))?;

        // build the key via serde_wasm_bindgen; no explicit JsValue type needed
        let key = swb::to_value(&id).map_err(|e| anyhow::anyhow!("uuid -> key: {e:?}"))?;
        let val = store.get(key).await.map_err(|e| anyhow::anyhow!("idb get: {e:?}"))?;
        tx.done().await.map_err(|e| anyhow::anyhow!("tx close: {e:?}"))?;

        if let Some(js) = val {
            let env: Envelope = swb::from_value(js).map_err(|e| anyhow::anyhow!("JsValue -> env: {e:?}"))?;
            Ok(Some(env))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, env: Envelope) -> anyhow::Result<()> {
        // same as put (id is the key)
        self.put(env).await
    }

    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        let db = self.open().await?;
        let tx = db.transaction(&[&self.store_name], TransactionMode::ReadWrite)
            .map_err(|e| anyhow::anyhow!("tx RW: {e:?}"))?;
        let store = tx.store(&self.store_name)
            .map_err(|e| anyhow::anyhow!("store: {e:?}"))?;
        let key = swb::to_value(&id).map_err(|e| anyhow::anyhow!("uuid -> key: {e:?}"))?;

        store
            .delete(key)
            .await
            .map_err(|e| anyhow::anyhow!("idb delete: {e:?}"))?;

        tx.done()
            .await
            .map_err(|e| anyhow::anyhow!("tx commit: {e:?}"))?;
        Ok(())
    }

    async fn due(&self, limit: usize) -> anyhow::Result<Vec<Envelope>> {
        let now = system_time_now();
        let db = self.open().await?;
        let tx = db.transaction(&[&self.store_name], TransactionMode::ReadOnly)
            .map_err(|e| anyhow::anyhow!("tx RO: {e:?}"))?;
        let store = tx.store(&self.store_name)
            .map_err(|e| anyhow::anyhow!("store: {e:?}"))?;

        // Simple first pass: load all, filter in Rust.
        // (If you want this to scale, add an index later — see note below.)
        let all = store
            .get_all(None, None)
            .await
            .map_err(|e| anyhow::anyhow!("idb get_all: {e:?}"))?;

        tx.done()
            .await
            .map_err(|e| anyhow::anyhow!("tx close: {e:?}"))?;

        let mut out = Vec::new();
        for js in all {
            let env: Envelope = match swb::from_value(js) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("skip malformed row: {e:?}");
                    continue;
                }
            };
            if matches!(env.state, DeliveryState::Pending | DeliveryState::InFlight)
                && env.next_attempt_after <= now
            {
                out.push(env);
                if out.len() >= limit {
                    break;
                }
            }
        }
        Ok(out)
    }

    async fn counts(&self) -> anyhow::Result<(usize, usize, usize)> {
        let db = self.open().await?;
        let tx = db.transaction(&[&self.store_name], TransactionMode::ReadOnly)
            .map_err(|e| anyhow::anyhow!("tx RO: {e:?}"))?;
        let store = tx.store(&self.store_name)
            .map_err(|e| anyhow::anyhow!("store: {e:?}"))?;

        let all = store
            .get_all(None, None)
            .await
            .map_err(|e| anyhow::anyhow!("idb get_all: {e:?}"))?;

        tx.done()
            .await
            .map_err(|e| anyhow::anyhow!("tx close: {e:?}"))?;

        let mut p = 0;
        let mut i = 0;
        let mut f = 0;
        for js in all {
            if let Ok(env) = swb::from_value::<Envelope>(js) {
                match env.state {
                    DeliveryState::Pending => p += 1,
                    DeliveryState::InFlight => i += 1,
                    DeliveryState::Failed => f += 1,
                    DeliveryState::Succeeded => {}
                }
            }
        }
        Ok((p, i, f))
    }
}

/*

#[cfg(test)]
mod tests {
    use super::*; // IdbStore + trait impls
    use crate::outbox::store::OutboxStore;
    use crate::outbox::types::*;
    use std::time::{Duration, SystemTime};

    fn env_with(state: DeliveryState, next_after: SystemTime) -> Envelope {
        let msg = Message::new(
            MessageKind::Create {
                collection: "todos".into(),
            },
            serde_json::json!({"title": "Buy hay bales", "done": false}),
        );
        let mut e = Envelope::new(msg);
        e.state = state;
        e.next_attempt_after = next_after;
        e
    }

    #[tokio::test]
    async fn put_and_get_roundtrip() {
        let store = IdbStore::default();
        let env = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);

        store.put(env.clone()).await.unwrap();
        let got = store.get(env.msg.id).await.unwrap().expect("exists");
        assert_eq!(got.msg.id, env.msg.id);
        assert!(matches!(got.state, DeliveryState::Pending));
    }

    #[tokio::test]
    async fn update_replaces_existing() {
        let store = IdbStore::default();
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
        let store = IdbStore::default();
        let env = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);
        let id = env.msg.id;

        store.put(env).await.unwrap();
        store.delete(id).await.unwrap();

        let got = store.get(id).await.unwrap();
        assert!(got.is_none());
    }

    #[tokio::test]
    async fn due_filters_by_time_and_state() {
        let store = IdbStore::default();

        // Due now
        let a = env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH);
        let c = env_with(DeliveryState::InFlight, SystemTime::UNIX_EPOCH);

        // Not due yet
        let future = system_time_now() + Duration::from_secs(60);
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

        // We only assert membership; IdbStore (HashMap) has no stable order.
        assert!(ids.contains(&a.msg.id));
        assert!(ids.contains(&c.msg.id));
        assert_eq!(
            ids.len(),
            2,
            "only the two 'due now' (Pending/Inflight) items"
        );
    }

    #[tokio::test]
    async fn due_respects_limit() {
        let store = IdbStore::default();
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
        let store = IdbStore::default();

        // 2 pending
        store
            .put(env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH))
            .await
            .unwrap();
        store
            .put(env_with(DeliveryState::Pending, SystemTime::UNIX_EPOCH))
            .await
            .unwrap();

        // 1 inflight
        store
            .put(env_with(DeliveryState::InFlight, SystemTime::UNIX_EPOCH))
            .await
            .unwrap();

        // 1 failed
        store
            .put(env_with(DeliveryState::Failed, SystemTime::UNIX_EPOCH))
            .await
            .unwrap();

        // 1 succeeded (not counted in (pending, inflight, failed))
        store
            .put(env_with(DeliveryState::Succeeded, SystemTime::UNIX_EPOCH))
            .await
            .unwrap();

        let (p, i, f) = store.counts().await.unwrap();
        assert_eq!((p, i, f), (2, 1, 1));
    }
}
 */