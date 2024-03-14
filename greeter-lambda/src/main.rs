// use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde_json;
// use std::collections::HashMap;
use lambda_http::{service_fn, Error, RequestExt, IntoResponse, Request, Body, http::status::StatusCode};
use anyhow::{Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Firing up the old lambda!!!!");
    lambda_http::run(service_fn(hello)).await?;
    Ok(())
}

// anyhow: replace .expect("") with .context("")?

async fn hello(
    request: Request
) -> Result<impl IntoResponse> {
    let _context = request.lambda_context_ref();
    println!("request info: {:?}", request);
    println!("URI: {:?}", request.uri());
    println!("request body: {:?}", request.body());
    let body = request.body();
    let res = match body {
        Body::Text(some_text) => {
            let my_req: MyRequest = serde_json::from_str(some_text).expect("Error nate is dumb and doesn't know how to make better errors");
            (StatusCode::OK, format!(
                "hello {}",
                my_req.name
            ))
        },
        Body::Binary(_) => (StatusCode::NOT_FOUND, String::from("Not found")),
        Body::Empty => (StatusCode::BAD_REQUEST, String::from("Fuck you")),
    };
    Ok(res)
}

#[derive(Deserialize, Debug)]
struct MyRequest {
    name: String,
}

// #[derive(Serialize, Debug)]
// struct Response {
//     uuid: String,
//     greeting: String,
//     author_id: u64
// }

// pub(crate) async fn handle_post(event: LambdaEvent<String>) -> Result<Response, Error> {
//     println!("{:?}", event.payload);
//     let resp = Response {
//         uuid: "213".to_string(),
//         greeting: format!("Hello, {:?}", event),
//         author_id: 42,
//     };
//     Ok(resp)
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     println!("Greeting requested........");
//     tracing_subscriber::fmt()
//         .init();
//     let func = service_fn(handle_post);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }