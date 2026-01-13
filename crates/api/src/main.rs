use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use database::{create_pool, link_repo::LinkRepo};
use services::link_service::LinkService;

mod handlers;

pub struct AppState {
    link_service: LinkService,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    // Logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // DB pool
    let pool = create_pool();
    let link_repo = LinkRepo::new(pool);
    let link_service = LinkService::new(link_repo);

    let app_state = Arc::new(AppState { link_service });

    let api = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/links/create", post(handlers::create_link))
        .route("/links/{code}", get(handlers::get_link));

    let app = Router::new()
        .nest("/manager-api", api)
        .route("/{code}", get(handlers::code_redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
