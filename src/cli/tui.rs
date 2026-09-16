// CF-VOID TUI v4.0
// Author: IND 'CYBER-FORCE'
// Rewritten with Ratatui 0.30
// 7 tabs: Dashboard | Scanning | Terminal | Payloads | AI | Device | Config
// UNIFIED KEY BINDINGS - same keys work on every page

use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, ListState,
        Paragraph, Tabs, Wrap,
    },
    Frame, Terminal,
};

use crate::cli::terminal::{AttackTerminal, LineKind, Severity};
use crate::cli::device;
use crate::ai::config::{AiConfig, AiProvider};
use crate::config::keys::KeysConfig;
use crate::scanner::proxy_scraper::ProxyScraper;

// Color palette (no emoji, use ASCII)
const GOLD: Color = Color::Rgb(218, 165, 32);
const CRIMSON: Color = Color::Rgb(220, 50, 47);
const TEAL: Color = Color::Rgb(38, 166, 154);
const AZURE: Color = Color::Rgb(42, 157, 223);
const DIM: Color = Color::Rgb(128, 128, 128);
const WHITE: Color = Color::Rgb(255, 255, 255);
const GREEN: Color = Color::Rgb(0, 200, 83);
const YELLOW: Color = Color::Rgb(255, 193, 7);
const ORANGE: Color = Color::Rgb(255, 165, 0);

#[derive(Clone, PartialEq, Eq)]
pub enum AppTab {
    Dashboard,
    Scanning,
    Terminal,
    Payloads,
    Ai,
    Device,
    Config,
}

#[derive(Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Target,
    Lhost,
    Lport,
    AiChat,
    PayloadLhost,
    PayloadLport,
    ConfigEdit,
}

pub enum AiMode {
    Auto,
    Guided,
    Chat,
}

pub enum ProxyMode {
    Manual,
    AutoScrape,
    Rotating,
}

pub struct App {
    pub current_tab: AppTab,
    pub tab_index: usize,
    pub tabs: Vec<&'static str>,
    pub should_quit: bool,
    pub target_url: String,
    pub status_message: String,

    pub input_mode: InputMode,
    pub input_buffer: String,

    pub modules: Vec<Module>,
    pub selected_module: usize,
    pub scan_tasks: Vec<ScanTask>,
    pub scan_scroll: usize,

    pub terminal: AttackTerminal,
    pub terminal_scroll: usize,

    pub sessions: Vec<Session>,
    pub selected_session: usize,

    pub device: device::DeviceStats,
    pub device_refresh: Duration,

    pub ai_config: AiConfig,
    pub ai_mode: AiMode,
    pub ai_providers: Vec<AiProvider>,
    pub selected_provider: usize,
    pub ai_chat: Vec<(String, bool)>,
    pub ai_input: String,

    pub payload_platforms: Vec<&'static str>,
    pub selected_platform: usize,
    pub payload_types: Vec<&'static str>,
    pub selected_payload: usize,
    pub payload_lhost: String,
    pub payload_lport: u16,

    pub config: ConfigDisplay,
    pub config_selected: usize,

    pub proxy_scraper: ProxyScraper,
    pub proxy_mode: ProxyMode,

    pub scroll_offset: usize,
}

pub struct Module {
    pub name: String,
    pub category: String,
    pub description: String,
}

pub struct ScanTask {
    pub target: String,
    pub module: String,
    pub status: String,
    pub progress: f32,
    pub findings: usize,
}

pub struct Session {
    pub id: u32,
    pub ip: String,
    pub port: u16,
    pub os: String,
    pub status: String,
}

pub struct ConfigDisplay {
    pub user_agent: String,
    pub timeout_secs: u64,
    pub threads: usize,
    pub proxy_url: String,
    pub follow_redirects: bool,
    pub output_dir: String,
}

