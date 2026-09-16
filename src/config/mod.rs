use crate::cli::banner;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod keys;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub http: HttpConfig,
    #[serde(default)]
    pub scanning: ScanningConfig,
    #[serde(default)]
    pub fuzzing: FuzzingConfig,
    #[serde(default)]
    pub credentials: CredentialConfig,
    #[serde(default)]
    pub payloads: PayloadConfig,
    #[serde(default)]
    pub post_exploit: PostExploitConfig,
    #[serde(default)]
    pub reporting: ReportingConfig,
    #[serde(default)]
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub session: SessionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub stealth: bool,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_version() -> String { "1.0".to_string() }
fn default_name() -> String { "CF-VOID".to_string() }
fn default_author() -> String { "IND 'CYBER-FORCE'".to_string() }
fn default_timeout() -> u64 { 30 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpConfig {
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_true")]
    pub follow_redirects: bool,
    #[serde(default = "default_10")]
    pub max_redirects: usize,
    pub cookie: String,
    pub bearer_token: String,
    #[serde(default)]
    pub default_headers: Vec<String>,
}

fn default_user_agent() -> String {
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()
}
fn default_true() -> bool { true }
fn default_10() -> usize { 10 }
fn default_10_u64() -> u64 { 10 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanningConfig {
    #[serde(default = "default_ports")]
    pub default_ports: String,
    #[serde(default = "default_5")]
    pub host_timeout: u64,
    #[serde(default = "default_3")]
    pub max_retries: u32,
    #[serde(default = "default_connect")]
    pub scan_type: String,
    #[serde(default)]
    pub rate_limit: u64,
    #[serde(default)]
    pub decoy_scanning: bool,
    #[serde(default = "default_24")]
    pub subnet_cidr: String,
}

fn default_ports() -> String { "1-1000".to_string() }
fn default_5() -> u64 { 5 }
fn default_3() -> u32 { 3 }
fn default_connect() -> String { "connect".to_string() }
fn default_24() -> String { "24".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuzzingConfig {
    #[serde(default = "default_extensions")]
    pub extensions: String,
    #[serde(default = "default_wordlist")]
    pub wordlist_path: String,
    #[serde(default = "default_10")]
    pub threads: usize,
    #[serde(default)]
    pub delay: u64,
    #[serde(default = "default_10_u64")]
    pub timeout: u64,
    #[serde(default = "default_true")]
    pub follow_redirects: bool,
    #[serde(default)]
    pub match_status: Vec<u16>,
    #[serde(default)]
    pub exclude_status: Vec<u16>,
}

fn default_extensions() -> String {
    ".html,.php,.js,.css,.xml,.json,.asp,.aspx".to_string()
}
fn default_wordlist() -> String { "wordlists/rockyou.txt".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CredentialConfig {
    #[serde(default = "default_10")]
    pub brute_threads: usize,
    #[serde(default = "default_10_u64")]
    pub brute_timeout: u64,
    #[serde(default = "default_userlist")]
    pub userlist_path: String,
    #[serde(default = "default_wordlist")]
    pub passlist_path: String,
    #[serde(default = "default_22")]
    pub brute_ssh_port: u16,
    #[serde(default = "default_80")]
    pub brute_http_path: String,
    #[serde(default = "default_21")]
    pub brute_ftp_port: u16,
    #[serde(default = "default_3306")]
    pub brute_mysql_port: u16,
    #[serde(default = "default_3389")]
    pub brute_rdp_port: u16,
    #[serde(default = "default_true")]
    pub test_default: bool,
}

fn default_userlist() -> String { "wordlists/usernames.txt".to_string() }
fn default_22() -> u16 { 22 }
fn default_80() -> String { "/admin".to_string() }
fn default_21() -> u16 { 21 }
fn default_3306() -> u16 { 3306 }
fn default_3389() -> u16 { 3389 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayloadConfig {
    #[serde(default = "default_lhost")]
    pub default_lhost: String,
    #[serde(default = "default_4444")]
    pub default_lport: u16,
    #[serde(default = "default_x64")]
    pub default_arch: String,
    #[serde(default = "default_exe")]
    pub default_format: String,
    #[serde(default = "default_true")]
    pub encode_payloads: bool,
    #[serde(default)]
    pub obfuscate: bool,
}

fn default_lhost() -> String { "127.0.0.1".to_string() }
fn default_4444() -> u16 { 4444 }
fn default_x64() -> String { "x64".to_string() }
fn default_exe() -> String { "exe".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostExploitConfig {
    #[serde(default = "default_registry")]
    pub persistence_method: String,
    #[serde(default = "default_reflective")]
    pub process_injection: String,
    #[serde(default = "default_true")]
    pub keylogger_enabled: bool,
    #[serde(default = "default_30")]
    pub screenshot_interval: u64,
    #[serde(default = "default_http")]
    pub exfil_method: String,
    #[serde(default)]
    pub cleanup_traces: bool,
    #[serde(default)]
    pub proxy_enabled: bool,
}

fn default_registry() -> String { "registry".to_string() }
fn default_reflective() -> String { "reflective".to_string() }
fn default_http() -> String { "http".to_string() }
fn default_30() -> u64 { 30 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingConfig {
    #[serde(default = "default_reports")]
    pub report_dir: String,
    #[serde(default = "default_true")]
    pub html_enabled: bool,
    #[serde(default = "default_true")]
    pub json_enabled: bool,
    #[serde(default)]
    pub xml_enabled: bool,
    #[serde(default = "default_true")]
    pub csv_enabled: bool,
    #[serde(default = "default_true")]
    pub markdown_enabled: bool,
    #[serde(default = "default_true")]
    pub include_evidence: bool,
    #[serde(default = "default_true")]
    pub include_recommendations: bool,
}

fn default_reports() -> String { "reports".to_string() }

fn default_proxy_mode() -> String { "manual".to_string() }
fn default_500() -> usize { 500 }
fn default_10000() -> u64 { 10000 }
fn default_50() -> usize { 50 }
fn default_httpbin() -> String { "http://httpbin.org/ip".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyConfig {
    pub http_proxy: String,
    pub https_proxy: String,
    pub socks5_proxy: String,
    pub proxy_auth: String,
    #[serde(default = "default_10_u64")]
    pub proxy_timeout: u64,
    #[serde(default = "default_proxy_mode")]
    pub proxy_mode: String,           // "manual" | "auto_scrape" | "rotating"
    #[serde(default = "default_true")]
    pub auto_reconnect: bool,
    #[serde(default = "default_500")]
    pub scrape_limit: usize,
    #[serde(default = "default_10000")]
    pub scrape_timeout_ms: u64,
    #[serde(default = "default_true")]
    pub test_proxies: bool,
    #[serde(default = "default_50")]
    pub test_concurrent: usize,
    #[serde(default)]
    pub custom_proxies: Vec<String>,  // User-added proxies
    #[serde(default = "default_httpbin")]
    pub test_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    #[serde(default = "default_4444")]
    pub listener_default_port: u16,
    #[serde(default = "default_0_0_0_0")]
    pub listener_host: String,
    #[serde(default = "default_3600")]
    pub session_timeout: u64,
    #[serde(default = "default_10_sessions")]
    pub max_sessions: u32,
    #[serde(default)]
    pub auto_interact: bool,
}

fn default_0_0_0_0() -> String { "0.0.0.0".to_string() }
fn default_3600() -> u64 { 3600 }
fn default_10_sessions() -> u32 { 10 }

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            http: HttpConfig::default(),
            scanning: ScanningConfig::default(),
            fuzzing: FuzzingConfig::default(),
            credentials: CredentialConfig::default(),
            payloads: PayloadConfig::default(),
            post_exploit: PostExploitConfig::default(),
            reporting: ReportingConfig::default(),
            proxy: ProxyConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let path = Path::new(path);
        if !path.exists() {
            banner::info(&format!("Config file not found: {}, using defaults", path.display()));
            return Ok(Self::default());
        }
        
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        
        banner::success(&format!("Configuration loaded from: {}", path.display()));
        Ok(config)
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(
            Path::new(path).parent().unwrap_or(Path::new("."))
        )?;
        std::fs::write(path, contents)?;
        banner::success(&format!("Configuration saved to: {}", path));
        Ok(())
    }

    pub fn load_default() -> Self {
        let config_path = "config/cf-void.toml";
        Self::load(config_path).unwrap_or_else(|e| {
            banner::warning(&format!("Failed to load config: {}, using defaults", e));
            Self::default()
        })
    }

    pub fn validate(&self) -> Vec<String> {
        let mut warnings = vec![];
        
        if self.general.timeout < 5 {
            warnings.push("Timeout is very low, may cause incomplete scans".to_string());
        }
        
        if self.fuzzing.threads > 100 {
            warnings.push("Fuzzing threads > 100 may cause rate limiting".to_string());
        }
        
        if self.credentials.brute_threads > 50 {
            warnings.push("Brute force threads > 50 may lock accounts".to_string());
        }
        
        if self.payloads.default_lhost == "127.0.0.1" {
            warnings.push("Default LHOST is 127.0.0.1, update for remote connections".to_string());
        }
        
        warnings
    }
}
