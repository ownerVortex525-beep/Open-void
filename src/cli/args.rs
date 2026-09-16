use clap::Parser;

#[derive(Parser, Debug, Clone, Default)]
#[command(name = "cf-void")]
#[command(version = "1.0")]
#[command(about = "CF-VOID — Offensive Security Platform by IND CYBER-FORCE")]
#[command(long_about = "CF-VOID — Complete Offensive Security Platform.\nBuilt by IND 'CYBER-FORCE' for ethical hacking and security research.")]
pub struct CliArgs {
    // ═══════════════════ TARGET ═══════════════════
    #[arg(short, long, help = "Target URL")]
    pub url: Option<String>,

    #[arg(short, long, help = "Target IP or CIDR")]
    pub target: Option<String>,

    // ═══════════════════ WEB ATTACKS ═══════════════════
    #[arg(short = 'S', long, help = "SQL Injection")]
    pub sqli: bool,

    #[arg(long, help = "Blind SQL Injection (time-based)")]
    pub blind_sqli: bool,

    #[arg(short = 'Z', long, help = "Cross-Site Scripting")]
    pub xss: bool,

    #[arg(short = 'L', long, help = "Local File Inclusion")]
    pub lfi: bool,

    #[arg(long, help = "Path Traversal")]
    pub path_traversal: bool,

    #[arg(long, help = "Server-Side Template Injection")]
    pub ssti: bool,

    #[arg(long, help = "Server-Side Request Forgery")]
    pub ssrf: bool,

    #[arg(long, help = "XML External Entity")]
    pub xxe: bool,

    #[arg(long, help = "Insecure Direct Object Reference")]
    pub idor: bool,

    #[arg(long, help = "Command Injection")]
    pub cmdi: bool,

    #[arg(long, help = "CORS Misconfiguration")]
    pub cors: bool,

    #[arg(long, help = "Open Redirect")]
    pub redirect: bool,

    #[arg(long, help = "Clickjacking")]
    pub clickjack: bool,

    #[arg(long, help = "WebSocket Fuzzing")]
    pub websocket: bool,

    #[arg(long, help = "Session Hijack")]
    pub session_hijack: bool,

    #[arg(long, help = "Database Fingerprint")]
    pub db_fingerprint: bool,

    #[arg(long, help = "Run ALL web scans")]
    pub web_all: bool,

    // ═══════════════════ EXPLOITS ═══════════════════
    #[arg(long, help = "Exploit SQL Injection")]
    pub exploit_sqli: bool,

    #[arg(long, help = "Exploit Cross-Site Scripting")]
    pub exploit_xss: bool,

    #[arg(long, help = "Exploit Command Injection")]
    pub exploit_cmdi: bool,

    #[arg(long, help = "Exploit Local File Inclusion")]
    pub exploit_lfi: bool,

    #[arg(long, help = "Exploit Server-Side Request Forgery")]
    pub exploit_ssrf: bool,

    #[arg(long, help = "Exploit XML External Entity")]
    pub exploit_xxe: bool,

    #[arg(long, help = "Exploit Insecure Direct Object Reference")]
    pub exploit_idor: bool,

    #[arg(long, help = "Exploit Server-Side Template Injection")]
    pub exploit_ssti: bool,

    #[arg(long, help = "Run ALL exploit modules")]
    pub exploit_all: bool,

    // ═══════════════════ PAYLOADS ═══════════════════
    #[arg(short = 'R', long, help = "Generate reverse shell")]
    pub reverse_shell: bool,

    #[arg(long, help = "Generate bind shell")]
    pub bind_shell: bool,

    #[arg(long, help = "Generate web shell")]
    pub web_shell: bool,

    #[arg(long, help = "Generate custom payload")]
    pub payload_gen: bool,

    #[arg(long, help = "Payload type (reverse/bind/web)")]
    pub payload_type: Option<String>,

    #[arg(long, help = "Payload language (bash/python/php/perl/ruby)")]
    pub payload_lang: Option<String>,

    #[arg(long, help = "Local host for reverse shell")]
    pub lhost: Option<String>,

    #[arg(long, help = "Local port for reverse shell")]
    pub lport: Option<String>,

    #[arg(short = 'A', long, help = "Generate APK payload (Android)")]
    pub apk_payload: bool,

    #[arg(short = 'E', long, help = "Generate EXE payload (Windows)")]
    pub exe_payload: bool,

    #[arg(long, help = "PowerShell payload variant (iex/encoded/amsi/reflected/web)")]
    pub ps_variant: Option<String>,

