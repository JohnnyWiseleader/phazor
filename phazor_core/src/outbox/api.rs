/// Outbox façade (tiny wrapper)
/// Goal: give the app a simple API without touching OutboxService directly
use std::sync::Arc;
use crate::outbox::{service::OutboxService, store::OutboxStore, types::*,};
use crate::datasink::DataSink;

pub struct Outbox<S: OutboxStore, D: DataSink> {
    svc: Arc<OutboxService<S, D>>,
}

impl<S: OutboxStore, D: DataSink> Outbox<S, D> {
    pub fn new(store: Arc<S>, sink: Arc<D>) -> Self {
        Self { svc: Arc::new(OutboxService::new(store, sink)) }
    }

    pub async fn enqueue_create(&self, collection: &str, payload: serde_json::Value) -> anyhow::Result<()> {
        let msg = Message::new(MessageKind::Create { collection: collection.to_string() }, payload);
        self.svc.enqueue(msg).await
    }

    pub async fn drain_once(&self) -> anyhow::Result<()> {
        self.svc.drain_once().await
    }
}
