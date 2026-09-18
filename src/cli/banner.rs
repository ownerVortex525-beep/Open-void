// CF-VOID Banner Module
// Author: CYBER-FORCE

use colored::*;
use std::io::Write;
use std::time::Duration;

// ═══════════════════ COLOR PALETTE ═══════════════════
pub const GOLD: (u8, u8, u8) = (218, 165, 32);
pub const CRIMSON: (u8, u8, u8) = (220, 50, 47);
pub const TEAL: (u8, u8, u8) = (38, 166, 154);
pub const AZURE: (u8, u8, u8) = (42, 157, 223);
pub const DIM: (u8, u8, u8) = (128, 128, 128);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const GREEN: (u8, u8, u8) = (0, 200, 83);
pub const YELLOW: (u8, u8, u8) = (255, 193, 7);
pub const ORANGE: (u8, u8, u8) = (255, 165, 0);

pub fn tc(s: &str, (r, g, b): (u8, u8, u8)) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, s)
}

pub fn tc_string(s: String, (r, g, b): (u8, u8, u8)) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, s)
}

fn bold_tc(s: &str, c: (u8, u8, u8)) -> String {
    format!("\x1B[1m\x1B[38;2;{};{};{}m{}\x1B[0m", c.0, c.1, c.2, s)
}

// ═══════════════════ SMALL SKULL BANNER (for CLI args mode) ═══════════════════
pub fn print_banner() {
    let g = GOLD;
    let cr = CRIMSON;
    let t = TEAL;
    let az = AZURE;

    println!();
    println!("{}───▄▀▀▀▄▄▄▄▄▄▄▀▀▀▄───{}", tc("▐", g), "\x1B[0m");
    println!("{}───█▒▒░░░░░░░░░▒▒█───{}", tc("▐", g), "\x1B[0m");
    println!("{}───█░░█░░░░░█░░█────{}", tc("▐", g), "\x1B[0m");
    println!("{}─▄▄──█░░░▀█▀░░░█──▄▄─{}", tc("▐", g), "\x1B[0m");
    println!("{}█░░█─▀▄░░░░░░░▄▀─█░░█{}", tc("▐", g), "\x1B[0m");
    println!("{}█▀░▀▀▀▀▄▄▄░▄▄▄▀▀▀▀░▀█{}", tc("▐", g), "\x1B[0m");
    println!();
    println!("{}  ╔═══════════════════════════════════════════════════════════╗{}", tc("▐", g), "\x1B[0m");
    let line1 = format!("  ║  {} :: {}                            ║", bold_tc("CYBER-FORCE", cr), tc("PENETRATION TOOLKIT", t));
    println!("{}{}", tc("▐", g), tc(&line1, g));
    let line2 = format!("  ║  {} :: {}                   ║", bold_tc("CF-VOID", g), tc("Offensive Security Platform", az));
    println!("{}{}", tc("▐", g), tc(&line2, g));
    let line3 = format!("  ║  {}                                    ║", tc("by IND 'CYBER-FORCE'", az));
    println!("{}{}", tc("▐", g), tc(&line3, g));
    println!("{}  ╚═══════════════════════════════════════════════════════════╝{}", tc("▐", g), "\x1B[0m");
    println!();
}

// ═══════════════════ BIG CF-VOID BANNER (portrait orientation) ═══════════════════
pub fn print_cfvoid_banner() {
    let cr = CRIMSON;
    let g = GOLD;
    let _t = TEAL;
    let az = AZURE;

    println!();

    // Visual header bar
    print!("\x1B[38;2;{};{};{}m  ▁▂▃▅▇  CF-VOID  ▇▆▅▃▂\x1B[0m\n", g.0, g.1, g.2);
    std::io::stdout().flush().ok();
    std::thread::sleep(Duration::from_millis(50));

    // Compact CF-VOID ASCII Art (from ascii-text-art.txt)
    let banner_lines = [
        "  ____ _____          __     _____ ___ ____  ",
        " / ___|  ___|         \\ \\   / / _ \\_ _|  _ \\ ",
        "| |   | |_     _____   \\ \\ / /| | | | || | | |",
        "| |___|  _|   |_____|   \\ V / | |_| | || |_| |",
        " \\____|_|                \\_/  \\___/___|____/ ",
    ];

    for line in banner_lines {
        println!("\x1B[38;2;{};{};{}m{}\x1B[0m", cr.0, cr.1, cr.2, line);
    }

    std::io::stdout().flush().ok();
    std::thread::sleep(Duration::from_millis(50));

    // Compact legal disclaimer (one line)
    println!();
    println!("\x1B[1m\x1B[38;2;{};{};{}m  ⚠ Authorized penetration testing only. Use with written permission.\x1B[0m", g.0, g.1, g.2);
    println!("\x1B[38;2;{};{};{}m  Built by IND 'CYBER-FORCE' :: Offensive Security Platform\x1B[0m", az.0, az.1, az.2);
    println!("\x1B[0m");
}

