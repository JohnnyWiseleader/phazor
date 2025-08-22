use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

#[derive(Clone, Default)]
struct AppState {
    inbox: Arc<Mutex<Vec<(String, Value)>>>,
}

async fn create(
    Path(collection): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> &'static str {
    {
        let mut inbox = state.inbox.lock().unwrap();
        inbox.push((collection.clone(), body.clone()));
    }
    println!("→ received: collection={collection}, payload={body}");
    "ok"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::default();
    let app = Router::new()
        .route("/outbox/:collection", post(create))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

