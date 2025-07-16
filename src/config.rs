#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub report_ttl: u64,
}

impl Config {
    pub fn new(port: u16, report_ttl: u64) -> Self {
        Self {
            port: port,
            report_ttl: report_ttl
        }
    }
}