// ═══════════════════ SCAN HEADER ═══════════════════
pub fn print_scan_header(target: &str, modules: &[&str]) {
    let g = GOLD;
    let t = TEAL;
    let _az = AZURE;

    println!();
    println!("{}  ▐━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▌{}", tc("▐", g), "\x1B[0m");
    println!("{}  ▐━▌ SCAN CONFIGURATION", bold_tc("▐━▌", g));
    println!("{}  ▐━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▌{}", tc("▐", g), "\x1B[0m");
    println!(" {}  ➥ Target    :: {}", tc("▐", g), bold_tc(target, WHITE));
    println!(" {}  ➦ Modules   :: {}", tc("▐", g), tc(&modules.join(", ").to_string(), t));
    println!(" {}  ➤ Threads   :: {}", tc("▐", g), tc("10", t));
    println!(" {}  ➥ Timeout   :: {}", tc("▐", g), tc("30s", t));
    println!("{}  ▐─────────────────────────────────────────────────────────────▌{}", tc("▐", g), "\x1B[0m");
    println!();
}

pub fn print_section_header(title: &str) {
    let g = GOLD;
    println!();
    println!("{}  ▁▂▃▅▇  {}  ▇▆▅▃▂", tc("▐", g), bold_tc(title, GOLD));
    println!("{}  ──────────────────────────────────────────", tc("▐", g));
}

// Theme support
static THEME: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0); // 0=full, 1=minimal

pub fn toggle_theme() {
    let current = THEME.load(std::sync::atomic::Ordering::SeqCst);
    THEME.store(if current == 0 { 1 } else { 0 }, std::sync::atomic::Ordering::SeqCst);
    let name = if THEME.load(std::sync::atomic::Ordering::SeqCst) == 0 { "Full" } else { "Minimal" };
    println!("{}  Theme: {}", tc("▐", GOLD), name);
}

// ═══════════════════ ATTACK LOG ═══════════════════
pub fn print_attack_log(module: &str, target: &str, status: &str, detail: &str) {
    let g = GOLD;
    let t = TEAL;
    let cr = CRIMSON;

    let status_color = match status {
        "SCANNING" => AZURE,
        "EXPLOITING" => cr,
        "SUCCESS" => GREEN,
        "FAILED" => cr,
        "INFO" => (180, 180, 180),
        "START" => YELLOW,
        _ => g,
    };

    println!("  {} [{}] {} :: {} :: {}",
        tc("▐", g),
        bold_tc(status, status_color),
        bold_tc(module, WHITE),
        tc(target, t),
        tc(detail, (180, 180, 180))
    );
}

// ═══════════════════ LIVE FINDING ═══════════════════
pub fn print_finding_live(severity: &str, title: &str, url: &str, evidence: &str) {
    let g = GOLD;
    let sev_color = match severity {
        "CRITICAL" => CRIMSON,
        "HIGH" => ORANGE,
        "MEDIUM" => YELLOW,
        "LOW" => AZURE,
        _ => (180, 180, 180),
    };

    println!();
    println!("  {} [{}] {}",
        tc("▐", g),
        bold_tc(severity, sev_color),
        bold_tc(title, WHITE)
    );
    println!("    ➥ Target   :: {}", tc(url, TEAL));
    println!("    ➦ Evidence :: {}", tc(evidence, (255, 220, 100)));
    println!("  {}", tc_string("─".repeat(56), DIM));
}

// ═══════════════════ STATUS INDICATORS ═══════════════════
pub fn success(msg: &str) {
    println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("SUCCESS", GREEN), msg.white());
}

pub fn error(msg: &str) {
    println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("ERROR", CRIMSON), msg.white());
}

pub fn info(msg: &str) {
    println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("INFO", AZURE), msg.white());
}

pub fn warning(msg: &str) {
    println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("WARNING", YELLOW), msg.bright_yellow());
}

