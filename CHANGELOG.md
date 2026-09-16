# Changelog

All notable changes to this project will be documented in this file.

## [5.0] - 2024-09-16

### Added
- **Big CF-VOID ASCII art banner** with gold/crimson/teal color scheme for interactive mode
- **Progress bars** with `█` and `▱` characters throughout the tool (banner loading, APK generation, proxy scraping, device info)
- **Short CLI aliases**: `-S` (sqli), `-Z` (xss), `-L` (lfi), `-R` (reverse-shell), `-A` (apk-payload), `-E` (exe-payload), `-w` (wordlist), `-p` (proxy), `-b` (brute-force), `-n` (listen), `-s` (sessions), `-L` (list-templates), `-M` (list-emails)
- **Interactive payload builders** for Windows, Linux, and macOS with progress bars and build instructions
  - Windows: EXE, DLL, HTA, MSI, PowerShell, VBA, COM objects
  - Linux: ELF, cron, systemd, bash/nc/python reverse shells
  - macOS: Mach-O, AppleScript, Python, Bash reverse shells
- **Email templates**: 11 HTML email templates (birthday, love, offer, card, prize, invoice, shipping, bank, crypto, tax, meeting)
- **Phishing templates**: 43+ templates including 25 new social engineering templates (birthday, love, offer, wedding, halloween, valentine, tax, bank, crypto, etc.)
- **Shell AI commands**:
  - `ai menu` - Interactive AI attack menu
  - `ai config` - Configure AI providers with API keys
  - `ai attack <url> <provider>` - Run AI-powered attack
  - `ai chat` - Chat with AI assistant
  - `ai providers` - List available AI providers
  - `ai logs` - View AI activity logs
  - `ai chain` - Custom AI attack chain builder
- **Cross-platform detection**: Termux, Kali, Parrot, Ubuntu, Debian, Windows, macOS
- **Enhanced help command** with 7 categorized sections
- **GitHub Actions release workflow** for automated builds
- **Comprehensive README** with badges, features, and documentation
- **LICENSE** file (MIT)

### Changed
- Updated `banner.rs` to make color constants and `tc()` function `pub` for cross-module use
- Updated `list_templates` to also list email templates
- Updated shell prompt with two-line format (`┌──(CF➤VOID)` and `└─₹`)
- Updated APK payload generator to show 7-step progress bar with file names
- Progress bars updated from `=`/`-` to `█`/`▱` characters
- `CliArgs` now derives `Default` for programmatic construction

### Removed
- Battery feature from device info (unreliable on most platforms)
- Old `print_startup_screen()` function from main.rs

## [4.0] - 2024-09-14

### Added
- Base CF-VOID platform with 37 attack modules
- Interactive shell mode
- Full TUI interface (7 tabs: Dashboard, Scanning, Terminal, Payloads, AI, Device, Config)
- 9 AI providers (OpenAI, Anthropic, Gemini, Groq, Mimo, DeepSeek, Google AI Studio, Ollama, Custom)
- AI attack engine with recon/plan/execute pipeline
- 18 phishing templates
- Payload generators for Windows, Linux, macOS, Android
- Hash cracking (16 algorithms)
- Network scanning (SYN, UDP, ACK, FIN, subnet)
- URL fuzzing with rate limiting
- Post-exploitation modules
- Session management and reverse shell listeners

## [3.0] - 2024-08-20

Initial public release with:
- 40+ web vulnerability scanners
- 12 exploit modules
- 50+ payload generators
- 16 hash algorithms
