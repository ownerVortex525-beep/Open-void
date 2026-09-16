# CF-VOID v5.0

**CF-VOID** is a comprehensive offensive security platform built in Rust, designed for penetration testing and security research. It provides a complete toolkit for ethical hackers, security researchers, and red teams.

## Key Features

- **40+ Web Vulnerability Scanners** - SQLi, XSS, CMDi, LFI, SSRF, XXE, IDOR, SSTI
- **20+ Exploit Modules** - JWT, GraphQL, Deserialization, HTTP Smuggling, MS17-010, Redis
- **50+ Payload Generators** - Windows, Linux, macOS, Android APK, Cloud, Fileless
- **16 Hash Algorithms** - MD5, SHA1-512, SHA3, bcrypt, NTLM, Python hash, MySQL
- **Advanced Network Scanning** - SYN, UDP, ACK, FIN, Subnet scanning, Banner grab
- **URL Fuzzing** - Directory, Parameter, JS Endpoint discovery (429 handling)
- **Post-Exploitation** - Persistence, Privesc, Lateral Movement, Password Cracking
- **Interactive TUI** - Full terminal UI with tabs and keyboard navigation
- **Interactive Shell** - Command shell with all tool features accessible
- **AI-Powered Attacks** - GPT-4, Claude, Gemini integrations for recon/plan/execute
- **Phishing Toolkit** - 43+ phishing page templates, 11 email templates, custom builders
- **Proxy Scraping** - 15+ sources, auto-reconnect, latency testing
- **Subdomain Enumeration** - Multiple sources, port scanning integration
- **Cross-Platform Support** - Termux, Kali, Parrot, Ubuntu, Debian, Windows, macOS

## Quick Start

```bash
# Build from source
cargo build --release

# Install
cp target/release/cf-void /usr/local/bin/

# Interactive shell
cf-void

# Quick scan with short aliases
cf-void -u https://target.com -S        # SQLi scan
cf-void -u https://target.com -Z        # XSS scan
cf-void -u https://target.com -L        # LFI scan

# Full web scan
cf-void -u https://target.com --web-all

# Fuzzing
cf-void -u https://target.com --fuzz

# Interactive TUI
cf-void --tui

# AI attack
cf-void -u https://target.com --ai --ai-provider openai

# Generate phishing page
cf-void --phish-template login -l 127.0.0.1 8080

# Generate phishing email
cf-void --phish-email birthday -l 127.0.0.1 8080

# Reverse shell payload
cf-void -R --lhost 127.0.0.1 --lport 4444 --payload-lang bash

# APK payload
cf-void -A --lhost 127.0.0.1 --lport 4444

# EXE payload
cf-void -E --lhost 127.0.0.1 --lport 4444

# Password cracking
cf-void --crack-hash "5f4dcc3b5aa765d61d8327deb882cf99" --hash-type md5

# Wordlist generation
cf-void -w passwords

# Start listener
cf-void -n --lport 4444

# List sessions
cf-void -s
```

## Interactive Shell Commands

```
└─₹ help          Show comprehensive help with all commands
└─₹ ai menu       Interactive AI attack menu
└─₹ ai config     Configure AI providers (API keys)
└─₹ ai attack     Run AI-powered attack on target
└─₹ ai chat       Chat with AI assistant
└─₹ ai providers  List available AI providers
└─₹ ai logs       View AI activity logs
└─₹ ai chain      Build custom AI attack chain
└─₳ scan -u <url> --xss    Scan target for XSS
└─₳ exploit -u <url>       Run exploits
└─₳ payload --reverse-shell  Generate reverse shell
└─₳ phish --phish-template   Generate phishing page
└─₳ list-templates           List all phishing templates
└─₳ list-emails              List email templates
└─₳ modules                 List all modules
└─₳ stats                   Show tool statistics
└─₳ tui                     Launch interactive TUI
└─₯ exit                    Exit shell
```

## Short CLI Aliases

| Short Flag | Long Flag | Description |
|-----------|-----------|-------------|
| `-S` | `--sqli` | SQL Injection scan |
| `-Z` | `--xss` | Cross-Site Scripting |
| `-L` | `--lfi` | Local File Inclusion |
| `-R` | `--reverse-shell` | Reverse shell payload |
| `-A` | `--apk-payload` | APK payload generator |
| `-E` | `--exe-payload` | Windows EXE payload |
| `-w` | `--wordlist` | Wordlist generator |
| `-p` | `--proxy` | Proxy configuration |
| `-b` | `--brute-force` | Brute force attack |
| `-n` | `--listen` | Start listener |
| `-s` | `--sessions` | List sessions |
| `-L` | `--list-templates` | List phishing templates |
| `-M` | `--list-emails` | List email templates |

## Phishing Templates

### Web Page Templates (43+)
**Social Engineering:** birthday, love, offer, card, prize, wedding, baby_shower, christmas, halloween, valentine, fathers_day, mothers_day, new_year, thanksgiving, easter, resume, job_offer, invoice, shipping, tax, bank, crypto, social_media, cloud_storage, meeting

**Authentication:** login, google, facebook, twitter, instagram, github, microsoft, apple, yahoo, linkedin, netflix, amazon, paypal, coinbase, dropbox, salesforce, jira, slack, zoom, twitch, reddit, snapchat, microsoft-outlook, office-365, github-enterprise, aws, azure, digitalocean, server-login

### Email Templates (11)
birthday, love, offer, card, prize, invoice, shipping, bank, crypto, tax, meeting

## AI Providers

| Provider | Models | Key Required |
|----------|--------|--------------|
| OpenAI | GPT-4o, GPT-4-turbo | Yes |
| Anthropic | Claude-3-Opus, Claude-3.5-Sonnet | Yes |
| Gemini | Gemini-2.0-flash, Gemini-1.5-pro | Yes |
| Groq | Llama-3-70B, Mixtral-8x7B | Yes |
| Mimo | mimo-2.5 | Yes |
| DeepSeek | deepseek-chat | Yes |
| Google AI Studio | gemini-pro | Yes |
| Ollama | Local models (phi3, mistral, llama3) | No |
| Custom | Any OpenAI-compatible endpoint | Optional |

## TUI Interface

Access the full TUI interface with `cf-void --tui`:

- **Dashboard** - Tool stats, module list, recent activity
- **Scanning** - Web scanners, network, fuzzing with progress
- **Terminal** - Live shell with command history
- **Payloads** - Interactive payload builders for all platforms
- **AI** - AI attack menu, chat, provider config, logs
- **Device** - System info, platform detection
- **Config** - API keys, settings, proxy

## Build & Installation

```bash
git clone https://github.com/ownerVortex525-beep/Open-void.git
cd Open-void
cargo build --release
cp target/release/cf-void /usr/local/bin/cf-void
```

## Requirements

- Rust 1.75+
- Cargo
- (Optional) API keys for AI providers
- (Optional) Python for some payload modules
- (Optional) Metasploit, Nmap for advanced scanning

## License

This tool is for educational and authorized security testing purposes only. The author is not responsible for any misuse or damage caused by this tool.

## Author

**CYBER-FORCE** - IND 'CYBER-FORCE'
