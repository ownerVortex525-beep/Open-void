// CF-VOID Interactive Shell Module
// Author: CYBER-FORCE

use std::io::{self, Write, BufRead};
use std::process;
use clap::Parser;

use crate::cli::args::CliArgs;
use crate::cli::banner;

const GOLD: (u8, u8, u8) = (218, 165, 32);
const TEAL: (u8, u8, u8) = (38, 166, 154);
const AZURE: (u8, u8, u8) = (42, 157, 223);
const CRIMSON: (u8, u8, u8) = (220, 50, 47);
const YELLOW: (u8, u8, u8) = (255, 193, 7);


fn tc(s: &str, (r, g, b): (u8, u8, u8)) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, s)
}

fn bold_tc(s: &str, c: (u8, u8, u8)) -> String {
    format!("\x1B[1m\x1B[38;2;{};{};{}m{}\x1B[0m", c.0, c.1, c.2, s)
}

pub fn run_shell() {
    print_shell_banner();

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print_prompt();
        io::stdout().flush().unwrap_or(());

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                process_shell_command(input);
            }
            Err(e) => {
                banner::error(&format!("Read error: {}", e));
                break;
            }
        }
    }

    println!();
    println!("  {} {}", tc("▐", GOLD), tc("Goodbye! Stay sharp, operator.", TEAL));
    println!();
}

fn print_shell_banner() {
    println!();
    println!("{}───▄▀▀▀▄▄▄▄▄▄▄▀▀▀▄───{}", tc("▐", GOLD), "\x1B[0m");
    println!("{}───█▒▒░░░░░░░░░▒▒█───{}", tc("▐", GOLD), "\x1B[0m");
    println!("{}───█░░█░░░░░█░░█────{}", tc("▐", GOLD), "\x1B[0m");
    println!("{}─▄▄──█░░░▀█▀░░░█──▄▄─{}", tc("▐", GOLD), "\x1B[0m");
    println!("{}█░░█─▀▄░░░░░░░▄▀─█░░█{}", tc("▐", GOLD), "\x1B[0m");
    println!("{}█▀░▀▀▀▀▄▄▄░▄▄▄▀▀▀▀░▀█{}", tc("▐", GOLD), "\x1B[0m");
    println!();
    println!("  {} Welcome to CF-VOID Interactive Shell", bold_tc("▐━━▌", GOLD));
    crate::utils::platform::print_platform_info();
    println!("  {} Type {} for available commands, {} to exit", tc("➥", GOLD), bold_tc("'help'", TEAL), bold_tc("'exit'", CRIMSON));
    println!();
}

fn print_prompt() {
    println!();
    print!("{}{}{}\n{} ",
        tc("┌──(", AZURE),
        bold_tc("CF➤VOID", AZURE),
        tc(")", AZURE),
        tc("└─₹ ", GOLD)
    );
}

