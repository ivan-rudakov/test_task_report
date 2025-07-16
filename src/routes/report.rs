use axum::{
    extract::State,
    Json,
};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::state::{AppState, Report};

type SharedState = Arc<RwLock<AppState>>;

pub async fn serve_report(
    State(state): State<SharedState>,
    Json(payload): Json<Report>,
) -> Json<serde_json::Value> {
    let mut state = state.write().unwrap();
    state.add_report(payload);
    Json(json!({ "status": "success" }))
}