use axum::{response::IntoResponse, routing::get, Router, Server};
//use tower::limit::{ConcurrencyLimitLayer};
use std::error;
//use std::fmt;

mod db;
mod error_handler;

struct Note {
    id: i32,
    name: String,
}

#[tokio::main]
async fn http_server_run() {
    let app = Router::new().route("/", get(first_handler))
    .route("/foo", get(second_handler))
    //.layer(ConcurrencyLimitLayer::new(64))
    .route("/bar", get(third_handler));


    let addr = "[::]:8080".parse().unwrap();
    println!("Listening on http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



/*#[tokio::main]
async*/ fn main() -> std::io::Result<()> {
    db::init();
    //std::thread::spawn(|| {
    //    println!("spawn !");
        http_server_run();
    //});
    /*for row in client.execute("SELECT * FROM notes", &[])? {
        let note = Note{
            id: row.get(0),
            name: row.get(1),
        };
        println!("Note = {} {}", note.id, note.name);
    }*/


    Ok(())
}

async fn first_handler() -> impl IntoResponse {
    "hello from first\n"
}

async fn second_handler() -> impl IntoResponse {
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


/*
    let blocking_task = tokio::task::spawn(move || {
        let mut client = db::connection().expect("REASON");
        //        let result = client.expect("REASON").execute("SELECT 1", &[]);
        //        println!(Note = {}, result);
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
 */