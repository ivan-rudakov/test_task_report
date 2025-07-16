use axum::{
    extract::{State},
    Json,
};
use serde::{Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub pools: HashMap<String, PoolStats>,
}

#[derive(Debug, Serialize, Default)]
pub struct PoolStats {
    pub workers: usize,
    pub avg_hashrate: f64,
    pub avg_temp: f64,
}

type SharedState = Arc<RwLock<AppState>>;

pub async fn serve_stats(State(state): State<SharedState>) -> Json<StatsResponse> {
    let mut state = state.write().unwrap();
    state.cleanup_expired();
    
    let pools = state.reports.iter()
        .fold(HashMap::<String, (f64, f64, usize)>::new(), |mut acc, report| {
            let entry = acc.entry(report.pool.clone()).or_default();
            entry.0 += report.hashrate;
            entry.1 += report.temperature;
            entry.2 += 1;
            acc
        })
        .into_iter()
        .map(|(pool, (total_hash, total_temp, workers))| {
            let stats = PoolStats {
                workers,
                avg_hashrate: total_hash / workers as f64,
                avg_temp: total_temp / workers as f64,
            };
            (pool, stats)
        })
        .collect();
    
    Json(StatsResponse { pools })
}