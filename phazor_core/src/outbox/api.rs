/// Outbox façade (tiny wrapper)
/// Goal: give the app a simple API without touching OutboxService directly
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use super::{
    service::OutboxService,
    types::{Message, MessageKind},
};

#[cfg(feature = "fake")]
use crate::datasink::fake::FailThenOkSink;

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
use crate::datasink::rest_rexie::RexieSink;

#[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm"))]
use crate::datasink::rest_http::RestHttpSink;

#[cfg(all(not(target_arch = "wasm32"), feature = "rest-http-native"))]
use crate::datasink::rest_http::RestHttpSink;

#[cfg(all(target_arch = "wasm32", feature = "idb-store", feature = "rest-http-wasm"))]
use crate::outbox::store_idb::IdbStore;

// Define a concrete, non-generic Outbox per feature:
#[cfg(feature = "fake")]
#[derive(Clone)]
pub struct Outbox { 

    svc: Arc<OutboxService<super::store_mem::MemStore, FailThenOkSink>>,
}

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
#[derive(Clone)]
pub struct Outbox {
    svc: Arc<OutboxService<super::store_mem::MemStore, RexieSink>>,
}

#[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm", not(feature = "idb-store")))]
#[derive(Clone)]
pub struct Outbox {
    svc: Arc<OutboxService<super::store_mem::MemStore, RestHttpSink>>,
}
#[cfg(all(target_arch = "wasm32", feature = "idb-store", feature = "rest-http-wasm"))]
#[derive(Clone)]
pub struct Outbox {
    svc: Arc<OutboxService<IdbStore, RestHttpSink>>,
}

// Builders gated by features:
#[cfg(feature = "fake")]
impl Outbox {
    pub fn dev_mem_fake(fails: u32) -> Self {
        let store = Arc::new(super::store_mem::MemStore::default());
        let sink = Arc::new(FailThenOkSink::new(fails));
        Self {
            svc: Arc::new(OutboxService::new(store, sink)),
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
impl Outbox {
    pub fn dev_mem_rexie(db: &str) -> Self {
        let store = Arc::new(super::store_mem::MemStore::default());
        let sink = Arc::new(RexieSink::new(db));
        Self {
            svc: Arc::new(OutboxService::new(store, sink)),
        }
    }
}

impl Outbox {
    #[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm", not(feature = "idb-store")))]
    pub fn dev_mem_http(base_url: &str) -> Self {
        let store = Arc::new(super::store_mem::MemStore::default());
        let sink = Arc::new(RestHttpSink::new(base_url));
        Self {
            svc: Arc::new(OutboxService::new(store, sink)),
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "rest-http-native"))]
    pub fn dev_mem_http(base: &str) -> Self {
        let store = std::sync::Arc::new(super::store_mem::MemStore::default());
        let sink = std::sync::Arc::new(RestHttpSink::new(base));
        Self {
            svc: std::sync::Arc::new(super::service::OutboxService::new(store, sink)),
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "idb-store", feature = "rest-http-wasm"))]
impl Outbox {
    pub fn dev_idb_http(base_url: &str, db_name: &str) -> Self {
        let store = std::sync::Arc::new(super::store_idb::IdbStore::new(db_name));
        let sink  = std::sync::Arc::new(crate::datasink::rest_http::RestHttpSink::new(base_url));
        Self {
            svc: std::sync::Arc::new(super::service::OutboxService::new(store, sink)),
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
        if cfg!(all(target_arch = "wasm32", feature = "fake")) {
            "fake"
        } else if cfg!(all(target_arch = "wasm32", feature = "rest-http-wasm", not(feature = "idb-store"))) {
            "rest http wasm"
        } else if cfg!(all(not(target_arch = "wasm32"), feature = "rest-http-native")) {
            "rest http native"
        } else if cfg!(all(target_arch = "wasm32", feature = "idb-store", feature = "rest-http-wasm")) {
            "idb store with rest http wasm"
        } else {
            "unknown"
        }
    }
}
