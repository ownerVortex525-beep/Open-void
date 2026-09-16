// CF-VOID AI Configuration
// Author: CYBER-FORCE
// 9 AI providers: OpenAI, Anthropic, Gemini, Groq, Mimo, DeepSeek, GoogleAI, Ollama, Custom

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,          // "openai" | "anthropic" | "gemini" | "groq" | "mimo" | "deepseek" | "google" | "ollama" | "custom"
    pub api_key: String,            // User's API key (loaded from keys.rs)
    pub model: String,             // "gpt-4o" | "claude-3-5-sonnet" | etc.
    pub base_url: Option<String>,   // Custom endpoint URL for Custom provider
    pub temperature: f32,          // 0.0 - 1.0
    pub max_tokens: u32,           // Response limit
    pub auto_attack: bool,         // AI decides everything vs ask user
    pub log_dir: String,           // "ai-attacks/{target}/"
    pub auto_analyze: bool,        // Analyze findings automatically
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            api_key: String::new(),
            model: "gpt-4o".to_string(),
            base_url: None,
            temperature: 0.7,
            max_tokens: 4096,
            auto_attack: false,
            log_dir: "ai-attacks".to_string(),
            auto_analyze: true,
        }
    }
}

impl AiConfig {
    pub fn load_from_file() -> Option<Self> {
        let path = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cf-void").join("keys.toml");
        if let Ok(content) = std::fs::read_to_string(path) {
            toml::from_str(&content).ok()
        } else {
            None
        }
    }

    pub async fn show_config_menu() {
        use crate::cli::banner;
        use std::io::{self, Write};

        banner::info("=== AI Provider Configuration ===");
        let config = Self::load_from_file().unwrap_or_default();
        banner::info(&format!("  Current provider: {}", config.provider));
        banner::info(&format!("  Current model: {}", config.model));

        let providers = AiProvider::all_providers();
        banner::info("\nSelect provider to configure:");
        for (i, (name, desc)) in providers.iter().enumerate() {
            println!("  {} {} - {}", banner::tc(&format!("[{}]", i+1), banner::TEAL), banner::tc(name, banner::GOLD), desc);
        }
        print!("{} > ", banner::tc("Choice", banner::GOLD));
        io::stdout().flush().unwrap_or(());

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            banner::error("Failed to read input");
            return;
        }
        let choice: usize = input.trim().parse().unwrap_or(0);
        if choice == 0 || choice > providers.len() {
            banner::error("Invalid choice");
            return;
        }

        let selected = &providers[choice - 1];
        banner::info(&format!("Configuring: {}", selected.0));

        banner::info("Enter API key (leave empty for local models): ");
        print!("{} > ", banner::tc("Key", banner::GOLD));
        io::stdout().flush().unwrap_or(());
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).unwrap_or(0);
        let api_key = api_key.trim().to_string();

        banner::info("Enter model name (or press Enter for default): ");
        print!("{} > ", banner::tc("Model", banner::GOLD));
        io::stdout().flush().unwrap_or(());
        let mut model_input = String::new();
        io::stdin().read_line(&mut model_input).unwrap_or(0);
        let model_input = model_input.trim();

        let keys_dir = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cf-void");
        std::fs::create_dir_all(&keys_dir).unwrap_or(());
        let keys_file = keys_dir.join("keys.toml");

        let key_str = if api_key.is_empty() {
            format!("[{}]\n", selected.0.to_lowercase())
        } else {
            format!("[{}]\napi_key = \"{}\"\n", selected.0.to_lowercase(), api_key)
        };
        let model_str = if !model_input.is_empty() {
            format!("model = \"{}\"\n", model_input)
        } else {
            String::new()
        };
        let content = format!("{}{}", key_str, model_str);

        if std::fs::write(&keys_file, content).is_ok() {
            banner::success(&format!("Configuration saved to {:?}", keys_file));
        } else {
            banner::error("Failed to save configuration");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AiProvider {
    OpenAI,        // GPT-4o, GPT-5
    Anthropic,     // Claude 3.5, Claude 4
    Gemini,        // gemini-2.0-flash, gemini-1.5-pro
    Groq,          // Llama 3.2, Mixtral
    Mimo,          // mimo-2.5
    DeepSeek,      // deepseek-chat
    GoogleAI,      // Google AI Studio
    Ollama,        // Local models (no API key)
    Custom,        // Any OpenAI-compatible endpoint
}

impl AiProvider {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" => Some(Self::OpenAI),
            "anthropic" => Some(Self::Anthropic),
            "gemini" => Some(Self::Gemini),
            "groq" => Some(Self::Groq),
            "mimo" => Some(Self::Mimo),
            "deepseek" => Some(Self::DeepSeek),
            "google" => Some(Self::GoogleAI),
            "ollama" => Some(Self::Ollama),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAI => "OpenAI",
            Self::Anthropic => "Anthropic",
            Self::Gemini => "Gemini",
            Self::Groq => "Groq",
            Self::Mimo => "Mimo",
            Self::DeepSeek => "DeepSeek",
            Self::GoogleAI => "Google AI Studio",
            Self::Ollama => "Ollama",
            Self::Custom => "Custom",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::Gemini => "gemini",
            Self::Groq => "groq",
            Self::Mimo => "mimo",
            Self::DeepSeek => "deepseek",
            Self::GoogleAI => "google",
            Self::Ollama => "ollama",
            Self::Custom => "custom",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Self::OpenAI => "gpt-4o",
            Self::Anthropic => "claude-3-5-sonnet-20241022",
            Self::Gemini => "gemini-2.0-flash",
            Self::Groq => "llama-3.2-90b-vision-preview",
            Self::Mimo => "mimo-2.5",
            Self::DeepSeek => "deepseek-chat",
            Self::GoogleAI => "gemini-pro",
            Self::Ollama => "llama3.2",
            Self::Custom => "any",
        }
    }

    pub fn endpoint(&self, base_url: Option<&str>, model: &str) -> String {
        match self {
            Self::OpenAI => "https://api.openai.com/v1/chat/completions".to_string(),
            Self::Anthropic => "https://api.anthropic.com/v1/messages".to_string(),
            Self::Gemini => format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent", model),
            Self::Groq => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            Self::Mimo => "https://api.mimo.ai/v1/chat/completions".to_string(),
            Self::DeepSeek => "https://api.deepseek.com/v1/chat/completions".to_string(),
            Self::GoogleAI => format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent", model),
            Self::Ollama => base_url.unwrap_or("http://localhost:11434").to_string(),
            Self::Custom => base_url.unwrap_or("").to_string(),
        }
    }

    pub fn requires_key(&self) -> bool {
        match self {
            Self::Ollama => false,
            _ => true,
        }
    }

    pub fn all_providers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("OpenAI", "GPT-4o, GPT-5"),
            ("Anthropic", "Claude 3.5, Claude 4"),
            ("Gemini", "gemini-2.0-flash, gemini-1.5-pro"),
            ("Groq", "Llama 3.2, Mixtral"),
            ("Mimo", "mimo-2.5"),
            ("DeepSeek", "deepseek-chat"),
            ("Google AI Studio", "gemini-pro"),
            ("Ollama", "Local models (no key needed)"),
            ("Custom", "Any OpenAI-compatible endpoint"),
        ]
    }
}