fn process_shell_command(input: &str) {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args_str = parts.get(1).unwrap_or(&"").trim();

    match cmd.as_str() {
        "help" | "h" | "?" => show_help(),
        "exit" | "quit" | "q" | "ctrl+c" => {
            println!();
            println!("  {} {}", tc("▐", GOLD), tc("Goodbye! Stay sharp, operator.", TEAL));
            println!();
            process::exit(0);
        }
        "scan" | "s" => {
            if args_str.is_empty() {
                banner::error("Usage: scan -u <url> --xss (use 'help' for full commands)");
                return;
            }
            run_tool_command(args_str);
        }
        "exploit" | "e" => {
            if args_str.is_empty() {
                banner::error("Usage: exploit -u <url> --exploit-xss");
                return;
            }
            run_tool_command(args_str);
        }
        "payload" | "p" => {
            if args_str.is_empty() {
                banner::error("Usage: payload --reverse-shell --lhost <ip> --lport <port>");
                return;
            }
            run_tool_command(args_str);
        }
        "recon" | "r" => {
            if args_str.is_empty() {
                banner::error("Usage: recon <domain/ip>");
                banner::info("Commands: subdomain <domain> | dns <domain> | ping <host> | iplookup <ip>");
                banner::info("          netscan <cidr> | ports <host> | services <host> | whois <domain>");
                banner::info("          headers <url> | techstack <url>");
                return;
            }
            let parts2: Vec<&str> = args_str.split_whitespace().collect();
            let (subcmd, target) = (parts2[0], parts2.get(1).unwrap_or(&""));
            match subcmd {
                "subdomain" | "subs" | "sub" => run_tool_command(&format!("--subdomain-brute --target {}", target)),
                "dns" => run_tool_command(&format!("--osint-subdomains --target {}", target)),
                "ping" => run_system_command(&format!("ping -c 4 {}", target)),
                "iplookup" | "ipinfo" => {
                    let result = std::process::Command::new("curl")
                        .arg("-s").arg(format!("https://ipapi.co/{}/json/", target))
                        .output();
                    if let Ok(o) = result {
                        println!("{}", String::from_utf8_lossy(&o.stdout));
                    }
                }
                "netscan" | "scan-net" => run_tool_command(&format!("--subnet-scan --target {}", target)),
                "ports" => run_tool_command(&format!("--ports quick --target {}", target)),
                "services" => run_tool_command(&format!("--banner-grab --target {}", target)),
                "whois" => run_system_command(&format!("whois {}", target)),
                "headers" => run_system_command(&format!("curl -sI {}", target)),
                "techstack" => run_system_command(&format!("whatweb -v {}", target)),
                _ => run_tool_command(args_str),
            }
        }
        "crack" | "c" => {
            if args_str.is_empty() {
                banner::error("Usage: crack --crack-hash <hash> --hash-type md5");
                return;
            }
            run_tool_command(args_str);
        }
        "fuzz" | "f" => {
            if args_str.is_empty() {
                banner::error("Usage: fuzz -u <url> --fuzz");
                return;
            }
            run_tool_command(args_str);
        }
        "network" | "n" => {
            if args_str.is_empty() {
                banner::error("Usage: network -t <ip> --ports quick");
                return;
            }
            run_tool_command(args_str);
        }
        "listen" | "l" => {
            if args_str.is_empty() {
                run_tool_command("--listen --lport 4444");
            } else {
                run_tool_command(&format!("--listen {}", args_str));
            }
        }
        "sessions" | "session" => run_tool_command("--sessions"),
        "phish" | "phishing" => {
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            if parts.is_empty() {
                banner::error("Usage: phish serve <template> [--local <port>] [--tunnel <type>]");
                return;
            }
            match parts.get(1) {
                Some(&"serve") | Some(&"s") => {
                    let template = parts.get(2).map(|s| s.to_string()).unwrap_or("login".to_string());
                    let port = 8080;
                    let mut server = crate::phishing::PhishingServer::new("0.0.0.0", port, &template);
                    if parts.iter().any(|&p| p == "--local") {
                        // local only
                    }
                    for (i, p) in parts.iter().enumerate() {
                        if *p == "--tunnel" && i + 1 < parts.len() {
                            server = server.with_tunnel(parts[i + 1]);
                        }
                    }
                    let _ = server.start();
                }
                _ => {
                    banner::error("Usage: phish serve <template> [--local <port>] [--tunnel <type>]");
                }
            }
        }
        "serve" => {
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            let template = parts.first().map(|s| s.to_string()).unwrap_or("login".to_string());
            let port: u16 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(8080);
            let server = crate::phishing::PhishingServer::new("0.0.0.0", port, &template);
            let _ = server.start();
        }
        "tui" => {
            if let Err(e) = crate::cli::tui::run_tui() {
                banner::error(&format!("TUI error: {}", e));
            }
        }
        "banner" => print_shell_banner(),
        "stats" => banner::print_stats(),
        "modules" | "mod" => banner::print_module_list(),
        "list-templates" | "templates" => {
            crate::phishing::PhishingGen::list_templates();
            crate::phishing::list_email_templates();
        }
        "list-emails" | "emails" => {
            crate::phishing::list_email_templates();
        }
        "clear" | "cls" => {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap_or(());
        }
        "version" | "ver" => {
            println!("  {} CF-VOID", tc("▐", GOLD));
            println!("  {} IND 'CYBER-FORCE' :: Offensive Security Platform", tc("➥", TEAL));
        }
        "ls" | "pwd" | "cd" | "cat" | "echo" | "mkdir" | "rm" | "cp" | "mv" | "chmod" | "grep" | "find" | "whoami" | "ifconfig" | "ping" | "curl" | "wget" => {
            // Run as system command
            run_system_command(input);
        }
        _ => {
            // Check if it starts with "ai " for AI commands with subcommands
            if input.starts_with("ai ") {
                let ai_args = &input[3..];
                handle_ai_command_string(ai_args);
            } else if input == "ai" {
                handle_ai_command_string("");
            } else if input.starts_with("--") || input.starts_with("-u ") || input.starts_with("-t ") {
                run_tool_command(input);
            } else if input.starts_with("cf-void ") {
                let cf_args = &input[8..];
                run_tool_command(cf_args);
            } else {
                // Try to run as system command
                run_system_command(input);
            }
        }
    }
}

