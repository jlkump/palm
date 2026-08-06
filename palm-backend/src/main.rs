use std::sync::Arc;

use axum::{Router, routing::{delete, get, post, put}};
use palm_backend::{route, AppState};

#[tokio::main]
async fn main() {
    let app_state = AppState::initialize().await;

    // Difference between PUT & POST
    //  - PUT is idempotent (same result each time it is requested)
    //  - POST is not. It may fail if it is called a second time with the same info to prevent duplication.
    let app = Router::new()
        .route("/healthcheck", get(|| async { "Feeling good!\n" }))
        // Authentication
        // .route("/api/auth/login", post(route::create_user))
        // .route("/api/auth/logout", post(route::create_user))
        // User management
        // Create a user
        .route("/api/user/create", post(route::create_user))
        .route("/api/users", get(route::get_users))
        // Update palm user info, sync across all services when possible
        .route("/api/user/{user_id}", put(route::update_user))
        // Get user info
        // - What services they have
        // - Their profile info
        .route("/api/user/{user_id}", get(route::get_user))
        // Create service for a user
        .route("/api/user/{user_id}/{service_name}", post(route::create_user_service))
        // Sync user service
        .route("/api/user/{user_id}/{service_name}", put(route::sync_user_service))
        // Get to get info on a service for a user
        .route("/api/user/{user_id}/{service_name}", get(route::get_user_service))
        // Delete a user's service
        .route("/api/user/{user_id}/{service_name}", delete(route::delete_user_service))
        // Delete a user
        .route("/api/user/{user_id}/delete", delete(route::delete_user))
        // Get all services available
        // - with service info, include link to service
        .route("/api/services", get(route::get_services))
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}