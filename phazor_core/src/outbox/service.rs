use std::sync::Arc;
use std::time::SystemTime;

use crate::datasink::DataSink;
use crate::outbox::backoff::next_backoff;
use crate::outbox::store::OutboxStore;
use crate::outbox::types::{DeliveryState, Envelope, Message};

pub struct OutboxService<S: OutboxStore, D: DataSink> {
    store: Arc<S>,
    sink: Arc<D>,
    batch_size: usize,
}

impl<S: OutboxStore, D: DataSink> OutboxService<S, D> {
    pub fn new(store: Arc<S>, sink: Arc<D>) -> Self {
        Self { store, sink, batch_size: 16 }
    }

    pub fn with_batch_size(mut self, n: usize) -> Self {
        self.batch_size = n.max(1);
        self
    }

    pub async fn enqueue(&self, msg: Message) -> anyhow::Result<()> {
        let env = Envelope::new(msg);
        self.store.put(env).await
    }

    /// Process up to `batch_size` due messages once.
    /// Policy:
    /// - Success: mark Succeeded then delete the envelope (keeps store tidy).
    /// - Error: back to Pending, attempts += 1, schedule next_attempt_after with backoff, store last_error.
    pub async fn drain_once(&self) -> anyhow::Result<()> {
        let batch = self.store.due(self.batch_size).await?;
        for mut env in batch {
            // Mark inflight
            env.state = DeliveryState::InFlight;
            self.store.update(env.clone()).await?;

            match self.sink.send(&env.msg).await {
                Ok(()) => {
                    // Success → mark & delete
                    env.state = DeliveryState::Succeeded;
                    self.store.update(env.clone()).await?;
                    self.store.delete(env.msg.id).await?;
                }
                Err(e) => {
                    // Failure → schedule retry
                    env.state = DeliveryState::Pending;
                    env.attempts += 1;
                    env.last_error = Some(e.to_string());
                    env.next_attempt_after = SystemTime::now() + next_backoff(env.attempts);
                    self.store.update(env).await?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outbox::store::OutboxStore;
    use crate::outbox::types::*;
    use crate::outbox::store_mem::MemStore;
    use crate::datasink::fake::FailThenOkSink;

    use std::time::{Duration, SystemTime};
    use uuid::Uuid;

    fn new_msg() -> Message {
        Message::new(
            MessageKind::Create { collection: "todos".into() },
            serde_json::json!({"title": "Buy hay bales", "done": false}),
        )
    }

    #[tokio::test]
    async fn drain_once_retries_and_then_succeeds() {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(2)); // fail twice, then ok
        let svc = OutboxService::new(store.clone(), sink);

        // Enqueue one message; Envelope::new sets next_attempt_after = UNIX_EPOCH (due now)
        let msg = new_msg();
        let id = msg.id;
        svc.enqueue(msg).await.unwrap();

        // 1st drain: expect failure → attempts=1, backoff scheduled
        svc.drain_once().await.unwrap();
        let mut env = store.get(id).await.unwrap().expect("still present after failure");
        assert_eq!(env.attempts, 1);
        assert!(matches!(env.state, DeliveryState::Pending));
        assert!(env.next_attempt_after > SystemTime::now() - Duration::from_secs(1));

        // Force it due again without waiting
        env.next_attempt_after = SystemTime::UNIX_EPOCH;
        store.update(env.clone()).await.unwrap();

        // 2nd drain: expect failure → attempts=2
        svc.drain_once().await.unwrap();
        env = store.get(id).await.unwrap().expect("present after second failure");
        assert_eq!(env.attempts, 2);
        assert!(matches!(env.state, DeliveryState::Pending));

        // Force due again
        env.next_attempt_after = SystemTime::UNIX_EPOCH;
        store.update(env.clone()).await.unwrap();

        // 3rd drain: should succeed → envelope deleted
        svc.drain_once().await.unwrap();
        let got = store.get(id).await.unwrap();
        assert!(got.is_none(), "deleted on success");
    }

    #[tokio::test]
    async fn drain_once_marks_inflight_then_updates() {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(1)); // fail once
        let svc = OutboxService::new(store.clone(), sink);

        let msg = new_msg();
        let id = msg.id;
        svc.enqueue(msg).await.unwrap();

        // We can’t observe "inflight" externally without a hook, but we can at least
        // call drain_once and then verify it went back to Pending with attempts=1.
        svc.drain_once().await.unwrap();
        let env = store.get(id).await.unwrap().unwrap();
        assert!(matches!(env.state, DeliveryState::Pending));
        assert_eq!(env.attempts, 1);
    }

    #[tokio::test]
    async fn drain_once_is_noop_when_nothing_due() {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(0));
        let svc = OutboxService::new(store.clone(), sink);

        // Enqueue but schedule it in the future
        let msg = Message::new(
            MessageKind::Custom { topic: "future".into() },
            serde_json::json!({}),
        );
        let id = msg.id;
        svc.enqueue(msg).await.unwrap();

        // Make it "not due yet"
        let mut env = store.get(id).await.unwrap().unwrap();
        env.next_attempt_after = SystemTime::now() + Duration::from_secs(120);
        store.update(env).await.unwrap();

        // Drain → should leave it untouched
        svc.drain_once().await.unwrap();
        let env = store.get(id).await.unwrap().unwrap();
        assert_eq!(env.attempts, 0);
        assert!(matches!(env.state, DeliveryState::Pending));
    }
}
