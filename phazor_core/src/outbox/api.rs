/// Outbox façade (tiny wrapper)
/// Goal: give the app a simple API without touching OutboxService directly
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use super::{
    service::OutboxService,
    store_mem::MemStore,
    types::{Message, MessageKind},
};

#[cfg(feature = "fake")]
use crate::datasink::fake::FailThenOkSink;

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
use crate::datasink::rest_rexie::RexieSink;

// Define a concrete, non-generic Outbox per feature:
#[cfg(feature = "fake")]
#[derive(Clone)]
pub struct Outbox {
    svc: Arc<OutboxService<MemStore, FailThenOkSink>>,
}

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
#[derive(Clone)]
pub struct Outbox {
    svc: Arc<OutboxService<MemStore, RexieSink>>,
}

// Builders gated by features:
#[cfg(feature = "fake")]
impl Outbox {
    pub fn dev_mem_fake(fails: u32) -> Self {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(fails));
        Self {
            svc: Arc::new(OutboxService::new(store, sink)),
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
impl Outbox {
    pub fn dev_mem_rexie(db: &str) -> Self {
        let store = Arc::new(MemStore::default());
        let sink = Arc::new(RexieSink::new(db));
        Self {
            svc: Arc::new(OutboxService::new(store, sink)),
        }
    }
}

// Feature-agnostic API that forwards to the inner service:
impl Outbox {
    pub async fn enqueue_create(&self, collection: &str, payload: Value) -> anyhow::Result<Uuid> {
        let msg = Message::new(
            MessageKind::Create {
                collection: collection.into(),
            },
            payload,
        );
        
        let id = msg.id;
        self.svc.enqueue(msg).await?; // service returns Result<(), _>
        Ok(id) // façade returns the id
    }

    pub async fn drain_once(&self) -> anyhow::Result<()> {
        self.svc.drain_once().await
    }

    pub fn sink_name() -> &'static str {
        #[cfg(feature = "fake")] { "fake" }
        #[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))] { "rexie" }
        #[cfg(not(any(feature = "fake", all(target_arch="wasm32", feature="rexie-sink"))))] { "unknown" }
    }
}
