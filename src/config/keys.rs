// CF-VOID API Keys Configuration
// Author: CYBER-FORSE
// File: ~/.cf-void/keys.toml (chmod 600)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysConfig {
    #[serde(default)]
    pub openai: KeyEntry,
    #[serde(default)]
    pub anthropic: KeyEntry,
    #[serde(default)]
    pub gemini: KeyEntry,
    #[serde(default)]
    pub groq: KeyEntry,
    #[serde(default)]
    pub mimo: KeyEntry,
    #[serde(default)]
    pub deepseek: KeyEntry,
    #[serde(default)]
    pub google_ai_studio: KeyEntry,
    #[serde(default)]
    pub ollama: OllamaEntry,
    #[serde(default)]
    pub custom: CustomEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaEntry {
    #[serde(default = "default_ollama_url")]
    pub url: String,
    #[serde(default = "default_ollama_model")]
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub model: String,
}

impl Default for KeyEntry {
    fn default() -> Self {
        Self { key: String::new(), model: String::new() }
    }
}

impl Default for OllamaEntry {
    fn default() -> Self {
        Self {
            url: default_ollama_url(),
            model: default_ollama_model(),
        }
    }
}

impl Default for CustomEntry {
    fn default() -> Self {
        Self {
            name: String::new(),
            key: String::new(),
            endpoint: String::new(),
            model: String::new(),
        }
    }
}

impl Default for KeysConfig {
    fn default() -> Self {
        Self {
            openai: KeyEntry::default(),
            anthropic: KeyEntry::default(),
            gemini: KeyEntry::default(),
            groq: KeyEntry::default(),
            mimo: KeyEntry::default(),
            deepseek: KeyEntry::default(),
            google_ai_studio: KeyEntry::default(),
            ollama: OllamaEntry::default(),
            custom: CustomEntry::default(),
        }
    }
}

fn default_ollama_url() -> String { "http://localhost:11434".to_string() }
fn default_ollama_model() -> String { "llama3.2".to_string() }

impl KeysConfig {
    pub fn keys_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".cf-void");
        path.push("keys.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::keys_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<Self>(&content) {
                return config;
            }
        }
        // Create template if not exists
        let template = Self::template();
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if fs::write(&path, template).is_ok() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(mut perms) = fs::metadata(&path).map(|m| m.permissions()) {
                    perms.set_mode(0o600);
                    let _ = fs::set_permissions(&path, perms);
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::keys_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = fs::metadata(&path).map(|m| m.permissions()) {
                perms.set_mode(0o600);
                fs::set_permissions(&path, perms)?;
            }
        }
        Ok(())
    }

    pub fn get_key(&self, provider: &str) -> Option<&str> {
        match provider {
            "openai" => if !self.openai.key.is_empty() { Some(&self.openai.key) } else { None },
            "anthropic" => if !self.anthropic.key.is_empty() { Some(&self.anthropic.key) } else { None },
            "gemini" => if !self.gemini.key.is_empty() { Some(&self.gemini.key) } else { None },
            "groq" => if !self.groq.key.is_empty() { Some(&self.groq.key) } else { None },
            "mimo" => if !self.mimo.key.is_empty() { Some(&self.mimo.key) } else { None },
            "deepseek" => if !self.deepseek.key.is_empty() { Some(&self.deepseek.key) } else { None },
            "google" => if !self.google_ai_studio.key.is_empty() { Some(&self.google_ai_studio.key) } else { None },
            "custom" => if !self.custom.key.is_empty() { Some(&self.custom.key) } else { None },
            _ => None,
        }
    }

    pub fn get_model(&self, provider: &str) -> String {
        match provider {
            "openai" => self.openai.model.clone(),
            "anthropic" => self.anthropic.model.clone(),
            "gemini" => self.gemini.model.clone(),
            "groq" => self.groq.model.clone(),
            "mimo" => self.mimo.model.clone(),
            "deepseek" => self.deepseek.model.clone(),
            "google" => self.google_ai_studio.model.clone(),
            "ollama" => self.ollama.model.clone(),
            "custom" => self.custom.model.clone(),
            _ => String::new(),
        }
    }

    pub fn get_endpoint(&self, provider: &str) -> Option<String> {
        match provider {
            "openai" => Some("https://api.openai.com/v1/chat/completions".to_string()),
            "anthropic" => Some("https://api.anthropic.com/v1/messages".to_string()),
            "gemini" => Some("https://generativelanguage.googleapis.com/v1beta/openai/chat/completions".to_string()),
            "groq" => Some("https://api.groq.com/openai/v1/chat/completions".to_string()),
            "mimo" => Some("https://api.mimo.ai/v1/chat/completions".to_string()),
            "deepseek" => Some("https://api.deepseek.com/v1/chat/completions".to_string()),
            "google" => Some("https://generativelanguage.googleapis.com/v1beta/openai/chat/completions".to_string()),
            "ollama" => Some(self.ollama.url.clone()),
            "custom" => Some(self.custom.endpoint.clone()),
            _ => None,
        }
    }

    fn template() -> String {
        r#"# CF-VOID API Keys
# Author: IND 'CYBER-FORCE'
# WARNING: Keep this file secure! chmod 600
# This file stores your AI provider API keys separately from config.

[openai]
key = ""
model = "gpt-4o"

[anthropic]
key = ""
model = "claude-3-5-sonnet-20241022"

[gemini]
key = ""
model = "gemini-2.0-flash"

[groq]
key = ""
model = "llama-3.2-90b-vision-preview"

[mimo]
key = ""
model = "mimo-2.5"

[deepseek]
key = ""
model = "deepseek-chat"

[google_ai_studio]
key = ""
model = "gemini-pro"

[ollama]
url = "http://localhost:11434"
model = "llama3.2"

[custom]
name = ""
key = ""
endpoint = ""
model = ""
"#
        .to_string()
    }
}

