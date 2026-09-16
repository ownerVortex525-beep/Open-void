// CF-VOID Attack Terminal Module
// Author: CYBER-FORCE
// Live console: 1000-line scrollback, save to file,
// command/response streaming, severity-colored findings

use chrono::Local;
use std::fs;
use std::path::Path;

pub struct AttackTerminal {
    pub lines: Vec<TermLine>,
    pub scroll: usize,
    pub findings: Vec<TermFinding>,
    pub requests: u64,
    pub running: bool,
    pub progress: f32,
    pub current_module: String,
    pub target: String,
    pub start_time: std::time::Instant,
    pub saved: bool,
    pub max_scroll: usize,
}

pub struct TermLine {
    pub time: String,
    pub kind: LineKind,
    pub content: String,
    pub severity: Option<Severity>,
}

pub struct TermFinding {
    pub severity: Severity,
    pub module: String,
    pub detail: String,
    pub timestamp: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LineKind {
    Sent,       // ">" sent to target — CYAN
    Received,   // "<" received from target — DIM
    Finding,    // "[+]" or "[-]" finding result — severity colored
    Info,       // "[*]" informational — TEAL
    Error,      // "[!]" error — CRIMSON
    AiMsg,      // "[AI]" AI message — GREEN
    Progress,   // "[#]" progress — GOLD
    Command,    // "$" shell command sent to session — GOLD
    Response,   // "%" shell response from session — WHITE
    Debug,      // "@" debug — DIM
}

impl LineKind {
    pub fn prefix(&self) -> &'static str {
        match self {
            LineKind::Sent => "> ",
            LineKind::Received => "< ",
            LineKind::Finding => "[+]",
            LineKind::Info => "[*]",
            LineKind::Error => "[!]",
            LineKind::AiMsg => "[AI]",
            LineKind::Progress => "[#]",
            LineKind::Command => "$ ",
            LineKind::Response => "% ",
            LineKind::Debug => "@ ",
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            LineKind::Sent => (42, 157, 223),    // AZURE
            LineKind::Received => (128, 128, 128), // DIM
            LineKind::Finding => (255, 193, 7),  // YELLOW (default for info)
            LineKind::Info => (38, 166, 154),    // TEAL
            LineKind::Error => (220, 50, 47),    // CRIMSON
            LineKind::AiMsg => (0, 200, 83),     // GREEN
            LineKind::Progress => (218, 165, 32), // GOLD
            LineKind::Command => (255, 165, 0),  // ORANGE
            LineKind::Response => (255, 255, 255), // WHITE
            LineKind::Debug => (128, 128, 128),  // DIM
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Severity {
    Critical,  // CRIMSON
    High,      // ORANGE
    Medium,    // YELLOW
    Low,       // AZURE
    Info,      // DIM
    Success,   // GREEN
}

impl Severity {
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Severity::Critical => (220, 50, 47),    // CRIMSON
            Severity::High => (255, 165, 0),        // ORANGE
            Severity::Medium => (255, 193, 7),       // YELLOW
            Severity::Low => (42, 157, 223),        // AZURE
            Severity::Info => (128, 128, 128),       // DIM
            Severity::Success => (0, 200, 83),       // GREEN
        }
    }

    pub fn ratatui_color(&self) -> ratatui::style::Color {
        let (r, g, b) = self.color();
        ratatui::style::Color::Rgb(r, g, b)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
            Severity::Success => "SUCCESS",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            Severity::Critical => "[!]",
            Severity::High => "[#]",
            Severity::Medium => "[?]",
            Severity::Low => "[~]",
            Severity::Info => "[*]",
            Severity::Success => "[+]",
        }
    }
}

impl Default for AttackTerminal {
    fn default() -> Self {
        Self::new()
    }
}

impl AttackTerminal {
    pub fn new() -> Self {
        Self {
            lines: Vec::with_capacity(1000),
            scroll: 0,
            findings: Vec::new(),
            requests: 0,
            running: false,
            progress: 0.0,
            current_module: String::new(),
            target: String::new(),
            start_time: std::time::Instant::now(),
            saved: false,
            max_scroll: 1000,
        }
    }

