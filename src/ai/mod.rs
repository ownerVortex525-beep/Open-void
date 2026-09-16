// CF-VOID AI Module
// AI/ML implementations
// Author: IND 'CYBER-FORCE'

pub mod classifier;
pub mod anomaly;
pub mod mutator;
pub mod config;
pub mod engine;
pub mod logs;
pub mod prompts;

// Re-exports
pub use config::{AiConfig, AiProvider};
pub use engine::{AiEngine, ReconResult, AttackPlan, AttackStep, StepResult, StepFinding, FindingAnalysis, AttackReport};
pub use logs::{AiLogger, AiLogEntry, LogLevel};

use crate::cli::banner;
use std::io::{self, Write};

pub async fn start_interactive_ai_menu() {
    banner::info("=== AI Attack Menu ===");
    banner::info("1. Run AI-guided attack reconnaissance");
    banner::info("2. Generate attack plan");
    banner::info("3. Execute attack steps");
    banner::info("4. AI chat assistant");
    banner::info("5. Configure AI providers");
    banner::info("6. View AI logs");
    banner::info("7. Custom attack chain");
    banner::info("Enter choice (1-7) or 'back': ");
    // Note: Full interactive menu handled in TUI; this is CLI fallback
    banner::success("Use 'ai <subcommand>' for direct commands.");
}

pub async fn start_chat_mode() {
    use std::io::{self, Write};
    use crate::ai::config::AiConfig;
    use crate::ai::engine::AiEngine;

    let config = match AiConfig::load_from_file() {
        Some(c) => c,
        None => {
            banner::error("No AI config found. Run 'ai config' first.");
            return;
        }
    };

    let engine = AiEngine {
        config,
        client: reqwest::Client::new(),
        initialized: true,
        logger: crate::ai::logs::AiLogger::new("ai-logs", ""),
        start_time: None,
    };

    loop {
        print!("\n{} You: ", crate::cli::banner::tc("└─₹", crate::cli::banner::GOLD));
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            banner::error("Failed to read input");
            break;
        }
        let input = input.trim();
        if input == "exit" || input == "quit" || input == "back" {
            break;
        }
        if input.is_empty() {
            continue;
        }
        let response = engine.chat("You are a helpful cybersecurity assistant.", input).await;
        match response {
            Ok(r) => println!("{} AI: {}", crate::cli::banner::tc("┌──(CF➤VOID)", crate::cli::banner::GOLD), r),
            Err(e) => banner::error(&format!("AI error: {}", e)),
        }
    }
}

pub async fn start_chain_builder() {
    banner::info("Custom AI Attack Chain Builder");
    banner::info("Enter target URL: ");
    use std::io::{self, Write};
    print!("{} > ", crate::cli::banner::tc("!", crate::cli::banner::CRIMSON));
    io::stdout().flush().unwrap_or(());
    let mut target = String::new();
    if io::stdin().read_line(&mut target).is_err() {
        banner::error("Failed to read target");
        return;
    }
    let target = target.trim();
    if target.is_empty() {
        banner::error("Target URL is required");
        return;
    }

    banner::info("Enter attack phases (comma-separated, e.g., recon,scan,exploit): ");
    print!("{} > ", crate::cli::banner::tc("!", crate::cli::banner::CRIMSON));
    io::stdout().flush().unwrap_or(());
    let mut phases = String::new();
    if io::stdin().read_line(&mut phases).is_err() {
        banner::error("Failed to read phases");
        return;
    }
    let phases: Vec<String> = phases.trim().split(',').map(|s| s.trim().to_string()).collect();

    let mut cli_args = crate::cli::args::CliArgs::default();
    cli_args.url = Some(target.to_string());
    cli_args.ai = true;

    banner::info(&format!("Running custom attack chain with phases: {}", phases.join(" -> ")));
    let _ = crate::core::engine::run_ai_attack_with_provider(&cli_args, "openai").await;
}

pub fn list_providers() {
    crate::cli::banner::info("Available AI Providers:");
    let providers = [
        ("OpenAI", "GPT-4o, GPT-4-turbo"),
        ("Anthropic", "Claude-3-Opus, Claude-3.5-Sonnet"),
        ("Gemini", "Gemini-2.0-flash, Gemini-1.5-pro"),
        ("Groq", "Llama-3-70B, Mixtral-8x7B"),
        ("Mimo", "Mimo-7B-Instruct"),
        ("DeepSeek", "DeepSeek-V2, DeepSeek-Coder"),
        ("Google AI Studio", "Gemini via API key"),
        ("Ollama", "Local models (phi3, mistral, llama3, etc)"),
        ("Custom", "Custom endpoint"),
    ];
    for (name, models) in &providers {
        crate::cli::banner::info(&format!("  {} - {}", crate::cli::banner::tc(name, crate::cli::banner::TEAL), models));
    }
    crate::cli::banner::success("Configure providers with: ai config");
}
