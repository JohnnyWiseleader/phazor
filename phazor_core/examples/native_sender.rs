use phazor_core::outbox::Outbox;
use serde_json::json;

/// To test run:
/// cargo run -p phazor_core --example native_sender -F "rest-http-native"

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Uses the *native* sink on non-wasm targets
    let outbox = Outbox::dev_mem_http("http://127.0.0.1:3000");

    let id = outbox
        .enqueue_create("todos", json!({ "title": "from native", "done": false }))
        .await?;
    println!("enqueued id = {id}");

    outbox.drain_once().await?;
    println!("drained once");
    Ok(())
}
