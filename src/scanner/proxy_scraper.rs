// CF-VOID Proxy Scraper Module
// Author: CYBER-FORCE
// Scrapes 15+ sources to get 500-1000 proxies
// Auto-reconnect on proxy failure

use std::collections::HashSet;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Instant;

pub struct ProxyScraper {
    pub proxies: Vec<Proxy>,
    pub alive_proxies: Vec<Proxy>,
    pub dead_proxies: Vec<Proxy>,
    pub active_index: usize,
    pub last_scrape: Option<Instant>,
    pub scrape_in_progress: bool,
    pub total_scraped: usize,
    pub total_alive: usize,
    pub test_url: String,
}

#[derive(Clone, Debug)]
pub struct Proxy {
    pub ip: String,
    pub port: u16,
    pub kind: ProxyKind,
    pub country: String,
    pub latency_ms: u64,
    pub alive: bool,
    pub fail_count: u32,
    pub last_used: Option<Instant>,
    pub source: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProxyKind {
    Http,
    Socks4,
    Socks5,
}

impl ProxyKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyKind::Http => "HTTP",
            ProxyKind::Socks4 => "SOCKS4",
            ProxyKind::Socks5 => "SOCKS5",
        }
    }
}

impl Proxy {
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn proxy_url(&self) -> String {
        let scheme = match self.kind {
            ProxyKind::Http => "http",
            ProxyKind::Socks4 => "socks4",
            ProxyKind::Socks5 => "socks5",
        };
        format!("{}://{}:{}", scheme, self.ip, self.port)
    }

    pub fn from_str(line: &str, kind: ProxyKind, source: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 2 {
            if let (Ok(ip), Ok(port)) = (
                parts[0].trim().parse::<String>(),
                parts[1].trim().parse::<u16>(),
            ) {
                // Validate IP format
                if ip.parse::<std::net::Ipv4Addr>().is_ok() || ip.parse::<std::net::Ipv6Addr>().is_ok() {
                    return Some(Proxy {
                        ip,
                        port,
                        kind,
                        country: "??".to_string(),
                        latency_ms: 0,
                        alive: false,
                        fail_count: 0,
                        last_used: None,
                        source: source.to_string(),
                    });
                }
            }
        }
        None
    }
}

impl Default for ProxyScraper {
    fn default() -> Self {
        Self::new()
    }
}