    #[arg(long, help = "Architecture (x86/x64)")]
    pub arch: Option<String>,

    #[arg(long, help = "Windows-specific payloads")]
    pub windows_payload: bool,

    #[arg(long, help = "Linux-specific payloads")]
    pub linux_payload: bool,

    #[arg(long, help = "macOS-specific payloads")]
    pub macos_payload: bool,

    #[arg(long, help = "Cloud-specific payloads")]
    pub cloud_payload: bool,

    #[arg(long, help = "HTTP request smuggling payloads")]
    pub http_smuggle: bool,

    #[arg(long, help = "JWT vulnerabilities")]
    pub jwt_attack: bool,

    #[arg(long, help = "GraphQL vulnerabilities")]
    pub graphql_attack: bool,

    #[arg(long, help = "Deserialization vulnerabilities")]
    pub deserialization_attack: bool,

    #[arg(long, help = "Fileless payload generation")]
    pub fileless: bool,

    #[arg(long, help = "AMSIBypass technique (windows)")]
    pub amsi_bypass: bool,

    #[arg(long, help = "ETW bypass technique (windows)")]
    pub etw_bypass: bool,

    // ═══════════════════ FUZZING ═══════════════════
    #[arg(long, help = "Directory and parameter fuzzing")]
    pub fuzz: bool,

    #[arg(long, help = "Fuzz file extensions (php,txt,asp,aspx)")]
    pub fuzz_extensions: Option<String>,

    #[arg(long, help = "JavaScript endpoint discovery")]
    pub js_crawl: bool,

    #[arg(long, help = "Parameter discovery")]
    pub param_discovery: bool,

    #[arg(long, help = "Fuzz request delay (milliseconds)")]
    pub fuzz_delay: Option<u64>,

    // ═══════════════════ NETWORK SCANNER ═══════════════════
    #[arg(long, help = "Scan type (connect/syn/udp/ack/fin)")]
    pub scan_type: Option<String>,

    #[arg(long, help = "Banner grabbing")]
    pub banner_grab: bool,

    #[arg(long, help = "Subnet scanning (CIDR notation)")]
    pub subnet_scan: bool,

    #[arg(long, help = "Decoy scanning")]
    pub decoy_scan: bool,

    // ═══════════════════ PASSWORD CRACKING ═══════════════════
    #[arg(long, help = "Mask attack (charset + length)")]
    pub mask_attack: Option<String>,

    #[arg(long, help = "Hybrid attack (dictionary + rules)")]
    pub hybrid_attack: bool,

    #[arg(long, help = "Hash identification")]
    pub identify_hash: Option<String>,

    #[arg(long, help = "Generate rainbow table")]
    pub rainbow_table: bool,

    // ═══════════════════ SESSION MANAGEMENT ═══════════════════
    #[arg(long, help = "Interact with specific session")]
    pub session_id: Option<u32>,

    #[arg(short = 's', long, help = "List all sessions")]
    pub sessions: bool,

    #[arg(long, help = "Background current session")]
    pub background: bool,

    #[arg(long, help = "Session timeout in seconds")]
    pub session_timeout: Option<u64>,

    // ═══════════════════ POST-EXPLOITATION ═══════════════════
    #[arg(long, help = "Persistence method (registry, cron, systemd)")]
    pub persistence_method: Option<String>,

    #[arg(long, help = "Process injection technique")]
    pub process_injection: bool,

    #[arg(long, help = "Screenshot capture")]
    pub screenshot: bool,

    #[arg(long, help = "Keylogger")]
    pub keylogger: bool,

    #[arg(long, help = "File upload")]
    pub upload_file: Option<String>,

    #[arg(long, help = "File download")]
    pub download_file: Option<String>,

    // ═══════════════════ EVASION ═══════════════════
    #[arg(long, help = "User agent rotation")]
    pub ua_rotation: bool,

    #[arg(long, help = "IP rotation via proxy list")]
    pub ip_rotation: bool,

    #[arg(long, help = "Request timing obfuscation")]
    pub timing_obfuscation: bool,

    #[arg(long, help = "Protocol mimicry (http/dns/smb)")]
    pub protocol_mimicry: Option<String>,

    #[arg(long, help = "Anti-analysis tricks")]
    pub anti_analysis: bool,

    // ═══════════════════ CONFIGURATION ═══════════════════
    #[arg(short = 'c', long, help = "Config file path")]
    pub config: Option<String>,

    #[arg(long, help = "Save current config")]
    pub save_config: bool,

