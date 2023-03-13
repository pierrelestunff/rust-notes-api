use crate::{
    middleware::require_authentication::require_authentication,
    db
};

use axum::{
    response::IntoResponse, 
    routing::{delete, get, patch, post, put},
    Router, 
    Server, 
    TypedHeader, 
    headers::UserAgent,
    middleware
};

//use tower::limit::{ConcurrencyLimitLayer};
use std::error;
//use std::fmt;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;

use http::{header, Method};

struct Note {
    id: i32,
    name: String,
}

pub async fn run() {
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
//    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any)
    .allow_headers(vec![
        header::ACCEPT,
        header::ACCEPT_LANGUAGE,
        header::AUTHORIZATION,
        header::CONTENT_LANGUAGE,
        header::CONTENT_TYPE,
    ]);

    let app = Router::new().route("/", get(first_handler))
    .route("/foo", get(second_handler))
    //.layer(ConcurrencyLimitLayer::new(64))
    .route("/bar", get(third_handler))
    .layer(ServiceBuilder::new()
        .layer(cors)
        .layer(middleware::from_fn(require_authentication))
    );

    let addr = "[::]:3032".parse().unwrap();
    println!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn first_handler() -> impl IntoResponse {
    "hello from first\n"
}

async fn second_handler(TypedHeader(user_agent): TypedHeader<UserAgent>,) -> impl IntoResponse {

    println!("`{user_agent}`");

    let handle = tokio::task::spawn_blocking(|| {
        let mut client = db::connection().expect("REASON");
        let mut note : Note = Note{
            id: 0,
            name: "unknown".to_string(),
        };
                for row in client.query("SELECT * FROM notes", &[]).unwrap() {
                    note = Note{
                        id: row.get(0),
                        name: row.get(1),
                    };
                    println!("Note = {} {}", note.id, note.name);
                }
        note.name

    });
    let out = handle.await.unwrap();
    out
}

async fn third_handler() -> impl IntoResponse {
    "hello from third\n"
}