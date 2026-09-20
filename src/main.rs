use clap::Parser;
use std::process;
use std::io::Write;

use cfvoid::cli::args::CliArgs;
use cfvoid::cli::banner;
use cfvoid::core::engine::ScanEngine;
use cfvoid::scanner::proxy_scraper::ProxyScraper;

fn run_system_cmd(cmd: &str) {
    let output = std::process::Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(o) => {
            if !o.stdout.is_empty() { print!("{}", String::from_utf8_lossy(&o.stdout)); }
            if !o.stderr.is_empty() { print!("{}", String::from_utf8_lossy(&o.stderr)); }
        }
        Err(e) => banner::error(&format!("Command failed: {}", e)),
    }
}

fn print_proxy_bar(current: usize, total: usize) {
    let bar_width = 20;
    let filled = if total > 0 { (current as f32 / total as f32 * bar_width as f32) as usize } else { 0 };
    let empty = bar_width - filled;
    let bar = format!("{}{}", "█".repeat(filled), "▱".repeat(empty));
    let pct = if total > 0 { current * 100 / total } else { 0 };
    let g = (218, 165, 32);
    print!("\r\x1B[38;2;{};{};{}m  {} {}% ({} of {}) sources...\x1B[0m",
        g.0, g.1, g.2, bar, pct, current, total);
    std::io::stdout().flush().ok();
}

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        process::exit(1);
    });

    let args = CliArgs::parse();

    // If no arguments provided, launch interactive shell
    if std::env::args().len() <= 1 {
        banner::print_cfvoid_banner();
        cfvoid::cli::shell::run_shell();
        process::exit(0);
    }

    // Skip banner for utility/listing commands
    let skip_banner = args.list_modules || args.list_templates || args.list_emails || args.phish_template.is_some() || args.phish_serve.is_some();
    
    if !args.quiet && !skip_banner {
        banner::print_banner();
        cfvoid::utils::platform::print_platform_info();
    }

    if args.list_modules {
        banner::print_module_list();
        process::exit(0);
    }

    if args.list_templates {
        cfvoid::phishing::PhishingGen::list_templates();
        process::exit(0);
    }

    if let Some(template) = &args.phish_template {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("8080");
        cfvoid::phishing::PhishingGen::new().generate(template, lhost, lport, args.output.as_deref());
        process::exit(0);
    }

    if let Some(template) = &args.phish_serve {
        let lhost = args.lhost.as_deref().unwrap_or("0.0.0.0");
        let lport: u16 = args.lport.as_deref().and_then(|s| s.parse().ok()).unwrap_or(8080);
        let mut server = cfvoid::phishing::PhishingServer::new(lhost, lport, template);
        if let Some(tunnel_type) = &args.tunnel {
            server = server.with_tunnel(tunnel_type);
        }
        let _ = server.start();
        process::exit(0);
    }

    if args.list_emails {
        cfvoid::phishing::list_email_templates();
        process::exit(0);
    }

    if let Some(email_template) = &args.phish_email {
        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
        let lport = args.lport.as_deref().unwrap_or("8080");
        let link = format!("http://{}:{}/{}", lhost, lport, email_template);
        let name = "User".to_string();
        let email = match email_template.to_lowercase().as_str() {
            "birthday" => cfvoid::phishing::EmailTemplate::birthday_email(&name, &link),
            "love" => cfvoid::phishing::EmailTemplate::love_email(&name, &link),
            "offer" => cfvoid::phishing::EmailTemplate::offer_email(&name, &link),
            "card" => cfvoid::phishing::EmailTemplate::card_email(&name, &link),
            "prize" => cfvoid::phishing::EmailTemplate::prize_email(&name, &link),
            "invoice" => cfvoid::phishing::EmailTemplate::invoice_email(&name, &link),
            "shipping" => cfvoid::phishing::EmailTemplate::shipping_email(&name, &link),
            "bank" => cfvoid::phishing::EmailTemplate::bank_email(&name, &link),
            "crypto" => cfvoid::phishing::EmailTemplate::crypto_email(&name, &link),
            "tax" => cfvoid::phishing::EmailTemplate::tax_email(&name, &link),
            "meeting" => cfvoid::phishing::EmailTemplate::meeting_email(&name, &link),
            _ => {
                cfvoid::phishing::list_email_templates();
                process::exit(0);
            }
        };
        banner::info(&format!("Subject: {}", email.subject));
        email.save("phishing_emails");
        banner::success(&format!("Email template generated: phishing_emails/{}.html", email_template));
        banner::info(&format!("Host landing page with: python3 -m http.server {} --bind {}", lport, lhost));
        process::exit(0);
    }

    if args.tui {
        if let Err(e) = cfvoid::cli::tui::run_tui() {
            banner::error(&format!("TUI error: {}", e));
        }
        process::exit(0);
    }

    // New: Web mode
    if args.web {
        #[cfg(feature = "web")]
        {
            if let Err(e) = cfvoid::cli::web::run_web(args.web_port).await {
                banner::error(&format!("Web server error: {}", e));
            }
        }
        #[cfg(not(feature = "web"))]
        {
            banner::error("Web feature not compiled. Build with --features web");
        }
        process::exit(0);
    }

    // New: Proxy scrape mode
    if args.proxy_scrape {
        banner::print_cfvoid_banner();
        banner::success("Starting proxy scrape...");
        let mut scraper = ProxyScraper::new();
        let total = scraper.scrape_all().await;
        banner::success(&format!("Scraped {} proxies from 15+ sources", total));

        // Show progress while testing
        print!("\n  [*] Testing proxy health...\n");
        let (total_count, _, _) = scraper.stats();
        for i in 0..total_count.min(100) {
            print_proxy_bar(i + 1, total_count);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        println!();

        scraper.test_all(50).await;
        let (_total_count, alive, dead) = scraper.stats();
        banner::success(&format!("Alive: {} | Dead: {}", alive, dead));

        for proxy in scraper.alive_proxies.iter().take(20) {
            banner::info(&format!("Alive: {}://{}:{} ({}ms)",
                proxy.kind.as_str(), proxy.ip, proxy.port, proxy.latency_ms));
        }
        if alive > 20 {
            banner::info(&format!("... and {} more", alive - 20));
        }
        process::exit(0);
    }

    if args.crack_hash.is_some() || args.brute_force {
        cfvoid::core::engine::run_password_cracker(&args).await;
        process::exit(0);
    }

    if args.reverse_shell || args.bind_shell || args.web_shell || args.payload_gen ||
       args.apk_payload || args.exe_payload || args.ps_variant.is_some() ||
       args.windows_payload || args.linux_payload || args.macos_payload ||
       args.cloud_payload || args.fileless || args.payload_type.is_some() {
        cfvoid::core::engine::run_payloads_only(&args).await;
        process::exit(0);
    }

    if args.listen {
        cfvoid::core::engine::run_session_listener(&args).await;
        process::exit(0);
    }

    if args.sessions {
        cfvoid::core::engine::run_session_list(&args).await;
        process::exit(0);
    }

    if args.has_fuzz() {
        if let Some(_u) = &args.url {
            cfvoid::core::engine::run_fuzz(&args).await;
            process::exit(0);
        } else {
            banner::error("Fuzzing requires --url <target>");
            process::exit(1);
        }
    }

    // AI Attack
    if args.ai {
        if args.url.is_none() && args.target.is_none() {
            banner::error("AI attack requires --url <target>");
            process::exit(1);
        }
        cfvoid::core::engine::run_ai_attack(&args).await;
        process::exit(0);
    }

    if args.ports.is_some() || args.banner_grab || args.subnet_scan {
        if let Some(_t) = &args.target {
            cfvoid::core::engine::run_network_scan(&args).await;
            process::exit(0);
        } else {
            banner::error("Network scanning requires --target <ip>");
            process::exit(1);
        }
    }

    // Handle positional/command-style arguments (e.g., "cf-void phish gen instagram")
    if !args.extra_args.is_empty() {
        let first = args.extra_args[0].to_lowercase();
        match first.as_str() {
            // phish gen <template>
            "phish" | "phishing" => {
                let sub = args.extra_args.get(1).map(|s| s.to_lowercase());
                match sub.as_deref() {
                    Some("gen") | Some("generate") | Some("g") => {
                        let template = args.extra_args.get(2).cloned().unwrap_or("login".to_string());
                        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
                        let lport = args.lport.as_deref().unwrap_or("8080");
                        cfvoid::phishing::PhishingGen::new().generate(&template, lhost, lport, args.output.as_deref());
                        process::exit(0);
                    }
                    Some("serve") | Some("s") => {
                        let template = args.extra_args.get(2).cloned().unwrap_or("login".to_string());
                        let lhost = args.lhost.as_deref().unwrap_or("0.0.0.0");
                        let lport: u16 = args.lport.as_deref().and_then(|s| s.parse().ok()).unwrap_or(8080);
                        let mut server = cfvoid::phishing::PhishingServer::new(lhost, lport, &template);
                        if let Some(t) = &args.tunnel { server = server.with_tunnel(t); }
                        let _ = server.start();
                        process::exit(0);
                    }
                    Some("list") | Some("templates") => {
                        cfvoid::phishing::PhishingGen::list_templates();
                        process::exit(0);
                    }
                    _ => {
                        // Treat as template name
                        let template = first.clone();
                        if args.lport.is_some() || args.listen {
                            let lhost = args.lhost.as_deref().unwrap_or("0.0.0.0");
                            let lport: u16 = args.lport.as_deref().and_then(|s| s.parse().ok()).unwrap_or(8080);
                            let mut server = cfvoid::phishing::PhishingServer::new(lhost, lport, &template);
                            if let Some(t) = &args.tunnel { server = server.with_tunnel(t); }
                            let _ = server.start();
                            process::exit(0);
                        }
                        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
                        let lport = args.lport.as_deref().unwrap_or("8080");
                        cfvoid::phishing::PhishingGen::new().generate(&template, lhost, lport, args.output.as_deref());
                        process::exit(0);
                    }
                }
            }
            // recon commands
            "subfinder" | "subdomains" | "dns" | "headers" | "techstack" | "tech" | "whois" => {
                let domain = args.extra_args.iter().skip(1).find(|a| !a.starts_with('-'))
                    .map(|s| s.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string())
                    .unwrap_or_default();
                if domain.is_empty() {
                    banner::error(&format!("Usage: cf-void {} <domain>", first));
                    process::exit(1);
                }
                match first.as_str() {
                    "whois" => run_system_cmd(&format!("whois {}", domain)),
                    "headers" => run_system_cmd(&format!("curl -sI {}", domain)),
                    "techstack" | "tech" => run_system_cmd(&format!("whatweb -v {}", domain)),
                    _ => {
                        // Use scan engine with target
                        let mut cli = vec!["cf-void"];
                        cli.push("--target");
                        cli.push(&domain);
                        if first == "subfinder" || first == "subdomains" || first == "dns" {
                            cli.push("--osint-subdomains");
                        }
                        if let Ok(cli_args) = CliArgs::try_parse_from(cli) {
                            let engine = match ScanEngine::new(cli_args) {
                                Ok(e) => e,
                                Err(e) => { banner::error(&format!("Init failed: {}", e)); process::exit(1); }
                            };
                            let _ = engine.run().await;
                        }
                    }
                }
                process::exit(0);
            }
            "ping" => {
                let host = args.extra_args.iter().find(|a| !a.starts_with('-'))
                    .map(|s| s.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string());
                if let Some(h) = host {
                    run_system_cmd(&format!("ping -c 4 {}", h));
                } else {
                    banner::error("Usage: cf-void ping <host>");
                    process::exit(1);
                }
                process::exit(0);
            }
            "iplookup" | "ipinfo" => {
                let ip = args.extra_args.iter().find(|a| !a.starts_with('-')).cloned().unwrap_or_default();
                if ip.is_empty() {
                    banner::error("Usage: cf-void iplookup <ip>");
                    process::exit(1);
                }
                let result = std::process::Command::new("curl")
                    .arg("-s").arg(format!("https://ipapi.co/{}/json/", ip))
                    .output();
                if let Ok(o) = result {
                    println!("{}", String::from_utf8_lossy(&o.stdout));
                }
                process::exit(0);
            }
            _ => {
                // Check if the first arg is a known phishing template
                let template_names = ["instagram", "facebook", "twitter", "github", "gitlab", "google",
                    "microsoft", "slack", "dropbox", "paypal", "netflix", "adobe", "atlassian",
                    "aws", "docker", "vpn", "wifi", "router", "birthday", "love", "offer",
                    "card", "prize", "wedding", "baby_shower", "christmas", "halloween",
                    "valentine", "fathers_day", "mothers_day", "new_year", "thanksgiving",
                    "easter", "resume", "job_offer", "invoice", "shipping", "tax", "bank",
                    "crypto", "social_media", "cloud_storage", "meeting", "login"];
                if template_names.contains(&first.as_str()) {
                    if args.lport.is_some() || args.listen {
                        let lhost = args.lhost.as_deref().unwrap_or("0.0.0.0");
                        let lport: u16 = args.lport.as_deref().and_then(|s| s.parse().ok()).unwrap_or(8080);
                        let mut server = cfvoid::phishing::PhishingServer::new(lhost, lport, &first);
                        if let Some(t) = &args.tunnel { server = server.with_tunnel(t); }
                        let _ = server.start();
                    } else {
                        let lhost = args.lhost.as_deref().unwrap_or("127.0.0.1");
                        let lport = args.lport.as_deref().unwrap_or("8080");
                        cfvoid::phishing::PhishingGen::new().generate(&first, lhost, lport, args.output.as_deref());
                    }
                    process::exit(0);
                }
                // Otherwise, try as a target URL
                let extra: Vec<String> = args.extra_args.iter().map(|s| s.clone()).collect();
                let mut cli = vec!["cf-void"];
                for e in &extra { cli.push(e); }
                if let Ok(cli_args) = CliArgs::try_parse_from(cli) {
                    if cli_args.has_target() && cli_args.has_any_attack() {
                        let engine = match ScanEngine::new(cli_args.clone()) {
                            Ok(e) => e,
                            Err(e) => {
                                banner::error(&format!("Init failed: {}", e));
                                process::exit(1);
                            }
                        };
                        match engine.run().await {
                            Ok(result) => {
                                if let Some(report_path) = &cli_args.report {
                                    let format = cli_args.format.as_deref().unwrap_or("json");
                                    let gen = cfvoid::report::generator::ReportGenerator::new(result.clone());
                                    if let Err(e) = gen.save(report_path, format) {
                                        banner::error(&format!("Report save failed: {}", e));
                                    } else {
                                        banner::print_report_saved(report_path);
                                    }
                                }
                            }
                            Err(e) => banner::error(&format!("Scan failed: {}", e)),
                        }
                    }
                }
                process::exit(0);
            }
        }
    }

    if !args.has_target() {
        banner::error("No target specified. Use -u <url> or -t <ip>");
        process::exit(1);
    }

    if !args.has_any_attack() {
        banner::error("No attack module selected. Use --web-all or --help");
        process::exit(1);
    }

    let engine = match ScanEngine::new(args.clone()) {
        Ok(e) => e,
        Err(e) => {
            banner::error(&format!("Initialization failed: {}", e));
            process::exit(1);
        }
    };

    match engine.run().await {
        Ok(result) => {
            if let Some(report_path) = &args.report {
                let format = args.format.as_deref().unwrap_or("json");
                let gen = cfvoid::report::generator::ReportGenerator::new(result.clone());
                if let Err(e) = gen.save(report_path, format) {
                    banner::error(&format!("Report save failed: {}", e));
                } else {
                    banner::print_report_saved(report_path);
                }
            }

            if let Some(output) = &args.output {
                let format = args.format.as_deref().unwrap_or("json");
                let gen = cfvoid::report::generator::ReportGenerator::new(result.clone());
                if let Err(e) = gen.save(output, format) {
                    banner::error(&format!("Report save failed: {}", e));
                } else {
                    banner::success(&format!("Report saved: {}", output));
                }
            }

            process::exit(if result.finding_count() > 0 { 1 } else { 0 });
        }
        Err(e) => {
            banner::error(&format!("Scan failed: {}", e));
            process::exit(1);
        }
    }
}
