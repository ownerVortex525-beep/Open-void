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
                banner::error("Usage: recon <subcommand> <target>");
                banner::info("Subcommands: subdomain, dns, ping, iplookup, netscan, ports, services, whois, headers, techstack");
                return;
            }
            let parts2: Vec<&str> = args_str.split_whitespace().collect();
            let (subcmd, target) = (parts2[0], parts2.get(1).unwrap_or(&""));
            match subcmd {
                "subdomain" | "subs" | "sub" => run_tool_command(&format!("--subdomain-brute --target {}", target)),
                "dns" => run_tool_command(&format!("--osint-subdomains --target {}", target)),
                "ping" => run_system_command(&format!("ping -c 4 {}", target)),
                "iplookup" | "ipinfo" => {
                    let _ = run_system_command("curl");
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
        // Standalone recon commands
        "subfinder" | "subdomains" | "subs" => {
            if args_str.is_empty() {
                banner::error("Usage: subfinder <domain>");
                return;
            }
            let domain = args_str.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            run_tool_command(&format!("--subdomain-brute --target {}", domain));
        }
        "dns" => {
            if args_str.is_empty() {
                banner::error("Usage: dns <domain>");
                return;
            }
            let domain = args_str.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            run_tool_command(&format!("--osint-subdomains --target {}", domain));
        }
        "ping" => {
            if args_str.is_empty() { banner::error("Usage: ping <host>"); return; }
            let host = args_str.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            run_system_command(&format!("ping -c 4 {}", host));
        }
        "iplookup" | "ipinfo" => {
            if args_str.is_empty() { banner::error("Usage: iplookup <ip>"); return; }
            let ip = args_str.trim();
            let _ = run_system_command("curl");
            let result = std::process::Command::new("curl")
                .arg("-s").arg(format!("https://ipapi.co/{}/json/", ip))
                .output();
            if let Ok(o) = result {
                println!("{}", String::from_utf8_lossy(&o.stdout));
            }
        }
        "whois" => {
            if args_str.is_empty() { banner::error("Usage: whois <domain>"); return; }
            run_system_command(&format!("whois {}", args_str));
        }
        "headers" => {
            if args_str.is_empty() { banner::error("Usage: headers <url>"); return; }
            run_system_command(&format!("curl -sI {}", args_str));
        }
        "techstack" | "tech" => {
            if args_str.is_empty() { banner::error("Usage: techstack <url>"); return; }
            run_system_command(&format!("whatweb -v {}", args_str));
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
        "network" | "n" | "net" => {
            let args_vec: Vec<&str> = args_str.split_whitespace().collect();
            if args_vec.is_empty() {
                banner::error("Usage: network -t <ip/cidr> --ports quick");
                banner::info("Examples: network -t 192.168.1.1 --ports quick");
                banner::info("          network -t 192.168.1.0/24 --ports syn");
                return;
            }
            // Parse -t and --target
            let mut target = String::new();
            let mut remaining: Vec<String> = Vec::new();
            let mut i = 0;
            while i < args_vec.len() {
                if (args_vec[i] == "-t" || args_vec[i] == "--target") && i + 1 < args_vec.len() {
                    target = args_vec[i + 1].to_string();
                    i += 2;
                    continue;
                }
                remaining.push(args_vec[i].to_string());
                i += 1;
            }

            if target.is_empty() {
                // Try to use the first non-flag argument as target
                for arg in &args_vec {
                    if !arg.starts_with('-') {
                        target = arg.to_string();
                        break;
                    }
                }
            }

            if target.is_empty() {
                banner::error("Error: No target specified. Use -t <ip/cidr>");
                return;
            }

            let cmd = format!("-t {} {}", target, remaining.join(" "));
            run_tool_command(&cmd);
        }
        "listen" | "l" => {
            if args_str.is_empty() {
                run_tool_command("--listen --lport 4444");
            } else {
                run_tool_command(&format!("--listen {}", args_str));
            }
        }
        "sessions" | "session" => run_tool_command("--sessions"),
        "phish" | "phishing" | "phishy" => {
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            
            // Parse flags from args
            let mut template = String::new();
            let mut port: u16 = 8080;
            let mut tunnel: Option<String> = None;
            let mut gen_only = false;
            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "-h" | "--help" | "help" => {
                        banner::info("Phishing commands:");
                        banner::info("  phish <template>              Generate + auto-serve (e.g., phish instagram)");
                        banner::info("  phish serve <template>         Same as above");
                        banner::info("  phish gen <template>            Generate only (no server)");
                        banner::info("  phish list                     List all templates");
                        banner::info("Flags: --port <port>  --tunnel <cloudflared|localtunnel|serveo>");
                        banner::info("Templates: login, instagram, facebook, google, microsoft, etc.");
                        return;
                    }
                    "--port" | "-p" => { if i + 1 < parts.len() { port = parts[i+1].parse().unwrap_or(8080); i += 1; } }
                    "--tunnel" | "-t" => { if i + 1 < parts.len() { tunnel = Some(parts[i+1].to_string()); i += 1; } }
                    "gen" | "generate" | "g" => gen_only = true,
                    "list" | "templates" => { crate::phishing::PhishingGen::list_templates(); return; }
                    p if p.starts_with("--") => {} // Skip unknown flags
                    p if p == "serve" || p == "s" => {} // serve is default, skip
                    p => {
                        template = p.to_string();
                    }
                }
                i += 1;
            }

            // Default template
            if template.is_empty() {
                template = "login".to_string();
            }

            if gen_only {
                // Generate only
                let gen = crate::phishing::PhishingGen::new();
                match gen.generate(&template, "0.0.0.0", "8080", None) {
                    Some(path) => {
                        banner::success(&format!("Phishing page generated: {}", path));
                        banner::info(&format!("To serve: phish {} --port 8080 --tunnel cloudflared", template));
                    }
                    None => banner::error(&format!("Unknown template: {}. Use phish list", template)),
                }
                return;
            }

            // Default: generate and serve
            banner::info(&format!("Starting phishing server with template '{}'", template));
            let mut server = crate::phishing::PhishingServer::new("0.0.0.0", port, &template);
            if let Some(t) = tunnel { server = server.with_tunnel(&t); }
            let _ = server.start();
        }
        "serve" => {
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            if parts.is_empty() {
                banner::info("Phishing server - credential capture with powers");
                banner::info("Usage: serve <template> [--port <port>] [--tunnel <type>]");
                banner::info("Templates: login, instagram, facebook, google, etc. (see list-templates)");
                banner::info("Tunnels: cloudflared, localtunnel, serveo");
                banner::info("Tip: 'phish instagram' is equivalent to 'serve instagram'");
                return;
            }
            // Use same logic as phish command
            let mut template = String::new();
            let mut port: u16 = 8080;
            let mut tunnel: Option<String> = None;
            let mut i = 0;
            while i < parts.len() {
                match parts[i] {
                    "--port" | "-p" => { if i + 1 < parts.len() { port = parts[i+1].parse().unwrap_or(8080); i += 1; } }
                    "--tunnel" | "-t" => { if i + 1 < parts.len() { tunnel = Some(parts[i+1].to_string()); i += 1; } }
                    p => { template = p.to_string(); }
                }
                i += 1;
            }
            if template.is_empty() { template = "login".to_string(); }
            let mut server = crate::phishing::PhishingServer::new("0.0.0.0", port, &template);
            if let Some(t) = tunnel { server = server.with_tunnel(&t); }
            banner::info(&format!("Starting phishing server with template '{}' on port {}", template, port));
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
            println!("  {} CF-VOID v{}", tc("▐", GOLD), crate::VERSION);
            println!("  {} IND 'CYBER-FORCE' :: Offensive Security Platform", tc("➥", TEAL));
        }
        "ls" | "pwd" | "cd" | "cat" | "echo" | "mkdir" | "rm" | "cp" | "mv" | "chmod" | "grep" | "find" | "whoami" | "ifconfig" | "curl" | "wget" => {
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
            if parts.len() < 2 {
                banner::error("Usage: ai attack <url> [provider]");
                banner::info("Example: ai attack https://example.com cerebras");
                banner::info("Default provider: cerebras");
                return;
            }
            let url = parts[1].to_string();
            let provider = parts.get(2).map(|s| s.to_lowercase()).unwrap_or_else(|| "cerebras".to_string());
            
            banner::info(&format!("Starting AI attack on {} with provider '{}'...", url, provider));
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
