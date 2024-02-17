use axum::{
    routing::{get},
    http::StatusCode,
    Json, Router,
};
use serde::{Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("initializing server...");

    let app = Router::new()
        .route("/", get(root));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, Json<Score>)  {

    (StatusCode::OK, Json(Score {
        id: 1337,
        model: "me".to_string(),
        score: 12,
        size: 3.3,
        moe: false,
    }))
}

#[derive(Serialize)]
struct Score {
    id: u64,
    model: String,
    score: i64,
    size: f64,
    moe: bool,
}