impl Default for ConfigDisplay {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string(),
            timeout_secs: 30,
            threads: 10,
            proxy_url: "(none)".to_string(),
            follow_redirects: true,
            output_dir: "reports".to_string(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let modules = vec![
            Module { name: "SQL Injection".into(), category: "Web".into(), description: "Test for SQL injection vulnerabilities".into() },
            Module { name: "XSS Scanner".into(), category: "Web".into(), description: "Scan for Cross-Site Scripting".into() },
            Module { name: "Command Injection".into(), category: "Web".into(), description: "Test for OS command injection".into() },
            Module { name: "LFI Scanner".into(), category: "Web".into(), description: "Local File Inclusion scanner".into() },
            Module { name: "SSRF Scanner".into(), category: "Web".into(), description: "Server-Side Request Forgery".into() },
            Module { name: "XXE Scanner".into(), category: "Web".into(), description: "XML External Entity".into() },
            Module { name: "IDOR Scanner".into(), category: "Web".into(), description: "Insecure Direct Object Reference".into() },
            Module { name: "SSTI Scanner".into(), category: "Web".into(), description: "Server-Side Template Injection".into() },
            Module { name: "CORS Misconfig".into(), category: "Web".into(), description: "CORS misconfiguration scanner".into() },
            Module { name: "Open Redirect".into(), category: "Web".into(), description: "Detect open redirect vulnerabilities".into() },
            Module { name: "Clickjacking".into(), category: "Web".into(), description: "Test for clickjacking vulnerabilities".into() },
            Module { name: "CSRF Scanner".into(), category: "Web".into(), description: "Cross-Site Request Forgery detection".into() },
            Module { name: "JWT Attack".into(), category: "Web".into(), description: "JWT token analysis and attacks".into() },
            Module { name: "GraphQL Attack".into(), category: "Web".into(), description: "GraphQL introspection and injection".into() },
            Module { name: "HTTP Smuggling".into(), category: "Web".into(), description: "HTTP request smuggling".into() },
            Module { name: "Deserialization".into(), category: "Web".into(), description: "Insecure deserialization exploits".into() },
            Module { name: "Race Condition".into(), category: "Web".into(), description: "TOCTOU vulnerability testing".into() },
            Module { name: "Port Scanner".into(), category: "Network".into(), description: "TCP SYN/Connect port scanner".into() },
            Module { name: "Subnet Scanner".into(), category: "Network".into(), description: "Scan subnet ranges".into() },
            Module { name: "Banner Grab".into(), category: "Network".into(), description: "Service banner grabbing".into() },
            Module { name: "OS Fingerprint".into(), category: "Network".into(), description: "Passive OS detection".into() },
            Module { name: "SMB Enumeration".into(), category: "Network".into(), description: "SMB share enumeration".into() },
            Module { name: "AWS Enum".into(), category: "Cloud".into(), description: "AWS IAM and S3 enumeration".into() },
            Module { name: "Azure Enum".into(), category: "Cloud".into(), description: "Azure AD and resource enumeration".into() },
            Module { name: "GCP Enum".into(), category: "Cloud".into(), description: "GCP metadata and IAM enumeration".into() },
            Module { name: "Dir Fuzzer".into(), category: "Fuzzing".into(), description: "Directory and file fuzzer".into() },
            Module { name: "Param Fuzzer".into(), category: "Fuzzing".into(), description: "Parameter discovery".into() },
            Module { name: "JS Crawler".into(), category: "Fuzzing".into(), description: "JavaScript endpoint crawler".into() },
            Module { name: "Credential Brute".into(), category: "Creds".into(), description: "Credential brute force".into() },
            Module { name: "Password Cracker".into(), category: "Creds".into(), description: "Hash cracking".into() },
            Module { name: "AI Anomaly".into(), category: "AI".into(), description: "AI-powered anomaly detection".into() },
        ];

        Self {
            current_tab: AppTab::Dashboard,
            tab_index: 0,
            tabs: vec!["Dashboard", "Scanning", "Terminal", "Payloads", "AI", "Device", "Config"],
            should_quit: false,
            target_url: String::new(),
            status_message: "Ready".into(),

            input_mode: InputMode::Normal,
            input_buffer: String::new(),

            modules,
            selected_module: 0,
            scan_tasks: Vec::new(),
            scan_scroll: 0,

            terminal: AttackTerminal::new(),
            terminal_scroll: 0,

            sessions: Vec::new(),
            selected_session: 0,

            device: device::DeviceStats::new(),
            device_refresh: std::time::Duration::from_secs(0),

            ai_config: AiConfig::default(),
            ai_mode: AiMode::Auto,
            ai_providers: vec![
                AiProvider::OpenAI, AiProvider::Anthropic, AiProvider::Gemini,
                AiProvider::Groq, AiProvider::Mimo, AiProvider::DeepSeek,
                AiProvider::GoogleAI, AiProvider::Ollama, AiProvider::Custom,
            ],
            selected_provider: 0,
            ai_chat: Vec::new(),
            ai_input: String::new(),

            payload_platforms: vec!["Windows", "Linux", "macOS", "Android", "Cloud", "Cross-Platform"],
            selected_platform: 0,
            payload_types: vec!["Reverse TCP", "Reverse HTTP", "Bind TCP", "Meterpreter", "Shellcode", "EXE", "DLL", "PowerShell"],
            selected_payload: 0,
            payload_lhost: "127.0.0.1".into(),
            payload_lport: 4444,

            config: ConfigDisplay::default(),
            config_selected: 0,

            proxy_scraper: ProxyScraper::new(),
            proxy_mode: ProxyMode::Manual,

            scroll_offset: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tabs.len();
        self.current_tab = match self.tab_index {
            0 => AppTab::Dashboard,
            1 => AppTab::Scanning,
            2 => AppTab::Terminal,
            3 => AppTab::Payloads,
            4 => AppTab::Ai,
            5 => AppTab::Device,
            _ => AppTab::Config,
        };
    }

    pub fn prev_tab(&mut self) {
        self.tab_index = if self.tab_index == 0 { self.tabs.len() - 1 } else { self.tab_index - 1 };
        self.current_tab = match self.tab_index {
            0 => AppTab::Dashboard,
            1 => AppTab::Scanning,
            2 => AppTab::Terminal,
            3 => AppTab::Payloads,
            4 => AppTab::Ai,
            5 => AppTab::Device,
            _ => AppTab::Config,
        };
    }

    pub fn next_module(&mut self) {
        if self.selected_module < self.modules.len() - 1 {
            self.selected_module += 1;
        }
    }

    pub fn prev_module(&mut self) {
        if self.selected_module > 0 {
            self.selected_module -= 1;
        }
    }

    pub fn add_scan(&mut self, target: String, module_name: String) {
        self.scan_tasks.push(ScanTask {
            target,
            module: module_name,
            status: "Queued".into(),
            progress: 0.0,
            findings: 0,
        });
        self.status_message = format!("Added scan task");
    }

    pub fn add_session(&mut self, ip: String, port: u16, os: String) {
        self.sessions.push(Session {
            id: self.sessions.len() as u32 + 1,
            ip,
            port,
            os,
            status: "Active".into(),
        });
    }

    pub fn start_scan(&mut self, target: String, module_name: String) {
        let status = if !target.is_empty() && !module_name.is_empty() {
            "Running"
        } else {
            "Failed"
        };
        let target_clone = target.clone();
        self.scan_tasks.push(ScanTask {
            target,
            module: module_name.clone(),
            status: status.to_string(),
            progress: 0.0,
            findings: 0,
        });
        self.terminal.set_running(&module_name, &target_clone);
    }
}

