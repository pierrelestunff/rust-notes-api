mod db;
mod error_handler;
mod middleware;
mod server;
mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    db::init().await;
    server::run().await;
    Ok(())
}
