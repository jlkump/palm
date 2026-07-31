use axum::{Router, routing::{get, post}};
use palm_backend::route;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthcheck", get(|| async { "Feeling good!" }))
        // Authentication
        .route("/api/auth/login", post(route::create_user))
        .route("/api/auth/logout", post(route::create_user))
        // User management
        .route("/api/user/create", post(route::create_user))
        .route("/api/user/{user_id}", get(route::create_user))
        .route("/api/user/{user_id}/sync", post(route::create_user))
        .route("/api/user/{user_id}/sync/{service_name}", post(route::create_user))
        .route("/api/user/{user_id}/delete", post(route::create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}