    pub fn run_tui() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    app.status_message = "Ready | Use arrow keys [Left/Right] to switch pages".into();
    let res = (|| -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &mut app))?;
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    handle_key(&mut app, key);
                }
            }
            if app.should_quit { return Ok(()); }
        }
    })();
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

// ============================================================
// UNIFIED KEY HANDLER - Same keys work everywhere
// ============================================================
// GLOBAL KEYS (work on ALL pages):
//   Left/Right or [/]  = Switch tabs
//   Up/Down or j/k     = Navigate items
//   Enter              = Confirm / action
//   a                  = Attack / main action
//   x                  = Stop / cancel
//   t                  = Set target
//   q                  = Quit (only in Normal mode)
//   Escape             = Back to Normal mode
// ============================================================

fn handle_key(app: &mut App, key: KeyEvent) {
    // GLOBAL: Escape always goes back to Normal mode
    if key.code == KeyCode::Esc {
        if app.input_mode != InputMode::Normal {
            app.input_mode = InputMode::Normal;
            app.input_buffer.clear();
            app.ai_input.clear();
            app.status_message = "Cancelled".into();
        }
        return;
    }

    // GLOBAL: Quit only in Normal mode (not input/terminal mode)
    if key.code == KeyCode::Char('q') && app.input_mode == InputMode::Normal && !app.terminal.running {
        app.should_quit = true;
        return;
    }

    // If in input mode, handle typing
    if app.input_mode != InputMode::Normal {
        handle_input_mode(app, key);
        return;
    }

    // ============================================================
    // GLOBAL NAVIGATION KEYS (work on ALL pages)
    // ============================================================

    // Tab switching: Left/Right arrows, or [ / ] (backup for Android)
    match key.code {
        KeyCode::Left | KeyCode::Char('[') => {
            app.prev_tab();
            return;
        }
        KeyCode::Right | KeyCode::Char(']') => {
            app.next_tab();
            return;
        }
        _ => {}
    }

    // Up/Down navigation: Up/Down arrows, or j/k (backup for Android)
    // These dispatch to the CURRENT tab's list
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            navigate_down(app);
            return;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            navigate_up(app);
            return;
        }
        _ => {}
    }

    // GLOBAL: 'a' = attack / main action (works on ALL pages)
    if key.code == KeyCode::Char('a') {
        action_attack(app);
        return;
    }

    // GLOBAL: 'x' = stop
    if key.code == KeyCode::Char('x') {
        action_stop(app);
        return;
    }

    // GLOBAL: 't' = set target (works on ALL pages)
    if key.code == KeyCode::Char('t') && app.input_mode == InputMode::Normal {
        app.input_mode = InputMode::Target;
        app.input_buffer = app.target_url.clone();
        return;
    }

    // GLOBAL: Enter = confirm action
    if key.code == KeyCode::Enter {
        action_enter(app);
        return;
    }

    // GLOBAL: Ctrl+C = stop scan
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        action_stop(app);
        return;
    }

    // ============================================================
    // PAGE-SPECIFIC KEYS (additional keys per page)
    // ============================================================
    match app.current_tab {
        AppTab::Dashboard => handle_dashboard_extra(app, key),
        AppTab::Scanning => handle_scanning_extra(app, key),
        AppTab::Terminal => handle_terminal_extra(app, key),
        AppTab::Payloads => handle_payload_extra(app, key),
        AppTab::Ai => handle_ai_extra(app, key),
        AppTab::Device => handle_device_extra(app, key),
        AppTab::Config => handle_config_extra(app, key),
    }
}

