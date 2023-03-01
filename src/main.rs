use axum::{response::IntoResponse, routing::get, Router, Server};
use tower::limit::{ConcurrencyLimitLayer, ConcurrencyLimit};

#[tokio::main]
async fn main() {
    //let app = Router::new().route("/", get(index));

    // All requests to `first_handler` and `second_handler` will be sent through
    // `ConcurrencyLimit`
    let app = Router::new().route("/", get(first_handler))
        .route("/foo", get(second_handler))
        .layer(ConcurrencyLimitLayer::new(64))
        // Request to `GET /bar` will go directly to `third_handler` and
        // wont be sent through `ConcurrencyLimit`
        .route("/bar", get(third_handler));


    let addr = "[::]:8080".parse().unwrap();
    println!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn first_handler() -> impl IntoResponse {
    "hello from first\n"
}

async fn second_handler() -> impl IntoResponse {
    "hello from second\n"
}

async fn third_handler() -> impl IntoResponse {
    "hello from third\n"
}