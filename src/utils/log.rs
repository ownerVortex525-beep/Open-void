// CF-VOID Live Log Module
// Author: CYBER-FORCE
// Unified live log stream for all operations with color-coded tags

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Success,
    Fail,
    Warn,
    Info,
    Progress,
}

impl LogLevel {
    pub fn tag(&self) -> &'static str {
        match self {
            LogLevel::Success => "[+]",
            LogLevel::Fail => "[-]",
            LogLevel::Warn => "[!]",
            LogLevel::Info => "[*]",
            LogLevel::Progress => "[~]",
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        use crate::cli::banner::{GREEN, CRIMSON, YELLOW, AZURE};
        match self {
            LogLevel::Success => GREEN,
            LogLevel::Fail => CRIMSON,
            LogLevel::Warn => YELLOW,
            LogLevel::Info => AZURE,
            LogLevel::Progress => (180, 0, 255), // MAGENTA
        }
    }
}

#[derive(Clone)]
pub struct LiveLogger {
    stats: Arc<Mutex<LogStats>>,
}

#[derive(Debug, Default)]
struct LogStats {
    success_count: AtomicU64,
    fail_count: AtomicU64,
    warn_count: AtomicU64,
    #[allow(dead_code)]
    start_time: Option<Instant>,
}

impl LiveLogger {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(LogStats::default())),
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let tag = level.tag();
        let color = level.color();
        let tag_colored = crate::cli::banner::tc(tag, color);
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        
        // Update stats
        if let Ok(stats) = self.stats.lock() {
            match level {
                LogLevel::Success => { let _ = stats.success_count.fetch_add(1, Ordering::SeqCst); }
                LogLevel::Fail => { let _ = stats.fail_count.fetch_add(1, Ordering::SeqCst); }
                LogLevel::Warn => { let _ = stats.warn_count.fetch_add(1, Ordering::SeqCst); }
                _ => {}
            };
        }

        println!("{} {} {}", 
            crate::cli::banner::tc(&timestamp, (128, 128, 128)),
            tag_colored,
            message
        );
        std::io::stdout().flush().ok();
    }

    pub fn success(&self, msg: &str) {
        self.log(LogLevel::Success, msg);
    }

    pub fn fail(&self, msg: &str) {
        self.log(LogLevel::Fail, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn progress(&self, msg: &str) {
        self.log(LogLevel::Progress, msg);
    }

    pub fn log_finding(&self, target: &str, vector: &str, payload: &str, success: bool, data: &str) {
        let status_tag = if success { "[+]" } else { "[-]" };
        let color = if success { crate::cli::banner::GREEN } else { crate::cli::banner::CRIMSON };
        let tag = crate::cli::banner::tc(status_tag, color);
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        
        if success {
            println!("{} {} {} {} on {} | payload: {} | data: {}",
                crate::cli::banner::tc(&timestamp, (128, 128, 128)),
                tag,
                crate::cli::banner::tc("SUCCESS", color),
                vector,
                target,
                payload,
                data
            );
        } else {
            println!("{} {} {} {} on {} | payload: {}",
                crate::cli::banner::tc(&timestamp, (128, 128, 128)),
                tag,
                crate::cli::banner::tc("FAIL", color),
                vector,
                target,
                payload
            );
        }
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        if let Ok(stats) = self.stats.lock() {
            (
                stats.success_count.load(Ordering::SeqCst),
                stats.fail_count.load(Ordering::SeqCst),
                stats.warn_count.load(Ordering::SeqCst),
            )
        } else {
            (0, 0, 0)
        }
    }

    pub fn print_stats(&self) {
        let (success, fail, warn) = self.stats();
        let _t = crate::cli::banner::TEAL;
        let g = crate::cli::banner::GOLD;
        let cr = crate::cli::banner::CRIMSON;
        let y = crate::cli::banner::YELLOW;
        
        println!();
        println!("{}  ╔════════════════════════════════════╗", crate::cli::banner::tc("▐", g));
        println!("{}  ║  {} {}   {} {}   {} {}  ║", 
            crate::cli::banner::tc("▐", g),
            crate::cli::banner::tc("SUCCESS:", crate::cli::banner::GREEN), crate::cli::banner::tc_string(success.to_string(), g),
            crate::cli::banner::tc("FAIL:", cr), crate::cli::banner::tc_string(fail.to_string(), g),
            crate::cli::banner::tc("WARN:", y), crate::cli::banner::tc_string(warn.to_string(), g));
        println!("{}  ╚════════════════════════════════════╝", crate::cli::banner::tc("▐", g));
    }
}

impl Default for LiveLogger {
    fn default() -> Self {
        Self::new()
    }
}

use std::io::Write as _;
