use axum::{response::IntoResponse, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let addr = "[::]:8080".parse().unwrap();
    println!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "hello from axum\n"
}