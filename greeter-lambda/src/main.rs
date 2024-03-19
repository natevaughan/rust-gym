use anyhow::Result;
use axum::{routing::get, Json, Router};
use lambda_http::{
    http::status::StatusCode, service_fn, tower::ServiceExt, Error, IntoResponse, Request, Service,
};
use serde::Deserialize;

/**
 * Uses axum for routing but lambda_http for accepting web requests
 */
#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Firing up the lambda!!");
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn get_root() {}

async fn post_root(Json(payload): Json<MyRequest>) -> (StatusCode, String) {
    println!("request body: {:?}", payload);
    (StatusCode::OK, format!("Nice to meet you {}", payload.name))
}

async fn delete_root() {}

async fn handler(request: Request) -> Result<impl IntoResponse> {
    let mut router =
        Router::new().route("/axum", get(get_root).post(post_root).delete(delete_root));
    let response = router.as_service().ready().await?.call(request).await?;

    Ok(response)
}

#[derive(Deserialize, Debug)]
struct MyRequest {
    name: String,
}