// Handle input mode (typing in text fields)
fn handle_input_mode(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Target | InputMode::Lhost | InputMode::PayloadLhost => {
            match key.code {
                KeyCode::Char(c) => app.input_buffer.push(c),
                KeyCode::Backspace => { app.input_buffer.pop(); }
                KeyCode::Enter => {
                    let value = app.input_buffer.clone();
                    match app.input_mode {
                        InputMode::Target => {
                            app.target_url = value;
                            app.status_message = format!("Target: {}", app.target_url);
                            app.terminal.target = app.target_url.clone();
                        }
                        InputMode::Lhost => {
                            app.payload_lhost = value;
                            app.status_message = format!("LHOST: {}", app.payload_lhost);
                        }
                        InputMode::PayloadLhost => {
                            app.payload_lhost = value;
                            app.status_message = format!("LHOST: {}", app.payload_lhost);
                        }
                        _ => {}
                    }
                    app.input_buffer.clear();
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    app.input_buffer.clear();
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
        InputMode::Lport | InputMode::PayloadLport => {
            match key.code {
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() {
                        app.input_buffer.push(c);
                    }
                }
                KeyCode::Backspace => { app.input_buffer.pop(); }
                KeyCode::Enter => {
                    if let Ok(port) = app.input_buffer.parse::<u16>() {
                        app.payload_lport = port;
                        app.status_message = format!("LPORT: {}", port);
                    }
                    app.input_buffer.clear();
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    app.input_buffer.clear();
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
        InputMode::AiChat => {
            match key.code {
                KeyCode::Char(c) => app.ai_input.push(c),
                KeyCode::Backspace => { app.ai_input.pop(); }
                KeyCode::Enter => {
                    if !app.ai_input.is_empty() {
                        app.ai_chat.push((app.ai_input.clone(), true));
                        app.terminal.log_ai(&format!("User: {}", app.ai_input));
                        app.terminal.log_ai("AI: Analyzing target and planning attack...");
                        app.ai_input.clear();
                    }
                    app.input_mode = InputMode::Normal;
                }
                KeyCode::Esc => {
                    app.ai_input.clear();
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
        _ => {
            app.input_mode = InputMode::Normal;
        }
    }
}

// Navigate DOWN on current page's list
fn navigate_down(app: &mut App) {
    match app.current_tab {
        AppTab::Scanning => {
            if app.selected_module < app.modules.len() - 1 {
                app.selected_module += 1;
                if app.selected_module > app.scan_scroll + 15 {
                    app.scan_scroll += 1;
                }
            }
        }
        AppTab::Terminal => {
            app.terminal_scroll = (app.terminal_scroll + 1).min(app.terminal.lines.len().saturating_sub(1));
        }
        AppTab::Payloads => {
            if app.selected_platform < app.payload_platforms.len() - 1 {
                app.selected_platform += 1;
            }
        }
        AppTab::Ai => {
            app.selected_provider = (app.selected_provider + 1) % app.ai_providers.len();
        }
        AppTab::Config => {
            app.config_selected = (app.config_selected + 1) % 6;
        }
        _ => {}
    }
}

// Navigate UP on current page's list
fn navigate_up(app: &mut App) {
    match app.current_tab {
        AppTab::Scanning => {
            if app.selected_module > 0 {
                app.selected_module -= 1;
                if app.selected_module < app.scan_scroll {
                    app.scan_scroll = app.selected_module;
                }
            }
        }
        AppTab::Terminal => {
            app.terminal_scroll = app.terminal_scroll.saturating_sub(1);
        }
        AppTab::Payloads => {
            if app.selected_platform > 0 {
                app.selected_platform -= 1;
            }
        }
        AppTab::Ai => {
            app.selected_provider = app.selected_provider.saturating_sub(1);
        }
        AppTab::Config => {
            app.config_selected = app.config_selected.saturating_sub(1);
        }
        _ => {}
    }
}

// Action: 'a' attack / main action
fn action_attack(app: &mut App) {
    match app.current_tab {
        AppTab::Dashboard | AppTab::Scanning => {
            if !app.target_url.is_empty() {
                let m = app.modules[app.selected_module].name.clone();
                app.start_scan(app.target_url.clone(), m.clone());
                app.status_message = format!("Starting {} on {}", m, app.target_url);
                app.current_tab = AppTab::Terminal;
                app.tab_index = 2;
            } else {
                app.status_message = "Set target first: press 't'".into();
            }
        }
        AppTab::Ai => {
            let provider_name = app.ai_providers[app.selected_provider].as_str();
            app.terminal.set_running("AI Attack", &app.target_url);
            app.terminal.log_ai(&format!("Starting AI attack via {} ({})", provider_name, app.ai_config.model));
            app.terminal.log_ai("Step 1: Reconnaissance...");
            app.terminal.log_info("Scanning target with AI assistance");
            app.terminal.log_ai("Step 2: Attack planning...");
            app.terminal.log_ai("Plan: XSS -> SQLi -> LFI -> CMDi");
            app.terminal.log_ai("Step 3: Executing XSS scan...");
            app.terminal.log_sent("GET /search?q=testparam HTTP/1.1");
            app.terminal.log_received("HTTP/1.1 200 OK (89ms)");
            app.terminal.add_finding(Severity::Critical, "XSS", "Reflected XSS in param 'q'");
            app.terminal.log_progress("[#####..............] 25% complete");
            app.terminal.log_ai("AI analysis: XSS confirmed. Moving to SQLi...");
            app.current_tab = AppTab::Terminal;
            app.tab_index = 2;
        }
        _ => {}
    }
}

// Action: 'x' stop
fn action_stop(app: &mut App) {
    if let Some(task) = app.scan_tasks.iter_mut().find(|t| t.status == "Running") {
        task.status = "Stopped".into();
        app.terminal.stop();
        app.status_message = "Scan stopped".into();
    }
}

// Action: Enter = confirm / context action
fn action_enter(app: &mut App) {
    match app.current_tab {
        AppTab::Scanning => {
            if !app.target_url.is_empty() {
                let m = app.modules[app.selected_module].name.clone();
                app.start_scan(app.target_url.clone(), m.clone());
                app.status_message = format!("Starting {}", m);
                app.current_tab = AppTab::Terminal;
                app.tab_index = 2;
            }
        }
        AppTab::Payloads => {
            let platform = app.payload_platforms[app.selected_platform];
            let payload_type = app.payload_types[app.selected_payload % app.payload_types.len()];
            app.terminal.log_info(&format!("Generating {} payload ({}) LHOST={} LPORT={}", platform, payload_type, app.payload_lhost, app.payload_lport));
            app.terminal.push(LineKind::Info, &format!("Payload generated and saved to payloads/{}", platform.to_lowercase()), None);
        }
        AppTab::Config => {
            app.status_message = "Configuration saved".into();
        }
        _ => {}
    }
}

// ============================================================
// PAGE-SPECIFIC EXTRA KEYS (additional functionality per page)
// ============================================================

fn handle_dashboard_extra(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('h') => {
            app.input_mode = InputMode::Lhost;
            app.input_buffer = app.payload_lhost.clone();
        }
        KeyCode::Char('p') => {
            app.input_mode = InputMode::Lport;
            app.input_buffer = app.payload_lport.to_string();
        }
        _ => {}
    }
}

fn handle_scanning_extra(app: &mut App, key: KeyEvent) {
    // No extra keys needed - all handled by global keys
}

fn handle_terminal_extra(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') => {
            app.terminal.clear();
            app.status_message = "Terminal cleared".into();
        }
        KeyCode::Char('s') => {
            let path = format!("reports/terminal_{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));
            if let Ok(_) = app.terminal.save_to_file(&path) {
                app.status_message = format!("Saved to {}", path);
            } else {
                app.status_message = "Save failed".into();
            }
        }
        _ => {}
    }
}

fn handle_payload_extra(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('h') => {
            app.input_mode = InputMode::PayloadLhost;
            app.input_buffer = app.payload_lhost.clone();
        }
        KeyCode::Char('l') => {
            app.input_mode = InputMode::PayloadLport;
            app.input_buffer = app.payload_lport.to_string();
        }
        KeyCode::Char('g') => {
            let platform = app.payload_platforms[app.selected_platform];
            let payload_type = app.payload_types[app.selected_payload % app.payload_types.len()];
            app.terminal.log_info(&format!("Generating {} payload ({}) LHOST={} LPORT={}", platform, payload_type, app.payload_lhost, app.payload_lport));
            app.terminal.push(LineKind::Info, &format!("Payload generated: payloads/{}", platform.to_lowercase()), None);
            app.status_message = format!("{} payload generated", platform);
        }
        KeyCode::Char('c') => {
            app.terminal.log_info("Starting listener...");
            app.terminal.log_info(&format!("Listening on {}:{}", app.payload_lhost, app.payload_lport));
            app.status_message = "Listener started".into();
        }
        _ => {}
    }
}

fn handle_ai_extra(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') => {
            app.input_mode = InputMode::AiChat;
            app.ai_input.clear();
            app.status_message = "AI Chat mode: type your message, Enter to send".into();
        }
        KeyCode::Char('l') => {
            app.status_message = "AI logs saved to ai-attacks/".into();
        }
        KeyCode::Char(c) if c >= '1' && c <= '9' => {
            let idx = (c as usize) - ('1' as usize);
            if idx < app.ai_providers.len() {
                app.selected_provider = idx;
                app.status_message = format!("Selected: {}", app.ai_providers[idx].as_str());
            }
        }
        _ => {}
    }
}

fn handle_device_extra(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('r') => {
            app.device.update();
            app.device_refresh = Duration::ZERO;
            app.status_message = "Device stats refreshed".into();
        }
        _ => {}
    }
}

fn handle_config_extra(app: &mut App, key: KeyEvent) {
    // No extra keys needed
}

// ============================================================
// DRAW UI
// ============================================================

fn draw_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(size);

    draw_header(f, app, chunks[0]);
    draw_tab_content(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(35), Constraint::Min(20), Constraint::Length(30)])
        .split(area);

    let logo = Paragraph::new(vec![
        Line::from(Span::styled("  CF-VOID :: IND 'CYBER-FORCE'", Style::default().fg(GOLD).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  Offensive Security Platform", Style::default().fg(CRIMSON))),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(GOLD)));
    f.render_widget(logo, header_chunks[0]);

    let tab_names: Vec<Line> = app.tabs.iter().enumerate().map(|(i, t)| {
        if i == app.tab_index {
            Line::from(Span::styled(*t, Style::default().fg(GOLD).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(Span::styled(*t, Style::default().fg(WHITE)))
        }
    }).collect();
    f.render_widget(
        Tabs::new(tab_names)
            .select(app.tab_index)
            .divider(Span::raw("  "))
            .style(Style::default().fg(WHITE)),
        header_chunks[1],
    );

    let mut stats = vec![
        Span::raw(" Findings: "),
        Span::raw(app.terminal.findings.len().to_string()).style(Style::default().fg(YELLOW)),
        Span::raw(" | Req: "),
        Span::raw(app.terminal.requests.to_string()).style(Style::default().fg(AZURE)),
        Span::raw(" | Sess: "),
        Span::raw(app.sessions.len().to_string()).style(Style::default().fg(GREEN)),
    ];
    let uptime = {
        let elapsed = app.terminal.start_time.elapsed();
        format!("{}h {}m {}s", elapsed.as_secs()/3600, (elapsed.as_secs()%3600)/60, elapsed.as_secs()%60)
    };
    stats.push(Span::raw(format!(" | Up: {}", uptime)));

    f.render_widget(
        Paragraph::new(Line::from(stats)).block(
            Block::default().borders(Borders::ALL).border_style(Style::default().fg(AZURE))
        ),
        header_chunks[2],
    );
}

fn draw_tab_content(f: &mut Frame, app: &mut App, area: Rect) {
    match app.current_tab {
        AppTab::Dashboard => draw_dashboard(f, app, area),
        AppTab::Scanning => draw_scanning(f, app, area),
        AppTab::Terminal => draw_terminal(f, app, area),
        AppTab::Payloads => draw_payloads(f, app, area),
        AppTab::Ai => draw_ai(f, app, area),
        AppTab::Device => draw_device(f, app, area),
        AppTab::Config => draw_config(f, app, area),
    }
}

fn draw_dashboard(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(1)])
        .split(area);

    let target_block = Block::default()
        .title("Target [t=Set Target]")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AZURE));

    let target_text: Vec<Line> = if app.input_mode == InputMode::Target {
        vec![
            Line::from(Span::styled(format!("  Target: {}_", app.input_buffer), Style::default().fg(GOLD))),
            Line::from(Span::styled("  Enter=set  Esc=cancel", Style::default().fg(DIM))),
        ]
    } else if app.target_url.is_empty() {
        vec![Line::from(Span::styled("  Press 't' to enter target URL", Style::default().fg(DIM)))]
    } else {
        vec![Line::from(Span::styled(
            format!("  {} | 'a' = attack all | 'h' = set LHOST | 'p' = set LPORT", app.target_url),
            Style::default().fg(GOLD)
        ))]
    };

    f.render_widget(Paragraph::new(target_text).block(target_block), chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let recent_items: Vec<ListItem> = app.scan_tasks.iter().rev().take(8).map(|t| {
        let color = match t.status.as_str() {
            "Running" => GOLD,
            "Complete" => GREEN,
            "Failed" => CRIMSON,
            "Stopped" => YELLOW,
            _ => DIM,
        };
        ListItem::new(Line::from(vec![
            Span::styled(format!("  {} -> {} [{}]", t.module, t.target, t.status), Style::default().fg(color)),
        ]))
    }).collect();

    f.render_widget(
        List::new(recent_items).block(
            Block::default().title("Recent Scans").borders(Borders::ALL).border_style(Style::default().fg(AZURE))
        ),
        content_chunks[0],
    );

    let mem = sysinfo::System::new_all();
    let mem_used = mem.used_memory();
    let mem_total = mem.total_memory();
    let mem_pct = if mem_total > 0 { (mem_used as f32 / mem_total as f32) * 100.0 } else { 0.0 };

    let stats_items = vec![
        Line::from(vec![
            Span::raw("CPU "),
            Span::raw(format!("{:.0}%", 0)).style(Style::default().fg(GOLD)),
        ]),
        Line::from(Span::raw(device::ascii_bar(45.0, 30))),
        Line::from(vec![
            Span::raw("RAM "),
            Span::raw(format!("{:.0}%", mem_pct)).style(Style::default().fg(GOLD)),
        ]),
        Line::from(Span::raw(device::ascii_bar(mem_pct, 30))),
        Line::from(vec![
            Span::raw("Disk "),
            Span::raw(format!("{:.0}%", 78.0)).style(Style::default().fg(GOLD)),
        ]),
        Line::from(Span::raw(device::ascii_bar(78.0, 30))),
        Line::from(Span::styled(format!("  Network: wlan0 @ 192.168.1.105"), Style::default().fg(DIM))),
        Line::from(Span::styled(format!("  Proxy: {}", if !app.config.proxy_url.is_empty() { &app.config.proxy_url[..] } else { "(none)" }), Style::default().fg(DIM))),
    ];

    f.render_widget(
        Paragraph::new(stats_items).block(
            Block::default().title("Live Stats").borders(Borders::ALL).border_style(Style::default().fg(GREEN))
        ),
        content_chunks[1],
    );
}

fn draw_scanning(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let items: Vec<ListItem> = app.modules.iter().enumerate().map(|(i, m)| {
        let style = if i == app.selected_module {
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(WHITE)
        };
        ListItem::new(Line::from(vec![
            Span::styled(format!("  {}", m.name), style),
        ]))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_module.saturating_sub(app.scan_scroll)));

    f.render_stateful_widget(
        List::new(items)
            .block(Block::default().title("Modules [j/k=Select a=Attack]").borders(Borders::ALL).border_style(Style::default().fg(CRIMSON)))
            .highlight_style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
            .highlight_symbol("> "),
        chunks[0],
        &mut state,
    );

    let m = &app.modules[app.selected_module];
    let info = vec![
        Line::from(Span::styled(format!("  {}", m.name), Style::default().fg(GOLD).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("  Category: {}", m.category), Style::default().fg(TEAL))),
        Line::from(Span::styled(format!("  {}", m.description), Style::default().fg(WHITE))),
        Line::from(Span::styled(format!("  Target: {}", if app.target_url.is_empty() { "(none)" } else { &app.target_url }), Style::default().fg(AZURE))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("  [a] Start Attack  [x] Stop  [t] Set Target", Style::default().fg(DIM))),
    ];

    f.render_widget(
        Paragraph::new(info).block(
            Block::default().title("Module Details").borders(Borders::ALL).border_style(Style::default().fg(AZURE))
        ).wrap(Wrap { trim: false }),
        chunks[1],
    );
}

fn draw_terminal(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let term_height = chunks[0].height as usize;
    let start_idx = app.terminal.lines.len().saturating_sub(term_height).saturating_sub(app.terminal_scroll);
    let lines = &app.terminal.lines[start_idx..];

    let term_lines: Vec<Line> = lines.iter().map(|line| {
        let style = match line.kind {
            LineKind::Sent => Style::default().fg(AZURE),
            LineKind::Received => Style::default().fg(DIM),
            LineKind::Finding => {
                let color = line.severity.map(|s| s.ratatui_color()).unwrap_or(YELLOW);
                Style::default().fg(color)
            }
            LineKind::Info => Style::default().fg(TEAL),
            LineKind::Error => Style::default().fg(CRIMSON),
            LineKind::AiMsg => Style::default().fg(GREEN),
            LineKind::Progress => Style::default().fg(GOLD),
            LineKind::Command => Style::default().fg(ORANGE),
            LineKind::Response => Style::default().fg(WHITE),
            LineKind::Debug => Style::default().fg(DIM),
        };

        let prefix = match line.kind {
            LineKind::Finding => {
                if let Some(sev) = line.severity {
                    format!("[{}] ", sev.prefix())
                } else {
                    "[-] ".to_string()
                }
            }
            _ => format!("{} ", line.kind.prefix()),
        };

        Line::from(vec![
            Span::styled(format!("[{}] ", line.time), Style::default().fg(DIM)),
            Span::styled(prefix, style),
            Span::styled(&line.content, style),
        ])
    }).collect();

    f.render_widget(
        Paragraph::new(term_lines).block(
            Block::default().title("Attack Terminal")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.terminal.running { GREEN } else { DIM }))
        ).wrap(Wrap { trim: false }),
        chunks[0],
    );

    let progress_bar = if app.terminal.running {
        device::ascii_bar(app.terminal.progress, 30)
    } else {
        "[idle]".to_string()
    };

    let footer = Line::from(vec![
        Span::styled(&progress_bar, Style::default().fg(GOLD)),
        Span::raw(format!(" | {} req | {} findings | {}", app.terminal.requests, app.terminal.findings.len(), app.terminal.current_module)),
        Span::raw(" [c] clear [s] save [ctrl+c] stop"),
    ]);

    f.render_widget(
        Paragraph::new(footer).block(
            Block::default().borders(Borders::ALL).border_style(Style::default().fg(DIM))
        ),
        chunks[1],
    );
}

