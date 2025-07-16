use axum::{
    routing::{get, post},
    Router,
};
use std::{
    env, net::SocketAddr, sync::{Arc, RwLock}
};

use crate::{config::Config, routes::{report::serve_report, stats::serve_stats}, state::AppState};

mod config;
mod state;
mod routes;


type SharedState<AppState> = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .expect("PORT must be number");
    let report_ttl = env::var("REPORT_TTL")
                .unwrap_or_else(|_| "36".to_string())
                .parse()
                .expect("REPORT_TTL must be number");

    let config = Config::new(port, report_ttl);
    let state  = Arc::new(RwLock::new(AppState::new(config.report_ttl)));
    
    let app = Router::new()
        .route("/report", post(serve_report))
        .route("/stats", get(serve_stats))
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server started {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Address bind error");
    
    axum::serve(listener, app)
        .await
        .expect("Error while server start");
}