use anyhow::{Result, anyhow};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout: Duration,
    pub insecure: bool,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
    pub follow_redirects: bool,
    pub custom_headers: Vec<(String, String)>,
    pub cookie: Option<String>,
    pub proxy_rotation: bool,
    pub max_retries: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            insecure: false,
            proxy: None,
            user_agent: None,
            follow_redirects: true,
            custom_headers: Vec::new(),
            cookie: None,
            proxy_rotation: false,
            max_retries: 3,
        }
    }
}

pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let mut builder = Client::builder()
            .timeout(config.timeout)
            .danger_accept_invalid_certs(config.insecure)
            .redirect(reqwest::redirect::Policy::limited(10));

        if let Some(proxy) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| anyhow!("Proxy error: {}", e))?;
            builder = builder.proxy(proxy);
        }

        let ua = config.user_agent.clone().unwrap_or_else(|| "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());
        builder = builder.user_agent(ua);

        let client = builder.build().map_err(|e| anyhow!("Client error: {}", e))?;
        Ok(Self { client, config })
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        let mut req = self.client.get(url);
        req = self.apply_headers(req);
        req.send().await.map_err(|e| anyhow!("GET failed: {}", e))
    }

    pub async fn get_with_headers(&self, url: &str, extra: &[(String, String)]) -> Result<reqwest::Response> {
        let mut req = self.client.get(url);
        req = self.apply_headers(req);
        for (k, v) in extra {
            req = req.header(k.as_str(), v.as_str());
        }
        req.send().await.map_err(|e| anyhow!("GET failed: {}", e))
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<reqwest::Response> {
        let mut req = self.client.post(url).body(body.to_string()).header("Content-Type", "application/json");
        req = self.apply_headers(req);
        req.send().await.map_err(|e| anyhow!("POST failed: {}", e))
    }

    pub async fn post_with_headers(&self, url: &str, body: &str, extra: &[(String, String)]) -> Result<reqwest::Response> {
        let mut req = self.client.post(url).body(body.to_string());
        req = self.apply_headers(req);
        for (k, v) in extra {
            req = req.header(k.as_str(), v.as_str());
        }
        req.send().await.map_err(|e| anyhow!("POST failed: {}", e))
    }

    pub async fn head(&self, url: &str) -> Result<reqwest::Response> {
        let mut req = self.client.head(url);
        req = self.apply_headers(req);
        req.send().await.map_err(|e| anyhow!("HEAD failed: {}", e))
    }

    pub fn apply_headers(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        for (k, v) in &self.config.custom_headers {
            req = req.header(k.as_str(), v.as_str());
        }
        if let Some(cookie) = &self.config.cookie {
            req = req.header("Cookie", cookie.as_str());
        }
        req
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}
