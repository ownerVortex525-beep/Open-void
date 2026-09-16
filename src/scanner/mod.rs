// CF-VOID Scanner Module
// All scanner implementations

pub mod web;
pub mod network;
pub mod credentials;
pub mod dns;
pub mod osint;
pub mod social;
pub mod post;
pub mod iot;
pub mod cloud;
pub mod proxy_scraper;

use async_trait::async_trait;
use crate::core::finding::ScanResult;
use crate::http::client::HttpClient;

/// Scanner trait - all scanners implement this
#[async_trait]
pub trait Scanner: Send + Sync {
    /// Get scanner name
    fn name(&self) -> &str;

    /// Get scanner description
    fn description(&self) -> &str;

    /// Run the scan
    async fn scan(&self, target: &str, client: &HttpClient) -> Result<ScanResult, anyhow::Error>;
}

/// Scanner configuration
pub struct ScannerConfig {
    pub threads: usize,
    pub intensity: u8,
    pub timeout: u64,
    pub depth: u8,
    pub max_urls: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            threads: 10,
            intensity: 25,
            timeout: 30,
            depth: 3,
            max_urls: 1000,
        }
    }
}
