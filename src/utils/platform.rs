// CF-VOID Platform Utility Module
// Author: CYBER-FORCE
// Cross-platform detection and platform-specific helpers

use crate::cli::banner;

pub enum Platform {
    Termux,
    Kali,
    Parrot,
    Ubuntu,
    Debian,
    Windows,
    MacOS,
    Other(String),
}

impl Platform {
    pub fn detect() -> Self {
        #[cfg(target_os = "android")]
        {
            if std::env::var("PREFIX").map_or(false, |p| p.contains("com.termux")) {
                return Platform::Termux;
            }
            return Platform::Other("Android".to_string());
        }

        #[cfg(target_os = "windows")]
        {
            return Platform::Windows;
        }

        #[cfg(target_os = "macos")]
        {
            return Platform::MacOS;
        }

        #[cfg(target_os = "linux")]
        {
            // Check for Linux distributions
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if let Some(name) = line.strip_prefix("ID=") {
                        let name = name.trim().trim_matches('"').to_lowercase();
                        match name.as_str() {
                            "termux" => return Platform::Termux,
                            "kali" => return Platform::Kali,
                            "parrot" => return Platform::Parrot,
                            "ubuntu" => return Platform::Ubuntu,
                            "debian" => return Platform::Debian,
                            _ => return Platform::Other(name),
                        }
                    }
                }
            }
            // Check for Termux specifically
            if std::env::var("PREFIX").map_or(false, |p| p.contains("com.termux")) {
                return Platform::Termux;
            }
            return Platform::Other("Linux".to_string());
        }

        #[cfg(not(any(target_os = "android", target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Platform::Other("Unknown".to_string())
        }
    }

    pub fn name(&self) -> String {
        match self {
            Platform::Termux => "Termux".to_string(),
            Platform::Kali => "Kali Linux".to_string(),
            Platform::Parrot => "Parrot OS".to_string(),
            Platform::Ubuntu => "Ubuntu".to_string(),
            Platform::Debian => "Debian".to_string(),
            Platform::Windows => "Windows".to_string(),
            Platform::MacOS => "macOS".to_string(),
            Platform::Other(s) => s.clone(),
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        use crate::cli::banner::{GREEN, GOLD, TEAL};
        match self {
            Platform::Termux | Platform::Kali => GOLD,
            Platform::Parrot | Platform::Ubuntu => TEAL,
            Platform::Debian | Platform::MacOS => GREEN,
            Platform::Windows => GOLD,
            _ => (128, 128, 128),
        }
    }

    pub fn print_banner_message(&self) {
        let color = self.color();
        let name = self.name();
        match self {
            Platform::Termux => {
                banner::success(&format!("Running on Termux - Full feature set available"));
            }
            Platform::Kali => {
                banner::success(&format!("Running on Kali Linux - Full feature set available"));
            }
            Platform::Parrot => {
                banner::success(&format!("Running on Parrot OS - Full feature set available"));
            }
            Platform::Ubuntu | Platform::Debian => {
                banner::info(&format!("Running on {} - Full feature set available", name));
            }
            Platform::Windows => {
                banner::warning(&format!("Running on Windows - Some features may require WSL"));
            }
            Platform::MacOS => {
                banner::info(&format!("Running on macOS - Full feature set available"));
            }
            _ => {
                banner::info(&format!("Running on {} - Standard feature set", name));
            }
        }
    }

    pub fn tool_available(&self, tool: &str) -> bool {
        match self {
            Platform::Termux => {
                matches!(tool, "nmap" | "netcat" | "metasploit" | "sqlmap" | "nikto" | "dirb" | "gobuster" | "hydra" | "john" | "aircrack-ng" | "wireshark" | "burp" | "ffuf" | "wfuzz" | "dirbuster" | "masscan" | "zgrab" | "amass" | "sublist3r" | "rustscan" | "naabu" | "httpx")
            }
            Platform::Kali | Platform::Parrot | Platform::Ubuntu | Platform::Debian => {
                true
            }
            Platform::Windows => {
                matches!(tool, "nmap" | "netcat" | "sqlmap" | "nikto" | "burp" | "ffuf" | "wfuzz" | "amass" | "httpx")
                    || tool.starts_with("python") || tool.starts_with("ruby")
            }
            Platform::MacOS => {
                true
            }
            _ => false,
        }
    }

    pub fn install_hint(&self, tool: &str) -> Option<String> {
        match self {
            Platform::Termux => {
                Some(format!("pkg install -y {}", tool))
            }
            Platform::Kali | Platform::Parrot => {
                Some(format!("sudo apt install -y {}", tool))
            }
            Platform::Ubuntu | Platform::Debian => {
                Some(format!("sudo apt install -y {}", tool))
            }
            Platform::Windows => {
                Some(format!("choco install {}  (or use WSL for full support)", tool))
            }
            Platform::MacOS => {
                Some(format!("brew install {}", tool))
            }
            _ => None,
        }
    }
}

pub fn print_platform_info() {
    let platform = Platform::detect();
    let color = platform.color();
    banner::info(&format!(
        "{} Platform: {}",
        crate::cli::banner::tc("▌", color),
        platform.name()
    ));
}

pub fn check_tool(tool: &str) -> bool {
    let platform = Platform::detect();
    if !platform.tool_available(tool) {
        banner::warning(&format!("Tool '{}' may not be available on {}", tool, platform.name()));
        if let Some(hint) = platform.install_hint(tool) {
            banner::info(&format!("Install with: {}", hint));
        }
        return false;
    }
    true
}
