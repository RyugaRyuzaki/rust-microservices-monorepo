use jd_core::config::ProdConfig;
use jd_infra::initialed_db;
use jd_infra::middleware::{map_response, mw_auth};

use axum::{
  middleware::{self}, Router,
};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let subscriber = tracing_subscriber::fmt::Subscriber::builder()
    .without_time()
    .with_target(false)
    .with_max_level(tracing::Level::TRACE)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("Cannot setting subscriber global");

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new()
    .merge(jd_api::user_routes())
    .layer(middleware::map_response(map_response::mw_map_response)) // 1
    .layer(middleware::from_fn_with_state(pool.clone(), mw_auth::mw_auth)) // 2
    .with_state(pool);

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  info!("Server is running on port: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}
