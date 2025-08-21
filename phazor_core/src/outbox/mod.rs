#[cfg(any(
    feature = "fake",
    feature = "rexie-sink",
    feature = "rest-http-wasm",
    feature = "rest-http-native"
))]
pub mod api;
#[cfg(any(
    feature = "fake",
    feature = "rexie-sink",
    feature = "rest-http-wasm",
    feature = "rest-http-native"
))]
pub use api::Outbox; // re-export for a clean path: phazor_core::outbox::Outbox
pub mod backoff;
pub mod service;
pub mod store;
pub mod store_mem;
pub mod types;
