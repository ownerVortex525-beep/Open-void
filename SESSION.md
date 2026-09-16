# CF-VOID Session ID: cfvoid-build-2026-09-13

## Status: ✅ BUILD COMPLETE

## Binary
- **Name**: `cf-void`
- **Location**: `/data/data/com.termux/files/usr/bin/cf-void`
- **Source**: `/data/data/com.termux/files/home/cfvoid/`
- **Size**: ~86MB (debug build)

## Working Commands
```bash
# Show help
cf-void --help

# List all modules
cf-void --list-modules

# Web attacks
cf-void -u https://target.com --sqli
cf-void -u https://target.com --xss
cf-void -u https://target.com --lfi
cf-void -u https://target.com --ssti
cf-void -u https://target.com --cmdi
cf-void -u https://target.com --ssrf
cf-void -u https://target.com --xxe
cf-void -u https://target.com --idor
cf-void -u https://target.com --cors
cf-void -u https://target.com --redirect
cf-void -u https://target.com --clickjack
cf-void -u https://target.com --web-all

# Network attacks
cf-void -u https://target.com --ports full
cf-void -u https://target.com --ports common
cf-void -u https://target.com --os-detect

# Credential attacks
cf-void -u https://target.com --brute-http
cf-void -u https://target.com --default-creds
cf-void -u https://target.com --creds-all

# DNS attacks
cf-void -u https://target.com --subdomain-brute
cf-void -u https://target.com --dns-enum
cf-void -u https://target.com --dns-all

# OSINT
cf-void -u https://target.com --osint-subdomains
cf-void -u https://target.com --osint-emails
cf-void -u https://target.com --osint-github
cf-void -u https://target.com --osint-all

# Social Engineering
cf-void -u https://target.com --phish
cf-void -u https://target.com --payload-gen --payload-type reverse-shell

# Post-Exploitation
cf-void -u https://target.com --check-persistence
cf-void -u https://target.com --persist
cf-void -u https://target.com --privesc
cf-void -u https://target.com --post-all

# Report
cf-void -u https://target.com --web-all -o report.html -f html
cf-void -u https://target.com --web-all -o report.json -f json
```

## Implemented Scanners
- ✅ SQLi (13 payloads + UNION detection)
- ✅ XSS (10 payloads)
- ✅ LFI (10 payloads + markers)
- ✅ SSTI (6 payloads)
- ✅ CMDi (8 payloads)
- ✅ SSRF (5 internal targets)
- ✅ XXE (3 XML payloads)
- ✅ IDOR (user enumeration)
- ✅ CORS (5 evil origins)
- ✅ Open Redirect (5 bypasses)
- ✅ Clickjacking (header check)
- ✅ Port Scanner (async multi-threaded)
- ✅ Service Detection (20+ services)
- ✅ OS Fingerprint (HTTP headers)
- ✅ HTTP Brute Force (default + custom wordlist)
- ✅ Default Credentials
- ✅ Subdomain Brute Force (50+ subs)
- ✅ DNS Enumeration
- ✅ crt.sh Subdomain Discovery
- ✅ Email Harvesting
- ✅ GitHub Dorking
- ✅ Social Media Check
- ✅ Phishing Page Generator
- ✅ Payload Generator (reverse/bind shell)
- ✅ Post-Exploitation (persistence/privesc/lateral/exfil/cleanup)
- ✅ Report Generation (HTML/JSON/CSV/Markdown)

## Resume Session
To resume development:
```bash
cd /data/data/com.termux/files/home/cfvoid
# Edit source files
# Rebuild: CARGO_BUILD_JOBS=1 cargo build
# Install: cp target/debug/cf-void /data/data/com.termux/files/usr/bin/cf-void
```