pub fn finding(severity: &str, title: &str, url: &str, evidence: &str) {
    let sev_color = match severity {
        "CRITICAL" => CRIMSON,
        "HIGH" => ORANGE,
        "MEDIUM" => YELLOW,
        "LOW" => AZURE,
        _ => (180, 180, 180),
    };

    println!();
    println!("  {} [{}] {}",
        tc("▐", GOLD),
        bold_tc(severity, sev_color),
        bold_tc(title, WHITE)
    );
    println!("    ➥ Target   :: {}", url.bright_cyan());
    println!("    ➦ Evidence :: {}", evidence.bright_yellow());
    println!("  {}", tc_string("─".repeat(56), DIM));
}

pub fn exploit_result(s: bool, msg: &str) {
    if s {
        println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("EXPLOIT SUCCESS", GREEN), msg.white());
    } else {
        println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("EXPLOIT FAILED", CRIMSON), msg.white());
    }
}

pub fn shell_obtained(shell_type: &str, connection: &str) {
    let t = TEAL;
    println!();
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!("  {} {}", bold_tc("▐ [SESSION]", t), bold_tc(shell_type, GREEN));
    println!("  {} Connection :: {}", tc("➥", t), connection.bright_cyan());
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!();
}

pub fn print_complete(duration: &str, findings: usize, requests: usize) {
    let _g = GOLD;
    let t = TEAL;

    println!();
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!("  {} {}", bold_tc("▐ [COMPLETE]", t), bold_tc("CF-VOID :: Scan Complete", WHITE));
    println!("  {} {}", tc("➥", t), tc("IND 'CYBER-FORCE' :: Offensive Security Platform", AZURE));
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!("  ➤ Duration  :: {}", tc(duration, TEAL));
    println!("  ➥ Findings  :: {}", bold_tc(&findings.to_string(), YELLOW));
    println!("  ➦ Requests  :: {}", tc(&requests.to_string(), TEAL));
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!();
}

pub fn print_progress(current: usize, total: usize, _msg: &str) {
    let pct = if total > 0 { (current as f64 / total as f64 * 100.0) as usize } else { 0 };
    let bar_width = 20;
    let filled = if total > 0 { (current as f64 / total as f64 * bar_width as f64) as usize } else { 0 };
    let bar = format!("{}{}", "█".repeat(filled), "▱".repeat(bar_width - filled));
    print!("\r  {} {}% {} ({}/{})",
        tc("▐", TEAL),
        bold_tc(&pct.to_string(), YELLOW),
        bar.white(),
        current,
        total
    );
    std::io::stdout().flush().unwrap_or(());
}

pub fn print_loading(msg: &str) {
    println!("  {} [{}] {}", tc("▐", GOLD), bold_tc("LOADING", AZURE), msg.white());
}

pub fn print_scanning(module: &str, target: &str) {
    println!("  {} [{}] {} :: {}",
        tc("▐", GOLD),
        bold_tc("SCANNING", AZURE),
        bold_tc(module, WHITE),
        target.bright_cyan()
    );
}

pub fn print_exploiting(module: &str, target: &str) {
    println!("  {} [{}] {} :: {}",
        tc("▐", GOLD),
        bold_tc("EXPLOITING", CRIMSON),
        bold_tc(module, WHITE),
        target.bright_cyan()
    );
}

pub fn print_vuln_found(module: &str, count: usize) {
    if count > 0 {
        println!("  {} [{}] {} :: {} vulnerabilities found",
            tc("▐", GOLD),
            bold_tc("VULN", CRIMSON),
            bold_tc(module, WHITE),
            bold_tc(&count.to_string(), YELLOW)
        );
    } else {
        println!("  {} [{}] {} :: No vulnerabilities found",
            tc("▐", GOLD),
            bold_tc("SAFE", GREEN),
            bold_tc(module, WHITE)
        );
    }
}

pub fn print_request_made(url: &str, status: u16) {
    let status_color = match status {
        200..=299 => GREEN,
        300..=399 => YELLOW,
        400..=499 => CRIMSON,
        _ => (180, 180, 180),
    };
    println!("    {} [REQUEST] {} -> {}",
        tc("▐", GOLD),
        url.bright_cyan(),
        bold_tc(&status.to_string(), status_color)
    );
}

pub fn print_phase_banner(module: &str, desc: &str) {
    let az = AZURE;
    println!();
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", az));
    println!("  {} :: {}", bold_tc(module, az), desc.white());
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", az));
    println!();
}

pub fn print_worker_start(id: usize, url: &str) {
    println!("  {} Worker {} :: {}", tc("▐", GOLD), bold_tc(&id.to_string(), AZURE), url.bright_white());
}

pub fn print_worker_done(id: usize, findings: usize) {
    println!("  {} Worker {} :: {} findings",
        tc("▐", GOLD),
        bold_tc(&id.to_string(), AZURE),
        bold_tc(&findings.to_string(), YELLOW)
    );
}

