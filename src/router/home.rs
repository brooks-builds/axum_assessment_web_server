use axum::extract::Query;
use axum::http::StatusCode;
use axum::{extract::State, response::Html};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::app_state::AppState;

pub async fn home(
    state: State<AppState>,
    Query(query): Query<QueryParams>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    let name = if let Some(name) = &query.name {
        name.as_str()
    } else {
        "World"
    };
    context.insert("name", name);
    let html = state.tera.render("home.html", &context).map_err(|error| {
        tracing::error!("Error rendering home template: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(html))
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub name: Option<String>,
}
