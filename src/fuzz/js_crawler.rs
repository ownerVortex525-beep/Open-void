use crate::cli::banner;
use crate::http::client::HttpClient;
use crate::fuzz::FuzzResult;
use scraper::{Html, Selector};
use std::time::Duration;
use std::collections::HashSet;

pub async fn crawl_js_endpoints(
    url: &str,
    result: &mut FuzzResult,
    client: &HttpClient
) -> anyhow::Result<()> {
    banner::print_scanning("JS Endpoint Discovery", url);
    
    // First, get the HTML and find JS files
    match tokio::time::timeout(
        Duration::from_secs(10),
        client.get(url)
    ).await {
        Ok(Ok(response)) => {
            let body = response.text().await.unwrap_or_default();
            
            match Html::parse_document(&body) {
                document => {
                    let mut js_urls = HashSet::new();
                    
                    // Find script src attributes
                    let script_selector = Selector::parse(r#"script[src]"#).unwrap();
                    for element in document.select(&script_selector) {
                        if let Some(src) = element.value().attr("src") {
                            js_urls.insert(src.to_string());
                        }
                    }
                    
                    // Parse JS files for endpoints
                    for js_url in js_urls {
                        parse_js_for_endpoints(&js_url, url, result, client).await?;
                    }
                }
            }
        }
        _ => {
            banner::warning(&format!("Failed to fetch page for JS crawling: {}", url));
        }
    }
    
    Ok(())
}

async fn parse_js_for_endpoints(
    js_url: &str,
    base_url: &str,
    result: &mut FuzzResult,
    client: &HttpClient
) -> anyhow::Result<()> {
    let full_url = if js_url.starts_with("http://") || js_url.starts_with("https://") {
        js_url.to_string()
    } else if js_url.starts_with("//") {
        format!("https:{}", js_url)
    } else {
        let base = base_url.trim_end_matches('/').rsplit('/').next().unwrap_or("");
        format!("{}/{}", base_url.trim_end_matches(base), js_url)
    };
    
    result.total_requests += 1;
    
    match tokio::time::timeout(
        Duration::from_secs(5),
        client.get(&full_url)
    ).await {
        Ok(Ok(response)) => {
            let body = response.text().await.unwrap_or_default();
            
            // Look for URL patterns in JS
            let url_pattern = regex::Regex::new(r#"["']([^"']*(?:api|endpoint|url|v\d+/|controller|action)[^"']*)["']"#).unwrap();
            let fetch_pattern = regex::Regex::new(r#"(?:fetch|axios\.get|axios\.post|xhr\.open|jQuery\.get|jQuery\.post)\(["']([^"']+)["']\)"#).unwrap();
            
            for cap in url_pattern.captures_iter(&body) {
                let endpoint = cap.get(1).unwrap().as_str().to_string();
                banner::info(&format!("JS endpoint found: {}", endpoint));
                
                // Extract parameters if present
                let params: Vec<String> = if endpoint.contains('{') {
                    let param_pattern = regex::Regex::new(r#"\{\{(\w+)\}\}"#).unwrap();
                    param_pattern.captures_iter(&endpoint)
                        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
                        .collect()
                } else {
                    vec![]
                };
                
                result.js_endpoints.push(crate::fuzz::JSEndpoint {
                    url: endpoint,
                    method: "GET".to_string(),
                    params,
                });
                result.found_count += 1;
            }
            
            for cap in fetch_pattern.captures_iter(&body) {
                let endpoint = cap.get(1).unwrap().as_str().to_string();
                banner::info(&format!("Fetch endpoint found: {}", endpoint));
                
                result.js_endpoints.push(crate::fuzz::JSEndpoint {
                    url: endpoint,
                    method: "FETCH".to_string(),
                    params: vec![],
                });
                result.found_count += 1;
            }
        }
        _ => {}
    }
    
    Ok(())
}
