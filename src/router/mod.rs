mod home;

use axum::{routing::get, Router};
use home::home;

/// Update the router so that it serves all files in the public directory to the route /public.
///
/// For example, sending a GET request to /public/sample.json should result in the file sample.json being sent back to the client
///
/// Also update the router so that a 404 would instead return a 200 code and run the home route
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
