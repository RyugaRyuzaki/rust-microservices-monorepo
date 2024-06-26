use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/", get(|| async { "Hello World!!!" }));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  info!("Server is running on port 3000");
  axum::serve(listener, app).await.unwrap();
}
