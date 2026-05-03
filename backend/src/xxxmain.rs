mod auth;
mod db;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let pool = db::init_db().await.expect("Failed to initialize database");

    let app = Router::new()
        .route("/api/register", post(handlers::register))
        .route("/api/login", post(handlers::login))
        .route("/api/users", get(handlers::get_all_users))
        .route("/api/users/:id", get(handlers::get_user))
        .route("/api/users/:id", delete(handlers::delete_user))
        .route("/api/me", get(handlers::get_current_user))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