    // ═══════════════════ PASSWORDS ═══════════════════
    #[arg(long, help = "Crack password hash")]
    pub crack_hash: Option<String>,

    #[arg(long, help = "Hash type (md5/sha1/sha256/bcrypt)")]
    pub hash_type: Option<String>,

    #[arg(short = 'b', long, help = "Password brute force attack")]
    pub brute_force: bool,

    #[arg(short = 'n', long, help = "Start session listener")]
    pub listen: bool,

    // ═══════════════════ NETWORK ═══════════════════
    #[arg(long, help = "Port scan (quick/common/full)")]
    pub ports: Option<String>,

    #[arg(long, help = "Service enumeration")]
    pub enum_services: bool,

    #[arg(long, help = "OS fingerprint")]
    pub os_detect: bool,

    #[arg(long, help = "Network vuln scan")]
    pub vuln_scan: bool,

    #[arg(long, help = "Run ALL network scans")]
    pub network_all: bool,

    // ═══════════════════ CREDENTIALS ═══════════════════
    #[arg(long, help = "Brute force SSH")]
    pub brute_ssh: bool,

    #[arg(long, help = "Brute force HTTP")]
    pub brute_http: bool,

    #[arg(long, help = "Brute force FTP")]
    pub brute_ftp: bool,

    #[arg(long, help = "Brute force MySQL")]
    pub brute_mysql: bool,

    #[arg(long, help = "Brute force RDP")]
    pub brute_rdp: bool,

    #[arg(long, help = "Test default creds")]
    pub default_creds: bool,

    #[arg(long, help = "Run ALL creds attacks")]
    pub creds_all: bool,

    // ═══════════════════ DNS ═══════════════════
    #[arg(long, help = "DNS zone transfer")]
    pub zone_transfer: bool,

    #[arg(long, help = "Subdomain brute force")]
    pub subdomain_brute: bool,

    #[arg(long, help = "Enumerate DNS records")]
    pub dns_enum: bool,

    #[arg(long, help = "Run ALL DNS attacks")]
    pub dns_all: bool,

    // ═══════════════════ OSINT ═══════════════════
    #[arg(long, help = "Discover subdomains")]
    pub osint_subdomains: bool,

    #[arg(long, help = "Harvest emails")]
    pub osint_emails: bool,

    #[arg(long, help = "GitHub dorking")]
    pub osint_github: bool,

    #[arg(long, help = "Social media recon")]
    pub osint_social: bool,

    #[arg(long, help = "Run ALL OSINT")]
    pub osint_all: bool,

    // ═══════════════════ SOCIAL ENGINEERING ═══════════════════
    #[arg(long, help = "Generate phishing page (template: instagram, facebook, etc)")]
    pub phish_template: Option<String>,

    #[arg(long, help = "Generate phishing email template (birthday/love/offer/card/etc)")]
    pub phish_email: Option<String>,

    #[arg(long, short = 'L', help = "List available phishing templates")]
    pub list_templates: bool,

    #[arg(long, short = 'M', help = "List available phishing email templates")]
    pub list_emails: bool,

    // ═══════════════════ POST-EXPLOITATION ═══════════════════
    #[arg(long, help = "Check persistence")]
    pub check_persistence: bool,

    #[arg(long, help = "Establish persistence")]
    pub persist: bool,

    #[arg(long, help = "Privilege escalation check")]
    pub privesc: bool,

    #[arg(long, help = "Lateral movement")]
    pub lateral: bool,

    #[arg(long, help = "Data exfiltration")]
    pub exfil: bool,

    #[arg(long, help = "Cleanup traces")]
    pub cleanup: bool,

    #[arg(long, help = "Run ALL post-exploit")]
    pub post_all: bool,

    // ═══════════════════ AI ═══════════════════
    #[arg(long, help = "AI-powered attack on target")]
    pub ai: bool,

    #[arg(long, help = "AI provider (openai/claude/gemini/groq/ollama/custom)")]
    pub ai_provider: Option<String>,

    // ═══════════════════ IoT/SCADA ═══════════════════
    #[arg(long, help = "IoT SCADA scan")]
    pub iot_scan: bool,

    #[arg(long, help = "Cloud security audit")]
    pub cloud_audit: bool,

    #[arg(long, help = "Launch interactive TUI")]
    pub tui: bool,

    #[arg(long, help = "Launch web GUI server (local browser)")]
    pub web: bool,

    #[arg(long, default_value = "8080", help = "Port for web GUI server")]
    pub web_port: u16,

