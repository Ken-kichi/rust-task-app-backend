use axum::{Router, response::Json, routing::get};
use serde_json::{Value, json};

async fn json() -> Json<Value> {
    let data = json!({
        "message": "Hello, JSON!"
    });
    Json(data)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