pub fn print_worker_error(id: usize, error: &str) {
    println!("  {} Worker {} :: {}",
        tc("▐", GOLD),
        bold_tc(&id.to_string(), AZURE),
        error.bright_red()
    );
}

pub fn print_watermark() {
    let g = GOLD;
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", g));
    println!("  {} {}", tc("▐", g), tc("IND 'CYBER-FORCE' :: Offensive Security Platform", AZURE));
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", g));
}

pub fn print_stats() {
    let _g = GOLD;
    let cr = CRIMSON;
    let _t = TEAL;

    println!();
    println!("  {}", tc("▐═══✦═══════════════════════════════════════════════════════✦═══▌", cr));
    println!("  {} {}", tc("▐", cr), bold_tc("IND 'CYBER-FORCE' :: CF-VOID STATS", cr));
    println!("  {}", tc("▐═══✦═══════════════════════════════════════════════════════✦═══▌", cr));
    println!("  ➤ Scanners:    40+ Web Vuln + 7 Network + 6 Credential + 4 OSINT");
    println!("  ➥ Exploits:    12 Modules (SQLi, XSS, CMDi, LFI, SSRF, XXE, IDOR, SSTI, JWT, GraphQL)");
    println!("  ➦ Payloads:    50+ Generators (Windows, Linux, Android, Cloud, Fileless)");
    println!("  ➤ Hash Types:  16 Algorithms (MD5, SHA1-512, SHA3, bcrypt, NTLM, RIPEMD)");
    println!("  ➥ Platforms:   Windows 7-11, Linux x64/ARM, macOS x64/ARM, Android 8-14");
    println!();
}

pub fn print_report_saved(path: &str) {
    let t = TEAL;

    println!();
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!("  {} {}", bold_tc("▐ [REPORT]", t), bold_tc("Attack logs & report saved", GREEN));
    println!("  {} Location :: {}", tc("➥", t), bold_tc(path, WHITE));
    println!("  {}", tc("▐═══════════════════════════════════════════════════════════════▌", t));
    println!();
}