fn run_system_command(cmd: &str) {
    use std::process::Command;
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(o) => {
            if !o.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&o.stdout));
            }
            if !o.stderr.is_empty() {
                print!("{}", String::from_utf8_lossy(&o.stderr));
            }
        }
        Err(e) => {
            banner::error(&format!("Command failed: {}", e));
        }
    }
}

fn handle_ai_command_string(args: &str) {
    let parts: Vec<&str> = args.trim().split_whitespace().collect();
    if parts.is_empty() {
        banner::info("AI Commands:");
        println!("  {} ai menu        - Interactive AI attack menu", tc("▶", TEAL));
        println!("  {} ai config      - Configure AI providers", tc("▶", TEAL));
        println!("  {} ai attack      - Run AI-powered attack (requires --url)", tc("▶", TEAL));
        println!("  {} ai chat        - Chat with AI assistant", tc("▶", TEAL));
        println!("  {} ai providers    - List AI providers", tc("▶", TEAL));
        println!("  {} ai logs        - View AI activity logs", tc("▶", TEAL));
        println!("  {} ai chain       - Custom AI attack chain", tc("▶", TEAL));
        return;
    }

    let subcommand = parts[0].to_lowercase();

    match subcommand.as_str() {
        "menu" => {
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    crate::ai::start_interactive_ai_menu().await;
                });
            }).join().unwrap_or(());
        }
        "config" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                crate::ai::config::AiConfig::show_config_menu().await;
            });
        }
        "attack" => {
            if parts.len() < 3 {
                banner::error("Usage: ai attack <url> [provider]");
                banner::info("Example: ai attack https://example.com cerebras");
                return;
            }
            let url = parts[1].to_string();
            let provider = parts.get(2).map(|s| s.to_lowercase()).unwrap_or_else(|| "cerebras".to_string());
            
            banner::info("Starting AI attack...");
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let mut cli_args = crate::cli::args::CliArgs::default();
                    cli_args.url = Some(url);
                    cli_args.ai = true;
                    crate::core::engine::run_ai_attack_with_provider(&cli_args, &provider).await;
                });
            }).join().unwrap_or(());
        }
        "chat" => {
            banner::info("AI Chat mode - type 'exit' to quit");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                crate::ai::start_chat_mode().await;
            });
        }
        "providers" => {
            crate::ai::list_providers();
        }
        "logs" => {
            crate::ai::logs::print_ai_logs();
        }
        "chain" => {
            banner::info("AI Custom Attack Chain Builder");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                crate::ai::start_chain_builder().await;
            });
        }
        _ => {
            banner::error(&format!("Unknown AI subcommand: {}", subcommand));
            handle_ai_command_string("");
        }
    }
}

fn run_tool_command(args: &str) {
    let full_args = format!("cf-void {}", args);
    let args_vec: Vec<String> = full_args.split_whitespace().map(|s| s.to_string()).collect();

    let cli_args = match CliArgs::try_parse_from(args_vec) {
        Ok(a) => a,
        Err(e) => {
            banner::error(&format!("Invalid arguments: {}", e));
            return;
        }
    };

    // Check if we have enough to run
    if !cli_args.has_target() && !cli_args.has_any_attack() && !cli_args.sessions && !cli_args.listen && cli_args.crack_hash.is_none() {
        banner::error("No target or attack specified. Use 'help' for usage.");
        return;
    }

    banner::info(&format!("Running: cf-void {}", args));

    // Use tokio::task::spawn_blocking to avoid runtime-within-runtime panic
    let handle = tokio::runtime::Handle::current();
    std::thread::spawn(move || {
        handle.block_on(async {
            // Phishing templates
            if cli_args.list_templates {
                crate::phishing::PhishingGen::list_templates();
                return;
            }
            if let Some(template) = &cli_args.phish_template {
                let lhost = cli_args.lhost.as_deref().unwrap_or("127.0.0.1");
                let lport = cli_args.lport.as_deref().unwrap_or("8080");
                crate::phishing::PhishingGen::new().generate(template, lhost, lport, cli_args.output.as_deref());
                return;
            }

            execute_command(cli_args).await;
        });
    }).join().ok();
}

