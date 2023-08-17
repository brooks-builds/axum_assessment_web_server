use axum::http::StatusCode;
use axum::{extract::State, response::Html};
use tera::Context;

use crate::app_state::AppState;

pub async fn home(state: State<AppState>) -> Result<Html<String>, StatusCode> {
    todo!("pass in context to the template with with value in query params");
    let context = Context::new();
    let html = state.tera.render("home.html", &context).map_err(|error| {
        tracing::error!("Error rendering home template: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(html))
}
