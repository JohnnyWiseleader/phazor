use crate::outbox::types::{Message, MessageKind};
use super::DataSink; 

// Shared type for constructing the URL
fn collection_path(kind: &MessageKind) -> (&str, &str) {
    match kind {
        MessageKind::Create { collection } => ("POST", collection.as_str()),
        // Add Update/Delete later:
        // MessageKind::Update { collection, id } => ("PUT", &format!("{}/{}", collection, id)),
        // MessageKind::Delete { collection, id } => ("DELETE", &format!("{}/{}", collection, id)),
        _ => ("POST", "unknown"),
    }
}

// ---- wasm32 implementation (browser): uses fetch via gloo_net ----
#[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm"))]
mod wasm_impl {
    use super::*;
    use anyhow::Context;
    use gloo_net::http::Request;
    use serde_json::Value;

    #[derive(Clone)]
    pub struct WasmHttpSink {
        pub base_url: String, // e.g. "http://localhost:3000"
    }

    impl WasmHttpSink {
        pub fn new(base_url: impl Into<String>) -> Self {
            Self { base_url: base_url.into() }
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    impl super::DataSink for WasmHttpSink {
        async fn send(&self, msg: &Message) -> anyhow::Result<()> {
            let (method, path) = super::collection_path(&msg.kind);
            let url = format!("{}/outbox/{}", self.base_url.trim_end_matches('/'), path);

            // decide what body to send - here we send the payload only
            let body: &Value = &msg.payload;

            let builder = match method {
                "POST" => Request::post(&url),
                "PUT" => Request::put(&url),
                "DELETE" => Request::delete(&url),
                m => anyhow::bail!("unsupported method {m}"),
            };

            let resp = builder
                .header("content-type", "application/json")
                .json(body)
                .context("serialize body")?
                .send()
                .await
                .with_context(|| format!("fetch {method} {url}"))?;

            if !resp.ok() {
                anyhow::bail!("server returned status {}", resp.status());
            }
            Ok(())
        }
    }
}

// ---- native implementation: Tower stack + reqwest ----
#[cfg(all(not(target_arch = "wasm32"), feature = "rest-http-native"))]
mod native_impl {
    use super::*;
    use anyhow::{Context, Result};
    use reqwest::Client;
    use serde_json::Value;
    use std::time::Duration;
    use tower::{Service, ServiceBuilder, ServiceExt};
    use tower::limit::ConcurrencyLimitLayer;
    use tower::retry::{Policy, RetryLayer};
    use tower::timeout::TimeoutLayer;

    #[derive(Clone)]
    pub struct NativeHttpSink {
        base_url: String,
        client: Client,
        timeout: Duration,
        concurrency: usize,
    }

    impl NativeHttpSink {
        pub fn new(base_url: impl Into<String>) -> Self {
            Self {
                base_url: base_url.into(),
                client: Client::new(),
                timeout: Duration::from_secs(5),
                concurrency: 16,
            }
        }
        pub fn with_timeout(mut self, d: Duration) -> Self { self.timeout = d; self }
        pub fn with_concurrency(mut self, n: usize) -> Self { self.concurrency = n; self }
    }

    // A cloneable “request” type for the retry policy
    #[derive(Clone)]
    struct Job {
        method: String,
        url: String,
        body: Value,
    }

    // Simple retry policy: retry up to N times on network errs or 5xx
    #[derive(Clone)]
    struct SimplePolicy { max: usize }
    impl<Req> Policy<Req, (), anyhow::Error> for SimplePolicy
    where
        Req: Clone,
    {
        type Future = std::future::Ready<()>;
        fn retry(&mut self, _req: &mut Req, result: &mut Result<(), anyhow::Error>) -> Option<Self::Future> {
            if result.is_err() && self.max > 0 {
                self.max -= 1;
                Some(std::future::ready(()))
            } else {
                None
            }
        }

        fn clone_request(&mut self, req: &Req) -> Option<Req> { 
            Some(req.clone()) 
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl super::DataSink for NativeHttpSink {
        async fn send(&self, msg: &Message) -> Result<()> {
            let (method, path) = super::collection_path(&msg.kind);
            let url = format!("{}/outbox/{}", self.base_url.trim_end_matches('/'), path);
            let body = msg.payload.clone();

            // Inner service: perform the reqwest call
            let client = self.client.clone();
            let inner = tower::service_fn(move |job: Job| {
                let client = client.clone();
                async move {
                    let builder = match job.method.as_str() {
                        "POST" => client.post(&job.url),
                        "PUT"  => client.put(&job.url),
                        "DELETE" => client.delete(&job.url),
                        m => anyhow::bail!("unsupported method {m}"),
                    };
                    let resp = builder.json(&job.body).send().await
                        .with_context(|| format!("reqwest {} {}", job.method, job.url))?;
                    if !resp.status().is_success() {
                        anyhow::bail!("status {} from {}", resp.status(), job.url);
                    }
                    Ok::<(), anyhow::Error>(())
                }
            });

            // Tower stack: timeout + retry + concurrency limit
            let policy = SimplePolicy { max: 2 }; // retry twice
            let mut svc = ServiceBuilder::new()
                .layer(ConcurrencyLimitLayer::new(self.concurrency))
                .layer(TimeoutLayer::new(self.timeout))
                .layer(RetryLayer::new(policy))
                .service(inner);

            let job = Job { method: method.to_string(), url, body };
            svc.ready().await
                .map_err(|e| anyhow::anyhow!("service not ready: {e}"))?;

            svc.call(job).await
                .map_err(|e| anyhow::anyhow!("service call error: {e}"))?;

            Ok(())
        }
    }
}

// Re-export a uniform name for callers (cfg picks the right one)
#[cfg(all(target_arch = "wasm32", feature = "rest-http-wasm"))]
pub use wasm_impl::WasmHttpSink as RestHttpSink;

#[cfg(all(not(target_arch = "wasm32"), feature = "rest-http-native"))]
pub use native_impl::NativeHttpSink as RestHttpSink;
