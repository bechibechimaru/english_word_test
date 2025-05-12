use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use axum::{
    routing::post,
    Router,
};
use hyper::Method;
use tower_http::cors::{CorsLayer, AllowHeaders, Any};
use sqlx::mysql::MySqlPool;

use crate::presentation::handler::generate_test_handler;

mod presentation;
mod application;
mod domain;
mod infra;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = MySqlPool::connect(&database_url).await?;
    let app_state = Arc::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any())
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS]);

    let app = Router::new()
        .route("/generate-test", post(generate_test_handler))
        .with_state(app_state)
        .layer(cors);

    let port = env::var("PORT").expect("PORT must be set.");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
