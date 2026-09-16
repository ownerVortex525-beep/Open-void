pub mod directory;
pub mod parameter;
pub mod js_crawler;

use crate::cli::banner;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::http::client::HttpClient;
use crate::core::finding::Finding;

pub struct FuzzResult {
    pub target: String,
    pub found_paths: Vec<FuzzFinding>,
    pub found_params: Vec<FuzzFinding>,
    pub js_endpoints: Vec<JSEndpoint>,
    pub total_requests: usize,
    pub found_count: usize,
}

pub struct FuzzFinding {
    pub path: String,
    pub status: u16,
    pub content_length: usize,
    pub content_type: String,
}

pub struct JSEndpoint {
    pub url: String,
    pub method: String,
    pub params: Vec<String>,
}

impl FuzzResult {
    pub fn new(target: String) -> Self {
        Self {
            target,
            found_paths: Vec::new(),
            found_params: Vec::new(),
            js_endpoints: Vec::new(),
            total_requests: 0,
            found_count: 0,
        }
    }

    pub fn print_summary(&self) {
        banner::print_phase_banner("Fuzzing Results", &self.target);
        
        if self.found_count == 0 {
            banner::info("No findings discovered during fuzzing");
            return;
        }
        
        for finding in &self.found_paths {
            banner::finding(
                "LOW",
                &format!("{} ({})", finding.path, finding.status),
                &self.target,
                &format!("Size: {} bytes, Type: {}", finding.content_length, finding.content_type)
            );
        }
        
        for param in &self.found_params {
            banner::finding(
                "LOW",
                &format!("Parameter: {} ({})", param.path, param.status),
                &self.target,
                &format!("Size: {} bytes", param.content_length)
            );
        }
        
        for js in &self.js_endpoints {
            banner::info(&format!("JS Endpoint: {} [{}]", js.url, js.method));
            for param in &js.params {
                banner::info(&format!("  Parameter: {}", param));
            }
        }
        
        banner::print_complete("Fuzz completed", self.found_count, self.total_requests);
    }
}

pub async fn run_fuzz(
    url: &str,
    extensions: Option<&str>,
    client: &HttpClient
) -> anyhow::Result<FuzzResult> {
    let mut result = FuzzResult::new(url.to_string());
    
    directory::fuzz_directories(url, extensions, &mut result, client).await?;
    
    parameter::fuzz_parameters(url, &mut result, client).await?;
    
    js_crawler::crawl_js_endpoints(url, &mut result, client).await?;
    
    Ok(result)
}