pub fn print_module_list() {
    let g = GOLD;
    let az = AZURE;
    let _t = TEAL;

    println!();
    println!("{}───▄▀▀▀▄▄▄▄▄▄▄▀▀▀▄───{}", tc("▐", g), "\x1B[0m");
    println!("{}───█▒▒░░░░░░░░░▒▒█───{}", tc("▐", g), "\x1B[0m");
    println!("{}───█░░█░░░░░█░░█────{}", tc("▐", g), "\x1B[0m");
    println!("{}─▄▄──█░░░▀█▀░░░█──▄▄─{}", tc("▐", g), "\x1B[0m");
    println!("{}█░░█─▀▄░░░░░░░▄▀─█░░█{}", tc("▐", g), "\x1B[0m");
    println!("{}█▀░▀▀▀▀▄▄▄░▄▄▄▀▀▀▀░▀█{}", tc("▐", g), "\x1B[0m");
    println!();
    println!("{}  ▐━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▌{}", tc("▐", g), "\x1B[0m");
    println!("  {} CF-VOID :: MODULE REFERENCE", bold_tc("▐━━▌", g));
    println!("  {} IND 'CYBER-FORCE' :: Offensive Security Platform", tc("  ➥", az));
    println!("{}  ▐━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━▌{}", tc("▐", g), "\x1B[0m");
    println!();

    println!("  {} WEB SCANNERS", bold_tc("[1]", az));
    println!("    ➤ --sqli            SQL Injection Scanner");
    println!("    ➥ --blind-sqli      Blind SQL Injection Scanner");
    println!("    ➦ --xss             Cross-Site Scripting Scanner");
    println!("    ➤ --lfi             Local File Inclusion Scanner");
    println!("    ➥ --path-traversal  Path Traversal Scanner");
    println!("    ➦ --ssti            Server-Side Template Injection Scanner");
    println!("    ➤ --ssrf            Server-Side Request Forgery Scanner");
    println!("    ➥ --xxe             XML External Entity Scanner");
    println!("    ➦ --idor            Insecure Direct Object Reference Scanner");
    println!("    ➤ --cmdi            Command Injection Scanner");
    println!("    ➥ --cors            CORS Misconfiguration Scanner");
    println!("    ➦ --redirect        Open Redirect Scanner");
    println!("    ➤ --clickjack       Clickjacking Scanner");
    println!("    ➥ --web-all         Run ALL web vulnerability scanners");
    println!();

    println!("  {} FUZZING MODULES", bold_tc("[2]", az));
    println!("    ➤ --fuzz            Directory & parameter fuzzing");
    println!("    ➥ --fuzz-extensions  Custom file extensions");
    println!("    ➦ --js-crawl        JavaScript endpoint discovery");
    println!();

    println!("  {} NETWORK MODULES", bold_tc("[3]", az));
    println!("    ➤ --ports quick     Quick port scan (top 100)");
    println!("    ➥ --ports common    Common port scan (top 1000)");
    println!("    ➦ --ports full      Full port scan (1-65535)");
    println!("    ➤ --ports syn       SYN scan (requires root)");
    println!("    ➥ --banner-grab     Service banner grabbing");
    println!("    ➦ --os-detect       OS fingerprint");
    println!("    ➤ --subnet-scan     CIDR network scanning");
    println!();

    println!("  {} CREDENTIAL MODULES", bold_tc("[4]", az));
    println!("    ➤ --brute-ssh       Brute force SSH");
    println!("    ➥ --brute-http      Brute force HTTP");
    println!("    ➦ --brute-ftp       Brute force FTP");
    println!("    ➤ --default-creds   Test default credentials");
    println!("    ➥ --creds-all       Run ALL credential modules");
    println!();

    println!("  {} EXPLOIT MODULES", bold_tc("[5]", az));
    println!("    ➤ --exploit-sqli    Exploit SQL Injection");
    println!("    ➥ --exploit-xss     Exploit Cross-Site Scripting");
    println!("    ➦ --exploit-cmdi    Exploit Command Injection");
    println!("    ➤ --exploit-lfi     Exploit Local File Inclusion");
    println!("    ➥ --exploit-ssrf    Exploit Server-Side Request Forgery");
    println!("    ➦ --exploit-xxe     Exploit XML External Entity");
    println!("    ➤ --exploit-idor    Exploit IDOR");
    println!("    ➥ --exploit-ssti    Exploit Server-Side Template Injection");
    println!("    ➦ --exploit-jwt     Exploit JWT Vulnerabilities");
    println!("    ➤ --exploit-all     Run ALL exploit modules");
    println!();

    println!("  {} PAYLOAD MODULES", bold_tc("[6]", az));
    println!("    ➤ --reverse-shell   Generate reverse shell");
    println!("    ➥ --bind-shell      Generate bind shell");
    println!("    ➦ --web-shell       Generate web shell");
    println!("    ➤ --apk-payload     Generate APK payload (Android)");
    println!("    ➥ --exe-payload     Generate EXE payload (Windows)");
    println!("    ➦ --ps-variant      PowerShell payload variant");
    println!("    ➤ --windows-payload Windows-specific payloads");
    println!("    ➥ --linux-payload   Linux-specific payloads");
    println!("    ➦ --macos-payload   macOS-specific payloads");
    println!("    ➤ --cloud-payload   Cloud-specific payloads");
    println!("    ➥ --fileless        Generate fileless payloads");
    println!();

    println!("  {} PASSWORD MODULES", bold_tc("[7]", az));
    println!("    ➤ --crack-hash      Crack password hash");
    println!("    ➥ --hash-type       Hash type (16 algorithms)");
    println!("    ➦ --wordlist        Wordlist for cracking");
    println!("    ➤ --brute-force     Brute force attack");
    println!();

    println!("  {} POST-EXPLOITATION", bold_tc("[8]", az));
    println!("    ➤ --persist          Establish persistence");
    println!("    ➥ --privesc          Privilege escalation");
    println!("    ➦ --lateral          Lateral movement");
    println!("    ➤ --exfil            Data exfiltration");
    println!("    ➥ --cleanup          Cleanup traces");
    println!("    ➦ --post-all         Run ALL post-exploit");
    println!();

    println!("  {} OPTIONS", bold_tc("[>]", az));
    println!("    ➤ -u, --url        Target URL");
    println!("    ➥ -t, --target     Target IP or CIDR");
    println!("    ➦ -r, --report     Save attack logs & report to file");
    println!("    ➤ -j, --threads    Thread count (default: 10)");
    println!("    ➥ -v, --verbose    Verbose output");
    println!("    ➦ --ai            AI-powered attack");
    println!("    ➤ --ai-provider   AI provider (openai/claude/gemini/groq/ollama/custom)");
    println!("    ➥ --tui           Interactive TUI");
    println!("    ➦ --listen        Start session listener");
    println!("    ➤ --sessions      List all sessions");
    println!();
}
