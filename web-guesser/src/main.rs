use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};
use serde::{
    Deserialize, 
    Serialize
};
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref COUNTER: Mutex<isize> = Mutex::new(0);
    static ref RAND: Mutex<isize> = Mutex::new(rand::thread_rng().gen_range(0..100));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/guess", post(guess));

    // run our app with hyper, listening globally on port 3333
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn guess(
    Json(payload): Json<Guess>,
) -> (StatusCode, Json<GuessResponse>) {

    *COUNTER.lock().unwrap() += 1;
    let cc = *COUNTER.lock().unwrap();
    let message = process(payload.guess);

    let user = GuessResponse {
        attempts: cc,
        message: message.to_string(),
    };

    (StatusCode::OK, Json(user))
}

fn process(guess: isize) -> &'static str {

    let mut rand = *RAND.lock().unwrap();
    println!("SECRET rand was: {}", rand);

    return if guess < rand {
        "Your guess was too low"
    } else if guess > rand {
        "Your guess was too high"
    } else {
        *COUNTER.lock().unwrap() = 0;
        *RAND.lock().unwrap() = rand::thread_rng().gen_range(0..100);
        "Nailed it!"
    }
}

#[derive(Deserialize)]
struct Guess {
    guess: isize,
}

#[derive(Serialize)]
struct GuessResponse {
    attempts: isize,
    message: String,
}