pub fn print_keys_help() {
    let g = (218, 165, 32);
    let t = (38, 166, 154);
    let az = (42, 157, 223);
    let cr = (220, 50, 47);
    let dim = (128, 128, 128);

    println!();
    println!("\x1B[38;2;{};{};{}m  ╔═══════════════════════════════════════════════════════════╗\x1B[0m", g.0, g.1, g.2);
    println!("\x1B[38;2;{};{};{}m  ║  {} :: {}                            ║\x1B[0m", g.0, g.1, g.2,
        format!("\x1B[1m\x1B[38;2;{};{};{}m{}\x1B[0m", cr.0, cr.1, cr.2, "AI PROVIDERS"),
        format!("\x1B[38;2;{};{};{}m{}", t.0, t.1, t.2, "Available in AI Attack Mode")
    );
    println!("\x1B[38;2;{};{};{}m  ║  {}                          ║\x1B[0m", g.0, g.1, g.2,
        "  IND 'CYBER-FORCE' :: CF-VOID"
    );
    println!("\x1B[38;2;{};{};{}m  ╠═══════════════════════════════════════════════════════════╣\x1B[0m", g.0, g.1, g.2);
    println!("\x1B[38;2;{};{};{}m  ║  Supported Providers:                                        ║\x1B[0m", g.0, g.1, g.2);
    println!("\x1B[38;2;{};{};{}m  ║    [OpenAI]      GPT-4, GPT-5                               ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Anthropic]   Claude 3.5, Claude 4                         ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Gemini]      gemini-2.0-flash, gemini-1.5-pro            ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Groq]        llama-3, mixtral, gemma                   ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Mimo]        mimo-2.5                                    ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [DeepSeek]    deepseek-chat                               ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Google AI]   gemini-pro (Google AI Studio)              ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Ollama]      Local models (no API key needed)            ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║    [Custom]      Any OpenAI-compatible endpoint              ║\x1B[0m", t.0, t.1, t.2);
    println!("\x1B[38;2;{};{};{}m  ║  Add your API key in: {}                        ║\x1B[0m",
        g.0, g.1, g.2,
        format!("\x1B[38;2;{};{};{}m~ / . c f - v o i d / k e y s . t o m l", az.0, az.1, az.2)
    );
    println!("\x1B[38;2;{};{};{}m  ╚═══════════════════════════════════════════════════════════╝\x1B[0m", g.0, g.1, g.2);
    println!("\x1B[38;2;{};{};{}m  [i] Tip: Ollama runs locally, no internet/API key required\x1B[0m", dim.0, dim.1, dim.2);
    println!();
}