    #[arg(long, help = "Scrape free proxy list and test them")]
    pub proxy_scrape: bool,

    // ═══════════════════ OPTIONS ═══════════════════
    #[arg(short = 'j', long, default_value = "10", help = "Threads")]
    pub threads: usize,

    #[arg(short = 'x', long, default_value = "25", help = "Intensity (1-100)")]
    pub intensity: u8,

    #[arg(long, default_value = "30", help = "Timeout (sec)")]
    pub timeout: u64,

    #[arg(long, help = "Cookie")]
    pub cookie: Option<String>,

    #[arg(long, help = "Auth (user:pass)")]
    pub auth: Option<String>,

    #[arg(long, help = "Bearer token")]
    pub token: Option<String>,

    #[arg(long, help = "Header (Key:Value)")]
    pub header: Vec<String>,

    #[arg(short = 'p', long, help = "Proxy (http://127.0.0.1:8080)")]
    pub proxy: Option<String>,

    #[arg(long, help = "Userlist for brute")]
    pub userlist: Option<String>,

    #[arg(long, help = "Passlist for brute")]
    pub passlist: Option<String>,

    #[arg(short = 'w', long, help = "Wordlist")]
    pub wordlist: Option<String>,

    #[arg(short = 'o', long, help = "Output file")]
    pub output: Option<String>,

    #[arg(short = 'r', long, help = "Save attack logs & report to file")]
    pub report: Option<String>,

    #[arg(short = 'f', long, help = "Format (html/json/csv/markdown)")]
    pub format: Option<String>,

    #[arg(short = 'v', long, help = "Verbose")]
    pub verbose: bool,

    #[arg(short = 'q', long, help = "Quiet")]
    pub quiet: bool,

    #[arg(long, help = "WAF bypass")]
    pub waf_bypass: bool,

    #[arg(long, help = "Random user-agent")]
    pub random_ua: bool,

    #[arg(long, help = "Zero-day ML")]
    pub zeroday: bool,

    #[arg(long, help = "Skip SSL verify")]
    pub insecure: bool,

    #[arg(long, help = "Follow redirects")]
    pub follow_redirects: bool,

    #[arg(long, help = "List all modules")]
    pub list_modules: bool,

    #[arg(long, help = "Stealth mode")]
    pub stealth: bool,
}

impl CliArgs {
    pub fn has_target(&self) -> bool {
        self.url.is_some() || self.target.is_some()
    }

    pub fn primary_target(&self) -> Option<String> {
        self.url.clone().or_else(|| self.target.clone())
    }

    pub fn has_web_attacks(&self) -> bool {
        self.sqli || self.blind_sqli || self.xss || self.lfi || self.path_traversal ||
        self.ssti || self.ssrf || self.xxe || self.idor || self.cmdi || self.cors ||
        self.redirect || self.clickjack || self.websocket || self.session_hijack ||
        self.db_fingerprint || self.web_all
    }

    pub fn has_network_attacks(&self) -> bool {
        self.ports.is_some() || self.enum_services || self.os_detect ||
        self.vuln_scan || self.network_all
    }

    pub fn has_creds_attacks(&self) -> bool {
        self.brute_ssh || self.brute_http || self.brute_ftp ||
        self.brute_mysql || self.brute_rdp || self.default_creds ||
        self.creds_all
    }

    pub fn has_dns_attacks(&self) -> bool {
        self.zone_transfer || self.subdomain_brute || self.dns_enum || self.dns_all
    }

    pub fn has_osint(&self) -> bool {
        self.osint_subdomains || self.osint_emails || self.osint_github ||
        self.osint_social || self.osint_all
    }

    pub fn has_social_eng(&self) -> bool {
        self.phish_template.is_some() || self.list_templates || self.payload_gen
    }

    pub fn has_post_exploit(&self) -> bool {
        self.check_persistence || self.persist || self.privesc ||
        self.lateral || self.exfil || self.cleanup || self.post_all
    }

    pub fn has_payloads(&self) -> bool {
        self.reverse_shell || self.bind_shell || self.web_shell || self.payload_gen ||
        self.apk_payload || self.exe_payload || self.ps_variant.is_some() ||
        self.windows_payload || self.linux_payload || self.macos_payload ||
        self.cloud_payload || self.fileless
    }

    pub fn has_hash_or_crack(&self) -> bool {
        self.crack_hash.is_some() || self.brute_force || self.mask_attack.is_some() ||
        self.hybrid_attack || self.identify_hash.is_some()
    }