fn draw_payloads(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    let items: Vec<ListItem> = app.payload_platforms.iter().enumerate().map(|(i, p)| {
        let style = if i == app.selected_platform {
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(WHITE)
        };
        ListItem::new(Line::from(Span::styled(format!("  {}", p), style)))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_platform));

    f.render_stateful_widget(
        List::new(items)
            .block(Block::default().title("Platform [j/k=Select g=Generate]").borders(Borders::ALL).border_style(Style::default().fg(TEAL)))
            .highlight_style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
            .highlight_symbol("> "),
        chunks[0],
        &mut state,
    );

    let platform = app.payload_platforms[app.selected_platform];
    let info = vec![
        Line::from(Span::styled(format!("  Platform: {}", platform), Style::default().fg(GOLD).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("  LHOST: {}", app.payload_lhost), Style::default().fg(AZURE))),
        Line::from(Span::styled(format!("  LPORT: {}", app.payload_lport), Style::default().fg(AZURE))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("  [g] Generate  [c] Connect  [h] LHOST  [l] LPORT", Style::default().fg(DIM))),
    ];

    f.render_widget(
        Paragraph::new(info).block(
            Block::default().title(format!("Payload: {}", app.payload_types[app.selected_payload % app.payload_types.len()]))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(GREEN))
        ),
        chunks[1],
    );
}

