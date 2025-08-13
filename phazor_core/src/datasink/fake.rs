use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::bail;
use async_trait::async_trait;

use crate::datasink::DataSink;
use crate::outbox::types::Message;

/// A sink that fails the first `fail_count` calls, then succeeds.
#[derive(Clone)]
pub struct FailThenOkSink {
    remaining: Arc<AtomicU32>,
}
impl FailThenOkSink {
    pub fn new(fail_count: u32) -> Self {
        Self {
            remaining: Arc::new(AtomicU32::new(fail_count)),
        }
    }
}

#[async_trait]
impl DataSink for FailThenOkSink {
    async fn send(&self, _msg: &Message) -> anyhow::Result<()> {
        let left = self
            .remaining
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                Some(x.saturating_sub(1))
            })
            .unwrap();

        if left > 0 {
            bail!("simulated network error ({} fails remaining)", left);
        }
        Ok(())
    }
}
