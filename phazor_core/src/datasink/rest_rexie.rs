#![cfg(all(target_arch = "wasm32", feature = "rexie-sink"))]
use async_trait::async_trait;
use serde::Serialize;
use crate::outbox::types::Message;
use crate::datasink::DataSink;
// use anyhow::anyhow;

#[derive(Clone)]
pub struct RexieSink {
    db_name: String,
    store_name: String,
    // keep it simple: don't cache the DB inside the struct (avoids Send/Sync issues)
}

impl RexieSink {
    pub fn new(db_name: impl Into<String>) -> Self {
        Self {
            db_name: db_name.into(),
            store_name: "rest_dev".to_string(),
        }
    }
}

// #[async_trait]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DataSink for RexieSink {
    async fn send(&self, msg: &Message) -> anyhow::Result<()> {
        // Open / upgrade the DB every call (fine for dev; easy to reason about).
        let db = rexie::Rexie::builder(&self.db_name)
            .version(1)
            .add_object_store(
                rexie::ObjectStore::new(&self.store_name)
                    .key_path("id")
                    .auto_increment(false)
            )
            .build()
            .await.map_err(|e|  anyhow::anyhow!("open IndexedDB: {:?}", e))?;

        // Build a “what we would POST” record
        #[derive(Serialize)]
        struct OutboundRecord<'a> {
            id: String,
            path: String,                  // pretend endpoint
            method: &'static str,
            body: &'a Message,             // full message
            attempted_at_ms: i64,
        }

        let record = OutboundRecord {
            id: msg.id.to_string(),
            path: "/sync/outbox".to_string(),
            method: "POST",
            body: msg,
            attempted_at_ms: js_sys::Date::now() as i64,
        };

        // Convert to JsValue or map the error
        let js = serde_wasm_bindgen::to_value(&record)
            .map_err(|e|  anyhow::anyhow!("to JsValue: {:?}", e))?;

        // Write to the object store (key = message id)
        let tx = db
            .transaction(&[&self.store_name], rexie::TransactionMode::ReadWrite)
            .map_err(|e|  anyhow::anyhow!("begin tx: {:?}", e))?;

        let store = tx.store(&self.store_name)
            .map_err(|e|  anyhow::anyhow!("open store: {:?}", e))?;

        store.put(&js, None).await
            .map_err(|e|  anyhow::anyhow!("put: {e:?}"))?;

        tx.done().await.map_err(|e|  anyhow::anyhow!("commit tx: {:?}", e))?;

        // In real sink, Ok means “server accepted it”. For dev, Ok = “logged”.
        Ok(())
    }
}
