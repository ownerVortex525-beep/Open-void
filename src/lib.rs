// CF-VOID - Complete Offensive Security Platform
// Library Root Module
// Author: CYBER-FORCE

pub mod cli;
pub mod core;
pub mod http;
pub mod scanner;
pub mod fuzz;
pub mod ai;
pub mod evasion;
pub mod payload;
pub mod payloads;
pub mod exploits;
pub mod exploit_new;
pub mod post_exploit;
pub mod session;
pub mod report;
pub mod utils;
pub mod config;
pub mod phishing;

use std::sync::atomic::{AtomicBool, Ordering};

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn is_shutdown_requested() -> bool {
    SHUTDOWN.load(Ordering::Acquire)
}

pub const VERSION: &str = "1.0";
pub const NAME: &str = "CF-VOID";
pub const DESCRIPTION: &str = "Complete Offensive Security Platform";
pub const AUTHOR: &str = "CYBER-FORCE";

// Re-exports for convenience
pub use cli::args::CliArgs;
pub use core::engine::ScanEngine;
pub use http::client::HttpClient;