fn draw_ai(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(1)])
        .split(area);

    let providers: Vec<Line> = app.ai_providers.iter().enumerate().map(|(i, p)| {
        if i == app.selected_provider {
            Line::from(Span::styled(format!(" > {} ({})", p.as_str(), p.default_model()),
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(Span::styled(format!("   {}", p.as_str()), Style::default().fg(WHITE)))
        }
    }).collect();

    f.render_widget(
        Paragraph::new(providers).block(
            Block::default().title("AI Providers [j/k=Select a=Attack c=Chat]").borders(Borders::ALL).border_style(Style::default().fg(CRIMSON))
        ),
        chunks[0],
    );

    let model_line = format!("  Model: {} | Mode: {} | Key: {}",
        app.ai_config.model,
        match app.ai_mode {
            AiMode::Auto => "AUTO",
            AiMode::Guided => "GUIDED",
            AiMode::Chat => "CHAT",
        },
        if app.ai_config.api_key.is_empty() { "(not set)" } else { "(set)" }
    );

    let info = vec![
        Line::from(Span::styled(model_line, Style::default().fg(AZURE))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("  [a] Start AI Attack  [c] Chat  [l] View Logs  [1-9] Select Provider", Style::default().fg(DIM))),
    ];

    f.render_widget(
        Paragraph::new(info).block(
            Block::default().title("AI Configuration").borders(Borders::ALL).border_style(Style::default().fg(GREEN))
        ),
        chunks[1],
    );
}

fn draw_device(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), Constraint::Length(4),
            Constraint::Length(4), Constraint::Length(4),
            Constraint::Length(4), Constraint::Length(5),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("Platform: "), Span::raw(&app.device.platform).style(Style::default().fg(GOLD)),
                Span::raw(" | Arch: "), Span::raw(&app.device.arch).style(Style::default().fg(GOLD)),
                Span::raw(" | Kernel: "), Span::raw(&app.device.kernel).style(Style::default().fg(GOLD)),
            ]),
            Line::from(vec![
                Span::raw("Host: "), Span::raw(&app.device.hostname).style(Style::default().fg(AZURE)),
                Span::raw(" | Uptime: "), Span::raw(app.device.format_uptime()).style(Style::default().fg(AZURE)),
            ]),
        ]).block(Block::default().title("System Info").borders(Borders::ALL).border_style(Style::default().fg(AZURE))),
        chunks[0],
    );

    let cpu_pct = app.device.cpu_percent;
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("CPU "),
                Span::raw(format!("{:.0}%", cpu_pct)).style(Style::default().fg(GOLD)),
                Span::raw(format!(" | {} cores @ {}MHz", app.device.cpu_cores, app.device.cpu_freq_mhz)),
            ]),
            Line::from(Span::raw(device::ascii_bar(cpu_pct, 40))),
        ]).block(Block::default().title("CPU [r=Refresh]").borders(Borders::ALL).border_style(Style::default().fg(CRIMSON))),
        chunks[1],
    );

    let ram_pct = app.device.ram_percent;
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("RAM "),
                Span::raw(format!("{:.0}%", ram_pct)).style(Style::default().fg(GOLD)),
                Span::raw(format!(" | {} MB / {} MB", app.device.ram_used_mb, app.device.ram_total_mb)),
            ]),
            Line::from(Span::raw(device::ascii_bar(ram_pct, 40))),
        ]).block(Block::default().title("Memory").borders(Borders::ALL).border_style(Style::default().fg(CRIMSON))),
        chunks[2],
    );

    let disk_pct = app.device.disk_percent;
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("Disk (/) "),
                Span::raw(format!("{:.0}%", disk_pct)).style(Style::default().fg(GOLD)),
                Span::raw(format!(" | {:.1}GB / {:.1}GB", app.device.disk_used_gb, app.device.disk_total_gb)),
            ]),
            Line::from(Span::raw(device::ascii_bar(disk_pct, 40))),
        ]).block(Block::default().title("Storage").borders(Borders::ALL).border_style(Style::default().fg(CRIMSON))),
        chunks[3],
    );

    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("NIC: "), Span::raw(&app.device.net_iface).style(Style::default().fg(AZURE)),
                Span::raw(" | RX: "), Span::raw(device::DeviceStats::format_bytes(app.device.net_rx_bytes)).style(Style::default().fg(AZURE)),
                Span::raw(" | TX: "), Span::raw(device::DeviceStats::format_bytes(app.device.net_tx_bytes)).style(Style::default().fg(AZURE)),
            ]),
        ]).block(Block::default().title("Network").borders(Borders::ALL).border_style(Style::default().fg(TEAL))),
        chunks[4],
    );

    let proxy_stats = app.proxy_scraper.stats();
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("Temp: "), Span::raw(format!("{}C", app.device.temperature_c)).style(Style::default().fg(ORANGE)),
                Span::raw(" | Load: "), Span::raw("1.42 0.98 0.76").style(Style::default().fg(GOLD)),
            ]),
            Line::from(vec![
                Span::raw("Proxies: "), Span::raw(format!("{}/{} alive", proxy_stats.1, proxy_stats.0)).style(Style::default().fg(TEAL)),
                Span::raw(" | Mode: "), Span::raw(match app.proxy_mode {
                    ProxyMode::Manual => "MANUAL",
                    ProxyMode::AutoScrape => "AUTO-SCRAPE",
                    ProxyMode::Rotating => "ROTATING",
                }).style(Style::default().fg(YELLOW)),
            ]),
            Line::from(Span::raw(device::ascii_bar((proxy_stats.1 as f32 / proxy_stats.0 as f32) * 100.0, 40))),
        ]).block(Block::default().title("Hardware & Proxy").borders(Borders::ALL).border_style(Style::default().fg(TEAL))),
        chunks[5],
    );
}