    pub fn push(&mut self, kind: LineKind, content: &str, severity: Option<Severity>) {
        let line = TermLine {
            time: Local::now().format("%H:%M:%S").to_string(),
            kind,
            content: content.to_string(),
            severity,
        };
        self.lines.push(line);
        if self.lines.len() > self.max_scroll {
            self.lines.remove(0);
            if self.scroll > 0 {
                self.scroll -= 1;
            }
        }
    }

    pub fn add_finding(&mut self, severity: Severity, module: &str, detail: &str) {
        let finding = TermFinding {
            severity,
            module: module.to_string(),
            detail: detail.to_string(),
            timestamp: Local::now().format("%H:%M:%S").to_string(),
        };
        self.findings.push(finding);
        self.push(LineKind::Finding, &format!("{} {}", severity.prefix(), detail), Some(severity));
    }

    pub fn log_sent(&mut self, content: &str) {
        self.requests += 1;
        self.push(LineKind::Sent, content, None);
    }

    pub fn log_received(&mut self, content: &str) {
        self.push(LineKind::Received, content, None);
    }

    pub fn log_info(&mut self, content: &str) {
        self.push(LineKind::Info, content, None);
    }

    pub fn log_error(&mut self, content: &str) {
        self.push(LineKind::Error, content, None);
    }

    pub fn log_ai(&mut self, content: &str) {
        self.push(LineKind::AiMsg, content, None);
    }

    pub fn log_debug(&mut self, content: &str) {
        self.push(LineKind::Debug, content, None);
    }

    pub fn log_command(&mut self, content: &str) {
        self.push(LineKind::Command, content, None);
    }

    pub fn log_response(&mut self, content: &str) {
        self.push(LineKind::Response, content, None);
    }

    pub fn log_progress(&mut self, content: &str) {
        self.push(LineKind::Progress, content, None);
    }

    pub fn scroll_up(&mut self, lines: usize) {
        if self.scroll >= lines {
            self.scroll -= lines;
        } else {
            self.scroll = 0;
        }
    }

    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.lines.len().saturating_sub(1);
        self.scroll = (self.scroll + lines).min(max_scroll);
    }

    pub fn visible_lines(&self, height: usize) -> std::ops::Range<usize> {
        let end = self.lines.len().saturating_sub(self.scroll);
        let start = end.saturating_sub(height);
        start..end
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.findings.clear();
        self.requests = 0;
        self.progress = 0.0;
        self.scroll = 0;
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let content = self.format_all();
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }

    pub fn format_all(&self) -> String {
        let mut output = String::new();
        output.push_str("CF-VOID Attack Terminal Log\n");
        output.push_str(&format!("Target: {}\n", self.target));
        output.push_str(&format!("Module: {}\n", self.current_module));
        output.push_str(&format!("Started: {}\n", self.start_time.elapsed().as_secs()));
        output.push_str(&format!("Requests: {}\n", self.requests));
        output.push_str(&format!("Findings: {}\n", self.findings.len()));
        output.push_str(&format!("Timestamp: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));

        for line in &self.lines {
            let prefix = match line.kind {
                LineKind::Finding => {
                    if let Some(sev) = line.severity {
                        format!("[{}]", sev.as_str())
                    } else {
                        "[-]".to_string()
                    }
                }
                _ => line.kind.prefix().to_string(),
            };
            output.push_str(&format!("[{}] {} {}\n", line.time, prefix, line.content));
        }

        output.push_str(&format!("\n---\nFindings Summary:\n"));
        for f in &self.findings {
            output.push_str(&format!("  [{}] {} :: {}\n", f.severity.as_str(), f.module, f.detail));
        }

        output
    }

    pub fn set_running(&mut self, module: &str, target: &str) {
        self.running = true;
        self.current_module = module.to_string();
        self.target = target.to_string();
        self.start_time = std::time::Instant::now();
        self.log_info(&format!("Starting {} attack on {}", module, target));
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.progress = 0.0;
        self.log_info("Attack stopped by user");
    }
}
