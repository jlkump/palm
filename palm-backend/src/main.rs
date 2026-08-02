use axum::{Router, routing::{delete, get, post}};
use palm_backend::route;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthcheck", get(|| async { "Feeling good!" }))
        // Authentication
        .route("/api/auth/login", post(route::create_user))
        .route("/api/auth/logout", post(route::create_user))
        // User management
        // Create a user
        .route("/api/user/create", post(route::create_user))
        // Get user info
        // - What services they have
        // - Their profile info
        .route("/api/user/{user_id}", get(route::create_user))
        // Sync all user services
        .route("/api/user/{user_id}/sync", post(route::create_user))
        // Sync specific user service
        .route("/api/user/{user_id}/sync/{service_name}", post(route::create_user))
        // Post to create service for a user
        .route("/api/user/{user_id}/{service_name}", post(route::create_user))
        // Get to get info on a service for a user
        .route("/api/user/{user_id}/{service_name}", get(route::create_user))
        // Delete a user's service
        .route("/api/user/{user_id}/{service_name}", delete(route::create_user))
        // Delete a user
        .route("/api/user/{user_id}/delete", delete(route::create_user))
        // Get all services available
        // - with service info, include link to service
        .route("/api/services", get(route::create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}