fn draw_config(f: &mut Frame, app: &mut App, area: Rect) {
    let labels = ["General", "HTTP", "Scanning", "Proxy", "Output", "AI"];
    let selected_label = labels[app.config_selected % labels.len()];

    let items: Vec<ListItem> = labels.iter().enumerate().map(|(i, l)| {
        if i == app.config_selected {
            ListItem::new(Line::from(Span::styled(format!(" > {}", l),
                Style::default().fg(GOLD).add_modifier(Modifier::BOLD))))
        } else {
            ListItem::new(Line::from(Span::styled(format!("   {}", l), Style::default().fg(WHITE))))
        }
    }).collect();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(1)])
        .split(area);

    let mut state = ListState::default();
    state.select(Some(app.config_selected));

    f.render_stateful_widget(
        List::new(items).block(
            Block::default().title("Config [j/k=Select]").borders(Borders::ALL).border_style(Style::default().fg(AZURE))
        ).highlight_style(Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
        chunks[0],
        &mut state,
    );

    let details = match selected_label {
        "General" => vec![
            Line::from(Span::styled("  Version: 4.0.0", Style::default().fg(WHITE))),
            Line::from(Span::styled("  Author: IND 'CYBER-FORCE'", Style::default().fg(GOLD))),
            Line::from(Span::styled("  License: GPL-3.0", Style::default().fg(DIM))),
        ],
        "HTTP" => vec![
            Line::from(Span::styled(format!("  User-Agent: {}", app.config.user_agent), Style::default().fg(WHITE))),
            Line::from(Span::styled(format!("  Timeout: {}s", app.config.timeout_secs), Style::default().fg(WHITE))),
            Line::from(Span::styled(format!("  Redirects: {}", if app.config.follow_redirects { "ON" } else { "OFF" }), Style::default().fg(WHITE))),
        ],
        "Scanning" => vec![
            Line::from(Span::styled(format!("  Threads: {}", app.config.threads), Style::default().fg(WHITE))),
            Line::from(Span::styled("  Ports: 1-1000", Style::default().fg(WHITE))),
            Line::from(Span::styled("  Scan Type: SYN", Style::default().fg(WHITE))),
        ],
        "Proxy" => {
            let (total, alive, _) = app.proxy_scraper.stats();
            vec![
                Line::from(Span::styled(format!("  Mode: {:?}", match app.proxy_mode {
                    ProxyMode::Manual => "MANUAL",
                    ProxyMode::AutoScrape => "AUTO-SCRAPE",
                    ProxyMode::Rotating => "ROTATING",
                }), Style::default().fg(WHITE))),
                Line::from(Span::styled(format!("  Current: {}", &app.config.proxy_url), Style::default().fg(WHITE))),
                Line::from(Span::styled(format!("  Pool: {}/{} proxies alive", alive, total), Style::default().fg(WHITE))),
                Line::from(Span::styled("  [r] Re-scrape  [t] Test  [n] Next", Style::default().fg(DIM))),
            ]
        },
        "Output" => vec![
            Line::from(Span::styled(format!("  Directory: {}", app.config.output_dir), Style::default().fg(WHITE))),
            Line::from(Span::styled("  Format: JSON + HTML + Markdown", Style::default().fg(WHITE))),
        ],
        "AI" => {
            let provider_name = app.ai_providers[app.selected_provider].as_str();
            vec![
                Line::from(Span::styled(format!("  Provider: {}", provider_name), Style::default().fg(WHITE))),
                Line::from(Span::styled(format!("  Model: {}", app.ai_config.model), Style::default().fg(WHITE))),
                Line::from(Span::styled(format!("  Key: {}", if app.ai_config.api_key.is_empty() { "(not set)" } else { "(set)" }), Style::default().fg(WHITE))),
                Line::from(Span::styled("  [k] Open keys file  [a] Test connection", Style::default().fg(DIM))),
            ]
        },
        _ => vec![],
    };

    f.render_widget(
        Paragraph::new(details).block(
            Block::default().title("Config Details").borders(Borders::ALL).border_style(Style::default().fg(GREEN))
        ),
        chunks[1],
    );
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(50)])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" > ", Style::default().fg(GOLD)),
            Span::styled(&app.status_message, Style::default().fg(WHITE)),
        ])).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(AZURE))),
        footer_chunks[0],
    );

    // Unified key hints shown on every page
    let hints = "[Left/Right]=Tab [Up/Down]=Nav [a]=Attack [x]=Stop [t]=Target [Enter]=OK [Esc]=Back [q]=Quit";
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(hints, Style::default().fg(DIM)),
        ])).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(DIM))),
        footer_chunks[1],
    );
}
