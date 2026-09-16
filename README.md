# CF-VOID v3.0

CF-VOID is a comprehensive offensive security platform built in Rust, designed for penetration testing and security research.

## Features

- **40+ Web Vulnerability Scanners** - SQLi, XSS, CMDi, LFI, SSRF, XXE, IDOR, SSTI
- **12 Exploit Modules** - JWT, GraphQL, Deserialization, HTTP Smuggling
- **50+ Payload Generators** - Windows, Linux, macOS, Cloud, Android
- **16 Hash Algorithms** - MD5, SHA1-512, SHA3, bcrypt, NTLM
- **Advanced Network Scanning** - SYN, UDP, ACK, FIN, Subnet scanning
- **URL Fuzzing** - Directory, Parameter, JS Endpoint discovery
- **Post-Exploitation** - Persistence, Privesc, Lateral Movement
- **Interactive TUI** - Full terminal UI with tabs and keyboard navigation

## Quick Start

```bash
# Install
cargo build --release
cp target/release/cf-void /usr/local/bin/

# Basic usage
cf-void -u https://target.com --sqli
cf-void -u https://target.com --web-all
cf-void --fuzz --url https://target.com
cf-void --tui
```

## Modules

| Category | Modules |
|----------|---------|
| Web Scanning | SQLi, XSS, CMDi, LFI, SSRF, XXE, IDOR, SSTI |
| Network | Port Scanner, Subnet Scanner, Banner Grab, OS Fingerprint |
| Exploits | JWT, GraphQL, Deserialization, HTTP Smuggling, Subdomain Takeover |
| Payloads | Windows (EXE/DLL/HTA/MSI), Linux (ELF), Cloud (AWS/GCP/Azure) |
| Post-Exploit | Persistence, Privesc, Credentials, Lateral Movement |
| Password | Hash Cracker (16 algorithms), Brute Force |

## Keyboard Shortcuts (TUI)

- `t`/`T` - Next/Previous tab
- `j`/`k` - Select module
- `i` - Input target URL
- `a` - Attack with selected module
- `s` - Add test session
- `q` - Quit

## Author

**IND 'CYBER-FORCE'** - Offensive Security Platform

## License

GPL-3.0