async fn execute_command(cli_args: CliArgs) {
    if cli_args.crack_hash.is_some() || cli_args.brute_force {
        crate::core::engine::run_password_cracker(&cli_args).await;
    } else if cli_args.reverse_shell || cli_args.bind_shell || cli_args.web_shell || cli_args.apk_payload || cli_args.exe_payload || cli_args.ps_variant.is_some() || cli_args.windows_payload || cli_args.linux_payload || cli_args.macos_payload || cli_args.cloud_payload || cli_args.fileless {
        crate::core::engine::run_payloads_only(&cli_args).await;
    } else if cli_args.listen {
        crate::core::engine::run_session_listener(&cli_args).await;
    } else if cli_args.sessions {
        crate::core::engine::run_session_list(&cli_args).await;
    } else     if cli_args.has_fuzz() {
        if cli_args.url.is_some() {
            crate::core::engine::run_fuzz(&cli_args).await;
        } else {
            banner::error("Fuzzing requires --url <target>");
        }
    } else if cli_args.ai {
        if cli_args.url.is_some() || cli_args.target.is_some() {
            crate::core::engine::run_ai_attack(&cli_args).await;
        } else {
            banner::error("AI attack requires --url <target>");
        }
    } else if cli_args.ports.is_some() || cli_args.banner_grab || cli_args.subnet_scan {
        if cli_args.target.is_some() {
            crate::core::engine::run_network_scan(&cli_args).await;
        } else {
            banner::error("Network scanning requires --target <ip>");
        }
    } else if cli_args.has_target() && cli_args.has_any_attack() {
        match crate::core::engine::ScanEngine::new(cli_args.clone()) {
            Ok(engine) => {
                match engine.run().await {
                    Ok(result) => {
                        if let Some(report_path) = &cli_args.report {
                            let format = cli_args.format.as_deref().unwrap_or("json");
                            let gen = crate::report::generator::ReportGenerator::new(result.clone());
                            if let Err(e) = gen.save(report_path, format) {
                                banner::error(&format!("Report save failed: {}", e));
                            } else {
                                banner::print_report_saved(report_path);
                            }
                        }
                        if let Some(output) = &cli_args.output {
                            let format = cli_args.format.as_deref().unwrap_or("json");
                            let gen = crate::report::generator::ReportGenerator::new(result.clone());
                            if let Err(e) = gen.save(output, format) {
                                banner::error(&format!("Report save failed: {}", e));
                            } else {
                                banner::success(&format!("Report saved: {}", output));
                            }
                        }
                    }
                    Err(e) => {
                        banner::error(&format!("Scan failed: {}", e));
                    }
                }
            }
            Err(e) => {
                banner::error(&format!("Initialization failed: {}", e));
            }
        }
    }
}

