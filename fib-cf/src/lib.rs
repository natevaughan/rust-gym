use worker::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct FibResponse {
    nth: u64,
    value: u64
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {

    let router = Router::new();
    router
        .get_async("/", |req, _ctx| async move {
            let parsed_url = req.url().unwrap();
            let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
            let opt_n = hash_query.get("n");
            return match opt_n {
                Some(n) => Response::from_json(&FibResponse{nth: n.parse::<u64>().expect("Failed to parse n"), value: fib(n.parse::<u64>().expect("Failed to parse n"), 2, 1, 1)}),
                None => Response::error("Bad Request", 400)
            };
            
        })
        .run(req, env).await
}

fn fib(nth: u64, step: u64, last: u64, previous: u64) -> u64 {
    if step >= nth {
        return last
    } else {
        return fib(nth, step + 1, last + previous, last)
    }
}