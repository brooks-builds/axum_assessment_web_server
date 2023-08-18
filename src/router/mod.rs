mod home;

use axum::{routing::get, Router};
use home::home;
use tower_http::services::ServeDir;

use crate::app_state::AppState;

pub fn create_router(state: AppState) -> Router {
    let serve_dir = ServeDir::new("public");

    Router::new()
        .route("/", get(home))
        .fallback(home)
        .nest_service("/public/", serve_dir)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