fn show_help() {
    let g = GOLD;
    let az = AZURE;
    let t = TEAL;
    let y = YELLOW;

    println!();
    println!("  {} CF-VOID Interactive Shell :: Commands", bold_tc("▐━━▌", g));
    println!();
    println!("  {} SCAN", bold_tc("[1]", az));
    println!("    {} scan -u <url> --xss         Scan target for XSS", tc("➥", t));
    println!("    {} scan -u <url> --sqli        Scan target for SQLi", tc("➥", t));
    println!("    {} scan -u <url> --web-all     Run ALL web scanners", tc("➥", t));
    println!("    {} scan -u <url> --lfi         Scan for LFI", tc("➥", t));
    println!("    {} scan -u <url> --cmdi        Scan for CMD injection", tc("➥", t));
    println!("    {} scan -u <url> --ssti        Scan for SSTI", tc("➥", t));
    println!("    {} scan -u <url> --ssrf        Scan for SSRF", tc("➥", t));
    println!("    {} scan -u <url> --xxe         Scan for XXE", tc("➥", t));
    println!("    {} scan -u <url> --idor        Scan for IDOR", tc("➥", t));
    println!("    {} proxy-scrape --proxy <url>  Scrape & test proxies", tc("➥", t));
    println!("    {} sublist3r -t <domain>      Find subdomains", tc("➥", t));
    println!();
    println!("  {} EXPLOIT", bold_tc("[2]", az));
    println!("    {} exploit -u <url> --exploit-xss   Exploit XSS", tc("➥", t));
    println!("    {} exploit -u <url> --exploit-sqli  Exploit SQLi", tc("➥", t));
    println!("    {} exploit -u <url> --exploit-cmdi  Exploit CMDi", tc("➥", t));
    println!("    {} exploit -u <url> --exploit-lfi   Exploit LFI", tc("➥", t));
    println!("    {} exploit -u <url> --exploit-all   Run ALL exploits", tc("➥", t));
    println!("    {} exploit -t <ip> --exploit-ms17   Exploit MS17-010", tc("➥", t));
    println!("    {} exploit -t <ip> --exploit-redis Exploit Redis", tc("➥", t));
    println!();
    println!("  {} PAYLOAD", bold_tc("[3]", az));
    println!("    {} payload --reverse-shell --lhost <ip> --lport <port> --payload-lang bash", tc("➥", t));
    println!("    {} payload --exe-payload --lhost <ip> --lport <port> --arch x64", tc("➥", t));
    println!("    {} payload --apk-payload --lhost <ip> --lport <port>", tc("➥", t));
    println!("    {} payload --ps-variant encoded --lhost <ip> --lport <port>", tc("➥", t));
    println!("    {} payload --windows         Interactive Windows payload builder", tc("➥", t));
    println!("    {} payload --linux           Interactive Linux payload builder", tc("➥", t));
    println!("    {} payload --macos           Interactive macOS payload builder", tc("➥", t));
    println!("    {} payload --cloud-payload   Cloud-based payload delivery", tc("➥", t));
    println!("    {} payload --fileless        Fileless payload generation", tc("➥", t));
    println!("    {} payload --wordlist        Generate wordlists", tc("➥", t));
    println!();
    println!("  {} CRACK", bold_tc("[4]", az));
    println!("    {} crack --crack-hash <hash> --hash-type md5", tc("➥", t));
    println!("    {} crack --crack-hash <hash> --hash-type sha256", tc("➥", t));
    println!("    {} crack --crack-hash <hash> --hash-type ntlm", tc("➥", t));
    println!("    {} crack --wordlist <file> --target <host> --brute-force", tc("➥", t));
    println!();
    println!("  {} FUZZ", bold_tc("[5]", az));
    println!("    {} fuzz -u <url> --fuzz", tc("➥", t));
    println!("    {} fuzz -u <url> --fuzz --fuzz-extensions php,txt,asp", tc("➥", t));
    println!();
    println!("  {} NETWORK", bold_tc("[6]", az));
    println!("    {} subfinder <domain>             Subdomain enumeration", tc("➥", t));
    println!("    {} dns <domain>                  DNS records lookup", tc("➥", t));
    println!("    {} ping <host>                  ICMP reachability + latency", tc("➥", t));
    println!("    {} iplookup <ip>               Geolocation + ASN + ISP info", tc("➥", t));
    println!("    {} netscan <ip/CIDR>          Host discovery + port scan", tc("➥", t));
    println!("    {} ports <host>                Port + service/version detection", tc("➥", t));
    println!("    {} services <host>            Service banner grabbing", tc("➥", t));
    println!("    {} whois <domain>             WHOIS registration data", tc("➥", t));
    println!("    {} headers <url>              HTTP + security headers", tc("➥", t));
    println!("    {} techstack <url>           Detect web technologies", tc("➥", t));
    println!("    {} network -t <ip> --ports quick  Scan common ports", tc("➥", t));
    println!("    {} network -t <ip> --ports full   Scan all ports", tc("➥", t));
    println!("    {} network -t <ip> --banner-grab  Grab service banners", tc("➥", t));
    println!("    {} network -t <ip> --subnet <cidr> Scan subnet range", tc("➥", t));
    println!("    {} portfwd add -L <local> -l <lport> -r <remote> -p <rport>  Port forward", tc("➥", t));
    println!("    {} portfwd list            List active port forwards", tc("➥", t));
    println!("    {} portfwd delete <id>    Delete a port forward", tc("➥", t));
    println!();
    println!("  {} PHISHING", bold_tc("[7]", az));
    println!("    {} phish --phish-template <name>    Generate phishing page", tc("➥", t));
    println!("    {} list-templates                   List all phishing templates", tc("➥", t));
    println!("    {} phish --phish-email <type>       Generate phishing email", tc("➥", t));
    println!("    {} list-emails                      List email templates", tc("➥", t));
    println!("    {} phish --phish-template login     Available: login, google, facebook...", tc("➥", t));
    println!();
    println!("  {} AI ATTACKS", bold_tc("[8]", az));
    println!("    {} ai menu                        Interactive AI attack menu", tc("➥", t));
    println!("    {} ai config                      Configure AI providers", tc("➥", t));
    println!("    {} ai attack <url> <provider>     Run AI-powered attack", tc("➥", t));
    println!("    {} ai chat                        Chat with AI assistant", tc("➥", t));
    println!("    {} ai providers                   List AI providers", tc("➥", t));
    println!("    {} ai logs                        View AI activity logs", tc("➥", t));
    println!("    {} ai chain                       Build custom AI attack chain", tc("➥", t));
    println!("    {} ai-menu                         Shortcut for ai menu", tc("➥", t));
    println!();
    println!("  {} SHORTCUTS", bold_tc("[>]", az));
    println!("    {} -u <url> --xss                   Scan for XSS", tc("➥", t));
    println!("    {} -u <url> --sqli                  Scan for SQLi", tc("➥", t));
    println!("    {} -u <url> --lfi                   Scan for LFI", tc("➥", t));
    println!("    {} -R --lhost <ip> --lport <port>   Reverse shell payload", tc("➥", t));
    println!("    {} -A --lhost <ip> --lport <port>   APK payload", tc("➥", t));
    println!("    {} -E --lhost <ip> --lport <port>   Windows EXE payload", tc("➥", t));
    println!("    {} -Z --lhost <ip> --lport <port>   XSS payload", tc("➥", t));
    println!("    {} -w <base>                        Generate wordlist", tc("➥", t));
    println!();
    println!("  {} SESSIONS & PORTS", bold_tc("[>]", az));
    println!("    {} listen --lport 4444        Start reverse shell listener", tc("➥", t));
    println!("    {} listen <host> <port>      Start listener on specific host:port", tc("➥", t));
    println!("    {} listen list               List active listeners", tc("➥", t));
    println!("    {} listen stop <id>          Stop a listener", tc("➥", t));
    println!("    {} sessions                  List active sessions", tc("➥", t));
    println!("    {} sessions use <id>         Interact with a session", tc("➥", t));
    println!("    {} sessions kill <id>        Kill a session", tc("➥", t));
    println!("    {} portfwd add <lport> <rhost> <rport>  Add port forward", tc("➥", t));
    println!("    {} portfwd list              List port forwards", tc("➥", t));
    println!();
    println!("  {} SYSTEM COMMANDS", bold_tc("[>]", az));
    println!("    {} ls, pwd, cd, cat, echo, etc. (run any shell command)", tc("➥", t));
    println!();
    println!("  {} UTILITY", bold_tc("[>]", az));
    println!("    {} help                       Show this help", tc("➥", t));
    println!("    {} modules                    List all modules", tc("➥", t));
    println!("    {} stats                      Show tool statistics", tc("➥", t));
    println!("    {} banner                     Show banner", tc("➥", t));
    println!("    {} tui                        Launch interactive TUI", tc("➥", t));
    println!("    {} clear                      Clear screen", tc("➥", t));
    println!("    {} exit                       Exit shell", tc("➥", t));
    println!();
    println!("  {} Direct cf-void commands also work:", bold_tc("TIP:", y));
    println!("    {} cf-void -u <url> --xss --report report.txt", tc("➥", t));
    println!("    {} --help                      Show all CLI flags", tc("➥", t));
    println!();
}