    pub fn has_fuzz(&self) -> bool {
        self.fuzz || self.js_crawl || self.param_discovery
    }

    pub fn has_exploits(&self) -> bool {
        self.exploit_sqli || self.exploit_xss || self.exploit_cmdi ||
        self.exploit_lfi || self.exploit_ssrf || self.exploit_xxe ||
        self.exploit_idor || self.exploit_ssti || self.exploit_all ||
        self.jwt_attack || self.graphql_attack || self.deserialization_attack ||
        self.http_smuggle
    }

    pub fn has_any_attack(&self) -> bool {
        self.has_web_attacks() || self.has_network_attacks() ||
        self.has_creds_attacks() || self.has_dns_attacks() ||
        self.has_osint() || self.has_social_eng() || self.has_post_exploit() ||
        self.has_exploits() || self.has_payloads() || self.has_hash_or_crack() ||
        self.has_fuzz() || self.listen || self.sessions ||
        self.iot_scan || self.cloud_audit ||
        self.ai
    }

    pub fn get_auth(&self) -> Option<(String, String)> {
        self.auth.as_ref().and_then(|a| {
            let parts: Vec<&str> = a.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
    }

    pub fn get_headers(&self) -> Vec<(String, String)> {
        self.header.iter().filter_map(|h| {
            let parts: Vec<&str> = h.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        }).collect()
    }

    pub fn active_modules(&self) -> Vec<&'static str> {
        let mut m = Vec::new();
        if self.sqli || self.web_all { m.push("sqli"); }
        if self.blind_sqli || self.web_all { m.push("blind-sqli"); }
        if self.xss || self.web_all { m.push("xss"); }
        if self.lfi || self.web_all { m.push("lfi"); }
        if self.path_traversal || self.web_all { m.push("path-traversal"); }
        if self.ssti || self.web_all { m.push("ssti"); }
        if self.ssrf || self.web_all { m.push("ssrf"); }
        if self.xxe || self.web_all { m.push("xxe"); }
        if self.idor || self.web_all { m.push("idor"); }
        if self.cmdi || self.web_all { m.push("cmdi"); }
        if self.cors || self.web_all { m.push("cors"); }
        if self.redirect || self.web_all { m.push("redirect"); }
        if self.clickjack || self.web_all { m.push("clickjack"); }
        if self.websocket || self.web_all { m.push("websocket"); }
        if self.session_hijack || self.web_all { m.push("session-hijack"); }
        if self.db_fingerprint || self.web_all { m.push("db-fingerprint"); }
        if self.ports.is_some() || self.network_all { m.push("ports"); }
        if self.enum_services || self.network_all { m.push("enum-svc"); }
        if self.os_detect || self.network_all { m.push("os-detect"); }
        if self.vuln_scan || self.network_all { m.push("vuln-scan"); }
        if self.brute_ssh || self.creds_all { m.push("brute-ssh"); }
        if self.brute_http || self.creds_all { m.push("brute-http"); }
        if self.brute_ftp || self.creds_all { m.push("brute-ftp"); }
        if self.brute_mysql || self.creds_all { m.push("brute-mysql"); }
        if self.brute_rdp || self.creds_all { m.push("brute-rdp"); }
        if self.default_creds || self.creds_all { m.push("def-creds"); }
        if self.zone_transfer || self.dns_all { m.push("zone-xfer"); }
        if self.subdomain_brute || self.dns_all { m.push("sub-brute"); }
        if self.dns_enum || self.dns_all { m.push("dns-enum"); }
        if self.osint_subdomains || self.osint_all { m.push("osint-sub"); }
        if self.osint_emails || self.osint_all { m.push("osint-email"); }
        if self.osint_github || self.osint_all { m.push("osint-gh"); }
        if self.osint_social || self.osint_all { m.push("osint-social"); }
        if self.phish_template.is_some() { m.push("phish"); }
        if self.payload_gen { m.push("payload-gen"); }
        if self.check_persistence || self.post_all { m.push("persist-check"); }
        if self.persist || self.post_all { m.push("persist"); }
        if self.privesc || self.post_all { m.push("privesc"); }
        if self.lateral || self.post_all { m.push("lateral"); }
        if self.exfil || self.post_all { m.push("exfil"); }
        if self.cleanup || self.post_all { m.push("cleanup"); }
        if self.iot_scan { m.push("iot"); }
        if self.cloud_audit { m.push("cloud"); }
        if self.zeroday { m.push("zeroday"); }
        if m.is_empty() { m.push("none"); }
        m
    }
}