impl ProxyScraper {
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            alive_proxies: Vec::new(),
            dead_proxies: Vec::new(),
            active_index: 0,
            last_scrape: None,
            scrape_in_progress: false,
            total_scraped: 0,
            total_alive: 0,
            test_url: "http://httpbin.org/ip".to_string(),
        }
    }

    pub async fn scrape_all(&mut self) -> usize {
        self.scrape_in_progress = true;
        self.proxies.clear();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        // Source 1: ProxyScrape HTTP
        if let Ok(proxies) = self.scrape_proxyscrape(&client, "http").await {
            self.proxies.extend(proxies);
        }

        // Source 2: ProxyScrape SOCKS5
        if let Ok(proxies) = self.scrape_proxyscrape(&client, "socks5").await {
            self.proxies.extend(proxies);
        }

        // Source 3: ProxyScrape SOCKS4
        if let Ok(proxies) = self.scrape_proxyscrape(&client, "socks4").await {
            self.proxies.extend(proxies);
        }

        // Source 4: Proxy-list.download HTTP
        if let Ok(proxies) = self.scrape_proxy_list_download(&client, "http").await {
            self.proxies.extend(proxies);
        }

        // Source 5: Proxy-list.download SOCKS5
        if let Ok(proxies) = self.scrape_proxy_list_download(&client, "socks5").await {
            self.proxies.extend(proxies);
        }

        // Source 6: Proxy-list.download SOCKS4
        if let Ok(proxies) = self.scrape_proxy_list_download(&client, "socks4").await {
            self.proxies.extend(proxies);
        }

        // Source 7: Geonode
        if let Ok(proxies) = self.scrape_geonode(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 8: Free Proxy List (free-proxy-list.net)
        if let Ok(proxies) = self.scrape_free_proxy_list(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 9: Spys.one
        if let Ok(proxies) = self.scrape_spys_one(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 10: PubProxy
        if let Ok(proxies) = self.scrape_pubproxy(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 11: ProxyDaily
        if let Ok(proxies) = self.scrape_proxy_daily(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 12: Socks-proxy.net
        if let Ok(proxies) = self.scrape_socks_proxy_net(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 13: ProxyDB
        if let Ok(proxies) = self.scrape_proxy_db(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 14: Free-Proxy.world
        if let Ok(proxies) = self.scrape_free_proxy_world(&client).await {
            self.proxies.extend(proxies);
        }

        // Source 15: ProxySpace
        if let Ok(proxies) = self.scrape_proxy_space(&client).await {
            self.proxies.extend(proxies);
        }

        // Deduplicate
        let mut seen = HashSet::new();
        self.proxies.retain(|p| {
            let key = format!("{}:{}:{}", p.ip, p.port, p.kind.as_str());
            seen.insert(key)
        });

        self.total_scraped = self.proxies.len();
        self.last_scrape = Some(Instant::now());
        self.scrape_in_progress = false;

        self.total_scraped
    }

    async fn scrape_proxyscrape(&self, client: &reqwest::Client, proto: &str) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = format!("https://api.proxyscrape.com/v2/?request=displayproxies&protocol={}&timeout=10000&country=all&ssl=yes", proto);
        let resp = client.get(&url).send().await?;
        let text = resp.text().await?;

        let kind = match proto {
            "socks5" => ProxyKind::Socks5,
            "socks4" => ProxyKind::Socks4,
            _ => ProxyKind::Http,
        };

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, kind, "proxyscrape") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_proxy_list_download(&self, client: &reqwest::Client, proxy_type: &str) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = format!("https://www.proxy-list.download/api/v1/get?type={}", proxy_type);
        let resp = client.get(&url).send().await?;
        let text = resp.text().await?;

        let kind = match proxy_type {
            "socks5" => ProxyKind::Socks5,
            "socks4" => ProxyKind::Socks4,
            _ => ProxyKind::Http,
        };

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, kind, "proxy-list.download") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_geonode(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://proxylist.geonode.com/api/proxy-list?limit=500&page=1&sort_by=lastChecked&sort_type=desc";
        let resp = client.get(url).send().await?;
        let json: serde_json::Value = resp.json().await?;

        let mut proxies = Vec::new();
        if let Some(data) = json.get("data") {
            if let Some(arr) = data.as_array() {
                for item in arr {
                    if let (Some(ip), Some(port)) = (
                        item.get("ip").and_then(|v| v.as_str()),
                        item.get("port").and_then(|v| v.as_str()).and_then(|s| s.parse::<u16>().ok())
                    ) {
                        let country = item.get("country")
                            .and_then(|v| v.as_str())
                            .unwrap_or("??")
                            .to_string();
                        proxies.push(Proxy {
                            ip: ip.to_string(),
                            port,
                            kind: ProxyKind::Http,
                            country,
                            latency_ms: 0,
                            alive: false,
                            fail_count: 0,
                            last_used: None,
                            source: "geonode".to_string(),
                        });
                    }
                }
            }
        }
        Ok(proxies)
    }

    async fn scrape_free_proxy_list(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://free-proxy-list.net/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if line.contains('.') && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    if let (Ok(ip_str), Ok(port)) = (
                        parts[0].trim().parse::<String>(),
                        parts[1].trim().parse::<u16>()
                    ) {
                        if ip_str.parse::<std::net::Ipv4Addr>().is_ok() {
                            proxies.push(Proxy {
                                ip: ip_str,
                                port,
                                kind: ProxyKind::Http,
                                country: "??".to_string(),
                                latency_ms: 0,
                                alive: false,
                                fail_count: 0,
                                last_used: None,
                                source: "free-proxy-list".to_string(),
                            });
                        }
                    }
                }
            }
        }
        Ok(proxies)
    }

    async fn scrape_spys_one(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://spys.one/proxy-ssl/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            let cleaned: String = line.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
            let parts: Vec<&str> = cleaned.split('.').collect();
            if parts.len() >= 4 {
                let candidate = line.trim();
                if let Some(proxy) = Proxy::from_str(candidate, ProxyKind::Http, "spys.one") {
                    proxies.push(proxy);
                }
            }
        }
        Ok(proxies)
    }

    async fn scrape_pubproxy(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://pubproxy.com/api/proxy?limit=500&format=txt&type=http";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Http, "pubproxy") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_proxy_daily(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://www.proxy-daily.com/get-free-proxy";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Http, "proxy-daily") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_socks_proxy_net(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://www.socks-proxy.net/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Socks5, "socks-proxy.net") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_proxy_db(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://proxydb.l33tsite.org/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Http, "proxydb") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_free_proxy_world(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://www.free-proxy.world/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Http, "free-proxy.world") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    async fn scrape_proxy_space(&self, client: &reqwest::Client) -> Result<Vec<Proxy>, reqwest::Error> {
        let url = "https://proxyspace.net/";
        let resp = client.get(url).send().await?;
        let text = resp.text().await?;

        let mut proxies = Vec::new();
        for line in text.lines() {
            if let Some(proxy) = Proxy::from_str(line, ProxyKind::Http, "proxy-space") {
                proxies.push(proxy);
            }
        }
        Ok(proxies)
    }

    pub async fn test_proxy(&self, proxy: &Proxy) -> Result<(bool, u64), reqwest::Error> {
        let proxy_url = proxy.proxy_url();
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_url)?)
            .timeout(std::time::Duration::from_secs(5))
            .build()?;

        let start = Instant::now();
        let result = client.get(&self.test_url).send().await;
        let elapsed = start.elapsed().as_millis() as u64;

        match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok((true, elapsed))
                } else {
                    Ok((false, elapsed))
                }
            }
            Err(_) => Ok((false, elapsed)),
        }
    }

    pub async fn test_all(&mut self, max_concurrent: usize) {
        self.alive_proxies.clear();
        self.dead_proxies.clear();

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrent));
        let mut tasks = Vec::new();

        for proxy in self.proxies.clone() {
            let sem = semaphore.clone();
            let test_url = self.test_url.clone();
            let client = reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(&proxy.proxy_url()).unwrap())
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new());

            let task = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let start = Instant::now();
                let result = client.get(&test_url).send().await;
                let elapsed = start.elapsed().as_millis() as u64;

                match result {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            (proxy, true, elapsed)
                        } else {
                            (proxy, false, elapsed)
                        }
                    }
                    Err(_) => (proxy, false, elapsed),
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            if let Ok((mut proxy, alive, latency)) = task.await {
                proxy.alive = alive;
                proxy.latency_ms = latency;
                if alive {
                    self.alive_proxies.push(proxy);
                } else {
                    self.dead_proxies.push(proxy);
                }
            }
        }

        // Sort alive proxies by latency (fastest first)
        self.alive_proxies.sort_by_key(|p| p.latency_ms);
        self.total_alive = self.alive_proxies.len();
    }

    pub fn get_next_proxy(&mut self) -> Option<&Proxy> {
        if self.alive_proxies.is_empty() {
            return None;
        }

        // Round-robin with health check
        let len = self.alive_proxies.len();
        for i in 0..len {
            let idx = (self.active_index + i) % len;
            let proxy = &mut self.alive_proxies[idx];
            if proxy.fail_count < 3 {
                self.active_index = idx;
                proxy.last_used = Some(Instant::now());
                return Some(&self.alive_proxies[idx]);
            }
        }
        None
    }

    pub fn mark_failed(&mut self, ip: &str, port: u16) {
        for proxy in &mut self.alive_proxies {
            if proxy.ip == ip && proxy.port == port {
                proxy.fail_count += 1;
                if proxy.fail_count >= 3 {
                    proxy.alive = false;
                    if let Some(pos) = self.alive_proxies.iter().position(|p| p.ip == ip && p.port == port) {
                        let proxy = self.alive_proxies.remove(pos);
                        self.dead_proxies.push(proxy);
                    }
                }
                break;
            }
        }
    }

    pub async fn auto_reconnect(&mut self) -> Result<Option<Proxy>, ()> {
        // Mark current as failed
        if let Some(current) = self.alive_proxies.get_mut(self.active_index) {
            current.fail_count += 1;
            if current.fail_count >= 3 {
                current.alive = false;
            }
        }

        // Try to get next alive
        if let Some(proxy) = self.get_next_proxy() {
            return Ok(Some(proxy.clone()));
        }

        // No alive proxies, re-scrape
        self.scrape_all().await;
        // Test new batch (quick test with fewer concurrent)
        self.test_all(50).await;

        if let Some(proxy) = self.get_next_proxy() {
            Ok(Some(proxy.clone()))
        } else {
            Err(())
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.total_scraped, self.alive_proxies.len(), self.dead_proxies.len())
    }

    pub fn format_status(&self) -> String {
        let (total, alive, dead) = self.stats();
        let bar_width = 20;
        let pct = if total > 0 { (alive as f32 / total as f32) * 100.0 } else { 0.0 };
        let filled = (pct / 100.0 * bar_width as f32) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "▱".repeat(bar_width - filled));
        format!("{} {:.0}% {}/{} alive", bar, pct, alive, total)
    }
}
