#[cfg(any(feature = "fake", all(target_arch = "wasm32", feature = "rexie-sink")))]
pub mod api;
#[cfg(any(feature = "fake", all(target_arch = "wasm32", feature = "rexie-sink")))]
pub use api::Outbox; // re-export for a clean path: phazor_core::outbox::Outbox
pub mod backoff;
pub mod service;
pub mod store_mem;
pub mod store;
pub mod types;

