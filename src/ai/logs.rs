// CF-VOID AI Attack Log System
// Author: CYBER-FORGE
// Saves all AI attack logs, reasoning, and data to separate folders

use std::path::PathBuf;
use std::time::Instant;

pub struct AiLogger {
    pub base_dir: String,
    pub target: String,
    pub session_dir: PathBuf,
    pub start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct AiLogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub module: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    AiMessage,
    Attack,
    Finding,
    Error,
    Warning,
}

impl LogLevel {
    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Info => "[*]",
            LogLevel::AiMessage => "[AI]",
            LogLevel::Attack => "[>]",
            LogLevel::Finding => "[+]",
            LogLevel::Error => "[!]",
            LogLevel::Warning => "[?]",
        }
    }
}

impl AiLogger {
    pub fn new(base_dir: &str, target: &str) -> Self {
        let session_dir = if target.is_empty() {
            let mut dir = PathBuf::from(base_dir);
            dir.push("unknown_target");
            dir.push(chrono::Local::now().format("%Y-%m-%d_%H%M%S").to_string());
            dir
        } else {
            let mut dir = PathBuf::from(base_dir);
            dir.push(sanitize_target(target));
            dir.push(chrono::Local::now().format("%Y-%m-%d_%H%M%S").to_string());
            dir
        };

        let _ = std::fs::create_dir_all(&session_dir);

        Self {
            base_dir: base_dir.to_string(),
            target: target.to_string(),
            session_dir,
            start_time: Instant::now(),
        }
    }

    pub async fn log(&self, message: &str) -> anyhow::Result<()> {
        self.write_log(LogLevel::Info, None, message).await
    }

    pub async fn log_ai(&self, message: &str) -> anyhow::Result<()> {
        self.write_log(LogLevel::AiMessage, None, message).await
    }

    pub async fn log_attack(&self, module: &str, message: &str) -> anyhow::Result<()> {
        self.write_log(LogLevel::Attack, Some(module.to_string()), message).await
    }

    pub async fn log_finding(&self, module: &str, message: &str) -> anyhow::Result<()> {
        self.write_log(LogLevel::Finding, Some(module.to_string()), message).await
    }

    pub async fn log_error(&self, message: &str) -> anyhow::Result<()> {
        self.write_log(LogLevel::Error, None, message).await
    }

    pub async fn log_step(&self, step_id: usize, module: &str, result: &str) -> anyhow::Result<()> {
        let message = format!("Step {}: {} -> {}", step_id, module, result);
        self.write_log(LogLevel::Attack, Some(module.to_string()), &message).await
    }

    async fn write_log(&self, level: LogLevel, module: Option<String>, message: &str) -> anyhow::Result<()> {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        let line = format!("[{}] {} {}\n", timestamp, level.prefix(), message);

        let log_path = self.session_dir.join("ai_attack.log");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            use std::io::Write;
            let _ = file.write_all(line.as_bytes());
        }

        Ok(())
    }

    pub async fn save_json(&self, filename: &str, data: &serde_json::Value) -> anyhow::Result<()> {
        let path = self.session_dir.join(filename);
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(&path, json)?;
        Ok(())
    }

    pub async fn save_raw(&self, filename: &str, content: &str) -> anyhow::Result<()> {
        let path = self.session_dir.join(filename);
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub async fn save_report(&self, report: &str) -> anyhow::Result<()> {
        let path = self.session_dir.join("report.md");
        std::fs::write(&path, report)?;
        Ok(())
    }

    pub fn session_dir_str(&self) -> String {
        self.session_dir.to_string_lossy().to_string()
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

fn sanitize_target(target: &str) -> String {
    target.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn print_ai_logs() {
    use crate::cli::banner;
    let log_dir = std::path::Path::new("ai-logs");
    if !log_dir.exists() {
        banner::info("No AI logs found yet.");
        return;
    }

    banner::info("=== AI Activity Logs ===");
    if let Ok(entries) = std::fs::read_dir(log_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name() {
                        banner::info(&format!("  Target: {}", name.to_string_lossy()));
                    }
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub in sub_entries {
                            if let Ok(sub) = sub {
                                if let Some(subname) = sub.path().file_name() {
                                    banner::info(&format!("    Session: {}", subname.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
