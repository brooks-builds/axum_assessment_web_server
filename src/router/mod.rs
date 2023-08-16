use axum::Router;
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    let serve_dir = ServeDir::new("public");

    Router::new()
        .nest_service("/", serve_dir)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
