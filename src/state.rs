use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub worker_id: String,
    pub pool: String,
    pub hashrate: f64,
    pub temperature: f64,
    pub timestamp: u64,
}

#[derive(Default)]
pub struct AppState {
    pub report_ttl: u64,
    pub reports: Vec<Report>,
}

impl AppState {
    pub fn new(report_ttl: u64) -> Self {
        Self {
            report_ttl,
            reports: Vec::new(),
        }
    }

    pub fn add_report(&mut self, report: Report) {
        let pos = self.reports
            .binary_search_by(|r| r.timestamp.cmp(&report.timestamp))
            .unwrap_or_else(|pos| pos);
        self.reports.insert(pos, report);
    }

    pub fn cleanup_expired(&mut self) {
        let cutoff_time = Utc::now().timestamp() as u64 - self.report_ttl;
        let first_valid = self.reports.iter().position(|report| report.timestamp >= cutoff_time);
        match first_valid {
            Some(index) => {
                self.reports.drain(0..index);
            }
            None => {
                self.reports.clear();
            }
        }
    }
}