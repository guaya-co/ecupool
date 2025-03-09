use axum::{Json, Router, routing::get};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
struct Health {
    ok: bool,
}

pub fn init_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/ping", get(async || Json(Health { ok: true })))
        .with_state(app_state)
}
